#!/bin/bash
set -euo pipefail

NAME="mac-node"
CYCLE="${1:-1}"

# Kill
echo "Killing $NAME..."
pkill -9 -f "lattice-node --name $NAME" 2>/dev/null || true
sleep 1
REMAINING=$(pgrep -fl lattice-node 2>/dev/null || true)
if [ -n "$REMAINING" ]; then
    echo "WARNING: remaining processes:"
    echo "$REMAINING"
fi

# Evidence copy with timestamp
TS=$(date +%H%M%S)
EVIDENCE_DIR="lattice-storage.cycle${CYCLE}.${TS}"
if [ -d "./lattice-storage" ]; then
    mkdir -p "$EVIDENCE_DIR"
    cp -r ./lattice-storage/* "$EVIDENCE_DIR/" 2>/dev/null || true
    echo "Evidence saved to $EVIDENCE_DIR/"
fi

# Frozen grep
echo "=== Pre-kill state ==="
grep "Epoch complete" mac.cycle0.log | tail -1
grep "broadcast" mac.cycle0.log | tail -1

# WAL check on evidence
echo "=== WAL in evidence ==="
ls -la "$EVIDENCE_DIR/persistence/" 2>/dev/null || echo "(no persistence dir)"
