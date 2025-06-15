#!/bin/sh

set -e

URL="https://github.com/manavsu/tanki/releases/download/v0.1.0/tanki"
DEST="/usr/local/bin/tanki"

echo "Downloading Tanki..."
curl -L "$URL" -o tanki

echo "Making Tanki executable..."
chmod +x tanki

echo "Installing to $DEST (requires sudo)..."
sudo mv tanki "$DEST"

echo "Tanki installed successfully! Run with: tanki"
