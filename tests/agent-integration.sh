#!/bin/bash
# Phase 8 — Agent harness integration test.
set -uo pipefail

TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

BINARY="./target/release/lattice-node"

ALPHA_DIR="$TMPDIR/alpha"
BRAVO_DIR="$TMPDIR/bravo"
mkdir -p "$ALPHA_DIR/storage" "$BRAVO_DIR/storage"

PASS=0
FAIL=0
pass() { echo "  PASS: $1"; PASS=$((PASS+1)); }
fail() { echo "  FAIL: $1"; FAIL=$((FAIL+1)); }

echo "=== Phase 8 Agent Harness Integration Test ==="

# ── Start alpha ────────────────────────────────────
echo "[1/5] Starting alpha node..."
"$BINARY" \
    --name alpha \
    --identity-dir "$ALPHA_DIR/identity" \
    --storage-dir "$ALPHA_DIR/storage" \
    --fresh-identity \
    --port 0 \
    --agent-mode \
    &> "$TMPDIR/alpha.log" &
ALPHA_PID=$!
sleep 4

CLEAN=$(sed $'s/\x1b\[[0-9;]*m//g' "$TMPDIR/alpha.log")
ALPHA_ADDR=$(echo "$CLEAN" | grep -oP 'Listening addr=\K[^ ]+' | head -1)
ALPHA_PEER=$(echo "$CLEAN" | grep -oP 'peer_id=\K\S+' | head -1)

if [ -z "$ALPHA_ADDR" ] || [ -z "$ALPHA_PEER" ]; then
    fail "Could not extract alpha address/peer from log"
    echo "  Log excerpt:"
    echo "$CLEAN" | grep -E "Listening|peer_id|Node identity" | head -5
    kill $ALPHA_PID 2>/dev/null
    exit 1
fi
pass "Alpha started: $ALPHA_ADDR/p2p/$ALPHA_PEER"

# ── Start bravo ────────────────────────────────────
echo "[2/5] Starting bravo node..."
"$BINARY" \
    --name bravo \
    --identity-dir "$BRAVO_DIR/identity" \
    --storage-dir "$BRAVO_DIR/storage" \
    --fresh-identity \
    --port 0 \
    --no-mdns \
    --bootstrap-peer "$ALPHA_ADDR/p2p/$ALPHA_PEER" \
    --agent-mode \
    &> "$TMPDIR/bravo.log" &
BRAVO_PID=$!
sleep 5

CLEAN_BRAVO=$(sed $'s/\x1b\[[0-9;]*m//g' "$TMPDIR/bravo.log")
if echo "$CLEAN_BRAVO" | grep -q "Connection established"; then
    pass "Bravo connected to alpha"
else
    fail "Bravo did not connect to alpha"
fi

# ── Test UDS API ──────────────────────────────────
echo "[3/5] Submitting agent task via UDS..."
sleep 1
SOCK="$ALPHA_DIR/storage/lattice.sock"

if [ ! -S "$SOCK" ]; then
    fail "Alpha UDS socket not found at $SOCK"
else
    echo '{"type":"AgentSubmit","task_id":"itest-001","model":"test-model","graph_blob_b64":"AQIDBAUG","deadline_epoch":999}' | nc -U "$SOCK" -w 3 &
    sleep 2
    pass "Task submitted via UDS socket"
fi

# ── Check alpha registry ──────────────────────────
echo "[4/5] Checking alpha log for task submission..."
CLEAN_ALPHA2=$(sed $'s/\x1b\[[0-9;]*m//g' "$TMPDIR/alpha.log")
if echo "$CLEAN_ALPHA2" | grep -q "Task submitted"; then
    pass "Task submission logged on alpha"
else
    fail "No task submission log on alpha"
fi

# ── Check bravo received task ─────────────────────
echo "[5/5] Checking bravo received the task..."
sleep 2
CLEAN_BRAVO2=$(sed $'s/\x1b\[[0-9;]*m//g' "$TMPDIR/bravo.log")
if echo "$CLEAN_BRAVO2" | grep -q "Agent task received"; then
    pass "Bravo received agent task via gossipsub"
else
    fail "Bravo did not receive agent task"
fi

# ── Registry persistence ──────────────────────────
REGISTRY_FILE="$ALPHA_DIR/storage/agent_registry.jsonl"
if [ -f "$REGISTRY_FILE" ] && grep -q "itest-001" "$REGISTRY_FILE"; then
    pass "Agent registry file persisted with task itest-001"
else
    fail "Agent registry file missing or empty"
fi

# ── Cleanup ──────────────────────────────────────
kill $ALPHA_PID $BRAVO_PID 2>/dev/null
wait $ALPHA_PID $BRAVO_PID 2>/dev/null

echo ""
echo "=== Results: $PASS passed, $FAIL failed ==="
if [ $FAIL -gt 0 ]; then
    echo ""
    echo "Alpha log (last 20 lines):"
    tail -20 "$TMPDIR/alpha.log"
    echo ""
    echo "Bravo log (last 20 lines):"
    tail -20 "$TMPDIR/bravo.log"
    exit 1
fi
echo "ALL TESTS PASSED"
