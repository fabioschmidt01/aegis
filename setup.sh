#!/bin/bash
set -e

echo "Defined dependencies..."
# Update apt
sudo apt-get update

# Install System Dependencies for Tauri & Anonsurf & Stealth Modules
echo "Installing System Dependencies..."
# Note: macchanger usually prompts for configuration. DEBIAN_FRONTEND=noninteractive avoids this if possible, 
# or we let the user handle it.
sudo apt-get install -y build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libwebkit2gtk-4.1-dev tor iptables macchanger

# Install Rust if not present
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust is already installed."
fi

# Install Node.js (Latest LTS) if not present
if ! command -v node &> /dev/null; then
    echo "Installing Node.js..."
    curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
    sudo apt-get install -y nodejs
else
    echo "Node.js is already installed."
fi

echo "Environment Setup Complete!"
echo "Please run: source $HOME/.cargo/env"
