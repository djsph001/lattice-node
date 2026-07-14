// ── Sybil swarm distribution measurement ───────────────────────────
//
// This is NOT a pass/fail unit test. It's a measurement harness that
// answers: "At what swarm size does the floor-clamp let Sybils accumulate
// meaningful combined selection share?"
//
// The mechanism under test: select_weighted_witness_panel with
// FLOOR_WEIGHT = 0.01. Every cheap key gets 0.01 floor weight.
// N Sybils = N × 0.01 combined. One honest node has earned thickness T.
//
// The question: as N grows, does N × 0.01 ever compete meaningfully
// with T for 5-person witness panel seats?
//
// Run with: cargo test --lib swarm_distribution -- --nocapture

#[cfg(test)]
mod swarm_tests {
    use crate::sortition::{select_weighted_witness_panel, FLOOR_WEIGHT};
    use libp2p::PeerId;

    /// Number of random seeds to draw per configuration.
    const DRAWS: usize = 2000;

    /// Run one configuration and return the fraction of 5 seats
    /// captured by Sybils on average.
    fn measure_sybil_share(honest_thickness: f64, sybil_count: usize) -> (f64, f64, f64) {
        let honest = PeerId::random();
        let sybils: Vec<PeerId> = (0..sybil_count).map(|_| PeerId::random()).collect();

        // Build the weighted pool
        let mut pool: Vec<(PeerId, f64)> = vec![(honest, honest_thickness)];
        for s in &sybils {
            pool.push((*s, 0.0)); // floor-clamped to FLOOR_WEIGHT inside select_weighted
        }

        let exclusions: Vec<PeerId> = vec![];

        let mut total_sybil_seats = 0usize;
        let mut draws_with_honest = 0usize;

        // Use different seeds for each draw to sample the distribution
        let base_seed = blake3::hash(b"swarm-distribution-measurement");

        for i in 0..DRAWS {
            // Derive a unique seed for each draw
            let mut seed_bytes = base_seed.as_bytes().to_vec();
            seed_bytes.extend_from_slice(&(i as u64).to_be_bytes());
            let seed = hex::encode(blake3::hash(&seed_bytes).as_bytes());

            let panel = select_weighted_witness_panel(&seed[..16], &pool, &exclusions);

            let sybil_seats = panel.iter().filter(|p| sybils.contains(p)).count();
            total_sybil_seats += sybil_seats;

            if panel.contains(&honest) {
                draws_with_honest += 1;
            }
        }

        let avg_sybil_seats = total_sybil_seats as f64 / DRAWS as f64;
        let sybil_seat_share = avg_sybil_seats / 5.0;
        let honest_appearance_rate = draws_with_honest as f64 / DRAWS as f64;

        // Theoretical combined floor weight share
        let floor_total = sybil_count as f64 * FLOOR_WEIGHT;
        let total_weight = honest_thickness + floor_total;
        let floor_weight_share = floor_total / total_weight;

        (sybil_seat_share, honest_appearance_rate, floor_weight_share)
    }

