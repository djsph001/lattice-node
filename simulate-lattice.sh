#!/usr/bin/env bash
# simulate-lattice.sh — Local Multi-Node Matrix Orchestration
#
# Spins up 5 sovereign lattice nodes that discover each other via
# mDNS + Kademlia, run the Georgist economic engine on a 10-second
# epoch cycle, and pass storage challenges against registered
# resource claims.
#
# Usage:
#   chmod +x simulate-lattice.sh
#   ./simulate-lattice.sh
#
# While running, watch the economic story in another terminal:
#   tail -f ./lattice-simulation/*.log | grep -E 'epoch|Epoch|VERIFIED|FAILED|health'

set -uo pipefail

SIM_DIR="./lattice-simulation"
BINARY="./target/debug/lattice-node"
EPOCH_INTERVAL=10
HEARTBEAT_INTERVAL=3
MINT_START=1000

# ── Clean slate ────────────────────────────────────────────
echo "🧹 Purging previous simulation state..."
rm -rf "$SIM_DIR"
mkdir -p "$SIM_DIR"

# ── Build ──────────────────────────────────────────────────
echo "🦀 Building debug binary..."
cargo build 2>&1 | tail -1

# ── Alpha (bootstrap / genesis node) ──────────────────────
echo ""
echo "🚀 Launching 5-Node Lattice Matrix..."
echo ""

"$BINARY" \
    --name alpha \
    --epoch-interval "$EPOCH_INTERVAL" \
    --heartbeat-interval "$HEARTBEAT_INTERVAL" \
    --storage-dir "$SIM_DIR/alpha-storage" \
    --identity-dir "$SIM_DIR/alpha-id" \
    --fresh-identity \
    --mint "$MINT_START" \
    > "$SIM_DIR/alpha.log" 2>&1 &
ALPHA_PID=$!
echo "  ↳ Alpha launched (PID: $ALPHA_PID) [Bootstrap Target]"

# ── Wait for alpha's listen address ───────────────────────
ALPHA_ADDR=""
ALPHA_PEER=""
for i in $(seq 1 30); do
    sleep 0.5
    # Strip ANSI color codes from the log before parsing.
    CLEAN=$(sed $'s/\x1b\\[[0-9;]*m//g' "$SIM_DIR/alpha.log" 2>/dev/null)
    ALPHA_ADDR=$(echo "$CLEAN" | awk -F'addr=' '/Listening.*127\.0\.0\.1/ {print $2; exit}')
    ALPHA_PEER=$(echo "$CLEAN" | awk -F'peer_id=' '/Node identity established/ {print $2; exit}')
    if [ -n "$ALPHA_ADDR" ] && [ -n "$ALPHA_PEER" ]; then
        break
    fi
done

if [ -z "$ALPHA_ADDR" ] || [ -z "$ALPHA_PEER" ]; then
    echo "❌ Failed to capture alpha's listen address. Check $SIM_DIR/alpha.log"
    kill $ALPHA_PID 2>/dev/null
    exit 1
fi

BOOTSTRAP="${ALPHA_ADDR}/p2p/${ALPHA_PEER}"
echo "  ↳ Alpha listening at $BOOTSTRAP"

# ── Beta, Gamma, Delta, Epsilon ───────────────────────────
NODES=("beta" "gamma" "delta" "epsilon")
for NODE in "${NODES[@]}"; do
    "$BINARY" \
        --name "$NODE" \
        --epoch-interval "$EPOCH_INTERVAL" \
        --heartbeat-interval "$HEARTBEAT_INTERVAL" \
        --storage-dir "$SIM_DIR/${NODE}-storage" \
        --identity-dir "$SIM_DIR/${NODE}-id" \
        --fresh-identity \
        --mint "$MINT_START" \
        --bootstrap-peer "$BOOTSTRAP" \
        > "$SIM_DIR/${NODE}.log" 2>&1 &
    echo "  ↳ $NODE launched (PID: $!) bootstrapping to alpha"
    sleep 0.5
done

echo ""
echo "════════════════════════════════════════════════════════════"
echo "🎯 Simulation Active — 5 nodes, mDNS + Kademlia"
echo "   Epoch: ${EPOCH_INTERVAL}s  |  Heartbeat: ${HEARTBEAT_INTERVAL}s"
echo ""
echo "   Watch the economic story:"
echo "   tail -f $SIM_DIR/*.log | sed 's/\x1b\[[0-9;]*m//g' | grep -E 'epoch|Epoch|VERIFIED|FAILED|health|minted'"
echo ""
echo "   Watch discovery:"
echo "   tail -f $SIM_DIR/*.log | sed 's/\x1b\[[0-9;]*m//g' | grep -E 'Peer discovered|bootstrap|Heartbeat received'"
echo ""
echo "   All logs:"
echo "   ls $SIM_DIR/*.log"
echo "════════════════════════════════════════════════════════════"
echo "Press [CTRL+C] to terminate the matrix."
echo ""

# ── Trap: kill all nodes on exit ──────────────────────────
cleanup() {
    echo ""
    echo "🛑 Shutting down lattice matrix..."
    kill $ALPHA_PID 2>/dev/null
    jobs -p 2>/dev/null | xargs -r kill 2>/dev/null
    wait 2>/dev/null
    echo "✓ All nodes stopped."
}
trap cleanup EXIT INT TERM

# ── Wait for Ctrl+C ───────────────────────────────────────
wait
