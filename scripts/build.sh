#!/bin/bash

set -e  # Exit on error

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
binPath="$root/bin"
originalPath="$(pwd)"

echo "🚀 Starting build process..."
echo "==> Building backend..."
cd "$root/backend"
cargo build --release

echo "==> Cleaning and creating bin directory..."
rm -rf "$binPath"
mkdir -p "$binPath"

echo "==> Building client frontend..."
cd "$root/client"
pnpm build

echo "==> Moving backend binary to bin folder..."
mv "$root/target/release/backend" "$binPath/backend"

echo "==> Copying config.json to bin directory..."
cp "$root/config.json" "$binPath/config.json"

echo "==> Returning to original path..."
cd "$originalPath"

echo "✅ Build process complete."
