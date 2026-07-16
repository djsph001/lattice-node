# Derivation Model Specification

**Status:** Three tiers. Each labelled. Read the labels before applying a claim.

---

## Tier 1: Built and Tested (27/27 tests pass)

### The One Change

A vouch stores INTEGER BASIS POINTS (u32), not an amount or a float.

Pre-mutation:
  ThicknessEdge.amount = 819.0 (stored, mutated)
Post-derivation (2026-07-14):
  ThicknessEdge.stake_fraction = 0.90 (stored, immutable)
Current (2026-07-17):
  ThicknessEdge.stake_bps = 9000 (stored, immutable)
  derived amount = voucher's current total × stake_bps / 10_000

The amount is never stored for Vouch or Genesis edges. It is computed
at read time from the graph topology + a contribution count.

### What's Stored (the inputs)

Three edge types, each stores only its inputs:

Genesis {
    original_amount: f64,          // e.g. 1000.0
    amortize_over: Option<u64>,    // e.g. Some(10)
}

VerifiedContribution {
    amount: f64,                   // e.g. 10.0 — fixed, never changes
}

Vouch {
    voucher: PeerId,
    stake_bps: u32,                // e.g. 9000 — integer basis points (0-10_000)
    expiration_epoch: Option<u64>,
}

No `amount` field on Genesis or Vouch edges.
No `encumbered` HashMap.
No `amortize_over` counter that decrements.
No `stake_fraction: f64` — f64 never touches the vouch path.

### What's Computed (the derivation rules)

contribution_count = count of all VerifiedContribution edges in the graph.
This is a set cardinality — order-independent. Two nodes with the same
edge set agree on the count without sequencing.

derived_amount(Genesis { original_amount, amortize_over }) =
    match amortize_over {
        None => original_amount,
        Some(n) => original_amount * max(0.0, (n - contribution_count) as f64 / n as f64),
    }

derived_amount(VerifiedContribution { amount }) = amount

derived_amount(Vouch { voucher, stake_bps, .. }) =
    total_thickness(voucher) * stake_bps as f64 / 10_000.0

total_thickness(peer) =
    sum of derived_amount(edge) for all incoming edges of peer

This is recursive. It terminates at Genesis and VerifiedContribution edges
(base cases — non-recursive). Vouch chains form a DAG; cycles are rejected
at vouch time.

usable_thickness(peer) =
    total_thickness(peer) * (10_000 - active_stake_bps(peer)) / 10_000.0

where active_stake_bps = sum of stake_bps across all active (non-expired)
Vouch edges where this peer is the voucher. Integer sum, exact.

### What Gets Deleted

- amortize_genesis()         — no mutation on contribution
- re_divide_vouchees()       — amounts are derived, not stored
- remove_vouchees_recursive() — zero propagates through arithmetic
- encumbered HashMap         — replaced by derived sum of bps
- The equal-split per-vouchee logic — each vouchee gets their own bps
- Float-to-float multiplication across the vouch path

### Integer bps (vouch path — EXACT)

The vouch path uses u32 basis points throughout:
- Wire format: Transaction::Vouch { stake_bps: u32 }
- Validation: active_stake_bps + new_stake_bps > 10_000 — integer, no epsilon
- Storage: ThicknessSource::Vouch { stake_bps: u32 }
- Read time: *stake_bps as f64 / 10_000.0 (deterministic, single conversion)

Two callers passing what they think is the same fraction (e.g.
0.1+0.1+0.1=0.30000000000000004 vs 0.3) land on DIFFERENT f64s. Under the
old model those could round to different bps at the conversion boundary.
Under this model, there is no conversion — the caller specifies bps
directly. The divergence vector is closed, not tolerated.

### Float aggregation (read path — DETERMINISTIC PER NODE)

`total_thickness()` returns f64. Its intermediate results are float arithmetic:
- `*stake_bps as f64 / 10_000.0` — one conversion, deterministic
- `.sum()` over an arbitrary number of peers — float addition
- Genesis formula — float multiplication and division

This is deterministic per node (same inputs = same outputs). The aggregation
path is not the divergence risk — two nodes with the same edge set compute
the same f64 through deterministic traversal. The risk is the contribution
count, which can differ during gossip (see Tier 2).

### Semantic Change: Equal Split → Per-Vouchee bps

The current model re-divides all vouchees of the same voucher to equal
amounts. If Alice vouches 50% to Bob then 30% to Charlie, both get 40%.

The derivation model gives each vouchee their stated bps. Bob gets
5000 bps, Charlie gets 3000 bps. Total encumbrance = 8000 bps (80%).

This is MORE correct — it matches what the voucher intended.

### Encumbrance Validation

At vouch time, reject if:
    current_bps + new_stake_bps > 10_000

This prevents over-extension. A voucher with 9000 bps staked can only
vouch 1000 more. The check is integer, exact, order-independent by
construction. No epsilon, no float comparison, no divergence.

