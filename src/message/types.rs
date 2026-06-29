use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

/// Direct request for a peer's status, sent over the request-response
/// protocol (`/lattice/rpc/v1`) rather than broadcast.
///
/// The `nonce` correlates a response with its request — the same habit
/// the transaction layer will need to match "I asked X" with "you answered Y".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusRequest {
    /// PeerId of the requesting node (string form).
    pub from: String,
    /// Correlation nonce, echoed back in the matching StatusResponse.
    pub nonce: u64,
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
}
