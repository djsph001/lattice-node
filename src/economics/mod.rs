// ── economics/mod.rs — the economic engine ──────────────────────
//
// The `EconomicEngine` owns the measurement layer (NodeMetrics), the
// mint logic, and the tax engine.  It exposes a single `run_epoch()`
// method called from the node's event loop on every epoch tick.
//
// One epoch = one economic cycle:
//   1. Measure contribution since last epoch
//   2. Mint proportional to contribution
//   3. Tax proportional to consumption (via contribution ratio)
//   4. Redistribute tax equally to all known peers
//
// The result is a batch of `Transaction` values that the node signs
// and publishes to the `/lattice/tx/v1` gossipsub topic so every
// peer can update its local ledger.

pub mod metrics;
pub mod mint;
pub mod receipts;
pub mod tax;

use libp2p::PeerId;

use crate::ledger::types::DigitalUtilityUnit;
use crate::state::peers::PeerTable;

use self::metrics::NodeMetrics;
use self::tax::{EpochTransactions, TaxEngine};

/// Summary of one completed epoch's economic outcome.
/// Returned by `EconomicEngine::last_epoch_summary()`.
#[derive(Debug, Clone)]
pub struct EpochSummary {
    pub ratio: f64,
    pub tax_calculated: u64,
    pub tax_collected: u64,
    pub minted: u64,
    pub redistributed_to: u64,
}

/// The economic engine — measurement, minting, and taxation.
///
/// Created once at node startup and called on every epoch tick.
pub struct EconomicEngine {
    /// Cumulative contribution/consumption metrics for this node.
    pub metrics: NodeMetrics,

    /// Last-epoch snapshot (reset each epoch for delta calculation).
    epoch_metrics: NodeMetrics,

    /// Number of completed epochs.
    epoch_count: u64,

    /// Summary of the last completed epoch (None before first run).
    last_epoch_summary: Option<EpochSummary>,

    /// Claims that have been accepted.
    /// At epoch boundary, claims with `applied_at_epoch: None` are
    /// credited to the thickness graph and marked `Some(current_epoch)`.
    /// This is the single source of truth — no separate pending queue.
    accepted_claims: Vec<crate::ledger::persistence::StoredClaim>,
}

impl EconomicEngine {
    /// Create a new economic engine with zeroed metrics.
    pub fn new() -> Self {
        Self {
            metrics: NodeMetrics::new(),
            epoch_metrics: NodeMetrics::new(),
            epoch_count: 0,
            last_epoch_summary: None,
            accepted_claims: Vec::new(),
        }
    }

