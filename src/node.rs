use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use libp2p::{
    futures::StreamExt,
    gossipsub, identity, mdns, noise, request_response,
    swarm::SwarmEvent,
    tcp, yamux, PeerId, SwarmBuilder,
};
use tokio::time;
use tracing::{debug, info, warn};

use crate::message::codec::rpc::LatticeProtocol;
use crate::message::types::{Heartbeat, LatticeMessage, StatusRequest, StatusResponse};
use crate::network::protocol::{
    LatticeBehaviour, LatticeBehaviourEvent, LATTICE_HEARTBEAT_TOPIC,
};
use crate::state::peers::PeerTable;

/// Lattice protocol version advertised in status responses.
const PROTOCOL_VERSION: u32 = 1;

/// A sovereign node in the Lattice mesh.
pub struct LatticeNode {
    swarm: libp2p::Swarm<LatticeBehaviour>,
    peer_table: PeerTable,
    local_peer_id: PeerId,
    node_name: String,
    heartbeat_interval: Duration,
    /// When the node started — used to report uptime.
    start_time: Instant,
    /// Count of heartbeats this node has broadcast.
    heartbeats_sent: u64,
    /// Monotonic nonce for correlating outbound status queries.
    query_nonce: u64,
    /// Peers we've already sent an initial status query to, so the handshake
    /// fires once per peer rather than once per mDNS interface re-discovery.
    queried_peers: HashSet<PeerId>,
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

