use libp2p::{gossipsub, kad, mdns, relay, dcutr, request_response, swarm::NetworkBehaviour};

use crate::message::codec::rpc::{BalanceCodec, LatticeCodec};
use crate::message::codec::rpc::VerifyCodec;
use crate::message::types::{BalanceRequest, BalanceResponse, StatusRequest, StatusResponse};
use crate::message::types::{VerifyRequest, VerifyResponse};

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
/// Phase 4 adds a second request-response channel for balance queries
/// and the economic primitives layer.
/// Phase 6 adds a third request-response channel for storage
/// verification — the peer-verified contribution claims layer.
/// Phase 6c adds relay client support (for firewalled nodes behind
/// p2p-circuits) and DCUtR (for automatic NAT hole-punching).
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "LatticeBehaviourEvent")]
pub struct LatticeBehaviour {
    pub mdns: mdns::tokio::Behaviour,
    pub gossipsub: gossipsub::Behaviour,
    pub rpc: request_response::Behaviour<LatticeCodec>,
    pub balance_rpc: request_response::Behaviour<BalanceCodec>,
    pub verify_rpc: request_response::Behaviour<VerifyCodec>,
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
    /// Relay client behaviour — enables firewalled nodes to
    /// communicate through publicly reachable relay peers via
    /// `/p2p-circuit` reservations.
    pub relay_client: relay::client::Behaviour,
    /// Direct Connection Upgrade through Relay — ambiently
    /// attempts to upgrade a relayed connection to a direct
    /// hole-punched path when firewalls permit.
    pub dcutr: dcutr::Behaviour,
}

impl LatticeBehaviour {
    pub fn new(
        mdns: mdns::tokio::Behaviour,
        gossipsub: gossipsub::Behaviour,
        rpc: request_response::Behaviour<LatticeCodec>,
        balance_rpc: request_response::Behaviour<BalanceCodec>,
        verify_rpc: request_response::Behaviour<VerifyCodec>,
        kademlia: kad::Behaviour<kad::store::MemoryStore>,
        relay_client: relay::client::Behaviour,
        dcutr: dcutr::Behaviour,
    ) -> Self {
        Self {
            mdns,
            gossipsub,
            rpc,
            balance_rpc,
            verify_rpc,
            kademlia,
            relay_client,
            dcutr,
        }
    }
}

/// Events emitted by the composed behaviour.
#[derive(Debug)]
pub enum LatticeBehaviourEvent {
    Mdns(mdns::Event),
    Gossipsub(gossipsub::Event),
    Rpc(request_response::Event<StatusRequest, StatusResponse>),
    BalanceRpc(request_response::Event<BalanceRequest, BalanceResponse>),
    VerifyRpc(request_response::Event<VerifyRequest, VerifyResponse>),
    Kad(kad::Event),
    /// Relay client events — reservation lifecycle, circuit
    /// establishment and teardown.
    RelayClient(relay::client::Event),
    /// DCUtR events — hole-punch attempts and outcomes.
    Dcutr(dcutr::Event),
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

impl From<request_response::Event<StatusRequest, StatusResponse>>
    for LatticeBehaviourEvent
{
    fn from(
        event: request_response::Event<StatusRequest, StatusResponse>,
    ) -> Self {
        LatticeBehaviourEvent::Rpc(event)
    }
}

impl From<request_response::Event<BalanceRequest, BalanceResponse>>
    for LatticeBehaviourEvent
{
    fn from(
        event: request_response::Event<BalanceRequest, BalanceResponse>,
    ) -> Self {
        LatticeBehaviourEvent::BalanceRpc(event)
    }
}

impl From<kad::Event> for LatticeBehaviourEvent {
    fn from(event: kad::Event) -> Self {
        LatticeBehaviourEvent::Kad(event)
    }
}

impl From<request_response::Event<VerifyRequest, VerifyResponse>>
    for LatticeBehaviourEvent
{
    fn from(
        event: request_response::Event<VerifyRequest, VerifyResponse>,
    ) -> Self {
        LatticeBehaviourEvent::VerifyRpc(event)
    }
}

impl From<relay::client::Event> for LatticeBehaviourEvent {
    fn from(event: relay::client::Event) -> Self {
        LatticeBehaviourEvent::RelayClient(event)
    }
}

impl From<dcutr::Event> for LatticeBehaviourEvent {
    fn from(event: dcutr::Event) -> Self {
        LatticeBehaviourEvent::Dcutr(event)
    }
}
