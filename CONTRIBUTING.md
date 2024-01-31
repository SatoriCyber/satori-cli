# Contributing to satori-cli

Thank you for considering contributing to SatoriCli! By contributing, you help improve the Rust ecosystem.

## Prerequisites

Before you start, ensure you have the following prerequisites installed:
- Rust and Cargo: [Install Rust](https://www.rust-lang.org/tools/install)
- Clippy: [Install Clippy](https://github.com/rust-lang/rust-clippy)
- Git: [Install Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
- Your favorite code editor with Rust support

## Setting Up the Development Environment

1. Clone the repository:
   ```bash
   git clone https://github.com/SatoriCyber/satori-cli.git
   cd satori-cli
   ```

## Adding new tools to connect

Edit the configuration file `configuration/tools.yaml` and add a new entry.
The entry should contain the following fields:
- `name` - the name of the tool
- `command` - the command to invoke the tool
- `arguments` - the arguments to pass to the tool
- `env` - which environment variables to pass to the tool
- `requires` - other parameters that are required to be passed to the tool, for example -d database for psql.

**Parameters**:
- `{{host}}` - the datastore host
- `{{user}}` - the username 
- `{{password}}` - the password

The parameters can be used both in the args and in the env.

## Before pushing your changes
Verify that your changes are formatted correctly and pass all tests:
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```


## Release a new version
If you don't have the `cargo-release` tool, install it:
```bash
cargo install cargo-release
```

Make sure you are on the main branch:
```bash
git checkout main
```
Verify first that all as expected:
```bash
cargo release patch --no-publish
```
If everything is ok, release a new version:
```bash
cargo release patch --execute --no-publish
```

Go to [Github Releases](https://www.github.com/SatoriCyber/satori-cli/releases) and add a new release.
In the tags field chose te tag which was pushed by the release command.
In the release title write the version number.
Click on generate release notes and edit them.
Click on publish release.
