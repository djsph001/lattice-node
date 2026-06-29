use libp2p::{gossipsub, kad, mdns, request_response, swarm::NetworkBehaviour};

use crate::message::codec::rpc::LatticeCodec;

/// The gossipsub topic all Lattice nodes subscribe to for heartbeat
/// propagation. Versioned so the wire protocol can evolve without
/// silently mixing incompatible nodes on the same topic.
pub const LATTICE_HEARTBEAT_TOPIC: &str = "lattice/heartbeat/v1";

/// Kademlia protocol name — versioned for the same reason as the
/// heartbeat topic: wire-level evolution without silent breakage.
pub const LATTICE_KAD_PROTOCOL: &str = "/lattice/kad/v1";

/// Composed network behaviour for a Lattice node.
///
/// Phase 2 starts with mDNS for local peer discovery.
/// Phase 2b adds gossipsub for heartbeat propagation across the mesh.
/// Phase 2c adds request-response for direct peer queries (the handshake
/// channel, complementing gossipsub's fire-and-forget broadcast).
/// Phase 3 adds Kademlia DHT for broader discovery beyond LAN.
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "LatticeBehaviourEvent")]
pub struct LatticeBehaviour {
    pub mdns: mdns::tokio::Behaviour,
    pub gossipsub: gossipsub::Behaviour,
    pub rpc: request_response::Behaviour<LatticeCodec>,
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
}

impl LatticeBehaviour {
    pub fn new(
        mdns: mdns::tokio::Behaviour,
        gossipsub: gossipsub::Behaviour,
        rpc: request_response::Behaviour<LatticeCodec>,
        kademlia: kad::Behaviour<kad::store::MemoryStore>,
    ) -> Self {
        Self {
            mdns,
            gossipsub,
            rpc,
            kademlia,
        }
    }
}

/// Events emitted by the composed behaviour.
#[derive(Debug)]
pub enum LatticeBehaviourEvent {
    Mdns(mdns::Event),
    Gossipsub(gossipsub::Event),
    Rpc(
        request_response::Event<
            crate::message::types::StatusRequest,
            crate::message::types::StatusResponse,
        >,
    ),
    Kad(kad::Event),
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

impl
    From<
        request_response::Event<
            crate::message::types::StatusRequest,
            crate::message::types::StatusResponse,
        >,
    > for LatticeBehaviourEvent
{
    fn from(
        event: request_response::Event<
            crate::message::types::StatusRequest,
            crate::message::types::StatusResponse,
        >,
    ) -> Self {
        LatticeBehaviourEvent::Rpc(event)
    }
}

impl From<kad::Event> for LatticeBehaviourEvent {
    fn from(event: kad::Event) -> Self {
        LatticeBehaviourEvent::Kad(event)
    }
}
