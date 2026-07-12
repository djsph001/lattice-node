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
}

impl EconomicEngine {
    /// Create a new economic engine with zeroed metrics.
    pub fn new() -> Self {
        Self {
            metrics: NodeMetrics::new(),
            epoch_metrics: NodeMetrics::new(),
            epoch_count: 0,
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
        let epoch_delta = NodeMetrics {
            bytes_relayed: self.metrics.bytes_relayed - self.epoch_metrics.bytes_relayed,
            messages_propagated: self.metrics.messages_propagated
                - self.epoch_metrics.messages_propagated,
            dht_records_stored: self.metrics.dht_records_stored
                - self.epoch_metrics.dht_records_stored,
            heartbeats_sent: self.metrics.heartbeats_sent - self.epoch_metrics.heartbeats_sent,
            transactions_relayed: self.metrics.transactions_relayed
                - self.epoch_metrics.transactions_relayed,
            bytes_consumed: self.metrics.bytes_consumed - self.epoch_metrics.bytes_consumed,
            queries_issued: self.metrics.queries_issued - self.epoch_metrics.queries_issued,
            transactions_submitted: self.metrics.transactions_submitted
                - self.epoch_metrics.transactions_submitted,
            verified_bytes_relayed: 0,
            verified_messages_relayed: 0,
            agent_tasks_active: self.metrics.agent_tasks_active,
        };

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

        // ── Snapshot current metrics for next epoch ───────
        self.epoch_metrics = self.metrics.clone();

        result
    }

    /// Number of epochs completed so far.
    pub fn epoch_count(&self) -> u64 {
        self.epoch_count
    }
}

impl Default for EconomicEngine {
    fn default() -> Self {
        Self::new()
    }
}