### Vouch Cycle Rejection

A vouch creates a DAG edge: voucher → vouchee. The graph must remain
acyclic. Before accepting a vouch from A to B, check that B does not
have (transitively) a vouch edge pointing back to A. If it does, reject.

Without this, total_thickness enters infinite recursion.

**Not yet implemented** (2026-07-17). At n=3 with simple chains, never
arises. Flagged for correctness.

### Step-by-Step Trace: Test 1

derivation_closes_laundering_through_honest_contribution

Setup: root genesis 1000, amortize_over=Some(10)
       root → B at 10000 bps (100%)
       B earns honest contribution (10)
       B → C at 9000 bps (90%)
       9 more contributions from peer
       Total contributions = 10

After add_genesis_thickness(root, 1000, Some(10)):
  contribution_count = 0
  root genesis derived = 1000 * (10-0)/10 = 1000.0
  root total = 1000.0
  B total = 0.0 (no edges yet)
  C total = 0.0

After stake_vouch(root → B, 10000):
  B has Vouch edge: {voucher: root, bps: 10000}
  root active bps = 10000 → sum = 10000
  B derived = total_thickness(root) * 10000 / 10000 = 1000.0 * 1.0 = 1000.0
  B total = 1000.0
  root usable = 1000 * (10000 - 10000) / 10000 = 0.0

After add_verified_contribution(B, 10):
  contribution_count = 1
  root genesis derived = 1000 * (10-1)/10 = 900.0
  root total = 900.0
  B derived from root vouch = 900.0 * 10000/10000 = 900.0
  B honest edge = 10.0
  B total = 900.0 + 10.0 = 910.0

After stake_vouch(B → C, 9000):
  B active bps = 9000 → sum = 9000
  C derived = total_thickness(B) * 9000 / 10000 = 910.0 * 0.90 = 819.0
  C total = 819.0
  B usable = 910.0 * (10000 - 9000) / 10000 = 91.0

After 9 more contributions (contribution_count = 10):
  root genesis derived = 1000 * (10-10)/10 = 0.0
  root total = 0.0
  B derived from root vouch = 0.0 * 10000/10000 = 0.0
  B honest edge = 10.0
  B total = 0.0 + 10.0 = 10.0
  C derived = 10.0 * 9000/10000 = 9.0
  C total = 9.0 ✓

PREDICTED (and TESTED) OUTPUT:
  root = 0.0 ✓
  B = 10.0 ✓
  C = 9.0 ✓ (was 819.0 under mutation model)

### Step-by-Step Trace: Test 2

derivation_closes_laundering_at_arbitrary_depth

Setup: root → B (10000 bps), B honest (10), B → C (8000 bps), C → D (5000 bps)
       10 total contributions

After full liquidation (contribution_count = 10):
  root = 1000 * 0/10 = 0.0
  B = 0.0 * 10000/10000 + 10.0 = 10.0
  C = 10.0 * 8000/10000 = 8.0
  D = 8.0 * 5000/10000 = 4.0

PREDICTED (and TESTED) OUTPUT:
  root = 0.0 ✓
  B = 10.0 ✓
  C = 8.0 ✓ (was 728.0 under mutation model)
  D = 4.0 ✓

### Why Laundering Is Closed

The laundering vector exists because the mutation model stores C's amount
as a snapshot (819) taken when B's total was 910. When B's total drops to
10, the snapshot is stale.

In the derivation model, C's amount is NEVER stored. Every call to
total_thickness(C) recomputes: B's current total × stake_bps / 10_000.
When B's genesis-derived share drops to zero (because root liquidated),
B's total drops, and C's derived amount drops proportionally.
Automatically. At any depth. No traversal, no cascade, no mutation.

