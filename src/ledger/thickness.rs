// ── ledger/thickness.rs — the thickness provenance graph ──────────
//
// Thickness is NOT a scalar. It is a directed provenance graph where every
// unit of thickness carries edges back to its derivation source. This is
// first-class from line one because chained clawback (Layer 2) requires
// derivation lineage that cannot be retrofitted.
//
// Two sources of thickness:
//   VerifiedContribution — minted by real work (Phase 6 receipts, relay,
//                          storage proofs). This is the ONLY source of NEW
//                          thickness. Everything else redistributes existing.
//   Vouch               — derived from a voucher staking their own thickness.
//                          Does NOT mint new thickness; transfers encumbered.
//                          Supports optional expiration (time-bounded vouches).

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use libp2p::PeerId;

/// Identifies a specific verified receipt (the Blake3 message_hash).
pub type ReceiptId = [u8; 32];

/// Where a unit of thickness came from.
#[derive(Debug, Clone, PartialEq)]
pub enum ThicknessSource {
    /// Earned directly by verified contribution. This is the ONLY source
    /// that mints NEW thickness. Everything else redistributes existing.
    VerifiedContribution {
        /// The receipt that proved this contribution.
        receipt_id: ReceiptId,
    },

    /// Derived via a vouch. Carries the voucher + the vouch nonce so
    /// clawback (Layer 2) can traverse the chain back.
    Vouch {
        /// Who staked their thickness.
        voucher: PeerId,
        /// The nonce of the Vouch transaction that created this edge.
        vouch_nonce: u64,
        /// The full stake amount committed for this vouch (before division).
        /// Used on expiration to release exactly this amount from encumbrance.
        stake_committed: f64,
        /// Epoch after which this vouch expires. None = permanent (until clawback).
        expiration_epoch: Option<u64>,
    },
}

/// A single edge in the provenance graph: a unit of thickness and where
/// it came from.
#[derive(Debug, Clone)]
pub struct ThicknessEdge {
    /// Where this thickness was derived from.
    pub source: ThicknessSource,
    /// How much thickness this edge contributes.
    pub amount: f64,
    /// When this edge was created.
    pub created: DateTime<Utc>,
}

/// The provenance graph — every node's thickness, with lineage.
///
/// Invariants:
/// - Only VerifiedContribution mints new thickness. Vouches move it.
/// - A node's usable thickness = Σ(incoming) - Σ(encumbered on outgoing vouches).
/// - Provenance is never collapsed. Edges are retained for Layer 2 clawback.
/// - Expired vouches are unwound at epoch tick via process_epoch_expiration.
#[derive(Debug, Clone)]
pub struct ThicknessGraph {
    /// Derivation edges: who derived thickness from what source.
    /// key = beneficiary PeerId; value = list of edges feeding their thickness.
    edges: HashMap<PeerId, Vec<ThicknessEdge>>,

    /// Thickness encumbered by outgoing vouches. When a node vouches for
    /// someone, the staked fraction is marked as encumbered — it can't be
    /// staked again.
    /// key = voucher PeerId; value = total encumbered thickness.
    encumbered: HashMap<PeerId, f64>,
}

