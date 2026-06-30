#!/usr/bin/env bash
# Lattice node setup — macOS (Apple Silicon / Intel)
# Run from the repo root
set -euo pipefail

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Lattice Node — macOS Setup"
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

# Xcode command line tools (provides linker)
if ! xcode-select -p >/dev/null 2>&1; then
    echo "Installing Xcode command line tools..."
    xcode-select --install 2>/dev/null || true
    echo "⚠️  Follow the macOS prompt to install command line tools,"
    echo "   then re-run this script."
    exit 0
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
echo "  Join a remote node:"
echo "    ./target/release/lattice-node --name mac --no-mdns \\"
echo "      --bootstrap-peer /dns4/<host>/tcp/<port>/p2p/<peer_id>"
echo ""
echo "  Behind NAT? Advertise your external address:"
echo "    ./target/release/lattice-node --name mac --no-mdns \\"
echo "      --external-addr /ip4/<public-ip>/tcp/6001 \\"
echo "      --bootstrap-peer /dns4/<host>/tcp/<port>/p2p/<peer_id>"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
