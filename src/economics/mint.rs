// ── economics/mint.rs — contribution-based issuance ─────────────
//
// The Lattice does not have a central bank.  Units are created by
// nodes that *do the work* — relaying traffic, storing DHT records,
// propagating messages.  At each epoch boundary every node evaluates
// its own contribution since the last epoch and mints proportionally.
//
// Self-minting is safe because every metric is verifiable by peers
// (Phase 6).  A node that claims to have relayed bytes it didn't
// actually relay will eventually be contradicted by its neighbours'
// observations.  For Phase 5, nodes are honest reporters.
//
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

/// Compute the mint amount for an epoch based on this node's
/// contribution metrics.
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
///
/// `base_rate` is a configurable constant (default 10 units per epoch
/// at a contribution score of 1.0).  All weights start equal so every
/// form of contribution is valued the same; tune them as the network
/// reveals which signals matter most.
pub fn calculate_mint(
    metrics: &NodeMetrics,
    base_rate: u64,
) -> u64 {
    let contribution_score = RELAY_WEIGHT * metrics.bytes_relayed as f64
        + ROUTE_WEIGHT * metrics.dht_records_stored as f64
        + PROPAGATION_WEIGHT * metrics.messages_propagated as f64;

    let mint_amount = base_rate as f64 * contribution_score;

    // Floor at zero — a node that contributed nothing gets nothing.
    if mint_amount <= 0.0 {
        return 0;
    }

    mint_amount as u64
}

// TODO Phase 6: peer-verified contribution claims.
// When peers can attest to each other's metrics, self-minting
// gains cryptographic integrity.  Until then, honest reporting
// is assumed — the economic incentives already point toward
// contribution (tax rate is lower for contributors), so a node
// that inflates its metrics is playing against its own long-term
// interest in a healthy mesh.
