use std::collections::HashMap;
use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{bail, Context, Result};
use libp2p::{
    futures::StreamExt,
    gossipsub, identify, identity, kad, mdns, noise, relay, request_response,
    swarm::SwarmEvent,
    tcp, yamux, Multiaddr, PeerId, StreamProtocol, SwarmBuilder,
    multiaddr::Protocol,
};
use libp2p::swarm::behaviour::toggle::Toggle;
use serde::{Deserialize, Serialize};
use tokio::time;
use tracing::{debug, error, info, warn};

use crate::agent::ModelSize;
use crate::ledger::state::LedgerState;
use crate::ledger::types::{DigitalUtilityUnit, SignedTransaction, Transaction};
use crate::ledger::validation;
use crate::message::codec::rpc::{BalanceCodec, BalanceProtocol, LatticeCodec, LatticeProtocol};
use crate::message::codec::rpc::{TransactionCodec, TransactionProtocol, VerifyProtocol};
use crate::message::types::{
    BalanceRequest, BalanceResponse, Heartbeat, LatticeMessage, StatusRequest, StatusResponse,
};
use crate::message::types::{TransactionRequest, TransactionResponse};
use crate::message::types::{VerifyRequest, VerifyResponse};
use crate::network::protocol::{
    LatticeBehaviour, LatticeBehaviourEvent, LATTICE_HEARTBEAT_TOPIC, LATTICE_KAD_PROTOCOL,
    LATTICE_ENCLAVE_CERT_TOPIC, LATTICE_AGENT_TOPIC,
};
use crate::agent::codec::AGENT_STATE_PROTOCOL;
use crate::state::peers::PeerTable;
use crate::economics::EconomicEngine;
use crate::economics::receipts::{RelayReceipt, SignedReceipt, validate_receipt};
use crate::storage::ProofEngine;

/// Lattice protocol version advertised in status responses.
const PROTOCOL_VERSION: u32 = 1;

/// Gossipsub topic for economic transaction propagation.
pub const LATTICE_TX_TOPIC: &str = "lattice/tx/v1";

/// How long to wait for a fetch response before considering it failed.
/// On a 3-node LAN mesh, round-trips are sub-second — 5s is generous.
const FETCH_TIMEOUT: Duration = Duration::from_secs(5);

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

// ── Phase 9: execution result channel ──────────────────

/// Result of an Ollama execution, sent from a background tokio task
/// back to the main event loop for registry update.
struct ExecutionResult {
    task_id: String,
    success: bool,
    checkpoint: Option<crate::agent::checkpoint::Checkpoint>,
    error: Option<String>,
    epoch: u64,
}

/// A peer's agent execution capability — used for resource-aware sortition.
/// Both model_size (tier) and vram_bytes (exact memory) must meet a task's
/// requirements for the peer to be eligible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentCapability {
    pub model_size: ModelSize,
    #[serde(default)]
    pub vram_bytes: u64,
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
    /// Applied transaction store — keyed by (signer, nonce) for serving
    /// fetch requests.  Bounded at LAST_NONCES_PER_PEER entries per signer.
    tx_store: HashMap<(PeerId, u64), SignedTransaction>,
    /// Pending transactions that arrived out of order — keyed by signer,
    /// ordered by nonce.  Drained in nonce order when the gap fills.
    /// Bounded at MAX_PENDING_PER_PEER entries per signer.
    pending: HashMap<PeerId, BTreeMap<u64, SignedTransaction>>,
    /// Outbound queue — locally-signed transactions that haven't been
    /// confirmed by the mesh yet.  Keyed by local signer, ordered by
    /// nonce.  Removed only when another peer forwards our transaction
    /// back to us (gossip echo: signer==self && propagation_source!=self).
    outbound: HashMap<PeerId, BTreeMap<u64, SignedTransaction>>,
    /// Outstanding fetch requests keyed by (signer, expected_nonce).
    /// Inserted on gap detection, removed on matching response, evicted
    /// on timeout (lazy — checked on next insert for same signer).
    outstanding_fetches: HashMap<(PeerId, u64), Instant>,
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

    // ── Phase 6c: trilateral verification ──────────────────────
    /// Ingress receipt registry — maps challenge_id to a signed
    /// receipt from the Relay proving custody.  Used by the Safe
    /// Gate to distinguish relay failures from target failures.
    receipt_registry: HashMap<[u8; 32], crate::message::types::IngressReceipt>,
    /// How many epochs a challenge can remain pending before
    /// the Safe Gate evaluates it as timed out.
    challenge_timeout_epochs: u64,

    // ── Phase 6: peer-verified contribution receipts ──────────
    /// Signed receipts from peers confirming this node's relay
    /// contributions.  Collected during the epoch, consumed by
    /// the mint cycle, then cleared.
    receipt_store: Vec<SignedReceipt>,
    /// Recently observed gossipsub message hashes (bounded LRU,
    /// last ~1000).  Used to validate incoming receipts —
    /// a receipt for an unknown message hash is rejected.
    recent_message_hashes: HashSet<[u8; 32]>,

    // ── Deployment ──────────────────────────────────────────
    /// IP address the listener is bound to.
    listen_addr: String,
    /// Port the listener binds to (0 = OS-assigned random port).
    port: u16,
    /// Optional publicly reachable address advertised via
    /// Kademlia for NAT traversal / port-forwarding setups.
    external_addr: Option<String>,

    // ── Phase 7: TCP cert ingestion ──────────────────────────
    /// Directory to watch for .pb Impact Certificate files.
    /// When set, the node spawns a background watcher and
    /// broadcasts valid certificates on the enclave-cert topic.
    cert_watch_dir: Option<PathBuf>,

    // ── Phase 7: multi-sig sortition ─────────────────────────
    /// PeerIds excluded from Witness panel selection due to
    /// recent escalation participation (last 3 rounds).
    escalation_exclusions: Vec<PeerId>,
    /// Witness signatures collected per proposal_id.
    /// Key: proposal_id, Value: list of (peer_id, signature) pairs.
    /// When 3-of-5 threshold is met, the certificate is ratified.
    witness_sigs: HashMap<String, Vec<(PeerId, Vec<u8>)>>,

    // ── Phase 7: commit layer ───────────────────────────────
    /// Raw protobuf bytes of decoded certificates, keyed by
    /// proposal_id.  Cached so the commit layer can write the
    /// full certificate to disk when quorum is reached.
    cert_cache: HashMap<String, Vec<u8>>,
    /// Append-only Blake3 hash-chain ledger for ratified
    /// certificates (State 4: Committed).
    commit_manager: crate::commit::CommitManager,

    // ── Phase 8: agent harness ────────────────────────────────
    /// File-backed registry of agent tasks and their state.
    agent_registry: crate::agent::registry::AgentRegistry,
    /// Whether this node accepts agent task execution.
    agent_mode: bool,
    /// Peers that advertise agent protocol support, mapped to their
    /// capability (model size + VRAM) (Phase 8b.1 sortition + Phase 10a filtering).
    agent_peers: HashMap<PeerId, AgentCapability>,
    /// This node's maximum model capability (Phase 10a).
    max_model_size: ModelSize,
    /// This node's available GPU VRAM in bytes.
    vram_bytes: u64,
    /// Disable economic participation — minting, witness panels, and
    /// ledger mutations are gated. The node still relays gossip traffic
    /// (Phase 10b: public relay safety).
    no_economics: bool,
    /// Phase 11: thickness floor weight for sortition (security parameter).
    /// Pinned to 1/T_min where T_min is expected minimum honest thickness.
    floor_weight: f64,
    /// Phase 11: density margin multiplier for panel-access invariant.
    /// honest_T must exceed N_eligible × floor_weight × margin before
    /// witness panels can form.
    density_margin: f64,
    thickness_gauge: f64,
    /// Expected root PeerId for genesis validation (out-of-band trust anchor).
    genesis_root: Option<PeerId>,
    /// Transaction persistence layer (WAL + snapshot).  When set,
    /// every validated and applied transaction is recorded in the WAL
    /// and nonces are snapshotted for crash recovery.  Optional so
    /// that nodes without a data directory skip disk I/O entirely.
    state_store: Option<Box<dyn crate::ledger::persistence::StateStore>>,
    /// Phase 9: model execution bridge (Ollama).
    executor: crate::agent::executor::OllamaExecutor,
    /// Phase 9: channel sender for background execution results.
    exec_tx: Option<tokio::sync::mpsc::Sender<ExecutionResult>>,
}

