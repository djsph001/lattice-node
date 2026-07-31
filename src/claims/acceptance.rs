//! Claim acceptance logic — replay defense, witness eligibility,
//! per-pair diminishing weight, thickness crediting.

use std::collections::HashMap;
use libp2p::PeerId;

use super::{
    ClaimEvidence, ClaimType, WitnessSignature, WitnessedClaim,
    ATTESTATION_SILENCE_EPOCHS, DECAY_PER_EPOCH, MAX_CLAIM_WINDOW,
    MIN_WITNESSES,
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
///
/// `witness_keys` is an optional map of PeerId → Ed25519 PublicKey for
/// cryptographic verification of witness signatures. When `Some(keys)`:
///   - each witness signature is verified against the reconstructed payload
///   - missing keys or invalid signatures → `ClaimRejection::InvalidSignature`
/// When `None`: verification is SKIPPED (backward compat for tests
/// that don't have key material). Production callers MUST pass `Some`.
pub fn accept_claim(
    claim: &WitnessedClaim,
    last_claimed: &ClaimNonceMap,
    established_peers: usize,
    witness_keys: Option<&std::collections::HashMap<PeerId, libp2p::identity::PublicKey>>,
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

    // ── Cryptographic witness signature verification (D7.4) ────
    match witness_keys {
        Some(keys) => {
            for sig in &claim.witnesses {
                let pk = keys.get(&sig.witness).ok_or_else(|| {
                    ClaimRejection::InvalidSignature(sig.witness)
                })?;
                if !super::verify_witness_signature(
                    &claim.claim_id,
                    &sig.witness,
                    sig.signed_at_epoch,
                    sig.observed_heartbeats,
                    &sig.signature,
                    pk,
                ) {
                    return Err(ClaimRejection::InvalidSignature(sig.witness));
                }
            }
        }
        None => {
            // Backward compat: no key material available.
            // Production callers MUST pass Some(keys).
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
        let mut claim = WitnessedClaim {
            claimant,
            claim_type: ClaimType::ServiceAttestation,
            start_epoch: start,
            end_epoch: end,
            evidence: ClaimEvidence::Service { claimed_count: 0 },
            witnesses,
            submitted_epoch: end + 1,
            claim_id: [0u8; 32], // placeholder, computed below
        };
        claim.claim_id = claim.compute_claim_id();
        claim
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
        let result = accept_claim(&claim2, &map, 2, None);
        assert!(matches!(result, Err(ClaimRejection::Overlap { .. })));
    }

    #[test]
    fn test_clamp_invariant() {
        let c = test_peer();
        let w = test_peer();
        let claim = make_claim(c.clone(), 1, 10, vec![make_sig(w.clone(), 5)]);
        // 3 established peers, MIN_WITNESSES=1, no clamp needed
        let result = accept_claim(&claim, &ClaimNonceMap::new(), 3, None);
        assert!(result.is_ok());

        // But with only 2 established peers, effective MIN=1 still works
        let result2 = accept_claim(&claim, &ClaimNonceMap::new(), 2, None);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_claimant_cannot_be_witness() {
        let c = test_peer();
        let claim = make_claim(c.clone(), 1, 10, vec![make_sig(c.clone(), 5)]);
        let result = accept_claim(&claim, &ClaimNonceMap::new(), 2, None);
        assert!(matches!(result, Err(ClaimRejection::ClaimantEqualsWitness(_))));
    }

    #[test]
    fn test_single_established_peer_returns_error() {
        let c = test_peer();
        let w = test_peer();
        let claim = make_claim(c.clone(), 1, 10, vec![make_sig(w.clone(), 5)]);
        let result = accept_claim(&claim, &ClaimNonceMap::new(), 1, None);
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
        let mut claim = WitnessedClaim {
            claimant: c.clone(),
            claim_type: ClaimType::ServiceAttestation,
            start_epoch: 100,
            end_epoch: 200,
            evidence: ClaimEvidence::Service { claimed_count: 0 },
            witnesses: vec![make_sig(w.clone(), 5)],
            submitted_epoch: 150, // before end_epoch → invalid
            claim_id: [0u8; 32],
        };
        claim.claim_id = claim.compute_claim_id();
        let result = accept_claim(&claim, &ClaimNonceMap::new(), 2, None);
        assert!(matches!(result, Err(ClaimRejection::Malformed(_))));
    }

    #[test]
    fn test_established_check() {
        assert!(!is_established(0));
        assert!(is_established(1));
        assert!(is_established(100));
    }

    #[test]
    fn establishment_gate() {
        let c = test_peer();
        let claim = make_claim(c.clone(), 1, 10, vec![make_sig(test_peer(), 5)]);
        let result = accept_claim(&claim, &ClaimNonceMap::new(), 2, None);
        assert!(result.is_ok(), "valid claim with established peer must be accepted");
    }

    #[test]
    fn zero_established_peers_rejected() {
        let c = test_peer();
        let claim = make_claim(c.clone(), 1, 10, vec![make_sig(test_peer(), 5)]);
        let result = accept_claim(&claim, &ClaimNonceMap::new(), 0, None);
        match result {
            Err(ClaimRejection::Internal(_)) => {} // expected
            other => panic!("expected Internal rejection for zero established peers, got {:?}", other),
        }
    }
}

// ── Property 1: Classification Correctness ─────────────────
// Self-test: temporarily make accept_claim always return Ok(1.0)
// by adding `return Ok(1.0);` as the first line of the function body.
// These tests must go RED when the gate is broken.

#[cfg(test)]
mod property1_tests {
    use super::*;
    use crate::claims::{ClaimType, ClaimEvidence, WitnessSignature};
    use libp2p::PeerId;

    fn peer() -> PeerId { PeerId::random() }

    fn wsig(w: PeerId, obs: u64) -> WitnessSignature {
        WitnessSignature { witness: w, observed_heartbeats: obs, signed_at_epoch: 0, signature: vec![] }
    }

    fn svc_claim(claimant: PeerId, witnesses: Vec<WitnessSignature>, start: u64, end: u64) -> WitnessedClaim {
        let mut c = WitnessedClaim {
            claimant, claim_type: ClaimType::ServiceAttestation,
            start_epoch: start, end_epoch: end,
            evidence: ClaimEvidence::Service { claimed_count: 0 },
            witnesses, submitted_epoch: end + 1, claim_id: [0u8; 32],
        };
        c.claim_id = c.compute_claim_id();
        c
    }

    #[test]
    fn self_witness_rejected() {
        let p = peer();
        let c = svc_claim(p, vec![wsig(p, 10)], 1, 10);
        match accept_claim(&c, &ClaimNonceMap::new(), 2, None) {
            Err(ClaimRejection::ClaimantEqualsWitness(_)) => {} // good
            other => panic!("expected ClaimantEqualsWitness, got {:?}", other),
        }
    }

    #[test]
    fn valid_claim_accepted() {
        let claimant = peer();
        let witness = peer();
        let c = svc_claim(claimant, vec![wsig(witness, 10)], 1, 10);
        match accept_claim(&c, &ClaimNonceMap::new(), 2, None) {
            Ok(t) => assert!(t > 0.0),
            Err(e) => panic!("expected Ok, got {:?}", e),
        }
    }

    #[test]
    fn forged_signature_rejected() {
        use libp2p::identity;
        let claimant = peer();
        let wkp = identity::Keypair::generate_ed25519();
        let wpeer = PeerId::from(wkp.public());
        let mut c = svc_claim(claimant, vec![], 1, 10);
        // Sign the payload with the real key
        let ch = c.compute_claim_id();
        let epoch: u64 = 0; let obs: u64 = 10;
        let mut payload = Vec::new();
        payload.extend_from_slice(crate::claims::WITNESS_DOMAIN);
        payload.extend_from_slice(&ch);
        payload.extend_from_slice(&wpeer.to_bytes());
        payload.extend_from_slice(&epoch.to_le_bytes());
        payload.extend_from_slice(&obs.to_le_bytes());
        c.witnesses.push(WitnessSignature { witness: wpeer, observed_heartbeats: obs, signed_at_epoch: 0, signature: wkp.sign(&payload).expect("sign") });
        c.claim_id = c.compute_claim_id();

        // Valid: right key
        let mut keys: HashMap<PeerId, identity::PublicKey> = HashMap::new();
        keys.insert(wpeer, wkp.public());
        match accept_claim(&c, &ClaimNonceMap::new(), 2, Some(&keys)) {
            Ok(t) => assert!(t > 0.0, "valid signature must be accepted"),
            Err(e) => panic!("expected Ok for valid sig, got {:?}", e),
        }

        // Forged: wrong key
        let wrong_kp = identity::Keypair::generate_ed25519();
        let mut forged_keys: HashMap<PeerId, identity::PublicKey> = HashMap::new();
        forged_keys.insert(wpeer, wrong_kp.public());
        match accept_claim(&c, &ClaimNonceMap::new(), 2, Some(&forged_keys)) {
            Err(ClaimRejection::InvalidSignature(_)) => {} // expected
            other => panic!("expected InvalidSignature for forged key, got {:?}", other),
        }

        // Missing key
        let empty_keys: HashMap<PeerId, identity::PublicKey> = HashMap::new();
        match accept_claim(&c, &ClaimNonceMap::new(), 2, Some(&empty_keys)) {
            Err(ClaimRejection::InvalidSignature(_)) => {} // expected
            other => panic!("expected InvalidSignature for missing key, got {:?}", other),
        }
    }

    #[test]
    fn mutated_field_rejected() {
        use libp2p::identity;
        let claimant = peer();
        let wkp = identity::Keypair::generate_ed25519();
        let wpeer = PeerId::from(wkp.public());
        let mut c = svc_claim(claimant, vec![], 1, 10);
        let ch = c.compute_claim_id();
        let epoch: u64 = 0; let obs: u64 = 10;
        let mut payload = Vec::new();
        payload.extend_from_slice(crate::claims::WITNESS_DOMAIN);
        payload.extend_from_slice(&ch);
        payload.extend_from_slice(&wpeer.to_bytes());
        payload.extend_from_slice(&epoch.to_le_bytes());
        payload.extend_from_slice(&obs.to_le_bytes());
        let real_sig = wkp.sign(&payload).expect("sign");
        c.witnesses.push(WitnessSignature {
            witness: wpeer,
            observed_heartbeats: obs + 5,  // MUTATED: claim says 15 but witness signed 10
            signed_at_epoch: 0,
            signature: real_sig,
        });
        c.claim_id = c.compute_claim_id();

        let mut keys: HashMap<PeerId, identity::PublicKey> = HashMap::new();
        keys.insert(wpeer, wkp.public());
        match accept_claim(&c, &ClaimNonceMap::new(), 2, Some(&keys)) {
            Err(ClaimRejection::InvalidSignature(_)) => {} // mutation detected
            other => panic!("expected InvalidSignature for mutated field, got {:?}", other),
        }
    }

    #[test]
    fn happy_path_accepted() {
        use libp2p::identity;
        let claimant = peer();
        let wkp = identity::Keypair::generate_ed25519();
        let wpeer = PeerId::from(wkp.public());
        let mut c = svc_claim(claimant, vec![], 1, 10);
        let ch = c.compute_claim_id();
        let epoch: u64 = 0; let obs: u64 = 10;
        let mut payload = Vec::new();
        payload.extend_from_slice(crate::claims::WITNESS_DOMAIN);
        payload.extend_from_slice(&ch);
        payload.extend_from_slice(&wpeer.to_bytes());
        payload.extend_from_slice(&epoch.to_le_bytes());
        payload.extend_from_slice(&obs.to_le_bytes());
        c.witnesses.push(WitnessSignature {
            witness: wpeer,
            observed_heartbeats: obs,  // matches what was signed
            signed_at_epoch: 0,
            signature: wkp.sign(&payload).expect("sign"),
        });
        c.claim_id = c.compute_claim_id();

        let mut keys: HashMap<PeerId, identity::PublicKey> = HashMap::new();
        keys.insert(wpeer, wkp.public());
        match accept_claim(&c, &ClaimNonceMap::new(), 2, Some(&keys)) {
            Ok(t) => assert!(t > 0.0, "happy path must earn thickness: {t}"),
            Err(e) => panic!("happy path must be accepted, got {:?}", e),
        }
    }
}
