use aegis_vault_utils::otp::EntryInfo::Totp;
use aegis_vault_utils::{
	otp::{calculate_remaining_time, generate_otp, Entry, EntryInfo},
	vault::{parse_vault, PasswordGetter},
};
use clap::{Args, Parser};
use color_eyre::eyre::{eyre, Result};
use console::{Key, Style, Term};
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Password};
use std::sync::mpsc::{self, TryRecvError};
use std::{fs, path::PathBuf, process::exit, thread, time::Duration};
use urlencoding::encode;

#[derive(Parser)]
#[command(version, about)]
#[command(help_template(
	"\
{before-help}{name} {version} - {about}
{usage-heading} {usage}
{all-args}{after-help}
"
))]

struct Cli {
	#[clap(
		help = "Encrypted Aegis Vault JSON file (separate it from name/issuer
filters by putting -- before it",
		env = "AEGIS_VAULT_FILE"
	)]
	vault_file: PathBuf,
	#[clap(short, long, help = "Show OTP entries in plain text")]
	otp: bool,
	#[clap(short, long, help = "Export entries to Plain Aegis Vault JSON")]
	json: bool,
	#[clap(short, long, help = "Export entries in URL format")]
	url: bool,
	#[clap(flatten)]
	password_input: PasswordInput,
	#[clap(flatten, help = "Filter by ISSUER and/or NAME")]
	entry_filter: EntryFilter,
}

#[derive(Args)]
struct PasswordInput {
	#[clap(short, long, env = "AEGIS_PWFILE", help = "Aegis Vault passwordfile")]
	pwfile: Option<PathBuf>,
	#[clap(
		short('P'),
		long,
		env = "AEGIS_PASSWORD",
		help = "PASSWORD for Aegis Vault",
		conflicts_with = "password_file",
		hide_env_values = true
	)]
	password: Option<String>,
}

#[derive(Args)]
struct EntryFilter {
	#[clap(short, long, num_args(1..), help = "Filter by ISSUER (multiple allowed)")]
	issuer: Option<String>,
	#[clap(short, long, num_args(1..), help = "Filter by NAME (multiple allowed)")]
	name: Option<String>,
}

#[derive(Debug, serde::Serialize)]
struct EntryInfo_ {
	secret: String,
	algo: String,
	digits: i32,
	period: i32,
}

#[derive(Debug, serde::Serialize)]
struct Entry_ {
	r#type: String,
	name: String,
	uuid: Option<String>,
	issuer: String,
	note: Option<String>,
	icon: Option<String>,
	icon_mime: Option<String>,
	icon_hash: Option<String>,
	favorite: bool,
	info: EntryInfo_,
	groups: Option<String>,
}

impl EntryFilter {
	fn matches(&self, entry: &Entry) -> bool {
		if let Some(issuer) = &self.issuer {
			if !entry.issuer.to_lowercase().contains(&issuer.to_lowercase()) {
				return false;
			}
		}
		if let Some(name) = &self.name {
			if !entry.name.to_lowercase().contains(&name.to_lowercase()) {
				return false;
			}
		}
		true
	}
}

impl PasswordGetter for PasswordInput {
	fn get_password(&self) -> Result<String> {
		match (&self.password, &self.pwfile) {
			(Some(password), None) => Ok(password.clone()),
			(None, Some(password_file)) => {
				let password = fs::read_to_string(password_file)?;
				Ok(password.trim().to_string())
			}
			_ => Password::with_theme(&ColorfulTheme::default())
				.with_prompt("Enter Aegis vault Password")
				.interact()
				.map_err(|e| eyre!("Failed to get password: {}", e)),
		}
	}
}

fn set_sigint_hook() {
	ctrlc::set_handler(move || {
		Term::stdout().show_cursor().expect("Showing cursor");
	})
	.expect("Setting SIGINT handler");
}

fn print_otp_every_second(entry_info: &EntryInfo) -> Result<()> {
	let term = Term::stdout();
	term.hide_cursor()?;
	let (tx, rx) = mpsc::channel();
	// Spawn a thread to listen for key presses
	thread::spawn(move || {
		let term = Term::stdout();
		loop {
			if let Ok(key) = term.read_key() {
				if key == Key::Escape {
					let _ = tx.send(());
					break;
				}
			}
		}
	});

	let mut clipboard = arboard::Clipboard::new().ok();
	let mut otp_code = String::new();
	let mut last_remaining_time = 0;

	loop {
		match rx.try_recv() {
			Ok(_) | Err(TryRecvError::Disconnected) => {
				term.clear_last_lines(1)?;
				term.show_cursor()?;
				break;
			}
			Err(TryRecvError::Empty) => {}
		}

		let remaining_time = calculate_remaining_time(entry_info)?;
		if last_remaining_time < remaining_time {
			otp_code = generate_otp(entry_info)?;
			if let Some(clipboard) = clipboard.as_mut() {
				clipboard.set_text(otp_code.clone())?;
			}
		}

		let style = match remaining_time {
			0..=5 => Style::new().red(),
			6..=15 => Style::new().yellow(),
			_ => Style::new().green(),
		};
		let line = style.bold().apply_to(format!("{} ({}s left) - Esc to exit", otp_code, remaining_time));
		term.write_line(line.to_string().as_str())?;
		std::thread::sleep(Duration::from_millis(60));
		term.clear_last_lines(1)?;
		last_remaining_time = remaining_time;
	}

	Ok(())
}

