# Lattice Node

Sovereign peer-to-peer application layer for the Lattice mesh network.

## Phase 2 — Skeleton Node

What it does today:
- Generates an Ed25519 identity on startup
- Listens for peers on the local network via mDNS
- Establishes encrypted channels (Noise protocol, Yamux mux)
- Tracks peer state (join, leave, heartbeat)
- Broadcasts CBOR-encoded heartbeat messages

## Quick Start (Z4)

```bash
# Build and run locally
cargo run -- --name alpha --heartbeat-interval 5

# In a second terminal, simulate a second node
cargo run -- --name bravo --heartbeat-interval 5

# They should discover each other via mDNS within seconds.
# Watch for "Peer discovered" log lines.
```

## Cross-Compile for Raspberry Pi

```bash
# One-time setup: install the aarch64 target and cross-linker
rustup target add aarch64-unknown-linux-gnu
sudo apt install gcc-aarch64-linux-gnu

# Build for Pi
cargo build --release --target aarch64-unknown-linux-gnu

# The binary lands at:
# target/aarch64-unknown-linux-gnu/release/lattice-node
#
# scp it to the Pi and run:
# ./lattice-node --name pi-alpha
```

Or use the `cross` tool (Docker-based, no system linker needed):

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
│   └── protocol.rs      Composed libp2p behaviour (mDNS now, gossipsub later)
├── message/
│   ├── types.rs         Heartbeat, StatusReport, future Transaction
│   └── codec.rs         CBOR encode/decode
└── state/
    └── peers.rs         In-memory peer table
```

## Roadmap

- [x] **Phase 2a**: Persist node identity to disk (stable PeerId across restarts)
- [x] **Phase 2b**: Add gossipsub for heartbeat propagation to all peers
- [ ] **Phase 2c**: Request-response protocol for direct peer queries
- [ ] **Phase 3**: Kademlia DHT for discovery beyond LAN
- [ ] **Phase 4**: Digital utility unit transaction types
- [ ] **Phase 5**: Nash equilibrium / Georgist resource accounting
