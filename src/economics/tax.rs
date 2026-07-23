// ── economics/tax.rs — Georgist redistribution engine ───────────
//
// This is the soul of the Lattice economic model.
//
// The insight: a mesh is a *commons*.  Every node that occupies
// connections, consumes bandwidth, and issues queries is using a
// shared resource.  A Georgist tax charges nodes for their footprint
// on that commons and redistributes the proceeds equally to all
// participants.
//
// The contribution ratio determines the effective tax rate:
//
//   tax_rate = BASE_TAX_RATE * (1.0 / contribution_ratio)
//
//   • ratio 2.0 (gives twice what it takes) → half the base tax
//   • ratio 1.0 (balanced)                 → base tax
//   • ratio 0.5 (takes twice what it gives) → double the base tax
//
// This creates a Nash equilibrium where *contributing is the
// dominant strategy* — the more you give, the less you're taxed.
// The more you take without giving, the more you're taxed.
//
// Redistribution model (Phase 5): all tax collected is split equally
// among all known peers.  This is the simplest model and the right
// starting point.  Phase 6 may explore proportional-to-contribution
// or uptime-weighted redistribution.

use libp2p::PeerId;

use crate::ledger::types::{DigitalUtilityUnit, Transaction};
use crate::state::peers::PeerTable;

use super::metrics::NodeMetrics;

// ── Policy constants (governance decisions) ──────────────────────

/// Basis-point denominator — how many basis points make 100%.
const BPS_DENOMINATOR: u64 = 10_000;

/// Scale factor for fixed-point contribution ratio arithmetic.
/// Gives 2 decimal places of precision: ratio 2.0 → scaled 200.
const RATIO_SCALE: u64 = 100;

/// Minimum contribution ratio (scaled) to prevent division by zero
/// in the tax-rate calculation.  A node with zero contribution gets
/// the maximum tax rate.
const MIN_RATIO_SCALED: u64 = 1;

/// Produced by `TaxEngine::execute_epoch()` — the batch of economic
/// transactions that encode one epoch's outcome.
#[derive(Debug)]
pub struct EpochTransactions {
    /// Mint transaction for this node's contribution reward.
    pub mint: Option<Transaction>,
    /// Transfer transactions for redistribution (one per known peer).
    pub redistributions: Vec<Transaction>,
    /// Contribution ratio for this epoch (0.0–2.0).
    pub ratio: f64,
    /// Tax amount calculated (owed) for this epoch.
    pub tax_calculated: u64,
    /// Tax amount actually collected (may differ from calculated
    /// when peers exist but shares are below split threshold).
    pub tax_collected: u64,
    /// Amount minted as contribution reward.
    pub minted: u64,
    /// Number of peers redistributed to (count, not summed amount).
    pub redistributed_to: u64,
}

/// The Georgist tax and redistribution engine.
///
/// Call `execute_epoch()` once per epoch.  It reads the node's
/// current balance, contribution ratio, and peer table, then produces
/// the set of transactions that encode the epoch's economic outcome.
pub struct TaxEngine;

impl TaxEngine {
    /// Run the tax cycle for one epoch.
    ///
    /// # Arguments
    ///
    /// * `self_peer` — this node's PeerId (taxpayer and beneficiary).
    /// * `self_balance` — this node's current balance (in local ledger).
    /// * `peer_table` — all currently known peers (redistribution recipients).
    /// * `metrics` — this node's contribution/consumption metrics.
    /// * `mint_amount` — units to mint for this epoch (from `mint::calculate_mint`).
    /// * `base_tax_rate_pct` — configurable base tax rate, as a percent (e.g. 5 = 5%).
    #[allow(clippy::too_many_arguments)]
    pub fn execute_epoch(
        self_peer: &PeerId,
        self_balance: DigitalUtilityUnit,
        peer_table: &PeerTable,
        metrics: &NodeMetrics,
        mint_amount: u64,
        epoch: u64,
        base_tax_rate_pct: u64,
    ) -> EpochTransactions {
        // ── 1. Contribution ratio → tax rate ──────────────
        let ratio = metrics.contribution_ratio();
        let ratio_scaled = {
            let s = (ratio * RATIO_SCALE as f64) as u64;
            std::cmp::max(s, MIN_RATIO_SCALED)
        };

        // tax_rate_bps = base_tax_rate_bps * RATIO_SCALE / ratio_scaled
        //   ratio 2.0 → scaled 200 → 500*100/200 = 250 bps = 2.5%
        //   ratio 0.5 → scaled  50 → 500*100/50  = 1000 bps = 10%
        let base_tax_rate_bps = base_tax_rate_pct * 100; // percent → basis points
        let tax_rate_bps = base_tax_rate_bps * RATIO_SCALE / ratio_scaled;

        // ── 2. Calculate tax owed ────────────────────────
        let tax_owed = self_balance.0 * tax_rate_bps / BPS_DENOMINATOR;

        // ── 3. Build mint transaction ────────────────────
        let now = chrono::Utc::now();
        let mint = if mint_amount > 0 {
            Some(Transaction::Mint {
                to: self_peer.to_string(),
                amount: DigitalUtilityUnit(mint_amount),
                authority: self_peer.to_string(),
                nonce: 0, // caller fills in the nonce
                timestamp: now,
            })
        } else {
            None
        };

        // ── 4. Build redistribution transfers ────────────
        let peers: Vec<PeerId> = peer_table.iter()
            .filter(|p| p.heartbeats_received > 0)
            .map(|p| p.peer_id)
            .collect();
        let peer_count = peers.len();

        let mut redistributions = Vec::new();
        let mut actual_collected = 0u64;
        if tax_owed > 0 && peer_count > 0 {
            let share_per_peer = tax_owed / peer_count as u64;
            if share_per_peer > 0 {
                for peer in &peers {
                    redistributions.push(Transaction::Transfer {
                        from: self_peer.to_string(),
                        to: peer.to_string(),
                        amount: DigitalUtilityUnit(share_per_peer),
                        nonce: 0, // caller fills in the nonce
                        timestamp: now,
                    });
                }
                let redistributed_total = share_per_peer * peer_count as u64;
                actual_collected = redistributed_total;
                let remainder = tax_owed - redistributed_total;
                if remainder > 0 {
                    tracing::debug!(
                        epoch,
                        remainder,
                        "Tax remainder absorbed (below split threshold)"
                    );
                }
            }
        }

        // ── 5. Log the economic story ────────────────────
        // Log the actual amount collected, not the theoretical amount owed.
        // This prevents confusion when tax_owed > 0 but no peers exist to
        // redistribute to (isolation) — the supply dynamics stay honest.
        tracing::info!(
            epoch,
            balance_before = %self_balance,
            ratio = %format!("{:.2}", ratio),
            tax_rate_bps,
            minted = mint_amount,
            tax_calculated = tax_owed,
            tax_collected = actual_collected,
            redistributed_to = peer_count,
            redistribution_share = if peer_count > 0 {
                (actual_collected / peer_count as u64).to_string()
            } else {
                "n/a (no peers)".to_string()
            },
            "Epoch economic cycle complete"
        );

        EpochTransactions {
            mint,
            redistributions,
            ratio,
            tax_calculated: tax_owed,
            tax_collected: actual_collected,
            minted: mint_amount,
            redistributed_to: peer_count as u64,
        }
    }
}
