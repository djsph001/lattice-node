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
/// Phase 5 scaffolding. Not used in the production mint path —
/// self-reported metrics are diagnostics, not economic inputs.
/// Retained for testing only.
#[cfg(test)]
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
///
/// Returns 0 when no verified receipts are available — self-reported
/// metrics are diagnostics, not economic inputs. A node with no peers
/// to witness its relay work correctly earns nothing.
pub fn calculate_mint_from_receipts(
    metrics: &NodeMetrics,
    base_rate: u64,
) -> u64 {
    let verified_score = RELAY_WEIGHT * metrics.verified_bytes_relayed as f64
        + PROPAGATION_WEIGHT * metrics.verified_messages_relayed as f64;

    if verified_score <= 0.0 {
        return 0;
    }

    let mint_amount = base_rate as f64 * verified_score;
    mint_amount as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::metrics::NodeMetrics;

    #[test]
    fn receipt_gated_mint_returns_zero_without_verified_metrics() {
        let metrics = NodeMetrics::new();
        assert_eq!(calculate_mint_from_receipts(&metrics, 10), 0);
    }

    #[test]
    fn receipt_gated_mint_returns_nonzero_with_verified_metrics() {
        let mut metrics = NodeMetrics::new();
        metrics.verified_bytes_relayed = 1000;
        metrics.verified_messages_relayed = 5;
        assert_eq!(calculate_mint_from_receipts(&metrics, 10), 10050);
    }

    #[test]
    fn self_reported_mint_retained_as_scaffolding() {
        let mut metrics = NodeMetrics::new();
        metrics.bytes_relayed = 500;
        assert_eq!(calculate_mint(&metrics, 10), 5000);
    }
}
