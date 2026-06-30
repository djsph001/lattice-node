use std::collections::HashMap;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use libp2p::{
    futures::StreamExt,
    gossipsub, identity, kad, mdns, noise, request_response,
    swarm::SwarmEvent,
    tcp, yamux, Multiaddr, PeerId, StreamProtocol, SwarmBuilder,
};
use tokio::time;
use tracing::{debug, info, warn};

use crate::ledger::state::LedgerState;
use crate::ledger::types::{DigitalUtilityUnit, SignedTransaction, Transaction};
use crate::ledger::validation;
use crate::message::codec::rpc::{BalanceCodec, BalanceProtocol, LatticeCodec, LatticeProtocol};
use crate::message::codec::rpc::VerifyProtocol;
use crate::message::types::{
    BalanceRequest, BalanceResponse, Heartbeat, LatticeMessage, StatusRequest, StatusResponse,
};
use crate::message::types::{VerifyRequest, VerifyResponse};
use crate::network::protocol::{
    LatticeBehaviour, LatticeBehaviourEvent, LATTICE_HEARTBEAT_TOPIC, LATTICE_KAD_PROTOCOL,
};
use crate::state::peers::PeerTable;
use crate::economics::EconomicEngine;
use crate::storage::ProofEngine;

/// Lattice protocol version advertised in status responses.
const PROTOCOL_VERSION: u32 = 1;

/// Gossipsub topic for economic transaction propagation.
pub const LATTICE_TX_TOPIC: &str = "lattice/tx/v1";

// ── Phase 6: storage verification types ────────────────────

/// Tracks the context of an outbound storage challenge so the
/// response can be verified when it arrives asynchronously.
#[derive(Debug, Clone)]
struct PendingChallenge {
    resource_id: [u8; 32],
    chunk_index: u64,
    salt: [u8; 32],
    /// The epoch in which the challenge was issued.
    epoch: u64,
    /// The peer being challenged.
    peer: PeerId,
}

// ── Phase 6: async bridge for storage verification ──────────

/// Bridges the `!Send` Swarm from a background `spawn_blocking` task
/// back to the main event loop.  The background thread produces a
/// proof, bundles it with the `ResponseChannel`, and drops it into
/// this channel.  The main loop picks it up and sends the response
/// — safely, on the thread that owns the `Swarm`.
#[derive(Debug)]
enum InternalBridgeEvent {
    VerificationReady {
        channel: libp2p::request_response::ResponseChannel<VerifyResponse>,
        chunk_hash: [u8; 32],
        salted_hash: [u8; 32],
        merkle_proof: Vec<Vec<u8>>,
    },
}

/// A sovereign node in the Lattice mesh.
pub struct LatticeNode {
    swarm: libp2p::Swarm<LatticeBehaviour>,
    peer_table: PeerTable,
    local_peer_id: PeerId,
    /// The node's persistent keypair — used to sign economic transactions.
    local_key: identity::Keypair,
    node_name: String,
    heartbeat_interval: Duration,
    /// When the node started — used to report uptime.
    start_time: Instant,
    /// Count of heartbeats this node has broadcast.
    heartbeats_sent: u64,
    /// Monotonic nonce for correlating outbound status queries.
    query_nonce: u64,
    /// Peers we've already sent an initial status query to.
    queried_peers: HashSet<PeerId>,
    /// Whether mDNS discovery is disabled.
    no_mdns: bool,
    /// Peers discovered via mDNS.
    mdns_peers: HashSet<PeerId>,
    /// Whether a Kademlia bootstrap has been triggered.
    kad_bootstrapped: bool,
    /// Explicit bootstrap peer addresses.
    bootstrap_peers: Vec<Multiaddr>,

    // ── Phase 4: economic layer ──────────────────────────────
    /// Local ledger — this node's view of balances.
    ledger: LedgerState,
    /// Highest nonce seen per peer for replay protection.
    seen_nonces: HashMap<PeerId, u64>,
    /// Monotonically increasing nonce for our own outbound transactions.
    tx_nonce: u64,
    /// Amount to mint at startup (test bootstrapping).
    mint_on_start: Option<u64>,
    /// One-shot transfer on startup: (to_peer_id, amount).
    transfer_on_start: Option<(String, u64)>,

    // ── Phase 5: economic engine ──────────────────────────────
    /// The Georgist economic engine — metrics, minting, taxation.
    economic_engine: EconomicEngine,
    /// How often the economic cycle runs (epoch interval).
    epoch_interval: Duration,
    /// Base units minted per epoch at contribution score 1.0.
    base_mint_rate: u64,
    /// Base tax rate in percent (at contribution ratio 1.0).
    base_tax_rate: u64,

    // ── Phase 6: storage verification ──────────────────────────
    /// Directory where verified resources are stored on disk.
    storage_dir: PathBuf,
    /// Sender half of the async bridge channel — background proof
    /// tasks drop results here, main loop picks them up.
    bridge_tx: Option<tokio::sync::mpsc::Sender<InternalBridgeEvent>>,
    /// Pending outbound storage challenges, keyed by libp2p request ID.
    /// When a VerifyResponse arrives, we look up the challenge context
    /// to verify the proof against the original (resource_id, chunk_index, salt).
    pending_challenges: HashMap<
        libp2p::request_response::OutboundRequestId,
        PendingChallenge,
    >,
}

