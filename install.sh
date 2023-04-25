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

if command -v jq >/dev/null 2>&1; then
  # Use jq if available
  latest_release_tag=$(curl -sL $latest_release_url | jq -r '.tag_name')
else
  # Fallback to grep and sed
  latest_release_tag=$(curl -sL $latest_release_url | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
fi

echo "Latest release tag: $latest_release_tag"

# Download the binary for the detected platform
binary_url="https://github.com/$REPO_OWNER/$REPO_NAME/releases/download/$latest_release_tag/$BINARY_NAME-$platform"

if [[ $platform == "x86_64-pc-windows-msvc" ]]; then
  binary_url+=".exe"
fi

echo "Downloading binary from: $binary_url"
curl -sL -o $BINARY_NAME $binary_url
chmod +x $BINARY_NAME

# Try to move the binary to a directory in PATH
#if [[ $platform == "x86_64-pc-windows-msvc" ]]; then
  # install_dir="C:/Program Files/$REPO_NAME"
  # mkdir -p "$install_dir"
  # echo "Moving binary to $install_dir"

  # if mv $BINARY_NAME $install_dir 2>/dev/null; then
  #   echo "Installation completed. The binary is located at $install_dir/$BINARY_NAME"
  # else
  #   echo "Failed to move binary. Please run this script as an administrator."
  # fi

#else
  # install_dir="/usr/local/bin"
  # echo "Moving binary to $install_dir with elevated privileges"
  # sudo mv $BINARY_NAME $install_dir
  install_dir="$HOME/.local/bin"
  echo "Moving binary to $install_dir"
  mkdir -p "$install_dir"
  mv $BINARY_NAME $install_dir
  echo "Installation completed. The binary is located at $install_dir/$BINARY_NAME"
  echo "Please ensure that $install_dir is in PATH."
#fi
