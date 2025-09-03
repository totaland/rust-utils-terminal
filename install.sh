#!/bin/bash

# Install script for shell explorer utility

set -e

echo "🔧 Shell Explorer Installer"
echo "=========================="

# Build the project
echo "📦 Building the project..."
cargo build --release

# Check if build was successful
if [ ! -f "target/release/shell-explorer" ]; then
    echo "❌ Build failed! Please check the errors above."
    exit 1
fi

echo "✅ Build successful!"

# Ask user where to install
echo ""
echo "Where would you like to install the shell-explorer?"
echo "1) /usr/local/bin (requires sudo, available system-wide)"
echo "2) ~/.local/bin (user only, make sure it's in your PATH)"
echo "3) Custom path"
echo "4) Don't install, just show the path"

read -p "Choose an option (1-4): " choice

case $choice in
    1)
        echo "Installing to /usr/local/bin..."
        sudo cp target/release/shell-explorer /usr/local/bin/shell-explorer
        sudo chmod +x /usr/local/bin/shell-explorer
        echo "✅ Installed as 'shell-explorer'"
        echo "Usage: shell-explorer --mode packages --package react --min-version 17.0.0"
        ;;
    2)
        mkdir -p ~/.local/bin
        cp target/release/shell-explorer ~/.local/bin/shell-explorer
        chmod +x ~/.local/bin/shell-explorer
        echo "✅ Installed to ~/.local/bin/shell-explorer"
        echo "Make sure ~/.local/bin is in your PATH:"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
        echo "Usage: shell-explorer --mode packages --package react --min-version 17.0.0"
        ;;
    3)
        read -p "Enter custom installation path: " custom_path
        if [ -d "$custom_path" ]; then
            cp target/release/shell-explorer "$custom_path/shell-explorer"
            chmod +x "$custom_path/shell-explorer"
            echo "✅ Installed to $custom_path/shell-explorer"
        else
            echo "❌ Directory doesn't exist: $custom_path"
            exit 1
        fi
        ;;
    4)
        echo "📍 Binary location: $(pwd)/target/release/shell-explorer"
        echo "Usage: $(pwd)/target/release/shell-explorer --mode packages --package react --min-version 17.0.0"
        ;;
    *)
        echo "❌ Invalid choice"
        exit 1
        ;;
esac

echo ""
echo "🎉 Installation complete!"
echo ""
echo "Try these commands:"
echo "  # Show all aliases (default mode)"
echo "  shell-explorer"
echo ""
echo "  # Show all functions"
echo "  shell-explorer --mode functions"
echo ""
echo "  # Filter functions containing 'git'"
echo "  shell-explorer --mode functions --filter git"
echo ""
echo "  # Find React packages with version > 17.0.0"
echo "  shell-explorer --mode packages --package react --min-version 17.0.0"
echo ""
echo "  # Find TypeScript packages with version > 4.0.0 in specific path"
echo "  shell-explorer --mode packages --package typescript --min-version 4.0.0 --path ./src"
echo ""
echo "  # Get help"
echo "  shell-explorer --help"
