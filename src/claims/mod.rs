//! Witnessed Claims — Contribution Primitive v1
//!
//! The first truth-claim type a peer can make about work performed,
//! witnessed by other peers, accruing thickness.
//!
//! ClaimType::ServiceAttestation (type 0) lets a peer claim it served
//! heartbeats during an epoch window. Witnesses confirm from their own
//! PeerInfo tables. Accepted claims add thickness edges to the graph,
//! subject to per-pair diminishing weight (1/sqrt(n)) and per-epoch
//! decay (30-day half-life at 64s/epoch).
//!
//! ── Thickness is local view, not consensus state ─────────────
//!
//! Each node maintains its own ThicknessGraph based on claims it has
//! independently accepted.  Nodes converge via gossip of accepted
//! claims, but nothing that reads thickness (Layer 2b eviction,
//! economic gating, witness eligibility) is consensus-critical —
//! each node decides for itself.  A one-epoch divergence between
//! nodes accepting the same claim at different epoch boundaries is
//! benign: a peer evicted by a slower node one epoch early re-
//! establishes on the next heartbeat.
//!
//! If thickness ever gates a consensus-critical function (sortition
//! selection, quorum weights), this assumption must be revisited.

mod acceptance;
mod old;

pub use acceptance::{accept_claim, ClaimRejection};
pub use old::{handle_state_claim, handle_judgment_claim};

use libp2p::PeerId;
use serde::{Deserialize, Serialize};

// ── Claim type enum — taxonomy lives in the type system ──────

/// The kind of truth-claim being made. Enum from day one so the
/// taxonomy is real, not a message bus with meaning retrofitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClaimType {
    /// Peer claims it served heartbeats / was online during the window.
    /// Witnesses confirm by checking their own `heartbeats_received`
    /// count for the claimant during the claimed epoch range.
    ServiceAttestation = 0,
}

impl ClaimType {
    pub fn name(&self) -> &'static str {
        match self {
            ClaimType::ServiceAttestation => "service_attestation",
        }
    }
}

// ── Claim envelope ───────────────────────────────────────────

/// A claim made by a peer about work performed, witnessed by other peers.
///
/// The claim type determines what is being attested. Future types slot
/// into the same envelope via the ClaimType enum.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessedClaim {
    /// The peer asserting the claim.
    pub claimant: PeerId,
    /// What kind of claim this is.
    pub claim_type: ClaimType,
    /// Epoch window this claim covers (inclusive).
    /// Epoch-bounded, never wall-clock — matches the rest of the economics.
    pub start_epoch: u64,
    pub end_epoch: u64,
    /// Claim-specific payload.
    pub evidence: ClaimEvidence,
    /// Witness signatures — each signer attests their OWN observation.
    pub witnesses: Vec<WitnessSignature>,
    /// The epoch at which this claim was submitted (for decay timing).
    pub submitted_epoch: u64,
}

/// Claim-type-specific payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClaimEvidence {
    Service {
        /// Heartbeats the claimant asserts it served during the window.
        /// The authoritative value is the witness's own observed count.
        claimed_count: u64,
    },
}

/// A single witness's signature, attesting its own observation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessSignature {
    /// The peer who observed the claimant and is signing.
    pub witness: PeerId,
    /// The epoch at which this witness signed.
    pub signed_at_epoch: u64,
    /// The witness's own count of heartbeats received FROM the claimant
    /// during the claimed window. This is the authoritative value.
    pub observed_heartbeats: u64,
    /// Ed25519 signature over the canonical message:
    ///   claimant || claim_type || start_epoch || end_epoch ||
    ///   evidence || witness || observed_heartbeats || signed_at_epoch
    pub signature: Vec<u8>,
}

// ── Constants ────────────────────────────────────────────────

/// Maximum epoch window for a single claim. ~18 hours at 64s/epoch.
pub const MAX_CLAIM_WINDOW: u64 = 1000;

/// Minimum number of witnesses required for a valid claim.
/// Stays at 1 for the self-attested era. Raising this is an explicit
/// governance act, not an auto-scaling parameter.
pub const MIN_WITNESSES: u64 = 1;

/// Per-epoch decay factor for a 30-day half-life at 64s/epoch.
/// 30 days ≈ 40,500 epochs. Factor = 2^(-1/40,500) ≈ 0.999983.
pub const DECAY_PER_EPOCH: f64 = 0.999_982_885_4;

/// Edges below this thickness are pruned during decay.
pub const MIN_THICKNESS: f64 = 0.001;

/// Domain separation prefix for witness signing.
/// Both signer and verifier must use the same constant to reconstruct
/// the canonical payload. Defined once, imported by both sides.
pub const WITNESS_DOMAIN: &[u8; 18] = b"lattice/witness/v1";

/// Verify a witness Ed25519 signature over the canonical payload:
///   WITNESS_DOMAIN || claim_hash || witness_peer_id_bytes || witnessed_at_epoch_bytes
///
/// Returns true if the signature is valid for the given public key.
pub fn verify_witness_signature(
    claim_hash: &[u8; 32],
    witness_peer_id: &PeerId,
    witnessed_at_epoch: u64,
    signature: &[u8],
    signer_public_key: &libp2p::identity::PublicKey,
) -> bool {
    let payload = [
        WITNESS_DOMAIN as &[u8],
        &claim_hash[..],
        &witness_peer_id.to_bytes()[..],
        &witnessed_at_epoch.to_le_bytes()[..],
    ]
    .concat();
    signer_public_key.verify(&payload, signature)
}

/// Number of epochs without an attestation before Layer 2b fires.
/// Matches the existing ZOMBIE_ATTESTATION_SILENCE_EPOCHS.
pub const ATTESTATION_SILENCE_EPOCHS: u64 = 10;
