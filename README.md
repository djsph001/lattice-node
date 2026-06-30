# Lattice Node

Sovereign peer-to-peer application layer for the Lattice mesh network.

## Quick Start

```bash
# One-time setup (Linux/Mac):
./scripts/setup-linux.sh   # or setup-mac.sh

# Start a node:
cargo run -- --name alpha --heartbeat-interval 5

# In a second terminal:
cargo run -- --name bravo --heartbeat-interval 5 --no-mdns \
  --bootstrap-peer /ip4/127.0.0.1/tcp/0/p2p/<alpha-peer-id>

# They discover each other via Kademlia DHT within seconds.
```

## Cross-Platform Deployment

The Lattice runs on Linux, macOS, and Windows. Each platform has a one-command setup script in `scripts/`.

### Setup

| Platform | Script | Binary |
|----------|--------|--------|
| Linux (Debian/Ubuntu) | `scripts/setup-linux.sh` | `target/release/lattice-node` |
| macOS (Apple Silicon / Intel) | `scripts/setup-mac.sh` | `target/release/lattice-node` |
| Windows | `scripts/setup-windows.ps1` | `target\release\lattice-node.exe` |

Each script installs Rust (if needed) and runs `cargo build --release`. The Linux script also optionally sets up `aarch64` cross-compilation for Raspberry Pi.

### Bootstrap Flow

The first node starts without a bootstrap peer and prints its PeerId:

```
cargo run -- --name genesis --heartbeat-interval 5
# → peer_id = 12D3KooWAbCdEf...
# → Listening on /ip4/0.0.0.0/tcp/XXXXX
```

Subsequent nodes point at it:

```
# From another machine:
cargo run -- --name mac-edge --no-mdns --heartbeat-interval 5 \
  --bootstrap-peer /dns4/<genesis-host>/tcp/<port>/p2p/12D3KooWAbCdEf...

# Windows:
.\target\release\lattice-node.exe --name win-edge --no-mdns ^
  --bootstrap-peer /dns4/<genesis-host>/tcp/<port>/p2p/12D3KooWAbCdEf...
```

The `/dns4/<host>` syntax in the bootstrap address lets libp2p resolve hostnames. Use `/ip4/<ip>` for raw IPs.

### Listen Address

By default the node binds to `0.0.0.0` (all interfaces). On multi-homed machines (Docker bridges, VPNs, multiple NICs), pin the listener to a specific interface:

```
cargo run -- --name z4 --listen-addr 192.168.1.100
```

### External Address (NAT Traversal)

Nodes behind NAT can advertise their public address so remote peers know where to dial:

```
cargo run -- --name edge --no-mdns \
  --external-addr /ip4/203.0.113.5/tcp/6001 \
  --bootstrap-peer /dns4/genesis.example.com/tcp/6001/p2p/<peer-id>
```

When `--external-addr` is set, libp2p registers it with Kademlia and the Identify protocol. Remote peers see the public address rather than the private bind address.

## Cross-Compile for Raspberry Pi

```bash
# One-time: install the aarch64 target and cross-linker
rustup target add aarch64-unknown-linux-gnu
sudo apt install gcc-aarch64-linux-gnu

# Build for Pi
cargo build --release --target aarch64-unknown-linux-gnu
# Binary: target/aarch64-unknown-linux-gnu/release/lattice-node
```

Or use `cross` (Docker-based, no system linker needed):

```bash
cargo install cross
cross build --release --target aarch64-unknown-linux-gnu
```

## Logging

Set `RUST_LOG` for granularity:

```bash
RUST_LOG=debug cargo run -- --name alpha
RUST_LOG=lattice_node::state=trace cargo run -- --name alpha
```

## Module Map

```
src/
├── main.rs              CLI, tokio runtime bootstrap
├── node.rs              Node identity, swarm, event loop
├── network/
│   └── protocol.rs      Composed libp2p behaviour (mDNS, gossipsub, Kademlia, RPC, relay)
├── message/
│   ├── types.rs         Heartbeat, StatusReport, Transactions, Verify receipts
│   └── codec/           CBOR encode/decode, RPC codecs
├── state/
│   └── peers.rs         In-memory peer table
├── ledger/              Transaction types, validation, local state
├── economics/           Georgist resource accounting engine
├── storage/             Blake3-addressed chunks, Merkle proofs, challenge engine
└── scripts/             Platform setup scripts
```

## Roadmap

- [x] **Phase 2a**: Persist node identity to disk (stable PeerId across restarts)
- [x] **Phase 2b**: Gossipsub for heartbeat propagation to all peers
- [x] **Phase 2c**: Request-response protocol for direct peer queries
- [x] **Phase 3**: Kademlia DHT for discovery beyond LAN
- [x] **Phase 4**: Digital utility unit transaction types
- [x] **Phase 5**: Georgist resource accounting and economic engine
- [x] **Phase 6**: Storage verification with Blake3 + Merkle proofs
- [x] **Phase 6b**: Scheduled challenges with tenure decay model
- [x] **Phase 6c**: Trilateral verification receipts, relay client, DCUtR
- [ ] **Phase 7**: Relay transport composition and full p2p-circuit routing