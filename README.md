# Lattice Node

Sovereign peer-to-peer node for the Lattice mesh network. Join the mesh in 15 minutes.

## Quick Start

```bash
# 1. Install Rust (skip if you have it)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# 2. Clone and build
git clone https://github.com/djsph001/lattice-node.git
cd lattice-node
cargo build --release

# 3. Start your node — it connects to the public relay
./target/release/lattice-node \
  --name your-name-here \
  --port 0 \
  --listen-addr 0.0.0.0 \
  --identity-dir ~/.lattice \
  --storage-dir ./lattice-storage \
  --bootstrap-peer /ip4/167.233.223.174/tcp/4001
```

Your node generates a persistent Ed25519 identity, connects to the relay in Germany, and joins the mesh. Look for:

```
Connection established peer=12D3KooWBoVfr...
Identify: peer supports relay (HOP_PROTOCOL)
Kademlia routing table: peer added
Status response received from=relay-hub
```

## What's Running

The Lattice is a mesh of sovereign nodes — no central server, no blockchain, no crypto wallet required. Nodes peer directly, exchange gossip, and coordinate through cryptographic attestation.

**Current mesh:** two nodes (Florida + Germany), one operator. Cross-Atlantic gossip confirmed. The receipt-gated mint correctly produces 0 DUUs — contribution requires a third party to relay traffic for, and there isn't one yet.

**When a third peer joins:** real relay work begins, receipts flow to nodes that earned them, and the mint produces its first honest DUUs. At 5+ independent nodes, quorum-gated certification and thickness-based governance activate.

## What You Need

- A computer with internet (laptop, Pi, cloud VM — anything running Linux or macOS)
- Rust toolchain (installed above)
- 15 minutes

No signup. No wallet. No approval. You run the binary and you're on the mesh.

## Keeping It Running

The node stops when you close the terminal. To run it as a persistent service, use the systemd unit at `deploy/lattice-lumen.service`:

```bash
sudo cp deploy/lattice-lumen.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable lattice-lumen
sudo systemctl start lattice-lumen
```

Customize `--name`, paths, and user before installing.

## Two Nodes on One Machine

You can run multiple nodes on the same machine with separate identities:

```bash
# Node 1
./target/release/lattice-node \
  --name alpha \
  --port 0 \
  --identity-dir ~/.lattice-alpha \
  --storage-dir ./storage-alpha \
  --bootstrap-peer /ip4/167.233.223.174/tcp/4001

# Node 2 (different terminal)
./target/release/lattice-node \
  --name bravo \
  --port 0 \
  --identity-dir ~/.lattice-bravo \
  --storage-dir ./storage-bravo \
  --bootstrap-peer /ip4/167.233.223.174/tcp/4001
```

Each gets its own Ed25519 keypair on first run. Distinct `--identity-dir` and `--storage-dir` keep them isolated.

## Architecture

```
src/
├── main.rs              CLI, tokio runtime
├── node.rs              Node identity, swarm, event loop
├── network/
│   └── protocol.rs      libp2p behaviour (mDNS, gossipsub, Kademlia, RPC, relay)
├── message/
│   ├── types.rs         Heartbeat, StatusReport, Transactions, receipts
│   └── codec/           CBOR encode/decode, RPC codecs
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

  -p, --port <PORT>              Port to listen on (0 = random) [default: 0]
  -n, --name <NAME>              Human-readable node name
      --listen-addr <ADDR>       IP to bind to [default: 0.0.0.0]
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
```

## Roadmap

- [x] Phase 2: Persistent identity, gossipsub heartbeat propagation
- [x] Phase 3: Kademlia DHT for cross-network discovery
- [x] Phase 4: Digital utility unit transactions (Transfer, Mint, Vouch)
- [x] Phase 5: Georgist resource accounting engine
- [x] Phase 6: Receipt-gated minting — self-report path removed
- [x] Phase 6b–6c: Storage verification, trilateral receipts, relay transport
- [x] Thickness provenance graph with chained clawback
- [x] Weighted witness sortition with panel-access density guard
- [x] Verify-before-sign claim architecture
- [ ] BootstrapEnded: one-way chain-anchored transition to quorum governance
- [ ] Economic events riding the certificate/chain flow
- [ ] State persistence via chain replay
- [ ] Pi 5 edge node deployment