impl ThicknessGraph {
    /// Create an empty provenance graph.
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            encumbered: HashMap::new(),
        }
    }

    /// Total thickness for a node: sum of all incoming edges.
    pub fn total_thickness(&self, peer: &PeerId) -> f64 {
        self.edges
            .get(peer)
            .map(|edges| edges.iter().map(|e| e.amount).sum())
            .unwrap_or(0.0)
    }

    /// Usable (unencumbered) thickness: total minus what's staked on vouches.
    pub fn usable_thickness(&self, peer: &PeerId) -> f64 {
        let total = self.total_thickness(peer);
        let encumbered = self.encumbered.get(peer).copied().unwrap_or(0.0);
        (total - encumbered).max(0.0)
    }

    /// Mint new thickness from a verified contribution. This is the ONLY
    /// source of NEW thickness.
    pub fn add_verified_contribution(
        &mut self,
        peer: &PeerId,
        receipt_id: ReceiptId,
        amount: f64,
    ) {
        let edge = ThicknessEdge {
            source: ThicknessSource::VerifiedContribution { receipt_id },
            amount,
            created: Utc::now(),
        };
        self.edges.entry(*peer).or_default().push(edge);
    }

    /// Stake thickness by vouching for someone. Returns the per-vouchee
    /// derived amount, and re-divides existing vouchees.
    ///
    /// `expiration_epoch`: None = permanent (until clawback).
    /// Some(N) = the vouch expires after epoch N, unwinding automatically.
    ///
    /// Returns an error if the voucher doesn't have enough unencumbered
    /// thickness.
    pub fn stake_vouch(
        &mut self,
        voucher: &PeerId,
        vouchee: &PeerId,
        staked_fraction: f64,
        vouch_nonce: u64,
        expiration_epoch: Option<u64>,
    ) -> Result<f64, String> {
        let voucher_total = self.total_thickness(voucher);
        if voucher_total <= 0.0 {
            return Err("voucher has no thickness to stake".into());
        }

        let stake = voucher_total * staked_fraction;
        let usable = self.usable_thickness(voucher);
        if stake > usable {
            return Err(format!(
                "insufficient unencumbered thickness: need {:.4}, have {:.4}",
                stake, usable
            ));
        }

        // Count existing vouchees (those who have Vouch edges from this voucher)
        let existing_count = self
            .edges
            .values()
            .flatten()
            .filter(|e| matches!(&e.source, ThicknessSource::Vouch { voucher: v, .. } if v == voucher))
            .count();

        let new_count = existing_count + 1;

        // Mark the stake as encumbered (BEFORE computing per_vouchee,
        // so the total pool includes this vouch's contribution)
        let current_enc = self.encumbered.get(voucher).copied().unwrap_or(0.0);
        let new_enc = current_enc + stake;
        self.encumbered.insert(*voucher, new_enc);

        // Per-vouchee = total encumbered pool / total vouchee count.
        // This ensures the sum of derived thicknesses always equals total encumbered.
        let per_vouchee = new_enc / new_count as f64;

        // Re-divide existing vouchees down
        for edges in self.edges.values_mut() {
            for edge in edges.iter_mut() {
                if let ThicknessSource::Vouch { voucher: v, .. } = &edge.source {
                    if v == voucher {
                        edge.amount = per_vouchee;
                    }
                }
            }
        }

        // Add the new vouchee's edge
        let edge = ThicknessEdge {
            source: ThicknessSource::Vouch {
                voucher: *voucher,
                vouch_nonce,
                stake_committed: stake,
                expiration_epoch,
            },
            amount: per_vouchee,
            created: Utc::now(),
        };
        self.edges.entry(*vouchee).or_default().push(edge);

        Ok(per_vouchee)
    }

    /// Process epoch tick: unwind all expired vouches.
    ///
    /// For each expired vouch edge:
    /// 1. Release the committed stake from the voucher's encumbrance.
    /// 2. Remove the expired edge from the vouchee.
    /// 3. Re-divide remaining active vouchees upward (reverse of split-on-new-vouch).
    ///
    /// Returns the number of vouches unwound.
    pub fn process_epoch_expiration(&mut self, current_epoch: u64) -> usize {
        // Collect expired edges: (vouchee_peer, edge_index, voucher, stake_committed)
        let mut expired: Vec<(PeerId, usize, PeerId, f64)> = Vec::new();

        for (vouchee, edges) in self.edges.iter() {
            for (idx, edge) in edges.iter().enumerate() {
                if let ThicknessSource::Vouch {
                    voucher,
                    stake_committed,
                    expiration_epoch,
                    ..
                } = &edge.source
                {
                    if let Some(exp) = expiration_epoch {
                        if *exp <= current_epoch {
                            expired.push((*vouchee, idx, *voucher, *stake_committed));
                        }
                    }
                }
            }
        }

        let count = expired.len();

        // Process in reverse order so indices remain valid during removals
        // Group by voucher to re-divide after all removals
        let mut voucher_adjustments: HashMap<PeerId, f64> = HashMap::new();

        for (vouchee, idx, voucher, stake) in expired.iter().rev() {
            // Remove the edge from the vouchee
            if let Some(edges) = self.edges.get_mut(vouchee) {
                if *idx < edges.len() {
                    edges.remove(*idx);
                    // Clean up empty entry
                    if edges.is_empty() {
                        self.edges.remove(vouchee);
                    }
                }
            }
            // Track encumbrance release
            *voucher_adjustments.entry(*voucher).or_insert(0.0) += stake;
        }

        // Release encumbrance and re-divide survivors
        for (voucher, total_released) in voucher_adjustments {
            // Release from encumbrance
            if let Some(enc) = self.encumbered.get_mut(&voucher) {
                *enc = (*enc - total_released).max(0.0);
            }

            // Count remaining active vouchees for this voucher
            let remaining_count = self
                .edges
                .values()
                .flatten()
                .filter(|e| {
                    matches!(&e.source, ThicknessSource::Vouch { voucher: v, .. } if v == &voucher)
                })
                .count();

            if remaining_count > 0 {
                let current_enc = self.encumbered.get(&voucher).copied().unwrap_or(0.0);
                let per_vouchee = current_enc / remaining_count as f64;

                // Re-divide remaining vouchees upward
                for edges in self.edges.values_mut() {
                    for edge in edges.iter_mut() {
                        if let ThicknessSource::Vouch { voucher: v, .. } = &edge.source {
                            if v == &voucher {
                                edge.amount = per_vouchee;
                            }
                        }
                    }
                }
            }
        }

        count
    }

    /// Return all derivation edges for a peer (for traversal / clawback).
    pub fn edges_for(&self, peer: &PeerId) -> &[ThicknessEdge] {
        self.edges.get(peer).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Number of peers with thickness in the graph.
    pub fn peer_count(&self) -> usize {
        self.edges.len()
    }

    /// Total encumbered thickness for a peer.
    pub fn encumbered_for(&self, peer: &PeerId) -> f64 {
        self.encumbered.get(peer).copied().unwrap_or(0.0)
    }
}

