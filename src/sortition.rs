// Phase 7 — Multi-Sig Witness Sortition Engine
//
// Deterministic selection of a 5-person Witness panel from the
// active peer registry, using a Blake3-seeded Fisher-Yates shuffle.
// Every node running this code independently arrives at the same
// panel when presented with the same witness_seed.
//
// TCP v0.1.0 §3.2: "A 5-person sortition panel is selected
// deterministically via a Blake3-seeded Fisher-Yates shuffle of the
// local registry. Panel members who participated in the last 3
// escalations are excluded."

use libp2p::PeerId;
use tracing::debug;

/// Select a deterministic 5-person Witness panel from the peer pool.
///
/// # Arguments
/// * `witness_seed` — The witness_seed from the ImpactCertificate (16 hex chars).
/// * `peer_pool` — All available peers (excluding self).
/// * `exclusions` — Peers excluded due to recent escalation participation.
///
/// # Returns
/// Up to 5 PeerIds. Fewer if the pool (after exclusions) is smaller.
pub fn select_witness_panel(
    witness_seed: &str,
    peer_pool: &[PeerId],
    exclusions: &[PeerId],
) -> Vec<PeerId> {
    // Filter out excluded peers
    let mut pool: Vec<PeerId> = peer_pool
        .iter()
        .filter(|p| !exclusions.contains(p))
        .cloned()
        .collect();

    // If the pool is too small, return whatever we have
    if pool.len() <= 5 {
        debug!(
            pool_size = pool.len(),
            "[sortition] Pool too small for full 5-person panel — returning all eligible peers"
        );
        return pool;
    }

    // Seed the RNG deterministically from the witness_seed
    // We chain Blake3 hashes to generate successive u64 indices
    let mut seed = blake3::hash(witness_seed.as_bytes());
    let mut panel = Vec::with_capacity(5);

    while panel.len() < 5 {
        // Convert first 8 bytes of hash to u64 for index selection
        let hash_bytes = seed.as_bytes();
        let index = u64::from_be_bytes([
            hash_bytes[0], hash_bytes[1], hash_bytes[2], hash_bytes[3],
            hash_bytes[4], hash_bytes[5], hash_bytes[6], hash_bytes[7],
        ]) as usize % pool.len();

        // Fisher-Yates: swap-remove the selected peer
        let selected = pool.swap_remove(index);
        panel.push(selected);

        // Advance the seed for the next round
        seed = blake3::hash(seed.as_bytes());
    }

    debug!(
        panel = ?panel.iter().map(|p| p.to_string()).collect::<Vec<_>>(),
        "[sortition] Witness panel selected deterministically"
    );

    panel
}

/// Check whether the local node is on the selected Witness panel.
pub fn is_local_witness(panel: &[PeerId], local_id: &PeerId) -> bool {
    panel.contains(local_id)
}