                // Gossipsub for heartbeat propagation across the mesh.
                // Signed messages: each publish is authenticated by the
                // sender's keypair, so peers can trust message origin.
                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(1))
                    .validation_mode(gossipsub::ValidationMode::Strict)
                    .build()
                    .map_err(|e| anyhow::anyhow!("gossipsub config: {e}"))?;

                let mut gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config,
                )
                .map_err(|e| anyhow::anyhow!("gossipsub init: {e}"))?;

                // Subscribe to the shared heartbeat topic on startup.
                let topic = gossipsub::IdentTopic::new(LATTICE_HEARTBEAT_TOPIC);
                gossipsub
                    .subscribe(&topic)
                    .map_err(|e| anyhow::anyhow!("gossipsub subscribe: {e}"))?;

                // Request-response for direct peer queries (/lattice/rpc/v1).
                let rpc = request_response::Behaviour::new(
                    [(LatticeProtocol, request_response::ProtocolSupport::Full)],
                    request_response::Config::default(),
                );

                Ok(LatticeBehaviour::new(mdns, gossipsub, rpc))
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
            start_time: Instant::now(),
            heartbeats_sent: 0,
            query_nonce: 0,
            queried_peers: HashSet::new(),
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
                    // Register with gossipsub so heartbeats propagate to it.
                    self.swarm
                        .behaviour_mut()
                        .gossipsub
                        .add_explicit_peer(&peer_id);
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
                    self.swarm
                        .behaviour_mut()
                        .gossipsub
                        .remove_explicit_peer(&peer_id);
                    self.peer_table.remove_peer(&peer_id);
                    // Allow a re-query if this peer is rediscovered later.
                    self.queried_peers.remove(&peer_id);
                }
            }

            SwarmEvent::Behaviour(LatticeBehaviourEvent::Gossipsub(
                gossipsub::Event::Message { message, .. },
            )) => {
                self.handle_gossip_message(&message.data);
            }

            SwarmEvent::Behaviour(LatticeBehaviourEvent::Rpc(
                request_response::Event::Message { peer, message },
            )) => {
                self.handle_rpc_message(peer, message);
            }

            SwarmEvent::Behaviour(LatticeBehaviourEvent::Rpc(
                request_response::Event::OutboundFailure { peer, error, .. },
            )) => {
                warn!(peer = %peer, error = ?error, "Status request failed");
            }

            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                info!(peer = %peer_id, "Connection established");
                // Handshake: once per peer, on the first confirmed connection,
                // directly ask "who are you, what are you running" via
                // request-response. Triggering here (not on mDNS discovery)
                // means the RPC reuses the live connection instead of dialing
                // — and the dedupe set avoids one query per network interface.
                if self.queried_peers.insert(peer_id) {
                    self.send_status_request(peer_id);
                }
            }

            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                debug!(peer = %peer_id, "Connection closed");
            }

            _ => {
                debug!(?event, "Unhandled swarm event");
            }
        }
    }

    /// Send a heartbeat to all connected peers via gossipsub.
    async fn broadcast_heartbeat(&mut self) {
        let heartbeat = LatticeMessage::Heartbeat(Heartbeat {
            node_name: self.node_name.clone(),
            peer_id: self.local_peer_id.to_string(),
            timestamp: chrono::Utc::now(),
            peer_count: self.peer_table.len(),
        });

        let encoded = crate::message::codec::encode(&heartbeat);
        let bytes = match encoded {
            Ok(b) => b,
            Err(e) => {
                warn!(error = %e, "Failed to encode heartbeat");
                return;
            }
        };

        let topic = gossipsub::IdentTopic::new(LATTICE_HEARTBEAT_TOPIC);
        match self
            .swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic, bytes.clone())
        {
            Ok(_) => {
                self.heartbeats_sent += 1;
                debug!(
                    name = %self.node_name,
                    peers = self.peer_table.len(),
                    bytes = bytes.len(),
                    "Heartbeat published"
                );
            }
            Err(gossipsub::PublishError::InsufficientPeers) => {
                // No subscribed peers yet — normal at startup or when alone.
                debug!("Heartbeat skipped: no gossipsub peers yet");
            }
            Err(e) => {
                warn!(error = %e, "Failed to publish heartbeat");
            }
        }
    }

    /// Decode an inbound gossip message and update peer state.
    fn handle_gossip_message(&mut self, data: &[u8]) {
        let msg: LatticeMessage = match crate::message::codec::decode(data) {
            Ok(m) => m,
            Err(e) => {
                warn!(error = %e, "Failed to decode gossip message");
                return;
            }
        };

        match msg {
            LatticeMessage::Heartbeat(hb) => {
                // Parse the originating peer ID from the message body.
                match hb.peer_id.parse::<PeerId>() {
                    Ok(peer_id) => {
                        // TODO Phase 3: insert unknown senders into peer_table on first
                        // gossip contact — required once Kademlia brings in peers not
                        // discovered via mDNS. Until then, mDNS always populates the
                        // table before gossip arrives.
                        self.peer_table.record_heartbeat(&peer_id);
                        let count = self
                            .peer_table
                            .get(&peer_id)
                            .map(|i| i.heartbeats_received)
                            .unwrap_or(0);
                        info!(
                            from = %hb.node_name,
                            peer = %peer_id,
                            total_heartbeats = count,
                            "Heartbeat received"
                        );
                    }
                    Err(e) => {
                        warn!(error = %e, peer = %hb.peer_id, "Bad peer_id in heartbeat");
                    }
                }
            }
            LatticeMessage::Status(status) => {
                debug!(from = %status.node_name, "Status report received");
            }
        }
    }

    /// Send a direct status query to a specific peer over request-response.
    ///
    /// Each query carries a fresh nonce so the matching response can be
    /// correlated — the habit the transaction layer will rely on.
    fn send_status_request(&mut self, peer: PeerId) {
        self.query_nonce += 1;
        let req = StatusRequest {
            from: self.local_peer_id.to_string(),
            nonce: self.query_nonce,
        };
        let req_id = self
            .swarm
            .behaviour_mut()
            .rpc
            .send_request(&peer, req);
        debug!(
            peer = %peer,
            nonce = self.query_nonce,
            ?req_id,
            "Sent status request"
        );
    }

    /// Build a StatusResponse from this node's current local state.
    fn build_status_response(&self, nonce: u64) -> StatusResponse {
        StatusResponse {
            nonce,
            node_name: self.node_name.clone(),
            peer_id: self.local_peer_id.to_string(),
            timestamp: chrono::Utc::now(),
            peer_count: self.peer_table.len(),
            uptime_secs: self.start_time.elapsed().as_secs(),
            heartbeats_sent: self.heartbeats_sent,
            version: env!("CARGO_PKG_VERSION").to_string(),
            protocol_version: PROTOCOL_VERSION,
        }
    }

    /// Handle an inbound request-response message — either a peer asking us
    /// for our status (respond), or a peer's reply to our query (log/record).
    fn handle_rpc_message(
        &mut self,
        peer: PeerId,
        message: request_response::Message<StatusRequest, StatusResponse>,
    ) {
        match message {
            request_response::Message::Request {
                request, channel, ..
            } => {
                info!(
                    from = %peer,
                    nonce = request.nonce,
                    "Status request received — responding"
                );
                let response = self.build_status_response(request.nonce);
                // send_response consumes the channel; failure means the
                // requester already dropped the connection.
                if self
                    .swarm
                    .behaviour_mut()
                    .rpc
                    .send_response(channel, response)
                    .is_err()
                {
                    warn!(peer = %peer, "Failed to send status response (channel closed)");
                }
            }
            request_response::Message::Response { response, .. } => {
                // TODO Phase 3: a response from a peer not in the table is the
                // first multi-hop signal — insert it here once Kademlia lands.
                info!(
                    from = %response.node_name,
                    peer = %peer,
                    nonce = response.nonce,
                    uptime_secs = response.uptime_secs,
                    heartbeats_sent = response.heartbeats_sent,
                    peer_count = response.peer_count,
                    protocol_version = response.protocol_version,
                    "Status response received"
                );
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
