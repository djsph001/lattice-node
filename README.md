# Lattice Node

Sovereign peer-to-peer node for the Lattice mesh network.

## Join the Mesh (15 minutes)

```bash
# 1. Clone
git clone https://github.com/djsph001/lattice-node.git
cd lattice-node

# 2. One-command setup (installs Rust if needed, builds release binary)
./scripts/setup-linux.sh    # Linux
./scripts/setup-mac.sh      # macOS (Apple Silicon: ~1 minute)
powershell scripts/setup-windows.ps1  # Windows

# 3. Join — connects to the live relay in Germany
./target/release/lattice-node \
  --name your-name-here \
  --port 0 \
  --bootstrap-peer /ip4/167.233.223.174/tcp/4001/p2p/12D3KooWBoVfrJJJMsbGGZDf3tBscvGhhEYd3C4JAayPEufcGPEu
```

You'll know you're on the mesh when you see:

```
Connection established peer=12D3KooWBoVfr...
Identify: peer supports relay (HOP_PROTOCOL)
Kademlia routing table: peer added
Status response received from=relay-hub
```

Your node will show `minted=0` and `balance=0` — **this is correct.** Relaying is a three-party act: someone originates, you carry, someone receives. At the current mesh size, there's no third party whose traffic you could carry, so the receipt-gated mint correctly produces nothing. The moment a third peer joins, relay work becomes possible and the mint activates.

## What This Is

The Lattice is a mesh of sovereign nodes — no central server, no blockchain, no crypto wallet required. Nodes peer directly, verify each other's contributions through cryptographic receipts, and earn thickness from work done for others.

**Current mesh:** three nodes (Florida + Germany + Florida), one operator. Cross-Atlantic gossip confirmed. Phase 6 receipt-gated mint active and verified. The mesh correctly produces economic output only when independent peers relay traffic for each other.

## What You Need

- A computer with internet (laptop, Pi, cloud VM — Linux, macOS, or Windows)
- 15 minutes
- No signup. No wallet. No approval.

## Keeping It Running

The node stops when you close the terminal. To run persistently:

```bash
sudo cp deploy/lattice-lumen.service /etc/systemd/system/
# Edit the service file to set --name and paths
sudo systemctl daemon-reload
sudo systemctl enable lattice-lumen
sudo systemctl start lattice-lumen
```

## Architecture

```
src/
├── main.rs              CLI, tokio runtime
├── node.rs              Node identity, swarm, event loop
├── network/             libp2p behaviour (mDNS, gossipsub, Kademlia, RPC, relay)
├── message/             Heartbeat, StatusReport, Transactions, receipts
├── ledger/              Thickness provenance graph, transactions, validation
├── economics/           Georgist engine, receipt-gated minting, metrics
├── agent/               Distributed task registry, executor bridge
├── claims/              Verify-before-sign claim architecture
├── sortition.rs         Deterministic weighted witness panel selection
├── commit.rs            Append-only Blake3 hash-chain ledger
├── storage/             Blake3-addressed chunks, Merkle proofs
├── api.rs               Unix Domain Socket query API
└── deploy/              Systemd units, provisioning scripts
```

## Options

```
lattice-node [OPTIONS]

  -p, --port <PORT>              Port (0 = random) [default: 0]
  -n, --name <NAME>              Human-readable node name
      --listen-addr <ADDR>       Bind address [default: 0.0.0.0]
      --identity-dir <DIR>       Persistent key storage [default: ~/.lattice]
      --storage-dir <DIR>        Chain and registry storage
      --bootstrap-peer <ADDR>    Relay multiaddr (repeatable)
      --agent-mode               Accept agent task execution
      --max-model-size <SIZE>    tiny|small|medium|large [default: small]
      --vram-bytes <BYTES>       Available GPU VRAM [default: 0]
      --relay-server             Accept relay circuits for other nodes
      --external-addr <ADDR>     Public address for NAT traversal
      --no-mdns                  Disable local network discovery
      --fresh-identity           Generate new key (discards existing)

      --genesis-root <PEER_ID>   Expected root PeerId for Genesis validation
                                 (required for economic participation; pending)
```

## What's Running vs. What's Built

**Running now:** peer-to-peer mesh, cross-Atlantic gossip, receipt-gated minting. Nodes verify each other's relay work through signed receipts. The economic layer activates when independent peers arrive.

**Built and pending activation:** thickness-based witness sortition, 3-of-5 certificate quorum, verified contribution claims, chain-anchored era partition with BootstrapEnded confession.

**Requires independent peers:** quorum-gated certification for economic transactions, state persistence via chain replay, governance by sortition.

## Roadmap

- [x] Phase 2: Persistent identity, gossipsub heartbeat propagation
- [x] Phase 3: Kademlia DHT for cross-network discovery
- [x] Phase 5: Georgist resource accounting engine
- [x] Phase 6: Receipt-gated minting — self-report path removed, verified in live 3-node mesh
- [x] Thickness provenance graph with chained clawback
- [x] Weighted witness sortition with panel-access density guard
- [x] BootstrapEnded: one-way chain-anchored era transition (spec + era derivation)
- [ ] Genesis authoring with root-key validation
- [ ] Economic events riding the certificate/chain flow
- [ ] State persistence via chain replay