The zero propagates through multiplication:
  root = 0
  B from root = 0 * 10000/10000 = 0
  B total = 0 + 10 = 10
  C from B = 10 * 9000/10000 = 9
  (C's genesis-derived share = 0 * 9000/10000 = 0, gone)

---

## Tier 2: Traced Against Code

### Ordering: Global → Per-Entity

The contribution_count is a set cardinality — order-independent.
The genesis amount is a pure function of the count — order-independent.
The vouch amount is a pure function of the voucher's current total ×
stake_bps/10000 — order-independent for DIFFERENT vouchers.

What remains order-dependent (per-voucher):
  - Two vouches from the same voucher: the validation check
    (current_bps + new_bps > 10_000) is state-dependent. If B vouches
    6000 bps to C then 6000 bps to D, the second should be rejected
    (12000 > 10000). But if they arrive concurrently, both check against
    pre-state (6000 bps each ≤ 10000), both pass, B is over-extended.

  This is per-voucher serialization, which is tractable. It is the
  reduction: global → per-entity.

### The contribution_count Dependency

contribution_count reads ALL VerifiedContribution edges in the graph.
The count converges to the same final value regardless of arrival order.
It is NOT concurrency-independent — two nodes can see different k during
gossip propagation. This has been traced through the following paths:

1. **Genesis amounts differ:** k differs → derived genesis differs →
   total_thickness differs for the same peer on different nodes.

2. **The density guard reads total_thickness:** a node with lower k may
   refuse a panel that a node with higher k accepts. This is traced in
   `node.rs::run_witness_sortition` (line 2722) →
   `check_panel_access_density` (line 3158).

3. **Refusal is terminal for this node's signature** on that specific
   certificate. `run_witness_sortition()` runs once per cert on arrival
   (line 1713). No re-evaluation loop exists. A node that refuses at k=9
   will not re-evaluate when k converges to 10.

4. **Refusal is NOT terminal for the certificate.** The cert is cached
   unconditionally before sortition runs (line 1709-1710). Other nodes
   with higher k may sign it, reach 3-of-5 quorum, and the refusing node
   still commits it from cache when the quorum attestation arrives.
   No fork — no two nodes commit different things from the same cert.

### The Scheduled Threshold Crossing

The threshold crossing during genesis liquidation is NOT a rare edge case.
As k approaches N (amortize_over), genesis-derived thickness falls through
the whole range [original_amount, 0]. If honest_T crosses the density
threshold at some k between 9 and 10, then every cert arriving near that
k will straddle the boundary — nodes at k=9 refuse, nodes at k=10 accept.

This is the normal, expected trajectory of a mesh whose founder is
self-liquidating. A stall at that transition is arguably correct — the
mesh genuinely doesn't have density yet and should say so. The cert
recovers through re-proposal after convergence.

### Safety Property (traced, not tested under load)

Despite the convergence window: **no fork possible.** The density guard
is a local safety check, not a consensus rule. A refused cert sits in
cache and is committed when quorum attestations arrive from other nodes.
The worst case is a stalled cert needing re-proposal (latency), not
divergent state (fork).

This has been traced through the code but has NOT been tested under
network conditions with realistic gossip latency.

### Precision: Float-free ≠ Float-free-everywhere

Tier 1 documents that f64 never touches the **vouch path** (wire format,
validation, storage). This is true. But f64 is still alive in thickness
**aggregation**. `total_thickness()` sums derived amounts across peers
with `.sum()`, and float addition over N peers is order-dependent in
its lowest bits — the same `0.1+0.1+0.1 ≠ 0.3` problem, one layer out.

The density guard compares `honest_t >= threshold`, where both sides
are f64. Two nodes with identical edge sets compute identical totals
(deterministic traversal), so this is not a divergence vector. But a
reader should not walk away believing floats are extinct — they're
extinct on the vouch path specifically, and still present in the
aggregation that feeds the safety check.

---

## Tier 3: Claimed, Not Built

### Per-Entity Serialization for a No-Proposer Chain

The derivation model removed the only cross-entity dependency (the
contribution cascade). This is **necessary** for per-entity serialization.
It is **not yet sufficient** for a no-proposer chain.

The sufficiency argument goes: "per-entity causal chains, encoded as
block-level references, allowing concurrent blocks on different entities
without a global sequencer."

**The causal-reference mechanism does not exist.**

Three unbuilt things this claim depends on:

1. **Certificate dependency references.** The current
   `ImpactCertificate` protobuf schema carries no "this block depends on
   block X" field. A cert is a standalone payload. Until a cert can name
   the specific prior blocks whose state it read, "per-entity causal
   chain" is a description of the economic layer, not the block layer.

2. **Assembler behavior on missing dependencies.** When a node receives
   a cert that references a block it hasn't seen, what does it do? Stall?
   Request? Reject? The assembly logic is unspecified.

3. **Determinable dependency sets at assembly time.** Does the assembler
   know which blocks it depended on at the moment of assembly? Or does it
   discover them after the fact, making the dependency reference
   impossible to state in the cert itself?

**Until these are designed and built, the ordering resolution recorded in
this spec is: the economic layer has no global ordering requirement. The
block/DAG layer's ordering requirement is unknown because the mechanism
that would satisfy it doesn't exist.**

---

## Honest Gaps

1. **Computational cost at scale.** total_thickness is a recursive graph
   walk. At n=3, depth 2-3, trivial. At n=50 with multi-hop chains,
   potentially expensive. Memoization helps but cache invalidation
   reintroduces state. Not tested.

2. **Cycle handling.** Vouch cycles (A→B→A) cause infinite recursion.
   Rejection at vouch time is specified but not implemented. The
   transitive reachability check could be O(n) per vouch.

3. **Expiration interaction.** When a vouch expires, its stake_bps is
   removed from the sum. The voucher's usable thickness increases. This
   is clean. But if expiration and a new vouch arrive concurrently on the
   same voucher, the validation check races. Same per-voucher issue.

4. **Density guard at scale.** The no-fork analysis is traced against
   code but not tested under network conditions with realistic gossip
   latency.
