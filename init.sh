#!/usr/bin/env bash
# init.sh — Universal Ingress Pipeline for Emergence Collective Lattice
#
# One-liner onboarding for any machine (Linux/Mac, x86_64/aarch64):
#   curl -fsSL https://emergencecollective.live/init.sh | bash
#
# Or with explicit bootstrap target:
#   curl -fsSL https://emergencecollective.live/init.sh | bash -s -- \\
#     --bootstrap /ip4/100.64.0.1/tcp/6001/p2p/12D3KooW...
#
# What this script does:
#   1. Fingerprints the local OS and architecture
#   2. Downloads the correct pre-compiled lattice-node binary
#   3. Creates an isolated sandbox at ~/.lattice/
#   4. Executes the node, routing to the configured bootstrap peer
#
# ─── B3 BLOCKER ──────────────────────────────────────────────
# The binary distribution pipeline is not yet live. Until CI/CD
# publishes per-target binaries to the BINARY_URL_BASE, this
# script will fail at the download step. The lattice-node must
# be compiled locally or distributed via another channel.
# ─────────────────────────────────────────────────────────────

set -euo pipefail

# ── Configuration ────────────────────────────────────────────
# Override these by setting environment variables before running,
# or pass --bootstrap <addr> as the first argument.

BOOTSTRAP_ADDRESS="${LATTICE_BOOTSTRAP:-}"
BINARY_URL_BASE="${LATTICE_BINARY_URL_BASE:-https://emergencecollective.live/binaries}"
LATTICE_DIR="${LATTICE_DIR:-$HOME/.lattice}"

# Parse CLI args
while [[ $# -gt 0 ]]; do
    case "$1" in
        --bootstrap)
            BOOTSTRAP_ADDRESS="$2"
            shift 2
            ;;
        --binary-url-base)
            BINARY_URL_BASE="$2"
            shift 2
            ;;
        *)
            echo "Unknown argument: $1"
            echo "Usage: $0 [--bootstrap <multiaddr>] [--binary-url-base <url>]"
            exit 1
            ;;
    esac
done

echo "🌐 Emergence Lattice — Universal Node Ingress"
echo ""

# ── Platform Architecture Detection ──────────────────────────
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)
        if [ "$ARCH" = "x86_64" ]; then
            TARGET="x86_64-unknown-linux-gnu"
        elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "armv8l" ]; then
            TARGET="aarch64-unknown-linux-gnu"
        else
            echo "❌ Unsupported Linux architecture: $ARCH"
            exit 1
        fi
        ;;
    Darwin)
        if [ "$ARCH" = "arm64" ]; then
            TARGET="aarch64-apple-darwin"
        elif [ "$ARCH" = "x86_64" ]; then
            TARGET="x86_64-apple-darwin"
        else
            echo "❌ Unsupported macOS architecture: $ARCH"
            exit 1
        fi
        ;;
    MINGW*|MSYS*|CYGWIN*)
        echo "❌ Windows detected. Use PowerShell onboarding script instead."
        exit 1
        ;;
    *)
        echo "❌ Unsupported OS: $OS"
        exit 1
        ;;
esac

echo "   Detected: $TARGET"

# ── Bootstrap Address Validation ─────────────────────────────
if [ -z "$BOOTSTRAP_ADDRESS" ]; then
    echo ""
    echo "⚠️  No bootstrap address provided."
    echo "   The node will start in isolated mode — it will listen but"
    echo "   won't connect to any existing mesh peers."
    echo ""
    echo "   To join a mesh, re-run with:"
    echo "     $0 --bootstrap /ip4/<addr>/tcp/<port>/p2p/<peer-id>"
    echo ""
fi

# ── Sandbox Creation ─────────────────────────────────────────
mkdir -p "$LATTICE_DIR/bin"
mkdir -p "$LATTICE_DIR/storage"

# ── Binary Fetch ─────────────────────────────────────────────
BINARY_URL="$BINARY_URL_BASE/$TARGET/lattice-node"
BINARY_PATH="$LATTICE_DIR/bin/lattice-node"

echo "📥 Fetching lattice-node binary for $TARGET..."
echo "   $BINARY_URL"

if command -v curl >/dev/null 2>&1; then
    if ! curl -fsSL "$BINARY_URL" -o "$BINARY_PATH"; then
        echo ""
        echo "❌ Failed to download binary from $BINARY_URL"
        echo ""
        echo "   This is expected — the binary distribution pipeline is not"
        echo "   yet live (B3 blocker). Build from source for now:"
        echo ""
        echo "     git clone https://github.com/emergencecollective/lattice-node"
        echo "     cd lattice-node && cargo build --release"
        echo "     cp target/release/lattice-node ~/.lattice/bin/"
        echo ""
        exit 1
    fi
elif command -v wget >/dev/null 2>&1; then
    if ! wget -qO "$BINARY_PATH" "$BINARY_URL"; then
        echo "❌ Failed to download binary (wget). See curl fallback above."
        exit 1
    fi
else
    echo "❌ Neither curl nor wget found. Install one and retry."
    exit 1
fi

chmod +x "$BINARY_PATH"

# ── Node Name Generation ─────────────────────────────────────
NODE_NAME="edge-$(hostname 2>/dev/null | tr '[:upper:]' '[:lower:]' || echo "unknown")"

echo "🚀 Starting lattice-node: $NODE_NAME"
echo ""

# ── Launch ───────────────────────────────────────────────────
if [ -n "$BOOTSTRAP_ADDRESS" ]; then
    exec "$BINARY_PATH" \
        --name "$NODE_NAME" \
        --storage-dir "$LATTICE_DIR/storage" \
        --bootstrap-peer "$BOOTSTRAP_ADDRESS"
else
    exec "$BINARY_PATH" \
        --name "$NODE_NAME" \
        --storage-dir "$LATTICE_DIR/storage"
fi
