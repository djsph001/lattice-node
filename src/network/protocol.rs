// Phase 6c: relay client for NAT traversal and p2p-circuit support.
// DCUtR re-add deferred — relay must be proven first before hole-punch
// upgrade layer can be re-enabled.
use libp2p::{gossipsub, identify, kad, mdns, relay, request_response, swarm::NetworkBehaviour};
use libp2p::swarm::behaviour::toggle::Toggle;

use crate::message::codec::rpc::{ChainSyncCodec, BalanceCodec, LatticeCodec};
use crate::message::codec::rpc::{TransactionCodec, VerifyCodec};
use crate::message::types::{BalanceRequest, BalanceResponse, StatusRequest, StatusResponse};
use crate::message::types::{TransactionRequest, TransactionResponse};
use crate::message::types::WireBlock;
use crate::message::types::{ChainRangeRequest, ChainRangeResponse};
use crate::message::types::{VerifyRequest, VerifyResponse};
use crate::agent::codec::{AgentStateCodec, AGENT_STATE_PROTOCOL};
use crate::agent::state::{AgentStateQuery, AgentStateReply};

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

/// Gossipsub topic for agent task propagation (Phase 8).
pub const LATTICE_AGENT_TOPIC: &str = "lattice/agent/v1";

/// Gossipsub topic for chain block propagation.
/// Blocks committed by any node are broadcast here, enabling
/// the chain to be a distributed structure rather than a
/// per-node local ledger. Peers validate and append contiguous
/// blocks; divergent blocks trigger fork resolution.
pub const LATTICE_BLOCK_TOPIC: &str = "lattice/block/v1";

/// Gossipsub topic for block sync requests and responses.
/// Nodes request block ranges when they detect they're behind;
/// peers serve blocks from their local chain.
pub const LATTICE_BLOCK_SYNC_TOPIC: &str = "lattice/block-sync/v1";

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
    /// Phase 6c: relay client for p2p-circuit NAT traversal.
    pub relay_client: relay::client::Behaviour,
    /// Phase 6c: relay server — when enabled (via Toggle), this node
    /// accepts and forwards relay circuits for other nodes.  Disabled
    /// for ordinary nodes that only use the relay client.
    pub relay_server: Toggle<relay::Behaviour>,
    /// Identify protocol — enables peers to exchange protocol
    /// support information.  Required for relay client to discover
    /// relay-capable peers.
    pub identify: identify::Behaviour,
    /// Phase 8: agent state query RPC.
    pub agent_rpc: request_response::Behaviour<AgentStateCodec>,
    /// Phase 4: transaction fetch RPC — request missing transactions
    /// by (signer, nonce) coordinate to close gaps.
    pub tx_rpc: request_response::Behaviour<TransactionCodec>,
    /// Phase 10: chain sync RPC — request a range of certificate-chain
    /// blocks from a peer to catch up when behind the mesh tip.
    pub chain_sync_rpc: request_response::Behaviour<ChainSyncCodec>,
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
        relay_server: Toggle<relay::Behaviour>,
        identify: identify::Behaviour,
        agent_rpc: request_response::Behaviour<AgentStateCodec>,
        tx_rpc: request_response::Behaviour<TransactionCodec>,
        chain_sync_rpc: request_response::Behaviour<ChainSyncCodec>,
    ) -> Self {
        Self {
            mdns,
            gossipsub,
            rpc,
            balance_rpc,
            verify_rpc,
            kademlia,
            relay_client,
            relay_server,
            identify,
            agent_rpc,
            tx_rpc,
            chain_sync_rpc,
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
    /// Phase 6c: relay client events (reservation, circuit establishment).
    RelayClient(relay::client::Event),
    /// Phase 6c: relay server events — inbound reservation/circuit
    /// requests from other nodes that this node is relaying for.
    RelayServer(relay::Event),
    /// Identify protocol events — enables discovery of relay-capable
    /// peers and other protocol support information.
    Identify(identify::Event),
    /// Phase 8: agent state query events.
    AgentRpc(request_response::Event<AgentStateQuery, AgentStateReply>),
    /// Phase 4: transaction fetch RPC events.
    TxRpc(request_response::Event<TransactionRequest, TransactionResponse>),
    /// Phase 10: chain sync RPC events.
    ChainSyncRpc(request_response::Event<ChainRangeRequest, ChainRangeResponse>),
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

impl From<relay::client::Event> for LatticeBehaviourEvent {
    fn from(event: relay::client::Event) -> Self {
        LatticeBehaviourEvent::RelayClient(event)
    }
}

impl From<relay::Event> for LatticeBehaviourEvent {
    fn from(event: relay::Event) -> Self {
        LatticeBehaviourEvent::RelayServer(event)
    }
}

impl From<identify::Event> for LatticeBehaviourEvent {
    fn from(event: identify::Event) -> Self {
        LatticeBehaviourEvent::Identify(event)
    }
}

impl From<request_response::Event<AgentStateQuery, AgentStateReply>>
    for LatticeBehaviourEvent
{
    fn from(
        event: request_response::Event<AgentStateQuery, AgentStateReply>,
    ) -> Self {
        LatticeBehaviourEvent::AgentRpc(event)
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

impl From<request_response::Event<TransactionRequest, TransactionResponse>>
    for LatticeBehaviourEvent
{
    fn from(
        event: request_response::Event<TransactionRequest, TransactionResponse>,
    ) -> Self {
        LatticeBehaviourEvent::TxRpc(event)
    }
}

impl From<request_response::Event<ChainRangeRequest, ChainRangeResponse>>
    for LatticeBehaviourEvent
{
    fn from(event: request_response::Event<ChainRangeRequest, ChainRangeResponse>) -> Self {
        LatticeBehaviourEvent::ChainSyncRpc(event)
    }
}
