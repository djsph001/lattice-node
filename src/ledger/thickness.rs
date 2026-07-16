// ── ledger/thickness.rs — the thickness provenance graph ──────────
//
// DERIVATION MODEL (2026-07-17):
// Thickness is NOT stored as a mutable scalar. It is DERIVED at read time
// from the graph topology + a contribution count. Every call to
// total_thickness() walks the edge graph and computes values on the fly.
//
// Key change from mutation model: Vouch edges store a stake_fraction (the
// ratio the voucher committed), not an absolute amount. The amount is
// computed as voucher_total × stake_fraction. When the source liquidates,
// the derived amount drops to zero through multiplication — no cascade,
// no traversal, no recursive mutation. Laundering is closed at any depth.
//
// Three sources of thickness:
//   VerifiedContribution — minted by real work. Fixed amount, stored.
//   Genesis              — trusted-setup. Original amount + amortize_over.
//                          Derived as original × (N-k)/N.
//   Vouch               — derived from voucher's total × stake_fraction.
//                          Immutable fraction. Never stores an amount.

use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use libp2p::PeerId;

/// Identifies a specific verified receipt (the Blake3 message_hash).
pub type ReceiptId = [u8; 32];

/// Where a unit of thickness came from. Each variant stores only the
/// INPUTS to the derivation — never the computed amount.
#[derive(Debug, Clone, PartialEq)]
pub enum ThicknessSource {
    /// Earned directly by verified contribution. Fixed amount.
    VerifiedContribution {
        receipt_id: ReceiptId,
        amount: f64,
    },

    /// Thickness granted at genesis — the explicit trusted-setup exception.
    ///
    /// `original_amount`: the initial grant.
    /// `amortize_over`: if Some(N), self-liquidates over N contributions.
    /// Derived amount = original_amount × max(0, (N-k)/N) where k is the
    /// global contribution count. None = permanent.
    Genesis {
        original_amount: f64,
        amortize_over: Option<u64>,
    },

    /// Derived via a vouch. The amount is NEVER stored — it is computed
    /// at read time as total_thickness(voucher) × stake_fraction.
    Vouch {
        voucher: PeerId,
        vouch_nonce: u64,
        stake_fraction: f64,
        expiration_epoch: Option<u64>,
    },
}

/// A single edge in the provenance graph.
#[derive(Debug, Clone)]
pub struct ThicknessEdge {
    pub source: ThicknessSource,
    pub created: DateTime<Utc>,
}

/// The provenance graph — every node's thickness, derived at read time.
///
/// Invariants:
/// - Only VerifiedContribution mints new thickness. Vouches redistribute.
/// - A node's total thickness = sum of derived_amount(edge) for all incoming.
/// - usable = total × (1 - sum_of_active_stake_fractions).
/// - Genesis amount is a pure function of contribution_count — order-free.
/// - Laundering is closed: genesis-derived share derives to zero at any
///   depth when the source liquidates, through multiplication.
#[derive(Debug, Clone)]
pub struct ThicknessGraph {
    edges: HashMap<PeerId, Vec<ThicknessEdge>>,
    genesis_used: bool,
}

