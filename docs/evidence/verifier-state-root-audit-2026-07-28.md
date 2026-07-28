# Verifier Audit: Era Two `state_root` Viability for Supply-Convergence Detection

**Date:** 2026-07-28
**Mission:** PHASE 2 VERIFIER — MISSION A (STATE_ROOT VIABILITY)
**Classification:** B — State roots cover economic state but aren't exposed/comparable

---

## 1. OBSERVED

### 1.1 What `state_root` Is

`state_root` is a deterministic Blake3 hash computed by `LedgerState::state_root()` in
`src/ledger/state.rs` lines 233-255. It hashes two data sets, each sorted by PeerId:

1. **Balances** — every `(PeerId, DigitalUtilityUnit)` entry, where `DigitalUtilityUnit`
   is a newtype over `u64` (`src/ledger/types.rs` line 10). Each entry contributes
   `peer_id_bytes || balance_u64.to_le_bytes()` to the hash.

2. **Nonces** — every `(PeerId, u64)` entry from the `seen_nonces` map. Each entry
   contributes `peer_id_bytes || nonce.to_le_bytes()` to the hash.

It does NOT include:
- Resource claims (`self.claims`)
- Thickness graph (has its own `thickness_root()`)
- Any aggregate/computed supply figure (no total sum)

The function comment states: "All honest nodes processing the same epoch arrive at the
same root" (line 235).

### 1.2 Where It's Used

`state_root` is embedded in the `RatificationBlock` struct (`src/message/types.rs`
lines 357-362):

```rust
pub struct RatificationBlock {
    pub epoch: u64,
    pub state_root: [u8; 32],
    pub thickness_root: [u8; 32],
    pub proposal_id: String,
}
```

It is CBOR-encoded with a `0x02` prefix byte and broadcast on `lattice/block/v1`
gossipsub topic.

### 1.3 When It's Computed (Epoch Cycle Ordering)

`state_root` is computed in `src/node.rs` `run_economic_epoch()` (line 1942). The
ordering within the function is:

1. **Line 1974:** `self.economic_engine.run_epoch(...)` — computes mint and
   redistribution transactions.
2. **Lines 1987-2018:** Apply mint transaction locally via `validate_and_apply()` —
   mutates `self.ledger.balances` and `self.seen_nonces`.
3. **Lines 2021-2059:** Apply redistribution transactions locally via
   `validate_and_apply()` — mutates `self.ledger.balances` and `self.seen_nonces`.
4. **Lines 2085-2121:** Credit claims to thickness graph, apply edge decay.
5. **Lines 2169-2212:** Compute `state_root` from `self.ledger.state_root(&self.seen_nonces)`
   and assemble `RatificationBlock`.

**Conclusion:** `state_root` is computed AFTER all epoch balance mutations are applied.
Two honest nodes at the same epoch with identical accepted transactions will produce
identical `state_root` values.

### 1.4 Exposure / Reachability

**API (Unix Domain Socket, `src/api.rs`):**
- There is NO API endpoint that returns `state_root`.
- `ApiRequest::GetEconomicState` (line 6431) returns `own_balance`, `own_nonce`, and
  per-peer `PeerBalance { peer_id, balance, nonce }` but does NOT include `state_root`.
- No other API response message carries `state_root`.

**Peer Protocol:**
- There is NO dedicated RPC or request-response message for exchanging `state_root`.
- `state_root` travels ONLY as a field inside `RatificationBlock` over
  `lattice/block/v1` gossipsub.

**Commit Chain:**
- `RatificationBlock` (including `state_root`) is committed to the hash chain at
  `src/node.rs` lines 1021-1044 and 3776-3787, making it available in catch-up
  responses but not independently queryable.

### 1.5 Fork Detection (Existing)

Two code paths detect `state_root` divergence:

**A. Live gossip reception** — `handle_ratification_block()` (lines 957-1055):
- Computes local roots (line 988-989).
- If `block.state_root != local_state` OR `block.thickness_root != local_thickness`:
  logs `"[block-recv] STATE FORK — RatificationBlock roots differ from local state.
  Dropping."` (line 1052).
