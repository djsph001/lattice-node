#!/bin/bash
# mesh-api-start.sh — start the api-test node for live API verification
set -e
cd ~/lattice-node
pkill -9 -f api-test 2>/dev/null || true
rm -f api-test.log
rm -rf lattice-storage-test
mkdir -p lattice-storage-test
nohup ./target/release/lattice-node \
  --name api-test --port 4005 \
  --bootstrap-peer /ip4/66.229.91.202/tcp/4002/p2p/12D3KooWBPyhSPBrVxSxq7oLo4q9X38Y9qosgnMyyaTs3452CiEz \
  --genesis-root 12D3KooWBPyhSPBrVxSxq7oLo4q9X38Y9qosgnMyyaTs3452CiEz \
  --identity-dir ~/.lattice-api-test \
  --storage-dir ./lattice-storage-test \
  --persistence --skip-ntp-check \
  > api-test.log 2>&1 &
echo "Started PID $!"