    /// Run one epoch cycle.
    ///
    /// Returns the set of economic transactions that encode this
    /// epoch's outcome.  The caller (node event loop) signs each
    /// one and broadcasts via gossipsub.
    ///
    /// # Arguments
    ///
    /// * `self_peer` — this node's PeerId.
    /// * `self_balance` — this node's current balance.
    /// * `peer_table` — all known peers (redistribution recipients).
    /// * `base_mint_rate` — configurable mint base rate (CLI).
    /// * `base_tax_rate_pct` — configurable base tax rate in percent (CLI).
    pub fn run_epoch(
        &mut self,
        self_peer: &PeerId,
        self_balance: DigitalUtilityUnit,
        peer_table: &PeerTable,
        base_mint_rate: u64,
        base_tax_rate_pct: u64,
    ) -> EpochTransactions {
        self.epoch_count += 1;

        // ── Calculate per-epoch deltas ────────────────────
        // Use saturating_sub to prevent underflow: when a u64 counter
        // wraps, the delta must go to 0, not u64::MAX.
        let epoch_delta = NodeMetrics {
            bytes_relayed: self.metrics.bytes_relayed.saturating_sub(self.epoch_metrics.bytes_relayed),
            messages_propagated: self.metrics.messages_propagated
                .saturating_sub(self.epoch_metrics.messages_propagated),
            dht_records_stored: self.metrics.dht_records_stored
                .saturating_sub(self.epoch_metrics.dht_records_stored),
            heartbeats_sent: self.metrics.heartbeats_sent.saturating_sub(self.epoch_metrics.heartbeats_sent),
            transactions_relayed: self.metrics.transactions_relayed
                .saturating_sub(self.epoch_metrics.transactions_relayed),
            bytes_consumed: self.metrics.bytes_consumed.saturating_sub(self.epoch_metrics.bytes_consumed),
            queries_issued: self.metrics.queries_issued.saturating_sub(self.epoch_metrics.queries_issued),
            transactions_submitted: self.metrics.transactions_submitted
                .saturating_sub(self.epoch_metrics.transactions_submitted),
            verified_bytes_relayed: self.metrics.verified_bytes_relayed
                .saturating_sub(self.epoch_metrics.verified_bytes_relayed),
            verified_messages_relayed: self.metrics.verified_messages_relayed
                .saturating_sub(self.epoch_metrics.verified_messages_relayed),
            agent_tasks_active: self.metrics.agent_tasks_active,
        };

        // Detect counter wraps: if any current metric is smaller than
        // the previous epoch's snapshot, a u64 counter wrapped between
        // epochs.  saturating_sub correctly pins the delta to 0, but
        // the wrap itself is worth logging — silently earning 0 would
        // look like a bug rather than a correct guard.
        if self.metrics.bytes_relayed < self.epoch_metrics.bytes_relayed
            || self.metrics.verified_bytes_relayed < self.epoch_metrics.verified_bytes_relayed
            || self.metrics.messages_propagated < self.epoch_metrics.messages_propagated
        {
            tracing::warn!(
                epoch = self.epoch_count,
                bytes = self.metrics.bytes_relayed,
                prev_bytes = self.epoch_metrics.bytes_relayed,
                verified = self.metrics.verified_bytes_relayed,
                prev_verified = self.epoch_metrics.verified_bytes_relayed,
                "u64 counter wrap detected — epoch delta pinned to 0. \
                 If this recurs, consider wider counters or per-epoch reset."
            );
        }

        tracing::debug!(
            epoch = self.epoch_count,
            bytes_relayed = epoch_delta.bytes_relayed,
            msgs_propagated = epoch_delta.messages_propagated,
            dht_stored = epoch_delta.dht_records_stored,
            bytes_consumed = epoch_delta.bytes_consumed,
            queries = epoch_delta.queries_issued,
            "Epoch metrics delta"
        );

        // ── Mint from contribution ────────────────────────
        // Phase 6: use receipt-verified metrics when available,
        // fall back to self-reported for solo operation.
        let mint_amount = mint::calculate_mint_from_receipts(&epoch_delta, base_mint_rate);

        // ── Tax & redistribute ────────────────────────────
        let result = TaxEngine::execute_epoch(
            self_peer,
            self_balance,
            peer_table,
            &self.metrics,
            mint_amount,
            self.epoch_count,
            base_tax_rate_pct,
        );

        // Store summary of this epoch for the API.
        self.last_epoch_summary = Some(EpochSummary {
            ratio: result.ratio,
            tax_calculated: result.tax_calculated,
            tax_collected: result.tax_collected,
            minted: result.minted,
            redistributed_to: result.redistributed_to,
        });

        // ── Snapshot current metrics for next epoch ───────
        self.epoch_metrics = self.metrics.clone();

        result
    }

    /// Number of epochs completed so far.
    pub fn epoch_count(&self) -> u64 {
        self.epoch_count
    }

    /// Summary of the most recently completed epoch, or None if no
    /// epoch has completed yet.
    pub fn last_epoch_summary(&self) -> Option<&EpochSummary> {
        self.last_epoch_summary.as_ref()
    }

    /// Queue a claim for crediting at the next epoch boundary.
    /// The claim is immediately stored in accepted_claims with
    /// applied_at_epoch: None, ensuring it survives crashes via snapshot.
    pub fn queue_claim(&mut self, claim: crate::claims::WitnessedClaim) {
        self.accepted_claims.push(
            crate::ledger::persistence::StoredClaim {
                claim,
                applied_at_epoch: None,
            }
        );
    }

    /// Take all unapplied claims for processing at the epoch boundary.
    /// Returns the claims to credit and their indices for marking applied.
    pub fn take_unapplied_claims(&mut self) -> Vec<(usize, crate::claims::WitnessedClaim)> {
        self.accepted_claims.iter().enumerate()
            .filter(|(_, sc)| sc.applied_at_epoch.is_none())
            .map(|(i, sc)| (i, sc.claim.clone()))
            .collect()
    }

    /// Mark claims as applied at the given epoch.
    pub fn mark_applied(&mut self, indices: &[usize], epoch: u64) {
        for &i in indices {
            if let Some(sc) = self.accepted_claims.get_mut(i) {
                sc.applied_at_epoch = Some(epoch);
            }
        }
    }

    /// Take all accepted claims for snapshot persistence.
    pub fn take_accepted_claims(&mut self) -> Vec<crate::ledger::persistence::StoredClaim> {
        std::mem::take(&mut self.accepted_claims)
    }

    /// Import accepted claims on recovery (rebuild from snapshot).
    pub fn import_accepted_claims(&mut self, claims: Vec<crate::ledger::persistence::StoredClaim>) {
        self.accepted_claims = claims;
    }
}

impl Default for EconomicEngine {
    fn default() -> Self {
        Self::new()
    }
}