impl LatticeNode {
    /// Create a new Lattice node.
    pub fn new(
        _port: u16,
        name: Option<String>,
        heartbeat_secs: u64,
        identity_dir: Option<PathBuf>,
        fresh_identity: bool,
        no_mdns: bool,
        bootstrap_peers: Vec<Multiaddr>,
        mint_on_start: Option<u64>,
        transfer_on_start: Option<(String, u64)>,
        epoch_interval_secs: u64,
        base_mint_rate: u64,
        base_tax_rate: u64,
        storage_dir: Option<PathBuf>,
    ) -> Result<Self> {
        let key_path = resolve_identity_path(identity_dir)?;
        let local_key = load_or_generate_identity(&key_path, fresh_identity)?;
        let local_peer_id = PeerId::from(local_key.public());

        let node_name = name.unwrap_or_else(|| {
            let id_str = local_peer_id.to_string();
            format!("node-{}", &id_str[id_str.len() - 8..])
        });

        info!(
            name = %node_name,
            peer_id = %local_peer_id,
            "Generating node identity"
        );

        let swarm = SwarmBuilder::with_existing_identity(local_key.clone())
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

                let topic = gossipsub::IdentTopic::new(LATTICE_HEARTBEAT_TOPIC);
                gossipsub
                    .subscribe(&topic)
                    .map_err(|e| anyhow::anyhow!("gossipsub subscribe: {e}"))?;

                // Subscribe to transaction topic as well.
                let tx_topic = gossipsub::IdentTopic::new(LATTICE_TX_TOPIC);
                gossipsub
                    .subscribe(&tx_topic)
                    .map_err(|e| anyhow::anyhow!("gossipsub tx subscribe: {e}"))?;

                let rpc = request_response::Behaviour::new(
                    [(LatticeProtocol, request_response::ProtocolSupport::Full)],
                    request_response::Config::default(),
                );

                // Balance query RPC channel.
                let balance_rpc = request_response::Behaviour::new(
                    [(BalanceProtocol, request_response::ProtocolSupport::Full)],
                    request_response::Config::default(),
                );

                // Storage verification RPC channel (Phase 6).
                let verify_rpc = request_response::Behaviour::new(
                    [(VerifyProtocol, request_response::ProtocolSupport::Full)],
                    request_response::Config::default(),
                );

                let mut kademlia = {
                    let store = kad::store::MemoryStore::new(key.public().to_peer_id());
                    let kconfig =
                        kad::Config::new(StreamProtocol::new(LATTICE_KAD_PROTOCOL));
                    kad::Behaviour::with_config(
                        key.public().to_peer_id(),
                        store,
                        kconfig,
                    )
                };
                kademlia.set_mode(Some(kad::Mode::Server));

                Ok(LatticeBehaviour::new(
                    mdns,
                    gossipsub,
                    rpc,
                    balance_rpc,
                    verify_rpc,
                    kademlia,
                ))
            })?
            .with_swarm_config(|c| {
                c.with_idle_connection_timeout(Duration::from_secs(60))
            })
            .build();

        Ok(Self {
            swarm,
            peer_table: PeerTable::new(),
            local_peer_id,
            local_key,
            node_name,
            heartbeat_interval: Duration::from_secs(heartbeat_secs),
            start_time: Instant::now(),
            heartbeats_sent: 0,
            query_nonce: 0,
            queried_peers: HashSet::new(),
            no_mdns,
            mdns_peers: HashSet::new(),
            kad_bootstrapped: false,
            bootstrap_peers,
            ledger: LedgerState::new(),
            seen_nonces: HashMap::new(),
            tx_nonce: 0,
            mint_on_start,
            transfer_on_start,
            economic_engine: EconomicEngine::new(),
            epoch_interval: Duration::from_secs(epoch_interval_secs),
            base_mint_rate,
            base_tax_rate,
            storage_dir: storage_dir.unwrap_or_else(|| {
                PathBuf::from("./lattice-storage")
            }),
            bridge_tx: None,
            pending_challenges: HashMap::new(),
        })
    }

    /// The node's public peer ID.
    pub fn peer_id(&self) -> &PeerId {
        &self.local_peer_id
    }

    /// Main event loop.
    pub async fn run(&mut self) -> Result<()> {
        let listen_addr = format!("/ip4/0.0.0.0/tcp/{}", 0)
            .parse()
            .expect("valid multiaddr");
        self.swarm.listen_on(listen_addr)?;

        for addr in &self.bootstrap_peers {
            info!(addr = %addr, "Dialling bootstrap peer");

            // Extract the PeerId from the trailing /p2p/ segment and inject
            // the transport address into Kademlia's routing table so the DHT
            // can resolve the peer. Without this, Kademlia has no way to
            // locate peers contacted through explicit bootstrap addresses.
            if let Some(libp2p::multiaddr::Protocol::P2p(peer_id)) = addr.iter().last() {
                let mut transport_addr = addr.clone();
                transport_addr.pop(); // strip /p2p/<peer-id>, leaving /ip4/x.x.x.x/tcp/<port>
                self.swarm
                    .behaviour_mut()
                    .kademlia
                    .add_address(&peer_id, transport_addr);
                debug!(peer = %peer_id, "Added bootstrap peer to Kademlia routing table");
            } else {
                warn!(addr = %addr, "Bootstrap peer address missing /p2p/<PeerId> segment");
            }

            if let Err(e) = self.swarm.dial(addr.clone()) {
                warn!(addr = %addr, error = %e, "Failed to dial bootstrap peer");
            }
        }

        // When mDNS is disabled, the normal Kademlia bootstrap trigger
        // (inside the mDNS::Discovered handler) never fires.  Bootstrap
        // explicitly from the provided addresses instead.
        if self.no_mdns && !self.bootstrap_peers.is_empty() && !self.kad_bootstrapped {
            if let Err(e) = self.swarm.behaviour_mut().kademlia.bootstrap() {
                warn!(error = %e, "Kademlia bootstrap from explicit peers failed");
            } else {
                self.kad_bootstrapped = true;
                info!("Kademlia bootstrap initiated from explicit bootstrap peers");
            }
        }

        // Phase 4: mint starting balance if requested.
        if let Some(amount) = self.mint_on_start {
            self.mint_local(amount)?;
        }

        // Phase 4: one-shot transfer on startup.
        if let Some((ref to, amount)) = self.transfer_on_start.clone() {
            self.send_transfer(&to, amount)?;
        }

        let mut heartbeat_timer = time::interval(self.heartbeat_interval);
        let mut epoch_timer = time::interval(self.epoch_interval);

        // Phase 6: async bridge for storage verification.
        // Background tasks drop proof results here; the main loop
        // picks them up and sends responses through the Swarm.
        let (bridge_tx, mut bridge_rx) =
            tokio::sync::mpsc::channel::<InternalBridgeEvent>(100);
        self.bridge_tx = Some(bridge_tx.clone());

        info!(
            name = %self.node_name,
            heartbeat_interval = ?self.heartbeat_interval,
            epoch_interval = ?self.epoch_interval,
            base_mint_rate = self.base_mint_rate,
            base_tax_rate = self.base_tax_rate,
            no_mdns = self.no_mdns,
            "Entering event loop"
        );

        loop {
            tokio::select! {
                event = self.swarm.select_next_some() => {
                    self.handle_swarm_event(event).await;
                }
                _ = heartbeat_timer.tick() => {
                    self.broadcast_heartbeat().await;
                }
                _ = epoch_timer.tick() => {
                    self.run_economic_epoch().await;
                }
                Some(bridge_event) = bridge_rx.recv() => {
                    self.handle_bridge_event(bridge_event);
                }
            }
        }
    }

    /// Run one economic epoch: measure contribution, mint reward, tax & redistribute.
    async fn run_economic_epoch(&mut self) {
        let self_balance = self.ledger.balance_of(&self.local_peer_id);
        let epoch = self.economic_engine.epoch_count() + 1;

        let epoch_txns = self.economic_engine.run_epoch(
            &self.local_peer_id,
            self_balance,
            &self.peer_table,
            self.base_mint_rate,
            self.base_tax_rate,
        );

        // Sign and broadcast the mint transaction.
        if let Some(mut mint) = epoch_txns.mint {
            self.tx_nonce += 1;
            set_transaction_nonce(&mut mint, self.tx_nonce);
            let signed = match self.sign_transaction(&mint) {
                Ok(s) => s,
                Err(e) => {
                    warn!(error = %e, "Failed to sign epoch mint transaction");
                    return;
                }
            };
            // Apply locally.
            let mut seen = HashMap::new();
            if let Err(e) = validation::validate_and_apply(
                &signed, &mut self.ledger, &mut seen,
            ) {
                warn!(error = %e, "Failed to apply epoch mint locally");
            } else {
                for (peer, nonce) in seen {
                    self.seen_nonces.insert(peer, nonce);
                }
                self.economic_engine.metrics.record_transaction_submitted();
                self.broadcast_transaction(&signed).ok();
            }
        }

        // Sign and broadcast redistribution transfers.
        for mut transfer in epoch_txns.redistributions {
            self.tx_nonce += 1;
            set_transaction_nonce(&mut transfer, self.tx_nonce);
            let signed = match self.sign_transaction(&transfer) {
                Ok(s) => s,
                Err(e) => {
                    warn!(error = %e, "Failed to sign redistribution transfer");
                    continue;
                }
            };
            // Apply locally.
            let mut seen = HashMap::new();
            if let Err(e) = validation::validate_and_apply(
                &signed, &mut self.ledger, &mut seen,
            ) {
                warn!(error = %e, "Failed to apply redistribution locally");
            } else {
                for (peer, nonce) in seen {
                    self.seen_nonces.insert(peer, nonce);
                }
                self.economic_engine.metrics.record_transaction_submitted();
                self.broadcast_transaction(&signed).ok();
            }
        }

        // Sync heartbeats_sent from node into metrics.
        self.economic_engine.metrics.heartbeats_sent = self.heartbeats_sent;

        let new_balance = self.ledger.balance_of(&self.local_peer_id);
        let ratio = self.economic_engine.metrics.contribution_ratio();
        info!(
            epoch,
            balance_before = %self_balance,
            balance_after = %new_balance,
            ratio = %format!("{:.2}", ratio),
            "Epoch complete"
        );

        // Phase 6b: schedule storage challenges for aging claims.
        self.schedule_storage_challenges(epoch);
    }

    /// Mint units to the local node (test bootstrapping only).
    fn mint_local(&mut self, amount: u64) -> Result<()> {
        self.tx_nonce += 1;
        let tx = Transaction::Mint {
            to: self.local_peer_id.to_string(),
            amount: DigitalUtilityUnit(amount),
            authority: self.local_peer_id.to_string(),
            nonce: self.tx_nonce,
            timestamp: chrono::Utc::now(),
        };
        let signed = self.sign_transaction(&tx)?;

        info!(
            amount = amount,
            "Minting starting balance to local node"
        );

        // Apply locally first.
        let mut seen = HashMap::new();
        if let Err(e) = validation::validate_and_apply(&signed, &mut self.ledger, &mut seen) {
            warn!(error = %e, "Failed to apply local mint");
            return Err(e.into());
        }
        // Merge seen nonces.
        for (peer, nonce) in seen {
            self.seen_nonces.insert(peer, nonce);
        }

        // Broadcast so other nodes learn about it.
        self.broadcast_transaction(&signed)?;

        Ok(())
    }

    /// Create, sign, apply, and broadcast a transfer.
    fn send_transfer(&mut self, to: &str, amount: u64) -> Result<()> {
        self.tx_nonce += 1;
        let tx = Transaction::Transfer {
            from: self.local_peer_id.to_string(),
            to: to.to_string(),
            amount: DigitalUtilityUnit(amount),
            nonce: self.tx_nonce,
            timestamp: chrono::Utc::now(),
        };
        let signed = self.sign_transaction(&tx)?;

        info!(
            to = %to,
            amount = amount,
            nonce = self.tx_nonce,
            "Sending transfer"
        );

        // Validate and apply locally.
        let mut seen = HashMap::new();
        if let Err(e) = validation::validate_and_apply(&signed, &mut self.ledger, &mut seen) {
            warn!(error = %e, "Failed to apply local transfer");
            return Err(e.into());
        }
        for (peer, nonce) in seen {
            self.seen_nonces.insert(peer, nonce);
        }

        // Broadcast.
        self.broadcast_transaction(&signed)?;

        Ok(())
    }

    /// Sign a transaction with the node's keypair.
    fn sign_transaction(&self, tx: &Transaction) -> Result<SignedTransaction> {
        let tx_bytes = serde_cbor::to_vec(tx)
            .map_err(|e| anyhow::anyhow!("failed to encode transaction: {e}"))?;
        let signature = self
            .local_key
            .sign(&tx_bytes)
            .map_err(|e| anyhow::anyhow!("failed to sign transaction: {e}"))?;
        let signer_public_key = self
            .local_key
            .public()
            .encode_protobuf();

        Ok(SignedTransaction {
            transaction: tx.clone(),
            signer_public_key,
            signature,
        })
    }

    /// Broadcast a signed transaction on the transaction gossipsub topic.
    fn broadcast_transaction(&mut self, signed: &SignedTransaction) -> Result<()> {
        let msg = LatticeMessage::Transaction(signed.clone());
        let encoded = crate::message::codec::encode(&msg)
            .map_err(|e| anyhow::anyhow!("failed to encode transaction message: {e}"))?;

        let topic = gossipsub::IdentTopic::new(LATTICE_TX_TOPIC);
        match self
            .swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic, encoded)
        {
            Ok(_) => {
                debug!(nonce = signed.transaction.nonce(), "Transaction broadcast");
            }
            Err(gossipsub::PublishError::InsufficientPeers) => {
                debug!("Transaction broadcast skipped: no peers yet");
            }
            Err(e) => {
                return Err(anyhow::anyhow!("failed to broadcast transaction: {e}"));
            }
        }
        Ok(())
    }

    /// Send a balance query to a specific peer.
    fn send_balance_query(&mut self, query_peer: PeerId, target: PeerId) {
        self.query_nonce += 1;
        self.economic_engine.metrics.record_query_issued();
        let req = BalanceRequest {
            peer_id: target.to_string(),
            nonce: self.query_nonce,
        };
        let req_id = self
            .swarm
            .behaviour_mut()
            .balance_rpc
            .send_request(&query_peer, req);
        debug!(
            peer = %query_peer,
            target = %target,
            nonce = self.query_nonce,
            ?req_id,
            "Sent balance query"
        );
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
                if self.no_mdns {
                    return;
                }
                let mut new_peers = false;
                for (peer_id, addr) in peers {
                    info!(peer = %peer_id, addr = %addr, "Peer discovered");
                    self.peer_table.add_peer(peer_id, addr.clone());
                    self.mdns_peers.insert(peer_id);
                    self.swarm
                        .behaviour_mut()
                        .gossipsub
                        .add_explicit_peer(&peer_id);
                    self.swarm
                        .behaviour_mut()
                        .kademlia
                        .add_address(&peer_id, addr.clone());
                    self.swarm.dial(addr.clone()).ok();
                    new_peers = true;
                }
                if new_peers && !self.kad_bootstrapped {
                    if let Err(e) = self.swarm.behaviour_mut().kademlia.bootstrap() {
                        warn!(error = %e, "Kademlia bootstrap failed");
                    } else {
                        self.kad_bootstrapped = true;
                        info!("Kademlia bootstrap initiated");
                    }
                }
            }

            SwarmEvent::Behaviour(LatticeBehaviourEvent::Mdns(
                mdns::Event::Expired(peers),
            )) => {
                if self.no_mdns {
                    return;
                }
                for (peer_id, _addr) in peers {
                    info!(peer = %peer_id, "Peer expired");
                    self.swarm
                        .behaviour_mut()
                        .gossipsub
                        .remove_explicit_peer(&peer_id);
                    self.peer_table.remove_peer(&peer_id);
                    self.mdns_peers.remove(&peer_id);
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

            SwarmEvent::Behaviour(LatticeBehaviourEvent::BalanceRpc(
                request_response::Event::Message { peer, message },
            )) => {
                self.handle_balance_rpc(peer, message);
            }

            SwarmEvent::Behaviour(LatticeBehaviourEvent::BalanceRpc(
                request_response::Event::OutboundFailure { peer, error, .. },
            )) => {
                warn!(peer = %peer, error = ?error, "Balance query failed");
            }

            // ── Phase 6: storage verification ──────────────
            SwarmEvent::Behaviour(LatticeBehaviourEvent::VerifyRpc(
                request_response::Event::Message { peer, message },
            )) => {
                match message {
                    request_response::Message::Request { request, channel, .. } => {
                        self.handle_verify_request(peer, request, channel);
                    }
                    request_response::Message::Response { response, request_id } => {
                        self.handle_verify_response(request_id, response);
                    }
                }
            }

            SwarmEvent::Behaviour(LatticeBehaviourEvent::VerifyRpc(
                request_response::Event::OutboundFailure { peer, error, .. },
            )) => {
                debug!(peer = %peer, error = ?error, "Storage verification challenge failed");
            }

            // ── Kademlia events ──────────────────────────────
            SwarmEvent::Behaviour(LatticeBehaviourEvent::Kad(
                kad::Event::OutboundQueryProgressed { result, .. },
            )) => {
                match result {
                    kad::QueryResult::Bootstrap(result) => {
                        match result {
                            Ok(kad::BootstrapOk { peer, num_remaining }) => {
                                debug!(peer = %peer, remaining = num_remaining, "Kademlia bootstrap progressing");
                                if num_remaining == 0 {
                                    info!(peer = %peer, "Kademlia bootstrap complete");
                                }
                            }
                            Err(e) => warn!(error = ?e, "Kademlia bootstrap query failed"),
                        }
                    }
                    kad::QueryResult::GetClosestPeers(result) => {
                        match result {
                            Ok(kad::GetClosestPeersOk { key: _, peers }) => {
                                for info in peers {
                                    if self.peer_table.get(&info.peer_id).is_none() {
                                        info!(peer = %info.peer_id, "Discovered peer via Kademlia DHT");
                                        self.peer_table.insert_peer(info.peer_id);
                                    }
                                }
                            }
                            Err(e) => warn!(error = ?e, "Kademlia GetClosestPeers failed"),
                        }
                    }
                    _ => debug!(?result, "Kademlia query result"),
                }
            }

            SwarmEvent::Behaviour(LatticeBehaviourEvent::Kad(
                kad::Event::RoutingUpdated { peer, is_new_peer, addresses, .. },
            )) => {
                if is_new_peer {
                    info!(peer = %peer, addresses = ?addresses, "Kademlia routing table: peer added");
                    self.economic_engine.metrics.record_dht_record_stored();
                } else {
                    debug!(peer = %peer, "Kademlia routing table: peer evicted");
                }
            }

            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                info!(peer = %peer_id, "Connection established");
                if !self.mdns_peers.contains(&peer_id) {
                    warn!(peer = %peer_id, "Connection from non-mDNS peer");
                }
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

    /// Broadcast a heartbeat.
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
                self.economic_engine.metrics.heartbeats_sent = self.heartbeats_sent;
                self.economic_engine.metrics.record_consumption(bytes.len() as u64);
                debug!(name = %self.node_name, peers = self.peer_table.len(), "Heartbeat published");
            }
            Err(gossipsub::PublishError::InsufficientPeers) => {
                debug!("Heartbeat skipped: no gossipsub peers yet");
            }
            Err(e) => {
                warn!(error = %e, "Failed to publish heartbeat");
            }
        }
    }

    /// Handle an inbound gossip message.
    fn handle_gossip_message(&mut self, data: &[u8]) {
        // Every inbound gossip message we process is one we're
        // participating in propagating.  The gossipsub layer handles
        // the actual forwarding; we track the contribution.
        self.economic_engine.metrics.record_relay(data.len() as u64);

        let msg: LatticeMessage = match crate::message::codec::decode(data) {
            Ok(m) => m,
            Err(e) => {
                warn!(error = %e, "Failed to decode gossip message");
                return;
            }
        };

        match msg {
            LatticeMessage::Heartbeat(hb) => {
                match hb.peer_id.parse::<PeerId>() {
                    Ok(peer_id) => {
                        if self.peer_table.get(&peer_id).is_none() {
                            info!(peer = %peer_id, from = %hb.node_name, "Inserting peer from gossip");
                            self.peer_table.insert_peer(peer_id);
                        }
                        self.peer_table.record_heartbeat(&peer_id);
                        let count = self
                            .peer_table
                            .get(&peer_id)
                            .map(|i| i.heartbeats_received)
                            .unwrap_or(0);
                        info!(from = %hb.node_name, peer = %peer_id, total_heartbeats = count, "Heartbeat received");
                    }
                    Err(e) => {
                        warn!(error = %e, peer = %hb.peer_id, "Bad peer_id in heartbeat");
                    }
                }
            }
            LatticeMessage::Status(status) => {
                debug!(from = %status.node_name, "Status report received");
            }
            // ── Phase 4: transaction handling ─────────────
            LatticeMessage::Transaction(signed) => {
                info!(
                    nonce = signed.transaction.nonce(),
                    signer = %signed.transaction.signer(),
                    "Transaction received via gossipsub"
                );
                // We're relaying this economic traffic for the sender.
                self.economic_engine.metrics.record_transaction_relayed();
                match validation::validate_and_apply(
                    &signed,
                    &mut self.ledger,
                    &mut self.seen_nonces,
                ) {
                    Ok(()) => {
                        let signer: PeerId = signed.transaction.signer().parse().unwrap();
                        let balance = self.ledger.balance_of(&signer);
                        info!(
                            signer = %signer,
                            balance = %balance,
                            "Transaction applied to local ledger"
                        );
                    }
                    Err(e) => {
                        warn!(error = %e, "Invalid transaction rejected");
                    }
                }
            }
        }
    }

    /// Handle an inbound status request-response message.
    fn handle_rpc_message(
        &mut self,
        peer: PeerId,
        message: request_response::Message<StatusRequest, StatusResponse>,
    ) {
        match message {
            request_response::Message::Request {
                request, channel, ..
            } => {
                info!(from = %peer, nonce = request.nonce, "Status request received");
                let response = self.build_status_response(request.nonce);
                if self
                    .swarm
                    .behaviour_mut()
                    .rpc
                    .send_response(channel, response)
                    .is_err()
                {
                    warn!(peer = %peer, "Failed to send status response");
                }
            }
            request_response::Message::Response { response, .. } => {
                if self.peer_table.get(&peer).is_none() {
                    info!(peer = %peer, from = %response.node_name, "Inserting peer from RPC");
                    self.peer_table.insert_peer(peer);
                }
                info!(
                    from = %response.node_name,
                    peer = %peer,
                    nonce = response.nonce,
                    "Status response received"
                );
            }
        }
    }

    /// Handle an inbound balance request-response message.
    fn handle_balance_rpc(
        &mut self,
        peer: PeerId,
        message: request_response::Message<BalanceRequest, BalanceResponse>,
    ) {
        match message {
            request_response::Message::Request {
                request, channel, ..
            } => {
                let target: PeerId = match request.peer_id.parse() {
                    Ok(id) => id,
                    Err(e) => {
                        warn!(error = %e, "Invalid peer_id in balance request");
                        return;
                    }
                };
                let balance = self.ledger.balance_of(&target);
                info!(
                    from = %peer,
                    target = %target,
                    balance = %balance,
                    "Balance request received — responding"
                );
                let response = BalanceResponse {
                    peer_id: target.to_string(),
                    balance: balance.0,
                    nonce: request.nonce,
                };
                if self
                    .swarm
                    .behaviour_mut()
                    .balance_rpc
                    .send_response(channel, response)
                    .is_err()
                {
                    warn!(peer = %peer, "Failed to send balance response");
                }
            }
            request_response::Message::Response { response, .. } => {
                info!(
                    peer = %response.peer_id,
                    balance = response.balance,
                    nonce = response.nonce,
                    "Balance response received"
                );
            }
        }
    }

    fn send_status_request(&mut self, peer: PeerId) {
        self.query_nonce += 1;
        self.economic_engine.metrics.record_query_issued();
        let req = StatusRequest {
            from: self.local_peer_id.to_string(),
            nonce: self.query_nonce,
        };
        let req_id = self.swarm.behaviour_mut().rpc.send_request(&peer, req);
        debug!(peer = %peer, nonce = self.query_nonce, ?req_id, "Sent status request");
    }

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

    // ── Phase 6: storage verification ──────────────────────
    /// Default chunk size for storage verification (1 MiB).
    const STORAGE_CHUNK_SIZE: usize = 1024 * 1024;

    /// Handle an incoming `StorageChallenge` request.
    ///
    /// Spawns a `spawn_blocking` task for disk I/O, then bridges the
    /// result back to the main event loop through the mpsc channel.
    /// The `Swarm` never leaves Thread 0 — only the `ResponseChannel`
    /// is moved out.
    fn handle_verify_request(
        &mut self,
        peer: PeerId,
        request: VerifyRequest,
        channel: libp2p::request_response::ResponseChannel<VerifyResponse>,
    ) {
        let storage_dir = self.storage_dir.clone();
        let bridge_tx = self
            .bridge_tx
            .clone()
            .expect("bridge_tx not initialized");

        match request {
            VerifyRequest::StorageChallenge {
                resource_id,
                chunk_index,
                salt,
            } => {
                info!(
                    peer = %peer,
                    chunk = chunk_index,
                    "Storage challenge received — delegating to blocking thread"
                );

                // Fire-and-forget: spawn an async task that awaits
                // the blocking I/O, then ships the result (including
                // the ResponseChannel) back to the main loop.
                tokio::spawn(async move {
                    let result = tokio::task::spawn_blocking(move || {
                        ProofEngine::generate_storage_proof(
                            &storage_dir,
                            &resource_id,
                            chunk_index,
                            Self::STORAGE_CHUNK_SIZE,
                            &salt,
                        )
                    })
                    .await
                    .unwrap_or_else(|join_err| {
                        Err(
                            crate::storage::proof::ProofError::Io(
                                std::io::Error::new(
                                    std::io::ErrorKind::Other,
                                    format!("proof task panicked: {join_err}"),
                                ),
                            ),
                        )
                    });

                    match result {
                        Ok(proof_result) => {
                            let _ = bridge_tx
                                .send(InternalBridgeEvent::VerificationReady {
                                    channel,
                                    chunk_hash: proof_result.chunk_hash,
                                    salted_hash: proof_result.salted_hash,
                                    merkle_proof: proof_result.merkle_proof,
                                })
                                .await;
                        }
                        Err(e) => {
                            tracing::warn!(
                                error = %e,
                                "Storage proof generation failed"
                            );
                            // Send an empty proof — the challenger
                            // will reject it.
                            let _ = bridge_tx
                                .send(InternalBridgeEvent::VerificationReady {
                                    channel,
                                    chunk_hash: [0u8; 32],
                                    salted_hash: [0u8; 32],
                                    merkle_proof: vec![],
                                })
                                .await;
                        }
                    }
                });
            }
        }
    }

    /// Handle a bridged verification result on the main event loop.
    ///
    /// This is called from `select!` on Thread 0 — we hold `&mut self`
    /// and can safely call `swarm.behaviour_mut().verify_rpc.send_response()`.
    fn handle_bridge_event(&mut self, event: InternalBridgeEvent) {
        match event {
            InternalBridgeEvent::VerificationReady {
                channel,
                chunk_hash,
                salted_hash,
                merkle_proof,
            } => {
                let response = VerifyResponse::StorageProof {
                    chunk_hash,
                    salted_hash,
                    merkle_proof,
                };
                if self
                    .swarm
                    .behaviour_mut()
                    .verify_rpc
                    .send_response(channel, response)
                    .is_err()
                {
                    warn!("Failed to send storage proof response — channel closed");
                } else {
                    info!("Storage proof response dispatched to challenger");
                }
            }
        }
    }

    /// Handle a VerifyResponse received from a challenged peer.
    ///
    /// Looks up the pending challenge context, verifies the Merkle
    /// proof against the known resource_id, and records the outcome
    /// in the ledger.
    fn handle_verify_response(
        &mut self,
        request_id: libp2p::request_response::OutboundRequestId,
        response: VerifyResponse,
    ) {
        let challenge = match self.pending_challenges.remove(&request_id) {
            Some(c) => c,
            None => {
                warn!(
                    ?request_id,
                    "VerifyResponse for unknown request — dropped"
                );
                return;
            }
        };

        match response {
            VerifyResponse::StorageProof {
                chunk_hash,
                salted_hash,
                merkle_proof,
            } => {
                let is_valid = ProofEngine::verify_storage_proof(
                    &challenge.resource_id,
                    &chunk_hash,
                    challenge.chunk_index,
                    &challenge.salt,
                    &salted_hash,
                    &merkle_proof,
                );

                if is_valid {
                    info!(
                        resource = %hex::encode(challenge.resource_id),
                        chunk = challenge.chunk_index,
                        peer = %challenge.peer,
                        epoch = challenge.epoch,
                        "Storage proof VERIFIED — peer holds the data"
                    );
                    let reward = self.ledger.record_verification_success(
                        &challenge.resource_id,
                        &challenge.peer,
                        challenge.epoch,
                    );
                    // Mint the contribution reward.
                    if reward > 0 {
                        self.mint_verification_reward(
                            &challenge.peer,
                            reward,
                            challenge.epoch,
                        );
                    }
                } else {
                    warn!(
                        resource = %hex::encode(challenge.resource_id),
                        chunk = challenge.chunk_index,
                        peer = %challenge.peer,
                        epoch = challenge.epoch,
                        "Storage proof FAILED — peer cannot prove possession"
                    );
                    self.ledger.record_verification_failure(
                        &challenge.resource_id,
                        &challenge.peer,
                        challenge.epoch,
                    );
                }
            }
        }
    }
}

