[![ci](https://github.com/pepa65/aegis-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/pepa65/aegis-cli/actions)
[![dependency status](https://deps.rs/repo/github/pepa65/aegis-cli/status.svg)](https://deps.rs/repo/github/pepa65/aegis-cli)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![downloads](https://img.shields.io/crates/d/aegis-cli.svg)](https://crates.io/crates/aegis-cli)

# aegis-cli 1.1.0
**Show TOTPs from Aegis vault on CLI**

* License: GPLv3.0
* Authors: github.com/pepa65, github.com/Granddave
* Repo: https:/github.com/pepa65/aegis-cli
* After: https://github.com/Granddave/aegis-rs

CLI app for showing TOTP codes from an Aegis vault file (like from the backup file from the Aegis Android app [Aegis Authenticator](https://github.com/beemdevelopment/Aegis)). After [aegis-rs](https://github.com/Granddave/aegis-rs).

## Features
* Decryption of the 256 bit AES-GCM encrypted vault 🔓
* Fuzzy selection 🔍
* TOTP display 🕒
* Clears the screen when done ☐
* Time left indication ⏳
* Clipboard support 📋
* Optional JSON output to stdout 📜
* Optional URL output to stdout 📜

## Usage
### Installation
The easiest way to install `aegis-cli` is by using [cargo](https://crates.io/).

#### Cargo from crates.io
`cargo install aegis-cli`

#### Cargo from git

`cargo install --git https://github.com/pepa65/aegis-cli`

#### Static build (avoiding GLIBC incompatibilities)
```
git clone https://github.com/pepa65/aegis-cli
cd aegis-cli
rustup target add x86_64-unknown-linux-musl
export RUSTFLAGS='-C target-feature=+crt-static'
cargo build --release --target=x86_64-unknown-linux-musl
```

## Install static single-binary
```
wget https://github.com/pepa65/aegis-cli/releases/download/1.1.0/aegis
sudo mv aegis /usr/local/bin
sudo chown root:root /usr/local/bin/aegis
sudo chmod +x /usr/local/bin/aegis
```

### Launching Aegis-cli with an Aegis vault file
To start `aegis-cli`, simply pass the path to your backup file as an argument and enter the password when prompted.
For example:

`aegis aegis-backup-20230512-193110.json`

? Insert Aegis Password › `********`

### Searching for an Entry
Fuzzy finding is supported for quickly locating entries. Type some letters of the entry's name to filter the list.
Pressing `Esc` exits the app.

### Displaying the OTP
After an entry is selected, the TOTP can be copied from the terminal or pasted through the integrated clipboard support.
TOTPs are updated automatically upon expiration. Pressing `Esc` will go back to the Fuzzy selection screen.

### Ways to unlock the Vault
To unlock the Aegis vault `aegis-cli` supports the following methods:

1. **Password prompt**: If no password is provided, `aegis-cli` will prompt for a password.
2. **Password file**: A file containing the password to unlock the Aegis vault:
  - Environment variable: `AEGIS_PWFILE`
  - Argument: `-p <PASSWORD_FILE>` or `--password-file <PASSWORD_FILE>`
  - Example: `aegis -p ~/.aegis.pw aegis-vault.json`
3. **Password**: The password can be passed as an argument or set as an environment variable:
  - Environment variable: `AEGIS_PASSWORD`
  - Argument: `-P <PASSWORD>` or `--password <PASSWORD>`
  - Example: `aegis -P jkhglhkjhkjf aegis-vault.json`

### Extra flags
* `-n <NAME>...` or `--name <NAME>...`: Pre-filter entries by entries NAME.
  - Example: `aegis -n git dave aegis-vault.json`
* `-i <ISSUER>...` or `--issuer <ISSUER>...`: Pre-filter entries by entries ISSUER.
* `-j` or `--json`: Output the (pre-filtered) TOTPs as JSON.

### Help
```
aegis-cli 1.1.0 - Show TOTPs from Aegis vault on CLI
Usage: aegis [OPTIONS] <VAULT_FILE>
Arguments:
  <VAULT_FILE>  Path to Aegis vault file [env: AEGIS_VAULT_FILE=]

Options:
  -p, --pwfile <PWFILE>      File with Aegis vault password [env: AEGIS_PWFILE=]
  -P, --password <PASSWORD>  PASSWORD to unlock Aegis vault [env: AEGIS_PASSWORD]
  -i, --issuer <ISSUER>...   Filter by ISSUER
  -n, --name <NAME>...       Filter by NAME
  -j, --json                 Output entries in JSON
  -u, --url                  Output entries in URL format
  -h, --help                 Print help
  -V, --version              Print version
```

## Project history
This project has been divided into a CLI binary (this repo) and a [vault
utility](https://github.com/Granddave/aegis-vault-utils) crate so that other
projects can utilize the parsing and TOTP generation functionalities as well.

## License
This project is licensed under the GNU General Public License v3.0. See the [LICENSE](LICENSE) file for details.
