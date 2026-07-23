//! Claim acceptance logic — replay defense, witness eligibility,
//! per-pair diminishing weight, thickness crediting.

use std::collections::HashMap;
use libp2p::PeerId;

use super::{
    ClaimEvidence, ClaimType, WitnessSignature, WitnessedClaim,
    ATTESTATION_SILENCE_EPOCHS, DECAY_PER_EPOCH, MAX_CLAIM_WINDOW,
    MIN_THICKNESS, MIN_WITNESSES,
};

// ── Transient state: last claimed epoch per (claimant, type) ──

/// Tracks the most recent end_epoch for accepted claims,
/// keyed by (claimant_base58, claim_type as u8).
/// Rebuilt from persisted claims on restart — same pattern as seen_nonces.
pub type ClaimNonceMap = HashMap<(String, u8), u64>;

// ── Per-pair attestation history ─────────────────────────────

/// Tracks per-pair attestation count within the current half-life window.
/// Key: (witness_base58, claimant_base58) → (epoch_of_last, count).
/// Count resets when decays push older claims below MIN_THICKNESS.
pub type PairHistory = HashMap<(String, String), (u64, u64)>;

// ── Acceptance ───────────────────────────────────────────────

/// Reasons a claim can be rejected.
#[derive(Debug, Clone)]
pub enum ClaimRejection {
    Malformed(&'static str),
    Overlap { last_end: u64 },
    InsufficientWitnesses { have: usize, need: u64 },
    WitnessNotEstablished(PeerId),
    InvalidSignature(PeerId),
    ClaimantEqualsWitness(PeerId),
    Internal(&'static str),
}

impl std::fmt::Display for ClaimRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClaimRejection::Malformed(msg) => write!(f, "malformed claim: {msg}"),
            ClaimRejection::Overlap { last_end } => {
                write!(f, "overlapping window: last end_epoch was {last_end}")
            }
            ClaimRejection::InsufficientWitnesses { have, need } => {
                write!(f, "need {need} witnesses, got {have}")
            }
            ClaimRejection::WitnessNotEstablished(pid) => {
                write!(f, "witness {pid} is not established (heartbeats == 0)")
            }
            ClaimRejection::InvalidSignature(pid) => {
                write!(f, "invalid signature from witness {pid}")
            }
            ClaimRejection::ClaimantEqualsWitness(pid) => {
                write!(f, "claimant {pid} cannot witness its own claim")
            }
            ClaimRejection::Internal(msg) => write!(f, "internal error: {msg}"),
        }
    }
}

impl std::error::Error for ClaimRejection {}

/// Check whether a peer is "established" — the constitutional separation
/// between presence (free, anyone who heartbeats can witness) and
/// standing (thickness, earned through claims).
pub fn is_established(heartbeats_received: u64) -> bool {
    heartbeats_received > 0
}

/// Compute per-pair diminishing weight for the nth attestation
/// from the same (witness, claimant) pair within the half-life window.
///
/// 1/sqrt(n) curve: gentle enough that small meshes don't starve,
/// strong enough to enforce witness diversity as the mesh scales.
pub fn pair_weight(n: u64) -> f64 {
    if n == 0 {
        return 0.0;
    }
    1.0 / (n as f64).sqrt()
}

/// Attempt to accept a WitnessedClaim.
///
/// Returns the thickness earned if accepted, or a ClaimRejection.
/// The caller is responsible for:
///   - Storing the accepted claim for persistence
///   - Updating the claim nonce map
///   - Adding a ThicknessEdge via the graph
///   - Broadcasting the acceptance to the mesh
pub fn accept_claim(
    claim: &WitnessedClaim,
    last_claimed: &ClaimNonceMap,
    established_peers: usize,
) -> Result<f64, ClaimRejection> {
    // ── Basic structural checks ───────────────────────────────
    if claim.start_epoch > claim.end_epoch {
        return Err(ClaimRejection::Malformed("start_epoch > end_epoch"));
    }

    let window = claim.end_epoch - claim.start_epoch;
    if window > MAX_CLAIM_WINDOW {
        return Err(ClaimRejection::Malformed("claim window exceeds MAX_CLAIM_WINDOW"));
    }

    if claim.submitted_epoch <= claim.end_epoch {
        return Err(ClaimRejection::Malformed("cannot claim the future"));
    }

    // ── Replay / overlap check ────────────────────────────────
    let key = (claim.claimant.to_base58(), claim.claim_type as u8);
    if let Some(&last_end) = last_claimed.get(&key) {
        if claim.start_epoch <= last_end {
            return Err(ClaimRejection::Overlap { last_end });
        }
    }

    // ── Witness eligibility ───────────────────────────────────
    // Clamp MIN_WITNESSES to (established_peers - 1) so claims
    // never become unsatisfiable in small meshes.
    let effective_min = if established_peers <= 1 {
        return Err(ClaimRejection::Internal("no established peers to witness"));
    } else if MIN_WITNESSES >= established_peers as u64 {
        (established_peers - 1) as u64
    } else {
        MIN_WITNESSES
    };

    if (claim.witnesses.len() as u64) < effective_min {
        return Err(ClaimRejection::InsufficientWitnesses {
            have: claim.witnesses.len(),
            need: effective_min,
        });
    }

    for sig in &claim.witnesses {
        if sig.witness == claim.claimant {
            return Err(ClaimRejection::ClaimantEqualsWitness(claim.claimant));
        }
        // NOTE: Full signature verification requires Ed25519 keys,
        // which are available via the node's identity keypair.
        // The caller must verify crypto; we verify structure here.
        if sig.witness.to_bytes().is_empty() {
            return Err(ClaimRejection::InvalidSignature(sig.witness));
        }
    }

    // ── Witness self-check: each witness attests its own data ──
    for sig in &claim.witnesses {
        if sig.observed_heartbeats == 0 && claim.claim_type == ClaimType::ServiceAttestation {
            // The witness claims it observed zero heartbeats from the
            // claimant during the window — this is not necessarily
            // invalid (the claimant might genuinely have been silent),
            // but such a claim would earn zero thickness.
        }
    }

    // ── Compute thickness earned ──────────────────────────────
    // For ServiceAttestation, each witness contributes its
    // observed_heartbeats as the base amount.
    let mut total = 0.0_f64;
    for (_i, sig) in claim.witnesses.iter().enumerate() {
        // The base contribution is the witness's observed heartbeats.
        // Each heartbeat observed = 0.001 thickness (1/1000 of a DUU).
        // At ~6 heartbeats/min for 24h, a day's attestation ≈ 8.5 thickness
        // before pair cap.
        total += sig.observed_heartbeats as f64 * 0.001;
    }
    // The pair-cap weight is applied by the caller based on PairHistory,
    // not computed here (the caller knows the pair's count).

    Ok(total)
}