    #[test]
    fn swarm_distribution_measurement() {
        eprintln!();
        eprintln!("═══════════════════════════════════════════════════════════════");
        eprintln!("  SYBIL SWARM DISTRIBUTION MEASUREMENT");
        eprintln!("  Floor clamp = {:.4}, Panel size = 5, Draws/config = {}",
            FLOOR_WEIGHT, DRAWS);
        eprintln!("═══════════════════════════════════════════════════════════════");
        eprintln!();
        eprintln!("  SCENARIO A: One honest node vs. N Sybils");
        eprintln!("  (Weighted selection without replacement: honest = max 1 seat)");
        eprintln!();
        eprintln!("{:>10} │ {:>8} │ {:>12} │ {:>14} │ {:>12}", 
            "Sybils N", "Honest T", "Floor wt shr", "Sybil seat shr", "Honest in %");
        eprintln!("{:-<10}─┼─{:-<8}─┼─{:-<12}─┼─{:-<14}─┼─{:-<12}", "", "", "", "", "");

        let t_values = [10.0, 50.0, 100.0, 500.0, 1000.0];
        let n_values = [10, 50, 100, 500, 1000, 5000, 10000];

        for &t in &t_values {
            for &n in &n_values {
                let (sybil_share, honest_rate, floor_share) = measure_sybil_share(t, n);
                eprintln!(
                    "{:>10} │ {:>8.0} │ {:>12.4} │ {:>14.4} │ {:>12.4}",
                    n, t, floor_share, sybil_share, honest_rate
                );
            }
            eprintln!("{:-<10}─┼─{:-<8}─┼─{:-<12}─┼─{:-<14}─┼─{:-<12}", "", "", "", "", "");
        }

        // ── Scenario B: M honest nodes, N Sybils ──────────────
        eprintln!();
        eprintln!("  SCENARIO B: M honest nodes (each T/M thickness) vs. N Sybils");
        eprintln!("  (More realistic — sparse mesh with multiple honest peers)");
        eprintln!();
        eprintln!("{:>4} hst │ {:>6} Syb │ {:>8} T │ {:>12} │ {:>14} │ {:>12} │ {:>14}", 
            "M", "N", "total", "Floor wt shr", "Sybil seat shr", "Hst seat shr", "Hst in %");
        eprintln!("{:-<4}─┼─{:-<6}─┼─{:-<8}─┼─{:-<12}─┼─{:-<14}─┼─{:-<12}─┼─{:-<14}", "", "", "", "", "", "", "");

        for &m in &[3, 5, 10, 20] {
            let total_thickness = 100.0;
            for &n in &[10, 100, 1000, 10000] {
                let (sybil_share, floor_share, hst_seat_share, any_hst_rate) = 
                    measure_multi_honest(m, n, 100.0);
                eprintln!(
                    "{:>4} │ {:>6} │ {:>8.0} │ {:>12.4} │ {:>14.4} │ {:>12.4} │ {:>12.4}",
                    m, n, total_thickness,
                    floor_share, sybil_share, hst_seat_share, any_hst_rate
                );
            }
            eprintln!("{:-<4}─┼─{:-<6}─┼─{:-<8}─┼─{:-<12}─┼─{:-<14}─┼─{:-<12}─┼─{:-<14}", "", "", "", "", "", "", "");
        }

        // ── Scenario C: Thickness-weighted voting power ─────
        eprintln!();
        eprintln!("  SCENARIO C: Thickness-weighted VOTING POWER within panel");
        eprintln!("  (Panel seats are one-per-node. But votes are");
        eprintln!("   thickness-proportional. 1 honest heavy seat");
        eprintln!("   can outvote 4 Sybil floor-weight seats.)");
        eprintln!();
        eprintln!("  ─── Single honest node ───");
        eprintln!("{:>8} T │ {:>6} Syb │ {:>12} │ {:>14} │ {:>14} │ {:>12}", 
            "Honest", "N", "Hst wt share", "Hst VOTE share", "Sybil vote shr", "Hst in panel");
        eprintln!("{:-<8}─┼─{:-<6}─┼─{:-<12}─┼─{:-<14}─┼─{:-<14}─┼─{:-<12}", "", "", "", "", "", "");

        for &t in &[10.0, 50.0, 100.0, 500.0, 1000.0] {
            for &n in &[10, 100, 1000, 10000] {
                let (hst_wt_share, hst_vote_share, sybil_vote_share, hst_in_panel) =
                    measure_voting_power(1, t, n);
                eprintln!(
                    "{:>8.0} │ {:>6} │ {:>12.4} │ {:>14.4} │ {:>14.4} │ {:>12.4}",
                    t, n, hst_wt_share, hst_vote_share, sybil_vote_share, hst_in_panel
                );
            }
            eprintln!("{:-<8}─┼─{:-<6}─┼─{:-<12}─┼─{:-<14}─┼─{:-<14}─┼─{:-<12}", "", "", "", "", "", "");
        }

        eprintln!();
        eprintln!("  ─── M honest nodes ───");
        eprintln!("{:>4} hst │ {:>6} Syb │ {:>12} │ {:>14} │ {:>14} │ {:>12} │ {:>14}", 
            "M", "N", "Hst wt share", "Hst VOTE share", "Sybil vote shr", "Hst in %", "Hst VETO?");
        eprintln!("{:-<4}─┼─{:-<6}─┼─{:-<12}─┼─{:-<14}─┼─{:-<14}─┼─{:-<12}─┼─{:-<14}", "", "", "", "", "", "", "");

        for &m in &[3, 5, 10] {
            let total_t = 100.0;
            for &n in &[10, 100, 1000, 10000] {
                let (hst_wt_share, hst_vote_share, sybil_vote_share, hst_in_panel) =
                    measure_voting_power(m, total_t, n);
                let veto = if hst_vote_share > 0.667 { "✓ SUPER" } 
                    else if hst_vote_share > 0.50 { "✓ maj" } 
                    else { "✗" };
                eprintln!(
                    "{:>4} │ {:>6} │ {:>12.4} │ {:>14.4} │ {:>14.4} │ {:>12.4} │ {:>14}",
                    m, n, hst_wt_share, hst_vote_share, sybil_vote_share, hst_in_panel, veto
                );
            }
            eprintln!("{:-<4}─┼─{:-<6}─┼─{:-<12}─┼─{:-<14}─┼─{:-<14}─┼─{:-<12}─┼─{:-<14}", "", "", "", "", "", "", "");
        }

        eprintln!();
        eprintln!("  Key finding:");
        eprintln!("    Seat share ≠ vote share. Thickness-weighted voting power");
        eprintln!("    decouples influence from seat count. A single honest node");
        eprintln!("    with T=50 outvotes 10,000 floor-weight Sybils (0.04 total)");
        eprintln!("    despite holding 1 of 5 seats.");
        eprintln!();
        eprintln!("  VETO thresholds:");
        eprintln!("    > 0.667 = supermajority (Sybils can't override)");
        eprintln!("    > 0.500 = simple majority");
        eprintln!("    < 0.500 = Sybil-capturable");
        eprintln!();
        eprintln!("═══════════════════════════════════════════════════════════════");
        eprintln!("  MEASUREMENT COMPLETE");
        eprintln!("═══════════════════════════════════════════════════════════════");
    }

