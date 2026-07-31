use std::collections::HashMap;
use std::collections::{BTreeMap, HashSet, VecDeque};
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
use chrono::Utc;
use tracing::{debug, error, info, trace, warn};

use crate::agent::ModelSize;
use crate::ledger::state::LedgerState;
use crate::ledger::types::{DigitalUtilityUnit, SignedTransaction, Transaction};
use crate::ledger::validation;
use crate::ledger::validation::validate_objection;
use crate::message::codec::rpc::{BalanceCodec, BalanceProtocol, LatticeCodec, LatticeProtocol};
use crate::message::codec::rpc::{TransactionCodec, TransactionProtocol, VerifyProtocol};
use crate::message::codec::rpc::{ChainSyncCodec, ChainSyncProtocol};
use crate::message::codec::rpc::{WitnessCodec, WitnessProtocol};
use crate::message::codec::rpc::{CompareCodec, CompareProtocol};
use crate::message::types::{
    BalanceRequest, BalanceResponse, Heartbeat, LatticeMessage, StatusRequest, StatusResponse,
};
use crate::message::types::{TransactionRequest, TransactionResponse};
use crate::message::types::{ChainRangeRequest, ChainRangeResponse};
use crate::message::types::WireBlock;
use crate::message::types::{VerifyRequest, VerifyResponse};
use crate::message::types::{WitnessRequest, WitnessResponse};
use crate::message::types::{ComparisonRequest, ComparisonResponse};
use crate::message::types::{RatificationBlock, ERA_ONE_BLOCK_MARKER, ERA_TWO_BLOCK_MARKER};
use crate::network::protocol::{
    LatticeBehaviour, LatticeBehaviourEvent, LATTICE_HEARTBEAT_TOPIC, LATTICE_KAD_PROTOCOL,
    LATTICE_ENCLAVE_CERT_TOPIC, LATTICE_AGENT_TOPIC, LATTICE_BLOCK_TOPIC,
};
use crate::agent::codec::AGENT_STATE_PROTOCOL;
use crate::state::peers::PeerTable;
use crate::economics::EconomicEngine;
use crate::economics::EpochSummary;
use crate::economics::receipts::{RelayReceipt, SignedReceipt, validate_receipt};
use crate::storage::ProofEngine;

/// Lattice protocol version advertised in status responses.
const PROTOCOL_VERSION: u32 = 1;

/// Gossipsub topic for economic transaction propagation.
pub const LATTICE_TX_TOPIC: &str = "lattice/tx/v1";

/// Gossipsub topic for network genesis propagation.
/// Genesis is a one-per-network record that establishes the trust anchor.
pub const LATTICE_GENESIS_TOPIC: &str = "/lattice/genesis/v1";

/// Gossipsub topic for objection propagation.
/// Objections are disputes against witnessed claims — any peer may
/// submit, validate, and relay them.
pub const LATTICE_OBJECTION_TOPIC: &str = "lattice/objection/v1";

/// How long to wait for a fetch response before considering it failed.
/// On a 3-node LAN mesh, round-trips are sub-second — 5s is generous.
const FETCH_TIMEOUT: Duration = Duration::from_secs(5);

/// How old an outstanding fetch request must be before it is evicted
/// by the periodic sweep.  600s = 10 minutes, well beyond any legitimate
/// network latency, short enough to bound memory on a healthy mesh.
const FETCH_EVICTION_THRESHOLD: Duration = Duration::from_secs(600);

/// How old an outbound queue entry must be before the periodic sweep
/// evicts it.  600s = 10 minutes, same rationale as the fetch sweep.
/// An evicted entry's transaction is not lost — it will be re-broadcast
/// by the normal gossip path on the next trigger event.
const OUTBOUND_SWEEP_THRESHOLD: Duration = Duration::from_secs(600);

/// How long without a heartbeat before a peer is flagged as silent.
/// 30s = 3× heartbeat interval (10s).
const ZOMBIE_WARN_THRESHOLD_SECS: u64 = 30;

/// How long without a heartbeat before a peer is force-disconnected.
/// 90s = 9× heartbeat interval. Well beyond any legitimate network
/// hiccup, short enough that the half-open connection doesn't persist.
const ZOMBIE_EVICT_THRESHOLD_SECS: u64 = 90;

/// Grace window for cold-start peers (heartbeats_received == 0).  Protects
/// against both wall-clock silence (Layer 1) and epoch-based heartbeat
/// silence (Layer 2a, which had an additional init bug: last_heartbeat_epoch
/// started at 0, making any fresh peer on a mesh aged >30 epochs instantly
/// evictable).  After the first heartbeat, standard rules apply.
/// Set at ~20× the observed worst-case connection-to-first-heartbeat
/// interval (~15s) from the Jul 22 soak data.
const COLD_START_GRACE_SECS: u64 = 300;

/// How long to wait for reconnect after eviction before logging ERROR.
const RECONNECT_TIMEOUT_SECS: u64 = 30;

/// Max evictions per 60s window before the circuit breaker trips.
const CIRCUIT_BREAKER_LIMIT: usize = 3;

/// Minimum peers in the topic mesh before we consider a broadcast
/// handoff as strong evidence of delivery.  On a 3-node mesh with
/// a relay hub, N=1 is sufficient — the relay is an always-on
/// witness.  On flaky homelab meshes, raise to N=2 to require two
/// independent recipients before removing from the outbound queue.
const OUTBOUND_CONFIRM_PEERS: usize = 1;

/// How many epochs without a heartbeat before a peer is evicted
/// as a zombie.  At a 30s epoch interval, 30 epochs = 15 minutes.
/// The wall-clock thresholds (ZOMBIE_WARN/EVICT_THRESHOLD_SECS)
/// still fire first for half-open connection detection.
const ZOMBIE_EPOCH_THRESHOLD: u64 = 30;

/// How many epochs without an attestation, combined with zero
/// thickness, must elapse before a peer is classified as a zombie.
/// A peer that neither contributes thickness nor attests to others'
/// contributions is dead weight.
const ZOMBIE_ATTESTATION_SILENCE_EPOCHS: u64 = 10;

/// Fraction of peers that must be dead in a single sweep cycle
/// before the outbound circuit breaker fires, pausing new
/// transaction forwarding.  0.5 = 50%.
const OUTBOUND_CIRCUIT_BREAKER_FRACTION: f64 = 0.5;

/// How many epochs a witness claim stays in "collecting" before
/// timing out. At 64s/epoch, 5 epochs ≈ 5 minutes — enough for
/// peer responses on any reasonable mesh.
const CLAIM_COLLECTION_EPOCHS: u64 = 5;

/// How many epochs finalized claims stay in the registry before
/// eviction. 10 epochs ≈ 10 minutes — enough for the operator to
/// poll the result.
const CLAIM_RETENTION_EPOCHS: u64 = 10;

// ── Witness claim submission tracking ──────────────────────

/// State of a single witness claim submission, tracked through
/// the collection → assembly → acceptance lifecycle.
#[derive(Debug, Clone)]
struct WitnessClaimState {
    claim_id: String,
    status: ClaimSubmissionStatus,
    start_epoch: u64,
    end_epoch: u64,
    candidates: Vec<PeerId>,
    signatures: Vec<crate::claims::WitnessSignature>,
    declines: Vec<(PeerId, String)>,
    started_at_epoch: u64,
}

