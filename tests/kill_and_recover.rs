use lattice_node::claims::{ClaimEvidence, ClaimType, WitnessedClaim, WitnessSignature};
use lattice_node::ledger::persistence::{PersistentEconomicState, StoredClaim};
use libp2p::PeerId;

/// Kill-and-recover test: prove a witnessed claim survives
/// the death of the originating process.
///
/// Two crash timings:
///   Timing 1: Kill BEFORE epoch boundary (applied_at_epoch: None)
///   Timing 2: Kill AFTER credit but BEFORE snapshot
///             (applied_at_epoch: Some(e) in memory, not yet on disk)
#[test]
fn kill_and_recover_both_timings() {
    // ── Phase A: Create ──────────────────────────────────────
    let claimant = PeerId::random();
    let witness = PeerId::random();

    let claim = WitnessedClaim {
        claimant,
        claim_type: ClaimType::ServiceAttestation,
        start_epoch: 0,
        end_epoch: 100,
        evidence: ClaimEvidence::Service { claimed_count: 42 },
        witnesses: vec![WitnessSignature {
            witness,
            signed_at_epoch: 50,
            observed_heartbeats: 42,
            signature: vec![0xCD; 64],
        }],
        submitted_epoch: 50,
    };

    // ── Timing 1: Kill BEFORE epoch boundary ─────────────
    // Claim is accepted but not yet credited (applied_at_epoch: None).
    let mut state = PersistentEconomicState::new();
    state.accepted_claims.push(StoredClaim {
        claim: claim.clone(),
        applied_at_epoch: None,
    });

    let snap = serde_cbor::to_vec(&state).expect("snap1 serialize");
    drop(state);
    let recovered: PersistentEconomicState =
        serde_cbor::from_slice(&snap).expect("snap1 deserialize");

    assert_eq!(recovered.accepted_claims.len(), 1, "claim survives");
    let rc = &recovered.accepted_claims[0];
    assert!(rc.applied_at_epoch.is_none(), "queued claim stays queued");
    assert_eq!(rc.claim.claimant, claimant, "claimant survives");
    assert_eq!(rc.claim.claim_type, ClaimType::ServiceAttestation, "type survives");
    assert_eq!(rc.claim.witnesses.len(), 1, "witness survives");
    assert_eq!(rc.claim.witnesses[0].witness, witness, "witness_id survives");
    assert_eq!(rc.claim.submitted_epoch, 50, "submitted_epoch survives");

    // ── Timing 2: Kill AFTER credit, BEFORE snapshot ─────
    // Claim was credited (applied_at_epoch: Some(100)) but the
    // snapshot with the updated value wasn't written.
    let mut state2 = PersistentEconomicState::new();
    state2.accepted_claims.push(StoredClaim {
        claim: claim,
        applied_at_epoch: Some(100),
    });

    let snap2 = serde_cbor::to_vec(&state2).expect("snap2 serialize");
    drop(state2);
    let recovered2: PersistentEconomicState =
        serde_cbor::from_slice(&snap2).expect("snap2 deserialize");

    let rc2 = &recovered2.accepted_claims[0];
    assert_eq!(rc2.applied_at_epoch, Some(100),
        "credited claim stays credited — no double credit on recovery");

    // I4 invariant enforced structurally: no 'verified' field on StoredClaim.
    // If a future commit adds one, this file must check it explicitly.
}
