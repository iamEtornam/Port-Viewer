#!/bin/bash
# Installation script for port-viewer

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Detect OS
OS="$(uname -s)"
ARCH="$(uname -m)"

echo -e "${BLUE}Port Viewer Installer${NC}"
echo "────────────────────────────────"

# Check dependencies
if ! command -v lsof &> /dev/null; then
    echo -e "${RED}Error: lsof is not installed${NC}"
    exit 1
fi

if ! command -v ps &> /dev/null; then
    echo -e "${RED}Error: ps is not installed${NC}"
    exit 1
fi

# Determine download URL based on OS and architecture
case "$OS" in
    Darwin)
        if [ "$ARCH" = "arm64" ]; then
            BINARY_NAME="ports-macos-arm64"
        else
            BINARY_NAME="ports-macos-x86_64"
        fi
        ;;
    Linux)
        BINARY_NAME="ports-linux-x86_64"
        ;;
    *)
        echo -e "${RED}Unsupported operating system: $OS${NC}"
        exit 1
        ;;
esac

echo -e "${YELLOW}Detected OS: $OS ($ARCH)${NC}"

# Check if user wants to install from source or binary
if command -v cargo &> /dev/null; then
    echo ""
    read -p "Cargo detected. Install from source? [Y/n] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]] || [[ -z $REPLY ]]; then
        echo -e "${BLUE}Installing from source...${NC}"
        cargo install --git https://github.com/iamEtornam/port-viewer
        echo -e "${GREEN}✓ Installation complete!${NC}"
        echo ""
        echo "Run 'ports --help' to get started"
        exit 0
    fi
fi

# Install from binary
echo -e "${BLUE}Installing binary...${NC}"

LATEST_RELEASE=$(curl -s https://api.github.com/repos/iamEtornam/port-viewer/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_RELEASE" ]; then
    echo -e "${RED}Failed to fetch latest release${NC}"
    exit 1
fi

echo -e "Latest version: ${GREEN}$LATEST_RELEASE${NC}"

DOWNLOAD_URL="https://github.com/iamEtornam/port-viewer/releases/download/$LATEST_RELEASE/$BINARY_NAME.tar.gz"

# Download and extract
TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR"

echo -e "${BLUE}Downloading...${NC}"
curl -LO "$DOWNLOAD_URL"

echo -e "${BLUE}Extracting...${NC}"
tar xzf "$BINARY_NAME.tar.gz"

# Install to /usr/local/bin
echo -e "${BLUE}Installing to /usr/local/bin...${NC}"
sudo mv ports /usr/local/bin/ports
sudo chmod +x /usr/local/bin/ports

# Create alias
echo ""
echo -e "${YELLOW}Optional: Add alias to your shell config${NC}"
echo "Add this line to ~/.zshrc or ~/.bashrc:"
echo -e "${BLUE}  alias whoisonport='ports'${NC}"

echo ""
echo -e "${GREEN}✓ Installation complete!${NC}"
echo ""
echo "Run 'ports --help' to get started"

# Cleanup
cd - > /dev/null
rm -rf "$TEMP_DIR"
