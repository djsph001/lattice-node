use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

mod message;
mod network;
mod node;
mod state;

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

    // Bootstrap the node
    let mut node = LatticeNode::new(
        cli.port,
        cli.name,
        cli.heartbeat_interval,
        cli.identity_dir,
        cli.fresh_identity,
    )?;

    info!(
        peer_id = %node.peer_id(),
        "Node identity established"
    );

    // Run the event loop — this is where the node lives
    node.run().await?;

    warn!("Node shutting down");
    Ok(())
}
