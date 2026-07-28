# Verifier: Peer-Set Exchange Mechanisms Audit
# Date: 2026-07-28
# Scope: src/message/, src/node.rs, src/ledger/state.rs, src/api.rs, src/network/protocol.rs
# Status: COMPLETE

---

## Q1: Does any existing protocol message carry a set of PeerIds for comparison between nodes?

**VERDICT: CONTRADICTED**

No existing message type in `src/message/types.rs` carries a `Vec<PeerId>` or similar
enumerated set of peer identifiers.

Evidence — the `LatticeMessage` enum (lines 13-25) has four variants:

  - `Heartbeat(Heartbeat)` — carries `peer_count: usize` (cardinality only, no IDs)
  - `Status(StatusReport)`  — carries `peer_count: usize` and single-peer metadata (node_name,
    peer_id, max_model_size, vram_bytes, uptime_secs, version, protocol_version)
  - `Transaction(SignedTransaction)` — economic payload, no peer-list data
  - `AgentTask(AgentTaskMsg)` — task blob, no peer-list data

The request-response types defined later in the same file likewise carry at most a single
PeerId or a count:

  - `StatusRequest` / `StatusResponse`   — single peer metadata + peer_count
  - `BalanceRequest` / `BalanceResponse` — single peer's balance
  - `TransactionRequest` / `TransactionResponse` — transaction ranges, no peer sets
  - `VerifyRequest` / `VerifyResponse`   — storage challenges, no peer sets
  - `ChainRangeRequest` / `ChainRangeResponse` — block ranges
  - `WitnessRequest` / `WitnessResponse` — witness signatures

The closest existing construct is `peer_count: usize` in `Heartbeat` and `StatusReport`,
which conveys cardinality but no identity — two nodes with the same peer_count can have
completely disjoint peer sets.

---

## Q2: Is there an existing request-response protocol that a new comparison message could piggyback on?

**VERDICT: OBSERVED — seven dedicated protocols exist; none are generic.**

The `LatticeBehaviour` struct in `src/network/protocol.rs` (lines 63-90) registers these
request-response channels, each with its own Protocol/Codec/Request/Response triplet:

  | Field              | Protocol ID              | Request/Response               |
  |--------------------|--------------------------|--------------------------------|
  | rpc                | `/lattice/rpc/v1`        | StatusRequest/StatusResponse   |
  | balance_rpc        | `/lattice/balance/v1`    | BalanceRequest/BalanceResponse |
  | verify_rpc         | `/lattice/verify/v1`     | VerifyRequest/VerifyResponse   |
  | tx_rpc             | `/lattice/tx-fetch/v1`   | TransactionRequest/TransactionResponse |
  | chain_sync_rpc     | `/lattice/chain-sync/v1` | ChainRangeRequest/ChainRangeResponse |
  | witness_rpc        | `/lattice/witness/v1`    | WitnessRequest/WitnessResponse  |
  | agent_rpc          | (AGENT_STATE_PROTOCOL)   | AgentStateQuery/AgentStateReply |

Each codec implements `request_response::Codec` with fixed associated types (`type Request`,
`type Response`). There is no generic or multi-purpose channel that could accept an
arbitrary message type — the type system binds each behaviour to exactly one request type
and one response type.

Gossipsub topics (`lattice/heartbeat/v1`, `lattice/tx/v1`, `lattice/block/v1`,
`lattice/agent/v1`, `lattice/block-sync/v1`, `lattice/enclave-cert/v1`) are pub-sub
broadcast channels, not request-response, and carry `LatticeMessage` variants — none of
which hold a peer-id list (see Q1).

A new comparison protocol cannot piggyback on any existing request-response channel
without changing that channel's associated types (breaking all existing traffic on it).

---

## Q3: Does the ledger's state.rs expose a function that could compute a fingerprint over an explicitly supplied set of peers?

**VERDICT: CONTRADICTED**

The `state_root` function in `src/ledger/state.rs` (line 236):

    pub fn state_root(&self, nonces: &HashMap<PeerId, u64>) -> [u8; 32]

Iterates `self.balances` — the full balance map — in PeerId-sorted order, hashing each
(PeerId, balance) pair followed by each (PeerId, nonce) pair. The function signature
accepts no peer-set filter argument; it always processes all entries in `self.balances`.

The only other balance-query function is `balance_of(&self, peer: &PeerId)` which returns
a single peer's balance as `DigitalUtilityUnit`.

There is no function with a signature resembling:

    fn state_root_for_peers(&self, peers: &HashSet<PeerId>, nonces: &HashMap<PeerId, u64>) -> [u8; 32]

The `state_root` algorithm (Blake3 hash of sorted (PeerId, balance) + sorted (PeerId, nonce))
is defined in the function body — it could be factored out into a helper that accepts an
iterator, but no such helper exists today.

---

## Q4: Is there anything in the existing code that would structurally prevent adding a new request-response protocol?

**VERDICT: CONTRADICTED — no structural obstacle exists.**

The registration pattern is well-established and purely additive. Seven protocols already
follow exactly the same steps, visible across three files:

  a) `src/message/types.rs` — define new Request/Response structs (Serialize + Deserialize)
  b) `src/message/codec.rs` — define Protocol type (AsRef<str>), Codec type (impl Codec)
  c) `src/network/protocol.rs` — add field to LatticeBehaviour struct, variant to
     LatticeBehaviourEvent, and From impl
  d) `src/node.rs` — construct `request_response::Behaviour<NewCodec>` and pass to
     LatticeBehaviour::new (the 13-argument constructor at lines 684-698)

The `#[derive(NetworkBehaviour)]` macro on `LatticeBehaviour` (protocol.rs line 61)
automatically handles event routing — adding a new field automatically wires it in.
The manual `From` impls (lines 165-253) map each behaviour's event type to its enum
variant, and the swarm event loop in node.rs dispatches on `LatticeBehaviourEvent`
variants via a match statement.

The pattern is mechanical and extensible. Registration requires:
  - ~15 lines for types (Request + Response structs)
  - ~40 lines for codec (Protocol + Codec with read/write impls)
  - ~4 lines in LatticeBehaviour struct
  - ~4 lines in LatticeBehaviourEvent enum
  - ~6 lines for From impl
  - ~4 lines in LatticeBehaviour::new
  - ~4 lines in node.rs construction
  - Event handling in the event loop (match arm)

All existing infrastructure (CBOR codec, length-prefixed framing, Yamux streams, swarm
composition) is reused identically by new protocols — no shared state mutates.

---

## Summary Matrix

| Question | Verdict      | Key Evidence File(s)                    |
|----------|--------------|----------------------------------------|
| Q1       | CONTRADICTED | src/message/types.rs (LatticeMessage enum, all request-response types) |
| Q2       | OBSERVED     | src/network/protocol.rs (LatticeBehaviour), src/message/codec.rs |
| Q3       | CONTRADICTED | src/ledger/state.rs (state_root fn, balance_of fn) |
| Q4       | CONTRADICTED | src/network/protocol.rs (derive + impls), src/node.rs (construction, lines 599-698) |
