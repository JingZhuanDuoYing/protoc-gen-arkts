#!/usr/bin/env bash

set -e

# Repository information
REPO="JingZhuanDuoYing/protoc-gen-arkts"
BINARY_NAME="protoc-gen-arkts"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Installing latest $BINARY_NAME...${NC}"

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     OS_NAME="linux";;
    Darwin*)    OS_NAME="macos";;
    CYGWIN*|MINGW*|MSYS*) OS_NAME="windows";;
    *)          echo -e "${RED}Unsupported OS: ${OS}${NC}" && exit 1;;
esac

# Detect Architecture
ARCH="$(uname -m)"
case "${ARCH}" in
    x86_64|amd64) ARCH_NAME="x86_64";;
    arm64|aarch64) ARCH_NAME="aarch64";;
    *)             echo -e "${RED}Unsupported architecture: ${ARCH}${NC}" && exit 1;;
esac

# Determine asset name based on OS and Architecture
if [ "$OS_NAME" = "windows" ]; then
    ASSET_NAME="${BINARY_NAME}-${OS_NAME}-${ARCH_NAME}.exe"
    FINAL_BINARY="${BINARY_NAME}.exe"
else
    ASSET_NAME="${BINARY_NAME}-${OS_NAME}-${ARCH_NAME}"
    FINAL_BINARY="${BINARY_NAME}"
fi

# We don't have a linux-aarch64 or windows-aarch64 build currently configured in GitHub Actions
if [ "$OS_NAME" = "linux" ] && [ "$ARCH_NAME" = "aarch64" ]; then
    echo -e "${RED}Linux aarch64 is currently not provided in the pre-built binaries.${NC}"
    echo "Please build from source using: cargo build --release"
    exit 1
fi

if [ "$OS_NAME" = "windows" ] && [ "$ARCH_NAME" = "aarch64" ]; then
    echo -e "${RED}Windows aarch64 is currently not provided in the pre-built binaries.${NC}"
    echo "Please build from source using: cargo build --release"
    exit 1
fi

# Fetch the latest release tag from GitHub API
LATEST_RELEASE_URL="https://api.github.com/repos/${REPO}/releases/latest"
echo "Fetching latest release information..."
TAG=$(curl -sL "$LATEST_RELEASE_URL" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$TAG" ]; then
    echo -e "${RED}Failed to fetch the latest release tag. Please check your network or repository visibility.${NC}"
    exit 1
fi

echo -e "Latest version: ${GREEN}${TAG}${NC}"

# Construct the download URL
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${TAG}/${ASSET_NAME}"
echo "Downloading from: $DOWNLOAD_URL"

# Create a temporary directory for downloading
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

# Download the binary
if curl -sL -f -o "$FINAL_BINARY" "$DOWNLOAD_URL"; then
    echo "Download successful."
else
    echo -e "${RED}Failed to download the binary. The asset $ASSET_NAME might not exist for version $TAG.${NC}"
    rm -rf "$TMP_DIR"
    exit 1
fi

# Make it executable
chmod +x "$FINAL_BINARY"

# Determine installation directory
INSTALL_DIR="/usr/local/bin"

if [ -w "$INSTALL_DIR" ]; then
    mv "$FINAL_BINARY" "$INSTALL_DIR/"
    echo -e "${GREEN}Successfully installed $BINARY_NAME to $INSTALL_DIR/$FINAL_BINARY${NC}"
else
    echo "Installation directory $INSTALL_DIR is not writable by current user."
    echo "Requesting sudo privileges to move the binary..."
    if sudo mv "$FINAL_BINARY" "$INSTALL_DIR/"; then
        echo -e "${GREEN}Successfully installed $BINARY_NAME to $INSTALL_DIR/$FINAL_BINARY${NC}"
    else
        echo -e "${RED}Installation failed. You can manually copy the binary from $TMP_DIR/$FINAL_BINARY to your PATH.${NC}"
        exit 1
    fi
fi

# Clean up
rm -rf "$TMP_DIR"

echo -e "\n${BLUE}Installation Complete!${NC}"
echo -e "You can now run: ${GREEN}$BINARY_NAME --version${NC}"
