#!/bin/sh
set -e

# Detect OS and architecture
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
  Linux)
    case "$ARCH" in
      x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    EXT="tar.gz"
    ;;
  *)
    echo "Unsupported OS: $OS"; exit 1
    ;;
esac

BINARY_NAME="forge_arch"
INSTALL_DIR="$HOME/.local/bin"
DOWNLOAD_URL="https://github.com/iamjuaness/ForgeArch/releases/latest/download/forge_arch-${TARGET}.${EXT}"

echo "Installing ForgeArch for ${TARGET}..."
mkdir -p "$INSTALL_DIR"

# Download and extract
echo "Downloading from ${DOWNLOAD_URL}..."
if ! curl -fsSL "$DOWNLOAD_URL" | tar xz -C "$INSTALL_DIR"; then
  echo "Error: Failed to download or extract. Check if the release exists."
  exit 1
fi

# Rename if needed
if [ -f "$INSTALL_DIR/$BINARY_NAME" ]; then
  mv "$INSTALL_DIR/$BINARY_NAME" "$INSTALL_DIR/forge"
fi

chmod +x "$INSTALL_DIR/forge"

echo "✓ ForgeArch installed successfully to $INSTALL_DIR/forge"
echo ""

# Configure PATH automatically
PATH_EXPORT="export PATH=\"\$HOME/.local/bin:\$PATH\""
CONFIGURED=0

# Detect shell and config file
if [ -n "$ZSH_VERSION" ] || [ -f "$HOME/.zshrc" ]; then
  SHELL_CONFIG="$HOME/.zshrc"
  SHELL_NAME="zsh"
elif [ -n "$BASH_VERSION" ] || [ -f "$HOME/.bashrc" ]; then
  SHELL_CONFIG="$HOME/.bashrc"
  SHELL_NAME="bash"
elif [ -f "$HOME/.profile" ]; then
  SHELL_CONFIG="$HOME/.profile"
  SHELL_NAME="profile"
else
  SHELL_CONFIG=""
fi

# Add to shell config if not already present
if [ -n "$SHELL_CONFIG" ]; then
  if ! grep -q "$HOME/.local/bin" "$SHELL_CONFIG" 2>/dev/null; then
    echo "" >> "$SHELL_CONFIG"
    echo "# Added by ForgeArch installer" >> "$SHELL_CONFIG"
    echo "$PATH_EXPORT" >> "$SHELL_CONFIG"
    echo "✓ Added $INSTALL_DIR to PATH in $SHELL_CONFIG"
    CONFIGURED=1
  else
    echo "✓ PATH already configured in $SHELL_CONFIG"
    CONFIGURED=1
  fi
fi

# Export for current session
export PATH="$HOME/.local/bin:$PATH"

echo ""
if [ $CONFIGURED -eq 1 ]; then
  echo "🎉 Installation complete! PATH configured automatically."
  echo ""
  echo "To use 'forge' in your current terminal, run:"
  echo "  source $SHELL_CONFIG"
  echo ""
  echo "Or simply open a new terminal."
else
  echo "⚠️  Could not detect shell config file."
  echo "Please add this line manually to your shell config:"
  echo "  $PATH_EXPORT"
fi

echo ""
echo "Usage: forge new my-project --arch=backend-api"
echo "Run 'forge --help' for more options."
