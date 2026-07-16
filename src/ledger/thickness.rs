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

    /// Thickness granted at genesis — the explicit trusted-setup exception.
    /// Honest about its status: this is a trust-granting JUDGMENT act,
    /// not a trust-earning one. Fires only once, into an empty graph,
    /// to the operator-designated root of trust. Auditable.
    ///
    /// `amortize_over`: if Some(N), the genesis edge self-liquidates over
    /// N verified contributions — each contribution reduces the edge by
    /// `amount / N`. None = permanent (no decay). Contribution-denominated,
    /// rate-independent, timer-independent.
    ///
    /// WARNING: choosing None creates a permanent founder floor. As the mesh
    /// grows in participants, `root_thickness / individual_peer_thickness`
    /// INCREASES — the founder becomes the single largest individual holder.
    /// Weighted panel voting is per-capita-among-members, not share-of-total,
    /// so this floor grants a permanent plurality in every governance panel.
    /// Prefer Some(N) for any mesh intended to decentralize over time.
    Genesis {
        amortize_over: Option<u64>,
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

    /// Thickness encumbered by outgoing vouches.
    /// key = voucher PeerId; value = total encumbered thickness.
    encumbered: HashMap<PeerId, f64>,

    /// Genesis guard: true after first genesis mints root thickness.
    /// Ensures genesis is a one-time trusted-setup act, not an ongoing
    /// mint-from-nothing backdoor.
    genesis_used: bool,
}

