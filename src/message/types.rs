use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Top-level message envelope for all Lattice protocol messages.
///
/// Every message between nodes is wrapped in this enum.
/// CBOR-encoded on the wire for compact, schema-evolvable transport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LatticeMessage {
    /// Periodic liveness signal
    Heartbeat(Heartbeat),

    /// Node status report (richer than heartbeat)
    Status(StatusReport),

    // === Future phases ===
    // /// Digital utility unit transaction
    // Transaction(Transaction),
    //
    // /// Governance proposal or vote
    // Governance(GovernanceAction),
}

/// Lightweight liveness signal broadcast at regular intervals.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeat {
    /// Human-readable node name
    pub node_name: String,
    /// Peer ID as string
    pub peer_id: String,
    /// When the heartbeat was generated
    pub timestamp: DateTime<Utc>,
    /// Number of peers this node currently sees
    pub peer_count: usize,
}

/// Richer status report, requested or broadcast less frequently.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusReport {
    pub node_name: String,
    pub peer_id: String,
    pub timestamp: DateTime<Utc>,
    pub peer_count: usize,
    /// Uptime in seconds since node start
    pub uptime_secs: u64,
    /// Software version
    pub version: String,
    /// Lattice protocol version for compatibility checks
    pub protocol_version: u32,
}

impl StatusReport {
    pub fn protocol_compatible(&self, other: &StatusReport) -> bool {
        self.protocol_version == other.protocol_version
    }
}
