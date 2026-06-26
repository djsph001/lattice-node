use libp2p::{gossipsub, mdns, request_response, swarm::NetworkBehaviour};

use crate::message::codec::rpc::LatticeCodec;

/// The gossipsub topic all Lattice nodes subscribe to for heartbeat
/// propagation. Versioned so the wire protocol can evolve without
/// silently mixing incompatible nodes on the same topic.
pub const LATTICE_HEARTBEAT_TOPIC: &str = "lattice/heartbeat/v1";

/// Composed network behaviour for a Lattice node.
///
/// Phase 2 starts with mDNS for local peer discovery.
/// Phase 2b adds gossipsub for heartbeat propagation across the mesh.
/// Phase 2c adds request-response for direct peer queries (the handshake
/// channel, complementing gossipsub's fire-and-forget broadcast).
/// Future phases add:
///   - Kademlia DHT for broader discovery beyond LAN
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "LatticeBehaviourEvent")]
pub struct LatticeBehaviour {
    pub mdns: mdns::tokio::Behaviour,
    pub gossipsub: gossipsub::Behaviour,
    pub rpc: request_response::Behaviour<LatticeCodec>,
    // TODO Phase 3: pub kademlia: kad::Behaviour<MemoryStore>,
}

impl LatticeBehaviour {
    pub fn new(
        mdns: mdns::tokio::Behaviour,
        gossipsub: gossipsub::Behaviour,
        rpc: request_response::Behaviour<LatticeCodec>,
    ) -> Self {
        Self {
            mdns,
            gossipsub,
            rpc,
        }
    }
}

/// Events emitted by the composed behaviour.
#[derive(Debug)]
pub enum LatticeBehaviourEvent {
    Mdns(mdns::Event),
    Gossipsub(gossipsub::Event),
    Rpc(request_response::Event<crate::message::types::StatusRequest, crate::message::types::StatusResponse>),
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
