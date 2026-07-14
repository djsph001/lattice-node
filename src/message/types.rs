use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::agent::state::ModelSize;
use crate::ledger::types::SignedTransaction;

/// Top-level message envelope for all Lattice protocol messages.
///
/// Every message between nodes is wrapped in this enum.
/// CBOR-encoded on the wire for compact, schema-evolvable transport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LatticeMessage {
    /// Periodic liveness signal
    Heartbeat(Heartbeat),

    /// Node status report (richer than heartbeat)
    Status(StatusReport),

    /// A signed economic transaction
    Transaction(SignedTransaction),

    /// Phase 8: Agent task submission.
    AgentTask(AgentTaskMsg),

    // === Future phases ===
    // /// Governance proposal or vote
    // Governance(GovernanceAction),
}

/// Lightweight liveness signal broadcast at regular intervals.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeat {
    /// Human-readable node name
    pub node_name: String,
    /// Peer ID as string
    pub peer_id: String,
    /// When the heartbeat was generated
    pub timestamp: DateTime<Utc>,
    /// Number of peers this node currently sees
    pub peer_count: usize,
}

/// Richer status report, requested or broadcast less frequently.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusReport {
    pub node_name: String,
    pub peer_id: String,
    pub timestamp: DateTime<Utc>,
    pub peer_count: usize,
    /// Maximum model size this node can execute (Phase 10a: sortition filtering).
    pub max_model_size: ModelSize,
    /// Available GPU VRAM in bytes that this node can allocate.
    pub vram_bytes: u64,
    /// Uptime in seconds since node start
    pub uptime_secs: u64,
    /// Software version
    pub version: String,
    /// Lattice protocol version for compatibility checks
    pub protocol_version: u32,
}

impl StatusReport {
    pub fn protocol_compatible(&self, other: &StatusReport) -> bool {
        self.protocol_version == other.protocol_version
    }
}

/// A message carrying an agent task for distributed execution.
/// Phase 8: broadcast on the lattice/agent/v1 gossipsub topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTaskMsg {
    pub task_id: String,
    pub origin: String,
    pub model: String,
    pub model_size: ModelSize,
    /// Minimum VRAM in bytes required to execute this task.
    /// Sortition filters out nodes with insufficient GPU memory.
    pub vram_bytes: u64,
    pub harness_version: u32,
    pub graph_blob: Vec<u8>,
    pub graph_hash: [u8; 32],
    pub deadline_epoch: u64,
    pub created_at: u64,
}

/// Direct request for a peer's status, sent over the request-response
/// protocol (`/lattice/rpc/v1`) rather than broadcast.
///
/// The `nonce` correlates a response with its request — the same habit
/// the transaction layer will need to match "I asked X" with "you answered Y".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatusRequest {
    /// Standard status query.
    Status {
        /// PeerId of the requesting node (string form).
        from: String,
        /// Correlation nonce, echoed back in the matching StatusResponse.
        nonce: u64,
    },
    /// Phase 6: relay receipt acknowledgment.
    /// Sent by the receiver of a gossipsub message to the delivering
    /// peer, confirming "I witnessed you relay this message."
    ReceiptAck {
        /// The signed receipt — proof that the delivering peer
        /// relayed a specific message.
        receipt: crate::economics::receipts::SignedReceipt,
    },
}

/// Direct reply to a `StatusRequest`, carrying the responder's self-reported
/// state straight from its peer table and runtime.
///
/// Kept distinct from `StatusReport` (the broadcast variant) on purpose: the
/// direct-reply form will diverge once the economic layer arrives — you don't
/// broadcast a balance, but you might disclose it in a credentialed query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    /// Echoed correlation nonce from the request.
    pub nonce: u64,
    pub node_name: String,
    pub peer_id: String,
    pub timestamp: DateTime<Utc>,
    /// Number of peers the responder currently sees.
    pub peer_count: usize,
    /// Maximum model size this node can execute (Phase 10a).
    pub max_model_size: ModelSize,
    /// Available GPU VRAM in bytes that this node can allocate.
    pub vram_bytes: u64,
    /// Uptime in seconds since the responder started.
    pub uptime_secs: u64,
    /// Total heartbeats this node has broadcast.
    pub heartbeats_sent: u64,
    /// Software version.
    pub version: String,
    /// Lattice protocol version for compatibility checks.
    pub protocol_version: u32,
}

/// Direct request for a specific peer's balance.
///
/// Phase 4: a node can interrogate another node's view of an account's
/// balance via request-response. This is the economic-layer equivalent
/// of the Phase 2c status handshake.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceRequest {
    /// PeerId whose balance is being queried.
    pub peer_id: String,
    /// Correlation nonce.
    pub nonce: u64,
}

