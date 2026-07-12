#!/bin/bash
# Phase 8b — Agent harness integration test (auto-registration + migration).
set -uo pipefail

TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

BINARY="./target/release/lattice-node"

if [ ! -f "$BINARY" ]; then
    cargo build --release 2>&1 | tail -1
fi

ALPHA_DIR="$TMPDIR/alpha"
BRAVO_DIR="$TMPDIR/bravo"
CHARLIE_DIR="$TMPDIR/charlie"
mkdir -p "$ALPHA_DIR/storage" "$BRAVO_DIR/storage" "$CHARLIE_DIR/storage"

PASS=0
FAIL=0
pass() { echo "  PASS: $1"; PASS=$((PASS+1)); }
fail() { echo "  FAIL: $1"; FAIL=$((FAIL+1)); }

echo "=== Phase 8b Agent Harness Integration Test ==="

# ── Start alpha ────────────────────────────────────
echo "[1/6] Starting alpha (agent mode)..."
"$BINARY" --name alpha --identity-dir "$ALPHA_DIR/identity" \
    --storage-dir "$ALPHA_DIR/storage" --fresh-identity --port 0 \
    --agent-mode &> "$TMPDIR/alpha.log" &
ALPHA_PID=$!
sleep 4

CLEAN=$(sed $'s/\x1b\[[0-9;]*m//g' "$TMPDIR/alpha.log")
ALPHA_ADDR=$(echo "$CLEAN" | grep -oP 'Listening addr=\K[^ ]+' | head -1)
ALPHA_PEER=$(echo "$CLEAN" | grep -oP 'peer_id=\K\S+' | head -1)

if [ -z "$ALPHA_ADDR" ] || [ -z "$ALPHA_PEER" ]; then
    fail "Alpha didn't start"
    kill $ALPHA_PID 2>/dev/null; exit 1
fi
pass "Alpha started"

# ── Start bravo (agent mode) ────────────────────────
echo "[2/6] Starting bravo (agent mode)..."
"$BINARY" --name bravo --identity-dir "$BRAVO_DIR/identity" \
    --storage-dir "$BRAVO_DIR/storage" --fresh-identity --port 0 \
    --no-mdns --bootstrap-peer "$ALPHA_ADDR/p2p/$ALPHA_PEER" \
    --agent-mode &> "$TMPDIR/bravo.log" &
BRAVO_PID=$!
sleep 5

CLEAN_B=$(sed $'s/\x1b\[[0-9;]*m//g' "$TMPDIR/bravo.log")
if echo "$CLEAN_B" | grep -q "Connection established"; then
    pass "Bravo connected"
else
    fail "Bravo didn't connect"
fi

# ── Submit task via alpha UDS ──────────────────────
echo "[3/6] Submitting agent task..."
SOCK="$ALPHA_DIR/storage/lattice.sock"
echo '{"type":"AgentSubmit","task_id":"itest-migrate","model":"test-model","graph_blob_b64":"AQIDBAUG","deadline_epoch":9999}' | nc -U "$SOCK" -w 3 &
sleep 3

# Check bravo auto-registered the task (agent_mode on)
CLEAN_B2=$(sed $'s/\x1b\[[0-9;]*m//g' "$TMPDIR/bravo.log")
if echo "$CLEAN_B2" | grep -q "auto-registering"; then
    pass "Bravo auto-registered task (agent mode)"
else
    fail "Bravo did not auto-register task"
fi

# ── Kill bravo, verify migration ───────────────────
echo "[4/6] Killing bravo to test heartbeat-failure migration..."
kill $BRAVO_PID 2>/dev/null
wait $BRAVO_PID 2>/dev/null
sleep 15  # Wait for mDNS expiry (default is ~10-15s)

CLEAN_A=$(sed $'s/\x1b\[[0-9;]*m//g' "$TMPDIR/alpha.log")
if echo "$CLEAN_A" | grep -q "Peer expired"; then
    pass "Alpha detected bravo expiry"
else
    fail "Alpha did not detect bravo expiry"
fi

# Check alpha logs for task migration
if echo "$CLEAN_A" | grep -q "Task migrated"; then
    pass "Task migrated from expired peer to alpha"
else
    fail "Task not migrated"
fi

# ── Verify registry on alpha has the task ──────────
echo "[5/6] Checking registry persistence..."
if grep -q "itest-migrate" "$ALPHA_DIR/storage/agent_registry.jsonl"; then
    pass "Task itest-migrate in alpha registry"
else
    fail "Task not in alpha registry"
fi

# ── Economic integration ───────────────────────────
echo "[6/6] Checking economic integration..."
# Epoch cycle runs with agent metrics synced
if echo "$CLEAN_A" | grep -q "Epoch complete"; then
    pass "Economic epoch ran with agent metrics"
else
    fail "Economic epoch did not run"
fi

# ── Cleanup ──────────────────────────────────────
kill $ALPHA_PID 2>/dev/null
wait $ALPHA_PID 2>/dev/null

echo ""
echo "=== Results: $PASS passed, $FAIL failed ==="
[ $FAIL -gt 0 ] && exit 1
echo "ALL TESTS PASSED"
