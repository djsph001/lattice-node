use lattice_node::claims::{ClaimEvidence, ClaimType, WitnessSignature, WitnessedClaim};
use lattice_node::ledger::persistence::{PersistentEconomicState, StoredClaim};
use lattice_node::ledger::thickness::{ThicknessEdge, ThicknessGraph};
use lattice_node::ledger::types::DigitalUtilityUnit;
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
    // Credit applied to graph, edges + markers serialized atomically.
    let mut graph2 = ThicknessGraph::new();
    graph2.add_verified_contribution(&claimant, [0; 32], credit_amount);

    let state2 = PersistentEconomicState::from_state(
        &HashMap::new(), &HashMap::new(), &graph2, 0,
        vec![StoredClaim { claim, applied_at_epoch: Some(100) }],
    );
    let snap2 = serde_cbor::to_vec(&state2).expect("snap2");
    drop(state2);
    drop(graph2);

    // Simulate restart: deserialize from snapshot
    let recovered2: PersistentEconomicState =
        serde_cbor::from_slice(&snap2).expect("snap2 recover");

    // Rebuild thickness graph from recovered CBOR-encoded edges
    let mut recovered_graph = ThicknessGraph::new();
    let decoded_edges: HashMap<String, Vec<ThicknessEdge>> = recovered2.thickness_edges
        .into_iter()
        .map(|(peer_str, encoded_vec)| {
            let decoded: Vec<ThicknessEdge> = encoded_vec
                .into_iter()
                .filter_map(|bytes| serde_cbor::from_slice(&bytes).ok())
                .collect();
            (peer_str, decoded)
        })
        .collect();
    recovered_graph.import_edges(decoded_edges);

    // Post-recovery thickness assertion — exact match, not range
    let post_recovery_thickness = recovered_graph.total_thickness(&claimant);
    assert!(
        (post_recovery_thickness - credit_amount).abs() < 0.001,
        "post-recovery thickness = {} (expected {}) — double credit would be {}",
        post_recovery_thickness, credit_amount, credit_amount * 2.0
    );

    // Claim marker also survived (same CBOR unit)
    assert_eq!(recovered2.accepted_claims[0].applied_at_epoch, Some(100));

    // I4 enforced structurally: no 'verified' field on StoredClaim.
}
