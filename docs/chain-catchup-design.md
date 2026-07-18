# G: Chain Catch-Up Sync (ChainRangeRequest Protocol)

> Design from collaborator, 2026-07-18.
> Converged before implementation.

## Problem

Block propagation (82988d8) handles forward distribution — new blocks are published and received. But a node that falls behind (restart, sleep, partition) has no way to fetch missed blocks. Same shape as the gapped-nonce fetch protocol, one level up.

## Design

### Triggers (both wired)

1. **Gossip trigger** — receiving a block via gossip with height > local_tip + 1 (the "future block" branch of the four-case handler, currently WARN-logs)
2. **Status trigger** — status response showing a peer's tip > local_tip

### Message Types

```rust
pub struct ChainRangeRequest {
    pub from_height: u64,   // inclusive
    pub to_height: u64,     // inclusive
}

pub struct ChainRangeResponse {
    pub blocks: Vec<BlockFrame>,
    pub complete: bool,  // false if capped
}
```

### Response Size Cap

Min(100 blocks, 5MB). `complete=false` if capped, requester makes follow-up request.

### Target Selection

The peer whose gossip triggered the catch-up is the primary target. Fallback on timeout. Dedup via parallel `outstanding_chain_requests` map (same shape as fetch protocol).

### Trust on Receive

Every block gets full validation: signature check, parent hash match to previous block in range (or local tip for first), block type validity for height. Apply in order.

**On failure:** keep successfully-applied blocks, reject the failing block and everything after it, re-request the remaining range from a different peer.

### Broadcast Suppression

Add `commit_from_catchup()` variant that skips the gossip publish. Avoids re-broadcast bursts during mass catch-up.

### Integration Points

- RPC codec: new variant on the request/response enum alongside TransactionRequest/Response
- Four-case block handler: future branch triggers range request instead of WARN-log
- Outstanding fetches: parallel `outstanding_chain_requests: HashMap<RangeKey, Instant>`
- commit(): new `commit_from_catchup()` or flag to suppress publish

## Tests

- **Positive**: tip=5, receives block at height=10 via gossip → request 6..10 → apply 5 blocks → tip=10
- **Bad signature**: any block in range has bad sig → whole response rejected, tip unchanged
- **Bad parent hash**: block 8 has wrong parent → blocks 6-7 kept, block 8+ rejected, re-request 8..10
- **Cap boundary**: request 6..200, responder caps at 100 → complete=false, follow-up 106..200
- **Dedup**: tip=5 receives future blocks 10 AND 11 → single range request, no overlap

## Scope (one session, 2-3h)

- ChainRangeRequest/Response types + codec
- Responder handler (cap, slice, respond)
- Requester handler (validate, apply in order, keep-on-failure)
- Gossip trigger (future block branch)
- Dedup (outstanding_chain_requests)
- commit_from_catchup() (skip broadcast)