- If roots match and advisory QC check passes: commits to chain.
- The block is DROPPED on mismatch — the fork is recorded in logs but no further
  reconciliation action is taken.

**B. Chain-sync catch-up** — (lines 3745-3772):
- On cold start (empty balances, empty nonces, zero thickness peers): waives root
  verification, accepts the block (line 3756-3762).
- On warm start (existing state): rejects with `"[chain-sync] RatificationBlock root
  mismatch after partial sync — rejecting fork"` (line 3768).

### 1.6 Determinism Test

`src/ledger/state.rs` lines 411-442 contains `test_state_root_deterministic` which
verifies: same balances and nonces inserted in different order produce the same root.

---

## 2. VERIFIED (claims supported by source evidence)

| # | Claim | Evidence |
|---|-------|----------|
| V1 | `state_root` covers balances (economic state) | `src/ledger/state.rs:238` — `self.balances.iter()` hashed per-peer |
| V2 | `state_root` covers nonces | `src/ledger/state.rs:242` — `nonces.iter()` hashed per-peer |
| V3 | Computed at epoch boundary AFTER all epoch transactions applied | `src/node.rs:1942-2212` — `run_economic_epoch()` ordering |
| V4 | Deterministic across nodes (same inputs → same root) | `src/ledger/state.rs:233-235` comment + `test_state_root_deterministic` |
| V5 | Embedded in RatificationBlock wire format | `src/message/types.rs:359` + `encode_wire()` at line 366 |
| V6 | Fork detection exists on receive | `src/node.rs:1050-1052` — logs "STATE FORK", drops block |
| V7 | No API endpoint exposes `state_root` | Searched all of `src/api.rs` + `handle_api_message` — not present |
| V8 | No dedicated RPC for exchanging `state_root` | Searched `src/` for `GetStateRoot`, `StateRootRequest` — zero matches |
| V9 | `DigitalUtilityUnit` is `u64` | `src/ledger/types.rs:10` — `pub struct DigitalUtilityUnit(pub u64)` |

---

## 3. CONTRADICTED (evidence that contradicts)

| # | Contradiction | Evidence |
|---|---------------|----------|
| C1 | Claim: "state_root only covers nonces/thickness/claims, not balances" | CONTRADICTED — `src/ledger/state.rs:238` explicitly iterates `self.balances` |

---

## 4. UNKNOWN (cannot establish from source alone)

| # | Unknown | Why unresolved |
|---|---------|----------------|
| U1 | Whether `seen_nonces` converges identically across honest nodes at epoch boundaries | Nonces accumulate from gossip transactions processed between epochs. If node A processes peer X's tx at nonce 7 but node B only sees up to nonce 5, their `state_root` values differ even with identical balances. The code comment assumes convergence ("All honest nodes processing the same epoch arrive at the same root") but actual convergence depends on gossip reliability. |
| U2 | Whether `thickness_root` divergence (separate field) could serve as a supplementary or alternative supply-convergence signal | `thickness_root` covers the provenance graph (Genesis, VerifiedContribution, Vouch edges), not DUU balances. Its relationship to economic supply divergence is untested. |
| U3 | Performance of `state_root()` at scale (hundreds/thousands of peers) | The function sorts all balances and nonces on every call — O(n log n) in peer count. Not measured. |
| U4 | Whether a recent RatificationBlock is always available in `cert_cache` or on-chain for arbitrary comparison | `cert_cache` is populated during `run_economic_epoch` but may be empty on catch-up or between epochs. Chain blocks are available but require decoding. |

---

## 5. EVIDENCE GAP

To resolve the UNKNOWNs above, the following would be needed:

1. **Multi-node convergence test** (resolves U1): Run 3+ nodes on a shared mesh through
   multiple epochs, snapshot `seen_nonces` and `state_root` at each epoch boundary, and
   verify all nodes produce identical values. Without this, `state_root` includes
   nonces that may diverge for reasons unrelated to economic state.

2. **Supply-specific extraction** (resolves U2): Confirm whether summing all balances
   from the `EconomicState` API response produces the same result across nodes. This
   would isolate supply divergence from nonce divergence.

3. **Benchmark** (resolves U3): Measure `state_root()` latency with 100, 1000, 10000
   balance entries.

