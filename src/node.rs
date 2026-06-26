use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};
use libp2p::{
    futures::StreamExt,
    identity, mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, PeerId, SwarmBuilder,
};
use tokio::time;
use tracing::{debug, info, warn};

use crate::message::types::{LatticeMessage, Heartbeat};
use crate::network::protocol::{LatticeBehaviour, LatticeBehaviourEvent};
use crate::state::peers::PeerTable;

/// A sovereign node in the Lattice mesh.
pub struct LatticeNode {
    swarm: libp2p::Swarm<LatticeBehaviour>,
    peer_table: PeerTable,
    local_peer_id: PeerId,
    node_name: String,
    heartbeat_interval: Duration,
}

impl LatticeNode {
    /// Create a new Lattice node, loading a persistent Ed25519 identity
    /// from disk if one exists (or generating and saving a fresh one).
    pub fn new(
        port: u16,
        name: Option<String>,
        heartbeat_secs: u64,
        identity_dir: Option<PathBuf>,
        fresh_identity: bool,
    ) -> Result<Self> {
        // Resolve the identity file path: <identity_dir>/identity.key,
        // defaulting to ~/.lattice/identity.key
        let key_path = resolve_identity_path(identity_dir)?;

        // Load an existing identity, or generate + persist a new one.
        let local_key = load_or_generate_identity(&key_path, fresh_identity)?;
        let local_peer_id = PeerId::from(local_key.public());

        let node_name = name.unwrap_or_else(|| {
            // Use last 8 chars of peer ID as default name
            let id_str = local_peer_id.to_string();
            format!("node-{}", &id_str[id_str.len() - 8..])
        });

        info!(
            name = %node_name,
            peer_id = %local_peer_id,
            "Generating node identity"
        );

        // Build the libp2p swarm with:
        //   - TCP transport
        //   - Noise encryption (XX handshake)
        //   - Yamux multiplexing
        //   - mDNS for local peer discovery
        let swarm = SwarmBuilder::with_existing_identity(local_key)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_behaviour(|key| {
                let mdns = mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id(),
                )?;
                Ok(LatticeBehaviour::new(mdns))
            })?
            .with_swarm_config(|c| {
                c.with_idle_connection_timeout(Duration::from_secs(60))
            })
            .build();

        Ok(Self {
            swarm,
            peer_table: PeerTable::new(),
            local_peer_id,
            node_name,
            heartbeat_interval: Duration::from_secs(heartbeat_secs),
        })
    }

    /// The node's public peer ID.
    pub fn peer_id(&self) -> &PeerId {
        &self.local_peer_id
    }

    /// Main event loop — listens, discovers, heartbeats.
    pub async fn run(&mut self) -> Result<()> {
        // Listen on all interfaces
        let listen_addr = format!("/ip4/0.0.0.0/tcp/{}", 0)
            .parse()
            .expect("valid multiaddr");
        self.swarm.listen_on(listen_addr)?;

        let mut heartbeat_timer = time::interval(self.heartbeat_interval);

        info!(
            name = %self.node_name,
            interval = ?self.heartbeat_interval,
            "Entering event loop"
        );

        loop {
            tokio::select! {
                // Handle swarm events (peer discovery, messages, etc.)
                event = self.swarm.select_next_some() => {
                    self.handle_swarm_event(event).await;
                }

                // Periodic heartbeat broadcast
                _ = heartbeat_timer.tick() => {
                    self.broadcast_heartbeat().await;
                }
            }
        }
    }

    /// Dispatch on swarm events.
    async fn handle_swarm_event(&mut self, event: SwarmEvent<LatticeBehaviourEvent>) {
        match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                info!(addr = %address, "Listening");
            }

            SwarmEvent::Behaviour(LatticeBehaviourEvent::Mdns(
                mdns::Event::Discovered(peers),
            )) => {
                for (peer_id, addr) in peers {
                    info!(
                        peer = %peer_id,
                        addr = %addr,
                        "Peer discovered"
                    );
                    self.peer_table.add_peer(peer_id, addr.clone());
                    self.swarm.dial(addr.clone()).ok();
                }
            }

            SwarmEvent::Behaviour(LatticeBehaviourEvent::Mdns(
                mdns::Event::Expired(peers),
            )) => {
                for (peer_id, addr) in peers {
                    info!(
                        peer = %peer_id,
                        "Peer expired"
                    );
                    self.peer_table.remove_peer(&peer_id);
                }
            }

            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                info!(peer = %peer_id, "Connection established");
            }

            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                debug!(peer = %peer_id, "Connection closed");
            }

            _ => {
                debug!(?event, "Unhandled swarm event");
            }
        }
    }

    /// Send a heartbeat to all connected peers.
    async fn broadcast_heartbeat(&self) {
        let heartbeat = LatticeMessage::Heartbeat(Heartbeat {
            node_name: self.node_name.clone(),
            peer_id: self.local_peer_id.to_string(),
            timestamp: chrono::Utc::now(),
            peer_count: self.peer_table.len(),
        });

        let encoded = crate::message::codec::encode(&heartbeat);
        match encoded {
            Ok(bytes) => {
                debug!(
                    name = %self.node_name,
                    peers = self.peer_table.len(),
                    bytes = bytes.len(),
                    "Heartbeat broadcast"
                );
                // TODO: actually send bytes to connected peers via
                // a custom request-response protocol or gossipsub
            }
            Err(e) => {
                warn!(error = %e, "Failed to encode heartbeat");
            }
        }
    }
}

