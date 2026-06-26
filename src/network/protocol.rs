use libp2p::{gossipsub, mdns, swarm::NetworkBehaviour};

/// The gossipsub topic all Lattice nodes subscribe to for heartbeat
/// propagation. Versioned so the wire protocol can evolve without
/// silently mixing incompatible nodes on the same topic.
pub const LATTICE_HEARTBEAT_TOPIC: &str = "lattice/heartbeat/v1";

/// Composed network behaviour for a Lattice node.
///
/// Phase 2 starts with mDNS for local peer discovery.
/// Phase 2b adds gossipsub for heartbeat propagation across the mesh.
/// Future phases add:
///   - request-response for direct peer queries
///   - Kademlia DHT for broader discovery beyond LAN
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "LatticeBehaviourEvent")]
pub struct LatticeBehaviour {
    pub mdns: mdns::tokio::Behaviour,
    pub gossipsub: gossipsub::Behaviour,
    // TODO Phase 3: pub kademlia: kad::Behaviour<MemoryStore>,
}

impl LatticeBehaviour {
    pub fn new(mdns: mdns::tokio::Behaviour, gossipsub: gossipsub::Behaviour) -> Self {
        Self { mdns, gossipsub }
    }
}

/// Events emitted by the composed behaviour.
#[derive(Debug)]
pub enum LatticeBehaviourEvent {
    Mdns(mdns::Event),
    Gossipsub(gossipsub::Event),
}

impl From<mdns::Event> for LatticeBehaviourEvent {
    fn from(event: mdns::Event) -> Self {
        LatticeBehaviourEvent::Mdns(event)
    }
}

impl From<gossipsub::Event> for LatticeBehaviourEvent {
    fn from(event: gossipsub::Event) -> Self {
        LatticeBehaviourEvent::Gossipsub(event)
    }
}
