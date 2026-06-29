// ── economics/metrics.rs — the measurement layer ───────────────────
//
// Every economic decision in the Lattice starts here.  This module
// tracks what each node *contributes* to the mesh and what it
// *consumes* from the mesh.  A single `contribution_ratio()` call
// collapses those two signals into one number that drives the entire
// Georgist feedback loop (minting reward and tax rate).
//
// Phase 5: honest self-reporting.  Phase 6 adds peer-verified
// contribution claims so a node that lies about its relay volume can
// be contradicted by its neighbours.

/// A node's tracked contribution and consumption signals.
///
/// Updated continuously during the event loop.  At each epoch boundary
/// the `EconomicEngine` reads the current values, computes deltas
/// against the last snapshot, and resets the per-epoch counters.
#[derive(Debug, Clone)]
pub struct NodeMetrics {
    // ── contribution signals ──────────────────────────────
    /// Total bytes this node has forwarded for others (gossipsub relay,
    /// Kademlia routing responses).  This is the single strongest
    /// contribution signal — bandwidth is the network's physical resource.
    pub bytes_relayed: u64,

    /// Total gossipsub messages forwarded.  Overlaps with
    /// `bytes_relayed` but captures participation in propagation
    /// independently of message size — a node forwarding many small
    /// messages is still serving the mesh.
    pub messages_propagated: u64,

    /// Number of DHT records this node is currently holding for the
    /// network.  Approximated by counting Kademlia routing-table
    /// additions (Phase 5 proxy; Phase 6 will query the record store
    /// directly).
    pub dht_records_stored: u64,

    /// Heartbeats this node has broadcast.  Already tracked in
    /// `LatticeNode` — this field mirrors it for the economic model.
    /// Staying alive and visible is a minimum contribution.
    pub heartbeats_sent: u64,

    /// Economic transactions forwarded for others via gossipsub.
    pub transactions_relayed: u64,

    // ── consumption signals ───────────────────────────────
    /// Traffic this node *generated* that other nodes had to carry
    /// (heartbeat broadcasts, status requests, transaction submissions).
    pub bytes_consumed: u64,

    /// Request-response and DHT queries this node initiated.  Each
    /// query costs other nodes work (CPU, I/O, bandwidth).
    pub queries_issued: u64,

    /// Economic transactions this node authored (mints and transfers).
    /// Each one consumes network propagation capacity.
    pub transactions_submitted: u64,
}

impl NodeMetrics {
    /// Fresh metrics — all counters at zero.
    pub fn new() -> Self {
        Self {
            bytes_relayed: 0,
            messages_propagated: 0,
            dht_records_stored: 0,
            heartbeats_sent: 0,
            transactions_relayed: 0,
            bytes_consumed: 0,
            queries_issued: 0,
            transactions_submitted: 0,
        }
    }

    /// The node's give-to-take ratio.
    ///
    /// A single number that captures whether this node is a net
    /// contributor (>1.0) or a net consumer (<1.0) of the mesh's
    /// shared resources.
    ///
    /// Numerator: contributions (relay, propagation, DHT storage).
    /// Denominator: consumption (traffic generated, queries issued).
    ///
    /// When both contributions and consumption are zero (the node
    /// hasn't participated yet), returns 1.0 — a neutral ratio.
    /// A new node is not a freeloader; it just hasn't had time to
    /// contribute.
    pub fn contribution_ratio(&self) -> f64 {
        let contributions =
            self.bytes_relayed + self.messages_propagated + self.dht_records_stored;
        let consumption = self.bytes_consumed + self.queries_issued;

        // Fresh node — neutral ratio, not punitive.
        if contributions == 0 && consumption == 0 {
            return 1.0;
        }

        contributions as f64 / std::cmp::max(1, consumption) as f64
    }

    /// Record a relay event: `size` bytes forwarded for another node.
    pub fn record_relay(&mut self, size: u64) {
        self.bytes_relayed += size;
        self.messages_propagated += 1;
    }

    /// Record a transaction we relayed for someone else.
    pub fn record_transaction_relayed(&mut self) {
        self.transactions_relayed += 1;
    }

    /// Record a DHT routing-table addition (proxy for record storage).
    pub fn record_dht_record_stored(&mut self) {
        self.dht_records_stored += 1;
    }

    /// Record consumption: `size` bytes of traffic we generated.
    pub fn record_consumption(&mut self, size: u64) {
        self.bytes_consumed += size;
    }

    /// Record a query we initiated (status request, balance query,
    /// Kademlia lookup).
    pub fn record_query_issued(&mut self) {
        self.queries_issued += 1;
    }

    /// Record a transaction we authored and submitted.
    pub fn record_transaction_submitted(&mut self) {
        self.transactions_submitted += 1;
    }
}

impl Default for NodeMetrics {
    fn default() -> Self {
        Self::new()
    }
}
