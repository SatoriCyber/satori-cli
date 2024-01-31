#!/bin/bash

# Define variables
APP_NAME="satori"
LATEST_APP_VERSION="0.0.15"
DOWNLOAD_URL_PREFIX="https://github.com/SatoriCyber/satori-cli/releases/download/v$LATEST_APP_VERSION/$APP_NAME-$LATEST_APP_VERSION"

HAS_CURL="$(type "curl" &> /dev/null && echo true || echo false)"
HAS_WGET="$(type "wget" &> /dev/null && echo true || echo false)"

HAS_ZSH="$(type "zsh" &> /dev/null && echo true || echo false)"
HAS_BASH="$(type "bash" &> /dev/null && echo true || echo false)"

# Function to check if the tool is already installed
function isInstalled() {
    command -v "$APP_NAME" &> /dev/null
}

# initArch discovers the architecture for this system.
initArch() {
  ARCH=$(uname -m)
  case $ARCH in
    armv5*) ARCH="armv5";;
    armv6*) ARCH="armv6";;
    armv7*) ARCH="arm";;
    aarch64) ARCH="arm64";;
    x86) ARCH="386";;
    x86_64) ARCH="amd64";;
    i686) ARCH="386";;
    i386) ARCH="386";;
  esac
}

function initOS() {
  OS=$(echo `uname`|tr '[:upper:]' '[:lower:]')
}

function verifySupported() {
    local supported="darwin-amd64\ndarwin-arm64\nlinux-amd64\n"
    if ! echo "${supported}" | grep -q "${OS}-${ARCH}"; then
        echo "No prebuilt binary for ${OS}-${ARCH}."
        echo "Open an issue and ask for support at https://github.com/SatoriCyber/satori-cli/issues/new add your OS/arch to the issue title."
        exit 1
    fi    
}

function downloadUrl() {
    if [ "$OS" == "darwin" ]; then
        DOWNLOAD_URL="$DOWNLOAD_URL_PREFIX-macOS.tar.gz"
    elif [ "$OS" == "linux" ]; then
        DOWNLOAD_URL="$DOWNLOAD_URL_PREFIX-linux.tar.gz"
    fi
    echo "download url: $DOWNLOAD_URL"
}

function downloadCommand() {
    if [ "$HAS_CURL" == "true" ]; then
        DOWNLOAD_COMMAND="curl -L"
    elif [ "$HAS_WGET" == "true" ]; then
        DOWNLOAD_COMMAND="wget -O -"
    else
        die "Either curl or wget is required for downloading. Install one of them and run the script again."
    fi
}

function shellFile() {
    # Check for zsh or bash availability
    if [ "$HAS_ZSH" == "true" ]; then
        SHELL_CONFIG_FILE="$HOME/.zshrc"
    elif [ "$HAS_BASH" == "true" ]; then
        SHELL_CONFIG_FILE="$HOME/.bashrc"
    else
        die "Either zsh or bash is required for configuring the shell. Install one of them and run the script again."
    fi
}

function installDir() {
    # Check if the script is run with root privileges
    if [ "$EUID" -eq 0 ]; then
        echo "Running with root privileges."
        INSTALL_DIR="/usr/local/bin"
    else
        echo "Not running with root privileges. Installing in user's home directory."
        INSTALL_DIR="$HOME/bin"
    fi
}

function createInstallDir() {
    # Create installation directory
    mkdir -p "$INSTALL_DIR" || die "Failed to create installation directory."
}

function downloadFile() {
    # Download and extract the Satori CLI
    $DOWNLOAD_COMMAND "$DOWNLOAD_URL" | tar -xz -C "$INSTALL_DIR" || die "Failed to download and extract $APP_NAME."
}

function updateProfileFile() {
    # Add the installation directory to the PATH
    echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_CONFIG_FILE" || die "Failed to update $SHELL_CONFIG_FILE."
    # Add auto-complete
    if [ "$HAS_ZSH" == "true" ]; then
        local suffix="zsh"
        echo "source $INSTALL_DIR/satori_auto_complete.zsh" >> "$SHELL_CONFIG_FILE" || die "Failed to add auto-complete to $SHELL_CONFIG_FILE."
        echo "Please restart your shell or run 'source ~/.zshrc' to update the PATH."
    elif [ "$HAS_BASH" == "true" ]; then
        local suffix="sh"
        echo "source $INSTALL_DIR/satori_auto_complete.sh" >> "$SHELL_CONFIG_FILE" || die "Failed to add auto-complete to $SHELL_CONFIG_FILE."
        echo "Please restart your shell or run 'source ~/.bashrc' to update the PATH."
    fi
    
    
}

# Function to display an error and exit
function die {
    echo "Error: $1" >&2
    exit 1
}

echo "Installing $APP_NAME-$LATEST_APP_VERSION..."
initArch
initOS
verifySupported
downloadCommand
shellFile
installDir
createInstallDir
downloadUrl
downloadFile




if isInstalled; then
    echo "The existing $APP_NAME binary has been replaced successfully."
else
    updateProfileFile
fi

# Provide user feedback
echo "You can run it using the command: $APP_NAME"



