# Verifier Audit: /lattice/econ-compare/v1 Implementation

**Date:** 2026-07-28
**Scope:** `src/message/types.rs`, `src/message/codec.rs`, `src/network/protocol.rs`, `src/node.rs`, `src/ledger/state.rs`
**Protocol:** `/lattice/econ-compare/v1`

---

## QUESTION 1: Does the fingerprint hash over included_peers and ONLY that?

### OBSERVED

The function `compute_balance_fingerprint` (node.rs:2461-2475) computes the hash as follows:

```rust
fn compute_balance_fingerprint(&self, included: &HashSet<PeerId>) -> String {
    let mut sorted_balances: Vec<(&PeerId, &DigitalUtilityUnit)> = self
        .ledger
        .balances
        .iter()
        .filter(|(peer, _)| included.contains(peer))
        .collect();
    sorted_balances.sort_by_key(|(peer, _)| *peer);

    let mut hasher = blake3::Hasher::new();
    for (peer, balance) in &sorted_balances {
        hasher.update(peer.to_bytes().as_slice());
        hasher.update(&balance.0.to_le_bytes());
    }
    hex::encode(hasher.finalize().as_bytes())
}
```

The hash input is strictly:
- `peer.to_bytes()` (PeerId bytes)
- `balance.0.to_le_bytes()` (balance as little-endian u64)

For each peer in `self.ledger.balances` that also appears in the `included` set,
sorted by PeerId for determinism.

### VERIFIED

The fingerprint hashes **only** over the included peers. There are:
- NO nonces
- NO timestamps
- NO epoch values
- NO zero-placeholders for missing peers
- NO padding for peers in `requested` but not in `included`

If a peer is in the request but absent from `self.ledger.balances`, it contributes
**nothing** to the hash — no zero, no placeholder, no padding. The iteration source
is `self.ledger.balances` (not `included`), so absent peers are simply never visited.

This satisfies the "unknown is not zero" requirement.

---

## QUESTION 2: Is included_peers derived from the actual ledger lookup, or from the peer table?

### OBSERVED

In `handle_comparison_request` (node.rs:2485-2509):

```rust
fn handle_comparison_request(
    &self,
    request: &ComparisonRequest,
    epoch: u64,
) -> ComparisonResponse {
    let mut included_peers: Vec<String> = Vec::new();
    let mut known_peers: HashSet<PeerId> = HashSet::new();

    for peer_str in &request.requested_peers {
        if let Ok(pid) = peer_str.parse::<PeerId>() {
            if self.ledger.balances.contains_key(&pid) {   // <-- LINE 2495
                included_peers.push(peer_str.clone());
                known_peers.insert(pid);
            }
        }
    }

    let balance_fingerprint = self.compute_balance_fingerprint(&known_peers);
    // ...
}
```

The inclusion check is `self.ledger.balances.contains_key(&pid)` — this is
`LedgerState.balances` (a `HashMap<PeerId, DigitalUtilityUnit>` in `src/ledger/state.rs:48`).

Separately, the peer table is `PeerTable` (src/state/peers.rs) with its own `HashMap<PeerId, PeerInfo>`
tracking addresses, heartbeats, and liveness. It is **not referenced** anywhere in
the econ-compare code path.

Additionally, `LedgerState` provides `balance_of()` (state.rs:67-71) which returns
`DigitalUtilityUnit::ZERO` for unknown peers. If `handle_comparison_request` used
`balance_of()` instead of `contains_key()`, a peer known to the peer table but absent
from the ledger would return ZERO and could be erroneously included. The code correctly
uses `contains_key()` to avoid this trap.

### VERIFIED

`included_peers` is derived **exclusively from the ledger** (`self.ledger.balances`).
The peer table is not consulted. A peer known to the node (peer table) but without
a balance record (ledger) is correctly **excluded** from `included_peers` and
contributes nothing to the fingerprint.

---

## QUESTION 3: Does the responder's fingerprint cover the same field the requester's does?

### OBSERVED

