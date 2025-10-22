#!/usr/bin/env bash
set -e

echo "Installing DAD (Dumbass Daemon) service..."

# Build Rust components
echo "Building Rust daemon..."
cargo build --release

# Copy binaries
echo "Installing binaries..."
sudo cp target/release/dad-daemon /usr/local/bin/
sudo cp target/release/dad /usr/local/bin/
sudo chmod +x /usr/local/bin/dad-daemon /usr/local/bin/dad

# Install Python ML sidecar
echo "Installing Python ML sidecar..."
cd ml-sidecar
pip3 install -e .
cd ..

# Create data directory
echo "Creating data directory..."
mkdir -p ~/.local/share/dad

# Install systemd service (user mode)
echo "Installing systemd service..."
mkdir -p ~/.config/systemd/user
cp scripts/systemd/dad.service ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable dad.service

echo ""
echo "✅ DAD installation complete!"
echo ""
echo "To start the daemon:"
echo "  systemctl --user start dad"
echo ""
echo "To enable shell hooks, add to your ~/.bashrc:"
echo "  source $(pwd)/scripts/shell-hooks/bash-hook.sh"
echo ""
echo "To verify installation:"
echo "  dad audit"