4. **Protocol availability** (resolves U4): Determine whether `RatificationBlock`s are
   always committed to chain and always accessible via `GetBlock` API or chain-sync,
   or whether they can be transiently absent.

---

## 6. CLASSIFICATION: B

**B) State roots cover economic state but aren't exposed/comparable → needs
interface/protocol work.**

### Reasoning:

- `state_root` DOES cover economic state: it hashes all `(PeerId, DigitalUtilityUnit)`
  balances (`VERIFIED V1`). This is the core economic value — the supply per account.
- `state_root` is NOT exposed via any API endpoint (`VERIFIED V7`). The only way to
  obtain another node's `state_root` is to receive its `RatificationBlock` over gossip
  or chain-sync — both are passive, event-driven mechanisms, not queryable interfaces.
- Two nodes CANNOT directly compare their `state_root` values at will. Comparison
  happens only when a `RatificationBlock` is received, and the receiver checks it
  against its own local computation. There is no "give me your state_root" protocol
  message.
- The existing fork detection (STATE FORK log, line 1052) IS a comparison mechanism
  but it's one-directional (receiver vs. sender) and triggered by gossip, not by
  proactive query.
- `state_root` is a COMBINED hash of balances AND nonces. You cannot extract the
  supply component alone. Two nodes could have identical total supply but different
  `state_root` values if their `seen_nonces` differ (`UNKNOWN U1`).

### What would move this to A:

- An API endpoint (e.g., `ApiRequest::GetStateRoot`) that returns the node's current
  `state_root` on demand.
- OR a peer protocol message that lets one node request another's `state_root`.
- AND resolution of U1: evidence that `seen_nonces` converges identically across honest
  nodes at epoch boundaries, or a variant of `state_root` that omits nonces and hashes
  only balances.

### Why not C:

- `state_root` unequivocally covers economic state (balances). It doesn't not cover
  it. The question is reachability, not coverage.

---

## 7. SOURCE FILES EXAMINED

| File | Relevant Lines | What it shows |
|------|---------------|---------------|
| `src/ledger/state.rs` | 46-54, 233-255, 411-442 | LedgerState struct, state_root() implementation, determinism test |
| `src/ledger/types.rs` | 9-10 | DigitalUtilityUnit is u64 |
| `src/ledger/thickness.rs` | 582-612 | thickness_root is separate, covers edges not balances |
| `src/message/types.rs` | 353-380 | RatificationBlock struct with state_root field, wire encode/decode |
| `src/node.rs` | 249-254 | seen_nonces field definition |
| `src/node.rs` | 957-1055 | handle_ratification_block — fork detection on gossip receive |
| `src/node.rs` | 1883-1890 | epoch timer → run_economic_epoch |
| `src/node.rs` | 1942-2212 | run_economic_epoch — full ordering: mint, redistribute, then state_root |
| `src/node.rs` | 3745-3787 | chain-sync state_root verification |
| `src/node.rs` | 6190-6453 | handle_api_message — no state_root in any response |
| `src/api.rs` | 125-224 | ApiResponse enum — no state_root variant |
| `docs/VERIFIED-BEHAVIOR.md` | 229-231 | Confirms: "No reconciliation mechanism. The state_root and thickness_root work from Era Two exists but nothing currently uses it to detect or correct cross-node divergence." |

---

## 8. METHODOLOGY

- Searched entire `src/` directory for "state_root" and "StateRoot" (lowercase and
  PascalCase).
- Read full `state_root()` function, `LedgerState` struct, `RatificationBlock` struct.
- Traced the epoch cycle: `epoch_timer.tick()` → `run_economic_epoch()` → all balance
  mutations → `state_root()` call → `RatificationBlock` assembly.
- Checked all `ApiResponse` variants and `handle_api_message` match arms for state_root.
- Searched for dedicated RPC messages (`GetStateRoot`, `StateRootRequest`) — none
  found.
- Verified `thickness_root` is a separate hash of the provenance graph edges.
- Cross-referenced with `docs/VERIFIED-BEHAVIOR.md` which independently confirms the
  lack of reconciliation mechanism.
