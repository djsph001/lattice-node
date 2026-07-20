#!/bin/bash
set -euo pipefail

NAME="mac-node"
PORT=4001
BOOTSTRAP="/ip4/10.0.0.133/tcp/4002/p2p/12D3KooWJQJ9n5ruSYb7UJQNvckrcYScFteHnfL1w15nRYRm9Vif"
GENESIS_ROOT="12D3KooWJQJ9n5ruSYb7UJQNvckrcYScFteHnfL1w15nRYRm9Vif"

# Guard: refuse if any lattice-node process is alive
if pgrep -fl lattice-node > /dev/null 2>&1; then
    echo "ERROR: existing lattice-node process found. Run mesh-kill.sh first."
    pgrep -fl lattice-node
    exit 1
fi

echo "Guard OK: no lattice-node processes running"

# Wipe and recreate
rm -rf ~/.lattice ./lattice-storage
mkdir -p ./lattice-storage

# Launch
./target/release/lattice-node \
  --name "$NAME" --port "$PORT" \
  --bootstrap-peer "$BOOTSTRAP" \
  --genesis-root "$GENESIS_ROOT" \
  --identity-dir ~/.lattice \
  --storage-dir ./lattice-storage \
  --mint 5000 \
  --persistence --skip-ntp-check \
  2>&1 | tee mac.cycle0.log &

NODE_PID=$!
sleep 4  # wait for startup + epoch 1

# Verify
echo "=== WAL check ==="
ls -la ./lattice-storage/persistence/
lsof -p "$NODE_PID" 2>/dev/null | grep -i wal || echo "(lsof not available)"
echo "=== Node PID: $NODE_PID ==="
echo "=== tail mac.cycle0.log ==="
tail -5 mac.cycle0.log