/// Direct reply to a `BalanceRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceResponse {
    /// PeerId of the account.
    pub peer_id: String,
    /// Balance as seen by the responding node's local ledger.
    pub balance: u64,
    /// Echoed correlation nonce.
    pub nonce: u64,
}

// ── Phase 6: storage verification ──────────────────────────

/// Challenge a peer to prove possession of a specific chunk within a
/// resource.  Deterministic per (resource_id, epoch) so that every
/// validator arrives at the same challenge for a given target — proofs
/// are reusable and instantly cross-verifiable.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerifyRequest {
    /// Prove you hold `chunk_index` of the resource identified by
    /// `resource_id`, salted with `salt` to prevent pre-computation.
    StorageChallenge {
        /// Blake3 hash of the full resource — the Merkle root.
        resource_id: [u8; 32],
        /// Which 1 MiB chunk to prove.
        chunk_index: u64,
        /// Epoch-derived salt so the same chunk can't be replayed
        /// across epochs without actually holding the data.
        salt: [u8; 32],
    },
    // ── Phase 6c: trilateral verification receipts ──────────
    /// Sent by Validator to Relay: forward this challenge to the
    /// firewalled Target behind the Relay's p2p-circuit.
    ChallengeForward {
        challenge_id: [u8; 32],
        /// The firewalled node the challenge is actually for.
        target_peer: String,
        /// The inner challenge, boxed to keep the enum small.
        challenge: Box<VerifyRequest>,
    },
    /// Audit request from Validator to Relay: produce a
    /// signed IngressReceipt for a previously forwarded challenge.
    RelayAudit {
        challenge_id: [u8; 32],
    },
}

/// Cryptographic receipt proving possession of a specific chunk.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerifyResponse {
    /// The proof bundle for a `StorageChallenge`.
    StorageProof {
        /// `blake3(chunk_bytes)` — the Merkle leaf hash.  The
        /// verifier uses this with `merkle_proof` to reconstruct
        /// the `resource_id` root.
        chunk_hash: [u8; 32],
        /// `blake3(chunk_bytes || salt)` — proves the responder
        /// computed a hash over the actual data with the epoch
        /// salt, not a cached digest.
        salted_hash: [u8; 32],
        /// Merkle inclusion proofs mapping the chunk back to the
        /// `resource_id` root.  Each element is a 32-byte sibling
        /// hash on the path from leaf to root.
        merkle_proof: Vec<Vec<u8>>,
    },
    // ── Phase 6c: trilateral verification receipts ──────────
    /// Signed by the Relay: "I accepted custody of Challenge X
    /// at Timestamp_A for delivery to Target T."
    IngressReceipt(IngressReceipt),
    /// Signed by the Target: "I received Challenge X via Relay R
    /// at Timestamp_B, and here is my cryptographic proof."
    EgressReceipt(EgressReceipt),
}

// ── Phase 6c: trilateral receipt data types ────────────────

/// Ingress receipt — signed by the Relay node.
///
/// Proves the Relay accepted custody of a challenge for delivery
/// to a firewalled Target.  Stored by the Validator as an audit
/// trail.  If the challenge times out but this receipt exists,
/// the Target's health is frozen and the Relay is penalized.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IngressReceipt {
    /// The challenge ID from the ChallengeForward envelope.
    pub challenge_id: [u8; 32],
    /// The Relay's PeerId (string form).
    pub relay_peer: String,
    /// The firewalled Target's PeerId (string form).
    pub target_peer: String,
    /// When the Relay accepted custody (epoch-relative timestamp).
    pub timestamp: u64,
    /// Signature by the Relay over (challenge_id || relay_peer || target_peer || timestamp).
    pub signature: Vec<u8>,
}

/// Egress receipt — signed by the Target node.
///
/// Proves the Target received the challenge through the specified
/// Relay.  Includes the storage proof so the Validator can verify
/// both delivery and data possession in one message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EgressReceipt {
    /// The challenge ID from the ChallengeForward envelope.
    pub challenge_id: [u8; 32],
    /// The Relay that forwarded this challenge.
    pub relay_peer: String,
    /// The Target's PeerId (string form).
    pub target_peer: String,
    /// When the Target received the challenge (epoch-relative timestamp).
    pub timestamp: u64,
    /// The cryptographic storage proof — proves the Target
    /// actually holds the data, not just received the message.
    pub proof: StorageProofData,
    /// Signature by the Target over (challenge_id || relay_peer || target_peer || timestamp || proof_hash).
    pub signature: Vec<u8>,
}

/// Inline storage proof data (same fields as VerifyResponse::StorageProof,
/// but as a concrete struct for embedding in receipts).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StorageProofData {
    pub chunk_hash: [u8; 32],
    pub salted_hash: [u8; 32],
    pub merkle_proof: Vec<Vec<u8>>,
}
