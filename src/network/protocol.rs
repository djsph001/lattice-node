use libp2p::{mdns, swarm::NetworkBehaviour};

/// Composed network behaviour for a Lattice node.
///
/// Phase 2 starts with mDNS for local peer discovery.
/// Future phases add:
///   - gossipsub for message propagation
///   - request-response for direct peer queries
///   - Kademlia DHT for broader discovery beyond LAN
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "LatticeBehaviourEvent")]
pub struct LatticeBehaviour {
    pub mdns: mdns::tokio::Behaviour,
    // TODO Phase 3: pub gossipsub: gossipsub::Behaviour,
    // TODO Phase 4: pub kademlia: kad::Behaviour<MemoryStore>,
}

impl LatticeBehaviour {
    pub fn new(mdns: mdns::tokio::Behaviour) -> Self {
        Self { mdns }
    }
}

/// Events emitted by the composed behaviour.
#[derive(Debug)]
pub enum LatticeBehaviourEvent {
    Mdns(mdns::Event),
}

impl From<mdns::Event> for LatticeBehaviourEvent {
    fn from(event: mdns::Event) -> Self {
        LatticeBehaviourEvent::Mdns(event)
    }
}
