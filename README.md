# Satori-CLI 
The Satori CLI enables you to access all of your available datasets from the command line, including both your personal datasets as well as datasets that are made available to you because they are open to groups and teams of which you are already a member. You can request access to read, write or administer datasets.

## Overview
- [Satori-CLI](#satori-cli)
  - [Overview](#overview)
  - [Installation](#installation)
    - [Mac](#mac)
    - [Windows](#windows)
    - [Linux](#linux)
  - [first time use](#first-time-use)
  - [Supported datastores](#supported-datastores)
  - [Usage](#usage)
    - [Run](#run)
      - [Supported Tools](#supported-tools)
      - [Arguments](#arguments)
      - [psql](#psql)
      - [mongosh](#mongosh)
      - [s3](#s3)
      - [DBT](#dbt)
        - [Invoking DBT](#invoking-dbt)
    - [PgPass](#pgpass)
    - [AWS](#aws)
      - [Arguments](#arguments-1)
    - [Login](#login)
      - [Arguments:](#arguments-2)
  - [Contributing](#contributing)


## Installation
### Mac
```zsh
brew tap satoricyber/satori
brew install satori_cli
source "$(brew --prefix)/etc/bash_completion.d/satori_auto_complete.zsh"
```

### Windows
**Manual install**
You can download the latest version from the [releases page](https://github.com/SatoriCyber/satori-cli/releases)
Download the windows zip file.
Unzip the files:
``` powershell
Expand-Archive <ZIP FILE> -DestinationPath <DESTINATION PATH>
```
Move the satori exe to a permanent location (e.g., `C:\Program Files\Satori`)
Add the exe to your path, in your `$profile` add the following line:

```powershell
$env:Path += ";C:\Program Files\Satori"
```
Add the auto-complete file to your scripts directory (e.g., `~\Documents\WindowsPowerShell\Scripts`)

Add to your profile to enable auto-complete:
```powershell
. ~\Documents\WindowsPowerShell\Scripts\satori_auto_complete.ps1
Import-Module ~\Documents\WindowsPowerShell\Scripts\satori_auto_complete.ps1
```


**SCOOP**

To install using [scoop](https://scoop.sh/), run the following command:
```powershell
scoop bucket add satori  https://www.github.com/satoricyber/satori-cli
scoop install satori/satori_cli
```

**Enable auto-complete**
Add the following line to you powershell profile (e.g., `~\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1`):
```powershell
. <SCOOP-APP-DIR>\satori_cli\current\satori_auto_complete.ps1
Import-Module <SCOOP-APP-DIR>\satori_cli\current\satori_auto_complete.ps1
```

### Linux
Use the install script:
```bash
curl -s https://raw.githubusercontent.com/SatoriCyber/satori-cli/main/scripts/install_script.sh | bash
```


## first time use
To enable auto-complete, run the [login](#login) command.

## Supported datastores
- [Athena](https://aws.amazon.com/athena/)
- [MongoDB](https://www.mongodb.com/)
- [PostgreSQL](https://www.postgresql.org/)
- [S3](https://aws.amazon.com/s3/)
- [Redshift](https://aws.amazon.com/redshift/)
- [CockroachDB](https://www.cockroachlabs.com/)
- [Greenplum](https://greenplum.org/)


## Usage
### Run
Invokes a CLI tool using Satori authentication.
If the credentials already exist, the CLI tool loads them from the cache. If the credentials do not already exist then they are invoked and authenticated.

#### Supported Tools
The Satori CLI supports psql, dbt, mongosh

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
satori run psql <datastore-host> <database> -- -c 'select 1'
```


#### mongosh
Triggers a mongosh session with the given datastore.

``` bash
satori run mongosh <datastore-host>
```

#### s3
Execute s3 command.

``` bash
satori run s3 <datastore-host> -- <s3 command>
```

**Example**
```bash
  satori run s3 <datastore-host> -- ls
```

#### DBT
The Satori CLI invokes the the DBT tool with the Satori credentials for a specific Satori dataset. This is a requires process for running the analytics models via Satori.

##### Invoking DBT
Just like [DBT](https://docs.getdbt.com/docs/core/connect-data-platform/connection-profiles), the Satori CLI reads the `dbt_project.yml` file and then looks up the target profile.
It then searches for the `profiles.yml` file in the following order:
1. The `--profiles-dir` command line argument.
2. The `DBT_PROFILES_DIR` environment variable.
3. Current working directory
4. The `~/.dbt/profiles.yml` file.

Same as [DBT selection](https://docs.getdbt.com/docs/core/connect-data-platform/connection-profiles#advanced-customizing-a-profile-directory)

If the `--target` option is used, the Satori CLI then uses the target provided. If it does not use the target provided, it will use the [default](https://docs.getdbt.com/docs/core/connect-data-platform/connection-profiles#setting-up-your-profile) target.

The Satori CLI then changes the username and password to an environment variable that is passed to DBT (`SATORI_USERNAME`, `SATORI_PASSWORD`).
The Satori CLI then creates a backup of the file before modification.

**Arguments**
* `--target` - The target to use. If not provided, the default target will be used.
* `--profiles-dir` - The directory looks for the profiles.yml file. If not provided, the default yml will be used.
* `--` - Pass the rest of the arguments to the tool.

**Examples**
```bash
satori run dbt debug
```

Specify the target
```bash
satori run dbt debug --target dev
```


**Example**
```bash
  satori run psql <datastore-host> <database>
```

### PgPass
Generates a pgpass file from all datastore information.
```bash
satori pgpass
```

### AWS
Generates aws profiles.

Each datastore will have its own profile.
The list of datastores to profile mapping will be printed at the end.

```bash
satori aws
The following profiles have been generated:
    athea-prod: profile satori_athena_939918
    s3-stage: profile satori_s3_438177
```

then you can use aws cli with the profile
```bash
aws s3 ls --profile satori_s3_438177
```

#### Arguments
  - `--refresh` - obtain new credentials and datastores information from the server, even if they already exist in the cache.
  - `--no-launch-browser` - Do not launch the browser to authenticate, instead print the URL to the terminal. 

### Login
Obtain credentials from the Satori data portal without the need to use a browser.

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
