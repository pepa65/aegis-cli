[![ci](https://github.com/pepa65/aegis-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/pepa65/aegis-rs/actions)
[![dependency status](https://deps.rs/repo/github/pepa65/aegis-cli/status.svg)](https://deps.rs/repo/github/pepa65/aegis-cli)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

# Aegis-vault compatible TOTP display on CLI 
CLI app for showing TOTP codes from an Aegis vault file (like from the backup file from the Aegis Android app [Aegis Authenticator](https://github.com/beemdevelopment/Aegis)).

## Features
- Decryption of the 256 bit AES-GCM encrypted vault 🔓
- Fuzzy selection 🔍
- TOTP display 🕒
- Time left indication ⏳
- Clipboard support 📋
- Optional JSON output to stdout 📜

## Getting Started with Aegis-rs
### Installation
The easiest way to install `aegis-cli` is by using [cargo](https://crates.io/):

```sh
cargo install --git https://github.com/pepa65/aegis-cli --tag latest
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
  - Environment variable: `AEGIS_PASSWORD_FILE`
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
* `-j` or `--json`: Output the (filtered) TOTPs as JSON.

## Project history
This project has been divided into a CLI binary (this repo) and a [vault
utility](https://github.com/Granddave/aegis-vault-utils) crate so that other
projects can utilize the parsing and TOTP generation functionalities as well.

# License
This project is licensed under the GNU General Public License v3.0. See the [LICENSE](LICENSE) file for details.