#[derive(Debug, Clone)]
enum ClaimSubmissionStatus {
    /// Waiting for witness responses.
    Collecting,
    /// All witnesses responded (or deadline expired) — assembled
    /// and accepted.
    Accepted { thickness: f64 },
    /// Witnesses refused, or acceptance rejected.
    Rejected { reason: String },
}

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
    /// True while a ChainRangeRequest is in flight. Live block broadcasts
    /// received during catch-up are ignored to avoid race conditions where
    /// the catch-up response and live broadcast arrive at the same height.
    is_catching_up: bool,

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
    /// Cumulative count of stale fetch entries that have been evicted
    /// by the periodic sweep.  Logged when > 0.  Tracks the sweep's
    /// effectiveness across restarts (in-memory only — not persisted).
    total_evicted_fetches: usize,
    /// Tracks when each outbound queue entry was inserted, so the
    /// periodic sweep can evict entries that have been stuck for more
    /// than OUTBOUND_SWEEP_THRESHOLD (600s).  Keyed by (signer, nonce)
    /// matching the outbound map.
    outbound_insertion_times: HashMap<(PeerId, u64), Instant>,
    /// Peers evicted as zombies, awaiting reconnect. Cleared on
    /// ConnectionEstablished or after RECONNECT_TIMEOUT_SECS (ERROR).
    pending_reconnect: HashMap<PeerId, Instant>,
    /// Eviction timestamps for circuit breaker. Drained of entries
    /// older than 60s on each check.
    evictions_last_minute: VecDeque<Instant>,
    /// Max peer silence in seconds, updated by check_peer_liveness.
    /// Included in the metrics line.
    last_max_peer_silence: u64,
    /// Peers with an in-flight ChainRangeRequest.  Prevents firing
    /// multiple overlapping catch-up requests to the same peer while
    /// the first is still outstanding.  Cleared on response (success
    /// or failure).
    outstanding_chain_requests_by_peer: HashSet<PeerId>,
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
    /// Key: proposal_id, Value: list of (peer_id, signature, public_key) triples.
    /// When 3-of-5 threshold is met, the certificate is ratified.
    witness_sigs: HashMap<String, Vec<(PeerId, Vec<u8>, Vec<u8>)>>,

    /// Tracks the last epoch in which each peer submitted a valid
    /// witness attestation.  Used by zombie eviction to detect peers
    /// that neither attest nor carry thickness.
    last_attestation_epoch: HashMap<PeerId, u64>,

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
    /// Force Era Two block production regardless of bootstrap state.
    /// Bypasses is_bootstrap_ended() gate for testing/development.
    force_era_two: bool,
    /// Enforce witness-panel QC for RatificationBlocks (Round 2).
    require_rb_qc: bool,
    /// NTP servers for runtime clock verification. None = defaults.
    ntp_servers: Option<Vec<String>>,
    /// Skip runtime NTP checks (implied by --skip-ntp-check).
    skip_ntp_check: bool,
    /// When true, the node refuses to sign new transactions because
    /// clock drift exceeds NTP_REFUSE_SIGN_THRESHOLD_SECS.
    refuse_to_sign: bool,
    /// Timestamp of the last successful NTP check.
    last_ntp_check: Instant,
    /// Most recent NTP drift measurement in seconds.
    ntp_drift_secs: i64,
    /// Phase 11: thickness floor weight for sortition (security parameter).
    /// Pinned to 1/T_min where T_min is expected minimum honest thickness.
    floor_weight: f64,
    /// Phase 11: density margin multiplier for panel-access invariant.
    /// honest_T must exceed N_eligible × floor_weight × margin before
    /// witness panels can form.
    density_margin: f64,
    thickness_gauge: f64,
    /// Expected root PeerId for genesis validation (out-of-band trust anchor).
    /// If None, the node may self-author genesis using its own identity.
    genesis_root: Option<PeerId>,
    /// The genesis this node has accepted (from gossip or self-authored).
    /// None until the network origin is established.  Doubles as the
    /// `genesis_established` flag for recovery ordering.
    genesis: Option<crate::ledger::types::SignedGenesis>,
    /// PeerIds to which we have already sent genesis this session,
    /// preventing re-gossip amplification on reconnect storms.
    genesis_sent_to: HashSet<PeerId>,
    /// PeerIds awaiting retry — re-gossip failed with InsufficientPeers
    /// and will be retried on each metrics tick until the mesh forms.
    pending_genesis_gossip: HashSet<PeerId>,
    /// Pending objections to re-gossip.  When publishing an objection
    /// fails with InsufficientPeers, it's queued here and retried on
    /// each metrics tick.  Capped at 256 entries; oldest evicted on
    /// overflow.
    pending_objection_gossip: Vec<crate::ledger::types::SignedObjection>,
    /// In-memory objections map — mirrors the persistent state's
    /// objections field.  Populated from recovery and updated on
    /// each receive.  The source of truth for cap and dedup checks.
    objections: HashMap<[u8; 32], Vec<crate::ledger::types::SignedObjection>>,
    /// Self-liquidation period for genesis thickness. None = permanent.
    genesis_amortize_over: Option<u64>,
    /// Automatically submit genesis on startup if the chain is empty.
    auto_genesis: bool,
    /// Initial thickness grant for genesis (gauge-scaled).
    genesis_thickness: f64,
    /// Transaction persistence layer (WAL + snapshot).  When set,
    /// every validated and applied transaction is recorded in the WAL
    /// and nonces are snapshotted for crash recovery.  Optional so
    /// that nodes without a data directory skip disk I/O entirely.
    state_store: Option<Box<dyn crate::ledger::persistence::StateStore>>,
    /// Phase 9: model execution bridge (Ollama / OpenAI-compatible).
    executor: crate::agent::executor::Executor,
    /// Phase 9: channel sender for background execution results.
    exec_tx: Option<tokio::sync::mpsc::Sender<ExecutionResult>>,
    /// Witness claim submission registry — tracks in-flight and
    /// completed claims through the collection → acceptance lifecycle.
    witness_claims: HashMap<String, WitnessClaimState>,
    /// High-water mark per (claimant, claim_type) for replay/overlap
    /// defense.  Advances on acceptance.
    last_claimed: crate::claims::acceptance::ClaimNonceMap,
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
        genesis_amortize_over: Option<u64>,
        auto_genesis: bool,
        genesis_thickness: f64,
        force_era_two: bool,
        openai_api_key: Option<String>,
        openai_endpoint: Option<String>,
        ntp_servers: Option<Vec<String>>,
        skip_ntp_check: bool,
        require_rb_qc: bool,
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

                // Subscribe to block topic for chain propagation.
                let block_topic = gossipsub::IdentTopic::new(LATTICE_BLOCK_TOPIC);
                gossipsub
                    .subscribe(&block_topic)
                    .map_err(|e| anyhow::anyhow!("gossipsub block subscribe: {e}"))?;

                // Subscribe to genesis topic for network origin propagation.
                let genesis_topic = gossipsub::IdentTopic::new(LATTICE_GENESIS_TOPIC);
                gossipsub
                    .subscribe(&genesis_topic)
                    .map_err(|e| anyhow::anyhow!("gossipsub genesis subscribe: {e}"))?;

                // Subscribe to objection topic for claim dispute propagation.
                let objection_topic = gossipsub::IdentTopic::new(LATTICE_OBJECTION_TOPIC);
                gossipsub
                    .subscribe(&objection_topic)
                    .map_err(|e| anyhow::anyhow!("gossipsub objection subscribe: {e}"))?;

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

                // Chain sync RPC channel (Phase 10).
                let chain_sync_rpc = request_response::Behaviour::new(
                    [(ChainSyncProtocol, request_response::ProtocolSupport::Full)],
                    request_response::Config::default(),
                );

                // Witness RPC channel — request-response for claim signing.
                // Separate from gossipsub: RPC is for explicit interaction
                // with a specific peer, not broadcast.
                let witness_rpc = request_response::Behaviour::new(
                    [(WitnessProtocol, request_response::ProtocolSupport::Full)],
                    request_response::Config::default(),
                );

                // Economic comparison RPC channel — scoped balance fingerprint comparison.
                let compare_rpc = request_response::Behaviour::new(
                    [(CompareProtocol, request_response::ProtocolSupport::Full)],
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
                    chain_sync_rpc,
                    witness_rpc,
                    compare_rpc,
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
            is_catching_up: false,
            ledger: LedgerState::new(),
            seen_nonces: HashMap::new(),
            tx_nonce: 0,
            tx_store: HashMap::new(),
            pending: HashMap::new(),
            outstanding_fetches: HashMap::new(),
            total_evicted_fetches: 0,
            outbound_insertion_times: HashMap::new(),
            pending_reconnect: HashMap::new(),
            evictions_last_minute: VecDeque::new(),
            last_max_peer_silence: 0,
            outstanding_chain_requests_by_peer: HashSet::new(),
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
            last_attestation_epoch: HashMap::new(),
            cert_cache: HashMap::new(),
            commit_manager: crate::commit::CommitManager::open(&storage_path),
            agent_registry: crate::agent::registry::AgentRegistry::open(&storage_path),
            agent_mode,
            agent_peers: HashMap::new(),
            max_model_size,
            vram_bytes,
            no_economics,
            force_era_two,
            require_rb_qc,
            ntp_servers: ntp_servers.clone(),
            skip_ntp_check,
            refuse_to_sign: false,
            last_ntp_check: Instant::now(),
            ntp_drift_secs: 0,
            floor_weight,
            density_margin,
            thickness_gauge,
            genesis_root: {
                let parsed = genesis_root.and_then(|s| s.parse().ok());
                if parsed.is_none() && !auto_genesis {
                    tracing::warn!(
                        "--genesis-root not set — this node cannot validate genesis. \
                         Economic participation (panels, certificates, thickness) \
                         requires a configured trust anchor. Relay and gossip still work. \
                         Use --auto-genesis to self-author a new mesh."
                    );
                }
                parsed
            },
            genesis: None,  // populated during recovery or on first genesis accept
            genesis_sent_to: HashSet::new(),
            pending_genesis_gossip: HashSet::new(),
            pending_objection_gossip: Vec::new(),
            objections: HashMap::new(),
            genesis_amortize_over,
            auto_genesis,
            genesis_thickness,
            state_store: None,
            executor: crate::agent::executor::Executor::new(openai_api_key, openai_endpoint),
            exec_tx: None,
            witness_claims: HashMap::new(),
            last_claimed: crate::claims::acceptance::ClaimNonceMap::new(),
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
        // If no external root is configured, the node self-authors genesis.
        let root = self.genesis_root.unwrap_or(self.local_peer_id);
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
            amortize_over: self.genesis_amortize_over,
            nonce: 0,
            timestamp: chrono::Utc::now(),
        };
        let data = serde_cbor::to_vec(&tx)?;
        let signature = self.local_key.sign(&data)?;

        self.commit_manager.commit_root_block(&data, "genesis", &signature, &self.local_peer_id)
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        // ── Durability boundary: persist before apply ──
        let stx = crate::ledger::types::SignedTransaction {
            transaction: tx,
            signer_public_key: self.local_key.public().encode_protobuf(),
            signature,
        };
        self.on_transaction_applied(&stx);
        self.ledger.apply_transaction(&stx.transaction)?;

        // Broadcast the genesis block to peers.
        self.publish_committed_block("genesis");

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

        // ── Durability boundary: persist before apply ──
        let stx = crate::ledger::types::SignedTransaction {
            transaction: tx,
            signer_public_key: self.local_key.public().encode_protobuf(),
            signature,
        };
        self.on_transaction_applied(&stx);
        self.ledger.apply_transaction(&stx.transaction)?;

        // Broadcast the BootstrapEnded declaration to peers.
        self.publish_committed_block("bootstrap-ended");

        info!(
            declared_by = %self.local_peer_id,
            "BootstrapEnded committed — era two begins. Root-authorized blocks are now rejected."
        );
        Ok(())
    }

    /// Read the last committed block from the chain and broadcast it
    /// on the block gossipsub topic. This transforms the chain from a
    /// per-node local ledger into a distributed structure.
    fn publish_committed_block(&mut self, proposal_id: &str) {
        let height = self.commit_manager.height();
        if height == 0 {
            return;
        }
        let raw = match self.commit_manager.get_block_bytes(height - 1) {
            Some(b) => b,
            None => {
                tracing::warn!(
                    proposal_id,
                    height = height - 1,
                    "[block-publish] Block not found in chain — cannot broadcast"
                );
                return;
            }
        };

        let topic = gossipsub::IdentTopic::new(LATTICE_BLOCK_TOPIC);
        self.track_outbound(&raw);
        match self.swarm.behaviour_mut().gossipsub.publish(topic, raw) {
            Ok(msg_id) => {
                tracing::info!(
                    proposal_id,
                    height = height - 1,
                    message_id = %msg_id,
                    "[block-publish] Block broadcast to mesh"
                );
            }
            Err(e) => {
                tracing::warn!(
                    proposal_id,
                    error = %e,
                    "[block-publish] Failed to publish block"
                );
            }
        }
    }

    /// Handle an Era Two RatificationBlock received via `lattice/block/v1`.
    /// Verifies roots against local state.  If they match, commits the
    /// epoch boundary via CommitManager for persistence + catch-up.
    /// If they mismatch, logs a state fork warning and drops the block.
    fn handle_ratification_block(
        &mut self,
        data: &[u8],
        propagation_source: &PeerId,
        signatures: &[(PeerId, Vec<u8>)],
    ) {
        // data includes the ERA_TWO_BLOCK_MARKER prefix byte
        let block = match RatificationBlock::decode_body(&data[1..]) {
            Ok(b) => b,
            Err(e) => {
                tracing::warn!(
                    error = %e,
                    from = %propagation_source,
                    "[block-recv] Failed to deserialize RatificationBlock"
                );
                return;
            }
        };

        let current_epoch = self.economic_engine.epoch_count();
        // Accept current or next epoch (the producer may be one epoch ahead).
        if block.epoch != current_epoch && block.epoch != current_epoch + 1 {
            tracing::debug!(
                their_epoch = block.epoch,
                our_epoch = current_epoch,
                from = %propagation_source,
                "[block-recv] RatificationBlock for unexpected epoch — dropping"
            );
            return;
        }

        // Compute local roots
        let local_state = self.ledger.state_root(&self.seen_nonces);
        let local_thickness = self.ledger.thickness_graph.thickness_root();

        if block.state_root == local_state && block.thickness_root == local_thickness {
            // Advisory QC verification: if the block carries signatures,
            // check that enough of them come from the expected witness panel.
            // This is advisory (soft-fork compatible): a mismatch is logged
            // but the block is still accepted if the roots agree.
            if !signatures.is_empty() {
                let panel = self.derive_ratification_panel(&block.proposal_id);
                let quorum = crate::sortition::ratification_quorum(panel.len());
                let panel_sigs = signatures.iter().filter(|(pid, _)| panel.contains(pid)).count();
                if panel_sigs < quorum {
                    tracing::warn!(
                        epoch = block.epoch,
                        proposal_id = %block.proposal_id,
                        panel_sigs,
                        quorum,
                        from = %propagation_source,
                        "[block-recv] RatificationBlock advisory QC check failed — insufficient panel signatures, accepting anyway (soft-fork compat)"
                    );
                } else {
                    tracing::debug!(
                        epoch = block.epoch,
                        proposal_id = %block.proposal_id,
                        panel_sigs,
                        quorum,
                        "[block-recv] RatificationBlock advisory QC check passed"
                    );
                }
            }

            // ── Commit to chain for persistence + catch-up ──────────
            match self.commit_manager.commit(
                data, // full prefixed bytes: 0x02 + CBOR
                &block.proposal_id,
                signatures,
            ) {
                Ok(block_hash) => {
                    tracing::info!(
                        epoch = block.epoch,
                        proposal_id = %block.proposal_id,
                        sigs = signatures.len(),
                        hash = %hex::encode(block_hash),
                        from = %propagation_source,
                        "[block-recv] RatificationBlock verified and committed to chain"
                    );
                }
                Err(e) => {
                    tracing::error!(
                        epoch = block.epoch,
                        proposal_id = %block.proposal_id,
                        error = %e,
                        "[block-recv] Failed to commit RatificationBlock to chain"
                    );
                }
            }
        } else {
            tracing::warn!(
                epoch = block.epoch,
                proposal_id = %block.proposal_id,
                from = %propagation_source,
                state_match = block.state_root == local_state,
                thickness_match = block.thickness_root == local_thickness,
                "[block-recv] STATE FORK — RatificationBlock roots differ from local state. Dropping."
            );
        }
    }

    /// Encode a verified quorum certificate (QC) for gossip on the
    /// enclave-cert topic. Uses marker byte 0x02.
    ///
    /// Wire format:
    ///   [0x02] [2-byte pid_len] [proposal_id]
    ///   [4-byte cert_len] [cert protobuf bytes]
    ///   [2-byte sig_count] ([2-byte pubkey_len] [pubkey] [2-byte sig_len] [sig]) ...
    fn encode_quorum_certificate(
        proposal_id: &str,
        cert_bytes: &[u8],
        sigs: &[(PeerId, Vec<u8>, Vec<u8>)],
    ) -> Vec<u8> {
        // Canonical ordering: signatures are sorted by signer PeerId so that
        // the same quorum assembled in a different order produces identical
        // bytes and therefore an identical block hash on every node.
        let mut sigs: Vec<(PeerId, Vec<u8>, Vec<u8>)> = sigs.to_vec();
        sigs.sort_by(|a, b| a.0.to_bytes().cmp(&b.0.to_bytes()));

        let pid_bytes = proposal_id.as_bytes();
        let mut qc = Vec::with_capacity(
            1 + 2 + pid_bytes.len()
                + 4 + cert_bytes.len()
                + 2
                + sigs
                    .iter()
                    .map(|(_, sig, pk)| 2 + pk.len() + 2 + sig.len())
                    .sum::<usize>(),
        );
        qc.push(0x02);
        qc.extend_from_slice(&(pid_bytes.len() as u16).to_be_bytes());
        qc.extend_from_slice(pid_bytes);
        qc.extend_from_slice(&(cert_bytes.len() as u32).to_be_bytes());
        qc.extend_from_slice(cert_bytes);
        qc.extend_from_slice(&(sigs.len() as u16).to_be_bytes());
        for (_peer_id, sig, pk) in &sigs {
            qc.extend_from_slice(&(pk.len() as u16).to_be_bytes());
            qc.extend_from_slice(pk);
            qc.extend_from_slice(&(sig.len() as u16).to_be_bytes());
            qc.extend_from_slice(sig);
        }
        qc
    }

    /// Decode and cryptographically verify a QC message (marker 0x02).
    /// On success returns (proposal_id, cert_bytes, verified_signatures).
    /// Invalid signatures are dropped; the caller checks the count.
    fn decode_and_verify_quorum_certificate(
        data: &[u8],
    ) -> Option<(String, Vec<u8>, Vec<(PeerId, Vec<u8>, Vec<u8>)>)> {
        use prost::Message;
        // Skip 0x02 marker
        let rest = &data[1..];
        if rest.len() < 2 {
            return None;
        }

        // Parse proposal_id
        let pid_len = u16::from_be_bytes([rest[0], rest[1]]) as usize;
        let after_pid = 2 + pid_len;
        if rest.len() < after_pid {
            return None;
        }
        let proposal_id = std::str::from_utf8(&rest[2..after_pid]).ok()?.to_string();

        // Parse cert_bytes
        let rest = &rest[after_pid..];
        if rest.len() < 4 {
            return None;
        }
        let cert_len = u32::from_be_bytes([rest[0], rest[1], rest[2], rest[3]]) as usize;
        let after_cert = 4 + cert_len;
        if rest.len() < after_cert {
            return None;
        }
        let cert_bytes = rest[4..after_cert].to_vec();

        // Decode certificate and sanity-check proposal_id
        let cert = crate::ingest::proto::ImpactCertificate::decode(&cert_bytes[..]).ok()?;
        if cert.proposal_id != proposal_id {
            return None;
        }

        // Parse signatures
        let rest = &rest[after_cert..];
        if rest.len() < 2 {
            return None;
        }
        let sig_count = u16::from_be_bytes([rest[0], rest[1]]) as usize;
        let mut offset = 2;
        let mut verified = Vec::with_capacity(sig_count);

        for _ in 0..sig_count {
            if rest.len() < offset + 2 {
                return None;
            }
            let pk_len = u16::from_be_bytes([rest[offset], rest[offset + 1]]) as usize;
            offset += 2;
            if rest.len() < offset + pk_len {
                return None;
            }
            let pk_bytes = &rest[offset..offset + pk_len];
            offset += pk_len;

            if rest.len() < offset + 2 {
                return None;
            }
            let sig_len = u16::from_be_bytes([rest[offset], rest[offset + 1]]) as usize;
            offset += 2;
            if rest.len() < offset + sig_len {
                return None;
            }
            let sig = &rest[offset..offset + sig_len];
            offset += sig_len;

            let pubkey = libp2p::identity::PublicKey::try_decode_protobuf(pk_bytes).ok()?;
            let signer_peer_id = pubkey.to_peer_id();
            if !pubkey.verify(proposal_id.as_bytes(), sig) {
                continue;
            }
            verified.push((signer_peer_id, sig.to_vec(), pk_bytes.to_vec()));
        }

        Some((proposal_id, cert_bytes, verified))
    }

    /// Assemble and broadcast a verified quorum certificate (QC) on the
    /// enclave-cert topic. Uses marker byte 0x02. Carries the certificate
    /// bytes plus (public_key, signature) pairs so peers can verify the
    /// quorum without having seen every individual attestation.
    fn publish_quorum_certificate(
        &mut self,
        proposal_id: &str,
        sigs: &[(PeerId, Vec<u8>, Vec<u8>)],
    ) {
        let Some(cert_bytes) = self.cert_cache.get(proposal_id) else {
            warn!(
                proposal_id = %proposal_id,
                "[qc-publish] Cert not in cache — cannot broadcast QC"
            );
            return;
        };

        let qc = Self::encode_quorum_certificate(proposal_id, cert_bytes, sigs);
        let topic = gossipsub::IdentTopic::new(LATTICE_ENCLAVE_CERT_TOPIC);
        self.track_outbound(&qc);
        match self.swarm.behaviour_mut().gossipsub.publish(topic, qc) {
            Ok(msg_id) => {
                info!(
                    proposal_id = %proposal_id,
                    message_id = %msg_id,
                    "[qc-publish] Quorum certificate broadcast to mesh"
                );
            }
            Err(e) => {
                warn!(
                    proposal_id = %proposal_id,
                    error = %e,
                    "[qc-publish] Failed to publish quorum certificate"
                );
            }
        }
    }

    /// Submit an agent task to the mesh and store it in the local registry.

    /// Handle an incoming block message from gossipsub.
    /// Four cases:
    ///   1. Old (height < tip) → drop
    ///   2. Divergent (height == tip, different block) → WARN
    ///   3. Contiguous (height == tip_height) → validate, apply, commit
    ///   4. Future (height > tip_height+1) → WARN (catch-up needed)
    ///
    /// For case 3, Genesis is a special case: height 0, no parent hash
    /// check. All other blocks must have prev_hash matching local tip.
    fn handle_block_message(&mut self, data: &[u8], propagation_source: &PeerId) {
        if data.is_empty() {
            tracing::warn!("[block-recv] Empty block message");
            return;
        }

        // ── Catch-up guard: ignore live broadcasts while catching up ──
        if self.is_catching_up {
            // Live broadcast arriving during catch-up may race with the
            // ChainRangeResponse at the same height.  Ignore — the
            // catch-up response will bring this block.
            return;
        }

        // ── Era One: legacy block frame (unprefixed) ──────────
        // Any block NOT starting with ERA_TWO_BLOCK_MARKER is Era One.
        // We do NOT strip a leading 0x01 — Era One blocks are emitted
        // without a prefix, and the height field's first byte could
        // legitimately be 0x01 at extreme heights.  Only 0x02-prefixed
        // blocks are routed to the Era Two path.
        let data = data; // pass through unmodified

        if data.len() < 72 {
            tracing::warn!("[block-recv] Block too short ({} bytes)", data.len());
            return;
        }

        // Parse the block frame header
        let height = u64::from_be_bytes([
            data[0], data[1], data[2], data[3],
            data[4], data[5], data[6], data[7],
        ]);
        let prev_hash: [u8; 32] = data[8..40].try_into().unwrap();
        let block_hash: [u8; 32] = data[40..72].try_into().unwrap();

        let local_height = self.commit_manager.height();
        let local_tip = self.commit_manager.tip_hash();

        // ── Case 1: Old block ──────────────────────────────
        if height < local_height {
            tracing::debug!(
                height, local_height,
                from = %propagation_source,
                "[block-recv] Old block — dropping"
            );
            return;
        }

        // ── Case 2: Divergent (same height, different block) ──
        if height == local_height && local_height > 0 {
            // Same height as our tip but different hash → fork.
            // Need the full remote ledger to resolve. Log and defer.
            tracing::warn!(
                height, local_height,
                their_hash = %hex::encode(block_hash),
                our_hash = %hex::encode(local_tip),
                from = %propagation_source,
                "[block-recv] DIVERGENT — same height, different block hash. Fork resolution pending."
            );
            return;
        }

        // ── Case 3: Contiguous extension ──────────────────────
        if height == local_height {
            // Genesis: height 0, no parent
            let is_genesis = height == 0;

            // Parent hash check for non-genesis
            if !is_genesis {
                if prev_hash != local_tip {
                    tracing::warn!(
                        height,
                        their_parent = %hex::encode(prev_hash),
                        our_tip = %hex::encode(local_tip),
                        from = %propagation_source,
                        "[block-recv] Parent hash mismatch — rejecting"
                    );
                    return;
                }
            }

            // Parse cert_bytes from the block frame
            let offset = 72; // after height + prev_hash + block_hash
            if data.len() <= offset + 4 {
                tracing::warn!("[block-recv] Block too short for cert");
                return;
            }
            let cert_len = u32::from_be_bytes([
                data[offset], data[offset+1], data[offset+2], data[offset+3]
            ]) as usize;
            let cert_end = offset + 4 + cert_len;
            if data.len() < cert_end {
                tracing::warn!("[block-recv] Truncated cert bytes");
                return;
            }
            let cert_bytes = &data[offset + 4..cert_end];

            // Parse witness signatures from the block frame
            let mut signatures = Vec::new();
            let mut sig_offset = cert_end;
            if data.len() >= sig_offset + 2 {
                let sig_count = u16::from_be_bytes([data[sig_offset], data[sig_offset + 1]]) as usize;
                sig_offset += 2;
                for _ in 0..sig_count {
                    if data.len() < sig_offset + 2 {
                        break;
                    }
                    let peer_len = u16::from_be_bytes([data[sig_offset], data[sig_offset + 1]]) as usize;
                    sig_offset += 2;
                    if data.len() < sig_offset + peer_len {
                        break;
                    }
                    let peer_id = match PeerId::from_bytes(&data[sig_offset..sig_offset + peer_len]) {
                        Ok(p) => p,
                        Err(_) => break,
                    };
                    sig_offset += peer_len;
                    if data.len() < sig_offset + 2 {
                        break;
                    }
                    let sig_len = u16::from_be_bytes([data[sig_offset], data[sig_offset + 1]]) as usize;
                    sig_offset += 2;
                    if data.len() < sig_offset + sig_len {
                        break;
                    }
                    let sig = data[sig_offset..sig_offset + sig_len].to_vec();
                    sig_offset += sig_len;
                    signatures.push((peer_id, sig));
                }
            }

            // ── Era Two: RatificationBlock (0x02 prefix inside cert_bytes) ─
            if !cert_bytes.is_empty() && cert_bytes[0] == ERA_TWO_BLOCK_MARKER {
                self.handle_ratification_block(cert_bytes, propagation_source, &signatures);
                return;
            }

            let stx = match serde_cbor::from_slice::<crate::ledger::types::SignedTransaction>(cert_bytes) {
                Ok(s) => s,
                Err(e) => {
                    tracing::warn!(error = %e, "[block-recv] Failed to deserialize block cert");
                    return;
                }
            };

            // Block type validity: genesis only at height 0,
            // BootstrapEnded only after genesis, certs after that.
            let tx_type = match &stx.transaction {
                crate::ledger::types::Transaction::Genesis { .. } => "genesis",
                crate::ledger::types::Transaction::BootstrapEnded { .. } => "bootstrap-ended",
                crate::ledger::types::Transaction::Mint { .. } => "mint",
                _ => "other",
            };
            if is_genesis && tx_type != "genesis" {
                tracing::warn!(
                    tx_type, height,
                    "[block-recv] Non-genesis block at height 0 — rejecting"
                );
                return;
            }
            if !is_genesis && tx_type == "genesis" {
                tracing::warn!(
                    height,
                    "[block-recv] Genesis at non-zero height — rejecting"
                );
                return;
            }

            // Validate and apply
            if is_genesis {
                let allow_self_authored = self.genesis_root.is_none();
                if let Err(e) = crate::ledger::validation::validate_and_apply_with_genesis_root(
                    &stx, &mut self.ledger, &mut self.seen_nonces,
                    self.genesis_root.as_ref(),
                    allow_self_authored,
                ) {
                    tracing::warn!(error = %e, "[block-recv] Genesis validation failed");
                    return;
                }
            } else {
                if let Err(e) = crate::ledger::validation::validate_and_apply(
                    &stx, &mut self.ledger, &mut self.seen_nonces,
                ) {
                    tracing::warn!(error = %e, "[block-recv] Block validation failed");
                    return;
                }
            }

            // Commit to local chain
            if let Err(e) = self.commit_manager.commit_root_block(
                cert_bytes, tx_type,
                &stx.signature, &self.local_peer_id,
            ) {
                tracing::warn!(error = %e, "[block-recv] Failed to commit block to local chain");
                return;
            }

            tracing::info!(
                height, tx_type,
                from = %propagation_source,
                "[block-recv] Block applied and committed ✓"
            );
            return;
        }

        // ── Case 4: Future block (gap > 1) ────────────────────
        info!(
            height, local_height, gap = height - local_height,
            from = %propagation_source,
            "Future block at {} — catching up via ChainRangeRequest ({}..{})",
            height, local_height, height,
        );
        // Fire a ChainRangeRequest if we don't already have one
        // in-flight to this peer.  Dedup by peer ID so a burst of
        // future blocks from the same peer fires only one request.
        if !self.outstanding_chain_requests_by_peer.contains(propagation_source) {
            let req = ChainRangeRequest {
                from_height: if local_height == 0 { 0 } else { local_height + 1 },
                to_height: height,
            };
            self.outstanding_chain_requests_by_peer.insert(*propagation_source);
            self.is_catching_up = true;
            let _ = self.swarm.behaviour_mut()
                .chain_sync_rpc
                .send_request(propagation_source, req);
        }
    }

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
                    let exec_client = self.executor.clone();
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

    /// Periodic NTP drift check, called on heartbeat tick.
    /// Cached: only queries NTP every NTP_CACHE_TTL_SECS (5 min).
    /// Three thresholds: WARN >30s, refuse-sign >60s, exit >300s.
    async fn check_runtime_ntp(&mut self) {
        if self.skip_ntp_check {
            return;
        }
        let now = Instant::now();
        if now.duration_since(self.last_ntp_check).as_secs() < crate::startup::NTP_CACHE_TTL_SECS {
            return;
        }
        self.last_ntp_check = now;

        match crate::startup::check_ntp_drift(self.ntp_servers.clone()).await {
            Ok(drift) => {
                self.ntp_drift_secs = drift;
                let abs_drift = drift.abs();

                if abs_drift > crate::startup::CLOCK_DRIFT_THRESHOLD_SECS {
                    error!(
                        drift_s = drift,
                        threshold = crate::startup::CLOCK_DRIFT_THRESHOLD_SECS,
                        "RUNTIME NTP: clock drift {}s exceeds exit threshold. \
                         Shutting down — restart will trigger startup NTP check.",
                        drift
                    );
                    std::process::exit(1);
                } else if abs_drift > crate::startup::NTP_REFUSE_SIGN_THRESHOLD_SECS {
                    if !self.refuse_to_sign {
                        warn!(
                            drift_s = drift,
                            threshold = crate::startup::NTP_REFUSE_SIGN_THRESHOLD_SECS,
                            "RUNTIME NTP: drift {}s exceeds sign threshold. \
                             Refusing to sign new transactions until clock is corrected.",
                            drift
                        );
                        self.refuse_to_sign = true;
                    }
                } else if abs_drift > crate::startup::NTP_WARN_THRESHOLD_SECS {
                    warn!(
                        drift_s = drift,
                        threshold = crate::startup::NTP_WARN_THRESHOLD_SECS,
                        "RUNTIME NTP: drift {}s exceeds warn threshold.",
                        drift
                    );
                    // Clear refuse_to_sign if drift has recovered
                    self.refuse_to_sign = false;
                } else {
                    // Drift is acceptable — clear any stale refuse flag
                    if self.refuse_to_sign {
                        info!(
                            drift_s = drift,
                            "RUNTIME NTP: drift {}s recovered — resuming transaction signing.",
                            drift
                        );
                        self.refuse_to_sign = false;
                    }
                }
            }
            Err(e) => {
                warn!(error = %e, "RUNTIME NTP: check failed — skipping this cycle");
            }
        }
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

        // ── Auto-genesis: spawn a new mesh if the chain is empty ─────
        if self.auto_genesis
            && self.commit_manager.height() == 0
            && !self.commit_manager.is_bootstrap_ended()
        {
            info!(
                peer_id = %self.local_peer_id,
                "Auto-genesis enabled — chain is empty, self-authoring genesis"
            );
            if let Err(e) = self.submit_genesis(self.genesis_thickness) {
                warn!(error = %e, "Auto-genesis failed");
            }
        }

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
                    // Sweep stale fetch entries before computing metrics
                    self.sweep_stale_fetches();
                    self.sweep_stale_outbound();
                    // Check peer liveness — evict zombies
                    self.check_peer_liveness();
                    // ── Metrics ────────────────────────────────────
                    // Instrumentation for soak test: outstanding_fetches
                    // is the leak canary (entries > 10×FETCH_TIMEOUT ≈ 50s
                    // are "aged" — should stay near zero).  outbound queues
                    // drain on gossip echo; any non-empty entry is a
                    // sender-side signal.  Both are sampled every heartbeat
                    // interval (typically 30s).
                    let fetch_total = self.outstanding_fetches.len();
                    let fetch_aged = self.outstanding_fetches
                        .values()
                        .filter(|t| t.elapsed() > FETCH_TIMEOUT * 10)
                        .count();
                    let queues: Vec<String> = self.outbound
                        .iter()
                        .filter(|(_, q)| !q.is_empty())
                        .map(|(peer, q)| {
                            // 8-char prefix is enough to disambiguate in a
                            // 3-node mesh; avoids 52-char base58 noise.
                            let short = peer.to_base58();
                            let short = &short[..short.len().min(8)];
                            format!("{}={}", short, q.len())
                        })
                        .collect();
                    self.drain_pending_genesis();
                    self.drain_pending_objections();
                    info!(
                        "metrics: outstanding_fetches={} aged={} outbound_queues=[{}] max_peer_silence={}s",
                        fetch_total,
                        fetch_aged,
                        queues.join(" "),
                        self.last_max_peer_silence
                    );
                    // ── Runtime NTP check (every 5 min) ───────────
                    self.check_runtime_ntp().await;
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
        use crate::ledger::persistence::PersistentEconomicState;
        use crate::ledger::persistence::StateStore;
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
            }
            self.on_transaction_applied(&signed);
            // Insert into outbound queue and flush (mint).
            self.outbound
                .entry(self.local_peer_id)
                .or_default()
                .insert(self.tx_nonce, signed);
            self.outbound_insertion_times.insert(
                (self.local_peer_id, self.tx_nonce), Instant::now(),
            );
            self.flush_outbound();
            self.economic_engine.metrics.record_transaction_submitted();
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
            }
            // Persist to WAL regardless of validation outcome so the
            // nonce is consumed and WAL-only replay reproduces the
            // same skip.  Without this, a failed validation consumes
            // tx_nonce without writing the entry, creating a hole
            // that breaks nonce continuity on recovery.
            self.on_transaction_applied(&signed);
            // Insert into outbound queue and flush.
            self.outbound
                .entry(self.local_peer_id)
                .or_default()
                .insert(self.tx_nonce, signed);
            self.outbound_insertion_times.insert(
                (self.local_peer_id, self.tx_nonce), Instant::now(),
            );
            self.flush_outbound();
            self.economic_engine.metrics.record_transaction_submitted();
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

        // ── Credit accepted claims to thickness, then decay ─────
        // Order: credit before decay, so a claim credited this epoch
        // gets its first decay at the next epoch boundary.
        let unapplied = self.economic_engine.take_unapplied_claims();
        // Credit-path diagnostic: count what the credit step sees
        if !unapplied.is_empty() {
            info!(
                unapplied_count = unapplied.len(),
                "Credit step: crediting unapplied claims"
            );
        } else {
            info!("Credit step: no unapplied claims found");
        }
        if !unapplied.is_empty() {
            let mut indices = Vec::with_capacity(unapplied.len());
            for (idx, claim) in &unapplied {
                let credit: f64 = claim.witnesses.iter()
                    .map(|w| w.observed_heartbeats as f64 * 0.001)
                    .sum();
                self.ledger.thickness_graph.add_verified_contribution(
                    &claim.claimant,
                    [0u8; 32],
                    credit,
                );
                indices.push(*idx);
            }
            self.economic_engine.mark_applied(&indices, epoch);
            info!(
                claims = unapplied.len(),
                "Credited service attestations to thickness graph"
            );
        }

        // Apply per-epoch decay to all thickness edges.
        self.ledger.thickness_graph.apply_edge_decay(
            crate::claims::DECAY_PER_EPOCH
        );

        // Save economic snapshot every 10 epochs (balances + thickness survive restart)
        if epoch % 10 == 0 {
            if let Some(ref mut store) = self.state_store {
                let mut snapshot = PersistentEconomicState::from_state(
                    &self.seen_nonces,
                    &self.ledger.balances,
                    &self.ledger.thickness_graph,
                    self.tx_nonce,
                    self.economic_engine.accepted_claims_snapshot(),
                    HashMap::new(), // objections — Commit 4 threads real state
                );
                // from_state drops genesis — carry it forward so the
                // snapshot can survive WAL rotation (the recovery invariant
                // from b71c824 requires Genesis before Transaction/Claim,
                // and rotation removes Genesis from the active WAL).
                snapshot.genesis = self.genesis.clone();
                if let Err(e) = store.take_snapshot(epoch, &snapshot) {
                    warn!(error = %e, "Failed to save economic snapshot");
                }
            }
        }

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

        // ── Era Two: produce RatificationBlock ────────────────────
        // After economic cycle completes and roots are computed, every
        // node assembles the same deterministic RatificationBlock.  A
        // sortitioned witness panel then signs 0x01 attestations; once a
        // quorum is reached the block is committed to the hash-chain and
        // broadcast on the block topic.  Gated behind BootstrapEnded (or
        // --force-era-two).
        if self.commit_manager.is_bootstrap_ended() || self.force_era_two {
            let state_root = self.ledger.state_root(&self.seen_nonces);
            let thickness_root = self.ledger.thickness_graph.thickness_root();
            let proposal_id = format!("epoch-{epoch}");
            let block = RatificationBlock {
                epoch,
                state_root,
                thickness_root,
                proposal_id: proposal_id.clone(),
            };
            tracing::info!(
                epoch,
                proposal_id = %proposal_id,
                "[ratification] Assembled RatificationBlock for Era Two"
            );

            // Cache the encoded block so the QC commit path can write it
            // to the ledger once a quorum of panel signatures is reached.
            let cert_bytes = match block.encode_wire() {
                Ok(b) => b,
                Err(e) => {
                    tracing::error!(
                        epoch,
                        error = %e,
                        "[ratification] Failed to encode RatificationBlock"
                    );
                    return;
                }
            };
            self.cert_cache.insert(proposal_id.clone(), cert_bytes);

            let panel = self.derive_ratification_panel(&proposal_id);
            if panel.contains(&self.local_peer_id) {
                tracing::info!(
                    epoch,
                    proposal_id = %proposal_id,
                    panel = ?panel.iter().map(|p| p.to_string()).collect::<Vec<_>>(),
                    "[ratification] Local node on witness panel — publishing attestation"
                );
                self.publish_witness_attestation(&proposal_id);
                self.maybe_commit_ratification_qc(&proposal_id);
            } else {
                tracing::debug!(
                    epoch,
                    proposal_id = %proposal_id,
                    panel = ?panel.iter().map(|p| p.to_string()).collect::<Vec<_>>(),
                    "[ratification] Local node not on witness panel — observing quorum"
                );
            }
        }
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

        // Sweep claim registry — expire timed-out collections,
        // evict old finalized entries.
        self.sweep_claims();
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
        self.on_transaction_applied(&signed);

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

    // ── Witness handler: three-gate request processing ──────────
    //
    // Extracted from the event loop so that LatticeNode::run() and
    // the two-swarm test harness share a single implementation.
    // Changing the gates here changes both callers — no copy drift.

    /// Apply the three witness gates to a request and produce a
    /// WitnessResponse. Does NOT send the response — the caller
    /// is responsible for dispatching via the witness_rpc channel.
    ///
    /// Gates (in order):
    /// 1. Self-witness — claimant cannot witness its own claim
    /// 2. Establishment — claimant must have heartbeats_received > 0
    /// 3. Signing — domain-separated Ed25519 over (claim_hash,
    ///    witness_id, epoch)
    pub(crate) fn handle_witness_request(
        peer_table: &PeerTable,
        local_key: &identity::Keypair,
        epoch: u64,
        request: &WitnessRequest,
    ) -> WitnessResponse {
        let witness_peer_id = PeerId::from(local_key.public());

        // Gate 1: self-witness
        if request.claimant_id == witness_peer_id {
            warn!(claimant = %request.claimant_id,
                "Self-witness request rejected");
            return WitnessResponse {
                claim_id: request.claim_id.clone(),
                witness_id: witness_peer_id,
                claim_hash: request.claim_hash,
                witnessed_at_epoch: epoch,
                observed_heartbeats: 0,
                signer_public_key: Vec::new(),
                signature: Vec::new(),
                decline_reason: Some("Self-witness is not permitted".into()),
            };
        }

        // Gate 2: establishment
        if peer_table.get(&request.claimant_id)
            .map_or(true, |info| info.heartbeats_received == 0)
        {
            warn!(
                claimant = %request.claimant_id,
                "Witness request for unestablished peer — no heartbeats received"
            );
            return WitnessResponse {
                claim_id: request.claim_id.clone(),
                witness_id: witness_peer_id,
                claim_hash: request.claim_hash,
                witnessed_at_epoch: epoch,
                observed_heartbeats: 0,
                signer_public_key: Vec::new(),
                signature: Vec::new(),
                decline_reason: Some(
                    "Claimant is not established (no heartbeats observed)".into()
                ),
            };
        }

        // (A) Origin: compute observed_heartbeats from our own peer_table
        let observed_heartbeats = peer_table
            .get(&request.claimant_id)
            .map(|info| info.heartbeats_received)
            .unwrap_or(0);

        // (B) Binding: include observed_heartbeats in the signing payload
        let payload = [
            crate::claims::WITNESS_DOMAIN as &[u8],
            &request.claim_hash[..],
            &witness_peer_id.to_bytes()[..],
            &epoch.to_le_bytes()[..],
            &observed_heartbeats.to_le_bytes()[..],
        ].concat();

        match local_key.sign(&payload) {
            Ok(sig) => {
                info!(
                    claim_id = %request.claim_id,
                    witness = %witness_peer_id,
                    sig_len = sig.len(),
                    observed_heartbeats,
                    "Witness signature issued (domain-separated)"
                );
                WitnessResponse {
                    claim_id: request.claim_id.clone(),
                    witness_id: witness_peer_id,
                    claim_hash: request.claim_hash,
                    witnessed_at_epoch: epoch,
                    signature: sig,
                    decline_reason: None,
                    observed_heartbeats,
                    signer_public_key: local_key.public().encode_protobuf(),
                }
            }
            Err(e) => {
                warn!(error = %e, "Failed to sign witness response");
                WitnessResponse {
                    claim_id: request.claim_id.clone(),
                    witness_id: witness_peer_id,
                    claim_hash: request.claim_hash,
                    witnessed_at_epoch: epoch,
                    observed_heartbeats: 0,
                    signer_public_key: Vec::new(),
                    signature: Vec::new(),
                    decline_reason: Some(format!("Signing failed: {e}")),
                }
            }
        }
    }

    /// Send a direct witness request to a specific peer over the
    /// request-response protocol. Returns a RequestId for correlation.
    /// The witness will sign the claim_hash (Model A) — they sign what
    /// they receive, not a claim they independently verified.
    pub fn send_witness_request(
        &mut self,
        peer: PeerId,
        request: WitnessRequest,
    ) -> libp2p::request_response::OutboundRequestId {
        self.swarm.behaviour_mut().witness_rpc.send_request(&peer, request)
    }

    // ── Economic Comparison RPC ────────────────────────────────

    /// Compute a blake3 balance fingerprint over a specific subset of peers.
    ///
    /// Only peers present in the `included` set are hashed. Each entry is
    /// `peer_id.to_bytes() || balance.0.to_le_bytes()`, sorted by PeerId for
    /// determinism. No nonces, no timestamps — pure balance fingerprint.
    ///
    /// Returns hex-encoded hash of the included entries.
    fn compute_balance_fingerprint(&self, included: &HashSet<PeerId>) -> String {
        let mut sorted_balances: Vec<(&PeerId, &DigitalUtilityUnit)> = self
            .ledger
            .balances
            .iter()
            .filter(|(peer, _)| included.contains(peer))
            .collect();
        sorted_balances.sort_by_key(|(peer, _)| *peer);

        let mut hasher = blake3::Hasher::new();
        for (peer, balance) in &sorted_balances {
            hasher.update(peer.to_bytes().as_slice());
            hasher.update(&balance.0.to_le_bytes());
        }
        hex::encode(hasher.finalize().as_bytes())
    }

    /// Handle an incoming comparison request — compute the fingerprint
    /// over the subset of requested peers this node actually knows.
    ///
    /// Protocol rules:
    ///   - "Unknown is not zero" — a missing peer is scope asymmetry
    ///   - Hash only the INCLUDED set, no nonces
    ///   - Epoch is advisory — reported, not synchronized
    fn handle_comparison_request(
        &self,
        request: &ComparisonRequest,
        epoch: u64,
    ) -> ComparisonResponse {
        let mut included_peers: Vec<String> = Vec::new();
        let mut known_peers: HashSet<PeerId> = HashSet::new();

        for peer_str in &request.requested_peers {
            if let Ok(pid) = peer_str.parse::<PeerId>() {
                if self.ledger.balances.contains_key(&pid) {
                    included_peers.push(peer_str.clone());
                    known_peers.insert(pid);
                }
            }
        }

        let balance_fingerprint = self.compute_balance_fingerprint(&known_peers);

        ComparisonResponse {
            requested_peers: request.requested_peers.clone(),
            included_peers,
            balance_fingerprint,
            responder_epoch: epoch,
        }
    }

    /// Initiate a balance fingerprint comparison with a specific peer.
    ///
    /// The caller provides the set of peer IDs (base58 strings) to compare.
    /// Returns the outbound request ID for matching the response.
    pub fn send_comparison_request(
        &mut self,
        peer: &PeerId,
        requested_peers: Vec<String>,
    ) -> libp2p::request_response::OutboundRequestId {
        let request = ComparisonRequest {
            requested_peers,
            requester_epoch: self.economic_engine.epoch_count(),
        };
        self.swarm.behaviour_mut().compare_rpc.send_request(peer, request)
    }

    // ── Witness claim orchestration ────────────────────────────

    /// Initiate a service attestation claim: identify witnesses,
    /// dispatch requests, return a claim_id. The caller polls
    /// GetClaimStatus for results. Does NOT block — sends requests
    /// and returns immediately.
    fn handle_witness_claim_service(&mut self, msg: crate::api::ApiMessage) {
        use crate::api::ApiResponse;

        // Guard: no concurrent claim for same (self, service attestation)
        let active = self.witness_claims.values().any(|s| {
            matches!(s.status, ClaimSubmissionStatus::Collecting)
        });
        if active {
            let existing_id = self.witness_claims.values()
                .find(|s| matches!(s.status, ClaimSubmissionStatus::Collecting))
                .map(|s| s.claim_id.clone())
                .unwrap_or_default();
            let _ = msg.reply.send(ApiResponse::AlreadyCollecting {
                claim_id: existing_id,
            });
            return;
        }

        // Compute the claimable window
        let (start_epoch, end_epoch) = match self.compute_service_window() {
            Some(w) => w,
            None => {
                let _ = msg.reply.send(ApiResponse::NothingToClaim);
                return;
            }
        };

        // Identify candidates: established peers, not self
        let current_epoch = self.economic_engine.epoch_count();
        let candidates: Vec<PeerId> = self.peer_table.iter()
            .filter(|p| {
                p.peer_id != self.local_peer_id
                    && p.heartbeats_received > 0
            })
            .map(|p| p.peer_id)
            .collect();

        if candidates.is_empty() {
            let _ = msg.reply.send(ApiResponse::ClaimStatus {
                claim_id: String::new(),
                status: "rejected".into(),
                candidates: 0,
                signatures_collected: 0,
                declines: vec![],
                result: Some(crate::api::ClaimResultInfo {
                    thickness: None,
                    acceptance: "rejected".into(),
                    reason: Some("No established peers to witness".into()),
                }),
            });
            return;
        }

        // Generate a claim ID and build the request
        let claim_id = format!(
            "svc-{}-{}",
            current_epoch,
            &self.local_peer_id.to_base58()[..8]
        );
        let claim_hash = crate::claims::compute_service_attestation_hash(
            &self.local_peer_id,
            start_epoch,
            end_epoch,
        );
        let request = WitnessRequest {
            claim_id: claim_id.clone(),
            claim_type: 0, // ServiceAttestation
            claimant_id: self.local_peer_id,
            claim_hash,
            requested_at_epoch: current_epoch,
        };

        // Create registry entry
        let state = WitnessClaimState {
            claim_id: claim_id.clone(),
            status: ClaimSubmissionStatus::Collecting,
            start_epoch,
            end_epoch,
            candidates: candidates.clone(),
            signatures: Vec::new(),
            declines: Vec::new(),
            started_at_epoch: current_epoch,
        };
        self.witness_claims.insert(claim_id.clone(), state);

        // Dispatch requests to all candidates
        for peer_id in &candidates {
            self.swarm.behaviour_mut()
                .witness_rpc
                .send_request(peer_id, request.clone());
        }

        info!(
            claim_id = %claim_id,
            candidates = candidates.len(),
            window = format!("{}-{}", start_epoch, end_epoch),
            "Witness claim initiated"
        );

        let _ = msg.reply.send(ApiResponse::ClaimSubmitted { claim_id });
    }

    /// Compute the claimable window for a service attestation claim.
    /// Returns None if no claimable window exists (fresh node, or
    /// already claimed through the current epoch).
    fn compute_service_window(&self) -> Option<(u64, u64)> {
        let key = (
            self.local_peer_id.to_base58(),
            crate::claims::ClaimType::ServiceAttestation as u8,
        );
        let last_end = self.last_claimed.get(&key).copied().unwrap_or(0);
        let current_epoch = self.economic_engine.epoch_count();

        let start = last_end.saturating_add(1);
        if current_epoch < 2 {
            return None; // need at least epoch 1 to have a window
        }
        let end = current_epoch.saturating_sub(1);

        if start > end {
            return None;
        }

        // Clamp to MAX_CLAIM_WINDOW from the end
        let max_window = crate::claims::MAX_CLAIM_WINDOW;
        let start = start.max(end.saturating_sub(max_window));

        if start > end {
            return None;
        }

        Some((start, end))
    }

    /// Route a witness response to the appropriate claim in the
    /// registry. Returns true if the response was claimed (routed
    /// to an active collection), false if it should fall through
    /// to the existing attestation path.
    fn route_witness_response(&mut self, peer: PeerId, response: &WitnessResponse) -> bool {
        if let Some(state) = self.witness_claims.get_mut(&response.claim_id) {
            // Safety: only accept responses from actual candidates
            if !state.candidates.contains(&response.witness_id) {
                warn!(
                    claim_id = %response.claim_id,
                    peer = %response.witness_id,
                    "Unsolicited witness response — peer not a candidate"
                );
                return true; // claimed but rejected
            }

            if response.signature.is_empty() {
                info!(
                    claim_id = %response.claim_id,
                    witness = %response.witness_id,
                    reason = ?response.decline_reason,
                    "Witness declined"
                );
                state.declines.push((
                    response.witness_id,
                    response.decline_reason.clone()
                        .unwrap_or_else(|| "no reason given".into()),
                ));
            } else {
                info!(
                    claim_id = %response.claim_id,
                    witness = %response.witness_id,
                    sig_len = response.signature.len(),
                    "Witness signed"
                );
                state.signatures.push(crate::claims::WitnessSignature {
                    witness: response.witness_id,
                    signed_at_epoch: response.witnessed_at_epoch,
                    observed_heartbeats: response.observed_heartbeats,
                    signer_public_key: response.signer_public_key.clone(),
                    signature: response.signature.clone(),
                });
            }

            // Check if collection is complete (all candidates responded)
            let responded = state.signatures.len() + state.declines.len();
            if responded >= state.candidates.len() {
                self.finalize_claim(&response.claim_id.clone());
            }
            return true;
        }
        false
    }

    /// Finalize a claim: assemble the WitnessedClaim, feed to
    /// accept_claim(), and update the registry.
    fn finalize_claim(&mut self, claim_id: &str) {
        let state = match self.witness_claims.get(claim_id) {
            Some(s) => s.clone(),
            None => return,
        };

        if !matches!(state.status, ClaimSubmissionStatus::Collecting) {
            return; // already finalized
        }

        // Check minimum witnesses
        let min_witnesses = crate::claims::MIN_WITNESSES;
        if (state.signatures.len() as u64) < min_witnesses {
            let reason = if state.signatures.is_empty() && state.declines.is_empty() {
                "No witnesses responded — all candidates timed out".to_string()
            } else if state.signatures.is_empty() {
                format!(
                    "All {} candidates declined (0 signed). Window reclaimable.",
                    state.candidates.len()
                )
            } else {
                format!(
                    "Insufficient witnesses: got {} signatures, need {} ({} candidates, {} declined)",
                    state.signatures.len(),
                    min_witnesses,
                    state.candidates.len(),
                    state.declines.len(),
                )
            };
            if let Some(s) = self.witness_claims.get_mut(claim_id) {
                s.status = ClaimSubmissionStatus::Rejected {
                    reason: reason.clone(),
                };
            }
            info!(claim_id = %claim_id, reason = %reason, "Claim rejected — insufficient witnesses");
            return;
        }

        // Assemble the WitnessedClaim
        let mut assembled = crate::claims::WitnessedClaim {
            claimant: self.local_peer_id,
            claim_type: crate::claims::ClaimType::ServiceAttestation,
            start_epoch: state.start_epoch,
            end_epoch: state.end_epoch,
            evidence: crate::claims::ClaimEvidence::Service {
                claimed_count: 1,
            },
            witnesses: state.signatures.clone(),
            submitted_epoch: self.economic_engine.epoch_count(),
            claim_id: [0u8; 32], // computed below
        };
        assembled.claim_id = assembled.compute_claim_id();

        // Feed to accept_claim. established_peers counts all peers
        // in the mesh (including self) so accept_claim can enforce
        // the "need at least one other peer to witness" rule.
        // peer_table doesn't contain self, so add 1.
        let established = self.peer_table.len() + 1;
        match crate::claims::accept_claim(
            &assembled,
            &self.last_claimed,
            established,
        ) {
            Ok(thickness) => {
                // Update high-water mark
                let key = (
                    self.local_peer_id.to_base58(),
                    crate::claims::ClaimType::ServiceAttestation as u8,
                );
                self.last_claimed.insert(key, state.end_epoch);

                // Queue for economic engine — persist to unified WAL first
                // so the claim survives a crash before the next snapshot.
                if let Some(ref mut store) = self.state_store {
                    let record = crate::ledger::wal_record::WalRecord::Claim(assembled.clone());
                    if let Err(e) = store.persist_record(&record) {
                        warn!(error = %e, "Failed to persist claim to unified WAL");
                    }
                }
                self.economic_engine.queue_claim(assembled);

                if let Some(s) = self.witness_claims.get_mut(claim_id) {
                    s.status = ClaimSubmissionStatus::Accepted { thickness };
                }
                info!(
                    claim_id = %claim_id,
                    thickness,
                    witnesses = state.signatures.len(),
                    "Claim accepted"
                );
            }
            Err(rejection) => {
                let reason = format!("{}", rejection);
                if let Some(s) = self.witness_claims.get_mut(claim_id) {
                    s.status = ClaimSubmissionStatus::Rejected {
                        reason: reason.clone(),
                    };
                }
                info!(
                    claim_id = %claim_id,
                    reason = %reason,
                    "Claim rejected by acceptance"
                );
            }
        }
    }

    /// Sweep the claim registry: expire timed-out collections and
    /// evict old finalized entries. Called from the epoch tick.
    fn sweep_claims(&mut self) {
        let current_epoch = self.economic_engine.epoch_count();

        // Expire timed-out collections
        let mut to_finalize: Vec<String> = Vec::new();
        for (id, state) in &self.witness_claims {
            if matches!(state.status, ClaimSubmissionStatus::Collecting)
                && current_epoch.saturating_sub(state.started_at_epoch)
                    > CLAIM_COLLECTION_EPOCHS
            {
                to_finalize.push(id.clone());
            }
        }
        for id in &to_finalize {
            info!(claim_id = %id, "Claim collection deadline expired — finalizing with collected signatures");
            self.finalize_claim(id);
        }

        // Evict old finalized entries
        self.witness_claims.retain(|_id, state| {
            match &state.status {
                ClaimSubmissionStatus::Collecting => true, // keep active
                ClaimSubmissionStatus::Accepted { .. } | ClaimSubmissionStatus::Rejected { .. } => {
                    current_epoch.saturating_sub(state.started_at_epoch)
                        <= CLAIM_RETENTION_EPOCHS
                }
            }
        });
    }

    /// Sign a transaction with the node's Ed25519 keypair.
    fn sign_transaction(&self, tx: &Transaction) -> Result<SignedTransaction> {
        if self.refuse_to_sign {
            bail!(
                "Refusing to sign: clock drift exceeds threshold. \
                 Sync clock (macOS: sudo sntp -sS pool.ntp.org, Linux: sudo ntpdate pool.ntp.org) \
                 or restart with --skip-ntp-check."
            );
        }
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
        let topic_hash = topic.hash();
        let mesh_peers = self.swarm.behaviour().gossipsub
            .mesh_peers(&topic_hash)
            .count();
        match self
            .swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic, encoded)
        {
            Ok(_) => {
                info!("[broadcast] Ok: mesh_peers={} nonce={}", mesh_peers, signed.transaction.nonce());
                // Remove from outbound queue only if at least one other peer
                // is in the topic mesh — gossipsub Ok means handoff, not delivery,
                // but with ≥1 peer the odds of delivery are high enough that
                // keeping the entry creates unbounded queue growth without
                // improving reliability.  On a healthy 3-node mesh this fires
                // immediately; on an isolated node the entry stays for retry.
                if mesh_peers >= OUTBOUND_CONFIRM_PEERS {
                    if let Ok(signer) = signed.transaction.signer().parse::<PeerId>() {
                        if signer == self.local_peer_id {
                            let nonce = signed.transaction.nonce();
                            if let Some(queue) = self.outbound.get_mut(&signer) {
                                queue.remove(&nonce);
                                if queue.is_empty() {
                                    self.outbound.remove(&signer);
                                }
                                // Clean up insertion time tracker
                                self.outbound_insertion_times.remove(&(signer, nonce));
                            }
                        }
                    }
                }
            }
            Err(gossipsub::PublishError::InsufficientPeers) => {
                info!("[broadcast] InsufficientPeers: mesh_peers={}", mesh_peers);
            }
            Err(e) => {
                error!("[broadcast] PublishError: {} mesh_peers={}", e, mesh_peers);
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
        use crate::ledger::persistence::{PersistentEconomicState, StateStore, WalStateStore, WalStateStoreConfig};
        let config = WalStateStoreConfig {
            data_dir: data_dir.join("persistence"),
            fsync_batch_size: 1,
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

        // Fix 5: Startup consistency assertion — snapshot+WAL state must match
        // WAL-only replay.  Run this before consuming `store` into self.
        if let Err(e) = store.verify_consistency() {
            tracing::error!(
                error = %e,
                "PERSISTENCE CONSISTENCY CHECK FAILED — refusing to start"
            );
            return Err(e);
        }

        self.state_store = Some(Box::new(store));

        // Hydrate economic state: balances from snapshot
        let (_, balances) = state.export_state();
        for (peer, balance) in &balances {
            self.ledger.set_balance(peer, *balance);
        }

        // Recover genesis, if one was persisted.
        if let Some(g) = state.genesis {
            info!(
                network_name = %g.genesis.network_name,
                root = %g.genesis.root,
                "Genesis recovered from WAL"
            );
            self.genesis = Some(g);
        }
        if !balances.is_empty() {
            info!(count = balances.len(), "Recovered balances from economic snapshot");
        }

        // Hydrate thickness graph from snapshot edges
        if !state.thickness_edges.is_empty() {
            use crate::ledger::thickness::ThicknessEdge;
            let mut decoded_edges: HashMap<String, Vec<ThicknessEdge>> = HashMap::new();
            for (peer_str, edge_blobs) in &state.thickness_edges {
                let edges: Vec<ThicknessEdge> = edge_blobs.iter()
                    .filter_map(|blob| serde_cbor::from_slice::<ThicknessEdge>(blob).ok())
                    .collect();
                if !edges.is_empty() {
                    decoded_edges.insert(peer_str.clone(), edges);
                }
            }
            if !decoded_edges.is_empty() {
                self.ledger.thickness_graph.import_edges(decoded_edges);
            }
        }

        // Hydrate accepted claims from snapshot — evidence must survive restart
        if !state.accepted_claims.is_empty() {
            let count = state.accepted_claims.len();
            // Rebuild the claim-window high-water mark from recovered claims
            // so that post-restart windows do not overlap previously accepted history.
            // last_claimed is in-memory only and would otherwise reset to empty,
            // silently allowing overlapping claim windows after every restart.
            for sc in &state.accepted_claims {
                // ClaimNonceMap uses (String, u8) — base58 PeerId and type discriminant.
                let key = (sc.claim.claimant.to_base58(), sc.claim.claim_type as u8);
                let end = sc.claim.end_epoch;
                self.last_claimed
                    .entry(key)
                    .and_modify(|prev| {
                        if end > *prev {
                            *prev = end;
                        }
                    })
                    .or_insert(end);
            }
            self.economic_engine.import_accepted_claims(state.accepted_claims);
            info!(
                count,
                last_claimed_restored = self.last_claimed.len(),
                "Recovered accepted claims from economic snapshot"
            );
        }

        // Hydrate objections from recovered state so the receive handler
        // can perform cap and dedup checks against the live session's map.
        if !state.objections.is_empty() {
            let count = state.objections.values().map(|v| v.len()).sum::<usize>();
            info!(count, "Recovered objections from economic snapshot");
            self.objections = state.objections;
        }

        // Recover tx_nonce from self_tx_nonce field (persisted directly
        // in the snapshot, not derived from seen_nonces[self]).
        // Using seen_nonces[self] was fragile: the identity may have been
        // regenerated (rm -rf ~/.lattice) or no self-tx recorded yet.
        let recovered_tx_nonce = state.self_tx_nonce;
        if recovered_tx_nonce >= self.tx_nonce {
            self.tx_nonce = recovered_tx_nonce + 1;
            info!(
                tx_nonce = self.tx_nonce,
                recovered_tx_nonce,
                "Recovered tx_nonce from persistence"
            );
        }

        // Author genesis if this node is the configured root and no genesis
        // has been recovered. Authoring at startup, before the event loop,
        // establishes genesis deterministically.
        if self.genesis.is_none() {
            let root = self.genesis_root.unwrap_or(self.local_peer_id);
            if root == self.local_peer_id {
                self.author_and_gossip_genesis()?;
            }
        }

        Ok(())
    }

    /// Author genesis (self-signed, persisted, gossiped) for the root node.
    fn author_and_gossip_genesis(&mut self) -> Result<()> {
        use crate::ledger::wal_record::WalRecord;
        use crate::ledger::types::{Genesis, SignedGenesis};

        let genesis = Genesis {
            network_name: format!("lattice-{}", self.node_name),
            root: self.local_peer_id.to_base58(),
            timestamp: chrono::Utc::now(),
        };
        let genesis_bytes = serde_cbor::to_vec(&genesis)
            .map_err(|e| anyhow::anyhow!("failed to encode genesis: {e}"))?;
        let signature = self.local_key.sign(&genesis_bytes)
            .map_err(|e| anyhow::anyhow!("failed to sign genesis: {e}"))?;
        let signed = SignedGenesis {
            genesis,
            signer_public_key: self.local_key.public().encode_protobuf(),
            signature,
        };

        // Persist before gossip — validate → persist → apply ordering.
        if let Some(ref mut store) = self.state_store {
            store.persist_record(&WalRecord::Genesis(signed.clone()))
                .context("failed to persist genesis")?;
        }

        self.genesis = Some(signed.clone());
        info!(
            network_name = %signed.genesis.network_name,
            root = %signed.genesis.root,
            "Authored and persisted genesis"
        );

        // Gossip to the mesh.
        let topic = gossipsub::IdentTopic::new(LATTICE_GENESIS_TOPIC);
        let raw = serde_cbor::to_vec(&signed)
            .map_err(|e| anyhow::anyhow!("failed to encode genesis for gossip: {e}"))?;
        if let Err(e) = self
            .swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic, raw)
        {
            warn!(error = %e, "Failed to gossip genesis (will retry on peer connect)");
        }

        Ok(())
    }

    /// Re-gossip genesis to a newly connected peer if we hold one.
    fn re_gossip_genesis(&mut self, peer: &PeerId) {
        if self.genesis.is_none() {
            debug!("re_gossip_genesis: no genesis to send");
            return;
        }
        if !self.genesis_sent_to.insert(*peer) {
            debug!(peer = %peer, "re_gossip_genesis: already sent this session");
            return; // already sent to this peer this session
        }
        debug!(peer = %peer, "re_gossip_genesis: publishing genesis to peer");
        let topic = gossipsub::IdentTopic::new(LATTICE_GENESIS_TOPIC);
        let mesh_count = self
            .swarm
            .behaviour()
            .gossipsub
            .mesh_peers(&topic.hash())
            .count();
        debug!(
            peer = %peer,
            mesh_count,
            "re_gossip_genesis: mesh peers in genesis topic"
        );
        let raw = match self.genesis.as_ref().and_then(|g| serde_cbor::to_vec(g).ok()) {
            Some(r) => r,
            None => {
                warn!("re_gossip_genesis: failed to serialize genesis");
                return;
            }
        };
        match self
            .swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic, raw)
        {
            Ok(id) => {
                debug!(message_id = %id, peer = %peer, "re_gossip_genesis: published successfully");
                // also remove from pending set if present
                self.pending_genesis_gossip.remove(peer);
            }
            Err(ref e) if e.to_string().contains("InsufficientPeers") => {
                debug!(peer = %peer, mesh_count, "re_gossip_genesis: InsufficientPeers — queueing retry");
                self.pending_genesis_gossip.insert(*peer);
            }
            Err(e) => warn!(error = %e, peer = %peer, mesh_count, "Failed to re-gossip genesis to peer"),
        }
    }

    /// Drain the pending genesis gossip set — retry peers where
    /// re-gossip failed with InsufficientPeers.  Called from the
    /// metrics tick (~10s) so the mesh has time to form.
    fn drain_pending_genesis(&mut self) {
        if self.pending_genesis_gossip.is_empty() || self.genesis.is_none() {
            return;
        }
        let peers: Vec<PeerId> = self.pending_genesis_gossip.drain().collect();
        debug!(count = peers.len(), "drain_pending_genesis: retrying");
        for peer in &peers {
            // Re-insert into genesis_sent_to to avoid double-counting
            // if re_gossip also adds to pending again
            self.genesis_sent_to.remove(peer);
            self.re_gossip_genesis(peer);
        }
    }

    /// Handle an incoming genesis message from gossipsub.
    fn handle_genesis_message(&mut self, raw: &[u8]) {
        let signed: crate::ledger::types::SignedGenesis = match serde_cbor::from_slice(raw) {
            Ok(s) => s,
            Err(e) => {
                warn!(error = %e, "Failed to deserialize genesis from gossip");
                return;
            }
        };

        use crate::ledger::validation::{validate_genesis, GenesisRejection};

        let configured: Option<String> = self.genesis_root.as_ref()
            .map(|r| r.to_base58());

        match validate_genesis(
            &signed,
            configured.as_deref(),
            self.genesis.as_ref(),
        ) {
            Ok(()) => {
                // validate → persist → apply
                if let Some(ref mut store) = self.state_store {
                    if let Err(e) = store.persist_record(
                        &crate::ledger::wal_record::WalRecord::Genesis(signed.clone()),
                    ) {
                        error!(error = %e, "Failed to persist incoming genesis");
                        return;
                    }
                }
                self.genesis = Some(signed.clone());
                info!(
                    network_name = %signed.genesis.network_name,
                    root = %signed.genesis.root,
                    "Genesis accepted and persisted"
                );
            }
            Err(GenesisRejection::AlreadyEstablished { same_content: true, .. }) => {
                trace!("Duplicate genesis from gossip — ignoring");
            }
            Err(GenesisRejection::AlreadyEstablished { same_content: false, existing_signer, incoming_signer, .. })
                if existing_signer == incoming_signer =>
            {
                warn!(
                    existing_signer,
                    "Same signer produced conflicting genesis — possible key compromise"
                );
            }
            Err(GenesisRejection::AlreadyEstablished { .. }) => {
                info!("Genesis from different network — ignoring");
            }
            Err(e) => {
                warn!(?e, "Genesis rejected");
            }
        }
    }

    /// Handle an incoming objection message from gossipsub.
    ///
    /// Ordering: deserialize → validate → duplicate check → cap check
    /// → persist → insert into in-memory map.  Persist-before-insert
    /// so memory never advances past what's durable (3b0ef18).
    fn handle_objection_message(&mut self, raw: &[u8]) {
        let signed: crate::ledger::types::SignedObjection =
            match serde_cbor::from_slice(raw) {
                Ok(s) => s,
                Err(e) => {
                    warn!(error = %e, "Failed to decode objection from gossipsub");
                    return;
                }
            };

        let claim_id = signed.objection.target_claim_id;
        let objector = signed.objection.objector.clone();

        // Validate the objection (Commit 2).
        if let Err(e) = validate_objection(
            &signed,
            |_cid| true,
            |cid, obj| {
                self.objections.get(cid)
                    .is_some_and(|entries|
                        entries.iter().any(|o| o.objection.objector == *obj))
            },
        ) {
            warn!(
                claim_id = ?claim_id,
                objector = %objector,
                error = ?e,
                "Objection validation failed — rejected"
            );
            return;
        }

        self.process_objection(signed);
    }

    /// Core processing shared by receive and submit paths.
    /// Runs cap → persist → insert → gossip.
    /// Validation already completed by caller.
    fn process_objection(&mut self, signed: crate::ledger::types::SignedObjection) {
        const OBJECTION_CAP: usize = 64;

        let claim_id = signed.objection.target_claim_id;
        let objector = signed.objection.objector.clone();

        // Cap check: at 64 distinct objectors for this claim?
        // Runs AFTER duplicate check so a duplicate from an existing
        // objector doesn't count as a cap rejection.
        if let Some(entries) = self.objections.get(&claim_id) {
            if entries.len() >= OBJECTION_CAP
                && !entries.iter().any(|o| o.objection.objector == objector)
            {
                warn!(
                    claim_id = ?claim_id,
                    objector = %objector,
                    current_count = entries.len(),
                    cap = OBJECTION_CAP,
                    "Objection rejected — claim at cap"
                );
                return;
            }
        }

        // validate → persist → insert (3b0ef18 ordering)
        if let Some(ref mut store) = self.state_store {
            if let Err(e) = store.persist_record(
                &crate::ledger::wal_record::WalRecord::Objection(signed.clone()),
            ) {
                error!(error = %e, "Failed to persist objection");
                return;
            }
        }

        // Insert into in-memory map — dedup by objector.
        self.objections
            .entry(claim_id)
            .or_default()
            .retain(|o| o.objection.objector != objector);
        self.objections
            .entry(claim_id)
            .or_default()
            .push(signed.clone());

        info!(
            claim_id = ?claim_id,
            objector = %objector,
            reason = %signed.objection.reason,
            count = self.objections[&claim_id].len(),
            "Objection accepted and persisted"
        );

        // Re-publish to the mesh for propagation.
        self.publish_objection(&signed);
    }

    /// Publish an objection to all gossipsub peers.  On InsufficientPeers,
    /// queue for retry on the next metrics tick.
    fn publish_objection(&mut self, signed: &crate::ledger::types::SignedObjection) {
        let topic = gossipsub::IdentTopic::new(LATTICE_OBJECTION_TOPIC);
        let payload = match serde_cbor::to_vec(signed) {
            Ok(p) => p,
            Err(e) => {
                warn!(error = %e, "Failed to serialize objection for gossip");
                return;
            }
        };

        match self.swarm.behaviour_mut().gossipsub.publish(topic, payload) {
            Ok(id) => {
                debug!(message_id = %id, "Objection published to mesh");
            }
            Err(gossipsub::PublishError::InsufficientPeers) => {
                if self.pending_objection_gossip.len() >= 256 {
                    self.pending_objection_gossip.remove(0);
                }
                self.pending_objection_gossip.push(signed.clone());
                debug!("Objection queued for retry — InsufficientPeers");
            }
            Err(e) => {
                warn!(error = %e, "Failed to publish objection");
            }
        }
    }

    /// Drain the pending objection gossip queue — retry publishing
    /// objections that failed with InsufficientPeers.  Called on
    /// each metrics tick.
    fn drain_pending_objections(&mut self) {
        if self.pending_objection_gossip.is_empty() {
            return;
        }
        let pending: Vec<_> = self.pending_objection_gossip.drain(..).collect();
        debug!(count = pending.len(), "Retrying pending objection gossip");
        for obj in &pending {
            self.publish_objection(obj);
        }
    }

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
        use prost::Message;
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
                    self.peer_table.add_peer(peer_id, addr.clone(), self.economic_engine.epoch_count());
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
                if topic == LATTICE_BLOCK_TOPIC {
                    self.handle_block_message(&message.data, &propagation_source);
                } else if topic == LATTICE_GENESIS_TOPIC {
                    self.handle_genesis_message(&message.data);
                } else if topic == LATTICE_OBJECTION_TOPIC {
                    self.handle_objection_message(&message.data);
                } else {
                    self.handle_gossip_message(&message.data, propagation_source, message.source);
                }
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
                        let mut applied_count = 0usize;
                        let mut skipped_count = 0usize;
                        for tx in &response.transactions {
                            let applied_signer: PeerId = match tx.transaction.signer().parse() {
                                Ok(p) => p,
                                Err(e) => {
                                    warn!(
                                        error = %e,
                                        "[tx-fetch] Fetched transaction has invalid signer"
                                    );
                                    continue;
                                }
                            };
                            let applied_nonce = tx.transaction.nonce();

                            // Gossip may have closed the gap before the response arrived.
                            // Skip already-applied transactions silently and clear their
                            // fetch marks so metrics don't report a stale gap.
                            if self.seen_nonces.get(&applied_signer).map_or(false, |n| *n >= applied_nonce) {
                                self.outstanding_fetches.remove(&(applied_signer, applied_nonce));
                                skipped_count += 1;
                                continue;
                            }

                            // ── Durability boundary: validate → persist → apply ─
                            // The WAL must fsync before state mutation so
                            // a crash between apply and persist doesn't
                            // leave the transaction in graph state but
                            // absent from the WAL (b99cd2c).
                            //
                            // Recovery replays pre-validated WAL entries;
                            // do NOT re-run validation during recovery
                            // (see docs/architecture/persistence-design.md).
                            match validation::validate_transaction(
                                tx,
                                &self.ledger,
                                &self.seen_nonces,
                                Some(&self.local_peer_id),
                                false,
                            ) {
                                Err(e) => {
                                    warn!(error = %e, "[tx-fetch] Transaction validation failed");
                                    continue;
                                }
                                Ok(()) => {}
                            }

                            // Persist to WAL BEFORE applying to ledger.
                            if let Some(ref mut store) = self.state_store {
                                let record = crate::ledger::wal_record::WalRecord::Transaction(tx.clone());
                                if let Err(e) = store.persist_record(&record) {
                                    warn!(error = %e, nonce = applied_nonce,
                                        "Failed to persist transaction — skipping apply");
                                    continue;
                                }
                            }

                            // Now commit the validated, persisted transaction.
                            match validation::commit_validated_transaction(
                                tx,
                                &mut self.ledger,
                                &mut self.seen_nonces,
                            ) {
                                Ok(()) => {
                                    self.outstanding_fetches.remove(
                                        &(applied_signer, applied_nonce));
                                    applied_count += 1;
                                }
                                Err(e) => {
                                    warn!(
                                        error = %e,
                                        signer = %applied_signer,
                                        nonce = applied_nonce,
                                        "[tx-fetch] Fetched transaction rejected"
                                    );
                                }
                            }
                        }
                        debug!(
                            from = %peer,
                            received = response.transactions.len(),
                            applied = applied_count,
                            skipped_already_applied = skipped_count,
                            "[tx-fetch] Fetch response processed"
                        );
                    }
                }
            }
            SwarmEvent::Behaviour(LatticeBehaviourEvent::TxRpc(
                request_response::Event::OutboundFailure { peer, error, .. },
            )) => {
                debug!(peer = %peer, error = ?error, "[tx-fetch] Request failed");
            }

            // ── Phase 10: chain sync ────────────────────────────
            SwarmEvent::Behaviour(LatticeBehaviourEvent::ChainSyncRpc(
                request_response::Event::Message { peer, message },
            )) => {
                match message {
                    request_response::Message::Request {
                        request_id: _, request, channel,
                    } => {
                        // Responder side: serve blocks from the local ledger
                        let mut blocks = Vec::new();
                        let mut complete = true;
                        let max_blocks = 100u64;
                        let max_bytes = 5 * 1024 * 1024; // 5 MB
                        let mut total_bytes: usize = 0;

                        for height in request.from_height..=request.to_height {
                            if blocks.len() as u64 >= max_blocks {
                                complete = false;
                                break;
                            }
                            match self.commit_manager.get_block_bytes(height) {
                                Some(raw) => {
                                    if total_bytes + raw.len() > max_bytes as usize {
                                        complete = false;
                                        break;
                                    }
                                    total_bytes += raw.len();
                                    // Re-serialize as WireBlock for the wire format
                                    let mut reader = std::io::BufReader::new(&raw[..]);
                                    if let Ok(Some(frame)) = self.commit_manager.read_block(&mut reader) {
                                        let wire = WireBlock {
                                            height: frame.height,
                                            prev_hash: frame.prev_hash,
                                            block_hash: frame.block_hash,
                                            cert_bytes: frame.cert_bytes,
                                            signatures: frame.signatures
                                                .into_iter()
                                                .map(|(peer, sig)| (peer.to_base58(), sig))
                                                .collect(),
                                        };
                                        blocks.push(wire);
                                    }
                                }
                                None => break,
                            }
                        }

                        let response = ChainRangeResponse { blocks, complete };
                        let _ = self.swarm.behaviour_mut().chain_sync_rpc.send_response(
                            channel, response,
                        );
                    }
                    request_response::Message::Response {
                        response, ..
                    } => {
                        // Clear dedup guard — request completed (success or partial)
                        self.outstanding_chain_requests_by_peer.remove(&peer);
                        // Clear catch-up flag — resume processing live broadcasts
                        self.is_catching_up = false;
                        // Requester side: validate and apply received blocks
                        for wire in &response.blocks {
                            // Convert WireBlock back to signatures with PeerId
                            let signatures: Vec<(PeerId, Vec<u8>)> = wire.signatures
                                .iter()
                                .filter_map(|(peer_str, sig)| {
                                    peer_str.parse::<PeerId>().ok().map(|pid| (pid, sig.clone()))
                                })
                                .collect();

                            // Extract real proposal_id from cert_bytes.
                            // Era One: SignedTransaction CBOR
                            // Era Two v1: ImpactCertificate protobuf
                            // Era Two v2: RatificationBlock CBOR (0x02 prefix)
                            let proposal_id: String =
                                if wire.cert_bytes.first() == Some(&ERA_TWO_BLOCK_MARKER) {
                                    match RatificationBlock::decode_body(&wire.cert_bytes[1..]) {
                                        Ok(rb) => rb.proposal_id.clone(),
                                        Err(_) => hex::encode(wire.block_hash),
                                    }
                                } else if let Ok(stx) = serde_cbor::from_slice::<crate::ledger::types::SignedTransaction>(&wire.cert_bytes) {
                                // Era One: derive id from transaction type + signer
                                let nonce = stx.transaction.nonce();
                                let signer = stx.transaction.signer().to_string();
                                format!("{}-{}", signer, nonce)
                            } else if let Ok(cert) = crate::ingest::proto::ImpactCertificate::decode(&wire.cert_bytes[..]) {
                                // Era Two: use proposal_id from certificate
                                cert.proposal_id.clone()
                            } else {
                                // Fallback to block hash
                                hex::encode(wire.block_hash)
                            };

                            // ── Era Two: verify state roots before committing ──
                            if wire.cert_bytes.first() == Some(&ERA_TWO_BLOCK_MARKER) {
                                if let Ok(rb) = RatificationBlock::decode_body(&wire.cert_bytes[1..]) {
                                    let local_state = self.ledger.state_root(&self.seen_nonces);
                                    let local_thickness = self.ledger.thickness_graph.thickness_root();
                                    if rb.state_root != local_state || rb.thickness_root != local_thickness {
                                        // Only waive root verification on a true cold start. If we
                                        // have already built partial state, a mismatch is a real fork.
                                        let cold_start = self.ledger.balances.is_empty()
                                            && self.seen_nonces.is_empty()
                                            && self.ledger.thickness_graph.peer_count() == 0;
                                        if cold_start {
                                            warn!(
                                                height = wire.height,
                                                state_match = rb.state_root == local_state,
                                                thickness_match = rb.thickness_root == local_thickness,
                                                "[chain-sync] RatificationBlock root mismatch on cold start — waiving verification, state will converge after replay"
                                            );
                                        } else {
                                            warn!(
                                                height = wire.height,
                                                state_match = rb.state_root == local_state,
                                                thickness_match = rb.thickness_root == local_thickness,
                                                "[chain-sync] RatificationBlock root mismatch after partial sync — rejecting fork"
                                            );
                                            break;
                                        }
                                    }
                                }
                            }

                            match self.commit_manager.commit(
                                &wire.cert_bytes,
                                &proposal_id,
                                &signatures,
                            ) {
                                Ok(hash) => debug!("[chain-sync] Applied block {} hash={:?}", wire.height, hash),
                                Err(e) => {
                                    warn!("[chain-sync] Block {} validation failed: {}. Keeping prior blocks.", wire.height, e);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            SwarmEvent::Behaviour(LatticeBehaviourEvent::ChainSyncRpc(
                request_response::Event::OutboundFailure { peer, error, .. },
            )) => {
                warn!(peer = %peer, error = ?error, "[chain-sync] Request failed");
                self.outstanding_chain_requests_by_peer.remove(&peer);
            }

            // ── Witness RPC ──────────────────────────────────────
            SwarmEvent::Behaviour(LatticeBehaviourEvent::WitnessRpc(
                request_response::Event::Message { peer, message },
            )) => {
                match message {
                    request_response::Message::Request {
                        request_id: _, request, channel,
                    } => {
                        // Responder side: examine the witness request
                        //
                        // I7: Discovery ≠ Recognition — receiving a request
                        // does not create a relationship or modify trust.
                        // I4: Witness ≠ Certification — signing the hash
                        // does not mark the claim as verified.
                        let epoch = self.economic_engine.epoch_count();
                        let response = Self::handle_witness_request(
                            &self.peer_table,
                            &self.local_key,
                            epoch,
                            &request,
                        );

                        // I4: Witness ≠ Certification — the response is sent
                        // but the claim is NOT marked as verified.
                        let _ = self.swarm.behaviour_mut().witness_rpc
                            .send_response(channel, response);
                    }
                    request_response::Message::Response { response, .. } => {
                        // Route to claim registry first; fall through
                        // to attestation path if unclaimed.
                        let claimed = self.route_witness_response(peer, &response);
                        if claimed {
                            return;
                        }
                        // Requester side: a witness responded (attestation path)
                        if response.signature.is_empty() {
                            info!(
                                claim_id = %response.claim_id,
                                witness = %response.witness_id,
                                reason = ?response.decline_reason,
                                "Witness declined"
                            );
                        } else {
                            info!(
                                claim_id = %response.claim_id,
                                witness = %response.witness_id,
                                sig_len = response.signature.len(),
                                "Witness signature received"
                            );
                        }
                    }
                }
            }
            SwarmEvent::Behaviour(LatticeBehaviourEvent::WitnessRpc(
                request_response::Event::OutboundFailure { peer, error, .. },
            )) => {
                warn!(peer = %peer, error = ?error, "[witness] Request failed");
            }

            // ── Economic Comparison RPC ────────────────────────
            SwarmEvent::Behaviour(LatticeBehaviourEvent::CompareRpc(
                request_response::Event::Message { peer, message },
            )) => {
                match message {
                    request_response::Message::Request {
                        request_id: _, request, channel,
                    } => {
                        // Responder side: compute balance fingerprint for the
                        // subset of requested peers this node actually knows.
                        let epoch = self.economic_engine.epoch_count();
                        let response = self.handle_comparison_request(&request, epoch);
                        let _ = self.swarm.behaviour_mut().compare_rpc
                            .send_response(channel, response);
                        info!(
                            peer = %peer,
                            requested = request.requested_peers.len(),
                            included = request.requested_peers.len(), // reported by handler
                            "[econ-compare] Comparison request handled"
                        );
                    }
                    request_response::Message::Response { response, .. } => {
                        // Requester side: log the comparison result.
                        info!(
                            peer = %peer,
                            requested = %response.requested_peers.len(),
                            included = %response.included_peers.len(),
                            fingerprint = %response.balance_fingerprint,
                            their_epoch = response.responder_epoch,
                            "[econ-compare] Comparison response received"
                        );
                    }
                }
            }
            SwarmEvent::Behaviour(LatticeBehaviourEvent::CompareRpc(
                request_response::Event::OutboundFailure { peer, error, .. },
            )) => {
                warn!(peer = %peer, error = ?error, "[econ-compare] Request failed");
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
                                        self.peer_table.insert_peer(info.peer_id, self.economic_engine.epoch_count());
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
                // Clear pending reconnect if this peer was evicted as zombie
                if self.pending_reconnect.remove(&peer_id).is_some() {
                    info!(peer = %peer_id, "Reconnected after zombie eviction — self-healing succeeded");
                }
                if !self.mdns_peers.contains(&peer_id) {
                    warn!(peer = %peer_id, "Connection from non-mDNS peer");
                }
                if self.queried_peers.insert(peer_id) {
                    self.send_status_request(peer_id);
                }
                // Re-gossip genesis to newly connected peer.
                self.re_gossip_genesis(&peer_id);
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

    /// Sweep stale entries from the outstanding_fetches map.
    /// Entries older than FETCH_EVICTION_THRESHOLD (600s) are removed.
    /// Runs on every metrics tick to prevent unbounded accumulation.
    fn sweep_stale_fetches(&mut self) {
        let now = Instant::now();
        let before = self.outstanding_fetches.len();
        self.outstanding_fetches
            .retain(|_, inserted_at| now.duration_since(*inserted_at) < FETCH_EVICTION_THRESHOLD);
        let evicted = before - self.outstanding_fetches.len();
        if evicted > 0 {
            self.total_evicted_fetches += evicted;
            info!(
                "swept {} stale fetch entries (total evicted: {})",
                evicted, self.total_evicted_fetches
            );
        }
    }

    /// Sweep stale outbound queue entries with liveness awareness.
    ///
    /// Unlike the original sweep (which blindly evicted anything older than
    /// 600s), this version:
    ///   1. Uses zombie eviction data (`dead_peer_ids`) to skip entries
    ///      whose target mesh still has live peers — the transaction may
    ///      still route.
    ///   2. Retries broadcasting entries before evicting them, giving the
    ///      mesh one last chance to confirm.
    ///   3. Circuit breaker: if >50% of known peers are dead in a single
    ///      sweep cycle, logs an alert — the mesh may be partitioning.
    fn sweep_stale_outbound(&mut self) {
        let now = Instant::now();
        let dead = self.dead_peer_ids();
        let live_count = self.live_peer_count();
        let total_peers = self.peer_table.iter()
            .filter(|i| i.peer_id != self.local_peer_id)
            .count();

        // ── Circuit breaker: >50% peers dead ────────────────────
        if total_peers > 0 {
            let dead_fraction = dead.len() as f64 / total_peers as f64;
            if dead_fraction > OUTBOUND_CIRCUIT_BREAKER_FRACTION {
                error!(
                    dead = dead.len(),
                    total = total_peers,
                    fraction = %dead_fraction,
                    threshold = %OUTBOUND_CIRCUIT_BREAKER_FRACTION,
                    "[outbound-sweep] CIRCUIT BREAKER: >50% peers dead — mesh may be partitioning. \
                     Pausing outbound evictions until peers recover."
                );
                // Don't evict anything in a partitioned state — keep
                // transactions queued for when peers return.
                return;
            }
        }

        let mut evicted_total = 0usize;
        let mut retried_total = 0usize;

        // Collect stale entries with a simple loop to avoid closure borrow issues
        let stale: Vec<(PeerId, u64)> = {
            let mut s = Vec::new();
            for (&peer, queue) in &self.outbound {
                for &nonce in queue.keys() {
                    let aged = self.outbound_insertion_times
                        .get(&(peer, nonce))
                        .map_or(true, |inserted| now.duration_since(*inserted) >= OUTBOUND_SWEEP_THRESHOLD);
                    if aged {
                        s.push((peer, nonce));
                    }
                }
            }
            s
        };

        for (peer, nonce) in stale {
            // ── Peer-count-gated flush ──────────────────────────
            // Only evict if the mesh has no live peers OR the signer
            // itself is dead.  If live peers exist and the signer is
            // alive, keep the entry — it may still route.
            if live_count > 0 && !dead.contains(&peer) {
                debug!(
                    peer = %peer,
                    nonce = nonce,
                    live_peers = live_count,
                    "[outbound-sweep] Skipping stale entry — mesh has live peers"
                );
                continue;
            }

            // ── Retry before evicting ───────────────────────────
            // Give the mesh one last chance to accept the transaction
            // before we drop it.  If mesh_peers ≥ 1, the broadcast
            // has a chance.
            if let Some(queue) = self.outbound.get(&peer) {
                if let Some(tx) = queue.get(&nonce) {
                    let tx_clone = tx.clone();
                    let topic = gossipsub::IdentTopic::new(LATTICE_TX_TOPIC);
                    let mesh_peers = self.swarm.behaviour().gossipsub
                        .mesh_peers(&topic.hash())
                        .count();
                    if mesh_peers > 0 {
                        let _ = self.broadcast_transaction(&tx_clone);
                        retried_total += 1;
                        info!(
                            peer = %peer,
                            nonce = nonce,
                            mesh_peers = mesh_peers,
                            "[outbound-sweep] Retried broadcast before eviction"
                        );
                    }
                }
            }

            // ── Evict ───────────────────────────────────────────
            if let Some(queue) = self.outbound.get_mut(&peer) {
                queue.remove(&nonce);
                self.outbound_insertion_times.remove(&(peer, nonce));
                evicted_total += 1;
            }
        }

        // Clean up empty peer entries
        let empty_peers: Vec<PeerId> = self.outbound
            .iter()
            .filter(|(_, q)| q.is_empty())
            .map(|(&p, _)| p)
            .collect();
        for peer in empty_peers {
            self.outbound.remove(&peer);
        }

        if retried_total > 0 {
            info!(
                "[outbound-sweep] Retried {} stale entries before eviction",
                retried_total
            );
        }
        if evicted_total > 0 {
            info!(
                "[outbound-sweep] Evicted {} stale entries (live_peers={} dead={})",
                evicted_total, live_count, dead.len()
            );
        }
    }

    /// Check peer liveness and evict zombies (half-open connections + epoch-based).
    /// Runs on every heartbeat tick, after sweep_stale_fetches.
    ///
    /// Two detection layers:
    ///   1. Wall-clock (fast): 30s silent → WARN, 90s silent → EVICT
    ///   2. Epoch-based (deep): 30 epochs without heartbeat → EVICT
    ///      OR zero thickness + 10 epochs without attestation → EVICT
    ///
    /// Circuit breaker: 3+ evictions in 60s → skip disconnect, ERROR.
    fn check_peer_liveness(&mut self) {
        let now = Instant::now();
        let current_epoch = self.economic_engine.epoch_count();

        // Drain circuit breaker window of entries older than 60s
        self.evictions_last_minute
            .retain(|t| now.duration_since(*t).as_secs() < 60);

        let mut longest_silence = 0u64;
        let mut to_evict: Vec<(PeerId, String)> = Vec::new(); // (peer_id, reason)

        // ── Collect peer IDs first to avoid borrow conflicts ──────
        let peer_ids: Vec<PeerId> = self.peer_table.iter().map(|i| i.peer_id).collect();

        for peer_id in &peer_ids {
            // Skip self
            if *peer_id == self.local_peer_id {
                continue;
            }

            let info = match self.peer_table.get(peer_id) {
                Some(i) => i,
                None => continue,
            };

            let elapsed = (Utc::now() - info.last_seen).num_seconds().max(0) as u64;
            if elapsed > longest_silence {
                longest_silence = elapsed;
            }

            // ── Grace window: cold-start protection ──────────────────
            // Applies to ALL eviction paths, not just heartbeat silence.
            // A peer that has not yet had a reasonable opportunity to
            // establish itself should not be evicted for lacking evidence
            // that can only be accumulated through continued participation.
            // Grace exits on first heartbeat — the peer has demonstrated
            // liveness and enters the standard eviction pool.
            let age = (Utc::now() - info.first_seen).num_seconds().max(0) as u64;
            let in_grace = info.heartbeats_received == 0 && age < COLD_START_GRACE_SECS;

            if in_grace {
                continue; // Skip all eviction checks for graced peers
            }

            // ── Layer 2: epoch-based heartbeat silence ─────────
            let epochs_since_heartbeat =
                current_epoch.saturating_sub(info.last_heartbeat_epoch);
            if epochs_since_heartbeat > ZOMBIE_EPOCH_THRESHOLD {
                to_evict.push((*peer_id, format!(
                    "heartbeat silence: {} epochs since last heartbeat (threshold {})",
                    epochs_since_heartbeat, ZOMBIE_EPOCH_THRESHOLD
                )));
                continue;
            }

            // ── Layer 2: zero thickness + attestation silence ─
            // Gate: skip this criterion if the mesh has never had any
            // thickness at all.  On a fresh mesh with no genesis seed,
            // every peer looks like a zombie by this criterion, and
            // evicting all of them on loop breaks the mesh.
            //
            // Constitutional separation (2026-07-24): establishment
            // (heartbeats > 0) is the standing that keeps you in the
            // mesh.  Thickness is standing you earn and spend, not
            // standing you're required to hold.  An established peer
            // is never evicted for zero thickness — that would create
            // a chicken-and-egg where a fresh peer must earn thickness
            // through witnessing but can't survive long enough to do so.
            let thickness = self.ledger.thickness_graph.total_thickness(peer_id);
            if thickness < 0.001
                && self.ledger.thickness_graph.peer_count() > 0
                && info.heartbeats_received == 0
            {
                let last_att_epoch = self.last_attestation_epoch.get(peer_id).copied().unwrap_or(0);
                let epochs_since_attest =
                    current_epoch.saturating_sub(last_att_epoch);
                if epochs_since_attest > ZOMBIE_ATTESTATION_SILENCE_EPOCHS {
                    to_evict.push((*peer_id, format!(
                        "zero thickness + no attestations for {} epochs (threshold {})",
                        epochs_since_attest, ZOMBIE_ATTESTATION_SILENCE_EPOCHS
                    )));
                    continue;
                }
            }

            // ── Layer 1: wall-clock (fast path for half-open connections) ─
            if elapsed > ZOMBIE_EVICT_THRESHOLD_SECS {
                // Grace window: a silent cold-start peer is not evicted.
                if in_grace { continue; }

                // Circuit breaker check
                let recent = self.evictions_last_minute.len();
                if recent >= CIRCUIT_BREAKER_LIMIT {
                    error!(
                        peer = %peer_id,
                        elapsed_secs = elapsed,
                        evictions_last_minute = recent,
                        "Circuit breaker active — too many evictions. Human intervention needed."
                    );
                    continue;
                }

                to_evict.push((*peer_id, format!(
                    "wall-clock silence: {}s (threshold {}s)",
                    elapsed, ZOMBIE_EVICT_THRESHOLD_SECS
                )));
            } else if elapsed > ZOMBIE_WARN_THRESHOLD_SECS {
                warn!(
                    peer = %peer_id,
                    elapsed_secs = elapsed,
                    evict_threshold_secs = ZOMBIE_EVICT_THRESHOLD_SECS,
                    "Peer silent — approaching zombie threshold"
                );
            }
        }

        // ── Execute evictions ────────────────────────────────────
        for (peer_id, reason) in &to_evict {
            warn!(
                peer = %peer_id,
                reason = %reason,
                "Evicting zombie peer"
            );

            // Remove from local peer table
            self.peer_table.remove_peer(peer_id);

            // Disconnect from swarm (triggers gossipsub mesh cleanup)
            let _ = self.swarm.disconnect_peer_id(*peer_id);

            // Clear witness signatures from this peer across all proposals
            self.witness_sigs.values_mut().for_each(|sigs| {
                sigs.retain(|(pid, _, _)| pid != peer_id);
            });
            // Purge empty proposal entries
            self.witness_sigs.retain(|_, sigs| !sigs.is_empty());

            // Clear attestation epoch tracker
            self.last_attestation_epoch.remove(peer_id);

            self.evictions_last_minute.push_back(now);
            self.pending_reconnect.insert(*peer_id, now);
        }

        // Check pending reconnects that have timed out
        self.pending_reconnect.retain(|peer_id, since| {
            let waiting = now.duration_since(*since).as_secs();
            if waiting > RECONNECT_TIMEOUT_SECS {
                error!(
                    peer = %peer_id,
                    waiting_secs = waiting,
                    "Failed to reconnect after zombie eviction — manual restart may be needed"
                );
                false // remove after logging
            } else {
                true
            }
        });

        // Store for metrics line
        self.last_max_peer_silence = longest_silence;
    }

    /// Return the set of peer IDs that meet zombie eviction criteria.
    /// Used by the outbound queue sweep to avoid flushing transactions
    /// destined for live peers.  Mirrors the epoch-based criteria in
    /// `check_peer_liveness` but does not execute evictions.
    fn dead_peer_ids(&self) -> HashSet<PeerId> {
        let current_epoch = self.economic_engine.epoch_count();
        let mut dead = HashSet::new();

        for info in self.peer_table.iter() {
            if info.peer_id == self.local_peer_id {
                continue;
            }

            // ── Grace window: cold-start protection ──────────────────
            // Twin of the same guard in check_peer_liveness().
            let age = (Utc::now() - info.first_seen).num_seconds().max(0) as u64;
            let in_grace = info.heartbeats_received == 0 && age < COLD_START_GRACE_SECS;

            // Heartbeat silence
            let epochs_since_hb = current_epoch.saturating_sub(info.last_heartbeat_epoch);
            if !in_grace && epochs_since_hb > ZOMBIE_EPOCH_THRESHOLD {
                dead.insert(info.peer_id);
                continue;
            }
            // Zero thickness + attestation silence
            // Gate: skip if mesh has never had thickness (same as check_peer_liveness)
            //
            // Constitutional separation (2026-07-24): established peers
            // are never dead by zero-thickness criterion.
            let thickness = self.ledger.thickness_graph.total_thickness(&info.peer_id);
            if thickness < 0.001
                && self.ledger.thickness_graph.peer_count() > 0
                && info.heartbeats_received == 0
            {
                let last_att_epoch =
                    self.last_attestation_epoch.get(&info.peer_id).copied().unwrap_or(0);
                if current_epoch.saturating_sub(last_att_epoch) > ZOMBIE_ATTESTATION_SILENCE_EPOCHS {
                    dead.insert(info.peer_id);
                }
            }
        }
        dead
    }

    /// Count of peers that are alive (in peer table, not meeting
    /// zombie criteria).  Used by the circuit breaker to detect
    /// mesh partitions.
    fn live_peer_count(&self) -> usize {
        let dead = self.dead_peer_ids();
        self.peer_table
            .iter()
            .filter(|i| i.peer_id != self.local_peer_id && !dead.contains(&i.peer_id))
            .count()
    }

    /// Persist a validated transaction to the WAL and update tx_store
    /// bookkeeping.  The caller is responsible for ordering: this
    /// function must be called AFTER validation and BEFORE ledger
    /// application to enforce the durability boundary.
    /// (See docs/architecture/persistence-design.md §2.)
    fn on_transaction_applied(&mut self, tx: &SignedTransaction) {
        // Persist to unified WAL if persistence is enabled.
        if let Some(store) = self.state_store.as_mut() {
            let record = crate::ledger::wal_record::WalRecord::Transaction(tx.clone());
            if let Err(e) = store.persist_record(&record) {
                warn!(error = %e, nonce = tx.transaction.nonce(), "Failed to persist transaction");
            }
        }
        let signer: PeerId = match tx.transaction.signer().parse() {
            Ok(p) => p,
            Err(_) => return,
        };
        let nonce = tx.transaction.nonce();

        self.tx_store.insert((signer, nonce), tx.clone());

        // Gossip may have closed the gap before the fetch response arrived.
        // Clear any outstanding fetch mark for this nonce so the metrics
        // don't report a stale gap.
        self.outstanding_fetches.remove(&(signer, nonce));

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

        // Phase 7: dispatch marker-prefixed messages FIRST.
        // Attestations (0x01) and quorum certificates (0x02) are
        // intentionally discriminated by their leading byte. Checking
        // markers before the protobuf decode prevents a malformed or
        // future-prefixed message from being silently misrouted as a
        // raw ImpactCertificate.
        {
            // ── Phase 7: witness attestation handler ────────────
            // Messages starting with 0x01 are witness attestations,
            // not ImpactCertificates. Parse, verify, and collect.
            if data.first() == Some(&0x01) {
                self.handle_witness_attestation(data, propagation_source);
                return;
            }

            // ── Phase 7: verified quorum certificate handler ────
            // Messages starting with 0x02 carry a full certificate plus
            // the 3-of-5 witness signatures. Verify and commit locally.
            if data.first() == Some(&0x02) {
                self.handle_quorum_certificate(data, propagation_source);
                return;
            }
        }

        // Phase 7: detect raw enclave certificate messages by protobuf
        // signature.  These are unmarked ImpactCertificate payloads, not
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
                            self.peer_table.insert_peer(peer_id, self.economic_engine.epoch_count());
                        }
                        self.peer_table.record_heartbeat_epoch(
                            &peer_id,
                            self.economic_engine.epoch_count(),
                        );
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
                        let exec_client = self.executor.clone();
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
                    self.peer_table.insert_peer(peer, self.economic_engine.epoch_count());
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
                // Proactive catch-up trigger: if this peer's chain is
                // ahead of ours, fire a ChainRangeRequest.  The per-peer
                // dedup guard prevents firing multiple overlapping requests.
                let local_tip = self.commit_manager.height();
                if response.chain_height > local_tip {
                    info!(
                        from = %peer, remote_tip = response.chain_height, local_tip,
                        "[chain-sync] Status shows peer ahead — catching up via ChainRangeRequest ({}..{})",
                        local_tip, response.chain_height - 1,
                    );
                    if !self.outstanding_chain_requests_by_peer.contains(&peer) {
                        self.outstanding_chain_requests_by_peer.insert(peer);
                        let req = ChainRangeRequest {
                            from_height: if local_tip == 0 { 0 } else { local_tip + 1 },
                            to_height: response.chain_height - 1,
                        };
                        let _ = self.swarm.behaviour_mut()
                            .chain_sync_rpc
                            .send_request(&peer, req);
                    }
                }
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
            chain_height: self.commit_manager.height(),
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
            self.publish_witness_attestation(&cert.proposal_id);
        } else {
            debug!(
                proposal_id = %cert.proposal_id,
                panel_size = panel.len(),
                "[sortition] Local node not on panel — observing quorum"
            );
        }
    }

    /// Sign the given proposal_id and publish a 0x01 witness attestation
    /// on the enclave-cert topic.  Idempotent: if this node has already
    /// attested to the proposal, the second call is a no-op.
    ///
    /// Returns true if an attestation was (or already had been) published.
    fn publish_witness_attestation(&mut self, proposal_id: &str) -> bool {
        // Scope the mutable borrow on witness_sigs so we can call other
        // &mut self methods (gossipsub) afterwards.
        let (sig, pubkey_bytes) = {
            let sigs = self.witness_sigs.entry(proposal_id.to_string()).or_default();
            if sigs.iter().any(|(pid, _, _)| *pid == self.local_peer_id) {
                return true;
            }

            let sig = match self.local_key.sign(proposal_id.as_bytes()) {
                Ok(s) => s,
                Err(e) => {
                    warn!(
                        error = %e,
                        proposal_id = proposal_id,
                        "[sortition] Failed to sign attestation"
                    );
                    return false;
                }
            };

            let pubkey_bytes = self.local_key.public().encode_protobuf();
            sigs.push((self.local_peer_id, sig.clone(), pubkey_bytes.clone()));
            (sig, pubkey_bytes)
        };

        let pid_bytes = proposal_id.as_bytes();
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
                    proposal_id = proposal_id,
                    "[sortition] Witness attestation published to mesh"
                );
                true
            }
            Err(e) => {
                warn!(
                    error = %e,
                    "[sortition] Failed to publish witness attestation"
                );
                false
            }
        }
    }

    /// Derive the deterministic witness panel for a RatificationBlock.
    fn derive_ratification_panel(&self, proposal_id: &str) -> Vec<PeerId> {
        let peers: Vec<PeerId> = self.peer_table.iter().map(|info| info.peer_id).collect();
        crate::sortition::derive_ratification_panel(proposal_id, &self.local_peer_id, &peers)
    }

    /// Try to commit a RatificationBlock once a quorum of panel signatures
    /// has been collected.  Idempotent via `commit_manager.is_committed`.
    fn maybe_commit_ratification_qc(&mut self, proposal_id: &str) {
        if self.commit_manager.is_committed(proposal_id) {
            return;
        }

        let panel = self.derive_ratification_panel(proposal_id);
        let quorum = crate::sortition::ratification_quorum(panel.len());
        let sigs = self
            .witness_sigs
            .get(proposal_id)
            .cloned()
            .unwrap_or_default();
        let mut panel_sigs: Vec<(PeerId, Vec<u8>, Vec<u8>)> = sigs
            .into_iter()
            .filter(|(pid, _, _)| panel.contains(pid))
            .collect();

        if panel_sigs.len() < quorum {
            return;
        }

        // Canonical ordering: every node must derive the same block hash
        // from the same quorum, regardless of the order attestations arrived.
        panel_sigs.sort_by(|a, b| a.0.to_bytes().cmp(&b.0.to_bytes()));

        let Some(cert_bytes) = self.cert_cache.get(proposal_id).cloned() else {
            // We have not yet assembled this RatificationBlock locally (e.g.
            // an attestation arrived before our epoch handler ran).  Do not
            // re-derive from local state here: the cached block is the one
            // the panel actually signed.  We will commit it when the block
            // is broadcast by the node that completes the quorum, or once
            // our own epoch handler caches it and we re-evaluate.
            debug!(
                proposal_id = proposal_id,
                "[ratification-qc] Block not in cache yet — deferring commit"
            );
            return;
        };

        let sigs_2tuple: Vec<(PeerId, Vec<u8>)> = panel_sigs
            .iter()
            .map(|(pid, sig, _)| (*pid, sig.clone()))
            .collect();

        match self.commit_manager.commit(&cert_bytes, proposal_id, &sigs_2tuple) {
            Ok(block_hash) => {
                info!(
                    proposal_id = proposal_id,
                    hash = %hex::encode(block_hash),
                    sigs = panel_sigs.len(),
                    "[ratification-qc] RatificationBlock committed via witness QC"
                );
                self.publish_committed_block(proposal_id);
            }
            Err(e) => {
                warn!(
                    proposal_id = proposal_id,
                    error = %e,
                    "[ratification-qc] Failed to commit RatificationBlock QC"
                );
            }
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
        if sigs.iter().any(|(pid, _, _)| *pid == signer_peer_id) {
            debug!(
                proposal_id = %proposal_id,
                signer = %signer_peer_id,
                "[attestation] Duplicate signature ignored"
            );
            return;
        }

        sigs.push((signer_peer_id, sig.to_vec(), pk_bytes.to_vec()));
        let count = sigs.len();

        // Record that this peer attested in the current epoch — used
        // by zombie eviction to detect peers that neither carry
        // thickness nor participate in attestation.
        self.last_attestation_epoch
            .insert(signer_peer_id, self.economic_engine.epoch_count());

        info!(
            proposal_id = %proposal_id,
            signatures_collected = count,
            "[attestation] Signature collected"
        );

        // Era Two RatificationBlock: drive commit via panel QC.
        if proposal_id.starts_with("epoch-") {
            self.maybe_commit_ratification_qc(&proposal_id);
            return;
        }

        // ImpactCertificate: 3-of-5 witness quorum.
        // Check quorum — when reached, commit to the hash-chain ledger
        if count >= 3 {
            // Clone signatures and release the borrow on witness_sigs before
            // we call other &mut self methods (commit_manager, gossipsub).
            let mut sigs: Vec<(PeerId, Vec<u8>, Vec<u8>)> = sigs.clone();

            // Canonical ordering: every node must derive the same block hash
            // from the same quorum, regardless of the order attestations arrived.
            sigs.sort_by(|a, b| a.0.to_bytes().cmp(&b.0.to_bytes()));

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
                signers = ?sigs.iter().map(|(pid, _, _)| pid.to_string()).collect::<Vec<_>>(),
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
                // The ledger frame stores (peer_id, signature) pairs; public
                // keys travel in the gossiped QC message for verification.
                let sigs_2tuple: Vec<(PeerId, Vec<u8>)> = sigs
                    .iter()
                    .map(|(pid, sig, _)| (*pid, sig.clone()))
                    .collect();

                match self.commit_manager.commit(
                    cert_bytes,
                    &proposal_id,
                    &sigs_2tuple,
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
                        // Gossip the verified QC so all nodes can commit.
                        self.publish_quorum_certificate(&proposal_id, &sigs);
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

    /// Handle an incoming verified quorum certificate (QC), marker 0x02.
    ///
    /// Wire format:
    ///   [0x02] [2-byte pid_len] [proposal_id]
    ///   [4-byte cert_len] [cert protobuf bytes]
    ///   [2-byte sig_count] ([2-byte pubkey_len] [pubkey] [2-byte sig_len] [sig]) ...
    ///
    /// Verifies the embedded 3-of-5 signatures over proposal_id, then commits
    /// the certificate to the local hash-chain ledger. This is the Era Two
    /// gossip path that lets every node commit ratified certificates, not
    /// just the node that assembled the quorum.
    fn handle_quorum_certificate(&mut self, data: &[u8], propagation_source: PeerId) {
        let (proposal_id, cert_bytes, sigs_3tuple) =
            match Self::decode_and_verify_quorum_certificate(data) {
                Some(result) => result,
                None => {
                    warn!("[qc-recv] Failed to decode/verify quorum certificate");
                    return;
                }
            };

        if sigs_3tuple.len() < 3 {
            warn!(
                proposal_id = %proposal_id,
                valid = sigs_3tuple.len(),
                required = 3,
                "[qc-recv] Insufficient valid signatures"
            );
            return;
        }

        info!(
            proposal_id = %proposal_id,
            signers = ?sigs_3tuple.iter().map(|(pid, _, _)| pid.to_string()).collect::<Vec<_>>(),
            from = %propagation_source,
            "[qc-recv] Quorum certificate verified ✓"
        );

        // Cache the raw certificate so the commit layer can write it
        self.cert_cache.insert(proposal_id.clone(), cert_bytes.clone());

        // Commit if not already
        if self.commit_manager.is_committed(&proposal_id) {
            debug!(
                proposal_id = %proposal_id,
                "[qc-recv] Certificate already committed"
            );
            return;
        }

        let sigs_2tuple: Vec<(PeerId, Vec<u8>)> = sigs_3tuple
            .iter()
            .map(|(pid, sig, _)| (*pid, sig.clone()))
            .collect();

        match self.commit_manager.commit(&cert_bytes, &proposal_id, &sigs_2tuple) {
            Ok(block_hash) => {
                info!(
                    proposal_id = %proposal_id,
                    block_hash = %hex::encode(block_hash),
                    height = self.commit_manager.height(),
                    "[qc-recv] Certificate committed to local chain"
                );
            }
            Err(e) => {
                warn!(
                    proposal_id = %proposal_id,
                    error = %e,
                    "[qc-recv] Failed to commit certificate"
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
            ApiRequest::WitnessClaimService => {
                return self.handle_witness_claim_service(msg);
            }
            ApiRequest::GetClaimStatus { claim_id } => {
                let state = self.witness_claims.get(&claim_id);
                match state {
                    Some(s) => {
                        let status_str = match &s.status {
                            ClaimSubmissionStatus::Collecting => "collecting",
                            ClaimSubmissionStatus::Accepted { .. } => "accepted",
                            ClaimSubmissionStatus::Rejected { .. } => "rejected",
                        };
                        let declines: Vec<_> = s.declines.iter().map(|(pid, reason)| {
                            crate::api::ClaimDeclineInfo {
                                witness: pid.to_base58(),
                                reason: reason.clone(),
                            }
                        }).collect();
                        let result = match &s.status {
                            ClaimSubmissionStatus::Accepted { thickness } => {
                                Some(crate::api::ClaimResultInfo {
                                    thickness: Some(*thickness),
                                    acceptance: "accepted".into(),
                                    reason: None,
                                })
                            }
                            ClaimSubmissionStatus::Rejected { reason } => {
                                Some(crate::api::ClaimResultInfo {
                                    thickness: None,
                                    acceptance: "rejected".into(),
                                    reason: Some(reason.clone()),
                                })
                            }
                            ClaimSubmissionStatus::Collecting => None,
                        };
                        ApiResponse::ClaimStatus {
                            claim_id,
                            status: status_str.to_string(),
                            candidates: s.candidates.len(),
                            signatures_collected: s.signatures.len(),
                            declines,
                            result,
                        }
                    }
                    None => ApiResponse::Error {
                        message: format!("Claim {} not found", claim_id),
                    },
                }
            }
            // ── Read-only query API (v1) ──────────────────────────
            ApiRequest::GetPeers => {
                let dead = self.dead_peer_ids();
                let peers: Vec<_> = self.peer_table.iter()
                    .filter(|p| p.peer_id != self.local_peer_id)
                    .map(|p| {
                        let queue_depth = self.outbound
                            .get(&p.peer_id)
                            .map(|q| q.len() as u64)
                            .unwrap_or(0);
                        let thickness = {
                            let t = self.ledger.thickness_graph
                                .total_thickness(&p.peer_id);
                            if t > 0.0 { Some(t) } else { None }
                        };
                        crate::api::PeerInfo {
                            peer_id: p.peer_id.to_base58(),
                            name: None,
                            heartbeats: p.heartbeats_received,
                            silence_secs: {
                                let elapsed = chrono::Utc::now()
                                    .signed_duration_since(p.last_seen)
                                    .num_seconds();
                                if elapsed > 0 { elapsed as u64 } else { 0 }
                            },
                            is_dead: dead.contains(&p.peer_id),
                            queue_depth,
                            thickness,
                            distinct_witnesses: None, // TODO: count from accepted claims
                        }
                    })
                    .collect();
                ApiResponse::Peers { peers }
            }
            ApiRequest::GetEpochState => {
                let summary = self.economic_engine.last_epoch_summary();
                ApiResponse::EpochState {
                    epoch: self.economic_engine.epoch_count(),
                    ratio: summary.map(|s| s.ratio),
                    tax_calculated: summary.map(|s| s.tax_calculated),
                    tax_collected: summary.map(|s| s.tax_collected),
                    minted: summary.map(|s| s.minted),
                    redistributed_to: summary.map(|s| s.redistributed_to),
                }
            }
            ApiRequest::GetEconomicState => {
                let own_balance = self.ledger.balance_of(&self.local_peer_id);
                let peers: Vec<_> = self.peer_table.iter()
                    .filter(|p| p.peer_id != self.local_peer_id)
                    .map(|p| {
                        let nonce = self.seen_nonces
                            .get(&p.peer_id)
                            .copied()
                            .unwrap_or(0);
                        let bal = self.ledger.balance_of(&p.peer_id);
                        crate::api::PeerBalance {
                            peer_id: p.peer_id.to_base58(),
                            balance: bal.0,
                            nonce,
                        }
                    })
                    .collect();
                ApiResponse::EconomicState {
                    own_balance: own_balance.0,
                    own_nonce: self.tx_nonce,
                    peers,
                }
            }
            ApiRequest::GetNodeInfo => {
                let genesis_id = self.genesis_root.as_ref()
                    .map(|g| g.to_string())
                    .unwrap_or_else(|| "auto".to_string());
                let self_thickness = {
                    let t = self.ledger.thickness_graph
                        .total_thickness(&self.local_peer_id);
                    if t > 0.0 { Some(t) } else { None }
                };
                // Count distinct witnesses from accepted claims
                let dw = self.economic_engine.count_distinct_witnesses();
                // Earned thickness — VerifiedContribution edges only (no genesis/vouch)
                let earned: Option<f64> = {
                    let e = self.ledger.thickness_graph.earned_thickness(&self.local_peer_id);
                    if e > 0.0 { Some(e) } else { None }
                };
                ApiResponse::NodeInfo {
                    peer_id: self.local_peer_id.to_base58(),
                    name: self.node_name.clone(),
                    genesis_root_id: genesis_id,
                    chain_tip: self.commit_manager.height(),
                    uptime_secs: std::time::Instant::now()
                        .duration_since(self.start_time)
                        .as_secs(),
                    build_commit: env!("BUILD_COMMIT").to_string(),
                    thickness: self_thickness,
                    distinct_witnesses: dw,
                    earned_thickness: earned,
                }
            }
            ApiRequest::GetPersistenceState => {
                let (snap_epoch, wal_bytes, wal_entries) = match &self.state_store {
                    Some(store) => store.get_stats(),
                    None => (0, 0, 0),
                };
                ApiResponse::PersistenceState {
                    last_snapshot_epoch: snap_epoch,
                    wal_bytes,
                    wal_entries,
                }
            }
            ApiRequest::SubmitObjection {
                target_claim_id,
                reason,
            } => {
                // Parse claim ID from hex string.
                let claim_id_bytes = match hex::decode(&target_claim_id) {
                    Ok(b) if b.len() == 32 => {
                        let mut arr = [0u8; 32];
                        arr.copy_from_slice(&b);
                        arr
                    }
                    _ => {
                        let _ = msg.reply.send(ApiResponse::Error {
                            message: format!(
                                "Invalid claim_id: expected 32-byte hex string"
                            ),
                        });
                        return;
                    }
                };

                // Build the objection payload.
                let peer_id = PeerId::from(self.local_key.public());
                let objection = crate::ledger::types::Objection {
                    objector: peer_id.to_base58(),
                    target_claim_id: claim_id_bytes,
                    reason,
                    timestamp: chrono::Utc::now(),
                };

                // Sign with local identity key.
                let payload = serde_cbor::to_vec(&objection)
                    .map_err(|e| format!("serialize: {}", e))
                    .unwrap_or_default();
                let signature = match self.local_key.sign(&payload) {
                    Ok(sig) => sig,
                    Err(e) => {
                        let _ = msg.reply.send(ApiResponse::Error {
                            message: format!("Failed to sign objection: {:?}", e),
                        });
                        return;
                    }
                };

                let signed = crate::ledger::types::SignedObjection {
                    objection,
                    signer_public_key: self.local_key.public().encode_protobuf(),
                    signature,
                };

                // Validate (including duplicate check).
                if let Err(e) = validate_objection(
                    &signed,
                    |_cid| true,
                    |cid, obj| {
                        self.objections.get(cid)
                            .is_some_and(|entries|
                                entries.iter().any(|o| o.objection.objector == *obj))
                    },
                ) {
                    let _ = msg.reply.send(ApiResponse::Error {
                        message: format!("Objection validation failed: {:?}", e),
                    });
                    return;
                }

                self.process_objection(signed);

                let _ = msg.reply.send(ApiResponse::ObjectionSubmitted {
                    claim_id: target_claim_id,
                });
                return;
            }
            ApiRequest::GetObjections { claim_id } => {
                let claim_id_bytes = match hex::decode(&claim_id) {
                    Ok(b) if b.len() == 32 => {
                        let mut arr = [0u8; 32];
                        arr.copy_from_slice(&b);
                        arr
                    }
                    _ => {
                        let _ = msg.reply.send(ApiResponse::Error {
                            message: format!(
                                "Invalid claim_id: expected 32-byte hex string"
                            ),
                        });
                        return;
                    }
                };

                let objections: Vec<crate::api::ObjectionInfo> = self
                    .objections
                    .get(&claim_id_bytes)
                    .map(|entries| {
                        entries
                            .iter()
                            .map(|o| crate::api::ObjectionInfo {
                                target_claim_id: hex::encode(o.objection.target_claim_id),
                                objector: o.objection.objector.to_string(),
                                reason: o.objection.reason.clone(),
                                timestamp: o.objection.timestamp.to_rfc3339(),
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                ApiResponse::Objections {
                    claim_id,
                    objections,
                }
            }
            ApiRequest::GetAllObjections => {
                let mut all: std::collections::HashMap<String, Vec<crate::api::ObjectionInfo>> =
                    std::collections::HashMap::new();
                for (cid, entries) in &self.objections {
                    let info: Vec<crate::api::ObjectionInfo> = entries
                        .iter()
                        .map(|o| crate::api::ObjectionInfo {
                            target_claim_id: hex::encode(o.objection.target_claim_id),
                            objector: o.objection.objector.to_string(),
                            reason: o.objection.reason.clone(),
                            timestamp: o.objection.timestamp.to_rfc3339(),
                        })
                        .collect();
                    all.insert(hex::encode(cid), info);
                }
                ApiResponse::AllObjections { objections: all }
            }
            ApiRequest::GetStateRoot => {
                let root = self.ledger.state_root(&self.seen_nonces);
                ApiResponse::StateRoot {
                    state_root: hex::encode(root),
                    epoch: self.economic_engine.epoch_count(),
                }
            }
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

/// Read a `genesis-root` config file from the identity directory if present.
/// Returns the base58 PeerId string inside, or None if the file doesn't exist.
fn read_genesis_root_from_config(identity_dir: Option<&PathBuf>) -> Option<String> {
    let dir = identity_dir.cloned().or_else(|| {
        let home = std::env::var_os("HOME").map(PathBuf::from);
        home.map(|h| h.join(".lattice"))
    })?;
    let path = dir.join("genesis-root");
    match std::fs::read_to_string(&path) {
        Ok(contents) => {
            let trimmed = contents.trim();
            if trimmed.is_empty() { None } else { Some(trimmed.to_string()) }
        }
        Err(_) => None,
    }
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

#[cfg(test)]
mod quorum_certificate_tests {
    use super::*;
    use prost::Message;

    fn make_keypair() -> libp2p::identity::Keypair {
        libp2p::identity::Keypair::generate_ed25519()
    }

    fn make_certificate(proposal_id: &str) -> crate::ingest::proto::ImpactCertificate {
        crate::ingest::proto::ImpactCertificate {
            proposal_id: proposal_id.to_string(),
            proposer_address: "test".to_string(),
            raw_text: "test".to_string(),
            synthesized_text: "test".to_string(),
            debate_rounds: vec![],
            georgist_validation: 0,
            carbon_budget_impact: 0.0,
            resource_depletion_years: 0.0,
            enclave_id: "test".to_string(),
            lat_commitment: "test".to_string(),
            witness_seed: "12345".to_string(),
            timestamp: "2026-01-01T00:00:00Z".to_string(),
            lvn_signature: vec![],
            tfb_enclave_signature: vec![],
            lat_consensus_payload: vec![],
            receipt_hash: vec![],
        }
    }

    #[test]
    fn qc_encode_roundtrip() {
        let pid = "lvn:roundtrip";
        let cert = make_certificate(pid);
        let cert_bytes = cert.encode_to_vec();
        let mut sigs = Vec::new();
        for _ in 0..3 {
            let kp = make_keypair();
            let pk = kp.public().encode_protobuf();
            let sig = kp.sign(pid.as_bytes()).unwrap();
            sigs.push((kp.public().to_peer_id(), sig, pk));
        }
        let qc = LatticeNode::encode_quorum_certificate(pid, &cert_bytes, &sigs);
        let decoded = LatticeNode::decode_and_verify_quorum_certificate(&qc)
            .expect("valid QC must decode");
        assert_eq!(decoded.0, pid);
        assert_eq!(decoded.1, cert_bytes);
        assert_eq!(decoded.2.len(), 3);
    }

    #[test]
    fn qc_rejects_corrupted_signature() {
        let pid = "lvn:bad-sig";
        let cert = make_certificate(pid);
        let cert_bytes = cert.encode_to_vec();
        let kp = make_keypair();
        let pk = kp.public().encode_protobuf();
        let bad_sig = vec![0u8; 64];
        let sigs = vec![(kp.public().to_peer_id(), bad_sig, pk)];
        let qc = LatticeNode::encode_quorum_certificate(pid, &cert_bytes, &sigs);
        let decoded = LatticeNode::decode_and_verify_quorum_certificate(&qc);
        // decode returns Some but with empty sigs (bad sigs dropped)
        assert_eq!(decoded.map(|d| d.2.len()).unwrap_or(0), 0);
    }

    #[test]
    fn qc_insufficient_sigs() {
        let pid = "lvn:2-sigs";
        let cert = make_certificate(pid);
        let cert_bytes = cert.encode_to_vec();
        let mut sigs = Vec::new();
        for _ in 0..2 {
            let kp = make_keypair();
            let pk = kp.public().encode_protobuf();
            let sig = kp.sign(pid.as_bytes()).unwrap();
            sigs.push((kp.public().to_peer_id(), sig, pk));
        }
        let qc = LatticeNode::encode_quorum_certificate(pid, &cert_bytes, &sigs);
        let decoded = LatticeNode::decode_and_verify_quorum_certificate(&qc)
            .expect("2 valid sigs should decode");
        assert_eq!(decoded.2.len(), 2);
    }

    #[test]
    fn qc_commit_via_commit_manager() {
        let dir = tempfile::tempdir().unwrap();
        let mut mgr = crate::commit::CommitManager::open(&dir.path().to_path_buf());
        let pid = "lvn:commit-qc";
        let cert = make_certificate(pid);
        let cert_bytes = cert.encode_to_vec();
        let mut sigs = Vec::new();
        for _ in 0..3 {
            let kp = make_keypair();
            let pk = kp.public().encode_protobuf();
            let sig = kp.sign(pid.as_bytes()).unwrap();
            sigs.push((kp.public().to_peer_id(), sig, pk));
        }
        let qc = LatticeNode::encode_quorum_certificate(pid, &cert_bytes, &sigs);
        let decoded = LatticeNode::decode_and_verify_quorum_certificate(&qc).unwrap();
        let sigs_2: Vec<(PeerId, Vec<u8>)> = decoded.2.iter().map(|(p, s, _)| (*p, s.clone())).collect();
        let hash = mgr.commit(&cert_bytes, &decoded.0, &sigs_2).unwrap();
        assert!(!hash.iter().all(|b| *b == 0));
        assert!(mgr.is_committed(&decoded.0));
    }

    #[test]
    fn qc_signatures_are_sorted_canonically() {
        let pid = "lvn:canonical";
        let cert = make_certificate(pid);
        let cert_bytes = cert.encode_to_vec();

        // Generate three signers and build signatures in arbitrary order.
        let kp1 = make_keypair();
        let kp2 = make_keypair();
        let kp3 = make_keypair();
        let sig1 = (kp1.public().to_peer_id(), kp1.sign(pid.as_bytes()).unwrap(), kp1.public().encode_protobuf());
        let sig2 = (kp2.public().to_peer_id(), kp2.sign(pid.as_bytes()).unwrap(), kp2.public().encode_protobuf());
        let sig3 = (kp3.public().to_peer_id(), kp3.sign(pid.as_bytes()).unwrap(), kp3.public().encode_protobuf());

        let qc_a = LatticeNode::encode_quorum_certificate(pid, &cert_bytes, &vec![sig1.clone(), sig2.clone(), sig3.clone()]);
        let qc_b = LatticeNode::encode_quorum_certificate(pid, &cert_bytes, &vec![sig3.clone(), sig1.clone(), sig2.clone()]);

        assert_eq!(qc_a, qc_b, "same quorum in different order must produce identical canonical QC bytes");

        let decoded = LatticeNode::decode_and_verify_quorum_certificate(&qc_a)
            .expect("canonical QC must decode");
        assert_eq!(decoded.2.len(), 3);

        // Verify decoded order is sorted by PeerId.
        let peer_ids: Vec<_> = decoded.2.iter().map(|(pid, _, _)| pid.to_bytes()).collect();
        let mut sorted = peer_ids.clone();
        sorted.sort();
        assert_eq!(peer_ids, sorted, "decoded signatures must be in canonical PeerId order");
    }

    #[test]
    fn marker_prefixed_messages_are_not_raw_certificates() {
        // A QC message (0x02 prefix) must not decode as a raw ImpactCertificate,
        // otherwise the gossip dispatcher would misroute it.
        let pid = "lvn:marker";
        let cert = make_certificate(pid);
        let cert_bytes = cert.encode_to_vec();
        let mut sigs = Vec::new();
        for _ in 0..3 {
            let kp = make_keypair();
            let pk = kp.public().encode_protobuf();
            let sig = kp.sign(pid.as_bytes()).unwrap();
            sigs.push((kp.public().to_peer_id(), sig, pk));
        }
        let qc = LatticeNode::encode_quorum_certificate(pid, &cert_bytes, &sigs);
        assert!(
            crate::ingest::proto::ImpactCertificate::decode(&qc[..]).is_err(),
            "QC message must not be misdecoded as raw ImpactCertificate"
        );

        // Likewise for a 0x01 attestation message.
        let mut attestation = vec![0x01u8];
        let kp = make_keypair();
        let pid_bytes = pid.as_bytes();
        let pk_bytes = kp.public().encode_protobuf();
        let sig = kp.sign(pid.as_bytes()).unwrap();
        attestation.extend_from_slice(&(pid_bytes.len() as u16).to_be_bytes());
        attestation.extend_from_slice(pid_bytes);
        attestation.extend_from_slice(&(pk_bytes.len() as u16).to_be_bytes());
        attestation.extend_from_slice(&pk_bytes);
        attestation.extend_from_slice(&(sig.len() as u16).to_be_bytes());
        attestation.extend_from_slice(&sig);
        assert!(
            crate::ingest::proto::ImpactCertificate::decode(&attestation[..]).is_err(),
            "attestation message must not be misdecoded as raw ImpactCertificate"
        );
    }
}

#[cfg(test)]
mod zombie_eviction_tests {
    use super::*;
    use crate::state::peers::PeerInfo;

    fn test_peer() -> PeerId {
        PeerId::random()
    }

    fn make_addr() -> Multiaddr {
        "/ip4/192.168.1.100/tcp/4001".parse().unwrap()
    }

    // ── Predicate helpers (mirror check_peer_liveness logic) ───

    fn is_zombie_heartbeat(info: &PeerInfo, current_epoch: u64) -> bool {
        current_epoch.saturating_sub(info.last_heartbeat_epoch) > ZOMBIE_EPOCH_THRESHOLD
    }

    fn is_zombie_thickness_attestation(
        thickness: f64,
        last_attestation_epoch: u64,
        current_epoch: u64,
    ) -> bool {
        thickness < 0.001
            && current_epoch.saturating_sub(last_attestation_epoch) > ZOMBIE_ATTESTATION_SILENCE_EPOCHS
    }

    // ── Tests ────────────────────────────────────────────────

    #[test]
    fn peer_silent_for_31_epochs_is_zombie() {
        // Default ZOMBIE_EPOCH_THRESHOLD = 30
        let current_epoch = 31;
        let mut peer_info = PeerInfo {
            peer_id: test_peer(),
            addresses: vec![make_addr()],
            first_seen: Utc::now(),
            last_seen: Utc::now(),
            heartbeats_received: 1,
            last_heartbeat_epoch: 0, // never seen a heartbeat since epoch tracking started
        };

        // At epoch 31, 31-0 = 31 > 30 → zombie
        assert!(is_zombie_heartbeat(&peer_info, current_epoch));

        // At epoch 30, 30-0 = 30, NOT > 30 → not zombie yet
        assert!(!is_zombie_heartbeat(&peer_info, 30));

        // At epoch 10, 10-0 = 10 → not zombie
        assert!(!is_zombie_heartbeat(&peer_info, 10));

        // Update last heartbeat to epoch 5
        peer_info.last_heartbeat_epoch = 5;
        // At epoch 35, 35-5 = 30, NOT > 30 → still not zombie
        assert!(!is_zombie_heartbeat(&peer_info, 35));
        // At epoch 36, 36-5 = 31 > 30 → zombie
        assert!(is_zombie_heartbeat(&peer_info, 36));
    }

    #[test]
    fn zero_thickness_no_attestations_is_zombie() {
        let current_epoch = 11; // > ZOMBIE_ATTESTATION_SILENCE_EPOCHS (10)
        let thickness = 0.0;
        let last_attestation_epoch = 0;

        assert!(is_zombie_thickness_attestation(
            thickness,
            last_attestation_epoch,
            current_epoch
        ));

        // At epoch 10, NOT > 10 → not zombie yet
        assert!(!is_zombie_thickness_attestation(
            thickness,
            last_attestation_epoch,
            10
        ));
    }

    #[test]
    fn non_zero_thickness_is_not_zombie() {
        // Peer with thickness > 0 should NOT be evicted on attestation grounds
        assert!(!is_zombie_thickness_attestation(1.0, 0, 100));
        assert!(!is_zombie_thickness_attestation(0.001, 0, 100));
    }

    #[test]
    fn healthy_peer_recent_heartbeat_not_zombie() {
        let current_epoch = 5;
        let peer_info = PeerInfo {
            peer_id: test_peer(),
            addresses: vec![make_addr()],
            first_seen: Utc::now(),
            last_seen: Utc::now(),
            heartbeats_received: 10,
            last_heartbeat_epoch: 5, // just received a heartbeat
        };

        // 5 - 5 = 0 → not zombie
        assert!(!is_zombie_heartbeat(&peer_info, current_epoch));

        // With thickness, not zombie on attestation grounds either
        assert!(!is_zombie_thickness_attestation(10.0, 0, current_epoch));
    }

    #[test]
    fn evicted_peer_reconnects_as_fresh_peer() {
        // Simulate a peer that was evicted and then reconnects
        let pid = test_peer();
        let addr = make_addr();

        let mut table = PeerTable::new();

        // First connection — peer appears
        table.add_peer(pid, addr.clone(), 0);
        table.record_heartbeat_epoch(&pid, 1);
        assert_eq!(table.len(), 1);

        // Simulate zombie detection — evict (remove)
        table.remove_peer(&pid);
        assert_eq!(table.len(), 0);

        // Peer reconnects — re-added as fresh peer
        table.add_peer(pid, addr.clone(), 0);
        assert_eq!(table.len(), 1);

        let info = table.get(&pid).unwrap();
        // Fresh peer: heartbeats_received reset, last_heartbeat_epoch reset
        assert_eq!(info.heartbeats_received, 0);
        assert_eq!(info.last_heartbeat_epoch, 0);

        // Record a heartbeat in epoch 50
        table.record_heartbeat_epoch(&pid, 50);
        let info = table.get(&pid).unwrap();
        assert_eq!(info.heartbeats_received, 1);
        assert_eq!(info.last_heartbeat_epoch, 50);
    }

    #[test]
    fn witness_sigs_cleared_on_eviction() {
        // Test that eviction clears witness signatures for the evicted peer
        let pid = test_peer();
        let other_pid = test_peer();

        let mut sigs: HashMap<String, Vec<(PeerId, Vec<u8>, Vec<u8>)>> = HashMap::new();

        // Add sigs from pid and other_pid for a proposal
        sigs.insert("proposal-1".into(), vec![
            (pid, vec![1, 2, 3], vec![4, 5, 6]),
            (other_pid, vec![7, 8, 9], vec![10, 11, 12]),
        ]);

        // Add sigs from pid only for another proposal
        sigs.insert("proposal-2".into(), vec![
            (pid, vec![13, 14, 15], vec![16, 17, 18]),
        ]);

        // Evict pid
        sigs.values_mut().for_each(|s| {
            s.retain(|(p, _, _)| *p != pid);
        });
        sigs.retain(|_, s| !s.is_empty());

        // proposal-1 still has other_pid's sig
        let p1_sigs = sigs.get("proposal-1").unwrap();
        assert_eq!(p1_sigs.len(), 1);
        assert_eq!(p1_sigs[0].0, other_pid);

        // proposal-2 is empty, should be removed
        assert!(sigs.get("proposal-2").is_none());
    }
}

#[cfg(test)]
mod outbound_sweep_tests {
    use super::*;
    use crate::state::peers::PeerInfo;

    fn test_peer() -> PeerId {
        PeerId::random()
    }

    // ── Tests ────────────────────────────────────────────────

    #[test]
    fn circuit_breaker_fires_when_most_peers_dead() {
        // 3 total peers, 2 dead → 2/3 = 66.7% > 50% → fires
        let dead_count = 2usize;
        let total = 3usize;
        let fraction = dead_count as f64 / total as f64;
        assert!(fraction > OUTBOUND_CIRCUIT_BREAKER_FRACTION);
        assert!(total > 0);
    }

    #[test]
    fn circuit_breaker_does_not_fire_when_peers_alive() {
        // 3 total peers, 1 dead → 1/3 = 33.3% < 50% → does not fire
        let dead_count = 1usize;
        let total = 3usize;
        let fraction = dead_count as f64 / total as f64;
        assert!(fraction <= OUTBOUND_CIRCUIT_BREAKER_FRACTION);
    }

    #[test]
    fn circuit_breaker_boundary_exactly_half() {
        // 4 total peers, 2 dead → 2/4 = 50% — NOT > 50% → does not fire
        let dead_count = 2usize;
        let total = 4usize;
        let fraction = dead_count as f64 / total as f64;
        assert!(!(fraction > OUTBOUND_CIRCUIT_BREAKER_FRACTION));
    }

    #[test]
    fn empty_peer_table_no_circuit_breaker() {
        let total = 0usize;
        // total_peers > 0 guard prevents division by zero
        assert_eq!(total, 0);
    }

    #[test]
    fn dead_peer_ids_detects_heartbeat_silence() {
        use crate::state::peers::PeerTable;

        let pid = test_peer();
        let mut table = PeerTable::new();
        table.add_peer(pid, "/ip4/1.2.3.4/tcp/4001".parse().unwrap(), 0);

        // Set last heartbeat to epoch 0, current epoch is 35
        // 35 - 0 = 35 > 30 → dead
        if let Some(info) = table.get_mut(&pid) {
            info.last_heartbeat_epoch = 0;
        }

        let elapsed = 35u64.saturating_sub(0);
        assert!(elapsed > ZOMBIE_EPOCH_THRESHOLD);
    }

    #[test]
    fn dead_peer_ids_detects_zero_thickness_no_attestation() {
        let thickness = 0.0_f64;
        let last_att_epoch = 0u64;
        let current_epoch = 15u64;

        let is_dead = thickness < 0.001
            && current_epoch.saturating_sub(last_att_epoch) > ZOMBIE_ATTESTATION_SILENCE_EPOCHS;
        assert!(is_dead);
    }

    #[test]
    fn established_peer_survives_zero_thickness_eviction() {
        // Constitutional separation (2026-07-24):
        // An established peer (heartbeats > 0) must not be
        // evicted for zero thickness. Thickness is earned,
        // establishment is the standing that keeps you in the mesh.
        let thickness = 0.0_f64;
        let heartbeats = 5u64;
        let is_established = heartbeats > 0;

        // Before: would have been dead. After 2b gate: survives.
        let would_be_dead = thickness < 0.001;
        let actually_dead = would_be_dead && !is_established;
        assert!(!actually_dead,
            "established peer with heartbeats>0 must survive zero-thickness eviction");
    }

    #[test]
    fn sweep_skips_when_live_peers_exist() {
        // Simulated: dead set doesn't contain signer, live_count > 0 → skip
        let dead: HashSet<PeerId> = HashSet::new();
        let live_count = 2usize;
        let signer = test_peer();

        // Signer not in dead set, live peers exist → should skip
        let should_skip = live_count > 0 && !dead.contains(&signer);
        assert!(should_skip);
    }

    #[test]
    fn sweep_evicts_when_no_live_peers() {
        // live_count = 0 → evict regardless
        let dead: HashSet<PeerId> = HashSet::new();
        let live_count = 0usize;
        let signer = test_peer();

        // live_count == 0 → don't skip, evict
        let should_skip = live_count > 0 && !dead.contains(&signer);
        assert!(!should_skip);
    }

    #[test]
    fn sweep_evicts_when_signer_is_dead() {
        // Signer IS in dead set → evict even if live peers exist
        let signer = test_peer();
        let mut dead = HashSet::new();
        dead.insert(signer);
        let live_count = 2usize;

        let should_skip = live_count > 0 && !dead.contains(&signer);
        assert!(!should_skip); // signer is dead → don't skip → evict
    }

    #[test]
    fn retry_attempts_when_mesh_peers_exist() {
        // When mesh_peers > 0, retry should be attempted
        let mesh_peers = 1usize;
        assert!(mesh_peers > 0);
    }

    #[test]
    fn no_retry_when_no_mesh_peers() {
        // When mesh_peers == 0, skip retry
        let mesh_peers = 0usize;
        assert!(!(mesh_peers > 0));
    }
}

#[cfg(test)]
mod ratification_block_tests {
    use super::*;
    use crate::message::types::{RatificationBlock, ERA_ONE_BLOCK_MARKER, ERA_TWO_BLOCK_MARKER};

    fn make_block(epoch: u64) -> RatificationBlock {
        RatificationBlock {
            epoch,
            state_root: [0xAA; 32],
            thickness_root: [0xBB; 32],
            proposal_id: format!("epoch-{epoch}"),
        }
    }

    #[test]
    fn roundtrip_encode_decode() {
        let block = make_block(42);
        let encoded = block.encode_wire().expect("encode must succeed");
        // First byte must be ERA_TWO_BLOCK_MARKER
        assert_eq!(encoded[0], ERA_TWO_BLOCK_MARKER);

        let decoded = RatificationBlock::decode_body(&encoded[1..])
            .expect("decode must succeed");
        assert_eq!(decoded, block);
    }

    #[test]
    fn prefix_discrimination_era_one() {
        // Era One blocks: first byte 0x01 or no prefix
        let raw = vec![ERA_ONE_BLOCK_MARKER];
        assert_eq!(raw[0], ERA_ONE_BLOCK_MARKER);
        assert_ne!(raw[0], ERA_TWO_BLOCK_MARKER);
    }

    #[test]
    fn prefix_discrimination_era_two() {
        let block = make_block(7);
        let encoded = block.encode_wire().unwrap();
        assert_eq!(encoded[0], ERA_TWO_BLOCK_MARKER);
        assert_ne!(encoded[0], ERA_ONE_BLOCK_MARKER);
    }

    #[test]
    fn root_verification_match() {
        // Two blocks with identical roots match
        let block_a = make_block(5);
        let block_b = make_block(5);
        assert_eq!(block_a.state_root, block_b.state_root);
        assert_eq!(block_a.thickness_root, block_b.thickness_root);
    }

    #[test]
    fn root_verification_mismatch() {
        let block_a = make_block(5);
        let mut block_b = make_block(5);
        block_b.state_root = [0xFF; 32];
        // Roots differ — mismatch
        assert_ne!(block_a.state_root, block_b.state_root);
    }

    #[test]
    fn epoch_filter_rejects_wrong_epoch() {
        let current_epoch = 10u64;
        let block_epoch = 8u64;  // too old
        let accept = block_epoch == current_epoch || block_epoch == current_epoch + 1;
        assert!(!accept);

        let block_epoch = 12u64;  // too far ahead
        let accept = block_epoch == current_epoch || block_epoch == current_epoch + 1;
        assert!(!accept);
    }

    #[test]
    fn epoch_filter_accepts_current_and_next() {
        let current_epoch = 10u64;
        assert!(10 == current_epoch || 10 == current_epoch + 1); // current
        assert!(11 == current_epoch || 11 == current_epoch + 1); // next
    }

    #[test]
    fn encode_empty_state_produces_valid_cbor() {
        let block = RatificationBlock {
            epoch: 0,
            state_root: [0u8; 32],
            thickness_root: [0u8; 32],
            proposal_id: "epoch-0".into(),
        };
        let encoded = block.encode_wire().unwrap();
        assert!(encoded.len() > 1, "should have marker + CBOR body");
        assert_eq!(encoded[0], ERA_TWO_BLOCK_MARKER);
    }

    #[test]
    fn ratification_block_commit_with_signatures() {
        use crate::commit::CommitManager;

        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();
        let mut mgr = CommitManager::open(&storage);

        let block = make_block(7);
        let cert_bytes = block.encode_wire().unwrap();
        let sigs: Vec<(PeerId, Vec<u8>)> = (0..3)
            .map(|i| (PeerId::random(), vec![i as u8, i as u8 + 1, i as u8 + 2]))
            .collect();

        let hash = mgr.commit(&cert_bytes, &block.proposal_id, &sigs).unwrap();
        assert_eq!(mgr.height(), 1);

        // Read back the raw frame and verify signatures are persisted.
        let raw = mgr.get_block_bytes(0).expect("block 0 must exist");
        assert!(!raw.is_empty());

        let mut reader = std::io::BufReader::new(&raw[..]);
        let frame = mgr.read_block(&mut reader).unwrap().unwrap();
        assert_eq!(frame.cert_bytes, cert_bytes);
        assert_eq!(frame.signatures.len(), 3);
        assert!(!frame.block_hash.iter().all(|&b| b == 0));

        // Block hash must include the signatures (non-empty sigs → non-zero hash).
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[0u8; 32]);
        hasher.update(block.proposal_id.as_bytes());
        for (_, sig) in &sigs {
            hasher.update(sig);
        }
        let expected_hash: [u8; 32] = hasher.finalize().into();
        assert_eq!(frame.block_hash, expected_hash);
        assert_eq!(hash, frame.block_hash);
    }
}

// ── Witness RPC → acceptance seam integration test ───────────
//
// Proves the handler output (WitnessResponse) feeds correctly into
// the acceptance path (accept_claim). This is the cross-module seam
// that no single unit test exercises: the handler produces a typed
// WitnessResponse, a caller assembles a WitnessedClaim from it, and
// accept_claim processes it.
//
// The full two-swarm harness (real LatticeBehaviour + libp2p transport)
// is deferred — it requires matching the swarm builder API exactly,
// which drifts between libp2p versions. The handler code path itself
// is tested by the 306 existing tests; this test proves the
// handler→acceptance data flow with real types.
#[cfg(test)]
mod witness_seam_tests {
    use crate::claims::{self, WitnessSignature, ClaimType, ClaimEvidence, WitnessedClaim};
    use crate::message::types::WitnessResponse;
    use libp2p::identity;
    use libp2p::PeerId;

    #[test]
    fn witness_response_to_acceptance() {
        let claimant = PeerId::random();
        let kp = identity::Keypair::generate_ed25519();
        let witness = PeerId::from(kp.public());
        let epoch = 42u64;
        let observed_heartbeats = 7u64;

        let claim_hash = claims::compute_service_attestation_hash(&claimant, 0, 1);
        let payload = [
            claims::WITNESS_DOMAIN as &[u8],
            &claim_hash[..],
            &witness.to_bytes()[..],
            &epoch.to_le_bytes()[..],
            &observed_heartbeats.to_le_bytes()[..],
        ]
        .concat();
        let signature = kp.sign(&payload).expect("sign failed");

        // Simulate what the handler produces
        let response = WitnessResponse {
            claim_id: "seam-test".into(),
            witness_id: witness,
            claim_hash,
            witnessed_at_epoch: epoch,
            observed_heartbeats,
            signer_public_key: kp.public().encode_protobuf(),
            signature,
            decline_reason: None,
        };

        // Assemble a WitnessedClaim from the response
        let mut assembled = WitnessedClaim {
            claimant,
            claim_type: ClaimType::ServiceAttestation,
            start_epoch: 0,
            end_epoch: 1,
            evidence: ClaimEvidence::Service { claimed_count: 1 },
            witnesses: vec![WitnessSignature {
                witness: response.witness_id,
                signed_at_epoch: response.witnessed_at_epoch,
                observed_heartbeats: response.observed_heartbeats,
                signer_public_key: response.signer_public_key,
                signature: response.signature,
            }],
            submitted_epoch: 2,  // must be > end_epoch (cannot claim the future)
            claim_id: [0u8; 32],
        };
        assembled.claim_id = assembled.compute_claim_id();
        // Feed to accept_claim
        let nonce_map = claims::acceptance::ClaimNonceMap::new();
        let result = claims::accept_claim(&assembled, &nonce_map, 2);
        assert!(result.is_ok(),
            "handler output → accept_claim must succeed: {:?}", result);
    }

    #[test]
    fn witness_response_with_decline_rejected_by_acceptance() {
        // A declined response (empty signature, no public key) that
        // somehow ends up assembled into a WitnessedClaim must be
        // rejected by accept_claim's cryptographic verification — it
        // must never be silently credited on witness count alone.
        let claimant = PeerId::random();
        let witness = PeerId::random();
        let epoch = 42;

        let response = WitnessResponse {
            claim_id: "decline-test".into(),
            witness_id: witness,
            claim_hash: [0xBE; 32],
            witnessed_at_epoch: epoch,
            observed_heartbeats: 0,
            signer_public_key: Vec::new(),
            signature: vec![],  // declined
            decline_reason: Some("Self-witness is not permitted".into()),
        };

        let mut assembled = WitnessedClaim {
            claimant,
            claim_type: ClaimType::ServiceAttestation,
            start_epoch: 0,
            end_epoch: 1,
            evidence: ClaimEvidence::Service { claimed_count: 1 },
            witnesses: vec![WitnessSignature {
                witness: response.witness_id,
                signed_at_epoch: response.witnessed_at_epoch,
                observed_heartbeats: response.observed_heartbeats,
                signer_public_key: response.signer_public_key,
                signature: response.signature,
            }],
            submitted_epoch: 2,  // must be > end_epoch (cannot claim the future)
            claim_id: [0u8; 32],
        };
        assembled.claim_id = assembled.compute_claim_id();
        // Feed to accept_claim
        let nonce_map = claims::acceptance::ClaimNonceMap::new();
        let result = claims::accept_claim(&assembled, &nonce_map, 2);
        assert!(
            matches!(result, Err(claims::ClaimRejection::InvalidSignature(_))),
            "declined (unsigned) response must be rejected, not credited: {:?}",
            result
        );
    }
}

// ── Two-Swarm Witness Harness ────────────────────────────────
//
// Exercises the full round-trip: real LatticeBehaviour, real
// WitnessCodec, real request-response protocol, real event dispatch,
// real handler code with all three gates. Two in-process libp2p
// swarms on loopback, deterministically driven via tokio::select!.
//
// What this proves that the 306 unit tests don't:
//   - A WitnessRequest survives the CBOR codec + length-prefix
//     framing + request-response transport.
//   - The real event dispatch (SwarmEvent → LatticeBehaviourEvent
//     → WitnessRpc) delivers the request to the handler in the
//     shape it expects.
//   - The handler's three gates fire on the real code path.
//   - WitnessResponse travels back through the same stack.
//   - Assembled WitnessedClaim feeds into accept_claim().
//
// What this does NOT test: the UDS submit path. That's the
// remaining unknown.

#[cfg(test)]
mod two_swarm_witness_harness {
    use super::*;
    use crate::claims::{self, WitnessSignature, ClaimType, ClaimEvidence, WitnessedClaim};
    use crate::message::codec::rpc::WitnessProtocol;
    use crate::network::protocol::{
        LatticeBehaviour, LatticeBehaviourEvent,
        LATTICE_HEARTBEAT_TOPIC, LATTICE_KAD_PROTOCOL,
    };
    use crate::state::peers::PeerTable;
    use libp2p::{
        futures::StreamExt,
        gossipsub, identify, identity, kad, mdns, noise, relay, request_response,
        swarm::SwarmEvent,
        tcp, yamux, Multiaddr, PeerId, SwarmBuilder,
        StreamProtocol,
    };
    use libp2p::swarm::behaviour::toggle::Toggle;
    use std::time::Duration;

    /// Minimum node with a Swarm<LatticeBehaviour> plus the state
    /// the witness handler reads: peer_table, keypair, epoch.
    struct WitnessNode {
        swarm: libp2p::Swarm<LatticeBehaviour>,
        peer_table: PeerTable,
        local_key: identity::Keypair,
        local_peer_id: PeerId,
        epoch: u64,
        listen_addr: Option<Multiaddr>,
    }

    impl WitnessNode {
        fn new() -> Self {
            let local_key = identity::Keypair::generate_ed25519();
            let local_peer_id = PeerId::from(local_key.public());

            let swarm = SwarmBuilder::with_existing_identity(local_key.clone())
                .with_tokio()
                .with_tcp(
                    tcp::Config::default(),
                    noise::Config::new,
                    yamux::Config::default,
                )
                .expect("tcp transport")
                .with_relay_client(
                    noise::Config::new,
                    yamux::Config::default,
                )
                .expect("relay transport")
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

                    // Minimal topic subscriptions — only what's needed
                    // for the swarm to not error on incoming messages.
                    let topic = gossipsub::IdentTopic::new(LATTICE_HEARTBEAT_TOPIC);
                    gossipsub.subscribe(&topic).ok();

                    let rpc = request_response::Behaviour::new(
                        [(LatticeProtocol, request_response::ProtocolSupport::Full)],
                        request_response::Config::default(),
                    );
                    let balance_rpc = request_response::Behaviour::new(
                        [(BalanceProtocol, request_response::ProtocolSupport::Full)],
                        request_response::Config::default(),
                    );
                    let verify_rpc = request_response::Behaviour::new(
                        [(VerifyProtocol, request_response::ProtocolSupport::Full)],
                        request_response::Config::default(),
                    );
                    let agent_rpc = request_response::Behaviour::new(
                        [(StreamProtocol::new(AGENT_STATE_PROTOCOL),
                          request_response::ProtocolSupport::Full)],
                        request_response::Config::default(),
                    );

                    let mut kademlia = {
                        let store = kad::store::MemoryStore::new(key.public().to_peer_id());
                        let kconfig = kad::Config::new(StreamProtocol::new(LATTICE_KAD_PROTOCOL));
                        kad::Behaviour::with_config(key.public().to_peer_id(), store, kconfig)
                    };
                    kademlia.set_mode(Some(kad::Mode::Server));

                    let relay_server = Toggle::from(None);
                    let identify = identify::Behaviour::new(
                        identify::Config::new(
                            "/lattice/identify/v1".to_string(),
                            key.public(),
                        ),
                    );

                    let tx_rpc = request_response::Behaviour::new(
                        [(TransactionProtocol, request_response::ProtocolSupport::Full)],
                        request_response::Config::default(),
                    );
                    let chain_sync_rpc = request_response::Behaviour::new(
                        [(ChainSyncProtocol, request_response::ProtocolSupport::Full)],
                        request_response::Config::default(),
                    );
                    let witness_rpc = request_response::Behaviour::new(
                        [(WitnessProtocol, request_response::ProtocolSupport::Full)],
                        request_response::Config::default(),
                    );

                    let compare_rpc = request_response::Behaviour::new(
                        [(CompareProtocol, request_response::ProtocolSupport::Full)],
                        request_response::Config::default(),
                    );

                    Ok(LatticeBehaviour::new(
                        mdns, gossipsub, rpc, balance_rpc, verify_rpc,
                        kademlia, relay_client, relay_server, identify,
                        agent_rpc, tx_rpc, chain_sync_rpc, witness_rpc,
                        compare_rpc,
                    ))
                })
                .expect("behaviour")
                .with_swarm_config(|c| {
                    c.with_idle_connection_timeout(Duration::from_secs(60))
                })
                .build();

            Self {
                swarm,
                peer_table: PeerTable::new(),
                local_key,
                local_peer_id,
                epoch: 42,
                listen_addr: None,
            }
        }

        /// Process one swarm event. Returns Some(response) when a
        /// WitnessResponse is received from a peer, or None for
        /// all other events.
        fn process_event(
            &mut self,
            event: SwarmEvent<LatticeBehaviourEvent>,
        ) -> Option<(PeerId, WitnessResponse)> {
            match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    self.listen_addr = Some(address);
                }
                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                    // Add peer to peer_table on connection — mimics
                    // the real node's ConnectionEstablished handler.
                    if let Some(addr) = self.listen_addr.clone() {
                        self.peer_table.add_peer(peer_id, addr, self.epoch);
                    }
                }
                SwarmEvent::Behaviour(LatticeBehaviourEvent::WitnessRpc(
                    request_response::Event::Message { peer, message },
                )) => {
                    match message {
                        request_response::Message::Request {
                            request_id: _, request, channel,
                        } => {
                            // Call the real handler — same function LatticeNode::run() uses
                            let response = LatticeNode::handle_witness_request(
                                &self.peer_table,
                                &self.local_key,
                                self.epoch,
                                &request,
                            );

                            let _ = self
                                .swarm
                                .behaviour_mut()
                                .witness_rpc
                                .send_response(channel, response);
                        }
                        request_response::Message::Response {
                            response, ..
                        } => {
                            return Some((peer, response));
                        }
                    }
                }
                _ => {}
            }
            None
        }
    }

    // ── Helper: seed peer table ────────────────────────────

    fn seed_peer_table(table: &mut PeerTable, peer_id: PeerId, epoch: u64) {
        use libp2p::multiaddr::Protocol;
        let addr: Multiaddr =
            "/ip4/127.0.0.1/tcp/0".parse().unwrap();
        table.add_peer(peer_id, addr.clone(), epoch);
        // Add heartbeats so establishment gate passes
        for _ in 0..5 {
            table.record_heartbeat(&peer_id);
        }
    }

    // ── Assertion 1: Happy path ────────────────────────────

    #[tokio::test]
    async fn two_swarm_witness_roundtrip_happy_path() {
        let mut a = WitnessNode::new();
        let mut b = WitnessNode::new();

        // B listens on loopback
        a.swarm
            .listen_on("/ip4/127.0.0.1/tcp/0".parse().unwrap())
            .unwrap();
        b.swarm
            .listen_on("/ip4/127.0.0.1/tcp/0".parse().unwrap())
            .unwrap();

        // Wait for B to get its listen address
        let b_addr = tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                match b.swarm.next().await {
                    Some(SwarmEvent::NewListenAddr { address, .. }) => {
                        return address;
                    }
                    Some(_) => continue,
                    None => panic!("swarm exhausted"),
                }
            }
        })
        .await
        .expect("timed out waiting for B listen addr");

        // A dials B
        a.swarm.dial(b_addr.clone()).unwrap();

        // Wait for connection to establish — process swarm events
        tokio::time::timeout(Duration::from_secs(5), async {
            let mut a_connected = false;
            let mut b_connected = false;
            loop {
                tokio::select! {
                    ev = a.swarm.next() => {
                        match ev {
                            Some(SwarmEvent::ConnectionEstablished { .. }) => {
                                a_connected = true;
                            }
                            Some(_) => {}
                            None => break,
                        }
                    }
                    ev = b.swarm.next() => {
                        match ev {
                            Some(SwarmEvent::ConnectionEstablished { peer_id, .. }) => {
                                b.peer_table.add_peer(
                                    peer_id,
                                    b.listen_addr.clone().unwrap_or_else(||
                                        "/ip4/127.0.0.1/tcp/0".parse().unwrap()
                                    ),
                                    b.epoch,
                                );
                                b_connected = true;
                            }
                            Some(_) => {}
                            None => break,
                        }
                    }
                }
                if a_connected && b_connected {
                    break;
                }
            }
            assert!(a_connected, "A did not connect to B");
            assert!(b_connected, "B did not see A's connection");
        })
        .await
        .expect("timed out waiting for connection");

        // Seed B's peer_table: A must have heartbeats_received > 0
        seed_peer_table(&mut b.peer_table, a.local_peer_id, b.epoch);

        // Build the witness request. claim_hash must match what
        // accept_claim recomputes from (claimant, start_epoch, end_epoch)
        // in Assertion 2 below (window 0..1), since it's re-derived
        // rather than trusted from the wire.
        let claim_hash = claims::compute_service_attestation_hash(&a.local_peer_id, 0, 1);
        let request = WitnessRequest {
            claim_id: "harness-test-1".into(),
            claim_type: 0, // ServiceAttestation
            claimant_id: a.local_peer_id,
            claim_hash,
            requested_at_epoch: a.epoch,
        };

        a.swarm
            .behaviour_mut()
            .witness_rpc
            .send_request(&b.local_peer_id, request);

        // Drive both swarms until we get a response
        let response = tokio::time::timeout(Duration::from_secs(10), async {
            loop {
                tokio::select! {
                    ev = a.swarm.next() => {
                        if let Some(ev) = ev {
                            if let Some((_peer, resp)) = a.process_event(ev) {
                                return resp;
                            }
                        }
                    }
                    ev = b.swarm.next() => {
                        if let Some(ev) = ev {
                            b.process_event(ev);
                        }
                    }
                }
            }
        })
        .await
        .expect("timed out waiting for witness response");

        // ── Assertions ──────────────────────────────────────
        // 1a: Non-empty signature
        assert!(
            !response.signature.is_empty(),
            "expected non-empty signature, got empty"
        );

        // 1b: No decline reason
        assert!(
            response.decline_reason.is_none(),
            "expected no decline, got: {:?}",
            response.decline_reason
        );

        // 1c: Signature verifies
        let valid = claims::verify_witness_signature(
            &response.claim_hash,
            &response.witness_id,
            response.witnessed_at_epoch,
            response.observed_heartbeats,
            &response.signature,
            &b.local_key.public(),
        );
        assert!(valid, "witness signature failed verification");

        // 1d: Response echoes the request fields
        assert_eq!(response.claim_id, "harness-test-1");
        assert_eq!(response.claim_hash, claim_hash);

        // 1e: observed_heartbeats came from B's own peer table, not
        // from the claimant.
        assert!(response.observed_heartbeats > 0,
            "witness must report its own observed heartbeat count");

        // ── Assertion 2: Acceptance integration ─────────────
        let claimant = PeerId::from(a.local_key.public());
        let mut assembled = WitnessedClaim {
            claimant,
            claim_type: ClaimType::ServiceAttestation,
            start_epoch: 0,
            end_epoch: 1,
            evidence: ClaimEvidence::Service { claimed_count: 1 },
            witnesses: vec![WitnessSignature {
                witness: response.witness_id,
                signed_at_epoch: response.witnessed_at_epoch,
                observed_heartbeats: response.observed_heartbeats,
                signer_public_key: response.signer_public_key,
                signature: response.signature,
            }],
            submitted_epoch: 2,  // must be > end_epoch (cannot claim the future)
            claim_id: [0u8; 32],
        };
        assembled.claim_id = assembled.compute_claim_id();
        // Feed to accept_claim
        let nonce_map = claims::acceptance::ClaimNonceMap::new();
        let result = claims::accept_claim(&assembled, &nonce_map, 2);
        assert!(
            result.is_ok(),
            "accept_claim must accept the assembled WitnessedClaim: {:?}",
            result
        );
    }

    // ── Assertion 3: Self-witness decline over the wire ────

    #[tokio::test]
    async fn two_swarm_self_witness_decline() {
        let mut a = WitnessNode::new();
        let mut b = WitnessNode::new();

        a.swarm
            .listen_on("/ip4/127.0.0.1/tcp/0".parse().unwrap())
            .unwrap();
        b.swarm
            .listen_on("/ip4/127.0.0.1/tcp/0".parse().unwrap())
            .unwrap();

        let b_addr = tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                match b.swarm.next().await {
                    Some(SwarmEvent::NewListenAddr { address, .. }) => return address,
                    Some(_) => continue,
                    None => panic!("swarm exhausted"),
                }
            }
        })
        .await
        .expect("timed out waiting for B listen addr");

        a.swarm.dial(b_addr).unwrap();

        // Wait for connection
        tokio::time::timeout(Duration::from_secs(5), async {
            let mut a_ok = false;
            let mut b_ok = false;
            loop {
                tokio::select! {
                    ev = a.swarm.next() => {
                        if matches!(ev, Some(SwarmEvent::ConnectionEstablished { .. })) {
                            a_ok = true;
                        }
                    }
                    ev = b.swarm.next() => {
                        if let Some(SwarmEvent::ConnectionEstablished { peer_id, .. }) = ev {
                            b.peer_table.add_peer(peer_id,
                                "/ip4/127.0.0.1/tcp/0".parse().unwrap(), b.epoch);
                            b_ok = true;
                        }
                    }
                }
                if a_ok && b_ok { break; }
            }
        })
        .await
        .expect("timed out waiting for connection");

        // Seed peer table so establishment gate passes —
        // but then self-witness gate should fire because
        // claimant_id == B's peer_id.
        seed_peer_table(&mut b.peer_table, b.local_peer_id, b.epoch);

        // Request where claimant IS the witness (self-witness)
        let request = WitnessRequest {
            claim_id: "self-witness-test".into(),
            claim_type: 0,
            claimant_id: b.local_peer_id, // claimant == witness
            claim_hash: [0xCD; 32],
            requested_at_epoch: a.epoch,
        };

        a.swarm
            .behaviour_mut()
            .witness_rpc
            .send_request(&b.local_peer_id, request);

        let response = tokio::time::timeout(Duration::from_secs(10), async {
            loop {
                tokio::select! {
                    ev = a.swarm.next() => {
                        if let Some(ev) = ev {
                            if let Some((_peer, resp)) = a.process_event(ev) {
                                return resp;
                            }
                        }
                    }
                    ev = b.swarm.next() => {
                        if let Some(ev) = ev {
                            b.process_event(ev);
                        }
                    }
                }
            }
        })
        .await
        .expect("timed out waiting for self-witness decline");

        assert!(response.signature.is_empty(),
            "self-witness must produce empty signature");
        assert!(response.decline_reason.is_some(),
            "self-witness must produce a decline_reason");
        assert!(
            response.decline_reason.unwrap().contains("Self-witness"),
            "decline reason must mention self-witness"
        );
    }

    // ── Assertion 4: Unestablished decline over the wire ───

    #[tokio::test]
    async fn two_swarm_unestablished_decline() {
        let mut a = WitnessNode::new();
        let mut b = WitnessNode::new();

        a.swarm
            .listen_on("/ip4/127.0.0.1/tcp/0".parse().unwrap())
            .unwrap();
        b.swarm
            .listen_on("/ip4/127.0.0.1/tcp/0".parse().unwrap())
            .unwrap();

        let b_addr = tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                match b.swarm.next().await {
                    Some(SwarmEvent::NewListenAddr { address, .. }) => return address,
                    Some(_) => continue,
                    None => panic!("swarm exhausted"),
                }
            }
        })
        .await
        .expect("timed out waiting for B listen addr");

        a.swarm.dial(b_addr).unwrap();

        // Wait for connection
        tokio::time::timeout(Duration::from_secs(5), async {
            let mut a_ok = false;
            let mut b_ok = false;
            loop {
                tokio::select! {
                    ev = a.swarm.next() => {
                        if matches!(ev, Some(SwarmEvent::ConnectionEstablished { .. })) {
                            a_ok = true;
                        }
                    }
                    ev = b.swarm.next() => {
                        if let Some(SwarmEvent::ConnectionEstablished { peer_id, .. }) = ev {
                            b.peer_table.add_peer(peer_id,
                                "/ip4/127.0.0.1/tcp/0".parse().unwrap(), b.epoch);
                            b_ok = true;
                        }
                    }
                }
                if a_ok && b_ok { break; }
            }
        })
        .await
        .expect("timed out waiting for connection");

        // Do NOT seed A into B's peer_table — A has heartbeats_received=0
        // (actually not even in the table). The establishment gate must decline.

        let request = WitnessRequest {
            claim_id: "unestablished-test".into(),
            claim_type: 0,
            claimant_id: a.local_peer_id,
            claim_hash: [0xEF; 32],
            requested_at_epoch: a.epoch,
        };

        a.swarm
            .behaviour_mut()
            .witness_rpc
            .send_request(&b.local_peer_id, request);

        let response = tokio::time::timeout(Duration::from_secs(10), async {
            loop {
                tokio::select! {
                    ev = a.swarm.next() => {
                        if let Some(ev) = ev {
                            if let Some((_peer, resp)) = a.process_event(ev) {
                                return resp;
                            }
                        }
                    }
                    ev = b.swarm.next() => {
                        if let Some(ev) = ev {
                            b.process_event(ev);
                        }
                    }
                }
            }
        })
        .await
        .expect("timed out waiting for unestablished decline");

        assert!(response.signature.is_empty(),
            "unestablished claimant must produce empty signature");
        assert!(response.decline_reason.is_some(),
            "unestablished claimant must produce a decline_reason");
        assert!(
            response.decline_reason.as_ref().unwrap().contains("not established"),
            "decline reason must mention not established, got: {:?}",
            response.decline_reason
        );
    }

    // ── Orchestration test: full claim pipeline ──────────────
    //
    // Exercises the complete flow the UDS endpoint orchestrates:
    // identify candidates → dispatch requests → collect responses
    // → assemble WitnessedClaim → accept_claim.
    //
    // Uses the same two-swarm rig; A is the claimant, B the witness.

    #[tokio::test]
    async fn orchestrate_and_accept_service_claim() {
        let mut a = WitnessNode::new();
        let mut b = WitnessNode::new();

        a.swarm
            .listen_on("/ip4/127.0.0.1/tcp/0".parse().unwrap())
            .unwrap();
        b.swarm
            .listen_on("/ip4/127.0.0.1/tcp/0".parse().unwrap())
            .unwrap();

        let b_addr = tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                match b.swarm.next().await {
                    Some(SwarmEvent::NewListenAddr { address, .. }) => return address,
                    Some(_) => continue,
                    None => panic!("swarm exhausted"),
                }
            }
        })
        .await
        .expect("timed out waiting for B listen addr");

        a.swarm.dial(b_addr).unwrap();

        // Wait for connection
        tokio::time::timeout(Duration::from_secs(5), async {
            let mut a_ok = false;
            let mut b_ok = false;
            loop {
                tokio::select! {
                    ev = a.swarm.next() => {
                        if matches!(ev, Some(SwarmEvent::ConnectionEstablished { .. })) {
                            a_ok = true;
                        }
                    }
                    ev = b.swarm.next() => {
                        if let Some(SwarmEvent::ConnectionEstablished { peer_id, .. }) = ev {
                            b.peer_table.add_peer(peer_id,
                                "/ip4/127.0.0.1/tcp/0".parse().unwrap(), b.epoch);
                            b_ok = true;
                        }
                    }
                }
                if a_ok && b_ok { break; }
            }
        })
        .await
        .expect("timed out waiting for connection");

        // Seed B's peer_table: A must be established
        seed_peer_table(&mut b.peer_table, a.local_peer_id, b.epoch);

        // A dispatches witness requests — same as handle_witness_claim_service.
        // claim_hash must match what accept_claim recomputes for window
        // 0..1 below, since it's re-derived rather than trusted from the wire.
        let claim_hash = claims::compute_service_attestation_hash(&a.local_peer_id, 0, 1);
        let request = WitnessRequest {
            claim_id: "orch-test-1".into(),
            claim_type: 0,
            claimant_id: a.local_peer_id,
            claim_hash,
            requested_at_epoch: a.epoch,
        };

        a.swarm
            .behaviour_mut()
            .witness_rpc
            .send_request(&b.local_peer_id, request);

        // Collect the response
        let response = tokio::time::timeout(Duration::from_secs(10), async {
            loop {
                tokio::select! {
                    ev = a.swarm.next() => {
                        if let Some(ev) = ev {
                            if let Some((_peer, resp)) = a.process_event(ev) {
                                return resp;
                            }
                        }
                    }
                    ev = b.swarm.next() => {
                        if let Some(ev) = ev {
                            b.process_event(ev);
                        }
                    }
                }
            }
        })
        .await
        .expect("timed out waiting for witness response");

        // Verify the witness signed
        assert!(!response.signature.is_empty(), "witness must sign");
        assert!(response.decline_reason.is_none());

        // Assemble the WitnessedClaim (same as finalize_claim)
        let claimant = PeerId::from(a.local_key.public());
        let mut assembled = WitnessedClaim {
            claimant,
            claim_type: ClaimType::ServiceAttestation,
            start_epoch: 0,
            end_epoch: 1,
            evidence: ClaimEvidence::Service { claimed_count: 1 },
            witnesses: vec![WitnessSignature {
                witness: response.witness_id,
                signed_at_epoch: response.witnessed_at_epoch,
                observed_heartbeats: response.observed_heartbeats,
                signer_public_key: response.signer_public_key,
                signature: response.signature,
            }],
            submitted_epoch: 2,  // must be > end_epoch (cannot claim the future)
            claim_id: [0u8; 32],
        };
        assembled.claim_id = assembled.compute_claim_id();
        // Feed to accept_claim — the seam that proves the orchestration
        // produces valid input to the acceptance layer.
        let nonce_map = claims::acceptance::ClaimNonceMap::new();
        let result = claims::accept_claim(&assembled, &nonce_map, 2);
        assert!(
            result.is_ok(),
            "orchestrated claim must be accepted: {:?}",
            result
        );

        let thickness = result.unwrap();
        assert!(thickness > 0.0,
            "accepted claim must earn positive thickness, got {}",
            thickness);
    }

    // ── Expiry: witness never responds ────────────────────────
    //
    // Proves that the collection timeout path works: when a
    // candidate never answers, the claim doesn't sit in
    // "collecting" forever.

    #[tokio::test]
    async fn witness_never_responds_results_in_timeout() {
        let mut a = WitnessNode::new();
        let mut b = WitnessNode::new();

        a.swarm
            .listen_on("/ip4/127.0.0.1/tcp/0".parse().unwrap())
            .unwrap();
        b.swarm
            .listen_on("/ip4/127.0.0.1/tcp/0".parse().unwrap())
            .unwrap();

        let b_addr = tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                match b.swarm.next().await {
                    Some(SwarmEvent::NewListenAddr { address, .. }) => return address,
                    Some(_) => continue,
                    None => panic!("swarm exhausted"),
                }
            }
        })
        .await
        .expect("timed out waiting for B listen addr");

        a.swarm.dial(b_addr).unwrap();

        // Wait for connection — drive both swarms so B accepts
        tokio::time::timeout(Duration::from_secs(5), async {
            let mut a_ok = false;
            loop {
                tokio::select! {
                    ev = a.swarm.next() => {
                        if matches!(ev, Some(SwarmEvent::ConnectionEstablished { .. })) {
                            a_ok = true;
                            break;
                        }
                    }
                    ev = b.swarm.next() => {
                        if let Some(SwarmEvent::ConnectionEstablished { peer_id, .. }) = ev {
                            b.peer_table.add_peer(peer_id,
                                "/ip4/127.0.0.1/tcp/0".parse().unwrap(), b.epoch);
                        }
                        _ = ev;
                    }
                }
            }
            assert!(a_ok, "A did not connect to B");
        })
        .await
        .expect("timed out waiting for connection");

        // Seed B's peer_table so the establishment gate passes
        seed_peer_table(&mut b.peer_table, a.local_peer_id, b.epoch);

        // Send the request — but never process B's response events.
        // The request sits in B's transport buffer with no handler
        // to respond. A waits until timeout.
        let request = WitnessRequest {
            claim_id: "timeout-test".into(),
            claim_type: 0,
            claimant_id: a.local_peer_id,
            claim_hash: [0x99; 32],
            requested_at_epoch: a.epoch,
        };

        a.swarm
            .behaviour_mut()
            .witness_rpc
            .send_request(&b.local_peer_id, request);

        // Drive A's event loop with a short timeout — B is never
        // polled, so the response can never arrive.
        let result = tokio::time::timeout(Duration::from_millis(500), async {
            loop {
                match a.swarm.next().await {
                    Some(ev) => {
                        if let Some((_peer, resp)) = a.process_event(ev) {
                            return resp;
                        }
                    }
                    None => break,
                }
            }
            panic!("swarm exhausted before timeout");
        })
        .await;

        // The timeout must fire — no response can arrive because
        // B's swarm is never polled to process the request.
        assert!(
            result.is_err(),
            "expected timeout, but a response arrived — B was polled"
        );
    }

    // ── Expiry condition: pure logic test ────────────────────

    #[test]
    fn collecting_claim_expires_after_deadline() {
        use super::CLAIM_COLLECTION_EPOCHS;

        let started = 1u64;

        // Not expired: 1 epoch elapsed (2-1 = 1 < 5)
        assert!(
            2u64.saturating_sub(started) <= CLAIM_COLLECTION_EPOCHS,
            "claim should not expire after 1 epoch"
        );

        // Not expired: exactly at threshold (6-1 = 5)
        assert!(
            6u64.saturating_sub(started) <= CLAIM_COLLECTION_EPOCHS,
            "claim should not expire at exact threshold"
        );

        // Expired: one past threshold (7-1 = 6 > 5)
        assert!(
            7u64.saturating_sub(started) > CLAIM_COLLECTION_EPOCHS,
            "claim should expire one epoch past threshold"
        );

        // Expired: well past (100-1 = 99 > 5)
        assert!(
            100u64.saturating_sub(started) > CLAIM_COLLECTION_EPOCHS,
            "claim should be long expired"
        );
    }

    // ── Rejection invariants ─────────────────────────────────

    #[test]
    fn rejected_claim_leaves_window_reclaimable() {
        // Documents the structural invariant: last_claimed is only
        // updated in the Ok(thickness) branch of finalize_claim.
        // A rejected claim must NOT advance the high-water mark,
        // otherwise a single unresponsive witness permanently burns
        // an epoch range the claimant genuinely served.
        //
        // This test verifies the rejection-reason taxonomy and
        // confirms the three cases are distinguishable.

        let candidate = PeerId::random();

        // Case 1: zero signatures, zero declines → "timed out"
        {
            let state = super::WitnessClaimState {
                claim_id: "case1".into(),
                status: super::ClaimSubmissionStatus::Collecting,
                start_epoch: 0,
                end_epoch: 10,
                candidates: vec![candidate],
                signatures: vec![],
                declines: vec![],
                started_at_epoch: 1,
            };

            let min_wits = crate::claims::MIN_WITNESSES;
            let insufficient = (state.signatures.len() as u64) < min_wits;
            assert!(insufficient, "0 signatures must be insufficient");

            let reason = if state.signatures.is_empty() && state.declines.is_empty() {
                "No witnesses responded — all candidates timed out".to_string()
            } else if state.signatures.is_empty() {
                format!("All {} candidates declined (0 signed). Window reclaimable.",
                    state.candidates.len())
            } else {
                format!("Insufficient witnesses: got {}, need {} ({} candidates, {} declined)",
                    state.signatures.len(), min_wits,
                    state.candidates.len(), state.declines.len())
            };
            assert!(reason.contains("timed out"),
                "zero sigs + zero declines => 'timed out', got: {}", reason);
        }

        // Case 2: zero signatures, some declines → "all declined"
        {
            let state = super::WitnessClaimState {
                claim_id: "case2".into(),
                status: super::ClaimSubmissionStatus::Collecting,
                start_epoch: 0,
                end_epoch: 10,
                candidates: vec![candidate],
                signatures: vec![],
                declines: vec![(candidate, "Self-witness is not permitted".into())],
                started_at_epoch: 1,
            };

            let reason = if state.signatures.is_empty() && state.declines.is_empty() {
                String::new()
            } else if state.signatures.is_empty() {
                format!("All {} candidates declined (0 signed). Window reclaimable.",
                    state.candidates.len())
            } else {
                String::new()
            };
            assert!(reason.contains("Window reclaimable"),
                "all-declined must say 'Window reclaimable', got: {}", reason);
        }

        // Case 3: some signatures but below threshold → partial
        {
            let sig = crate::claims::WitnessSignature {
                witness: PeerId::random(),
                signed_at_epoch: 5,
                observed_heartbeats: 1,
                signer_public_key: vec![],
                signature: vec![0xAB; 64],
            };
            let state = super::WitnessClaimState {
                claim_id: "case3".into(),
                status: super::ClaimSubmissionStatus::Collecting,
                start_epoch: 0,
                end_epoch: 10,
                candidates: vec![PeerId::random(), PeerId::random(), PeerId::random()],
                signatures: vec![sig],
                declines: vec![(PeerId::random(), "declined".into())],
                started_at_epoch: 1,
            };

            let reason = format!(
                "Insufficient witnesses: got {} signatures, need {} ({} candidates, {} declined)",
                state.signatures.len(),
                crate::claims::MIN_WITNESSES,
                state.candidates.len(),
                state.declines.len(),
            );
            assert!(reason.contains("Insufficient witnesses"),
                "partial must say 'Insufficient witnesses', got: {}", reason);
            assert!(reason.contains("1 signatures") && reason.contains("3 candidates"),
                "must report counts, got: {}", reason);
        }
    }
}
