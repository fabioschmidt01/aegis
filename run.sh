#!/bin/bash

# Navigate to the script's directory (project root)
cd "$(dirname "$0")"

# Source Rust environment for the current user
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "Check: Cargo not found in current user path."
    # Try generic locations
    export PATH="$HOME/.cargo/bin:$PATH"
fi

echo "Starting Anonsurf GUI..."
echo "Note: Asking for sudo password to modify network rules (iptables)."

# Run npm run tauri dev with the current PATH preserved
# We imply usage of 'npm' from the user's path or system path
sudo -E env "PATH=$PATH" npm run tauri dev
