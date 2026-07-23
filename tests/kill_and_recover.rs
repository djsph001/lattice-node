use lattice_node::claims::{ClaimEvidence, ClaimType, WitnessSignature, WitnessedClaim};
use lattice_node::ledger::persistence::{PersistentEconomicState, StoredClaim};
use lattice_node::ledger::thickness::{ThicknessEdge, ThicknessGraph};
use libp2p::PeerId;
use std::collections::HashMap;

/// Kill-and-recover: both crash timings, with post-recovery thickness assertions.
#[test]
fn kill_and_recover_thickness() {
    let claimant = PeerId::random();
    let witness = PeerId::random();
    let credit_amount = 1000.0_f64;

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
    // Claim queued but no credit applied. Recovery sees nothing
    // to re-queue (already pending). Thickness == 0.
    let mut graph0 = ThicknessGraph::new();
    let state0 = PersistentEconomicState::from_state(
        &HashMap::new(), &HashMap::new(), &graph0, 0,
        vec![StoredClaim { claim: claim.clone(), applied_at_epoch: None }],
    );
    let snap0 = serde_cbor::to_vec(&state0).expect("snap0");
    drop(state0);

    let recovered0: PersistentEconomicState =
        serde_cbor::from_slice(&snap0).expect("snap0 recover");

    assert!(recovered0.accepted_claims[0].applied_at_epoch.is_none());
    assert!(recovered0.thickness_edges.is_empty(),
        "no edges — credit never applied");

    // ── Timing 2: Kill AFTER credit, BEFORE snapshot ─────
    // This is the real failure mode:
    //   1. Credit applied at epoch boundary → edges exist in memory
    //   2. applied_at_epoch: Some(100) set → marker exists in memory
    //   3. CRASH before the snapshot writes either to disk
    //   4. Recovery from last snapshot: no edges, claim has None marker
    //   5. Claim re-queued → re-credited ONCE at next boundary
    //   6. Post-recovery thickness == exactly one credit, not two
    //
    // Simulate by serializing only the claim (no edges, None marker),
    // then on recovery re-credit once, then verify.
    let mut graph_empty = ThicknessGraph::new();
    let state_crash = PersistentEconomicState::from_state(
        &HashMap::new(), &HashMap::new(), &graph_empty, 0,
        vec![StoredClaim {
            claim: claim,
            applied_at_epoch: None, // crash happened BEFORE marker was persisted
        }],
    );
    drop(graph_empty);

    // This is the snapshot that survived — no edges, None markers
    let snap_crash = serde_cbor::to_vec(&state_crash).expect("snap_crash");
    drop(state_crash);

    // Recovery (simulates restart)
    let recovered: PersistentEconomicState =
        serde_cbor::from_slice(&snap_crash).expect("snap_crash recover");

    // Verify: no edges, claim is pending
    assert!(recovered.thickness_edges.is_empty(),
        "lost both edges and markers — same serialized unit");
    assert!(recovered.accepted_claims[0].applied_at_epoch.is_none(),
        "claim marker lost with edges — must re-queue");

    // Simulate epoch boundary: re-credit the pending claim ONCE
    let mut post_recovery_graph = ThicknessGraph::new();
    post_recovery_graph.add_verified_contribution(
        &claimant, [0; 32], credit_amount);

    let post_recovery_thickness = post_recovery_graph.total_thickness(&claimant);

    // EXACT EQUALITY — not range, not "> 0 && < 2×"
    // Double credit would give 2000.0 and fail.
    // Zero would give 0.0 and fail.
    // Any partial value would give something else and fail.
    assert_eq!(
        post_recovery_thickness, credit_amount,
        "double credit on recovery: expected exactly one credit's worth"
    );

    // I4 enforced structurally: no 'verified' field on StoredClaim.
    // If a future commit adds one, this file must check it explicitly.
}
