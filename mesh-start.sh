#!/bin/bash
set -euo pipefail

# mesh-start.sh — launch a single lattice-node with persistence for kill testing
# Usage:
#   bash mesh-start.sh [mint_amount]
#     mint_amount=5000 (default) — fresh join with startup mint
#     mint_amount=0             — restart from existing state (no --mint)

NAME="mac-node"
PORT=4001
BOOTSTRAP="/ip4/10.0.0.133/tcp/4002/p2p/12D3KooWJQJ9n5ruSYb7UJQNvckrcYScFteHnfL1w15nRYRm9Vif"
GENESIS_ROOT="12D3KooWJQJ9n5ruSYb7UJQNvckrcYScFteHnfL1w15nRYRm9Vif"
MINT="${1:-5000}"
LOG="mac.cycle0.log"

# Guard: refuse if any lattice-node process is alive
if pgrep -fl lattice-node > /dev/null 2>&1; then
    echo "ERROR: existing lattice-node process found. Run mesh-kill.sh first."
    pgrep -fl lattice-node
    exit 1
fi
echo "Guard OK: no lattice-node processes running"

# Fresh start — wipe.  Restart — don't wipe.
if [ "$MINT" != "0" ]; then
    echo "Fresh start: wiping state"
    rm -rf ~/.lattice ./lattice-storage
    mkdir -p ./lattice-storage
else
    echo "Restart: preserving existing state"
fi

# Build the argument list
ARGS=(
    --name "$NAME" --port "$PORT"
    --bootstrap-peer "$BOOTSTRAP"
    --genesis-root "$GENESIS_ROOT"
    --identity-dir ~/.lattice
    --storage-dir ./lattice-storage
    --persistence --skip-ntp-check
)
if [ "$MINT" != "0" ]; then
    ARGS+=(--mint "$MINT")
fi

# Launch — background with redirect (not tee), capture node PID
./target/release/lattice-node "${ARGS[@]}" > "$LOG" 2>&1 &
NODE_PID=$!
echo "Launched node PID=$NODE_PID, log=$LOG"

sleep 4  # wait for startup + epoch 1

# Verify
echo "=== WAL check ==="
ls -la ./lattice-storage/persistence/ 2>/dev/null || echo "(persistence dir not found)"
if command -v lsof &>/dev/null; then
    lsof -p "$NODE_PID" 2>/dev/null | grep -i wal || echo "(no WAL fd found)"
fi
echo "=== Node PID: $NODE_PID ==="
echo "=== tail $LOG ==="
tail -5 "$LOG"
