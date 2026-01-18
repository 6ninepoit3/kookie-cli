#!/bin/bash
# Kookie CLI Installer for Linux/macOS
# Run: curl -sSL https://raw.githubusercontent.com/yourusername/kookie-cli/main/install.sh | bash

set -e

INSTALL_DIR="${HOME}/.local/bin"
BINARY_NAME="kookie"

echo ""
echo "🍪 Kookie CLI Installer"
echo "========================"
echo ""

# Create install directory
mkdir -p "$INSTALL_DIR"

# Determine source binary
SOURCE_BINARY=""

# Check if running from repo with built binary
REPO_BINARY="$(dirname "$0")/target/release/kookie"
if [ -f "$REPO_BINARY" ]; then
    SOURCE_BINARY="$REPO_BINARY"
    echo "📦 Found local build: $REPO_BINARY"
fi

# If no local binary, check current directory
if [ -z "$SOURCE_BINARY" ] && [ -f "./kookie" ]; then
    SOURCE_BINARY="./kookie"
    echo "📦 Found binary in current directory"
fi

# If still no binary found
if [ -z "$SOURCE_BINARY" ]; then
    echo "❌ Error: kookie binary not found."
    echo ""
    echo "Please build from source first:"
    echo "  cargo build --release"
    echo ""
    echo "Or place the kookie binary in the current directory."
    exit 1
fi

# Copy binary
DEST_BINARY="$INSTALL_DIR/$BINARY_NAME"
echo "📋 Installing to: $DEST_BINARY"
cp "$SOURCE_BINARY" "$DEST_BINARY"
chmod +x "$DEST_BINARY"

# Add to PATH in shell config
add_to_path() {
    local config_file="$1"
    local path_line="export PATH=\"\$HOME/.local/bin:\$PATH\""
    
    if [ -f "$config_file" ]; then
        if ! grep -q ".local/bin" "$config_file" 2>/dev/null; then
            echo "" >> "$config_file"
            echo "# Kookie CLI" >> "$config_file"
            echo "$path_line" >> "$config_file"
            echo "✅ Added to $config_file"
            return 0
        fi
    fi
    return 1
}

# Detect shell and update appropriate config
SHELL_NAME=$(basename "$SHELL")
UPDATED_CONFIG=""

case "$SHELL_NAME" in
    zsh)
        if add_to_path "$HOME/.zshrc"; then
            UPDATED_CONFIG="~/.zshrc"
        fi
        ;;
    bash)
        if add_to_path "$HOME/.bashrc"; then
            UPDATED_CONFIG="~/.bashrc"
        elif add_to_path "$HOME/.bash_profile"; then
            UPDATED_CONFIG="~/.bash_profile"
        fi
        ;;
    fish)
        FISH_CONFIG="$HOME/.config/fish/config.fish"
        if [ -f "$FISH_CONFIG" ]; then
            if ! grep -q ".local/bin" "$FISH_CONFIG" 2>/dev/null; then
                echo "" >> "$FISH_CONFIG"
                echo "# Kookie CLI" >> "$FISH_CONFIG"
                echo "set -gx PATH \$HOME/.local/bin \$PATH" >> "$FISH_CONFIG"
                UPDATED_CONFIG="$FISH_CONFIG"
                echo "✅ Added to $FISH_CONFIG"
            fi
        fi
        ;;
esac

if [ -z "$UPDATED_CONFIG" ]; then
    if echo "$PATH" | grep -q ".local/bin"; then
        echo "✅ Already in PATH"
    else
        echo "⚠️  Please add ~/.local/bin to your PATH manually"
    fi
fi

# Update current session
export PATH="$HOME/.local/bin:$PATH"

# Verify installation
echo ""
echo "🔍 Verifying installation..."
if command -v kookie &> /dev/null; then
    VERSION=$(kookie --version 2>&1)
    echo "✅ $VERSION"
else
    echo "⚠️  Could not verify installation"
fi

echo ""
echo "🎉 Installation complete!"
echo ""
echo "Get started:"
echo "  kookie init            # Initialize your vault"
echo "  kookie add --password  # Add a password"
echo "  kookie --help          # See all commands"
echo ""
if [ -n "$UPDATED_CONFIG" ]; then
    echo "⚠️  NOTE: Run 'source $UPDATED_CONFIG' or restart your terminal."
fi
echo ""
