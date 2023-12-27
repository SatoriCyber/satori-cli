# Satori-CLI 
The Satori CLI enables you to access all of your available datasets from the command line, including both your personal datasets as well as datasets that are made available to you because they are open to groups and teams of which you are already a member. You can request access to read, write or administer datasets.

## Overview
- [satori-cli](#satori-cli)
  - [Overview](#overview)
  - [Installation](#installation)
    - [Mac](#mac)
  - [Usage](#usage)
    - [Connect](#connect)
      - [Supported tools:](#supported-tools)
      - [Arguments:](#arguments)
      - [psql](#psql)
    - [Login](#login)
      - [arguments:](#arguments-1)
  - [Contributing](#contributing)


## Installation
### Mac
```bash
brew tap satoricyber/satori
brew install satori_cli
```

## Usage
### Connect
Invokes a CLI tool using Satori authentication.
If the credentials already exist, the CLI tool loads from them from the cache. If the credentials do not already exist then they are invoked and authenticated.

#### Supported Tools
The Ssatori CLI supports psql

#### Arguments
 - `--no-persist` - Does not persist the credentials to the cache.
 - `--no-launch-browser` - Do not launch the browser to authenticate, instead print the URL to the terminal. 
 -  `<address>` - The datastore host address connection.
 - `--` - Pass the rest of the arguments to the tool.

#### psql
Triggers a psql session with the given datastore and database.

**Parameters**
 - `-d <database>` - The database to connect to.

**Example**
```bash
satori connect psql <datastore-host> -d <database>
```

Passing additional args to the tool
```bash
satori connect psql <datastore-host> -d <database> -- -c 'select 1'
```

### Login
Obtain credentials from Satori data portal without the need to use a browser.

**Example**:
Display the credentials in the terminal (this can be used to integrate with other tools):
```bash
satori login --display
```

#### Arguments: 
 - `--display` - Print the credentials to the terminal, with no persistence.
 - `--format` - Format of the output, 
   - `csv` (default).
   - `json`, 
   - `yaml`, 
 - `--no-launch-browser` - Do not launch the browser to authenticate, instead print the URL to the terminal.


## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.
