# satori-cli
Allow to interact with Satori data portal from the command line.

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
Invokes a CLI tool where the authentication happens using Satori authentication.
If the credentials already exist, loads from the cache, if not invoke authentication.

#### Supported tools:
 - psql

#### Arguments:
 - `--no-persist` - do not persist the credentials to the cache
 - `--no-launch-browser` - Do not launch the browser to authenticate, instead print the URL to the terminal. 

#### psql
Triggers a psql session with the given datastore and database.
```bash
satori connect psql <datastore-host> -d <database>
```


### Login
Obtain credentials from Satori data portal without the need to use a browser.

**Example**:
Display the credentials in the terminal (can be used to be integrated with other tools):
```bash
satori login --display
```

#### arguments: 
 - `--display` - Print the credentials to the terminal, no persistence.
 - `--format` - Format of the output, 
   - `csv` (default).
   - `json`, 
   - `yaml`, 
 - `--no-launch-browser` - Do not launch the browser to authenticate, instead print the URL to the terminal.


## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.