// FIXME(phase-6c): relay_client and dcutr temporarily removed due to
// libp2p-relay 0.18 race condition (transport channel closed before
// behaviour polled). Tracked as: relay startup panic blocks mesh test.
// Re-add before cross-internet NAT traversal testing with Mac/Windows nodes.
use libp2p::{gossipsub, kad, mdns, request_response, swarm::NetworkBehaviour};

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

/// Gossipsub topic for TCP v0.1.0 enclave certificate propagation.
/// Impact Certificates produced by the Python sandbox (tfb:) are
/// broadcast on this topic after validation by the local node.
pub const LATTICE_ENCLAVE_CERT_TOPIC: &str = "lattice/enclave-cert/v1";

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
///
/// NOTE: Phase 6c relay/DCUtR temporarily removed due to libp2p-relay
/// race condition. See FIXME at top of file.
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "LatticeBehaviourEvent")]
pub struct LatticeBehaviour {
    pub mdns: mdns::tokio::Behaviour,
    pub gossipsub: gossipsub::Behaviour,
    pub rpc: request_response::Behaviour<LatticeCodec>,
    pub balance_rpc: request_response::Behaviour<BalanceCodec>,
    pub verify_rpc: request_response::Behaviour<VerifyCodec>,
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
}

impl LatticeBehaviour {
    pub fn new(
        mdns: mdns::tokio::Behaviour,
        gossipsub: gossipsub::Behaviour,
        rpc: request_response::Behaviour<LatticeCodec>,
        balance_rpc: request_response::Behaviour<BalanceCodec>,
        verify_rpc: request_response::Behaviour<VerifyCodec>,
        kademlia: kad::Behaviour<kad::store::MemoryStore>,
    ) -> Self {
        Self {
            mdns,
            gossipsub,
            rpc,
            balance_rpc,
            verify_rpc,
            kademlia,
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
