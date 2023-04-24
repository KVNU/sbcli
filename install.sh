#!/bin/bash

set -e

REPO_OWNER="KVNU"
REPO_NAME="sbcli"
BINARY_NAME="sbcli"

# Detect the platform
platform=""
case "$(uname -s)" in
  Darwin)
    platform="x86_64-apple-darwin"
    ;;
  Linux)
    platform="x86_64-unknown-linux-gnu"
    ;;
  MINGW* | MSYS* | CYGWIN*)
    platform="x86_64-pc-windows-msvc"
    ;;
  *)
    echo "Unsupported platform: $(uname -s)"
    exit 1
    ;;
esac

echo "Platform detected: $platform"

# Fetch the latest release version
latest_release_url="https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases/latest"
latest_release_tag=$(curl -sL $latest_release_url | jq -r '.tag_name')
# latest_release_tag=$(curl -sL $latest_release_url | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
echo "Latest release tag: $latest_release_tag"

# Download the binary for the detected platform
binary_url="https://github.com/$REPO_OWNER/$REPO_NAME/releases/download/$latest_release_tag/$BINARY_NAME-$platform"

if [[ $platform == "x86_64-pc-windows-msvc" ]]; then
  binary_url+=".exe"
fi

echo "Downloading binary from: $binary_url"
curl -sL -o $BINARY_NAME $binary_url
chmod +x $BINARY_NAME

# Prompt the user to move the binary to PATH
read -p "Do you want to move the binary to your PATH? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
  if [[ $platform == "x86_64-pc-windows-msvc" ]]; then
    install_dir="$HOME/.local/bin"
    mkdir -p "$install_dir"
  else
    install_dir="/usr/local/bin"
  fi

  # Check if the user has write permissions to the target directory
  if [[ -w $install_dir ]]; then
    echo "Moving binary to $install_dir"
    mv $BINARY_NAME $install_dir
    echo "Installation completed. The binary is located at $install_dir/$BINARY_NAME"
  else
    echo "Error: You don't have write permissions to $install_dir."
    echo "Please run this script with administrative privileges or move the binary manually."
    exit 1
  fi
else
  echo "Installation completed. The binary is located at ./$BINARY_NAME"
fi
