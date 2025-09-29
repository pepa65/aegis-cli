[![version](https://img.shields.io/crates/v/aegis-cli.svg)](https://crates.io/crates/dqr)
[![build](https://github.com/pepa65/aegis-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/pepa65/aegis-cli/actions/workflows/ci.yml)
[![dependencies](https://deps.rs/repo/github/pepa65/aegis-cli/status.svg)](https://deps.rs/repo/github/pepa65/aegis-cli)
[![docs](https://img.shields.io/badge/docs-aegis--cli-blue.svg)](https://docs.rs/crate/aegis-cli/latest)
[![license](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://github.com/pepa65/aegis-cli/blob/main/LICENSE)
[![downloads](https://img.shields.io/crates/d/aegis-cli.svg)](https://crates.io/crates/aegis-cli)

# aegis-cli 1.3.36
**Show TOTPs from Aegis vault on CLI**

* License: GPLv3.0
* Authors: github.com/pepa65, github.com/Granddave
* Repo: https:/github.com/pepa65/aegis-cli
* After: https://github.com/Granddave/aegis-rs

CLI app for showing TOTP codes from an Aegis vault file (backup file from the
Aegis Authenticator Android app [Aegis Authenticator](https://github.com/beemdevelopment/Aegis)).

## Features
* Decryption of the 256 bit AES-GCM encrypted vault 🔓
* Fuzzy selection 🔍
* TOTP display 🕒
* Clears the screen when done ☐
* Time left indication ⏳
* Clipboard support 📋
* Optional JSON output to stdout 📜
* Optional URL output to stdout 📜

## Installation
### Download static single-binary
```
wget https://github.com/pepa65/aegis-cli/releases/download/1.3.36/aegis
sudo mv aegis /usr/local/bin
sudo chown root:root /usr/local/bin/aegis
sudo chmod +x /usr/local/bin/aegis
```

### Using cargo (rust toolchain)
If not installed yet, install a **Rust toolchain**, see https://www.rust-lang.org/tools/install

### Cargo from crates.io
`cargo install aegis-cli`

#### Cargo from git

`cargo install --git https://github.com/pepa65/aegis-cli`

#### Cargo static build (avoid GLIBC incompatibilities)
```
git clone https://github.com/pepa65/aegis-cli
cd aegis-cli
rustup target add x86_64-unknown-linux-musl
export RUSTFLAGS='-C target-feature=+crt-static'
cargo build --release --target=x86_64-unknown-linux-musl
```

## Install with cargo-binstall
Even without a full Rust toolchain, rust binaries can be installed with the static binary `cargo-binstall`:

```
# Install cargo-binstall for Linux x86_64
# (Other versions are available at https://crates.io/crates/cargo-binstall)
wget github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
tar xf cargo-binstall-x86_64-unknown-linux-musl.tgz
sudo chown root:root cargo-binstall
sudo mv cargo-binstall /usr/local/bin/
```

Only a linux-x86_64 (musl) binary available: `cargo-binstall aegis-cli`

Then `aegis` will be installed in `~/.cargo/bin/` which will need to be added to `PATH`!

## Usage
### Launching Aegis-cli with an Aegis vault file
To start `aegis-cli`, simply pass the path to your backup file as an argument and enter the password when prompted.
For example:

`aegis aegis-backup-20230512-1.3.360.json`

? Insert Aegis Password › `********`

#### Searching for an Entry
Fuzzy finding is supported for quickly locating entries. Type some letters of the entry's name to filter the list.
Pressing `Esc` exits the app.

#### Displaying the OTP
After an entry is selected, the TOTP can be copied from the terminal or pasted through the integrated clipboard support.
TOTPs are updated automatically upon expiration. Pressing `Esc` will go back to the Fuzzy selection screen.

#### Ways to unlock the Vault
To unlock the Aegis vault, `aegis-cli` supports the following methods:

1. **Password prompt**: If no password is provided, `aegis-cli` will prompt for a password.
2. **Password file**: A file containing the password to unlock the Aegis vault:
  - Environment variable: `AEGIS_PWFILE`
  - Argument: `-p <PASSWORD_FILE>` or `--password-file <PASSWORD_FILE>`
  - Example: `aegis -p ~/.aegis.pw aegis-vault.json`
3. **Password**: The password can be passed as an argument or set as an environment variable:
  - Environment variable: `AEGIS_PASSWORD`
  - Argument: `-P <PASSWORD>` or `--password <PASSWORD>`
  - Example: `aegis -P jkhglhkjhkjf aegis-vault.json`

#### Extra flags
* `-n <NAME>...` or `--name <NAME>...`: Pre-filter entries by entries NAME.
  - Example: `aegis -n git dave aegis-vault.json`
* `-i <ISSUER>...` or `--issuer <ISSUER>...`: Pre-filter entries by entries ISSUER.
* `-o` or `--otp`: Output selected OPTs as plain text.
* `-j` or `--json`: Output selected entries as JSON.
* `-u` or `--uri`: Output selected entries as otpauth URIs, according to
  https://datatracker.ietf.org/doc/draft-linuxgemini-otpauth-uri/01/

### Help
```
aegis-cli 1.3.36 - Show TOTPs from Aegis vault on CLI
Usage: aegis [OPTIONS] <VAULT_FILE>
Arguments:
  <VAULT_FILE>  Encrypted Aegis Vault JSON file (separate it from name/issuer
                filters by putting -- before it [env: AEGIS_VAULT_FILE=]

Options:
  -o, --otp                  Show OTP entries in plain text
  -j, --json                 Export entries to Plain Aegis Vault JSON
  -u, --url                  Export entries in URL format
  -p, --pwfile <PWFILE>      Aegis Vault passwordfile [env: AEGIS_PWFILE=]
  -P, --password <PASSWORD>  PASSWORD for Aegis Vault [env: AEGIS_PASSWORD]
  -i, --issuer <ISSUER>...   Filter by ISSUER (multiple allowed)
  -n, --name <NAME>...       Filter by NAME (multiple allowed)
  -h, --help                 Print help
  -V, --version              Print version
```

## Project history
This project has been divided into a CLI binary (this repo) and a [vault
utility](https://github.com/Granddave/aegis-vault-utils) crate so that other
projects can utilize the parsing and TOTP generation functionalities as well.

## License
This project is licensed under the GNU General Public License v3.0. See the [LICENSE](LICENSE) file for details.