// ── Phase 6b: scheduled challenges ────────────────────────

impl LatticeNode {
    /// Fire storage challenges for all claims due for re-verification.
    ///
    /// Called at each epoch tick after the economic cycle.  Uses the
    /// deterministic `ChallengeGenerator` so every validator challenges
    /// the same chunk for a given (resource_id, epoch) pair.
    fn schedule_storage_challenges(&mut self, epoch: u64) {
        let claims_due: Vec<_> = self
            .ledger
            .get_claims_due_for_verification(epoch)
            .into_iter()
            .cloned()
            .collect();

        if claims_due.is_empty() {
            return;
        }

        info!(
            epoch,
            count = claims_due.len(),
            "Scheduling storage challenges for due claims"
        );

        for claim in &claims_due {
            let owner: PeerId = match claim.owner.parse() {
                Ok(p) => p,
                Err(e) => {
                    warn!(owner = %claim.owner, error = %e, "Invalid owner PeerId in claim");
                    continue;
                }
            };

            // Don't challenge ourselves.
            if owner == self.local_peer_id {
                continue;
            }

            let (chunk_index, salt) =
                crate::storage::challenge::ChallengeGenerator::derive_challenge(
                    &claim.resource_id,
                    claim.total_chunks,
                    epoch,
                );

            let request = VerifyRequest::StorageChallenge {
                resource_id: claim.resource_id,
                chunk_index,
                salt,
            };

            let request_id = self
                .swarm
                .behaviour_mut()
                .verify_rpc
                .send_request(&owner, request);

            self.pending_challenges.insert(
                request_id,
                PendingChallenge {
                    resource_id: claim.resource_id,
                    chunk_index,
                    salt,
                    epoch,
                    peer: owner,
                },
            );

            debug!(
                peer = %owner,
                resource = %hex::encode(claim.resource_id),
                chunk = chunk_index,
                "Storage challenge sent"
            );
        }
    }

