# Derivation Model Specification

Status: SPEC — not implemented. Answers "is a vouch a fraction or an amount?"

## The One Change

A vouch stores a FRACTION, not an amount.

Current: ThicknessEdge.amount = 819.0 (stored, mutated)
Derived: ThicknessEdge.stake_fraction = 0.90 (stored, immutable)
         derived amount = voucher's current total × stake_fraction

The amount is never stored for Vouch or Genesis edges. It is computed
at read time from the graph topology + a contribution count.

## What's Stored (the inputs)

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
    stake_fraction: f64,           // e.g. 0.90 — the fraction committed
    expiration_epoch: Option<u64>,
}

No `amount` field on Genesis or Vouch edges.
No `encumbered` HashMap.
No `amortize_over` counter that decrements.

## What's Computed (the derivation rules)

contribution_count = count of all VerifiedContribution edges in the graph.
This is a set cardinality — order-independent. Two nodes with the same
edge set agree on the count without sequencing.

derived_amount(Genesis { original_amount, amortize_over }) =
    match amortize_over {
        None => original_amount,
        Some(n) => original_amount * max(0.0, (n - contribution_count) as f64 / n as f64),
    }

derived_amount(VerifiedContribution { amount }) = amount

derived_amount(Vouch { voucher, stake_fraction, .. }) =
    total_thickness(voucher) * stake_fraction

total_thickness(peer) =
    sum of derived_amount(edge) for all incoming edges of peer

This is recursive. It terminates at Genesis and VerifiedContribution edges
(base cases — non-recursive). Vouch chains form a DAG; cycles are rejected
at vouch time.

usable_thickness(peer) =
    total_thickness(peer) * (1.0 - sum_of_active_stake_fractions(peer))

where sum_of_active_stake_fractions = sum of stake_fraction across all
active (non-expired) Vouch edges where this peer is the voucher.

## What Gets Deleted

- amortize_genesis()         — no mutation on contribution
- re_divide_vouchees()       — amounts are derived, not stored
- remove_vouchees_recursive() — zero propagates through arithmetic
- encumbered HashMap         — replaced by derived sum of fractions
- The equal-split per-vouchee logic — each vouchee gets their own fraction

## Semantic Change: Equal Split → Per-Vouchee Fractions

The current model re-divides all vouchees of the same voucher to equal
amounts. If Alice vouches 50% to Bob then 30% to Charlie, both get 40%.

The derivation model gives each vouchee their stated fraction. Bob gets
50%, Charlie gets 30%. Total encumbrance = 80%.

This is MORE correct — it matches what the voucher intended. The existing
re-division tests (vouch_re_divides_existing_vouchees, etc.) will need
updating because they test the equal-split behavior.

## Encumbrance Validation (replaces stake > usable check)

At vouch time, reject if:
    sum_of_active_stake_fractions(voucher) + new_stake_fraction > 1.0

This prevents over-extension. A voucher with 90% staked can only vouch
10% more. The check is against fractions, not absolute amounts.

## Vouch Cycle Rejection

A vouch creates a DAG edge: voucher → vouchee. The graph must remain
acyclic. Before accepting a vouch from A to B, check that B does not
have (transitively) a vouch edge pointing back to A. If it does, reject.

Without this, total_thickness enters infinite recursion.

At n=3 with simple chains, this never arises. Flagged for correctness.

## Step-by-Step Trace: Test 1

derivation_closes_laundering_through_honest_contribution

Setup: root genesis 1000, amortize_over=Some(10)
       root → B at 100% (fraction=1.0)
       B earns honest contribution (10)
       B → C at 90% (fraction=0.90)
       9 more contributions from peer
       Total contributions = 10

After add_genesis_thickness(root, 1000, Some(10)):
  contribution_count = 0
  root genesis derived = 1000 * (10-0)/10 = 1000.0
  root total = 1000.0
  B total = 0.0 (no edges yet)
  C total = 0.0

After stake_vouch(root → B, 1.0):
  B has Vouch edge: {voucher: root, fraction: 1.0}
  root active fractions = {1.0} → sum = 1.0
  B derived = total_thickness(root) * 1.0 = 1000.0 * 1.0 = 1000.0
  B total = 1000.0
  root usable = 1000 * (1.0 - 1.0) = 0.0

After add_verified_contribution(B, 10):
  contribution_count = 1
  root genesis derived = 1000 * (10-1)/10 = 900.0
  root total = 900.0
  B derived from root vouch = 900.0 * 1.0 = 900.0
  B honest edge = 10.0
  B total = 900.0 + 10.0 = 910.0