/// Resolve the path to the identity key file.
///
/// Uses `<identity_dir>/identity.key` when a directory is given, otherwise
/// defaults to `~/.lattice/identity.key`.
fn resolve_identity_path(identity_dir: Option<PathBuf>) -> Result<PathBuf> {
    let dir = match identity_dir {
        Some(d) => d,
        None => {
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .context("HOME environment variable not set; pass --identity-dir")?;
            home.join(".lattice")
        }
    };
    Ok(dir.join("identity.key"))
}

/// Load an Ed25519 identity from `path`, or generate a new one and persist it.
///
/// When `fresh` is true, any existing key is ignored and a new identity is
/// generated and written (overwriting the old file). The key is stored in
/// libp2p's protobuf encoding with `0600` permissions (owner read/write only).
fn load_or_generate_identity(path: &Path, fresh: bool) -> Result<identity::Keypair> {
    if path.exists() && !fresh {
        let bytes = std::fs::read(path)
            .with_context(|| format!("reading identity key at {}", path.display()))?;
        let key = identity::Keypair::from_protobuf_encoding(&bytes)
            .with_context(|| format!("decoding identity key at {}", path.display()))?;
        info!(path = %path.display(), "Loaded persistent identity");
        return Ok(key);
    }

    // Generate a fresh identity and persist it.
    let key = identity::Keypair::generate_ed25519();
    let bytes = key
        .to_protobuf_encoding()
        .context("encoding new identity key")?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating identity dir {}", parent.display()))?;
    }
    write_key_file(path, &bytes)
        .with_context(|| format!("writing identity key to {}", path.display()))?;

    if fresh && path.exists() {
        info!(path = %path.display(), "Generated fresh identity (--fresh-identity)");
    } else {
        info!(path = %path.display(), "Generated and saved new identity");
    }
    Ok(key)
}

/// Write the key file with `0600` permissions on Unix.
///
/// Permissions are applied before the bytes are written so the secret is never
/// momentarily readable by other users.
fn write_key_file(path: &Path, bytes: &[u8]) -> Result<()> {
    #[cfg(unix)]
    {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(path)?;
        file.write_all(bytes)?;
        // Re-assert mode in case the file pre-existed with looser perms.
        std::fs::set_permissions(path, std::os::unix::fs::PermissionsExt::from_mode(0o600))?;
    }

    #[cfg(not(unix))]
    {
        std::fs::write(path, bytes)?;
    }

    Ok(())
}
