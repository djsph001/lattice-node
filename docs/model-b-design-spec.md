# Model B — Block Propagation Design Spec

Status: DESIGN — not built. Next session's primary work.

## Origin

Found 2026-07-17. `commit()` and `commit_root_block()` write to a local file.
Nothing gossips. The chain does not distribute. This is the single root cause
behind four symptoms chased separately across three sessions:

1. Nonce divergence between nodes
2. Ledger split-brain
3. Genesis isolation (Z4 has genesis, relay/Mac don't)
4. Density guard refusing on peers with empty thickness graphs

## The Build

**Model B: certificate broadcasts at quorum, every node commits on receipt.**

The commit path splits in two:

```
THEN (current, broken):
  handle_witness_attestation → accumulate → count >= 3 → commit() (local only)

NOW (proposed):
  handle_witness_attestation → accumulate → count >= 3
    → assemble canonical certificate → broadcast via gossipsub → return

  handle_certificate_received → validate quorum → verify all sigs
    → commit() (every node, including assembler, from received cert)
```

The assembler does not wait for its own broadcast to come back. It commits
from the same canonical bytes it serialized.

## Canonical Encoding

Same signatures arriving in different order must produce identical bytes.

- Signatures sorted by signer PeerId, lexicographic (BTreeMap or sorted Vec,
  NOT HashMap — Rust randomizes iteration)
- Fixed field order in serialization (event hash, height, signature set)
- Block hash computed over canonical bytes only, not transport metadata
- The serialized canonical form IS the block identity — no separate "logical
  block" vs "serialized block" (avoids Bitcoin-style malleability)

## Genesis as Config Prerequisite

"No amount of gossip resolves 'do I trust this root?'" — genesis is a trust
anchor, same as --genesis-root. Every node must have the genesis block before
it starts. It's a deployment prerequisite, not a gossip problem.

Nodes without genesis cannot participate in certification. This is correct
behavior, not a bug — the density guard refusing on nodes with empty thickness
graphs was doing its job.

## Fork Resolution: Conditional

**No reorg capability. No deterministic hash tiebreak.** Both are structurally
unnecessary IF panel enforcement is in place AND blocks are ordered.

This system CANNOT reorg. The thickness graph has lineage — decay follows it,
clawback traverses provenance, `remove_vouchees_recursive` cascades. Rolling
back a block means rolling back a graph mutation whose effects fanned out
transitively. No undo log exists. Designing for no-fork is the only option.

Panel enforcement eliminates fork-by-race (same panel, same signatures, same
canonical bytes, one block). But panel enforcement depends on the dependency
chain below — it can't be built until the chain distributes.

## The Dependency Graph (acyclic, one root)

```
chain distribution (Model B)
  → thickness convergence (every node replays same chain)
    → deterministic weighted sortition (weights converge)
      → panel enforcement (same panel on every node)
        → no-fork certificates (same panel, same sigs, same block)
```

### What the code does NOW (verified 2026-07-17)

- Attestation receipt: verifies signature, checks `count >= 3`, commits.
  NO panel membership check. Any valid signature from any node counts.
  (node.rs ~2890-2938)
- Sortition panel IS computed (select_weighted_witness_panel, node.rs:2759)
  but used ONLY to decide if local node should produce its own attestation.
  Never used to validate incoming attestations. Decorative.
- Self IS included in the pool (node.rs:2737-2743, explicit chain of local_peer_id)
- At pool.len() <= 5: sortition bypassed, returns all eligible peers (sortition.rs:106-111)
- At n>5: weighted Fisher-Yates runs, weights = thickness from chain replay
- Thickness diverges because chain doesn't distribute → weights diverge
  → panels diverge → panel enforcement impossible until chain distributes

### Why the tactical opening was rejected

At n<=5, "signer ∈ panel" reduces to "signer ∈ eligible pool" — a Sybil
membership check wearing panel vocabulary. Vacuous at current scale. Would
hide the visibly-open `count >= 3` hole behind a check that passes trivially.
At n=6 the weighted path engages, weights diverge, check rejects valid signers,
debugging leads three layers down to chain non-distribution.

Pattern to avoid: a check that's correct, tested, and enforcing a property
the code path doesn't have. Don't add a second decorative layer on a
decorative mechanism.

## OPEN QUESTIONS (must be resolved before or during build)

### 1. Block Ordering (CRITICAL — unresolved)

Model B distributes blocks. Gossipsub does not order them.

Two different events certified concurrently by different panels. Both valid.
Node A receives cert(E1) then cert(E2) → heights {H: E1, H+1: E2}.
Node B receives cert(E2) then cert(E1) → heights {H: E2, H+1: E1}.

Hash chain: block at H incorporates prev_hash, so the hash differs. Every
subsequent block diverges. Three chains with the same contents in different
orders. Zero equivocation. Zero Byzantine behavior. Just unordered transport.

**What determines a block's height?** This question is unanswered and is the
one that decides whether Model B produces a shared chain or N chains with the
same contents in different orders.

Possible approaches (NOT decided):
- Consensus round for ordering (leader-based, view-change)
- DAG structure where ordering derived from causal references (Hashgraph/Sui)
- Logical clocks (Lamport timestamps) embedded in certificates
- Epoch-based: blocks within an epoch unordered, epochs ordered
- CRDT-style deterministic merge

### 2. Emergence Hypothesis (to be tested, not assumed)

CLAIM (unproven): "Model B enables downstream convergence implicitly through
log-authoritative replay — same blocks in, same state out."

This is a PREDICTION about emergent behavior. It assumes:
- Replay is deterministic (requires block ordering convergence — see Q1)
- Nothing outside the log feeds state (UNVERIFIED — peer table is
  gossip-populated, not chain-anchored; may be other external inputs)
- Thickness graph converges (requires identical block sequence, which
  requires ordering solution)

Every layer in this project has had a hidden input nobody named. This claim
has the same shape. It is the hypothesis Model B is built to test, not a
solved consequence.

## Key Decisions (settled)

- Model B over Model A (commit-then-broadcast): the no-proposer design only
  works if signatures travel as a unit and every node commits from the same
  unit. Model A reintroduces a proposer through the back door.
- Genesis as config, not gossip: trust anchor, one thing not two.
- Thickness informs sortition (participation), never arbitration (fork choice).
  Using thickness for fork choice creates circular dependency: thickness derives
  from chain, chain selection needs thickness — bootstrap problem inside
  consensus rule.
- No reorg: structurally impossible given thickness graph lineage.
- No tactical opening: don't build panel enforcement at n=3 where it's vacuous.
- Canonical encoding: sorted signatures, fixed field order, hash over canonical bytes.

## State Mutation Classification (2026-07-17 code pass)

Every state mutation in the economic layer, classified by whether its effect
depends on prior state a concurrent block could modify:

1. MINT (balances.add)              → COMMUTATIVE
2. TRANSFER (debit + credit)        → CAUSAL (balance check on sender)
3. GENESIS (add_genesis_thickness)  → CAUSAL (one-shot, first-write-wins)
4. CONTRIBUTION (add_verified_contribution) → CAUSAL (triggers amortize_genesis)
5. VOUCH (stake_vouch)              → CAUSAL (read-modify-write on encumbrance + re-division)
6. EPOCH EXPIRATION (process_epoch) → CAUSAL (removal depends on prior insertion)
7. EVICTION (record_verification_failure) → CAUSAL (threshold + removal)

Score: 1 commutative, 6 causal. The economic layer is a provenance graph
with lineage, not an accumulator. DAG-without-total-order is dead at the
state layer.

Attempted recovery: per-entity serial order. Broken by finding #4 —
contribution triggers amortize_genesis, which cascades through re_divide_vouchees
and remove_vouchees_recursive. There is no entity partition. Every contribution
mutates the whole graph.

## CRITICAL WIRE: Genesis Amortization Unreachable from Production

`Transaction::Genesis` (types.rs:154-165) has NO `amortize_over` field.
The production genesis path (state.rs:155) calls:

    add_genesis_thickness(&root_peer, *thickness_grant, None)

Hardcoded `None`. Permanent genesis. The self-liquidating genesis — designed
deliberately, tested thoroughly, the contribution-denominated decay the sweep
proved was the meaningful axis — is unreachable from any transaction the
system can produce.

The `Some(N)` path exists only on the internal `ThicknessGraph::add_genesis_thickness`
method and is exercised only in tests (lines 977+, 1029+, 1058+). Four tests
pass. None of them run against anything the transaction layer can reach.

The code's own warning (thickness.rs:37-49) describes the exact configuration
the mesh runs in: "choosing None creates a permanent founder floor...
Prefer Some(N) for any mesh intended to decentralize over time."

This is the ninth instance of the name-asserts-property pattern.

## The Incompatibility: Self-Liquidation Requires Total Order

Self-liquidating genesis (Some(N)) is in genuine tension with no-proposer
ordering. Traced specifically:

After k contributions with amortize_over=N:
  genesis_amount = A * (N-k) / N

This part IS commutative — a pure function of the count, which converges
without ordering.

BUT the cascade is NOT a function of count alone. It depends on ORDER:

  amortize_over=3, root vouches for Lumen at 90%, someone vouches for Lumen.

  Order A: contribution×3 → cascade (root→0, Lumen→0, vouchees removed)
           → new vouch to Lumen
  Order B: new vouch to Lumen → contribution×3 → cascade (root→0, but
           Lumen has the new vouch as buffer, Lumen≠0, no recursive removal)

  Same count (3). Same vouch set. Different cascade outcome. Different graph.

The cascade is a function of order, not of state. Self-liquidating genesis
requires total order. The no-proposer design cannot provide total order.

Path A (keep None): buys tractable ordering at the cost of permanent founder
floor = autocracy-by-arithmetic. Not viable. The code's warning is our own
conclusion quoted back.

## Open Question 3: Derivation — STATUS UPDATE

### RESOLVED: Laundering closed

Derivation model implemented and tested. Both RED tests GREEN at predicted
values (9.0 and 8.0/4.0). Laundering vector closed at any depth through
multiplication. No traversal needed.

### OPEN: Dilution vs Ordering Tradeoff

The fraction model eliminated the per-voucher ordering requirement — the
last ordering dependency in the vouch path. But it also eliminated the
pooling/dilution invariant that was a deliberate Sybil defense.

Pooling model (deleted):
  - per_vouchee = encumbered/count
  - 100 sock puppets each get 1/100th of stake
  - Breadth visibly costs weight — endorsement is a finite pool
  - Requires re-division = read-modify-write = per-voucher ordering

Fraction model (built):
  - Each vouchee gets their stated fraction, no dilution
  - 100 vouchees at 1% each = full cap, all full-strength
  - Encumbrance cap bounds total but not per-leg value
  - No ordering dependency — fractions are immutable

CANNOT HAVE BOTH. This is a design fork:
  - Pooling = honest Sybil defense + ordering requirement
  - Fractions = order-free + weaker Sybil defense (cap-only)

The fraction model's order-freedom is its main argument, and it has
nothing to do with which economics is "more correct." It's about whether
the no-proposer architecture can accommodate the vouch path at all.

DECISION NEEDED: Dale + collaborators.

### OPEN: Float Precision at Breadth 3

0.10 + 0.10 + 0.10 = 0.30000000000000004 in IEEE 754. Not depth —
breadth three. The encumbrance check `sum + new ≤ 1.0` uses this
arithmetic. Two nodes summing the same fractions in different orders
get different last bits. A `>` comparison turns that into divergence.

This is the consistency problem arriving through arithmetic. Not a test-
tolerance issue.

Likely fix: store stake_fraction as integer basis points (u32, 0-10000).
0.10 → 1000. Sum is exact, order-independent. Convert to f64 only at
derivation time. But this is a design decision, not yet built.

## Build Scope (not a one-liner, not a raft import)

1. Resolve genesis self-liquidation formulation (derivation vs mutation — see Open Question 3)
   This determines whether the ordering problem is per-entity (tractable)
   or global (total order required, no-proposer in tension).
2. Restructure commit path: split accumulate→commit into accumulate→broadcast + receive→commit
3. Implement canonical certificate serialization (sorted sigs, fixed fields)
4. Add certificate-receipt handler (validate quorum, verify all sigs, commit)
5. Resolve block ordering question (see Open Question 1)
6. Panel membership validation — ONLY when sortition path actually engages (n>5)
