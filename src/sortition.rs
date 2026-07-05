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
}
