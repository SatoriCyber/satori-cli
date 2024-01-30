#!/bin/bash

# Define variables
APP_NAME="satori"
LATEST_APP_VERSION="0.0.14"
DOWNLOAD_URL="https://github.com/SatoriCyber/satori-cli/releases/download/v$LATEST_APP_VERSION/$APP_NAME-$LATEST_APP_VERSION-linux.tar.gz"

# Check if the script is run with root privileges
if [ "$EUID" -eq 0 ]; then
    echo "Running with root privileges."
    INSTALL_DIR="/usr/local/bin"
else
    echo "Not running with root privileges. Installing in user's home directory."
    INSTALL_DIR="$HOME/bin"
fi

# Function to get the currently installed version from satori --version
get_installed_version() {
    local installed_version
    installed_version=$(satori --version | awk '{print $2}')
    echo "$installed_version"
}

# Function to display an error and exit
function die {
    echo "Error: $1" >&2
    exit 1
}

# Create installation directory
mkdir -p "$INSTALL_DIR" || die "Failed to create installation directory."

# Get the currently installed version
CURRENT_VERSION=$(get_installed_version)

echo "Installing $APP_NAME-$LATEST_APP_VERSION..."

# Download and extract the Satori CLI
curl -L "$DOWNLOAD_URL" | tar -xz -C "$INSTALL_DIR" || die "Failed to download and extract $APP_NAME."

# Replace the existing binary
ln -sf "$INSTALL_DIR/$APP_NAME-$LATEST_APP_VERSION" "$INSTALL_DIR/$APP_NAME" || die "Failed to replace existing binary."

# Provide user feedback
echo "$APP_NAME-$LATEST_APP_VERSION has been installed and replaced successfully in $INSTALL_DIR."

# Add the user-specific bin directory to the user's PATH
echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$HOME/.bashrc" || die "Failed to update .bashrc."

# Add auto-complete to bashrc
echo "[ -f $INSTALL_DIR/satori_auto_complete.sh ] && source $INSTALL_DIR/satori_auto_complete.sh" >> "$HOME/.bashrc" || die "Failed to add auto-complete to .bashrc."


# Provide user feedback
echo "You can run it using the command: $APP_NAME"
echo "Please restart your shell or run 'source ~/.bashrc' to update the PATH."
