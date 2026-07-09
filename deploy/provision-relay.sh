#!/usr/bin/env bash
# Lattice relay hub — VPS provisioning script
# Run ON the VPS, as root or a sudo-capable user
# Usage: curl -sSL <raw-url> | bash
#    or: scp to VPS and run: bash provision-relay.sh
set -euo pipefail

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Lattice Relay Hub — VPS Provisioning"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# ── System dependencies ────────────────────
echo "► Installing system dependencies..."
sudo apt-get update -qq
sudo apt-get install -y -qq build-essential pkg-config libssl-dev curl git

# ── Create lattice user ────────────────────
if ! id lattice &>/dev/null; then
    echo "► Creating 'lattice' user..."
    sudo useradd -r -s /usr/sbin/nologin -d /opt/lattice-node lattice
fi

# ── Clone and build ────────────────────────
REPO="https://github.com/djsph001/lattice-node.git"
BRANCH="main"

if [ -d /opt/lattice-node/.git ]; then
    echo "► Repo exists, pulling latest..."
    cd /opt/lattice-node
    sudo -u lattice git pull origin "$BRANCH"
else
    echo "► Cloning lattice-node..."
    sudo mkdir -p /opt/lattice-node
    sudo chown lattice:lattice /opt/lattice-node
    sudo -u lattice git clone --branch "$BRANCH" "$REPO" /opt/lattice-node
    cd /opt/lattice-node
fi

# ── Rust toolchain ─────────────────────────
if ! command -v rustup >/dev/null 2>&1; then
    echo "► Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# ── Build ──────────────────────────────────
echo "► Building lattice-node (release)..."
cd /opt/lattice-node
sudo -u lattice bash -c 'source "$HOME/.cargo/env" && cargo build --release'

# ── Data dir ───────────────────────────────
echo "► Creating data directory..."
sudo mkdir -p /opt/lattice-node/data
sudo chown lattice:lattice /opt/lattice-node/data

# ── Install systemd service ────────────────
echo "► Installing systemd service..."
sudo cp /opt/lattice-node/deploy/lattice-relay.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable lattice-relay
sudo systemctl start lattice-relay

# ── Verify ─────────────────────────────────
sleep 2
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Service status:"
sudo systemctl status lattice-relay --no-pager --lines=5 || true
echo ""
echo "► Relay Peer ID:"
PEER_ID=$(sudo journalctl -u lattice-relay --no-pager -n 30 2>/dev/null | grep -oP 'PeerId:\s*\K\S+' | tail -1 || true)
if [ -n "$PEER_ID" ]; then
    echo "  $PEER_ID"
    echo ""
    echo "  Bootstrap multiaddr:"
    echo "  /dns4/$(curl -sS ifconfig.me)/tcp/4001/p2p/$PEER_ID"
else
    echo "  (check journal: sudo journalctl -u lattice-relay -f)"
fi
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