/// Select a deterministic 5-person Witness panel, weighted by thickness.
///
/// Each candidate's selection probability is proportional to their weight.
/// Weights are floor-clamped to `floor_weight` so new honest nodes retain
/// a nonzero chance of selection (the exploration epsilon from the mycelial
/// routing design).
///
/// # Arguments
///
pub fn select_weighted_witness_panel(
    witness_seed: &str,
    peer_pool: &[(PeerId, f64)],
    exclusions: &[PeerId],
    floor_weight: f64,
) -> Vec<PeerId> {
    // Filter out excluded peers and apply floor weight
    let mut pool: Vec<(PeerId, f64)> = peer_pool
        .iter()
        .filter(|(p, _)| !exclusions.contains(p))
        .map(|(p, w)| (*p, w.max(floor_weight)))
        .collect();

    if pool.is_empty() {
        return vec![];
    }

    if pool.len() <= 5 {
        debug!(
            pool_size = pool.len(),
            "[sortition] Weighted pool too small for full 5-person panel — returning all eligible peers"
        );
        return pool.into_iter().map(|(p, _)| p).collect();
    }

    // Weighted selection: use cumulative weights + Blake3-derived random number
    // to pick proportional to weight, then swap-remove (Fisher-Yates style)
    let mut seed = blake3::hash(witness_seed.as_bytes());
    let mut panel = Vec::with_capacity(5);

    while panel.len() < 5 && !pool.is_empty() {
        // Compute cumulative weights
        let total_weight: f64 = pool.iter().map(|(_, w)| w).sum();
        if total_weight <= 0.0 {
            break;
        }

        // Derive a random f64 in [0, 1) from the seed
        let hash_bytes = seed.as_bytes();
        let rand_val = u64::from_be_bytes([
            hash_bytes[0],
            hash_bytes[1],
            hash_bytes[2],
            hash_bytes[3],
            hash_bytes[4],
            hash_bytes[5],
            hash_bytes[6],
            hash_bytes[7],
        ]) as f64
            / u64::MAX as f64;

        let threshold = rand_val * total_weight;
        let mut cumulative = 0.0;
        let mut selected_idx = 0;

        for (i, (_, w)) in pool.iter().enumerate() {
            cumulative += w;
            if cumulative >= threshold {
                selected_idx = i;
                break;
            }
        }

        // Swap-remove the selected peer (Fisher-Yates pattern)
        let (selected, _) = pool.swap_remove(selected_idx);
        panel.push(selected);

        // Advance the seed
        seed = blake3::hash(seed.as_bytes());
    }

    debug!(
        panel = ?panel.iter().map(|p| p.to_string()).collect::<Vec<_>>(),
        "[sortition] Weighted witness panel selected deterministically"
    );

    panel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_panel_selection() {
        let peers: Vec<PeerId> = (0..20).map(|_| PeerId::random()).collect();
        let exclusions: Vec<PeerId> = vec![];

        let panel_a = select_witness_panel("b644ae83dae8edc6", &peers, &exclusions);
        let panel_b = select_witness_panel("b644ae83dae8edc6", &peers, &exclusions);

        assert_eq!(panel_a, panel_b, "Same seed must produce same panel");
    }

    #[test]
    fn test_different_seeds_produce_different_panels() {
        let peers: Vec<PeerId> = (0..20).map(|_| PeerId::random()).collect();
        let exclusions: Vec<PeerId> = vec![];

        let panel_a = select_witness_panel("b644ae83dae8edc6", &peers, &exclusions);
        let panel_b = select_witness_panel("c755bf94ebe9fde7", &peers, &exclusions);

        // Should be different with high probability (not 100% guaranteed
        // with random peers but astronomically unlikely to collide)
        assert_ne!(panel_a, panel_b, "Different seeds should produce different panels");
    }

    #[test]
    fn test_exclusions_are_respected() {
        let peers: Vec<PeerId> = (0..20).map(|_| PeerId::random()).collect();
        // Exclude the first 10 peers
        let exclusions: Vec<PeerId> = peers[..10].to_vec();

        let panel = select_witness_panel("b644ae83dae8edc6", &peers, &exclusions);

        for excluded in &exclusions {
            assert!(
                !panel.contains(excluded),
                "Excluded peer must not appear in panel"
            );
        }
    }

    #[test]
    fn test_small_pool_returns_available() {
        let peers: Vec<PeerId> = (0..3).map(|_| PeerId::random()).collect();
        let exclusions: Vec<PeerId> = vec![];

        let panel = select_witness_panel("b644ae83dae8edc6", &peers, &exclusions);
        assert_eq!(panel.len(), 3, "Small pool should return all available peers");
    }

    #[test]
    fn test_panel_size_is_at_most_5() {
        let peers: Vec<PeerId> = (0..50).map(|_| PeerId::random()).collect();
        let exclusions: Vec<PeerId> = vec![];

        let panel = select_witness_panel("b644ae83dae8edc6", &peers, &exclusions);
        assert_eq!(panel.len(), 5, "Panel must be exactly 5 when pool is sufficient");
    }

    // ── Weighted sortition tests ──────────────────────────

    #[test]
    fn test_weighted_deterministic() {
        let peers: Vec<(PeerId, f64)> = (0..20)
            .map(|i| (PeerId::random(), (i + 1) as f64))
            .collect();
        let exclusions: Vec<PeerId> = vec![];

        let panel_a = select_weighted_witness_panel("b644ae83dae8edc6", &peers, &exclusions, 0.01);
        let panel_b = select_weighted_witness_panel("b644ae83dae8edc6", &peers, &exclusions, 0.01);

        assert_eq!(panel_a, panel_b, "Same seed must produce same panel");
        assert_eq!(panel_a.len(), 5, "Must select 5 from sufficient pool");
    }

    #[test]
    fn test_weighted_floor_clamp_gives_new_nodes_a_chance() {
        // Create a pool with one heavy node (1000.0) and 19 floor-weight nodes (0.0).
        // The floor-clamp should give the floor-weight nodes FLOOR_WEIGHT (0.01) each.
        let mut peers: Vec<(PeerId, f64)> = vec![];
        peers.push((PeerId::random(), 1000.0));
        for _ in 0..19 {
            peers.push((PeerId::random(), 0.0));
        }
        let exclusions: Vec<PeerId> = vec![];

        let panel = select_weighted_witness_panel("b644ae83dae8edc6", &peers, &exclusions, 0.01);
        assert_eq!(panel.len(), 5, "Must select 5 from pool of 20");

        // The heavy node should appear (it dominates the weight), but it shouldn't
        // be EVERY selection — floor-weight nodes can still be picked.
        let heavy = peers[0].0;
        let heavy_count = panel.iter().filter(|p| **p == heavy).count();
        assert!(heavy_count >= 1, "Heavy node should appear at least once");
        // With 1000.0 vs 19 × 0.01 = 0.19 total floor weight, the heavy node
        // is overwhelmingly likely to be selected most/all of the time.
        // This test just verifies the mechanism runs.
    }

    #[test]
    fn test_weighted_exclusions() {
        let peers: Vec<(PeerId, f64)> = (0..20)
            .map(|_| (PeerId::random(), 1.0))
            .collect();
        let exclusions: Vec<PeerId> = peers.iter().take(10).map(|(p, _)| *p).collect();

        let panel = select_weighted_witness_panel("b644ae83dae8edc6", &peers, &exclusions, 0.01);

        for excluded in &exclusions {
            assert!(
                !panel.contains(excluded),
                "Excluded peer must not appear in panel"
            );
        }
    }

    #[test]
    fn test_weighted_empty_pool() {
        let peers: Vec<(PeerId, f64)> = vec![];
        let panel = select_weighted_witness_panel("42", &peers, &[], 0.01);
        assert!(panel.is_empty());
    }

    #[test]
    fn small_pool_includes_local_peer() {
        // The discriminating case: pool ≤ 5 returns ALL peers including
        // the local one. is_local_witness MUST return true when the local
        // peer is in the returned panel.
        let local = PeerId::random();
        let a = PeerId::random();
        let b = PeerId::random();

        let pool = vec![(a, 1.0), (b, 1.0), (local, 1.0)];
        let panel = select_weighted_witness_panel("test-seed", &pool, &[], 0.01);

        assert_eq!(panel.len(), 3, "small pool must return all 3 peers");
        assert!(
            is_local_witness(&panel, &local),
            "is_local_witness MUST return true when local peer is in the pool"
        );
    }
}