    /// Mint a contribution reward for a successfully verified
    /// storage claim.
    ///
    /// The reward flows through the normal transaction path: sign,
    /// apply locally, broadcast via gossipsub.  Other nodes update
    /// their ledgers when they receive the gossipsub message.
    fn mint_verification_reward(
        &mut self,
        peer: &PeerId,
        amount: u64,
        _epoch: u64,
    ) {
        self.tx_nonce += 1;
        let tx = Transaction::Mint {
            to: peer.to_string(),
            amount: DigitalUtilityUnit(amount),
            authority: self.local_peer_id.to_string(),
            nonce: self.tx_nonce,
            timestamp: chrono::Utc::now(),
        };

        let signed = match self.sign_transaction(&tx) {
            Ok(s) => s,
            Err(e) => {
                warn!(error = %e, "Failed to sign verification reward mint");
                return;
            }
        };

        // Apply locally.
        let mut seen = HashMap::new();
        if let Err(e) =
            validation::validate_and_apply(&signed, &mut self.ledger, &mut seen)
        {
            warn!(error = %e, "Failed to apply verification reward");
        } else {
            for (p, nonce) in seen {
                self.seen_nonces.insert(p, nonce);
            }
            self.economic_engine
                .metrics
                .record_transaction_submitted();
            self.broadcast_transaction(&signed).ok();
            info!(
                peer = %peer,
                amount,
                "Verification contribution reward minted"
            );
        }
    }
}

