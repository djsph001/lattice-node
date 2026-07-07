use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

mod api;
mod commit;
mod economics;
mod ingest;
mod ledger;
mod message;
mod network;
mod node;
mod sortition;
mod state;
mod storage;

use node::LatticeNode;

/// Lattice mesh node — sovereign peer-to-peer application layer
#[derive(Parser, Debug)]
#[command(name = "lattice-node", version, about)]
struct Cli {
    /// Port to listen on (0 = random available port)
    #[arg(short, long, default_value_t = 0)]
    port: u16,

    /// Human-readable node name (optional, for logging)
    #[arg(short, long)]
    name: Option<String>,

    /// Heartbeat interval in seconds
    #[arg(long, default_value_t = 10)]
    heartbeat_interval: u64,

    /// Directory to store the node's persistent identity key
    /// (defaults to ~/.lattice)
    #[arg(long)]
    identity_dir: Option<PathBuf>,

    /// Force generation of a fresh identity, overwriting any existing key.
    /// Useful when running multiple simulated nodes on one machine.
    #[arg(long)]
    fresh_identity: bool,

    /// Disable mDNS peer discovery. Used when joining a mesh via explicit
    /// bootstrap peers — the node participates in Kademlia DHT routing
    /// but does not scan the local network.
    #[arg(long)]
    no_mdns: bool,

    /// Explicit bootstrap peer address for Kademlia DHT join.
    /// Format: /ip4/<addr>/tcp/<port>/p2p/<peer-id>
    /// Repeat for multiple bootstrap peers.
    #[arg(long)]
    bootstrap_peer: Vec<String>,

    /// Amount of digital utility units to mint to this node on startup.
    /// Test bootstrapping only — in production, issuance comes from the
    /// Georgist resource accounting model (Phase 5).
    #[arg(long)]
    mint: Option<u64>,

    /// One-shot transfer on startup: <peer_id> <amount>.
    /// Format: --transfer 12D3KooW... 100
    /// Test-only — in production, transfers come from the application layer.
    #[arg(long, num_args = 2, value_names = ["PEER_ID", "AMOUNT"])]
    transfer: Option<Vec<String>>,

    // ── Phase 5: economic engine ────────────────────────────
    /// Epoch interval in seconds — how often the economic cycle runs.
    /// At each epoch boundary the node evaluates contribution, mints
    /// new units, and executes the Georgist tax/redistribution cycle.
    #[arg(long, default_value_t = 30)]
    epoch_interval: u64,

    /// Base mint rate — units minted per epoch at a contribution
    /// score of 1.0.  Higher values make contribution more rewarding.
    #[arg(long, default_value_t = 10)]
    base_mint_rate: u64,

    /// Base tax rate in percent of balance per epoch (at contribution
    /// ratio 1.0).  A node giving twice what it takes pays half this
    /// rate; a node taking twice what it gives pays double.
    #[arg(long, default_value_t = 5)]
    base_tax_rate: u64,

    // ── Phase 6: storage verification ──────────────────────
    /// Directory for verified resource storage (blake3-addressed
    /// chunk files).  Defaults to ./lattice-storage.
    #[arg(long)]
    storage_dir: Option<PathBuf>,

    // ── Deployment ──────────────────────────────────────────
    /// IP address to bind the listener to.  Defaults to 0.0.0.0
    /// (all interfaces).  On multi-homed machines with Docker
    /// bridges or multiple NICs, pin this to the actual interface
    /// IP to control what gets advertised.
    #[arg(long, default_value = "0.0.0.0")]
    listen_addr: String,

    /// Optional publicly reachable address for this node, for
    /// cases where the node is behind NAT and the bind address
    /// isn't what remote peers should dial.  Format as a full
    /// multiaddr: /ip4/<public-ip>/tcp/<port>[/p2p/<peer-id>].
    /// When set, libp2p advertises this through Kademlia so the
    /// network routes to the right place.
    #[arg(long)]
    external_addr: Option<String>,

    // ── Test / Debug ────────────────────────────────────────
    /// DEBUG ONLY: inflate self-reported relay bytes without
    /// actually relaying anything.  Used to verify that Phase 6
    /// receipt-based minting catches dishonest reporting —
    /// inflated self-reported metrics should NOT increase the
    /// verified mint amount.
    #[arg(long)]
    fake_relay_bytes: Option<u64>,

    // ── Phase 7: TCP cert ingestion ─────────────────────────
    /// Directory to watch for .pb Impact Certificate files
    /// produced by the Python sandbox orchestrator (tfb:).
    /// When a valid certificate appears, it is broadcast
    /// on the lattice/enclave-cert/v1 gossipsub topic.
    #[arg(long)]
    cert_watch_dir: Option<PathBuf>,

    // ── Phase 6c: relay server ───────────────────────────────
    /// Enable the relay server — this node accepts inbound relay
    /// reservation and circuit requests from other nodes and
    /// forwards traffic on their behalf.  Off by default; most
    /// nodes should only run the relay client, not the server.
    #[arg(long, default_value_t = false)]
    relay_server: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    info!("Lattice node starting...");

    // Parse bootstrap peer addresses from CLI strings
    let bootstrap_peers: Vec<libp2p::Multiaddr> = cli
        .bootstrap_peer
        .iter()
        .filter_map(|s| match s.parse() {
            Ok(addr) => Some(addr),
            Err(e) => {
                warn!(addr = %s, error = %e, "Invalid bootstrap peer address, skipping");
                None
            }
        })
        .collect();

    // Parse transfer: [peer_id, amount]
    let transfer = cli.transfer.and_then(|v| {
        if v.len() == 2 {
            let amount: u64 = match v[1].parse() {
                Ok(a) => a,
                Err(e) => {
                    warn!(error = %e, "Invalid transfer amount");
                    return None;
                }
            };
            Some((v[0].clone(), amount))
        } else {
            None
        }
    });

    // Bootstrap the node
    let mut node = LatticeNode::new(
        cli.port,
        cli.name,
        cli.heartbeat_interval,
        cli.identity_dir,
        cli.fresh_identity,
        cli.no_mdns,
        bootstrap_peers,
        cli.mint,
        transfer,
        cli.epoch_interval,
        cli.base_mint_rate,
        cli.base_tax_rate,
        cli.storage_dir,
        cli.listen_addr,
        cli.external_addr,
        cli.cert_watch_dir,
        cli.relay_server,
    )?;

    info!(
        peer_id = %node.peer_id(),
        "Node identity established"
    );

    // Debug: inflate self-reported relay metrics (test-only).
    // Phase 6 verifies that this does NOT increase verified minting.
    if let Some(fake_bytes) = cli.fake_relay_bytes {
        node.inflate_self_reported_relay(fake_bytes);
    }

    // Run the event loop — this is where the node lives
    node.run().await?;

    warn!("Node shutting down");
    Ok(())
}
