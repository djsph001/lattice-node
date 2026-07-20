#!/bin/bash
set -euo pipefail

# mesh-kill.sh — kill a running lattice-node, save evidence, report pre-kill state
# Usage:
#   bash mesh-kill.sh [cycle_number]
#     cycle_number defaults to 1
#     Evidence goes to lattice-storage.cycle${N}.${TS}/
#     Frozen log grep uses mac.cycle0.log (latest run)

NAME="mac-node"
CYCLE="${1:-1}"
TS=$(date +%H%M%S)
EVIDENCE_DIR="lattice-storage.cycle${CYCLE}.${TS}"

echo "Killing $NAME (cycle $CYCLE)..."

# Kill
pkill -9 -f "lattice-node --name $NAME" 2>/dev/null || true
sleep 1

# Verify dead
REMAINING=$(pgrep -fl lattice-node 2>/dev/null || true)
if [ -n "$REMAINING" ]; then
    echo "WARNING: remaining processes:"
    echo "$REMAINING"
else
    echo "Confirm dead: no lattice-node processes"
fi

# Evidence copy
if [ -d "./lattice-storage" ]; then
    mkdir -p "$EVIDENCE_DIR"
    # cp will warn about sockets; that's expected
    cp -r ./lattice-storage/* "$EVIDENCE_DIR/" 2>/dev/null || true
    echo "Evidence saved to $EVIDENCE_DIR/"
    echo "=== WAL in evidence ==="
    ls -la "$EVIDENCE_DIR/persistence/" 2>/dev/null || echo "(no persistence dir)"
else
    echo "WARNING: ./lattice-storage not found — no evidence saved"
fi

# Frozen log greps
echo "=== Pre-kill state (from mac.cycle0.log) ==="
grep "Epoch complete" mac.cycle0.log 2>/dev/null | tail -1
grep "broadcast" mac.cycle0.log 2>/dev/null | tail -1