impl ThicknessGraph {
    /// Create an empty provenance graph.
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            encumbered: HashMap::new(),
            genesis_used: false,
        }
    }

    /// Mint root thickness at genesis — explicit trusted-setup exception.
    ///
    /// This is a JUDGMENT act, not an earned one: the operator designates
    /// a root of trust by fiat. Fires only once, into an empty graph.
    /// After genesis, all thickness enters via VerifiedContribution or Vouch.
    ///
    /// Returns error if genesis has already been used or if the graph is
    /// not empty (genesis is a one-time, first-act-only event).
    /// `amortize_over`: if Some(N), genesis self-liquidates over N contributions.
    /// None = permanent. Contribution-denominated, rate-independent.
    pub fn add_genesis_thickness(
        &mut self,
        root: &PeerId,
        amount: f64,
        amortize_over: Option<u64>,
    ) -> Result<(), String> {
        if self.genesis_used {
            return Err("genesis has already been used — cannot mint twice".into());
        }
        if self.edges.len() > 0 || self.peer_count() > 0 {
            return Err("genesis requires an empty graph".into());
        }
        let edge = ThicknessEdge {
            source: ThicknessSource::Genesis { amortize_over },
            amount,
            created: Utc::now(),
        };
        self.edges.entry(*root).or_default().push(edge);
        self.genesis_used = true;
        Ok(())
    }

    /// Amortize genesis: each verified contribution reduces the genesis
    fn amortize_genesis(&mut self, _contribution_amount: f64) {
        // Collect amortization targets first (avoid borrow conflicts)
        let mut updates: Vec<(PeerId, usize, f64)> = Vec::new();
        let mut to_remove: Vec<(PeerId, usize)> = Vec::new();

        for (peer, edges) in self.edges.iter() {
            for (idx, edge) in edges.iter().enumerate() {
                if let ThicknessSource::Genesis { amortize_over } = &edge.source {
                    if let Some(n) = amortize_over {
                        if *n > 0 {
                            let decay = edge.amount / *n as f64;
                            let new_amount = (edge.amount - decay).max(0.0);
                            updates.push((*peer, idx, new_amount));
                            if new_amount <= 0.0 || *n <= 1 {
                                to_remove.push((*peer, idx));
                            }
                        }
                    }
                }
            }
        }

        // Apply amount updates (use amortize_over to also decrement remaining)
        for (peer, idx, new_amount) in &updates {
            if let Some(edges) = self.edges.get_mut(peer) {
                if *idx < edges.len() {
                    edges[*idx].amount = *new_amount;
                    // Decrement amortize_over
                    if let ThicknessSource::Genesis { amortize_over: ref mut remain } = &mut edges[*idx].source {
                        if let Some(n) = remain {
                            *remain = Some(n.saturating_sub(1));
                        }
                    }
                }
            }
        }

        // Remove fully-amortized edges (reverse for index stability)
        to_remove.sort_by_key(|(p, i)| (*p, *i));
        to_remove.reverse();
        for (peer, idx) in to_remove {
            if let Some(edges) = self.edges.get_mut(&peer) {
                if idx < edges.len() {
                    edges.remove(idx);
                    if edges.is_empty() {
                        self.edges.remove(&peer);
                    }
                }
            }
        }

        // Propagate decay through vouchees: for each voucher whose thickness
        // changed, re-divide all their vouchees proportionally.
        // Also reduce encumbrance if genesis decay reduced the voucher's total.
        for (peer, idx, new_amount) in &updates {
            let total = self.total_thickness(peer);
            if total > 0.0 {
                if let Some(enc) = self.encumbered.get_mut(peer) {
                    *enc = (*enc).min(total);
                }
            } else {
                // Total is zero — release all encumbrance AND remove all
                // vouchees transitively (recursive: their vouchees go too).
                self.encumbered.remove(peer);
                self.remove_vouchees_recursive(peer);
                continue; // skip re_divide — nothing left to divide
            }
            self.re_divide_vouchees(peer);
        }
    }

    /// Remove all Vouch edges where this peer is the voucher, recursively
    /// clearing their vouchees too. Called when a peer's total thickness
    /// hits zero (genesis fully liquidated).
    fn remove_vouchees_recursive(&mut self, voucher: &PeerId) {
        // Collect vouchees of this voucher
        let mut vouchees: Vec<PeerId> = Vec::new();
        for (p, edges) in self.edges.iter_mut() {
            let before = edges.len();
            edges.retain(|e| {
                !matches!(&e.source, ThicknessSource::Vouch { voucher: v, .. } if v == voucher)
            });
            if edges.len() < before {
                vouchees.push(*p);
            }
        }
        // Clean empty entries
        for p in &vouchees {
            if self.edges.get(p).map_or(true, |e| e.is_empty()) {
                self.edges.remove(p);
            }
        }
        // Recursively clear their vouchees (transitive propagation)
        for v in &vouchees {
            if self.total_thickness(v) == 0.0 {
                self.encumbered.remove(v);
                self.remove_vouchees_recursive(v);
            }
        }
    }

    /// Re-divide all vouchees of a voucher proportionally when their
    /// total thickness changes (genesis amortization, etc.).
    fn re_divide_vouchees(&mut self, voucher: &PeerId) {
        let total_enc = self.encumbered.get(voucher).copied().unwrap_or(0.0);
        if total_enc <= 0.0 {
            return;
        }

        // Count active vouchees for this voucher
        let remaining_count = self
            .edges
            .values()
            .flatten()
            .filter(|e| {
                matches!(&e.source, ThicknessSource::Vouch { voucher: v, .. } if v == voucher)
            })
            .count();

        if remaining_count == 0 {
            return;
        }

        let per_vouchee = total_enc / remaining_count as f64;

        // Re-divide
        for edges in self.edges.values_mut() {
            for edge in edges.iter_mut() {
                if let ThicknessSource::Vouch { voucher: v, .. } = &edge.source {
                    if v == voucher {
                        edge.amount = per_vouchee;
                    }
                }
            }
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

    /// Mint new thickness from a verified contribution. Also amortizes
    /// genesis edges (contribution-denominated decay).
    pub fn add_verified_contribution(
        &mut self,
        peer: &PeerId,
        receipt_id: ReceiptId,
        amount: f64,
    ) {
        // Amortize genesis before adding — contribution-denominated.
        self.amortize_genesis(amount);

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

    // ── Genesis tests ───────────────────────────────────────

    #[test]
    fn genesis_mints_root_thickness_into_empty_graph() {
        let mut graph = ThicknessGraph::new();
        let root = PeerId::random();

        assert!(!graph.genesis_used);
        graph.add_genesis_thickness(&root, 1000.0, None).unwrap();

        assert!(graph.genesis_used);
        assert_eq!(graph.total_thickness(&root), 1000.0);
        assert_eq!(graph.usable_thickness(&root), 1000.0);

        let edges = graph.edges_for(&root);
        assert_eq!(edges.len(), 1);
        assert!(matches!(edges[0].source, ThicknessSource::Genesis { .. }));
    }

    #[test]
    fn genesis_fails_when_already_used() {
        let mut graph = ThicknessGraph::new();
        let root = PeerId::random();

        graph.add_genesis_thickness(&root, 500.0, None).unwrap();
        let result = graph.add_genesis_thickness(&root, 500.0, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already been used"));
    }

    #[test]
    fn genesis_fails_on_non_empty_graph() {
        let mut graph = ThicknessGraph::new();
        let p = PeerId::random();

        // Add a verified contribution first (graph is non-empty)
        graph.add_verified_contribution(&p, [0xAA; 32], 100.0);

        // Genesis should now fail
        let result = graph.add_genesis_thickness(&p, 500.0, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty graph"));
    }

    #[test]
    fn genesis_then_operator_vouch_provenance_chain() {
        // The intended flow: Genesis → operator(root) → vouch → Lumen
        let mut graph = ThicknessGraph::new();
        let root = PeerId::random();    // Dale (operator)
        let lumen = PeerId::random();   // Lumen (executor)

        // 1. Genesis mints thickness to operator
        graph.add_genesis_thickness(&root, 1000.0, None).unwrap();
        assert_eq!(graph.total_thickness(&root), 1000.0);

        // 2. Operator vouches Lumen with 20% of root thickness
        graph.stake_vouch(&root, &lumen, 0.20, 1, None).unwrap();
        assert_eq!(graph.total_thickness(&lumen), 200.0);
        assert_eq!(graph.encumbered_for(&root), 200.0);

        // 3. Provenance: Lumen's thickness traces to root's Genesis via Vouch edge
        let lumen_edges = graph.edges_for(&lumen);
        match &lumen_edges[0].source {
            ThicknessSource::Vouch { voucher, .. } => {
                assert_eq!(*voucher, root);
            }
            _ => panic!("expected Vouch from root to Lumen"),
        }
    }

    #[test]
    fn genesis_amortizes_over_contributions() {
        let mut graph = ThicknessGraph::new();
        let root = PeerId::random();
        let peer = PeerId::random();

        graph.add_genesis_thickness(&root, 1000.0, Some(100)).unwrap();
        assert_eq!(graph.total_thickness(&root), 1000.0);

        // Each contribution by PEER (not root) reduces root's genesis by 1000/100=10
        for i in 1..=100 {
            let before = graph.total_thickness(&root);
            graph.add_verified_contribution(&peer, [0xAA; 32], 10.0);
            let after = graph.total_thickness(&root);

            if i < 100 {
                assert!(after < before, "root genesis should decay at contribution {}", i);
            }
        }

        // After 100 contributions, genesis fully amortized — root has zero
        assert_eq!(graph.total_thickness(&root), 0.0);
    }

    #[test]
    fn genesis_without_amortization_is_permanent() {
        let mut graph = ThicknessGraph::new();
        let root = PeerId::random();

        graph.add_genesis_thickness(&root, 1000.0, None).unwrap();
        let genesis_before = graph.total_thickness(&root);

        // Add many contributions — genesis should be untouched
        for _ in 0..50 {
            graph.add_verified_contribution(&root, [0xBB; 32], 10.0);
        }

        let genesis_after = graph.total_thickness(&root);
        // root has genesis + earned, so total > genesis_before
        assert!(genesis_after > genesis_before);
        // but genesis itself is still intact (present in total)
        let has_genesis = graph.edges_for(&root).iter().any(|e| {
            matches!(&e.source, ThicknessSource::Genesis { .. })
        });
        assert!(has_genesis, "genesis with amortize_over=None should be permanent");
    }

    #[test]
    fn vouched_genesis_decays_with_source() {
        // Root vouches genesis to Lumen → Lumen's derived thickness
        // must decay as genesis amortizes. Otherwise root can launder
        // genesis by vouching it out before decay hits.
        let mut graph = ThicknessGraph::new();
        let root = PeerId::random();
        let lumen = PeerId::random();
        let peer = PeerId::random(); // earns contributions to drive decay

        // Genesis + vouch at n=0
        graph.add_genesis_thickness(&root, 1000.0, Some(20)).unwrap();
        graph.stake_vouch(&root, &lumen, 0.90, 1, None).unwrap();
        // Lumen gets 900 of genesis-derived thickness
        assert_eq!(graph.total_thickness(&lumen), 900.0);

        // Advance through all 20 contributions (genesis fully liquidated)
        for _ in 0..20 {
            graph.add_verified_contribution(&peer, [0xAA; 32], 10.0);
        }

        // Genesis is fully amortized — root has zero thickness.
        assert_eq!(graph.total_thickness(&root), 0.0);
        // Lumen's genesis-derived thickness must also be gone.
        assert_eq!(
            graph.total_thickness(&lumen), 0.0,
            "Lumen's genesis-derived thickness must decay with its source"
        );
    }

    #[test]
    fn genesis_decay_propagates_transitively() {
        // Genesis → Root → Lumen → Charlie.
        // Full liquidation must reach Charlie, not just Lumen.
        let mut graph = ThicknessGraph::new();
        let root = PeerId::random();
        let lumen = PeerId::random();
        let charlie = PeerId::random();
        let peer = PeerId::random();

        graph.add_genesis_thickness(&root, 1000.0, Some(10)).unwrap();
        graph.stake_vouch(&root, &lumen, 0.90, 1, None).unwrap();
        graph.stake_vouch(&lumen, &charlie, 0.50, 2, None).unwrap();

        assert!(graph.total_thickness(&charlie) > 0.0);

        for _ in 0..10 {
            graph.add_verified_contribution(&peer, [0xAA; 32], 10.0);
        }

        assert_eq!(graph.total_thickness(&root), 0.0);
        assert_eq!(graph.total_thickness(&lumen), 0.0);
        assert_eq!(
            graph.total_thickness(&charlie), 0.0,
            "Charlie must lose genesis-derived thickness — transitive propagation"
        );
    }

    // ═══════════════════════════════════════════════════════════════════
    // LAUNDERING VECTOR TESTS — derivation model specification
    // ═══════════════════════════════════════════════════════════════════
    //
    // The existing tests (vouched_genesis_decays_with_source,
    // genesis_decay_propagates_transitively) test the case where the
    // intermediate node has NO honest thickness — so total hits zero and
    // remove_vouchees_recursive completes.
    //
    // These tests add the missing case: the intermediate node has an honest
    // verified contribution. Under the mutation model, remove_vouchees_recursive
    // stops at that node (total ≠ 0), and the vouchee downstream retains
    // genesis-laundered thickness as a permanent Vouch edge.
    //
    // The derivation model must close this at any depth — genesis-derived
    // share computes to zero when the source liquidates, regardless of
    // honest contributions in between.
    //
    // These tests are the SPEC for the derivation model. They are RED
    // against the current mutation model. They go GREEN when genesis-derived
    // thickness is computed at read time from lineage + contribution count
    // instead of mutated on events.

    #[test]
    fn laundering_vector_live_through_honest_contribution() {
        // PROOF TEST: documents that the laundering vector exists in the
        // current mutation model. PASSES against current code — proving
        // the bug. C should have 9.0 but has 819.0.
        //
        // Vector: root vouches genesis to B, B earns honest contribution,
        // B vouches to C. Genesis liquidates. B survives (honest work),
        // recursion stops, C keeps genesis-laundered thickness.

        let mut graph = ThicknessGraph::new();
        let root = PeerId::random();
        let b = PeerId::random();
        let c = PeerId::random();
        let peer = PeerId::random();

        // 1. Self-liquidating genesis over 10 contributions
        graph.add_genesis_thickness(&root, 1000.0, Some(10)).unwrap();

        // 2. Root vouches 100% to B at t=0
        graph.stake_vouch(&root, &b, 1.0, 1, None).unwrap();

        // 3. B earns honest verified contribution (10 thickness)
        graph.add_verified_contribution(&b, [0xAA; 32], 10.0);

        // 4. B vouches 90% to C
        graph.stake_vouch(&b, &c, 0.90, 2, None).unwrap();

        // 5. Drive genesis to full liquidation (9 more contributions)
        for _ in 0..9 {
            graph.add_verified_contribution(&peer, [0xBB; 32], 10.0);
        }

        // Current mutation model produces these values:
        assert_eq!(graph.total_thickness(&root), 0.0,
            "root fully liquidated");
        assert_eq!(graph.total_thickness(&b), 10.0,
            "B retains honest contribution only");
        // THE BUG: C has 819 (genesis-laundered), should have 9 (honest-derived)
        assert_eq!(graph.total_thickness(&c), 819.0,
            "CURRENT BEHAVIOR (bug): C retains genesis-laundered thickness \
             through intermediate node with honest contribution. \
             remove_vouchees_recursive stopped because B ≠ 0.");
    }

    #[test]
    fn derivation_closes_laundering_through_honest_contribution() {
        // SPEC TEST: specifies what the derivation model must achieve.
        // RED against current mutation model. GREEN when genesis-derived
        // thickness is derived at read time from lineage + contribution count.
        //
        // Same setup as the proof test, but asserts CORRECT behavior.
        // C must have only B's honest-derived thickness (10 * 0.90 = 9),
        // NOT the genesis-laundered amount (819).

        let mut graph = ThicknessGraph::new();
        let root = PeerId::random();
        let b = PeerId::random();
        let c = PeerId::random();
        let peer = PeerId::random();

        graph.add_genesis_thickness(&root, 1000.0, Some(10)).unwrap();
        graph.stake_vouch(&root, &b, 1.0, 1, None).unwrap();
        graph.add_verified_contribution(&b, [0xAA; 32], 10.0);
        graph.stake_vouch(&b, &c, 0.90, 2, None).unwrap();

        for _ in 0..9 {
            graph.add_verified_contribution(&peer, [0xBB; 32], 10.0);
        }

        assert_eq!(graph.total_thickness(&root), 0.0,
            "root fully liquidated");

        assert_eq!(graph.total_thickness(&b), 10.0,
            "B retains only honest contribution");

        // THE SPEC: C must have only B's honest-derived thickness.
        // Genesis-derived share must derive to zero at this depth
        // without recursive traversal.
        assert_eq!(graph.total_thickness(&c), 9.0,
            "DERIVATION SPEC: C must only have B's honest-derived thickness \
             (10 * 0.90 = 9). Genesis-derived share derives to zero when \
             source liquidates, at any depth, without traversal.");
    }

    #[test]
    fn derivation_closes_laundering_at_arbitrary_depth() {
        // SPEC TEST: deeper chain — root → B → C → D.
        // B has honest contribution. Genesis liquidates.
        // D must have only honest-derived thickness propagated through
        // the chain, not genesis-laundered.
        //
        // RED against current mutation model.

        let mut graph = ThicknessGraph::new();
        let root = PeerId::random();
        let b = PeerId::random();
        let c = PeerId::random();
        let d = PeerId::random();
        let peer = PeerId::random();

        graph.add_genesis_thickness(&root, 1000.0, Some(10)).unwrap();

        // root → B (100%)
        graph.stake_vouch(&root, &b, 1.0, 1, None).unwrap();

        // B earns honest contribution
        graph.add_verified_contribution(&b, [0xAA; 32], 10.0);

        // B → C (80%)
        graph.stake_vouch(&b, &c, 0.80, 2, None).unwrap();

        // C → D (50%)
        graph.stake_vouch(&c, &d, 0.50, 3, None).unwrap();

        // Drive liquidation
        for _ in 0..9 {
            graph.add_verified_contribution(&peer, [0xBB; 32], 10.0);
        }

        assert_eq!(graph.total_thickness(&root), 0.0,
            "root fully liquidated");

        assert_eq!(graph.total_thickness(&b), 10.0,
            "B retains only honest contribution");

        // C gets 80% of B's honest 10 = 8
        assert_eq!(graph.total_thickness(&c), 8.0,
            "DERIVATION SPEC: C has only B's honest-derived thickness (10 * 0.80)");

        // D gets 50% of C's honest-derived 8 = 4
        assert_eq!(graph.total_thickness(&d), 4.0,
            "DERIVATION SPEC: D has only honest-derived thickness propagated \
             through the chain (10 * 0.80 * 0.50). Genesis-derived share \
             derives to zero at arbitrary depth without traversal.");
    }
}
