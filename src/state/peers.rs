use std::collections::HashMap;

use chrono::{DateTime, Utc};
use libp2p::{Multiaddr, PeerId};
use tracing::debug;

/// Tracked state for a single peer in the mesh.
#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub peer_id: PeerId,
    pub addresses: Vec<Multiaddr>,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub heartbeats_received: u64,
    /// The epoch when the last heartbeat was received from this peer.
    /// 0 = never received a heartbeat.
    pub last_heartbeat_epoch: u64,
    /// Which cells this peer participates in.
    /// Empty = not a member of any cell (plain mesh node).
    /// Multiple entries = peer participates in multiple cells.
    /// This field records *declared participation* only — no trust,
    /// no authorization, no relationship is implied by its presence.
    pub cell_participations: Vec<crate::message::types::CellType>,
    /// Whether this peer operates Cell Network infrastructure
    /// (e.g. a relay or witness node that is not itself a cell).
    pub is_infrastructure: bool,
    /// Human-readable declared purpose (appears in dashboard).
    pub declared_purpose: Option<String>,
}

/// In-memory peer table tracking all known nodes in the mesh.
///
/// Phase 2: purely in-memory, rebuilt on restart via mDNS.
/// Future phases: persist to disk for faster reconnection,
/// add reputation scoring for the economic protocol layer.
#[derive(Debug)]
pub struct PeerTable {
    peers: HashMap<PeerId, PeerInfo>,
}

impl PeerTable {
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
        }
    }

    /// Add or update a peer when discovered.
    pub fn add_peer(&mut self, peer_id: PeerId, addr: Multiaddr, current_epoch: u64) {
        let now = Utc::now();
        self.peers
            .entry(peer_id)
            .and_modify(|info| {
                info.last_seen = now;
                if !info.addresses.contains(&addr) {
                    info.addresses.push(addr.clone());
                }
            })
            .or_insert_with(|| {
                debug!(peer = %peer_id, "New peer added to table");
                PeerInfo {
                    peer_id,
                    addresses: vec![addr],
                    first_seen: now,
                    last_seen: now,
                    heartbeats_received: 0,
                    last_heartbeat_epoch: current_epoch,
                    cell_participations: Vec::new(),
                    is_infrastructure: false,
                    declared_purpose: None,
                }
            });
    }

    /// Insert a peer with no known address — used when a peer is discovered
    /// via Kademlia or gossip before a direct connection. Addresses populate
    /// later via Identify or on connection.
    pub fn insert_peer(&mut self, peer_id: PeerId, current_epoch: u64) {
        if self.peers.contains_key(&peer_id) {
            return;
        }
        let now = Utc::now();
        debug!(peer = %peer_id, "New peer inserted into table (no address yet)");
        self.peers.insert(
            peer_id,
            PeerInfo {
                peer_id,
                addresses: vec![],
                first_seen: now,
                last_seen: now,
                heartbeats_received: 0,
                last_heartbeat_epoch: current_epoch,
                cell_participations: Vec::new(),
                is_infrastructure: false,
                declared_purpose: None,
            },
        );
    }

    /// Remove a peer when it expires or disconnects.
    pub fn remove_peer(&mut self, peer_id: &PeerId) {
        if self.peers.remove(peer_id).is_some() {
            debug!(peer = %peer_id, "Peer removed from table");
        }
    }

    /// Record receipt of a heartbeat from a peer.
    pub fn record_heartbeat(&mut self, peer_id: &PeerId) {
        if let Some(info) = self.peers.get_mut(peer_id) {
            info.last_seen = Utc::now();
            info.heartbeats_received += 1;
        }
    }

    /// Record receipt of a heartbeat with the current epoch number.
    /// Called from the node's heartbeat handler which knows the current epoch.
    pub fn record_heartbeat_epoch(&mut self, peer_id: &PeerId, epoch: u64) {
        if let Some(info) = self.peers.get_mut(peer_id) {
            info.last_seen = Utc::now();
            info.heartbeats_received += 1;
            info.last_heartbeat_epoch = epoch;
        }
    }

    /// Number of currently tracked peers.
    pub fn len(&self) -> usize {
        self.peers.len()
    }

    /// Whether the peer table is empty.
    pub fn is_empty(&self) -> bool {
        self.peers.is_empty()
    }

    /// Iterate over all known peers.
    pub fn iter(&self) -> impl Iterator<Item = &PeerInfo> {
        self.peers.values()
    }

    /// Get info for a specific peer.
    pub fn get(&self, peer_id: &PeerId) -> Option<&PeerInfo> {
        self.peers.get(peer_id)
    }

    /// Get mutable info for a specific peer.
    pub fn get_mut(&mut self, peer_id: &PeerId) -> Option<&mut PeerInfo> {
        self.peers.get_mut(peer_id)
    }

    /// Return peers that haven't been seen within the given duration.
    pub fn stale_peers(&self, timeout: chrono::Duration) -> Vec<PeerId> {
        let cutoff = Utc::now() - timeout;
        self.peers
            .values()
            .filter(|info| info.last_seen < cutoff)
            .map(|info| info.peer_id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_peer_id() -> PeerId {
        PeerId::random()
    }

    fn test_addr() -> Multiaddr {
        "/ip4/192.168.1.100/tcp/4001".parse().unwrap()
    }

    #[test]
    fn add_and_retrieve_peer() {
        let mut table = PeerTable::new();
        let pid = test_peer_id();
        let addr = test_addr();

        table.add_peer(pid, addr.clone());

        assert_eq!(table.len(), 1);
        let info = table.get(&pid).unwrap();
        assert_eq!(info.addresses, vec![addr]);
        assert_eq!(info.heartbeats_received, 0);
    }

    #[test]
    fn remove_peer() {
        let mut table = PeerTable::new();
        let pid = test_peer_id();
        table.add_peer(pid, test_addr());
        table.remove_peer(&pid);
        assert!(table.is_empty());
    }

    #[test]
    fn record_heartbeat_increments() {
        let mut table = PeerTable::new();
        let pid = test_peer_id();
        table.add_peer(pid, test_addr());
        table.record_heartbeat(&pid);
        table.record_heartbeat(&pid);

        let info = table.get(&pid).unwrap();
        assert_eq!(info.heartbeats_received, 2);
    }
}