After stake_vouch(B → C, 0.90):
  B active fractions = {0.90} → sum = 0.90 ≤ 1.0 ✓
  C derived = total_thickness(B) * 0.90 = 910.0 * 0.90 = 819.0
  C total = 819.0
  B usable = 910.0 * (1.0 - 0.90) = 91.0

After 9 more contributions (contribution_count = 10):
  root genesis derived = 1000 * (10-10)/10 = 1000 * 0/10 = 0.0
  root total = 0.0
  B derived from root vouch = 0.0 * 1.0 = 0.0
  B honest edge = 10.0
  B total = 0.0 + 10.0 = 10.0
  C derived = 10.0 * 0.90 = 9.0
  C total = 9.0 ✓

PREDICTED TEST OUTPUT:
  root = 0.0 ✓
  B = 10.0 ✓
  C = 9.0 ✓ (was 819.0 under mutation model)

## Step-by-Step Trace: Test 2

derivation_closes_laundering_at_arbitrary_depth

Setup: root → B (100%), B honest (10), B → C (80%), C → D (50%)
       10 total contributions

After full liquidation (contribution_count = 10):
  root = 1000 * 0/10 = 0.0
  B = 0.0 * 1.0 + 10.0 = 10.0
  C = 10.0 * 0.80 = 8.0
  D = 8.0 * 0.50 = 4.0

PREDICTED TEST OUTPUT:
  root = 0.0 ✓
  B = 10.0 ✓
  C = 8.0 ✓ (was 728.0 under mutation model)
  D = 4.0 ✓

## Why Laundering Is Closed

The laundering vector exists because the mutation model stores C's amount
as a snapshot (819) taken when B's total was 910. When B's total drops to
10, the snapshot is stale.

In the derivation model, C's amount is NEVER stored. Every call to
total_thickness(C) recomputes: B's current total × 0.90. When B's
genesis-derived share drops to zero (because root liquidated), B's total
drops, and C's derived amount drops proportionally. Automatically. At
any depth. No traversal, no cascade, no mutation.

The zero propagates through multiplication:
  root = 0
  B from root = 0 * 1.0 = 0
  B total = 0 + 10 = 10
  C from B = 10 * 0.90 = 9
  (C's genesis-derived share = 0 * 0.90 = 0, gone)

## Why Ordering Requirement Is Reduced (Not Eliminated)

The contribution_count is a set cardinality — order-independent.
The genesis amount is a pure function of the count — order-independent.
The vouch amount is a pure function of the voucher's current total —
order-independent for DIFFERENT vouchers.

What remains order-dependent (per-voucher):
  - Two vouches from the same voucher: the validation check
    (sum of fractions ≤ 1.0) is state-dependent. If B vouches 60% to C
    then 60% to D, the second should be rejected (120% > 100%). But if
    they arrive concurrently, both check against pre-state (60% each ≤ 100%),
    both pass, B is over-extended.

  This is per-voucher serialization, which is tractable. It is the
  reduction the spec records: global → per-entity.

## What I Cannot Predict (honest gaps)

1. Computational cost at scale. total_thickness is now a recursive graph
   walk. At n=3, depth 2-3, trivial. At n=50 with multi-hop chains,
   potentially expensive. Memoization helps but cache invalidation
   reintroduces state. Not tested.

2. Cycle handling. Vouch cycles (A→B→A) cause infinite recursion.
   Rejection at vouch time is specified but not implemented. The
   transitive reachability check could be O(n) per vouch.

3. Expiration interaction. When a vouch expires, its stake_fraction is
   removed from the sum. The voucher's usable thickness increases. This
   is clean. But if expiration and a new vouch arrive concurrently on the
   same voucher, the validation check races. Same per-voucher issue.

4. The equal-split tests. Changing from equal-split to per-vouchee
   fractions changes the semantics of multi-vouchee scenarios. The
   existing tests (vouch_re_divides_existing_vouchees, multi_vouch_*,
   all_vouchees_expire_*) will FAIL under the derivation model and need
   rewriting. This is expected and correct — the equal-split was never
   the design intent.

5. Fraction precision. Floating-point multiplication through deep chains
   accumulates error. At depth 3 with fractions 0.9 * 0.8 * 0.5, the
   error is negligible. At depth 20, it may matter. Not tested.
