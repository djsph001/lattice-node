#!/bin/bash
# mesh-api-check.sh — verify read-only API endpoints against a running node
# Usage: bash mesh-api-check.sh [socket-path]
#   socket-path defaults to ./lattice-storage-test/lattice.sock

SOCKET="${1:-./lattice-storage-test/lattice.sock}"

for query in NodeInfo Peers EconomicState EpochState PersistenceState Height; do
  echo "=== $query ==="
  echo "{\"type\":\"Get$query\"}" | nc -U "$SOCKET"
  echo ""
done

echo "=== Error shape (Nonsense) ==="
echo '{"type":"Nonsense"}' | nc -U "$SOCKET"
echo ""
