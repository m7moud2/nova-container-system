#!/bin/bash
# Nova Container System - Installation Script
# Installs Nova on macOS and Linux

set -e

echo "🚀 Installing Nova Container System..."
echo ""

# Detect OS and Architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Darwin)
        PLATFORM="macos"
        ;;
    Linux)
        PLATFORM="linux"
        ;;
    *)
        echo "❌ Unsupported operating system: $OS"
        echo "   Nova currently supports macOS and Linux"
        exit 1
        ;;
esac

echo "📋 Detected: $PLATFORM ($ARCH)"
echo ""

# GitHub repository (update this with your username)
GITHUB_USER="YOUR_USERNAME"
REPO="nova-container-system"

# Download URL
DOWNLOAD_URL="https://github.com/$GITHUB_USER/$REPO/releases/latest/download/nova-$PLATFORM-$ARCH"

# Download binary
echo "📥 Downloading Nova from GitHub..."
if command -v curl &> /dev/null; then
    curl -L "$DOWNLOAD_URL" -o /tmp/nova
elif command -v wget &> /dev/null; then
    wget "$DOWNLOAD_URL" -O /tmp/nova
else
    echo "❌ Error: curl or wget is required"
    exit 1
fi

# Make executable
chmod +x /tmp/nova

# Install to /usr/local/bin
echo "📦 Installing to /usr/local/bin/nova..."
if [ -w /usr/local/bin ]; then
    mv /tmp/nova /usr/local/bin/nova
else
    echo "   (requires sudo)"
    sudo mv /tmp/nova /usr/local/bin/nova
fi

# Verify installation
if command -v nova &> /dev/null; then
    echo ""
    echo "✅ Nova installed successfully!"
    echo ""
    echo "📖 Quick Start:"
    echo "   nova run app.py       # Run Python app"
    echo "   nova run server.js    # Run Node.js app"
    echo "   nova run app.wasm     # Run WebAssembly"
    echo ""
    echo "📚 Learn more:"
    echo "   https://github.com/$GITHUB_USER/$REPO"
    echo ""
else
    echo "❌ Installation failed"
    exit 1
fi