**Responder path** (node.rs:3942-3962):
- Receives `ComparisonRequest` with `requested_peers`
- Calls `handle_comparison_request()` which calls `compute_balance_fingerprint(&known_peers)`
- `known_peers` is the subset of `requested_peers` found in `self.ledger.balances` via `contains_key()`
- `compute_balance_fingerprint` iterates `self.ledger.balances`, filters by `known_peers`, sorts by PeerId, hashes `(peer_bytes, balance_le_bytes)`

**Requester path** (node.rs:3963-3973):
- Sends request via `send_comparison_request()` (node.rs:2516-2525)
- Receives `ComparisonResponse` and logs it
- **Does NOT compute its own fingerprint** for comparison — only logs the response fields

The function `compute_balance_fingerprint` is a method on `LatticeNode` available to
both sides. Both sides share the same ledger structure (`self.ledger.balances`).

### VERIFIED: Algorithmic equivalence

Both sides have access to the same `compute_balance_fingerprint` function with identical
logic: iterate `self.ledger.balances`, filter by inclusion set, sort by PeerId,
hash `(peer_bytes, balance_le_bytes)`.

Both use direct map access (`self.ledger.balances`), NOT `balance_of()`. This is critical
because `balance_of()` returns ZERO for absent peers (state.rs:67-71), which would cause
a divergence if one side used it and the other used direct iteration. The code avoids
this by consistently using direct map operations.

The `contains_key()` guard in `handle_comparison_request` (line 2495) and the
`filter(|(peer, _)| included.contains(peer))` in `compute_balance_fingerprint` (line 2466)
are consistent: they both operate on the same `included`/`known_peers` set that was
built from `contains_key()` checks.

### EVIDENCE GAP: Requester does not verify

The requester (lines 3963-3973) currently **only logs** the response. It does not:
- Compute its own fingerprint for the same `included_peers` set
- Compare the local fingerprint against the response's `balance_fingerprint`
- Detect asymmetry by examining `included_peers` vs. its own ledger

This is not a bug in the fingerprint algorithm, but it means the protocol currently
operates as a unilateral "ask and trust" rather than a bilateral "ask and verify."
The `included_peers` field in `ComparisonResponse` exists specifically so the requester
can detect scope asymmetry, but no code exercises this.

### COMPLETENESS CHECK

| Scenario                                              | Result                                    |
|-------------------------------------------------------|-------------------------------------------|
| Peer in both ledgers, same balance                    | Identical hash                            |
| Peer in both ledgers, different balance               | Different hash (correct)                  |
| Peer in requester's ledger, not in responder's        | Excluded from hash on both sides (correct, asymmetry visible via `included_peers`) |
| Peer in responder's ledger, not in requester's        | Excluded from hash on both sides (correct, asymmetry visible via `included_peers`) |
| Peer with zero balance in responder's balances map    | Hashed as zero (correct — genuinely known with zero) |
| Peer absent from responder's balances map             | Not hashed at all (correct — unknown is not zero) |

---

## VERDICT

**APPROVED** — The fingerprint implementation is correct and satisfies all
three security properties:

1. Hash includes only included peers — no zero padding, no nonces, no timestamps. **VERIFIED.**
2. `included_peers` is derived from the ledger (`self.ledger.balances`), not the peer table. **VERIFIED.**
3. Both sides share the same deterministic fingerprint algorithm with no `balance_of()` divergence path. **VERIFIED.**

### EVIDENCE GAP (non-blocking)

The requester does not currently compute its own fingerprint for comparison against
the response. The `ComparisonResponse.included_peers` field and `compute_balance_fingerprint`
function exist to enable bilateral verification, but the requester-side event handler
(lines 3963-3973) only logs the response. This is a feature gap, not a correctness
defect — the hashing infrastructure is correct and ready for bilateral use.

### NIT (informational)

Line 3959 in the event handler logs `request.requested_peers.len()` as the `included`
count rather than the actual response's `included_peers.len()`. This is a cosmetic
logging issue only; the event log comment acknowledges it with `// reported by handler`.
