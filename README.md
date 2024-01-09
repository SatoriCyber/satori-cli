# Satori-CLI 
The Satori CLI enables you to access all of your available datasets from the command line, including both your personal datasets as well as datasets that are made available to you because they are open to groups and teams of which you are already a member. You can request access to read, write or administer datasets.

## Overview
- [Satori-CLI](#satori-cli)
  - [Overview](#overview)
  - [Installation](#installation)
    - [Mac](#mac)
  - [Usage](#usage)
    - [Run](#run)
      - [Supported Tools](#supported-tools)
      - [Arguments](#arguments)
      - [psql](#psql)
      - [mongosh](#mongosh)
    - [PgPass](#pgpass)
      - [Arguments](#arguments-1)
    - [Login](#login)
      - [Arguments:](#arguments-2)
  - [Contributing](#contributing)


## Installation
### Mac
```bash
brew tap satoricyber/satori
brew install satori_cli
```

**Enable auto-completion**
Add the following line to your shell configuration file (e.g., ~/.bashrc or ~/.zshrc):
```bash 
  source "$(brew --prefix)/etc/bash_completion.d/satori_auto_complete.zsh"
```
Once the login is completed (one time), the auto-complete will work.

**first time use**
To enable auto-complete, run the [login](#login) command.

## Usage
### Run
Invokes a CLI tool using Satori authentication.
If the credentials already exist, the CLI tool loads them from the cache. If the credentials do not already exist then they are invoked and authenticated.

#### Supported Tools
The Satori CLI supports psql

#### Arguments
 - `--no-persist` - Does not persist the credentials to the cache.
 - `--no-launch-browser` - Do not launch the browser to authenticate, instead print the URL to the terminal. 
 - `--refresh` - obtain new credentials and datastores information from the server, even if they already exist in the cache.
 -  `<datastore host>` - The datastore host.
 - `--` - Pass the rest of the arguments to the tool.

#### psql
Triggers a psql session with the given datastore and database.

**Example**
```bash
  satori run psql <datastore-host> <database>
```

Passing additional args to the tool
```bash
satori run psql <datastore-host> -d <database> -- -c 'select 1'
```


#### mongosh
Triggers a mongosh session with the given datastore.

``` bash
satori connect mongosh <datastore-host>
```

### PgPass
Generates a pgpass file from all datastore information.
```bash
satori pgpass
```
#### Arguments
  - `--refresh` - obtain new credentials and datastores information from the server, even if they already exist in the cache.
  - `--no-launch-browser` - Do not launch the browser to authenticate, instead print the URL to the terminal. 

### Login
Obtain credentials from Satori data portal without the need to use a browser.

**Example**:
Display the credentials in the terminal (this can be used to integrate with other tools):
```bash
satori login --display
```

#### Arguments: 
 - `--display` - Print the credentials to the terminal, with no persistence.
 - `--refresh` - obtain new credentials and datastores information from the server, even if they already exist in the cache. 
 - `--format` - Format of the output, 
   - `csv` (default).
   - `json`, 
   - `yaml`, 
 - `--no-launch-browser` - Do not launch the browser to authenticate, instead print the URL to the terminal.


## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.
