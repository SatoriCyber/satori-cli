# satori-cli
Allow to interact with Satori data portal from the command line.

## Usage
### Login
Allow to obtain credentials from Satori data portal without the need to use a browser.

```bash
satori login --display
```
This will display the credentials in the terminal. 

You can also trigger the script in order to port it to your own tool, the output format can 
be adjusted using the `--format` flag. The following formats are supported:
- `csv` (default)
- `json`
- `yaml`

## Installation
### Mac brew
```bash
brew tap satoricyber/satori
brew install satori_cli
```