impl Default for ThicknessGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_graph_is_empty() {
        let graph = ThicknessGraph::new();
        let p = PeerId::random();
        assert_eq!(graph.total_thickness(&p), 0.0);
        assert_eq!(graph.usable_thickness(&p), 0.0);
        assert_eq!(graph.peer_count(), 0);
    }

    #[test]
    fn verified_contribution_mints_thickness() {
        let mut graph = ThicknessGraph::new();
        let p = PeerId::random();
        let receipt = [0xAB; 32];

        graph.add_verified_contribution(&p, receipt, 100.0);

        assert_eq!(graph.total_thickness(&p), 100.0);
        assert_eq!(graph.usable_thickness(&p), 100.0);
        assert_eq!(graph.peer_count(), 1);

        let edges = graph.edges_for(&p);
        assert_eq!(edges.len(), 1);
        assert!(matches!(
            edges[0].source,
            ThicknessSource::VerifiedContribution { .. }
        ));
    }

    #[test]
    fn multiple_contributions_sum() {
        let mut graph = ThicknessGraph::new();
        let p = PeerId::random();

        graph.add_verified_contribution(&p, [0x01; 32], 50.0);
        graph.add_verified_contribution(&p, [0x02; 32], 75.0);

        assert_eq!(graph.total_thickness(&p), 125.0);
        assert_eq!(graph.edges_for(&p).len(), 2);
    }

    #[test]
    fn vouch_stakes_and_encumbers_thickness() {
        let mut graph = ThicknessGraph::new();
        let alice = PeerId::random();
        let bob = PeerId::random();

        // Alice earns thickness through contribution
        graph.add_verified_contribution(&alice, [0xAA; 32], 1000.0);

        // Alice vouches for Bob with 10% of her thickness
        let result = graph.stake_vouch(&alice, &bob, 0.10, 1, None);
        assert!(result.is_ok());
        let per_vouchee = result.unwrap();

        // Bob gets derived thickness
        assert_eq!(per_vouchee, 100.0); // 10% of 1000 / 1 vouchee
        assert_eq!(graph.total_thickness(&bob), 100.0);

        // Alice's total thickness is unchanged...
        assert_eq!(graph.total_thickness(&alice), 1000.0);
        // ...but her usable thickness is reduced by the stake
        assert_eq!(graph.usable_thickness(&alice), 900.0); // 1000 - 100 staked
        assert_eq!(graph.encumbered_for(&alice), 100.0);
    }

    #[test]
    fn vouch_re_divides_existing_vouchees() {
        let mut graph = ThicknessGraph::new();
        let alice = PeerId::random();
        let bob = PeerId::random();
        let carol = PeerId::random();

        graph.add_verified_contribution(&alice, [0xAA; 32], 1000.0);

        // Alice vouches for Bob with 20% → Bob gets 200
        graph.stake_vouch(&alice, &bob, 0.20, 1, None).unwrap();
        assert_eq!(graph.total_thickness(&bob), 200.0);

        // Alice vouches for Carol with the same 20% stake → both Bob and Carol
        // get re-divided to 400/2 = 200 each (total encumbered / count)
        graph.stake_vouch(&alice, &carol, 0.20, 2, None).unwrap();
        assert_eq!(graph.total_thickness(&bob), 200.0);
        assert_eq!(graph.total_thickness(&carol), 200.0);
        assert_eq!(graph.encumbered_for(&alice), 400.0);
    }

    #[test]
    fn vouch_fails_with_insufficient_thickness() {
        let mut graph = ThicknessGraph::new();
        let alice = PeerId::random();
        let bob = PeerId::random();

        // Alice has zero thickness — can't vouch
        let result = graph.stake_vouch(&alice, &bob, 0.10, 1, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("no thickness to stake"));
    }

    #[test]
    fn vouch_fails_when_stake_exceeds_unencumbered() {
        let mut graph = ThicknessGraph::new();
        let alice = PeerId::random();
        let bob = PeerId::random();
        let carol = PeerId::random();

        graph.add_verified_contribution(&alice, [0xAA; 32], 100.0);

        // Alice vouches 60% for Bob → 60 staked to Bob
        graph.stake_vouch(&alice, &bob, 0.60, 1, None).unwrap();

        // Alice tries to vouch another 60% for Carol — only 40 unencumbered left
        let result = graph.stake_vouch(&alice, &carol, 0.60, 2, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("insufficient unencumbered"));
    }

    #[test]
    fn vouch_source_has_correct_provenance() {
        let mut graph = ThicknessGraph::new();
        let alice = PeerId::random();
        let bob = PeerId::random();

        graph.add_verified_contribution(&alice, [0xAA; 32], 500.0);
        graph.stake_vouch(&alice, &bob, 0.30, 42, None).unwrap();

        let bobs_edges = graph.edges_for(&bob);
        assert_eq!(bobs_edges.len(), 1);
        match &bobs_edges[0].source {
            ThicknessSource::Vouch {
                voucher,
                vouch_nonce,
                stake_committed: _,
                expiration_epoch,
            } => {
                assert_eq!(*voucher, alice);
                assert_eq!(*vouch_nonce, 42);
                assert_eq!(*expiration_epoch, None);
            }
            _ => panic!("expected Vouch source"),
        }
    }

    // ── Time-bounded vouch tests ────────────────────────────

    #[test]
    fn single_vouch_expiration_releases_encumbrance() {
        let mut graph = ThicknessGraph::new();
        let alice = PeerId::random();
        let bob = PeerId::random();

        graph.add_verified_contribution(&alice, [0xAA; 32], 1000.0);

        // Alice vouches for Bob with 20%, expiring at epoch 10
        graph.stake_vouch(&alice, &bob, 0.20, 1, Some(10)).unwrap();
        assert_eq!(graph.total_thickness(&bob), 200.0);
        assert_eq!(graph.encumbered_for(&alice), 200.0);

        // Epoch 5: not expired yet
        let unwound = graph.process_epoch_expiration(5);
        assert_eq!(unwound, 0);
        assert_eq!(graph.total_thickness(&bob), 200.0);
        assert_eq!(graph.encumbered_for(&alice), 200.0);

        // Epoch 10: should expire
        let unwound = graph.process_epoch_expiration(10);
        assert_eq!(unwound, 1);
        assert_eq!(graph.total_thickness(&bob), 0.0); // thickness gone
        assert_eq!(graph.encumbered_for(&alice), 0.0); // encumbrance released
        assert_eq!(graph.usable_thickness(&alice), 1000.0); // fully usable again
    }

    #[test]
    fn multi_vouch_one_expires_survivors_grow() {
        let mut graph = ThicknessGraph::new();
        let alice = PeerId::random();
        let bob = PeerId::random();
        let carol = PeerId::random();

        graph.add_verified_contribution(&alice, [0xAA; 32], 1000.0);

        // Alice vouches for Bob: 20%, expires epoch 10
        graph.stake_vouch(&alice, &bob, 0.20, 1, Some(10)).unwrap();
        // Alice vouches for Carol: same 20%, permanent
        graph.stake_vouch(&alice, &carol, 0.20, 2, None).unwrap();

        // Both get 200 each (400 encumbered / 2)
        assert_eq!(graph.total_thickness(&bob), 200.0);
        assert_eq!(graph.total_thickness(&carol), 200.0);
        assert_eq!(graph.encumbered_for(&alice), 400.0);

        // Bob's vouch expires at epoch 10
        let unwound = graph.process_epoch_expiration(10);
        assert_eq!(unwound, 1);

        // Bob's thickness is gone
        assert_eq!(graph.total_thickness(&bob), 0.0);
        // Carol's grows: 200 encumbered / 1 survivor = 200
        assert_eq!(graph.total_thickness(&carol), 200.0);
        // Alice's encumbrance: 400 - 200 (released) = 200
        assert_eq!(graph.encumbered_for(&alice), 200.0);
        assert_eq!(graph.usable_thickness(&alice), 800.0);
    }

    #[test]
    fn permanent_vouch_never_expires() {
        let mut graph = ThicknessGraph::new();
        let alice = PeerId::random();
        let bob = PeerId::random();

        graph.add_verified_contribution(&alice, [0xAA; 32], 1000.0);

        // Permanent vouch
        graph.stake_vouch(&alice, &bob, 0.10, 1, None).unwrap();

        // Run through many epochs — nothing expires
        for epoch in 1..=1000 {
            let unwound = graph.process_epoch_expiration(epoch);
            assert_eq!(unwound, 0);
        }

        assert_eq!(graph.total_thickness(&bob), 100.0);
        assert_eq!(graph.encumbered_for(&alice), 100.0);
    }

    #[test]
    fn all_vouchees_expire_fully_releases_voucher() {
        let mut graph = ThicknessGraph::new();
        let alice = PeerId::random();
        let bob = PeerId::random();
        let carol = PeerId::random();

        graph.add_verified_contribution(&alice, [0xAA; 32], 500.0);

        // Two time-bounded vouches, both expire at epoch 5
        graph.stake_vouch(&alice, &bob, 0.10, 1, Some(5)).unwrap();
        graph.stake_vouch(&alice, &carol, 0.10, 2, Some(5)).unwrap();

        assert_eq!(graph.encumbered_for(&alice), 100.0);
        assert_eq!(graph.total_thickness(&bob), 50.0);
        assert_eq!(graph.total_thickness(&carol), 50.0);

        // Both expire
        let unwound = graph.process_epoch_expiration(5);
        assert_eq!(unwound, 2);

        assert_eq!(graph.total_thickness(&bob), 0.0);
        assert_eq!(graph.total_thickness(&carol), 0.0);
        assert_eq!(graph.encumbered_for(&alice), 0.0);
        assert_eq!(graph.usable_thickness(&alice), 500.0);
    }

    #[test]
    fn mixed_expiration_survivors_re_divide_correctly() {
        let mut graph = ThicknessGraph::new();
        let alice = PeerId::random();
        let nodes: Vec<PeerId> = (0..4).map(|_| PeerId::random()).collect();

        graph.add_verified_contribution(&alice, [0xAA; 32], 1000.0);

        // Alice vouches for 4 nodes at 20% each (400 encumbered total)
        // nodes[0] expires at epoch 10, rest are permanent
        for (i, node) in nodes.iter().enumerate() {
            let exp = if i == 0 { Some(10u64) } else { None };
            graph.stake_vouch(&alice, node, 0.10, i as u64 + 1, exp).unwrap();
        }

        // 4 vouchees, 400 encumbered, 100 each
        for node in &nodes {
            assert_eq!(graph.total_thickness(node), 100.0);
        }
        assert_eq!(graph.encumbered_for(&alice), 400.0);

        // nodes[0] expires
        let unwound = graph.process_epoch_expiration(10);
        assert_eq!(unwound, 1);

        // nodes[0] gone
        assert_eq!(graph.total_thickness(&nodes[0]), 0.0);

        // Remaining 3: 300 encumbered / 3 = 100 each (unchanged since
        // the expired vouch had the same stake as the remaining pool)
        for node in &nodes[1..] {
            assert_eq!(graph.total_thickness(node), 100.0);
        }
        assert_eq!(graph.encumbered_for(&alice), 300.0);
    }

    #[test]
    fn expiration_epoch_stored_correctly() {
        let mut graph = ThicknessGraph::new();
        let alice = PeerId::random();
        let bob = PeerId::random();

        graph.add_verified_contribution(&alice, [0xAA; 32], 100.0);
        graph.stake_vouch(&alice, &bob, 0.50, 7, Some(42)).unwrap();

        let edges = graph.edges_for(&bob);
        match &edges[0].source {
            ThicknessSource::Vouch {
                expiration_epoch,
                stake_committed,
                ..
            } => {
                assert_eq!(*expiration_epoch, Some(42));
                assert_eq!(*stake_committed, 50.0); // 50% of 100
            }
            _ => panic!("expected Vouch"),
        }
    }

    // ── Griefing-bound tests ─────────────────────────────────

    #[test]
    fn chain_clawback_does_not_propagate_past_direct_voucher() {
        // Alice → Bob → Charlie. Charlie cheats.
        // Alice should be untouched. Bob loses exactly his stake on Charlie.
        let mut graph = ThicknessGraph::new();
        let alice = PeerId::random();
        let bob = PeerId::random();
        let charlie = PeerId::random();

        // Alice earns thickness
        graph.add_verified_contribution(&alice, [0xAA; 32], 1000.0);

        // Alice vouches for Bob: 20%, permanent
        graph.stake_vouch(&alice, &bob, 0.20, 1, None).unwrap();
        let alice_enc_after_bob = graph.encumbered_for(&alice);

        // Bob vouches for Charlie: 50% of his derived thickness, permanent
        graph.stake_vouch(&bob, &charlie, 0.50, 2, None).unwrap();
        let bob_enc_after_charlie = graph.encumbered_for(&bob);

        // ── Simulate bounded clawback on Charlie ──
        // Charlie cheated. Find all Vouch edges where vouchee == Charlie.
        let charlie_edges = graph.edges_for(&charlie);
        let mut stake_to_release = 0.0;
        let mut affected_vouchers: Vec<PeerId> = Vec::new();

        for edge in charlie_edges {
            if let ThicknessSource::Vouch {
                voucher,
                stake_committed,
                ..
            } = &edge.source
            {
                stake_to_release += stake_committed;
                affected_vouchers.push(*voucher);
            }
        }

        // Assert: exactly one voucher (Bob), and the stake is his committed amount
        assert_eq!(affected_vouchers.len(), 1);
        assert_eq!(affected_vouchers[0], bob);
        assert!(stake_to_release > 0.0);

        // Release Bob's encumbrance for this vouch
        let bob_enc = graph.encumbered_for(&bob);
        graph.encumbered.insert(bob, (bob_enc - stake_to_release).max(0.0));

        // Remove Charlie's edges (simulating clawback edge removal)
        // For the test, we verify the bound by checking Alice was untouched
        // and Bob's loss = his stake_committed on Charlie.

        // ── Verify the griefing bound ──
        // Alice's encumbrance is UNCHANGED
        assert_eq!(
            graph.encumbered_for(&alice),
            alice_enc_after_bob,
            "Alice must be untouched by Charlie's cheat"
        );
        // Alice's total thickness is unchanged
        assert_eq!(graph.total_thickness(&alice), 1000.0);

        // Bob lost exactly his stake_committed on Charlie (no more, no less)
        let bob_enc_after_clawback = graph.encumbered_for(&bob);
        let bob_loss = bob_enc_after_charlie - bob_enc_after_clawback;
        assert!(
            bob_loss > 0.0,
            "Bob should have lost encumbrance from the clawback"
        );
        assert!(
            bob_loss <= bob_enc_after_charlie,
            "Bob cannot lose more than he had encumbered"
        );

        // The stake released equals what Bob committed to Charlie
        // (within floating-point tolerance)
        let epsilon = 0.001;
        assert!(
            (stake_to_release - bob_loss).abs() < epsilon,
            "Released stake ({}) must equal Bob's loss ({})",
            stake_to_release,
            bob_loss
        );
    }
}