// ── Identity helpers ──────────────────────────────────────────

/// Set the nonce field on a Transaction (used after TaxEngine produces
/// transactions with placeholder nonce 0).
fn set_transaction_nonce(tx: &mut Transaction, nonce: u64) {
    match tx {
        Transaction::Transfer { nonce: ref mut n, .. } => *n = nonce,
        Transaction::Mint { nonce: ref mut n, .. } => *n = nonce,
    }
}

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

fn load_or_generate_identity(path: &Path, fresh: bool) -> Result<identity::Keypair> {
    if path.exists() && !fresh {
        let bytes = std::fs::read(path)
            .with_context(|| format!("reading identity key at {}", path.display()))?;
        let key = identity::Keypair::from_protobuf_encoding(&bytes)
            .with_context(|| format!("decoding identity key at {}", path.display()))?;
        info!(path = %path.display(), "Loaded persistent identity");
        return Ok(key);
    }

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
        info!(path = %path.display(), "Generated fresh identity");
    } else {
        info!(path = %path.display(), "Generated and saved new identity");
    }
    Ok(key)
}

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
        std::fs::set_permissions(path, std::os::unix::fs::PermissionsExt::from_mode(0o600))?;
    }

    #[cfg(not(unix))]
    {
        std::fs::write(path, bytes)?;
    }

    Ok(())
}
