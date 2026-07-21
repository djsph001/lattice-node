#!/bin/bash
# mesh-gates.sh — run after every restart to check recovery gates
# Usage: bash mesh-gates.sh [logfile]
#   logfile defaults to mac.cycle0.log

LOG="${1:-mac.cycle0.log}"

echo "════════════════════════════════════════"
echo " GATE CHECKS — $(date '+%H:%M:%S')"
echo " Log: $LOG"
echo "════════════════════════════════════════"

echo ""
echo "G1 — Consistency assertion:"
if grep -q "WAL consistency check passed" "$LOG"; then
    grep "WAL consistency check passed" "$LOG"
    echo "  ✅ PASS"
else
    echo "  ❌ FAIL — assertion not found"
fi

echo ""
echo "RECOVERY — Replay stats:"
grep -E "Replayed transactions|Recovered tx_nonce|Recovered balances" "$LOG"

echo ""
echo "G2 — First epoch balance:"
FIRST=$(grep "Epoch complete" "$LOG" | head -2)
echo "$FIRST"
BEFORE=$(echo "$FIRST" | head -1 | sed -n 's/.*balance_before=\([0-9]*\).*/\1/p')
AFTER=$(grep "Epoch complete" "$(echo "$1" | sed 's/cycle[0-9]/cycle0/')" 2>/dev/null | tail -1 | sed -n 's/.*balance_after=\([0-9]*\).*/\1/p')
echo "  Restart balance_before: $BEFORE"
echo "  Pre-kill balance_after: (check kill output)"

echo ""
echo "G4 — Unexpected warnings (must be empty):"
UNEXPECTED=$(grep -iE "warn|error" "$LOG" | grep -vE "skip-ntp-check|non-mDNS peer|insufficient balance|GRAFT|zombie|reconnect|Pending connection|timestamp is|WAL replay gap detected|Failed to reconnect")
if [ -z "$UNEXPECTED" ]; then
    echo "  ✅ PASS — no unexpected warnings"
else
    echo "$UNEXPECTED"
    echo "  ❌ FAIL — unexpected warnings found"
fi

echo ""
echo "════════════════════════════════════════"