impl ThicknessGraph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            genesis_used: false,
        }
    }

    // ── Genesis ────────────────────────────────────────────

    /// Mint root thickness at genesis. Stores original_amount + amortize_over.
    /// The amount is derived at read time — never mutated.
    pub fn add_genesis_thickness(
        &mut self,
        root: &PeerId,
        original_amount: f64,
        amortize_over: Option<u64>,
    ) -> Result<(), String> {
        if self.genesis_used {
            return Err("genesis has already been used — cannot mint twice".into());
        }
        if !self.edges.is_empty() {
            return Err("genesis requires an empty graph".into());
        }
        let edge = ThicknessEdge {
            source: ThicknessSource::Genesis {
                original_amount,
                amortize_over,
            },
            created: Utc::now(),
        };
        self.edges.entry(*root).or_default().push(edge);
        self.genesis_used = true;
        Ok(())
    }

    // ── Derived computations ───────────────────────────────

    /// Count all VerifiedContribution edges in the graph. This is a set
    /// cardinality — order-independent. Two nodes with the same edge set
    /// agree on the count without sequencing.
    fn contribution_count(&self) -> u64 {
        self.edges
            .values()
            .flatten()
            .filter(|e| matches!(e.source, ThicknessSource::VerifiedContribution { .. }))
            .count() as u64
    }

    /// Sum of active (non-expired) stake_fractions where this peer is the
    /// voucher. Used for encumbrance derivation.
    fn active_stake_fractions(&self, voucher: &PeerId) -> f64 {
        self.edges
            .values()
            .flatten()
            .filter_map(|e| match &e.source {
                ThicknessSource::Vouch {
                    voucher: v,
                    stake_fraction,
                    ..
                } if v == voucher => Some(*stake_fraction),
                _ => None,
            })
            .sum()
    }

    /// Total derived thickness for a node: recursive sum of all incoming
    /// edges' derived amounts. Cycle-safe via visited set.
    pub fn total_thickness(&self, peer: &PeerId) -> f64 {
        self.total_thickness_inner(peer, &mut HashSet::new())
    }

    fn total_thickness_inner(
        &self,
        peer: &PeerId,
        visited: &mut HashSet<PeerId>,
    ) -> f64 {
        if !visited.insert(*peer) {
            return 0.0; // cycle detected — treat as zero
        }
        let result = self
            .edges
            .get(peer)
            .map(|edges| {
                edges
                    .iter()
                    .map(|e| match &e.source {
                        ThicknessSource::VerifiedContribution { amount, .. } => *amount,
                        ThicknessSource::Genesis {
                            original_amount,
                            amortize_over,
                        } => {
                            let k = self.contribution_count();
                            match amortize_over {
                                None => *original_amount,
                                Some(n) if *n > 0 => {
                                    *original_amount
                                        * ((*n as f64 - k as f64).max(0.0))
                                        / *n as f64
                                }
                                Some(_) => 0.0,
                            }
                        }
                        ThicknessSource::Vouch {
                            voucher,
                            stake_fraction,
                            ..
                        } => self.total_thickness_inner(voucher, visited) * stake_fraction,
                    })
                    .sum()
            })
            .unwrap_or(0.0);
        visited.remove(peer);
        result
    }

    /// Usable (unencumbered) thickness: total × (1 - sum_of_stake_fractions).
    pub fn usable_thickness(&self, peer: &PeerId) -> f64 {
        let total = self.total_thickness(peer);
        let staked = self.active_stake_fractions(peer);
        (total * (1.0 - staked)).max(0.0)
    }

    /// Total encumbered thickness: total × sum_of_stake_fractions.
    /// Derived, not stored.
    pub fn encumbered_for(&self, peer: &PeerId) -> f64 {
        let total = self.total_thickness(peer);
        let staked = self.active_stake_fractions(peer);
        total * staked
    }

    // ── Mutations ──────────────────────────────────────────

    /// Add a verified contribution. Stores the fixed amount on the edge.
    /// Does NOT trigger any cascade — genesis liquidation is derived.
    pub fn add_verified_contribution(
        &mut self,
        peer: &PeerId,
        receipt_id: ReceiptId,
        amount: f64,
    ) {
        let edge = ThicknessEdge {
            source: ThicknessSource::VerifiedContribution { receipt_id, amount },
            created: Utc::now(),
        };
        self.edges.entry(*peer).or_default().push(edge);
    }

    /// Stake thickness by vouching. Stores the stake_fraction (immutable).
    /// Validates: sum of fractions ≤ 1.0 (can't over-extend).
    ///
    /// Returns the derived amount (voucher_total × stake_fraction).
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

        let current_fractions = self.active_stake_fractions(voucher);
        if current_fractions + staked_fraction > 1.0 {
            return Err(format!(
                "insufficient unencumbered thickness: staked {:.4} + new {:.4} > 1.0",
                current_fractions, staked_fraction
            ));
        }

        let edge = ThicknessEdge {
            source: ThicknessSource::Vouch {
                voucher: *voucher,
                vouch_nonce,
                stake_fraction: staked_fraction,
                expiration_epoch,
            },
            created: Utc::now(),
        };
        self.edges.entry(*vouchee).or_default().push(edge);

        Ok(voucher_total * staked_fraction)
    }

    /// Process epoch tick: remove all expired vouch edges.
    /// Under derivation, removing an edge is sufficient — encumbrance
    /// and derived amounts adjust automatically (fractions sum changes).
    pub fn process_epoch_expiration(&mut self, current_epoch: u64) -> usize {
        let mut count = 0;
        let mut to_remove: Vec<PeerId> = Vec::new();

        for (vouchee, edges) in self.edges.iter_mut() {
            let before = edges.len();
            edges.retain(|e| {
                if let ThicknessSource::Vouch {
                    expiration_epoch: Some(exp),
                    ..
                } = &e.source
                {
                    *exp > current_epoch
                } else {
                    true
                }
            });
            count += before - edges.len();
            if edges.is_empty() {
                to_remove.push(*vouchee);
            }
        }

        for peer in to_remove {
            self.edges.remove(&peer);
        }

        count
    }

    /// Remove all Vouch edges where vouchee receives from the specified voucher.
    /// Used for clawback. Returns the number of edges removed.
    pub fn remove_vouch_edges(&mut self, vouchee: &PeerId, voucher: &PeerId) -> usize {
        let mut removed = 0;
        if let Some(edges) = self.edges.get_mut(vouchee) {
            let before = edges.len();
            edges.retain(|e| {
                !matches!(
                    &e.source,
                    ThicknessSource::Vouch { voucher: v, .. } if v == voucher
                )
            });
            removed = before - edges.len();
        }
        if self.edges.get(vouchee).map_or(true, |e| e.is_empty()) {
            self.edges.remove(vouchee);
        }
        removed
    }

    // ── Read-only accessors ────────────────────────────────

    /// Return all derivation edges for a peer (for traversal / inspection).
    pub fn edges_for(&self, peer: &PeerId) -> &[ThicknessEdge] {
        self.edges.get(peer).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Number of peers with edges in the graph.
    pub fn peer_count(&self) -> usize {
        self.edges.len()
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
    use crate::ledger::types::DigitalUtilityUnit;
    use chrono::Utc;

    #[test]
    fn transfer_moves_units() {
        let mut state = crate::ledger::state::LedgerState::new();
        let alice: PeerId = PeerId::random();
        let bob: PeerId = PeerId::random();

        state.set_balance(&alice, DigitalUtilityUnit(1000));

        let tx = crate::ledger::types::Transaction::Transfer {
            from: alice.to_string(),
            to: bob.to_string(),
            amount: DigitalUtilityUnit(300),
            nonce: 1,
            timestamp: Utc::now(),
        };

        state.apply_transaction(&tx).unwrap();
        assert_eq!(state.balance_of(&alice), DigitalUtilityUnit(700));
        assert_eq!(state.balance_of(&bob), DigitalUtilityUnit(300));
    }

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

        graph.add_verified_contribution(&alice, [0xAA; 32], 1000.0);

        let result = graph.stake_vouch(&alice, &bob, 0.10, 1, None);
        assert!(result.is_ok());
        let per_vouchee = result.unwrap();

        // Bob gets derived thickness = 1000 * 0.10 = 100
        assert_eq!(per_vouchee, 100.0);
        assert_eq!(graph.total_thickness(&bob), 100.0);

        // Alice's total thickness is unchanged...
        assert_eq!(graph.total_thickness(&alice), 1000.0);
        // ...but her usable thickness is reduced
        assert_eq!(graph.usable_thickness(&alice), 900.0);
        assert_eq!(graph.encumbered_for(&alice), 100.0);
    }

    #[test]
    fn vouch_re_divides_existing_vouchees() {
        let mut graph = ThicknessGraph::new();
        let alice = PeerId::random();
        let bob = PeerId::random();
        let carol = PeerId::random();

        graph.add_verified_contribution(&alice, [0xAA; 32], 1000.0);

        graph.stake_vouch(&alice, &bob, 0.20, 1, None).unwrap();
        assert_eq!(graph.total_thickness(&bob), 200.0);

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

        graph.stake_vouch(&alice, &bob, 0.60, 1, None).unwrap();

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
                stake_fraction,
                expiration_epoch,
            } => {
                assert_eq!(*voucher, alice);
                assert_eq!(*vouch_nonce, 42);
                assert_eq!(*stake_fraction, 0.30);
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
        graph.stake_vouch(&alice, &bob, 0.20, 1, Some(10)).unwrap();
        assert_eq!(graph.total_thickness(&bob), 200.0);
        assert_eq!(graph.encumbered_for(&alice), 200.0);

        let unwound = graph.process_epoch_expiration(5);
        assert_eq!(unwound, 0);
        assert_eq!(graph.total_thickness(&bob), 200.0);
        assert_eq!(graph.encumbered_for(&alice), 200.0);

        let unwound = graph.process_epoch_expiration(10);
        assert_eq!(unwound, 1);
        assert_eq!(graph.total_thickness(&bob), 0.0);
        assert_eq!(graph.encumbered_for(&alice), 0.0);
        assert_eq!(graph.usable_thickness(&alice), 1000.0);
    }

    #[test]
    fn multi_vouch_one_expires_survivors_grow() {
        let mut graph = ThicknessGraph::new();
        let alice = PeerId::random();
        let bob = PeerId::random();
        let carol = PeerId::random();

        graph.add_verified_contribution(&alice, [0xAA; 32], 1000.0);
        graph.stake_vouch(&alice, &bob, 0.20, 1, Some(10)).unwrap();
        graph.stake_vouch(&alice, &carol, 0.20, 2, None).unwrap();

        assert_eq!(graph.total_thickness(&bob), 200.0);
        assert_eq!(graph.total_thickness(&carol), 200.0);
        assert_eq!(graph.encumbered_for(&alice), 400.0);

        let unwound = graph.process_epoch_expiration(10);
        assert_eq!(unwound, 1);

        assert_eq!(graph.total_thickness(&bob), 0.0);
        assert_eq!(graph.total_thickness(&carol), 200.0);
        assert_eq!(graph.encumbered_for(&alice), 200.0);
        assert_eq!(graph.usable_thickness(&alice), 800.0);
    }

    #[test]
    fn permanent_vouch_never_expires() {
        let mut graph = ThicknessGraph::new();
        let alice = PeerId::random();
        let bob = PeerId::random();

        graph.add_verified_contribution(&alice, [0xAA; 32], 1000.0);
        graph.stake_vouch(&alice, &bob, 0.20, 1, None).unwrap();

        let unwound = graph.process_epoch_expiration(100);
        assert_eq!(unwound, 0);
        assert_eq!(graph.total_thickness(&bob), 200.0);
        assert_eq!(graph.encumbered_for(&alice), 200.0);
    }

    #[test]
    fn all_vouchees_expire_fully_releases_voucher() {
        let mut graph = ThicknessGraph::new();
        let alice = PeerId::random();
        let bob = PeerId::random();
        let carol = PeerId::random();

        graph.add_verified_contribution(&alice, [0xAA; 32], 500.0);
        graph.stake_vouch(&alice, &bob, 0.10, 1, Some(5)).unwrap();
        graph.stake_vouch(&alice, &carol, 0.10, 2, Some(5)).unwrap();

        assert_eq!(graph.encumbered_for(&alice), 100.0);
        assert_eq!(graph.total_thickness(&bob), 50.0);
        assert_eq!(graph.total_thickness(&carol), 50.0);

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

        for (i, node) in nodes.iter().enumerate() {
            let exp = if i == 0 { Some(10u64) } else { None };
            graph.stake_vouch(&alice, node, 0.10, i as u64 + 1, exp).unwrap();
        }

        for node in &nodes {
            assert_eq!(graph.total_thickness(node), 100.0);
        }
        assert_eq!(graph.encumbered_for(&alice), 400.0);

        let unwound = graph.process_epoch_expiration(10);
        assert_eq!(unwound, 1);

        assert_eq!(graph.total_thickness(&nodes[0]), 0.0);

        for node in &nodes[1..] {
            let t = graph.total_thickness(node);
            assert!((t - 100.0).abs() < 0.001, "survivor expected 100.0, got {}", t);
        }
        let enc = graph.encumbered_for(&alice);
        assert!((enc - 300.0).abs() < 0.001,
            "encumbered expected 300.0, got {}", enc);
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
                expiration_epoch, ..
            } => {
                assert_eq!(*expiration_epoch, Some(42));
            }
            _ => panic!("expected Vouch source"),
        }
    }

    // ── Clawback test ───────────────────────────────────────

    #[test]
    fn chain_clawback_does_not_propagate_past_direct_voucher() {
        let mut graph = ThicknessGraph::new();
        let alice = PeerId::random();
        let bob = PeerId::random();
        let charlie = PeerId::random();

        graph.add_verified_contribution(&alice, [0xAA; 32], 1000.0);
        graph.stake_vouch(&alice, &bob, 0.20, 1, None).unwrap();
        let alice_enc_before = graph.encumbered_for(&alice);

        graph.stake_vouch(&bob, &charlie, 0.50, 2, None).unwrap();
        let bob_enc_before = graph.encumbered_for(&bob);

        // Clawback: remove Charlie's vouch edge from Bob
        let removed = graph.remove_vouch_edges(&charlie, &bob);
        assert_eq!(removed, 1, "exactly one vouch edge removed");

        // Alice untouched
        assert_eq!(
            graph.encumbered_for(&alice),
            alice_enc_before,
            "Alice must be untouched by Charlie's clawback"
        );
        assert_eq!(graph.total_thickness(&alice), 1000.0);

        // Bob's encumbrance dropped to zero (his only vouch was to Charlie)
        let bob_enc_after = graph.encumbered_for(&bob);
        assert_eq!(bob_enc_after, 0.0,
            "Bob's encumbrance should be zero after clawback");

        // Bob's loss is bounded
        let bob_loss = bob_enc_before - bob_enc_after;
        assert!(bob_loss > 0.0, "Bob should have lost encumbrance");
        assert!(bob_loss <= bob_enc_before,
            "Bob cannot lose more than he had encumbered");
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

        graph.add_verified_contribution(&p, [0xAA; 32], 100.0);

        let result = graph.add_genesis_thickness(&p, 500.0, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty graph"));
    }

    #[test]
    fn genesis_then_operator_vouch_provenance_chain() {
        let mut graph = ThicknessGraph::new();
        let root = PeerId::random();
        let lumen = PeerId::random();

        graph.add_genesis_thickness(&root, 1000.0, None).unwrap();
        graph.stake_vouch(&root, &lumen, 0.50, 1, None).unwrap();

        assert_eq!(graph.total_thickness(&root), 1000.0);
        assert_eq!(graph.total_thickness(&lumen), 500.0);
    }

    #[test]
    fn genesis_amortizes_over_contributions() {
        let mut graph = ThicknessGraph::new();
        let root = PeerId::random();
        let peer = PeerId::random();

        graph.add_genesis_thickness(&root, 1000.0, Some(100)).unwrap();
        assert_eq!(graph.total_thickness(&root), 1000.0);

        for i in 1..=100 {
            let before = graph.total_thickness(&root);
            graph.add_verified_contribution(&peer, [0xAA; 32], 10.0);
            let after = graph.total_thickness(&root);

            if i < 100 {
                assert!(
                    after < before,
                    "root genesis should decay at contribution {}",
                    i
                );
            }
        }

        assert_eq!(graph.total_thickness(&root), 0.0);
    }

    #[test]
    fn genesis_without_amortization_is_permanent() {
        let mut graph = ThicknessGraph::new();
        let root = PeerId::random();

        graph.add_genesis_thickness(&root, 1000.0, None).unwrap();
        let genesis_before = graph.total_thickness(&root);

        let peer = PeerId::random();
        graph.add_verified_contribution(&root, [0xAA; 32], 100.0);

        let genesis_after = graph.total_thickness(&root);
        assert!(genesis_after > genesis_before);

        let has_genesis = graph
            .edges_for(&root)
            .iter()
            .any(|e| matches!(&e.source, ThicknessSource::Genesis { .. }));
        assert!(
            has_genesis,
            "genesis with amortize_over=None should be permanent"
        );
    }

    #[test]
    fn vouched_genesis_decays_with_source() {
        let mut graph = ThicknessGraph::new();
        let root = PeerId::random();
        let lumen = PeerId::random();
        let peer = PeerId::random();

        graph.add_genesis_thickness(&root, 1000.0, Some(20)).unwrap();
        graph.stake_vouch(&root, &lumen, 0.90, 1, None).unwrap();
        assert_eq!(graph.total_thickness(&lumen), 900.0);

        for _ in 0..20 {
            graph.add_verified_contribution(&peer, [0xAA; 32], 10.0);
        }

        assert_eq!(graph.total_thickness(&root), 0.0);
        assert_eq!(
            graph.total_thickness(&lumen),
            0.0,
            "Lumen's genesis-derived thickness must decay with its source"
        );
    }

    #[test]
    fn genesis_decay_propagates_transitively() {
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
            graph.total_thickness(&charlie),
            0.0,
            "Charlie must lose genesis-derived thickness — transitive propagation"
        );
    }

    // ═══════════════════════════════════════════════════════════════════
    // LAUNDERING VECTOR TESTS — derivation model
    // ═══════════════════════════════════════════════════════════════════

    #[test]
    fn laundering_closed_through_honest_contribution() {
        // Under the derivation model, the laundering vector is closed:
        // root vouches genesis to B, B earns honest contribution, B vouches
        // to C, genesis liquidates. C retains only B's honest-derived
        // thickness (10 * 0.90 = 9), NOT genesis-laundered amount.
        //
        // Under the old mutation model, this was 819 — the vector was live.

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

        assert_eq!(graph.total_thickness(&root), 0.0, "root fully liquidated");
        assert_eq!(graph.total_thickness(&b), 10.0, "B retains honest contribution");
        assert_eq!(
            graph.total_thickness(&c),
            9.0,
            "DERIVATION: C has only honest-derived thickness (10 * 0.90 = 9). \
             Was 819.0 under mutation model."
        );
    }

    #[test]
    fn derivation_closes_laundering_through_honest_contribution() {
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

        assert_eq!(graph.total_thickness(&root), 0.0, "root fully liquidated");
        assert_eq!(graph.total_thickness(&b), 10.0, "B retains only honest contribution");
        assert_eq!(
            graph.total_thickness(&c),
            9.0,
            "DERIVATION SPEC: C must only have B's honest-derived thickness \
             (10 * 0.90 = 9). Genesis-derived share derives to zero when \
             source liquidates, at any depth, without traversal."
        );
    }

    #[test]
    fn derivation_closes_laundering_at_arbitrary_depth() {
        let mut graph = ThicknessGraph::new();
        let root = PeerId::random();
        let b = PeerId::random();
        let c = PeerId::random();
        let d = PeerId::random();
        let peer = PeerId::random();

        graph.add_genesis_thickness(&root, 1000.0, Some(10)).unwrap();
        graph.stake_vouch(&root, &b, 1.0, 1, None).unwrap();
        graph.add_verified_contribution(&b, [0xAA; 32], 10.0);
        graph.stake_vouch(&b, &c, 0.80, 2, None).unwrap();
        graph.stake_vouch(&c, &d, 0.50, 3, None).unwrap();

        for _ in 0..9 {
            graph.add_verified_contribution(&peer, [0xBB; 32], 10.0);
        }

        assert_eq!(graph.total_thickness(&root), 0.0, "root fully liquidated");
        assert_eq!(graph.total_thickness(&b), 10.0, "B retains only honest contribution");
        assert_eq!(
            graph.total_thickness(&c),
            8.0,
            "DERIVATION SPEC: C has only B's honest-derived thickness (10 * 0.80)"
        );
        assert_eq!(
            graph.total_thickness(&d),
            4.0,
            "DERIVATION SPEC: D has only honest-derived thickness propagated \
             through the chain (10 * 0.80 * 0.50). Genesis-derived share \
             derives to zero at arbitrary depth without traversal."
        );
    }
}
