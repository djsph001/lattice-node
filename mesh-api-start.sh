#!/bin/bash
# mesh-api-start.sh — start the api-test node for live API verification
set -e
cd ~/lattice-node
pkill -9 -f api-test 2>/dev/null
rm -f api-test.log
rm -rf lattice-storage-test
mkdir lattice-storage-test
nohup ./target/release/lattice-node \
  --name api-test --port 4005 \
  --bootstrap-peer /ip4/66.229.91.202/tcp/4002 \
  --storage-dir ./lattice-storage-test \
  --persistence --skip-ntp-check \
  > api-test.log 2>&1 &
echo "Started PID $!"