fn entries_otp(entries: &[Entry]) -> Result<()> {
	let mut remaining = 0;
	entries.iter().for_each(|entry| {
		println!("{}  {}:{}", generate_otp(&entry.info).unwrap(), entry.issuer.clone(), entry.name.clone(),);
		remaining = calculate_remaining_time(&entry.info).unwrap();
	});
	println!("Remaining: {remaining} s");
	Ok(())
}

fn entries_to_json(entries: &[Entry]) -> Result<()> {
	let output: Vec<Entry_> = entries
		.iter()
		.map(|entry| {
			let Entry { info, .. } = entry;
			let Totp(infototp) = info else { panic!("Invalid entry") };
			Ok(Entry_ {
				r#type: "totp".to_string(),
				uuid: None,
				name: entry.name.clone(),
				issuer: entry.issuer.clone(),
				note: None,
				icon: None,
				icon_mime: None,
				icon_hash: None,
				favorite: false,
				info: EntryInfo_ { secret: infototp.secret.to_string(), algo: "SHA1".to_string(), digits: infototp.digits, period: infototp.period },
				groups: None,
			})
		})
		.collect::<Result<Vec<Entry_>>>()?;
	if output.is_empty() {
		println!("No entries found");
	} else {
		println!("{}", serde_json::to_string_pretty(&output)?);
	}
	Ok(())
}

fn entries_to_url(entries: &[Entry]) {
	for entry in entries.iter() {
		let Entry { info, .. } = entry;
		let Totp(infototp) = info else { panic!("Invalid entry") };
		let algo = format!("{:?}", infototp.algo);
		println!(
			"otpauth://totp/{}?secret={}&algorithm={}&digits={}&period={}&issuer={}",
			encode(&entry.name),
			infototp.secret.replace("\"", "").clone(),
			algo.to_string().to_uppercase(),
			infototp.digits,
			infototp.period,
			encode(&entry.issuer),
		);
	}
}

fn fuzzy_select(entries: &[Entry]) -> Result<()> {
	set_sigint_hook();
	let items: Vec<String> = entries.iter().map(|entry| format!("{} ({})", entry.issuer.trim(), entry.name.trim())).collect();
	loop {
		let selection = match FuzzySelect::with_theme(&ColorfulTheme::default()).items(&items).default(0).clear(true).interact_opt() {
			Ok(selection) => selection,
			Err(_) => {
				// Exit on Ctrl-C
				print!("\x1Bc");
				exit(1);
			}
		};
		match selection {
			Some(index) => {
				let entry_info = &entries.get(index).unwrap().info;
				print_otp_every_second(entry_info)?;
			}
			None => {
				// Exit on Escape key
				exit(0);
			}
		}
	}
}

fn main() -> Result<()> {
	color_eyre::install()?;

	let args = Cli::parse();

	let file_contents = match fs::read_to_string(&args.vault_file) {
		Ok(contents) => contents,
		Err(e) => {
			eprintln!("Failed to read Aegis vault file: {}", e);
			exit(1);
		}
	};
	let entries = match parse_vault(&file_contents, &args.password_input) {
		Ok(db) => db
			.entries
			.into_iter()
			// Only TOTP entries are supported at the moment remove this filter later
			.filter(|e| matches!(e.info, EntryInfo::Totp(_)))
			.filter(|e| args.entry_filter.matches(e))
			.collect::<Vec<Entry>>(),
		Err(e) => {
			eprintln!("Failed to open Aegis vault: {}", e);
			exit(1);
		}
	};

	if entries.is_empty() {
		println!("No matching entries based on filters");
		return Ok(());
	}

	if args.otp {
		entries_otp(&entries)?;
	} else if args.json {
		entries_to_json(&entries)?;
	} else if args.url {
		entries_to_url(&entries);
	} else {
		fuzzy_select(&entries)?;
	}

	Ok(())
}
