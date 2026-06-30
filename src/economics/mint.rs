// ── economics/mint.rs — contribution-based issuance ─────────────
//
// The Lattice does not have a central bank.  Units are created by
// nodes that *do the work* — relaying traffic, storing DHT records,
// propagating messages.  At each epoch boundary every node evaluates
// its own contribution since the last epoch and mints proportionally.
//
// Phase 5: honest self-reporting.
// Phase 6: peer-verified contribution claims via signed receipts.
// When receipts are available, the mint calculation uses ONLY
// verified metrics — self-reported numbers become diagnostics.

// ── Policy constants (governance decisions, not magic numbers) ──

/// Weight applied to bytes relayed in the contribution score.
/// Bytes are the network's physical resource — relay is the
/// strongest contribution signal.
const RELAY_WEIGHT: f64 = 1.0;

/// Weight applied to DHT records stored.  Storing records for
/// others enables the mesh's routing plane.
const ROUTE_WEIGHT: f64 = 1.0;

/// Weight applied to gossipsub messages propagated.  Captures
/// participation in the broadcast layer independently of message
/// size.
const PROPAGATION_WEIGHT: f64 = 1.0;

use super::metrics::NodeMetrics;

/// Compute the mint amount from self-reported metrics.
///
/// Used when no peer receipts are available (solo operation,
/// or before any receipts have been collected).
///
/// # Formula
///
/// ```text
/// contribution_score =
///     RELAY_WEIGHT       * bytes_relayed_this_epoch
///   + ROUTE_WEIGHT       * dht_records_stored
///   + PROPAGATION_WEIGHT * messages_propagated_this_epoch
///
/// mint_amount = base_rate * contribution_score
/// ```
pub fn calculate_mint(
    metrics: &NodeMetrics,
    base_rate: u64,
) -> u64 {
    let contribution_score = RELAY_WEIGHT * metrics.bytes_relayed as f64
        + ROUTE_WEIGHT * metrics.dht_records_stored as f64
        + PROPAGATION_WEIGHT * metrics.messages_propagated as f64;

    let mint_amount = base_rate as f64 * contribution_score;

    if mint_amount <= 0.0 {
        return 0;
    }

    mint_amount as u64
}

/// Compute the mint amount from peer-verified receipt metrics.
///
/// Phase 6: trustless minting.  Only bytes and messages confirmed
/// by at least one peer's signed receipt count toward the mint.
/// Self-reported metrics are ignored when receipts exist.
///
/// Falls back to self-reported metrics if no receipts have been
/// collected (solo node operation).
pub fn calculate_mint_from_receipts(
    metrics: &NodeMetrics,
    base_rate: u64,
) -> u64 {
    // If we have verified metrics, use them exclusively.
    if metrics.verified_bytes_relayed > 0 || metrics.verified_messages_relayed > 0 {
        let verified_score = RELAY_WEIGHT * metrics.verified_bytes_relayed as f64
            + PROPAGATION_WEIGHT * metrics.verified_messages_relayed as f64;

        let mint_amount = base_rate as f64 * verified_score;
        if mint_amount <= 0.0 {
            return 0;
        }
        return mint_amount as u64;
    }

    // No receipts yet — fall back to self-reported (solo node).
    calculate_mint(metrics, base_rate)
}