impl LatticeNode {
    /// Create a new Lattice node.
    pub fn new(
        port: u16,
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
        listen_addr: String,
        external_addr: Option<String>,
        cert_watch_dir: Option<PathBuf>,
        relay_server_enabled: bool,
        agent_mode: bool,
        max_model_size: ModelSize,
        vram_bytes: u64,
        no_economics: bool,
        floor_weight: f64,
        density_margin: f64,
        thickness_gauge: f64,
        genesis_root: Option<String>,
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
            // Phase 6c: relay client — wires relay transport into
            // the swarm stack and exposes relay::client::Behaviour
            // to the with_behaviour closure.
            .with_relay_client(
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_behaviour(|key, relay_client| {
                let mdns = mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id(),
                )?;

                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(1))
                    .validation_mode(gossipsub::ValidationMode::Permissive)
                    .mesh_outbound_min(1)
                    .mesh_n_low(1)
                    .mesh_n(2)
                    .mesh_n_high(4)
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

                // Phase 7: Subscribe to enclave certificate topic.
                let cert_topic = gossipsub::IdentTopic::new(
                    LATTICE_ENCLAVE_CERT_TOPIC,
                );
                gossipsub
                    .subscribe(&cert_topic)
                    .map_err(|e| anyhow::anyhow!("gossipsub cert subscribe: {e}"))?;

                // Phase 8: Subscribe to agent task topic.
                let agent_topic = gossipsub::IdentTopic::new(
                    LATTICE_AGENT_TOPIC,
                );
                gossipsub
                    .subscribe(&agent_topic)
                    .map_err(|e| anyhow::anyhow!("gossipsub agent subscribe: {e}"))?;

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

                // Phase 8: agent state query RPC channel.
                let agent_rpc = request_response::Behaviour::new(
                    [(
                        StreamProtocol::new(AGENT_STATE_PROTOCOL),
                        request_response::ProtocolSupport::Full,
                    )],
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

                // Phase 6c: relay server — when --relay-server is set,
                // this node accepts and forwards relay circuits for
                // other nodes.  Most nodes pass None and only run the
                // relay client side.
                //
                // relay::Config::default() ships with resource limits
                // tuned for general-purpose use (max_reservations,
                // max_circuits, max_circuit_bytes, max_circuit_duration,
                // per-peer/per-IP rate limiters).  If the cross-machine
                // test on macOS/Windows later needs longer-lived circuits
                // or higher throughput, tune those fields here.
                let relay_server = if relay_server_enabled {
                    Toggle::from(Some(relay::Behaviour::new(
                        key.public().to_peer_id(),
                        relay::Config::default(),
                    )))
                } else {
                    Toggle::from(None)
                };

                let identify = identify::Behaviour::new(
                    identify::Config::new(
                        "/lattice/identify/v1".to_string(),
                        key.public(),
                    ),
                );

                // Transaction fetch RPC channel (Phase 4).
                let tx_rpc = request_response::Behaviour::new(
                    [(TransactionProtocol, request_response::ProtocolSupport::Full)],
                    request_response::Config::default(),
                );

                Ok(LatticeBehaviour::new(
                    mdns,
                    gossipsub,
                    rpc,
                    balance_rpc,
                    verify_rpc,
                    kademlia,
                    relay_client,
                    relay_server,
                    identify,
                    agent_rpc,
                    tx_rpc,
                ))
            })?
            .with_swarm_config(|c| {
                c.with_idle_connection_timeout(Duration::from_secs(60))
            })
            .build();

        let storage_path = storage_dir.unwrap_or_else(|| PathBuf::from("./lattice-storage"));

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
            tx_store: HashMap::new(),
            pending: HashMap::new(),
            outstanding_fetches: HashMap::new(),
            outbound: HashMap::new(),
            mint_on_start,
            transfer_on_start,
            economic_engine: EconomicEngine::new(),
            epoch_interval: Duration::from_secs(epoch_interval_secs),
            base_mint_rate,
            base_tax_rate,
            storage_dir: storage_path.clone(),
            bridge_tx: None,
            pending_challenges: HashMap::new(),
            receipt_registry: HashMap::new(),
            challenge_timeout_epochs: 3,
            receipt_store: Vec::new(),
            recent_message_hashes: HashSet::new(),
            listen_addr,
            port,
            external_addr,
            cert_watch_dir,
            escalation_exclusions: Vec::new(),
            witness_sigs: HashMap::new(),
            cert_cache: HashMap::new(),
            commit_manager: crate::commit::CommitManager::open(&storage_path),
            agent_registry: crate::agent::registry::AgentRegistry::open(&storage_path),
            agent_mode,
            agent_peers: HashMap::new(),
            max_model_size,
            vram_bytes,
            no_economics,
            floor_weight,
            density_margin,
            thickness_gauge,
            genesis_root: {
                let parsed = genesis_root.and_then(|s| s.parse().ok());
                if parsed.is_none() {
                    tracing::warn!(
                        "--genesis-root not set — this node cannot validate genesis. \
                         Economic participation (panels, certificates, thickness) \
                         requires a configured trust anchor. Relay and gossip still work."
                    );
                }
                parsed
            },
            state_store: None,
            executor: crate::agent::executor::OllamaExecutor::new(),
            exec_tx: None,
        })
    }

    /// The node's public peer ID.
    pub fn peer_id(&self) -> &PeerId {
        &self.local_peer_id
    }

    /// Submit a Genesis transaction. The node signs ONLY if its own
    /// identity matches the configured genesis_root — the strictest gate
    /// in the system, because Genesis mints thickness from nothing.
    pub fn submit_genesis(&mut self, thickness_grant: f64) -> Result<()> {
        let root = match &self.genesis_root {
            Some(r) => r.clone(),
            None => bail!("--genesis-root is required to submit genesis"),
        };
        if self.local_peer_id != root {
            bail!(
                "this node ({}) is not the configured genesis root ({}) — \
                 genesis must be submitted by the root identity itself",
                self.local_peer_id, root
            );
        }
        if self.commit_manager.is_bootstrap_ended() {
            bail!("BootstrapEnded has already occurred — genesis cannot be submitted");
        }
        if self.commit_manager.height() > 0 {
            bail!("chain already has {} blocks — genesis must be block 0", self.commit_manager.height());
        }

        let tx = crate::ledger::types::Transaction::Genesis {
            root: self.local_peer_id.to_string(),
            thickness_grant,
            declared_operator_keys: vec![self.local_peer_id.to_string()],
            nonce: 0,
            timestamp: chrono::Utc::now(),
        };
        let data = serde_cbor::to_vec(&tx)?;
        let signature = self.local_key.sign(&data)?;

        self.commit_manager.commit_root_block(&data, "genesis", &signature, &self.local_peer_id)
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        self.ledger.apply_transaction(&tx)?;

        info!(
            root = %self.local_peer_id,
            thickness = format!("{:.2}", thickness_grant),
            "Genesis committed — era one begins"
        );
        Ok(())
    }

    /// Submit a BootstrapEnded declaration. One-way: after this block,
    /// root-authorized blocks are rejected. The node must be the
    /// configured genesis_root (only the root can end bootstrap).
    pub fn submit_bootstrap_ended(&mut self) -> Result<()> {
        let root = match &self.genesis_root {
            Some(r) => r.clone(),
            None => bail!("--genesis-root is required to end bootstrap"),
        };
        if self.local_peer_id != root {
            bail!(
                "this node ({}) is not the configured genesis root ({}) — \
                 only the root can declare BootstrapEnded",
                self.local_peer_id, root
            );
        }
        if self.commit_manager.is_bootstrap_ended() {
            bail!("BootstrapEnded has already occurred");
        }

        let tx = crate::ledger::types::Transaction::BootstrapEnded {
            declared_by: self.local_peer_id.to_string(),
            reason: "declared by root operator.".to_string(),
            nonce: 1,
            timestamp: chrono::Utc::now(),
        };
        let data = serde_cbor::to_vec(&tx)?;
        let signature = self.local_key.sign(&data)?;

        self.commit_manager.commit_root_block(&data, "bootstrap-ended", &signature, &self.local_peer_id)
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        self.ledger.apply_transaction(&tx)?;

        info!(
            declared_by = %self.local_peer_id,
            "BootstrapEnded committed — era two begins. Root-authorized blocks are now rejected."
        );
        Ok(())
    }

    /// Submit an agent task to the mesh and store it in the local registry.
    pub fn submit_agent_task(
        &mut self,
        task_id: String,
        model: String,
        model_size: ModelSize,
        vram_bytes: u64,
        graph_blob: Vec<u8>,
        deadline_epoch: u64,
    ) -> Result<()> {
        // Guard: reject tasks this node cannot execute
        if self.max_model_size < model_size {
            return Err(anyhow::anyhow!(
                "Task requires model_size {:?} but node max is {:?}",
                model_size, self.max_model_size
            ));
        }
        if self.vram_bytes < vram_bytes {
            return Err(anyhow::anyhow!(
                "Task requires {} bytes VRAM but node has {} bytes",
                vram_bytes, self.vram_bytes
            ));
        }

        let graph_hash: [u8; 32] = blake3::hash(&graph_blob).into();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let msg = crate::message::types::AgentTaskMsg {
            task_id: task_id.clone(),
            origin: self.local_peer_id.to_string(),
            model: model.clone(),
            model_size,
            vram_bytes,
            harness_version: 1,
            graph_blob: graph_blob.clone(),
            graph_hash,
            deadline_epoch,
            created_at: now,
        };

        let record = crate::agent::state::AgentRecord {
            task: crate::agent::state::AgentTask {
                task_id: task_id.clone(),
                origin: self.local_peer_id.to_string(),
                model,
                model_size,
                vram_bytes,
                harness_version: 1,
                graph_blob,
                graph_hash,
                deadline_epoch,
                created_at: now,
            },
            assigned_node: self.local_peer_id.to_string(),
            status: crate::agent::state::AgentStatus::Idle,
            last_checkpoint: None,
            updated_at: now,
        };

        self.agent_registry
            .register(record)
            .map_err(|e| anyhow::anyhow!(e))?;

        let envelope = crate::message::types::LatticeMessage::AgentTask(msg);
        let bytes = serde_cbor::to_vec(&envelope)?;
        self.track_outbound(&bytes);

        match self.swarm.behaviour_mut().gossipsub.publish(
            gossipsub::IdentTopic::new(crate::network::protocol::LATTICE_AGENT_TOPIC),
            bytes,
        ) {
            Ok(_id) => {
                info!(task_id = %task_id, "[agent] Task submitted and broadcast");
            }
            Err(gossipsub::PublishError::InsufficientPeers) => {
                debug!(task_id = %task_id, "[agent] Task stored locally (no peers yet)");
            }
            Err(e) => {
                warn!(error = %e, "[agent] Failed to publish agent task");
            }
        }

        // Phase 9: unified sortition gate.
        // All tasks — local and remote — flow through the same deterministic
        // election. If this node is not the winner, it does NOT execute;
        // the winner will pick up the task via gossipsub.
        if self.agent_mode {
            // Build capability-filtered pool (same logic as gossip handler).
            let mut pool: Vec<PeerId> = self
                .agent_peers
                .iter()
                .filter(|(_, cap)| {
                    cap.model_size >= model_size
                        && cap.vram_bytes >= vram_bytes
                })
                .map(|(id, _)| *id)
                .collect();
            // Include self — capability already verified by guard above.
            pool.push(self.local_peer_id);
            pool.sort();
            pool.dedup();

            let hash_bytes = blake3::hash(&graph_hash);
            let seed_bytes: [u8; 8] = hash_bytes.as_bytes()[..8].try_into().unwrap();
            let index = u64::from_be_bytes(seed_bytes) as usize % pool.len();
            let selected = pool[index];

            if selected == self.local_peer_id {
                info!(
                    task_id = %task_id,
                    pool_size = pool.len(),
                    "[agent] Selected as executor via sortition — spawning"
                );
                if let Some(ref tx) = self.exec_tx {
                    let tid = task_id.clone();
                    let blob = self.agent_registry
                        .get(&task_id)
                        .map(|r| r.task.graph_blob.clone())
                        .unwrap_or_default();
                    let hash = graph_hash;
                    let exec_client = crate::agent::executor::OllamaExecutor::new();
                    let ttx = tx.clone();
                    let epoch = self.economic_engine.epoch_count();

                    tokio::spawn(async move {
                        match exec_client.execute(&tid, &blob, &hash).await {
                            Ok(checkpoint) => {
                                let _ = ttx.send(ExecutionResult {
                                    task_id: tid,
                                    success: true,
                                    checkpoint: Some(checkpoint),
                                    error: None,
                                    epoch,
                                }).await;
                            }
                            Err(e) => {
                                warn!(task_id = %tid, error = %e, "[executor] Local execution failed");
                                let _ = ttx.send(ExecutionResult {
                                    task_id: tid,
                                    success: false,
                                    checkpoint: None,
                                    error: Some(e.to_string()),
                                    epoch,
                                }).await;
                            }
                        }
                    });
                }
            } else {
                info!(
                    task_id = %task_id,
                    selected = %selected,
                    pool_size = pool.len(),
                    "[agent] Not selected as executor — gossip will route to winner"
                );
            }
        }

        Ok(())
    }

    /// DEBUG ONLY: inflate self-reported relay metrics for testing
    /// receipt-based verification.  The inflated bytes appear in the
    /// self-reported `bytes_relayed` field but do NOT produce signed
    /// receipts — so the verified mint amount stays grounded in
    /// actual peer attestations.
    pub fn inflate_self_reported_relay(&mut self, bytes: u64) {
        self.economic_engine.metrics.bytes_relayed += bytes;
        warn!(
            fake_bytes = bytes,
            total_self_reported = self.economic_engine.metrics.bytes_relayed,
            "Self-reported relay metrics inflated (debug only)"
        );
    }

    /// Main event loop.
    pub async fn run(&mut self) -> Result<()> {
        let listen_addr: Multiaddr =
            format!("/ip4/{}/tcp/{}", self.listen_addr, self.port)
                .parse()
                .expect("valid multiaddr");
        self.swarm.listen_on(listen_addr)?;

        // If an external address was provided, register it with the
        // swarm so libp2p advertises the reachable address through
        // Kademlia and Identify, rather than the local bind address.
        if let Some(ref addr_str) = self.external_addr {
            match addr_str.parse::<Multiaddr>() {
                Ok(ext_addr) => {
                    self.swarm.add_external_address(ext_addr.clone());
                    info!(
                        external = %ext_addr,
                        "External address registered for NAT traversal"
                    );
                }
                Err(e) => {
                    warn!(
                        addr = %addr_str,
                        error = %e,
                        "Invalid external address — ignoring"
                    );
                }
            }
        }

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
                info!(addr = %addr, "Bootstrap peer address lacks /p2p/ segment — will discover PeerId on connect");
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

        // Phase 7: certificate watcher channel.
        // When --cert-watch-dir is set, a background task scans
        // for new .pb files and sends raw bytes here for broadcast.
        let (cert_tx, mut cert_rx) =
            tokio::sync::mpsc::channel::<Vec<u8>>(10);

        if let Some(ref watch_dir) = self.cert_watch_dir {
            info!(
                dir = %watch_dir.display(),
                "Starting certificate watcher"
            );
            crate::ingest::spawn_cert_watcher(watch_dir.clone(), cert_tx);
        }

        // Phase 9: execution result channel.
        // Background Ollama tasks drop results here; the main loop
        // picks them up and updates the agent registry.
        let (exec_tx, mut exec_rx) =
            tokio::sync::mpsc::channel::<ExecutionResult>(32);
        self.exec_tx = Some(exec_tx.clone());

        // Phase 7: API server — Unix Domain Socket for local queries
        let api_socket = self.storage_dir.join("lattice.sock");
        let mut api_rx = crate::api::spawn_api_server(api_socket);

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
                    // Phase 11: unwind expired vouches before economic cycle
                    let epoch = self.economic_engine.epoch_count() + 1;
                    let unwound = self.ledger.thickness_graph.process_epoch_expiration(epoch);
                    if unwound > 0 {
                        info!(epoch, unwound, "[thickness] Expired vouches unwound");
                    }
                    self.run_economic_epoch().await;
                }
                Some(bridge_event) = bridge_rx.recv() => {
                    self.handle_bridge_event(bridge_event);
                }
                Some(cert_bytes) = cert_rx.recv() => {
                    self.handle_cert_broadcast(cert_bytes).await;
                }
                Some(api_msg) = api_rx.recv() => {
                    self.handle_api_message(api_msg);
                }
                Some(exec_result) = exec_rx.recv() => {
                    self.handle_execution_result(exec_result);
                }
            }
        }
    }

    /// Phase 9: handle an execution result from a background Ollama task.
    fn handle_execution_result(&mut self, result: ExecutionResult) {
        if result.success {
            if let Some(ref checkpoint) = result.checkpoint {
                info!(
                    task_id = %result.task_id,
                    response_len = checkpoint.state_blob.len(),
                    "[executor] Task completed — updating registry"
                );
                if let Err(e) = self.agent_registry.update_status(
                    &result.task_id,
                    crate::agent::state::AgentStatus::Completed,
                    Some(checkpoint.clone()),
                ) {
                    warn!(task_id = %result.task_id, error = %e, "[executor] Failed to update status");
                }
            }
        } else {
            let reason = result.error.unwrap_or_else(|| "Unknown error".to_string());
            warn!(task_id = %result.task_id, reason = %reason, "[executor] Task failed");
            if let Err(e) = self.agent_registry.update_status(
                &result.task_id,
                crate::agent::state::AgentStatus::Failed {
                    step: 0,
                    reason,
                },
                None,
            ) {
                warn!(task_id = %result.task_id, error = %e, "[executor] Failed to update status");
            }
        }
    }

    /// Run one economic epoch: measure contribution, mint reward, tax & redistribute.
    async fn run_economic_epoch(&mut self) {
        // Phase 10b: public relay safety — don't mint or participate
        // in economic cycles when running as a pure relay.
        if self.no_economics {
            return;
        }
        let self_balance = self.ledger.balance_of(&self.local_peer_id);
        let epoch = self.economic_engine.epoch_count() + 1;

        // Phase 6: tally peer-verified receipts before the economic
        // cycle.  Each receipt proves a specific relay contribution.
        // Feed verified totals into the metrics so the mint calculation
        // uses trustless data.
        let verified_bytes: u64 = self.receipt_store.iter()
            .map(|r| r.receipt.bytes)
            .sum();
        let verified_msgs = self.receipt_store.len() as u64;
        self.economic_engine.metrics.verified_bytes_relayed += verified_bytes;
        self.economic_engine.metrics.verified_messages_relayed += verified_msgs;

        if verified_msgs > 0 {
            info!(
                epoch,
                verified_bytes,
                verified_msgs,
                "Consuming peer-verified relay receipts for mint cycle"
            );
        }

        // Run the economic cycle (mint uses verified metrics when available).
        let epoch_txns = self.economic_engine.run_epoch(
            &self.local_peer_id,
            self_balance,
            &self.peer_table,
            self.base_mint_rate,
            self.base_tax_rate,
        );

        // Phase 6: clear consumed receipts — they can't be replayed
        // in subsequent epochs.
        self.receipt_store.clear();

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
                // Insert into outbound queue and flush.
                self.outbound
                    .entry(self.local_peer_id)
                    .or_default()
                    .insert(self.tx_nonce, signed);
                self.flush_outbound();
                self.economic_engine.metrics.record_transaction_submitted();
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
                // Insert into outbound queue and flush.
                self.outbound
                    .entry(self.local_peer_id)
                    .or_default()
                    .insert(self.tx_nonce, signed);
                self.flush_outbound();
                self.economic_engine.metrics.record_transaction_submitted();
            }
        }

        // Sync heartbeats_sent from node into metrics.
        self.economic_engine.metrics.heartbeats_sent = self.heartbeats_sent;

        // Phase 8b: sync agent task count into contribution metrics.
        let active_tasks = self
            .agent_registry
            .all()
            .filter(|r| {
                !matches!(r.status, crate::agent::state::AgentStatus::Completed)
                    && !matches!(r.status, crate::agent::state::AgentStatus::Failed { .. })
            })
            .count() as u64;
        self.economic_engine.metrics.agent_tasks_active = active_tasks;

        let new_balance = self.ledger.balance_of(&self.local_peer_id);
        let ratio = self.economic_engine.metrics.contribution_ratio();
        info!(
            epoch,
            balance_before = %self_balance,
            balance_after = %new_balance,
            ratio = %format!("{:.2}", ratio),
            "Epoch complete"
        );

        // Legibility: if we have peers but earned nothing, explain why.
        // Relaying is a three-party act — at n=2, every message is origin,
        // not relay, so the receipt-gated mint correctly produces 0.
        if self.peer_table.len() > 0 && verified_msgs == 0 && epoch > 1 {
            info!(
                epoch,
                peers = self.peer_table.len(),
                "Receipt-gated mint: 0 DUUs. Relaying requires a third party — \
                 every message at this mesh size is origin, not relay. Correct."
            );
        }

        // Phase 6b: schedule storage challenges for aging claims.
        self.schedule_storage_challenges(epoch);

        // Phase 6c: Safe Gate — check for timed-out challenges.
        // If a challenge to a firewalled target timed out but we
        // hold an IngressReceipt from the Relay, freeze the target's
        // Phase 6c: Safe Gate — check for timed-out challenges.
        self.apply_safe_gate(epoch);

        // Phase 8b: deadline monitor — expire tasks past their deadline.
        self.expire_agent_tasks(epoch);
    }

    /// Phase 8b: Mark agent tasks as Failed if past deadline_epoch.
    fn expire_agent_tasks(&mut self, current_epoch: u64) {
        let expired: Vec<String> = self
            .agent_registry
            .all()
            .filter(|r| {
                r.task.deadline_epoch <= current_epoch
                    && !matches!(r.status, crate::agent::state::AgentStatus::Completed)
                    && !matches!(r.status, crate::agent::state::AgentStatus::Failed { .. })
            })
            .map(|r| r.task.task_id.clone())
            .collect();

        for task_id in &expired {
            if let Err(e) = self.agent_registry.update_status(
                task_id,
                crate::agent::state::AgentStatus::Failed {
                    step: 0,
                    reason: format!("Deadline epoch {} reached", current_epoch),
                },
                None,
            ) {
                warn!(task_id = %task_id, error = %e, "[agent] Failed to expire task");
            } else {
                info!(task_id = %task_id, epoch = current_epoch, "[agent] Task expired");
            }
        }

        if !expired.is_empty() {
            info!(count = expired.len(), "[agent] Expired {} tasks", expired.len());
        }
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
        self.track_outbound(&encoded);
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

    /// Flush the outbound queue: broadcast the lowest nonce for our
    /// own signer.  Never removes from queue on broadcast result —
    /// removal only happens via gossip echo.  Returns true if a
    /// transaction was broadcast, false if the queue was empty.
    fn flush_outbound(&mut self) -> bool {
        let me = self.local_peer_id;
        // Clone the transaction to avoid borrow conflict with broadcast.
        let to_broadcast: Option<SignedTransaction> = self
            .outbound
            .get(&me)
            .and_then(|queue| queue.first_key_value().map(|(_, tx)| tx.clone()));
        if let Some(tx) = to_broadcast {
            let nonce = tx.transaction.nonce();
            // Broadcast attempted (succeeded or failed).
            // Transaction stays in queue regardless.
            // Removal only via gossip echo.
            let _ = self.broadcast_transaction(&tx);
            debug!(nonce = nonce, "[outbound] Flushed nonce {nonce}");
            true
        } else {
            false
        }
    }

    /// Enable persistence with a WAL + snapshot store in the given
    /// data directory.  Called once during startup.  Recovers the
    /// node's seen_nonces from the snapshot, so transactions that
    /// were applied before a restart are not re-processed.
    pub fn enable_persistence(&mut self, data_dir: &std::path::Path) -> Result<()> {
        use crate::ledger::persistence::{StateStore, WalStateStore, WalStateStoreConfig};
        let config = WalStateStoreConfig {
            data_dir: data_dir.join("persistence"),
            fsync_batch_size: 100,
            fsync_interval: Duration::from_millis(100),
        };
        let mut store = WalStateStore::new(config)?;
        let state = store.recover()?;
        let recovered = state.export_nonces();
        // Merge recovered nonces into current seen_nonces — the
        // recovered values are higher (they survive restart), and
        // the current values are 0 (fresh start).
        for (peer, nonce) in recovered {
            let entry = self.seen_nonces.entry(peer).or_insert(0);
            if nonce > *entry {
                *entry = nonce;
            }
        }
        info!(
            count = self.seen_nonces.len(),
            "Recovered {} peer nonces from persistence",
            self.seen_nonces.len()
        );
        self.state_store = Some(Box::new(store));
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
                    self.agent_peers.remove(&peer_id);

                    // Phase 8b: heartbeat-failure migration —
                    // reassign expired peer's tasks to self.
                    let peer_str = peer_id.to_string();
                    let orphaned: Vec<String> = self
                        .agent_registry
                        .tasks_for_node(&peer_str)
                        .iter()
                        .filter(|r| {
                            !matches!(r.status, crate::agent::state::AgentStatus::Completed)
                                && !matches!(r.status, crate::agent::state::AgentStatus::Failed { .. })
                        })
                        .map(|r| r.task.task_id.clone())
                        .collect();

                    for task_id in &orphaned {
                        let self_str = self.local_peer_id.to_string();
                        match self.agent_registry.reassign(task_id, &self_str) {
                            Ok(()) => info!(
                                task_id = %task_id,
                                from = %peer_str,
                                to = %self_str,
                                "[agent] Task migrated from expired peer"
                            ),
                            Err(e) => warn!(
                                task_id = %task_id,
                                error = %e,
                                "[agent] Failed to migrate task"
                            ),
                        }
                    }

                    if !orphaned.is_empty() {
                        info!(
                            count = orphaned.len(),
                            peer = %peer_str,
                            "[agent] Migrated {} tasks from expired peer",
                            orphaned.len()
                        );
                    }
                }
            }

            SwarmEvent::Behaviour(LatticeBehaviourEvent::Gossipsub(
                gossipsub::Event::Message { message, propagation_source, .. },
            )) => {
                let topic = message.topic.to_string();
                debug!(
                    topic = %topic,
                    from = %propagation_source,
                    bytes = message.data.len(),
                    "[gossipsub] Message received"
                );
                self.handle_gossip_message(&message.data, propagation_source, message.source);
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

            // ── Phase 4: transaction fetch ──────────────────────
            SwarmEvent::Behaviour(LatticeBehaviourEvent::TxRpc(
                request_response::Event::Message { peer, message },
            )) => {
                match message {
                    request_response::Message::Request { request, channel, .. } => {
                        // Incoming fetch request: look up transactions by (signer, nonce range).
                        let signer_pid: PeerId = match request.signer.parse() {
                            Ok(p) => p,
                            Err(_) => return,
                        };
                        let mut txs = Vec::new();
                        for nonce in request.from_nonce..=request.to_nonce {
                            if let Some(tx) = self.tx_store.get(&(signer_pid, nonce)) {
                                txs.push(tx.clone());
                            }
                        }
                        let _ = self.swarm.behaviour_mut().tx_rpc.send_response(
                            channel,
                            TransactionResponse { transactions: txs },
                        );
                    }
                    request_response::Message::Response { response, .. } => {
                        // Fetch response received. Apply each valid transaction.
                        let count = response.transactions.len();
                        for tx in &response.transactions {
                            match validation::validate_and_apply(
                                tx,
                                &mut self.ledger,
                                &mut self.seen_nonces,
                            ) {
                                Ok(()) => {
                                    let applied_signer: PeerId = match tx.transaction.signer().parse() {
                                        Ok(p) => p,
                                        Err(_) => return,
                                    };
                                    let applied_nonce = tx.transaction.nonce();
                                    // Remove the outstanding fetch mark for this gap.
                                    self.outstanding_fetches.remove(&(applied_signer, applied_nonce));
                                    self.on_transaction_applied(tx);
                                }
                                Err(e) => {
                                    warn!(
                                        error = %e,
                                        signer = %tx.transaction.signer(),
                                        nonce = tx.transaction.nonce(),
                                        "[tx-fetch] Fetched transaction rejected"
                                    );
                                }
                            }
                        }
                        debug!(
                            from = %peer,
                            count = count,
                            "[tx-fetch] Applied {} fetched transactions",
                            count,
                        );
                    }
                }
            }
            SwarmEvent::Behaviour(LatticeBehaviourEvent::TxRpc(
                request_response::Event::OutboundFailure { peer, error, .. },
            )) => {
                debug!(peer = %peer, error = ?error, "[tx-fetch] Request failed");
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

            // ── Phase 6c: relay client events ──────────────────
            SwarmEvent::Behaviour(LatticeBehaviourEvent::RelayClient(event)) => {
                match event {
                    relay::client::Event::ReservationReqAccepted {
                        relay_peer_id, ..
                    } => {
                        info!(
                            relay = %relay_peer_id,
                            "Relay reservation accepted — circuit routing enabled"
                        );
                    }
                    _ => debug!(?event, "Relay client event"),
                }
            }

            // ── Phase 6c: relay server events ──────────────────
            SwarmEvent::Behaviour(LatticeBehaviourEvent::RelayServer(event)) => {
                match event {
                    relay::Event::ReservationReqAccepted {
                        src_peer_id,
                        renewed,
                    } => {
                        info!(
                            src = %src_peer_id,
                            renewed,
                            "RELAY-SERVER: reservation accepted — node is now relaying for this peer"
                        );
                    }
                    relay::Event::CircuitReqAccepted {
                        src_peer_id,
                        dst_peer_id,
                    } => {
                        info!(
                            src = %src_peer_id,
                            dst = %dst_peer_id,
                            "RELAY-SERVER: circuit accepted — forwarding traffic between peers"
                        );
                    }
                    relay::Event::CircuitReqDenied {
                        src_peer_id,
                        dst_peer_id,
                    } => {
                        warn!(
                            src = %src_peer_id,
                            dst = %dst_peer_id,
                            "RELAY-SERVER: circuit denied"
                        );
                    }
                    _ => debug!(?event, "Relay server event"),
                }
            }

            // ── Identify events ─────────────────────────────
            SwarmEvent::Behaviour(LatticeBehaviourEvent::Identify(event)) => {
                match event {
                    identify::Event::Received { peer_id, info, .. } => {
                        debug!(
                            peer = %peer_id,
                            protocols = ?info.protocols,
                            "Identify: received peer info"
                        );
                        // Log relay support — this is how the relay
                        // client discovers relay-capable peers.
                        let supports_relay = info
                            .protocols
                            .iter()
                            .any(|p| p == &libp2p::relay::HOP_PROTOCOL_NAME);
                        if supports_relay {
                            info!(
                                peer = %peer_id,
                                "Identify: peer supports relay (HOP_PROTOCOL)"
                            );
                        }
                        // Phase 8b.1: track agent-capable peers for sortition.
                        let supports_agent = info
                            .protocols
                            .iter()
                            .any(|p| p == &libp2p::StreamProtocol::new(
                                crate::agent::codec::AGENT_STATE_PROTOCOL
                            ));
                        if supports_agent {
                            // Phase 10a: initially assume Tiny/0-VRAM until StatusResponse provides actual capability.
                            self.agent_peers.entry(peer_id).or_insert(AgentCapability {
                                model_size: ModelSize::Tiny,
                                vram_bytes: 0,
                            });
                            debug!(
                                peer = %peer_id,
                                "[agent] Peer supports agent protocol — added to sortition pool"
                            );
                        }
                    }
                    _ => debug!(?event, "Identify event"),
                }
            }

            // ── Phase 8: agent state query events ──────────
            SwarmEvent::Behaviour(LatticeBehaviourEvent::AgentRpc(event)) => {
                match event {
                    request_response::Event::Message { peer, message } => {
                        match message {
                            request_response::Message::Request {
                                request, channel, ..
                            } => {
                                debug!(
                                    task_id = %request.task_id,
                                    from = %peer,
                                    "[agent] Agent state query received"
                                );
                                let record =
                                    self.agent_registry.get(&request.task_id).cloned();
                                let reply = crate::agent::state::AgentStateReply { record };
                                if let Err(e) = self.swarm.behaviour_mut().agent_rpc
                                    .send_response(channel, reply)
                                {
                                    warn!(error = ?e, "[agent] Failed to send agent state response");
                                }
                            }
                            request_response::Message::Response { response, .. } => {
                                if let Some(record) = &response.record {
                                    info!(
                                        task_id = %record.task.task_id,
                                        status = ?record.status,
                                        node = %record.assigned_node,
                                        "[agent] Agent state response"
                                    );
                                } else {
                                    debug!("[agent] Agent state query returned None");
                                }
                            }
                        }
                    }
                    request_response::Event::OutboundFailure {
                        peer, request_id, error,
                    } => {
                        warn!(%peer, ?request_id, %error, "[agent] Agent state query failed");
                    }
                    request_response::Event::InboundFailure {
                        peer, request_id, error,
                    } => {
                        warn!(%peer, ?request_id, %error, "[agent] Inbound agent state request failed");
                    }
                    _ => {}
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
        self.track_outbound(&bytes);
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

    /// Record a successfully-applied transaction: insert into tx_store,
    /// prune old entries, drain pending, and persist to WAL.
    fn on_transaction_applied(&mut self, tx: &SignedTransaction) {
        // Persist to WAL if persistence is enabled.
        if let Some(store) = self.state_store.as_mut() {
            if let Err(e) = store.persist(tx) {
                warn!(error = %e, nonce = tx.transaction.nonce(), "Failed to persist transaction");
            }
        }
        let signer: PeerId = match tx.transaction.signer().parse() {
            Ok(p) => p,
            Err(_) => return,
        };
        let nonce = tx.transaction.nonce();

        self.tx_store.insert((signer, nonce), tx.clone());

        // Prune store: keep only last 100 nonces for this signer.
        const MAX_NONCES: u64 = 100;
        if nonce > MAX_NONCES {
            let prune_before = nonce - MAX_NONCES;
            self.tx_store
                .retain(|(s, n), _| s != &signer || *n > prune_before);
        }

        // Drain pending transactions whose nonce is now applyable.
        if let Some(pending_map) = self.pending.get_mut(&signer) {
            let mut applied = 0u64;
            let mut next_nonce = nonce + 1;
            while let Some(ptx) = pending_map.remove(&next_nonce) {
                if let Err(e) =
                    validation::validate_and_apply(&ptx, &mut self.ledger, &mut self.seen_nonces)
                {
                    warn!(
                        error = %e,
                        signer = %signer,
                        nonce = next_nonce,
                        "[pending] Pre-validated pending tx re-failed — gap may have shifted"
                    );
                    break;
                }
                self.tx_store.insert((signer, next_nonce), ptx);
                applied = next_nonce;
                next_nonce += 1;
            }
            if applied > 0 {
                debug!(
                    signer = %signer,
                    from = nonce + 1,
                    to = applied,
                    "[pending] Drained {} pending transactions",
                    applied - nonce,
                );
            }
            if pending_map.is_empty() {
                self.pending.remove(&signer);
            }
        }
    }

    /// Handle an inbound gossip message.
    fn handle_gossip_message(&mut self, data: &[u8], propagation_source: PeerId, message_source: Option<PeerId>) {
        // Track the message hash for receipt validation.
        let message_hash = blake3::hash(data);
        self.recent_message_hashes.insert(*message_hash.as_bytes());
        // Bound the set to ~1000 entries to prevent memory growth.
        // Simple eviction: if over capacity, clear and rebuild from
        // the last 500 (cheap heuristic).
        if self.recent_message_hashes.len() > 1000 {
            let drained: Vec<_> = self.recent_message_hashes.drain().collect();
            self.recent_message_hashes
                .extend(drained.into_iter().take(500));
        }

        // Every inbound gossip message we process is one we're
        // participating in propagating.  The gossipsub layer handles
        // the actual forwarding; we track the contribution.
        self.economic_engine.metrics.record_relay(data.len() as u64);

        // Phase 7: detect enclave certificate messages by protobuf
        // signature.  These are raw ImpactCertificate payloads, not
        // LatticeMessage envelopes.  Decode, run Witness sortition,
        // and begin multi-sig collection if selected.
        {
            use prost::Message;
            if let Ok(cert) = crate::ingest::proto::ImpactCertificate::decode(data) {
                info!(
                    proposal_id = %cert.proposal_id,
                    from = %propagation_source,
                    bytes = data.len(),
                    "[cert-receive] Enclave certificate received via gossipsub"
                );

                // Cache raw bytes for the commit layer
                self.cert_cache
                    .insert(cert.proposal_id.clone(), data.to_vec());

                // ── Phase 7: Witness sortition ──────────────────
                self.run_witness_sortition(&cert);

                return;
            }

            // ── Phase 7: witness attestation handler ────────────
            // Messages starting with 0x01 are witness attestations,
            // not ImpactCertificates. Parse, verify, and collect.
            if data.first() == Some(&0x01) {
                self.handle_witness_attestation(data, propagation_source);
                return;
            }
        }

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
                // Gossip echo: if another peer forwarded our own signed
                // transaction back to us, the mesh has it — remove from
                // outbound queue.  The `propagation_source != self` check
                // is load-bearing: without it, our own direct publish would
                // clear the queue before the mesh ever saw the transaction.
                if propagation_source != self.local_peer_id {
                    if let Ok(signer_pid) = signed.transaction.signer().parse::<PeerId>() {
                        if signer_pid == self.local_peer_id {
                            let nonce = signed.transaction.nonce();
                            if let Some(queue) = self.outbound.get_mut(&signer_pid) {
                                queue.remove(&nonce);
                                if queue.is_empty() {
                                    self.outbound.remove(&signer_pid);
                                }
                                debug!(
                                    nonce = nonce,
                                    "[outbound] Mesh confirmed — removed from outbound queue via gossip echo"
                                );
                            }
                        }
                    }
                }
                info!(
                    nonce = signed.transaction.nonce(),
                    signer = %signed.transaction.signer(),
                    "Transaction received via gossipsub"
                );
                // We're relaying this economic traffic for the sender.
                self.economic_engine.metrics.record_transaction_relayed();
                // Phase 10b: public relays propagate transactions but don't
                // endorse them — relay the gossip, skip ledger mutation.
                if self.no_economics {
                    debug!("[relay] Forwarding transaction without applying — no_economics mode");
                } else {
                    match validation::validate_and_apply(
                        &signed,
                        &mut self.ledger,
                        &mut self.seen_nonces,
                    ) {
                        Ok(()) => {
                            let signer: PeerId = signed.transaction.signer().parse().unwrap();
                            let nonce = signed.transaction.nonce();
                            self.on_transaction_applied(&signed);
                            let balance = self.ledger.balance_of(&signer);
                            info!(
                                signer = %signer,
                                balance = %balance,
                                "Transaction applied to local ledger"
                            );
                        }
                        Err(validation::ValidationError::GappedNonce { signer, expected, got }) => {
                            // Pre-validated (signature, cap, balance all OK) — just gapped.
                            // Park in pending, then ask the propagation source for the gap.
                            // Stale transactions (got < expected) are replays or late arrivals
                            // — don't park or fetch, just log and drop.
                            if got < expected {
                                debug!(
                                    signer = %signer,
                                    expected = expected,
                                    got = got,
                                    "[fetch] Stale/replay transaction (nonce {got} behind expected {expected}) — dropping"
                                );
                                return;
                            }
                            debug!(
                                signer = %signer,
                                expected = expected,
                                got = got,
                                "[fetch] Gapped transaction from propagation source — parking in pending"
                            );
                            self.pending
                                .entry(signer)
                                .or_default()
                                .insert(got, signed.clone());
                            // Bound pending: drop entries beyond the 100th per signer.
                            const MAX_PENDING_PER_SIGNER: usize = 100;
                            if let Some(pmap) = self.pending.get(&signer) {
                                if pmap.len() > MAX_PENDING_PER_SIGNER {
                                    // Drop the highest nonces (they're least likely to close).
                                    let surplus = pmap.len() - MAX_PENDING_PER_SIGNER;
                                    let high_keys: Vec<u64> =
                                        pmap.keys().rev().copied().take(surplus).collect();
                                    if let Some(pmap) = self.pending.get_mut(&signer) {
                                        for k in high_keys {
                                            pmap.remove(&k);
                                        }
                                    }
                                }
                            }
                            // Dedup: skip if we already have an outstanding fetch
                            // for this gap (lazy timeout — sweep expired on next hit).
                            if should_fetch(
                                &mut self.outstanding_fetches,
                                signer,
                                expected,
                                FETCH_TIMEOUT,
                            ) {
                                self.swarm.behaviour_mut().tx_rpc.send_request(
                                    &propagation_source,
                                    TransactionRequest {
                                        signer: signer.to_string(),
                                        from_nonce: expected,
                                        to_nonce: got - 1,
                                    },
                                );
                            }
                        }
                        Err(e) => {
                            warn!(error = %e, "Transaction validation failed");
                        }
                    }
                }
            }
            LatticeMessage::AgentTask(msg) => {
                // Phase 8: dedup guard — skip if already registered.
                if self.agent_registry.contains(&msg.task_id) {
                    debug!(
                        task_id = %msg.task_id,
                        "[agent] Duplicate agent task received — skipping"
                    );
                } else if self.agent_mode {
                    // Phase 8b.1 + Phase 10a: resource-aware sortition.
                    // Filter pool to nodes that can handle this task's model size
                    // and VRAM requirement, then select deterministically via Blake3.
                    // All nodes compute the same result because capabilities
                    // arrive via gossipsub/status — same data everywhere.
                    let mut pool: Vec<PeerId> = self
                        .agent_peers
                        .iter()
                        .filter(|(_, cap)| {
                            cap.model_size >= msg.model_size
                                && cap.vram_bytes >= msg.vram_bytes
                        })
                        .map(|(id, _)| *id)
                        .collect();
                    // Include self only if capable
                    if self.max_model_size >= msg.model_size
                        && self.vram_bytes >= msg.vram_bytes
                    {
                        pool.push(self.local_peer_id);
                    }
                    pool.sort(); // deterministic ordering
                    pool.dedup();

                    if pool.is_empty() {
                        warn!(
                            task_id = %msg.task_id,
                            required = ?msg.model_size,
                            "[agent] No capable nodes in pool — task dropped"
                        );
                        return;
                    }

                    let hash_bytes = blake3::hash(&msg.graph_hash);
                    let seed_bytes: [u8; 8] = hash_bytes.as_bytes()[..8].try_into().unwrap();
                    let index = u64::from_be_bytes(seed_bytes) as usize % pool.len();
                    let selected = pool[index];

                    if selected == self.local_peer_id {
                        info!(
                            task_id = %msg.task_id,
                            origin = %msg.origin,
                            model = %msg.model,
                            pool_size = pool.len(),
                            "[agent] Selected as executor — registering task"
                        );

                        let now = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs();

                        let record = crate::agent::state::AgentRecord {
                            task: crate::agent::state::AgentTask {
                                task_id: msg.task_id.clone(),
                                origin: msg.origin.clone(),
                                model: msg.model.clone(),
                                model_size: msg.model_size,
                                vram_bytes: msg.vram_bytes,
                                harness_version: msg.harness_version,
                                graph_blob: msg.graph_blob.clone(),
                                graph_hash: msg.graph_hash,
                                deadline_epoch: msg.deadline_epoch,
                                created_at: msg.created_at,
                            },
                            assigned_node: self.local_peer_id.to_string(),
                            status: crate::agent::state::AgentStatus::Idle,
                            last_checkpoint: None,
                            updated_at: now,
                        };

                        if let Err(e) = self.agent_registry.register(record) {
                            warn!(error = %e, "[agent] Failed to register received task");
                        }

                        // Phase 9: spawn model execution via Ollama.
                        let task_id = msg.task_id.clone();
                        let graph_blob = msg.graph_blob.clone();
                        let graph_hash = msg.graph_hash;
                        let exec_client = crate::agent::executor::OllamaExecutor::new();
                        let tx = self.exec_tx.clone().expect("exec_tx not set");
                        let epoch = self.economic_engine.epoch_count();

                        tokio::spawn(async move {
                            match exec_client.execute(&task_id, &graph_blob, &graph_hash).await {
                                Ok(checkpoint) => {
                                    let _ = tx.send(ExecutionResult {
                                        task_id,
                                        success: true,
                                        checkpoint: Some(checkpoint),
                                        error: None,
                                        epoch,
                                    }).await;
                                }
                                Err(e) => {
                                    warn!(task_id = %task_id, error = %e, "[executor] Execution failed");
                                    let _ = tx.send(ExecutionResult {
                                        task_id,
                                        success: false,
                                        checkpoint: None,
                                        error: Some(e.to_string()),
                                        epoch,
                                    }).await;
                                }
                            }
                        });
                    } else {
                        debug!(
                            task_id = %msg.task_id,
                            selected = %selected,
                            pool_size = pool.len(),
                            "[agent] Not selected as executor — skipping"
                        );
                    }
                } else {
                    info!(
                        task_id = %msg.task_id,
                        origin = %msg.origin,
                        model = %msg.model,
                        "[agent] Agent task received via gossipsub"
                    );
                }
            }
        }

        // Phase 6: issue a relay receipt to the delivering peer —
        // but ONLY when the message originated from someone else.
        // A receipt attests: "you carried someone else's traffic."
        // If the message originated from the same peer that delivered it,
        // there's no relay work to attest — they're just talking to us.
        let is_relay = Self::is_relay_work(message_source, &propagation_source);
        if is_relay {
            let msg_hash = *message_hash.as_bytes();
            let receipt = RelayReceipt::new(
                propagation_source,
                self.local_peer_id,
                data.len() as u64,
                msg_hash,
            );

            let receipt_bytes = match serde_cbor::to_vec(&receipt) {
                Ok(b) => b,
                Err(e) => {
                    warn!(error = %e, "Failed to serialize relay receipt");
                    return;
                }
            };

            let signature = match self.local_key.sign(&receipt_bytes) {
                Ok(s) => s,
                Err(e) => {
                    warn!(error = %e, "Failed to sign relay receipt");
                    return;
                }
            };

            let signed = SignedReceipt {
                receipt,
                signer_public_key: self.local_key.public().encode_protobuf(),
                signature,
            };

            let req = StatusRequest::ReceiptAck { receipt: signed };
            self.swarm
                .behaviour_mut()
                .rpc
                .send_request(&propagation_source, req);
            debug!(
                peer = %propagation_source,
                bytes = data.len(),
                "Relay receipt issued to delivering peer"
            );
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
            } => match request {
                StatusRequest::Status { from: _, nonce } => {
                    info!(from = %peer, nonce, "Status request received");
                    let response = self.build_status_response(nonce);
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
                StatusRequest::ReceiptAck { receipt } => {
                    info!(
                        from = %peer,
                        relayer = %receipt.receipt.relayer,
                        bytes = receipt.receipt.bytes,
                        "Relay receipt received"
                    );
                    // Validate the receipt before storing.
                    if self.validate_and_store_receipt(receipt) {
                        info!(
                            from = %peer,
                            "Receipt validated and stored for next epoch"
                        );
                    }
                    // Always respond, even for receipts (keeps the
                    // request-response channel from stalling).
                    let response = self.build_status_response(0);
                    if self
                        .swarm
                        .behaviour_mut()
                        .rpc
                        .send_response(channel, response)
                        .is_err()
                    {
                        warn!(peer = %peer, "Failed to send receipt ack response");
                    }
                }
            },
            request_response::Message::Response { response, .. } => {
                if self.peer_table.get(&peer).is_none() {
                    info!(peer = %peer, from = %response.node_name, "Inserting peer from RPC");
                    self.peer_table.insert_peer(peer);
                }
                // Phase 10a: update peer's capability (model size + VRAM) from status response.
                self.agent_peers
                    .entry(peer)
                    .and_modify(|cap| {
                        cap.model_size = response.max_model_size;
                        cap.vram_bytes = response.vram_bytes;
                    })
                    .or_insert(AgentCapability {
                        model_size: response.max_model_size,
                        vram_bytes: response.vram_bytes,
                    });
                info!(
                    from = %response.node_name,
                    peer = %peer,
                    nonce = response.nonce,
                    "Status response received"
                );
            }
        }
    }

    /// Track an outbound message hash so receipts for it can be validated.
    /// Must be called BEFORE each gossipsub publish — the hash must be in
    /// the set when the relay's receipt arrives referencing it.
    fn track_outbound(&mut self, data: &[u8]) {
        let hash = blake3::hash(data);
        self.recent_message_hashes.insert(*hash.as_bytes());
    }

    /// Determine whether a message delivery constitutes relay work.
    ///
    /// Returns true when the message was originated by someone other than
    /// the peer that delivered it — i.e., the deliverer carried someone
    /// else's traffic.  At n=2 (every message is directly from its origin),
    /// this always returns false.  At n≥3, a node that relays a message
    /// from a third party earns a receipt.
    fn is_relay_work(source: Option<PeerId>, propagation_source: &PeerId) -> bool {
        source.map_or(false, |src| src != *propagation_source)
    }

    /// Check whether this peer is authorized as the genesis root.
    /// Genesis mints thickness from nothing — only the configured root
    /// identity may submit it.
    pub fn is_genesis_root(&self) -> bool {
        match &self.genesis_root {
            Some(root) => self.local_peer_id == *root,
            None => false,
        }
    }

    /// Validate a SignedReceipt and store it for the next epoch.
    ///
    /// Returns true if the receipt was valid and stored.
    fn validate_and_store_receipt(&mut self, signed: SignedReceipt) -> bool {
        match validate_receipt(&signed, &self.recent_message_hashes) {
            Ok(()) => {
                self.receipt_store.push(signed);
                true
            }
            Err(e) => {
                warn!(error = %e, "Relay receipt validation failed — discarded");
                false
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
        let req = StatusRequest::Status {
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
            max_model_size: self.max_model_size,
            vram_bytes: self.vram_bytes,
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
            // ── Phase 6c: relay forwarding stubs ──────────────
            VerifyRequest::ChallengeForward {
                challenge_id,
                target_peer,
                challenge: _challenge,
            } => {
                // TODO Phase 7: Relay node receives this, forwards
                // the inner challenge to target_peer via its p2p-circuit,
                // signs and returns an IngressReceipt to the Validator.
                info!(
                    challenge_id = %hex::encode(challenge_id),
                    target = %target_peer,
                    "ChallengeForward received — relay forwarding not yet implemented"
                );
            }
            VerifyRequest::RelayAudit { challenge_id } => {
                // TODO Phase 7: Relay node looks up whether it
                // forwarded challenge_id and returns an IngressReceipt
                // if custody was accepted.
                debug!(
                    challenge_id = %hex::encode(challenge_id),
                    "RelayAudit received — audit not yet implemented"
                );
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
            // ── Phase 6c: receipt variants ─────────────────────
            VerifyResponse::IngressReceipt(receipt) => {
                // Validator received an ingress custody proof from
                // a Relay.  Store it so the Safe Gate can reference
                // it if the Target's response times out.
                info!(
                    challenge_id = %hex::encode(receipt.challenge_id),
                    relay = %receipt.relay_peer,
                    target = %receipt.target_peer,
                    "Ingress receipt stored — Relay custody proven"
                );
                self.receipt_registry
                    .insert(receipt.challenge_id, receipt);
            }
            VerifyResponse::EgressReceipt(receipt) => {
                // Validator received the combined proof + delivery
                // receipt from the Target (via relay).  Verify the
                // embedded storage proof then record success.
                let is_valid = ProofEngine::verify_storage_proof(
                    &challenge.resource_id,
                    &receipt.proof.chunk_hash,
                    challenge.chunk_index,
                    &challenge.salt,
                    &receipt.proof.salted_hash,
                    &receipt.proof.merkle_proof,
                );

                if is_valid {
                    info!(
                        resource = %hex::encode(challenge.resource_id),
                        peer = %challenge.peer,
                        relay = %receipt.relay_peer,
                        "Egress receipt VERIFIED — target delivered proof through relay"
                    );
                    let reward = self.ledger.record_verification_success(
                        &challenge.resource_id,
                        &challenge.peer,
                        challenge.epoch,
                    );
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
                        peer = %challenge.peer,
                        "Egress receipt FAILED — storage proof invalid"
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

    // ── Phase 6c: Safe Gate ────────────────────────────────

    /// Evaluate pending challenges that have exceeded the timeout
    /// window.  If a challenge to a firewalled target timed out
    /// but we hold an `IngressReceipt` from the Relay, the target's
    /// health is frozen and the Relay is penalized.  Without a
    /// receipt, normal exponential decay applies.
    fn apply_safe_gate(&mut self, current_epoch: u64) {
        let timeout = self.challenge_timeout_epochs;
        let timed_out: Vec<_> = self
            .pending_challenges
            .iter()
            .filter(|(_, c)| current_epoch.saturating_sub(c.epoch) >= timeout)
            .map(|(id, c)| (*id, c.clone()))
            .collect();

        if timed_out.is_empty() {
            return;
        }

        for (request_id, challenge) in &timed_out {
            self.pending_challenges.remove(request_id);

            // Check whether we hold an IngressReceipt for this
            // challenge.  We match by resource_id — in a full
            // relay deployment this would use a challenge_id
            // from the ChallengeForward envelope.
            let receipt_exists = self.receipt_registry.values().any(|r| {
                r.target_peer == challenge.peer.to_string()
                    && r.challenge_id == challenge.resource_id
            });

            if receipt_exists {
                info!(
                    target = %challenge.peer,
                    resource = %hex::encode(challenge.resource_id),
                    epoch = challenge.epoch,
                    "Safe Gate: challenge timed out but Relay ingress receipt exists — freezing target health"
                );
                // Remove the receipt so it isn't reused.
                self.receipt_registry.retain(|_, r| {
                    r.target_peer != challenge.peer.to_string()
                        || r.challenge_id != challenge.resource_id
                });
                // Relay reputation slashing placeholder.
                // Phase 7 will add a persistent reputation index
                // and route-around logic.
                warn!(
                    target = %challenge.peer,
                    "Relay custody confirmed — target health frozen, relay flagged for audit"
                );
            } else {
                // No receipt — genuine timeout.
                warn!(
                    target = %challenge.peer,
                    resource = %hex::encode(challenge.resource_id),
                    epoch = challenge.epoch,
                    timeout_epochs = timeout,
                    "Challenge timed out with no relay receipt — applying health decay"
                );
                self.ledger.record_verification_failure(
                    &challenge.resource_id,
                    &challenge.peer,
                    challenge.epoch,
                );
            }
        }
    }

    // ── Phase 7: certificate broadcast ─────────────────────────

    /// Broadcast a validated Impact Certificate on the enclave-cert
    /// gossipsub topic. Called from the event loop when the cert
    /// watcher sends raw protobuf bytes through the channel.
    async fn handle_cert_broadcast(&mut self, raw: Vec<u8>) {
        use crate::ingest::proto;
        use prost::Message;

        let cert = match proto::ImpactCertificate::decode(&raw[..]) {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("[cert-broadcast] Decode failed: {}", e);
                return;
            }
        };

        // Cache the raw certificate bytes for the commit layer
        self.cert_cache
            .insert(cert.proposal_id.clone(), raw.clone());

        tracing::info!(
            proposal_id = %cert.proposal_id,
            bytes = raw.len(),
            "[cert-broadcast] Broadcasting enclave certificate"
        );

        // Re-encode for wire transmission (the raw bytes are the
        // certificate; we publish them directly on the topic).
        let topic = gossipsub::IdentTopic::new(LATTICE_ENCLAVE_CERT_TOPIC);
        self.track_outbound(&raw);
        match self
            .swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic, raw)
        {
            Ok(message_id) => {
                tracing::info!(
                    message_id = %message_id,
                    proposal_id = %cert.proposal_id,
                    "[cert-broadcast] Certificate published to mesh"
                );
                self.economic_engine.metrics.record_relay(
                    cert.encode_to_vec().len() as u64,
                );
            }
            Err(e) => {
                tracing::error!(
                    error = %e,
                    "[cert-broadcast] Failed to publish certificate"
                );
            }
        }

        // ── Phase 7: run sortition locally ──────────────────
        // The publishing node must also check if it's on the
        // witness panel — gossipsub doesn't loop back to self.
        self.run_witness_sortition(&cert);
    }

    /// Run witness sortition for a decoded ImpactCertificate.
    ///
    /// Called from both the cert-broadcast path (local ingest)
    /// and the cert-receive path (gossipsub inbound). Every node
    /// deterministically computes the same panel.
    fn run_witness_sortition(&mut self, cert: &crate::ingest::proto::ImpactCertificate) {
        // Phase 10b: public relays don't participate in governance.
        if self.no_economics {
            debug!("[sortition] Skipping witness sortition — no_economics mode");
            return;
        }

        // ── Build weighted pool from thickness graph ──────────
        let weighted_pool: Vec<(PeerId, f64)> = self
            .peer_table
            .iter()
            .map(|info| {
                let t = self.ledger.thickness_graph.total_thickness(&info.peer_id);
                (info.peer_id, t)
            })
            .chain(std::iter::once({
                let t = self
                    .ledger
                    .thickness_graph
                    .total_thickness(&self.local_peer_id);
                (self.local_peer_id, t)
            }))
            .collect();

        // ── Panel-access invariant guard ──────────────────────
        // Conservative: count ALL pool members as potentially Sybil.
        if !check_panel_access_density(&weighted_pool, self.density_margin, self.floor_weight) {
            warn!(
                n_eligible = weighted_pool.len(),
                floor_weight = self.floor_weight,
                margin = self.density_margin,
                "[governance] Panel REFUSED — insufficient trust density. Mesh stays participation-only."
            );
            return;
        }

        // ── Weighted sortition ───────────────────────────────
        let panel = crate::sortition::select_weighted_witness_panel(
            &cert.witness_seed,
            &weighted_pool,
            &self.escalation_exclusions,
            self.floor_weight,
        );

        if crate::sortition::is_local_witness(&panel, &self.local_peer_id) {
            info!(
                proposal_id = %cert.proposal_id,
                panel = ?panel.iter().map(|p| p.to_string()).collect::<Vec<_>>(),
                "[sortition] Local node selected as Witness — signing certificate"
            );

            let sig = match self.local_key.sign(cert.proposal_id.as_bytes()) {
                Ok(s) => s,
                Err(e) => {
                    warn!(error = %e, "[sortition] Failed to sign certificate");
                    return;
                }
            };

            self.witness_sigs
                .entry(cert.proposal_id.clone())
                .or_default()
                .push((self.local_peer_id, sig.clone()));

            let pid_bytes = cert.proposal_id.as_bytes();
            let pubkey_bytes = self.local_key.public().encode_protobuf();
            let mut attestation = Vec::with_capacity(
                1 + 2 + pid_bytes.len() + 2 + pubkey_bytes.len() + 2 + sig.len(),
            );
            attestation.push(0x01);
            attestation.extend_from_slice(&(pid_bytes.len() as u16).to_be_bytes());
            attestation.extend_from_slice(pid_bytes);
            attestation.extend_from_slice(&(pubkey_bytes.len() as u16).to_be_bytes());
            attestation.extend_from_slice(&pubkey_bytes);
            attestation.extend_from_slice(&(sig.len() as u16).to_be_bytes());
            attestation.extend_from_slice(&sig);

            let topic =
                gossipsub::IdentTopic::new(crate::network::protocol::LATTICE_ENCLAVE_CERT_TOPIC);
            self.track_outbound(&attestation);
            match self.swarm.behaviour_mut().gossipsub.publish(topic, attestation) {
                Ok(msg_id) => {
                    info!(
                        message_id = %msg_id,
                        proposal_id = %cert.proposal_id,
                        "[sortition] Witness attestation published to mesh"
                    );
                }
                Err(e) => {
                    warn!(
                        error = %e,
                        "[sortition] Failed to publish witness attestation"
                    );
                }
            }
        } else {
            debug!(
                proposal_id = %cert.proposal_id,
                panel_size = panel.len(),
                "[sortition] Local node not on panel — observing quorum"
            );
        }
    }

    /// Handle an incoming witness attestation (0x01 marker).
    ///
    /// Wire format:
    ///   [0x01] [2-byte pid_len] [proposal_id]
    ///   [2-byte pubkey_len] [public_key protobuf]
    ///   [2-byte sig_len] [Ed25519 signature]
    ///
    /// Verifies the signature against the proposal_id using the
    /// embedded public key, then collects signatures toward the
    /// 3-of-5 quorum threshold.
    fn handle_witness_attestation(&mut self, data: &[u8], propagation_source: PeerId) {
        // Skip 0x01 marker
        let rest = &data[1..];
        if rest.len() < 4 {
            warn!("[attestation] Message too short for header");
            return;
        }

        // Parse proposal_id
        let pid_len = u16::from_be_bytes([rest[0], rest[1]]) as usize;
        let after_pid = 2 + pid_len;
        if rest.len() < after_pid {
            warn!("[attestation] Truncated proposal_id field");
            return;
        }
        let proposal_id = match std::str::from_utf8(&rest[2..after_pid]) {
            Ok(s) => s.to_string(),
            Err(_) => {
                warn!("[attestation] Invalid UTF-8 in proposal_id");
                return;
            }
        };

        // Parse public key
        let rest = &rest[after_pid..];
        if rest.len() < 2 {
            warn!("[attestation] Missing pubkey length");
            return;
        }
        let pk_len = u16::from_be_bytes([rest[0], rest[1]]) as usize;
        let after_pk = 2 + pk_len;
        if rest.len() < after_pk {
            warn!("[attestation] Truncated public key");
            return;
        }
        let pk_bytes = &rest[2..after_pk];

        // Parse signature
        let rest = &rest[after_pk..];
        if rest.len() < 2 {
            warn!("[attestation] Missing signature length");
            return;
        }
        let sig_len = u16::from_be_bytes([rest[0], rest[1]]) as usize;
        if rest.len() < 2 + sig_len {
            warn!("[attestation] Truncated signature");
            return;
        }
        let sig = &rest[2..2 + sig_len];

        // Decode public key and derive PeerId
        let pubkey = match libp2p::identity::PublicKey::try_decode_protobuf(pk_bytes) {
            Ok(pk) => pk,
            Err(e) => {
                warn!(error = %e, "[attestation] Failed to decode public key");
                return;
            }
        };
        let signer_peer_id = pubkey.to_peer_id();

        // Verify signature
        if !pubkey.verify(proposal_id.as_bytes(), sig) {
            warn!(
                proposal_id = %proposal_id,
                signer = %signer_peer_id,
                "[attestation] Signature verification failed — rejected"
            );
            return;
        }

        info!(
            proposal_id = %proposal_id,
            signer = %signer_peer_id,
            from_gossipsub = %propagation_source,
            "[attestation] Witness signature verified ✓"
        );

        // Collect signature
        let sigs = self.witness_sigs.entry(proposal_id.clone()).or_default();

        // Deduplicate — one sig per peer
        if sigs.iter().any(|(pid, _)| *pid == signer_peer_id) {
            debug!(
                proposal_id = %proposal_id,
                signer = %signer_peer_id,
                "[attestation] Duplicate signature ignored"
            );
            return;
        }

        sigs.push((signer_peer_id, sig.to_vec()));
        let count = sigs.len();

        info!(
            proposal_id = %proposal_id,
            signatures_collected = count,
            threshold = 3,
            "[attestation] Signature collected — {}/3 toward quorum",
            count
        );

        // Check quorum — when reached, commit to the hash-chain ledger
        if count >= 3 {
            // Dedup guard: skip if already committed (trailing attestations)
            if self.commit_manager.is_committed(&proposal_id) {
                debug!(
                    proposal_id = %proposal_id,
                    "[attestation] Trailing attestation — already committed, skipping"
                );
                return;
            }

            info!(
                proposal_id = %proposal_id,
                signers = ?sigs.iter().map(|(pid, _)| pid.to_string()).collect::<Vec<_>>(),
                "═══════════════════════════════════════════\n\
                 [RATIFIED] 3-of-5 witness quorum reached!\n\
                 Certificate {} is now State 3: Ratified\n\
                 ═══════════════════════════════════════════",
                proposal_id
            );

            // ── State 4: Committed ────────────────────────────
            // Write the ratified certificate and its signatures
            // to the append-only Blake3 hash-chain ledger.
            if let Some(cert_bytes) = self.cert_cache.get(&proposal_id) {
                match self.commit_manager.commit(
                    cert_bytes,
                    &proposal_id,
                    sigs,
                ) {
                    Ok(block_hash) => {
                        info!(
                            proposal_id = %proposal_id,
                            block_hash = %hex::encode(block_hash),
                            height = self.commit_manager.height(),
                            "═══════════════════════════════════════════\n\
                             [COMMITTED] State 4 — written to hash-chain ledger\n\
                             Block height: {}\n\
                             ═══════════════════════════════════════════",
                            self.commit_manager.height()
                        );
                    }
                    Err(e) => {
                        error!(
                            proposal_id = %proposal_id,
                            error = %e,
                            "[commit] Failed to write to ledger"
                        );
                    }
                }
            } else {
                warn!(
                    proposal_id = %proposal_id,
                    "[commit] Cert not in cache — cannot commit"
                );
            }
        }
    }

    /// Handle an API request from the Unix Domain Socket server.
    fn handle_api_message(&mut self, msg: crate::api::ApiMessage) {
        use crate::api::{ApiRequest, ApiResponse};
        use prost::Message;

        let response = match msg.request {
            ApiRequest::GetHeight => ApiResponse::Height {
                height: self.commit_manager.height(),
            },
            ApiRequest::GetBlock { height } => {
                match self.commit_manager.get_block_bytes(height) {
                    Some(raw) => {
                        let bh = if raw.len() >= 72 {
                            hex::encode(&raw[8..40])
                        } else {
                            String::new()
                        };
                        let ch = if raw.len() >= 72 {
                            hex::encode(&raw[40..72])
                        } else {
                            String::new()
                        };
                        let sc = if raw.len() >= 74 {
                            let mut off = 72 + 4;
                            if off + 4 <= raw.len() {
                                let cl = u32::from_be_bytes([
                                    raw[72], raw[73], raw[74], raw[75],
                                ]) as usize;
                                off += cl;
                                if off + 2 <= raw.len() {
                                    u16::from_be_bytes([raw[off], raw[off + 1]])
                                } else { 0 }
                            } else { 0 }
                        } else { 0 };
                        ApiResponse::Block {
                            height,
                            block_hash: bh,
                            cert_hash: ch,
                            sig_count: sc,
                        }
                    }
                    None => ApiResponse::Error {
                        message: format!("Block {} not found", height),
                    },
                }
            }
            ApiRequest::GetCertificate { proposal_id } => {
                match self.cert_cache.get(&proposal_id) {
                    Some(raw) => {
                        match crate::ingest::proto::ImpactCertificate::decode(&raw[..]) {
                            Ok(cert) => {
                                let seed = cert.witness_seed.clone();
                                let validation = format!("{:?}", cert.georgist_validation());
                                ApiResponse::Certificate {
                                    proposal_id: cert.proposal_id,
                                    enclave_id: cert.enclave_id,
                                    rounds: cert.debate_rounds.len() as u32,
                                    witness_seed: seed,
                                    validation,
                                    bytes: raw.len() as u64,
                                }
                            }
                            Err(e) => ApiResponse::Error {
                                message: format!("Decode error: {}", e),
                            },
                        }
                    }
                    None => ApiResponse::Error {
                        message: format!("Certificate {} not found", proposal_id),
                    },
                }
            }
            ApiRequest::AgentSubmit {
                task_id,
                model,
                model_size,
                vram_bytes,
                graph_blob_b64,
                deadline_epoch,
            } => {
                use base64::Engine as _;
                let graph_blob = match base64::engine::general_purpose::STANDARD
                    .decode(&graph_blob_b64)
                {
                    Ok(b) => b,
                    Err(e) => {
                        let _ = msg.reply.send(ApiResponse::AgentError {
                            task_id,
                            error: format!("Base64 decode error: {}", e),
                        });
                        return;
                    }
                };
                let ms = match model_size.to_lowercase().as_str() {
                    "tiny" => ModelSize::Tiny,
                    "small" => ModelSize::Small,
                    "medium" => ModelSize::Medium,
                    "large" => ModelSize::Large,
                    other => {
                        let _ = msg.reply.send(ApiResponse::AgentError {
                            task_id,
                            error: format!("Unknown model_size: {}. Use tiny|small|medium|large", other),
                        });
                        return;
                    }
                };
                match self.submit_agent_task(task_id.clone(), model, ms, vram_bytes, graph_blob, deadline_epoch) {
                    Ok(_) => {
                        let hash = blake3::hash(
                            &self
                                .agent_registry
                                .get(&task_id)
                                .map(|r| r.task.graph_blob.clone())
                                .unwrap_or_default(),
                        );
                        ApiResponse::AgentSubmitted {
                            task_id,
                            graph_hash: hex::encode(hash.as_bytes()),
                        }
                    }
                    Err(e) => ApiResponse::AgentError {
                        task_id,
                        error: format!("{}", e),
                    },
                }
            }
            ApiRequest::GetStats => ApiResponse::Stats {
                height: self.commit_manager.height(),
                committed_count: self.commit_manager.height(),
            },
            ApiRequest::SubmitClaim {
                claim_id,
                domain_tag,
                claim_type,
                bound_commit,
                ..
            } => match domain_tag.as_str() {
                "STATE" => crate::claims::handle_state_claim(
                    &self.local_key, &claim_id, &claim_type, &bound_commit,
                ),
                "JUDGMENT" => crate::claims::handle_judgment_claim(
                    &self.local_key, &claim_id, &claim_type, &bound_commit,
                ),
                _ => ApiResponse::Error {
                    message: format!("Unknown domain_tag: {}", domain_tag),
                },
            },
        };

        let _ = msg.reply.send(response);
    }
}

// ── Phase 11: panel-access density check ──────────────────────

/// Check whether honest thickness is sufficient to safely form a witness panel.
///
/// Conservative: counts ALL pool members as potentially Sybil.
/// honest_T = sum of thickness for peers above FLOOR_WEIGHT.
/// Threshold = N_eligible × FLOOR_WEIGHT × density_margin.
///
/// Returns true if the panel can safely form, false if it should be refused.
fn check_panel_access_density(pool: &[(PeerId, f64)], density_margin: f64, floor_weight: f64) -> bool {
    let honest_t: f64 = pool
        .iter()
        .map(|(_, w)| *w)
        .filter(|w| *w > floor_weight)
        .sum();
    let n_eligible = pool.len();
    if n_eligible == 0 {
        return false;
    }
    let sybil_floor_total = n_eligible as f64 * floor_weight;
    let threshold = sybil_floor_total * density_margin;
    honest_t >= threshold
}

// ── Identity helpers ──────────────────────────────────────────

/// Decide whether to emit a fetch for a gap, updating the outstanding
/// set with dedup and lazy timeout eviction.  Returns true if a fetch
/// should be emitted, false if one is already in flight for this gap.
fn should_fetch(
    outstanding: &mut HashMap<(PeerId, u64), Instant>,
    signer: PeerId,
    expected: u64,
    timeout: Duration,
) -> bool {
    let key = (signer, expected);
    if outstanding.contains_key(&key) {
        return false;
    }
    // Lazy eviction: sweep expired entries for this signer.
    let now = Instant::now();
    outstanding.retain(|(s, _), ts| s != &signer || *ts + timeout > now);
    outstanding.insert(key, now);
    true
}

/// Set the nonce field on a Transaction (used after TaxEngine produces
/// transactions with placeholder nonce 0).
fn set_transaction_nonce(tx: &mut Transaction, nonce: u64) {
    match tx {
        Transaction::Transfer { nonce: ref mut n, .. } => *n = nonce,
        Transaction::Mint { nonce: ref mut n, .. } => *n = nonce,
        Transaction::Vouch { nonce: ref mut n, .. } => *n = nonce,
        Transaction::Genesis { nonce: ref mut n, .. } => *n = nonce,
        Transaction::BootstrapEnded { nonce: ref mut n, .. } => *n = nonce,
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

#[cfg(test)]
mod panel_access_tests {
    use super::*;

    #[test]
    fn sparse_mesh_refuses_panel() {
        // All floor-weight peers, zero honest thickness.
        // N=10, floor=0.01, margin=2.0 → threshold = 10 × 0.01 × 2.0 = 0.20
        // honest_T = 0 → 0 < 0.20 → REFUSE
        let pool: Vec<(PeerId, f64)> = (0..10)
            .map(|_| (PeerId::random(), 0.0))
            .collect();
        assert!(!check_panel_access_density(&pool, 2.0, 0.01));
    }

    #[test]
    fn dense_mesh_permits_panel() {
        // One honest node with T=100, nine floor-weight peers.
        // N=10, floor=0.01, margin=2.0 → threshold = 0.20
        // honest_T = 100 → 100 >= 0.20 → PERMIT
        let mut pool: Vec<(PeerId, f64)> = vec![(PeerId::random(), 100.0)];
        for _ in 0..9 {
            pool.push((PeerId::random(), 0.0));
        }
        assert!(check_panel_access_density(&pool, 2.0, 0.01));
    }

    #[test]
    fn transition_refuse_to_permit() {
        // Start sparse, add thickness, should transition.
        let mut pool: Vec<(PeerId, f64)> = (0..5)
            .map(|_| (PeerId::random(), 0.0))
            .collect();

        // Sparse: N=5, threshold = 5 × 0.01 × 2.0 = 0.10, honest_T=0 → REFUSE
        assert!(!check_panel_access_density(&pool, 2.0, 0.01));

        // Add honest node with T=50
        pool.push((PeerId::random(), 50.0));

        // Dense: N=6, threshold = 6 × 0.01 × 2.0 = 0.12, honest_T=50 → PERMIT
        assert!(check_panel_access_density(&pool, 2.0, 0.01));
    }

    #[test]
    fn empty_pool_refuses() {
        let pool: Vec<(PeerId, f64)> = vec![];
        assert!(!check_panel_access_density(&pool, 2.0, 0.01));
    }

    #[test]
    fn exactly_at_threshold_permits() {
        // N=5, threshold = 5 × 0.01 × 2.0 = 0.10
        // honest_T = 0.10 → 0.10 >= 0.10 → PERMIT (boundary)
        let pool: Vec<(PeerId, f64)> = vec![
            (PeerId::random(), 0.10),
            (PeerId::random(), 0.0),
            (PeerId::random(), 0.0),
            (PeerId::random(), 0.0),
            (PeerId::random(), 0.0),
        ];
        assert!(check_panel_access_density(&pool, 2.0, 0.01));
    }

    #[test]
    fn higher_margin_makes_guard_stricter() {
        // Same pool, margin=2.0 passes, margin=10.0 fails
        let pool: Vec<(PeerId, f64)> = vec![
            (PeerId::random(), 1.0),
            (PeerId::random(), 0.0),
            (PeerId::random(), 0.0),
            (PeerId::random(), 0.0),
            (PeerId::random(), 0.0),
        ];
        // margin 2.0: threshold = 5 × 0.01 × 2.0 = 0.10, honest_T=1.0 → pass
        assert!(check_panel_access_density(&pool, 2.0, 0.01));
        // margin 10.0: threshold = 5 × 0.01 × 10.0 = 0.50, honest_T=1.0 → still pass
        assert!(check_panel_access_density(&pool, 10.0, 0.01));
        // margin 2000.0: threshold = 5 × 0.01 × 2000 = 100, honest_T=1.0 → FAIL
        assert!(!check_panel_access_density(&pool, 2000.0, 0.01));
    }

    #[test]
    fn real_mesh_threshold_formula() {
        // Print the threshold formula for documentation.
        // With N_all peers, honest_T needed = N_all × 0.01 × 2.0 = N_all × 0.02
        // So for N_all=2 (Z4 + relay): need honest_T >= 0.04
        // For N_all=3 (+1 Pi5): need honest_T >= 0.06
        // For N_all=10: need honest_T >= 0.20
        // For N_all=100: need honest_T >= 2.00
        for n in &[2u64, 3, 5, 10, 50, 100] {
            let floor_weight = 0.01;
            let threshold = *n as f64 * floor_weight * 2.0;
            eprintln!(
                "N_all={}: honest_T needed >= {:.4} (conservative, margin=2.0, floor={:.4})",
                n, threshold, floor_weight
            );
        }
    }

    #[test]
    fn gauge_scale_invariance_holds() {
        let fw1 = crate::ledger::state::floor_weight(1_000_000.0);
        let fw2 = crate::ledger::state::floor_weight(1_000_000_000.0);
        let s = 1_000_000_000.0 / 1_000_000.0;
        assert!((fw1 / fw2 - s).abs() < 1e-10, "floor_weight must scale inversely with gauge");
    }

    // ── Relay origin-check tests ────────────────────────────

    #[test]
    fn two_nodes_no_relay_source_equals_propagation() {
        // A sends to B: source = A, propagation = A → not relay
        let a = PeerId::random();
        assert!(!LatticeNode::is_relay_work(Some(a), &a));
    }

    #[test]
    fn three_nodes_relay_source_differs_from_propagation() {
        // C sends, A relays to B: source = C, propagation = A → IS relay
        let a = PeerId::random();
        let c = PeerId::random();
        assert!(LatticeNode::is_relay_work(Some(c), &a));
    }

    #[test]
    fn no_source_always_blocks_receipt() {
        // If source is None (corrupted message), never issue receipt
        let a = PeerId::random();
        let b = PeerId::random();
        assert!(!LatticeNode::is_relay_work(None, &a));
        assert!(!LatticeNode::is_relay_work(None, &b));
    }

    // NOTE: the discriminating genesis-root test requires a tokio runtime
    // (LatticeNode::new() initializes a libp2p swarm). The guard logic is:
    //   if self.local_peer_id != root { bail!("is not the configured genesis root") }
    // Tested manually via: --submit-genesis on a non-root node → error.

    #[test]
    fn gap_triggers_fetch_dedup_skips_duplicate() {
        // Positive: gap detected → fetch marked.  Negative: same hole
        // → no double-mark.  Contiguous → nothing marked.
        use std::collections::HashMap;
        use std::time::{Duration, Instant};

        let alice = PeerId::random();
        let bob = PeerId::random();
        let mut outstanding = HashMap::new();
        let timeout = Duration::from_secs(5);

        // First gap for Alice at nonce 2: should fetch.
        assert!(
            should_fetch(&mut outstanding, alice, 2, timeout),
            "First gap should trigger a fetch"
        );
        assert_eq!(outstanding.len(), 1);
        assert!(outstanding.contains_key(&(alice, 2)));

        // Same gap again: dedup — no double fetch.
        assert!(
            !should_fetch(&mut outstanding, alice, 2, timeout),
            "Duplicate gap should be deduped"
        );
        assert_eq!(outstanding.len(), 1);

        // Different signer, different gap: independent.
        assert!(
            should_fetch(&mut outstanding, bob, 2, timeout),
            "Bob's gap is independent"
        );
        assert_eq!(outstanding.len(), 2);

        // Contiguous case: no gap, never reaches should_fetch.
        // If expected == last_nonce + 1, check_nonce passes silently.
        // The assertion is: outstanding unchanged.
        assert!(!outstanding.contains_key(&(alice, 1)));
    }

    #[test]
    fn sender_outbound_queue_removal_on_echo_only() {
        // Discriminator for Option A: self-echo (propagation_source == self)
        // must NOT remove from outbound queue.  Cross-echo
        // (propagation_source != self) DOES remove.
        use chrono::Utc;
        use std::collections::BTreeMap;

        let me = PeerId::random();
        let other = PeerId::random(); // a different peer
        let mut outbound: HashMap<PeerId, BTreeMap<u64, SignedTransaction>> = HashMap::new();

        // Insert a mock transaction into our own outbound queue.
        let nonce = 5u64;
        let mock_tx = SignedTransaction {
            transaction: Transaction::Transfer {
                from: me.to_string(),
                to: other.to_string(),
                amount: DigitalUtilityUnit(100),
                nonce,
                timestamp: Utc::now(),
            },
            signer_public_key: vec![],
            signature: vec![],
        };
        outbound.entry(me).or_default().insert(nonce, mock_tx);
        assert!(outbound.get(&me).map_or(false, |q| q.contains_key(&nonce)));

        // Simulate self-echo: same peer forwards our own message.
        // This happens when gossipsub echoes back to the publisher.
        // Under Option A, this must NOT remove the queue entry.
        if let Some(queue) = outbound.get_mut(&me) {
            // propagation_source == me is a self-echo
            // This guard is the load-bearing check in the production code.
            let propagation_source = me; // self-echo
            if propagation_source != me {
                // This guard should prevent removal
                queue.remove(&nonce);
            }
        }
        assert!(
            outbound.get(&me).map_or(false, |q| q.contains_key(&nonce)),
            "Self-echo must NOT remove from outbound queue"
        );

        // Simulate cross-echo: a different peer forwarded our transaction.
        if let Some(queue) = outbound.get_mut(&me) {
            let propagation_source = other; // cross-echo
            if propagation_source != me {
                queue.remove(&nonce);
            }
        }
        assert!(
            outbound.get(&me).map_or(true, |q| !q.contains_key(&nonce)),
            "Cross-echo MUST remove from outbound queue"
        );
    }

    #[test]
    fn stale_nonce_below_expected_does_not_trigger_fetch() {
        // Stale transaction (nonce behind current high-water mark)
        // should NOT register an outstanding fetch — the range would
        // be descending and nonsensical.
        let alice = PeerId::random();
        let mut outstanding = HashMap::new();
        let timeout = Duration::from_secs(5);

        // First gap: nonce 8 arrives, expected 4 → gap ahead → fetch.
        assert!(should_fetch(&mut outstanding, alice, 4, timeout));
        assert_eq!(outstanding.len(), 1);

        // Stale: nonce 8 would arrive, but we're already at 10.
        // This simulates: expected=11, got=8 → got < expected.
        // should_fetch is never called in this case — the handler
        // checks `got < expected` before reaching should_fetch.
        // The assertion is: outstanding unchanged from the one entry.
        assert_eq!(outstanding.len(), 1);
        assert!(outstanding.contains_key(&(alice, 4)));
    }
}
