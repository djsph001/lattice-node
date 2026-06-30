#!/usr/bin/env bash
# Lattice node setup — Linux (Ubuntu/Debian)
# Run from the repo root
set -euo pipefail

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Lattice Node — Linux Setup"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Rust toolchain
if command -v rustup >/dev/null 2>&1; then
    echo "✓ Rust toolchain found"
else
    echo "Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Cross-compilation target for Raspberry Pi (optional)
if ! rustup target list --installed | grep -q aarch64-unknown-linux-gnu; then
    echo ""
    echo "Enabling Raspberry Pi cross-compilation target..."
    rustup target add aarch64-unknown-linux-gnu
    if ! command -v aarch64-linux-gnu-gcc >/dev/null 2>&1; then
        echo "Installing aarch64 cross-linker..."
        sudo apt-get install -y gcc-aarch64-linux-gnu 2>/dev/null || {
            echo "⚠️  Could not install aarch64 cross-linker."
            echo "   Pi cross-compilation won't work until it's installed:"
            echo "     sudo apt install gcc-aarch64-linux-gnu"
        }
    fi
fi

echo ""
echo "Building lattice-node (release)..."
cargo build --release

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Setup complete."
echo ""
echo "  Binary: target/release/lattice-node"
echo ""
echo "  Quick start:"
echo "    ./target/release/lattice-node --name z4 --heartbeat-interval 5"
echo ""
echo "  Join a remote node:"
echo "    ./target/release/lattice-node --name z4 --no-mdns \\"
echo "      --bootstrap-peer /dns4/<host>/tcp/<port>/p2p/<peer_id>"
echo ""
echo "  Cross-compile for Pi:"
echo "    cargo build --release --target aarch64-unknown-linux-gnu"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