/// Compute per-epoch decay multiplier for thickness.
pub fn decay_factor() -> f64 {
    DECAY_PER_EPOCH
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::PeerId;

    fn test_peer() -> PeerId {
        PeerId::random()
    }

    fn make_claim(
        claimant: PeerId,
        start: u64,
        end: u64,
        witnesses: Vec<WitnessSignature>,
    ) -> WitnessedClaim {
        WitnessedClaim {
            claimant,
            claim_type: ClaimType::ServiceAttestation,
            start_epoch: start,
            end_epoch: end,
            evidence: ClaimEvidence::Service { claimed_count: 0 },
            witnesses,
            submitted_epoch: end + 1,
        }
    }

    fn make_sig(witness: PeerId, obs: u64) -> WitnessSignature {
        WitnessSignature {
            witness,
            observed_heartbeats: obs,
            signed_at_epoch: 0,
            signature: vec![],
        }
    }

    #[test]
    fn test_overlap_rejection() {
        let c = test_peer();
        let w = test_peer();
        let claim1 = make_claim(c.clone(), 1, 100, vec![make_sig(w.clone(), 10)]);
        let mut map = ClaimNonceMap::new();
        let key = (c.to_base58(), 0u8);
        map.insert(key.clone(), 100);

        let claim2 = make_claim(c.clone(), 50, 150, vec![make_sig(w.clone(), 10)]);
        let result = accept_claim(&claim2, &map, 2);
        assert!(matches!(result, Err(ClaimRejection::Overlap { .. })));
    }

    #[test]
    fn test_clamp_invariant() {
        let c = test_peer();
        let w = test_peer();
        let claim = make_claim(c.clone(), 1, 10, vec![make_sig(w.clone(), 5)]);
        // 3 established peers, MIN_WITNESSES=1, no clamp needed
        let result = accept_claim(&claim, &ClaimNonceMap::new(), 3);
        assert!(result.is_ok());

        // But with only 2 established peers, effective MIN=1 still works
        let result2 = accept_claim(&claim, &ClaimNonceMap::new(), 2);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_claimant_cannot_be_witness() {
        let c = test_peer();
        let claim = make_claim(c.clone(), 1, 10, vec![make_sig(c.clone(), 5)]);
        let result = accept_claim(&claim, &ClaimNonceMap::new(), 2);
        assert!(matches!(result, Err(ClaimRejection::ClaimantEqualsWitness(_))));
    }

    #[test]
    fn test_single_established_peer_returns_error() {
        let c = test_peer();
        let w = test_peer();
        let claim = make_claim(c.clone(), 1, 10, vec![make_sig(w.clone(), 5)]);
        let result = accept_claim(&claim, &ClaimNonceMap::new(), 1);
        assert!(matches!(result, Err(ClaimRejection::Internal(_))));
    }

    #[test]
    fn test_pair_weight_1_over_sqrt_n() {
        assert!((pair_weight(1) - 1.0).abs() < 1e-10);
        assert!((pair_weight(4) - 0.5).abs() < 1e-10);
        assert!((pair_weight(9) - 1.0 / 3.0).abs() < 1e-10);
        assert!(pair_weight(0) == 0.0);
    }

    #[test]
    fn test_malformed_future_claim_rejected() {
        let c = test_peer();
        let w = test_peer();
        let claim = WitnessedClaim {
            claimant: c.clone(),
            claim_type: ClaimType::ServiceAttestation,
            start_epoch: 100,
            end_epoch: 200,
            evidence: ClaimEvidence::Service { claimed_count: 0 },
            witnesses: vec![make_sig(w.clone(), 5)],
            submitted_epoch: 150, // before end_epoch → invalid
        };
        let result = accept_claim(&claim, &ClaimNonceMap::new(), 2);
        assert!(matches!(result, Err(ClaimRejection::Malformed(_))));
    }

    #[test]
    fn test_established_check() {
        assert!(!is_established(0));
        assert!(is_established(1));
        assert!(is_established(100));
    }
}