    fn measure_multi_honest(
        honest_count: usize,
        sybil_count: usize,
        total_thickness: f64,
    ) -> (f64, f64, f64, f64) {
        let per_honest = total_thickness / honest_count as f64;
        let honest: Vec<PeerId> = (0..honest_count).map(|_| PeerId::random()).collect();
        let sybils: Vec<PeerId> = (0..sybil_count).map(|_| PeerId::random()).collect();

        let mut pool: Vec<(PeerId, f64)> = honest.iter().map(|h| (*h, per_honest)).collect();
        for s in &sybils {
            pool.push((*s, 0.0));
        }

        let exclusions: Vec<PeerId> = vec![];
        let base_seed = blake3::hash(b"swarm-multi-honest");

        let mut total_sybil_seats = 0usize;
        let mut total_honest_seats = 0usize;
        let mut draws_with_any_honest = 0usize;

        for i in 0..DRAWS {
            let mut seed_bytes = base_seed.as_bytes().to_vec();
            seed_bytes.extend_from_slice(&(i as u64).to_be_bytes());
            let seed = hex::encode(blake3::hash(&seed_bytes).as_bytes());

            let panel = select_weighted_witness_panel(&seed[..16], &pool, &exclusions);

            let sybil_seats = panel.iter().filter(|p| sybils.contains(p)).count();
            let honest_seats = panel.iter().filter(|p| honest.contains(p)).count();
            total_sybil_seats += sybil_seats;
            total_honest_seats += honest_seats;

            if panel.iter().any(|p| honest.contains(p)) {
                draws_with_any_honest += 1;
            }
        }

        let sybil_seat_share = total_sybil_seats as f64 / (DRAWS * 5) as f64;
        let honest_seat_share = total_honest_seats as f64 / (DRAWS * 5) as f64;
        let any_honest_rate = draws_with_any_honest as f64 / DRAWS as f64;

        let floor_total = sybil_count as f64 * FLOOR_WEIGHT;
        let honest_total = honest_count as f64 * per_honest;
        let floor_weight_share = floor_total / (floor_total + honest_total);

        (sybil_seat_share, floor_weight_share, honest_seat_share, any_honest_rate)
    }

