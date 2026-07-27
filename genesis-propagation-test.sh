#!/usr/bin/env bash
# genesis-propagation-test — verify root-authored genesis reaches witness
# Isolated from production mesh. Identity dirs preserved, storage wiped each run.

set -euo pipefail

export NO_COLOR=1

TEST_DIR="/tmp/genesis-test"
ROOT_ID="$TEST_DIR/root-id"
ROOT_STORE="$TEST_DIR/root-store"
WITNESS_ID="$TEST_DIR/witness-id"
WITNESS_STORE="$TEST_DIR/witness-store"
ROOT_PORT=4105
WITNESS_PORT=4110
BINARY="$HOME/Projects/lattice-node/target/release/lattice-node"
ROOT_LOG="$TEST_DIR/root.log"
WITNESS_LOG="$TEST_DIR/witness.log"
DURATION=60

mkdir -p "$ROOT_ID" "$WITNESS_ID" "$ROOT_STORE"/persistence "$WITNESS_STORE"/persistence

die() { printf '\033[31mFATAL: %s\033[0m\n' "$*" >&2; exit 1; }
green() { printf '\033[32m%s\033[0m\n' "$*"; }
red()   { printf '\033[31m%s\033[0m\n' "$*"; }
warn()  { printf '\033[33m%s\033[0m\n' "$*"; }

# ── 1. stop prior test nodes (by name, not pkill -f lattice-node) ──
for name in genesis-root genesis-witness; do
    for pid in $(pgrep -f "lattice-node.*--name $name" 2>/dev/null || true); do
        kill "$pid" 2>/dev/null || true
    done
done
sleep 1

# ── 2. wipe storage (identities survive) ──
rm -rf "$ROOT_STORE"/* "$WITNESS_STORE"/*
mkdir -p "$ROOT_STORE"/persistence "$WITNESS_STORE"/persistence
rm -f "$ROOT_STORE"/lattice.lock "$WITNESS_STORE"/lattice.lock
> "$ROOT_LOG"
> "$WITNESS_LOG"

# ── 3. launch root ──
echo "=== genesis-propagation-test ==="
echo "root :$ROOT_PORT  witness :$WITNESS_PORT  |  $(date +%H:%M:%S)"
echo

echo -n "launching root... "
$BINARY \
    --name genesis-root --port "$ROOT_PORT" \
    --identity-dir "$ROOT_ID" --storage-dir "$ROOT_STORE" \
    --auto-genesis --no-mdns --persistence --mint 5000 \
    >>"$ROOT_LOG" 2>&1 &
ROOT_PID=$!
echo "PID=$ROOT_PID"

# Extract PeerId from log
PEER_ID=""
for i in $(seq 1 20); do
    sleep 0.5
    PEER_ID=$(grep "peer_id=" "$ROOT_LOG" 2>/dev/null | head -1 | sed 's/.*peer_id=//' | awk '{print $1}' | tr -d '\r\n' || true)
    # PeerId format: 52-char base58 string starting with "12D3Koo"
    if echo "$PEER_ID" | grep -qE '^12D3Koo'; then
        break
    fi
    PEER_ID=""
done
[ -n "$PEER_ID" ] || die "root did not report a valid PeerId within 10s"
green "root PeerId=$PEER_ID"

# ── 4. launch witness ──
echo -n "launching witness... "
$BINARY \
    --name genesis-witness --port "$WITNESS_PORT" \
    --identity-dir "$WITNESS_ID" --storage-dir "$WITNESS_STORE" \
    --genesis-root "$PEER_ID" \
    --bootstrap-peer "/ip4/127.0.0.1/tcp/$ROOT_PORT/p2p/$PEER_ID" \
    --no-mdns --persistence --mint 0 \
    >>"$WITNESS_LOG" 2>&1 &
WITNESS_PID=$!
echo "PID=$WITNESS_PID"

# ── 5. wait ──
echo "waiting ${DURATION}s..."
sleep "$DURATION"

# ── 6. report ──
echo; echo "=== results ==="

# Root side
grep -q "Authored and persisted genesis" "$ROOT_LOG" 2>/dev/null \
    && green "✓ root authored genesis" \
    || red "✗ root did NOT author genesis"

# Witness side — check three signals
WITNESS_AUTHORED=$(grep -c "Authored and persisted genesis" "$WITNESS_LOG" 2>/dev/null || echo 0)
WITNESS_RECEIVED=$(grep -c "Genesis accepted and persisted" "$WITNESS_LOG" 2>/dev/null || echo 0)
WITNESS_WAL_GENESIS=""
if [ -f "$WITNESS_STORE/persistence/wal.log" ] && xxd "$WITNESS_STORE/persistence/wal.log" 2>/dev/null | grep -q "Genesis"; then
    WITNESS_WAL_GENESIS=1
fi

if [ "$WITNESS_AUTHORED" -gt 0 ]; then
    red "✗ witness self-authored genesis (should be participant, not root)"
    warn "  Witness genesis-root=$(grep -oP 'root=\K\S+' "$WITNESS_LOG" | head -1)"
elif [ "$WITNESS_RECEIVED" -gt 0 ]; then
    green "✓ witness accepted genesis from root"
elif [ -n "$WITNESS_WAL_GENESIS" ]; then
    green "✓ witness WAL contains Genesis record (delivered but no explicit accept log)"
else
    red "✗ no genesis on witness (no author, no accept, no WAL record)"
fi

# Peer connection
echo; echo "--- connections ---"
grep -c "Connection established" "$WITNESS_LOG" 2>/dev/null | xargs echo "witness connections:" || echo "witness connections: 0"
grep -c "Connection established" "$ROOT_LOG" 2>/dev/null | xargs echo "root connections:" || echo "root connections: 0"

# Re-gossip trace
echo; echo "--- re-gossip trace (root) ---"
grep "re_gossip_genesis" "$ROOT_LOG" 2>/dev/null || echo "(no re-gossip lines)"

# ── 7. logs ──
echo; echo "=== logs ==="
echo "root     : $ROOT_LOG"
echo "witness  : $WITNESS_LOG"
echo "root WAL : $(ls -la "$ROOT_STORE"/persistence/wal.log 2>/dev/null | awk '{print $5, $NF}' || echo none)"
echo "witn WAL : $(ls -la "$WITNESS_STORE"/persistence/wal.log 2>/dev/null | awk '{print $5, $NF}' || echo none)"

# ── 8. kill test nodes ──
kill "$ROOT_PID" "$WITNESS_PID" 2>/dev/null || true
echo "=== done ==="