    /// Measure thickness-weighted VOTING POWER within a selected panel.
    ///
    /// Seats are one-per-node (weighted selection without replacement).
    /// But VOTES within the panel are thickness-proportional: each panel
    /// member's vote weight = their thickness / total panel thickness.
    ///
    /// Returns: (honest_weight_share_in_pool,
    ///           honest_avg_vote_share,
    ///           sybil_avg_vote_share,
    ///           honest_in_panel_rate)
    fn measure_voting_power(
        honest_count: usize,
        total_honest_thickness: f64,
        sybil_count: usize,
    ) -> (f64, f64, f64, f64) {
        let per_honest = total_honest_thickness / honest_count as f64;
        let honest: Vec<PeerId> = (0..honest_count).map(|_| PeerId::random()).collect();
        let sybils: Vec<PeerId> = (0..sybil_count).map(|_| PeerId::random()).collect();

        let mut pool: Vec<(PeerId, f64)> = honest.iter().map(|h| (*h, per_honest)).collect();
        for s in &sybils {
            pool.push((*s, 0.0)); // floor-clamped inside select_weighted
        }

        let exclusions: Vec<PeerId> = vec![];
        let base_seed = blake3::hash(b"voting-power-measurement");

        // Weight share in the full pool (theoretical)
        let floor_total = sybil_count as f64 * FLOOR_WEIGHT;
        let honest_total = honest_count as f64 * per_honest;
        let hst_weight_share = honest_total / (honest_total + floor_total);

        let mut total_honest_vote = 0.0;
        let mut total_sybil_vote = 0.0;
        let mut draws_honest_in_panel = 0usize;

        for i in 0..DRAWS {
            let mut seed_bytes = base_seed.as_bytes().to_vec();
            seed_bytes.extend_from_slice(&(i as u64).to_be_bytes());
            let seed = hex::encode(blake3::hash(&seed_bytes).as_bytes());

            let panel = select_weighted_witness_panel(&seed[..16], &pool, &exclusions);

            // Build a lookup: PeerId → raw thickness (before floor-clamp)
            let raw_weights: std::collections::HashMap<PeerId, f64> = pool
                .iter()
                .map(|(p, w)| (*p, *w))
                .collect();

            // Compute thickness-weighted voting power within the panel
            let mut panel_honest_vote = 0.0;
            let mut panel_sybil_vote = 0.0;
            let mut panel_has_honest = false;

            for peer_id in &panel {
                let raw_w = raw_weights.get(peer_id).copied().unwrap_or(0.0);
                let weight = raw_w.max(FLOOR_WEIGHT);
                if honest.contains(peer_id) {
                    panel_honest_vote += weight;
                    panel_has_honest = true;
                } else {
                    panel_sybil_vote += weight;
                }
            }

            let panel_total = panel_honest_vote + panel_sybil_vote;
            if panel_total > 0.0 {
                total_honest_vote += panel_honest_vote / panel_total;
                total_sybil_vote += panel_sybil_vote / panel_total;
            }

            if panel_has_honest {
                draws_honest_in_panel += 1;
            }
        }

        let hst_vote_share = total_honest_vote / DRAWS as f64;
        let sybil_vote_share = total_sybil_vote / DRAWS as f64;
        let hst_in_panel = draws_honest_in_panel as f64 / DRAWS as f64;

        (hst_weight_share, hst_vote_share, sybil_vote_share, hst_in_panel)
    }
}
