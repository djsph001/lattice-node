# Spec: Mycelial Thickness + Vouch-Based Sybil Resistance

**Status:** Implementable spec. Layer 1 built, measured, and wired. Layer 2 designed, trigger gated.
**Amended:** 2026-07-14 — weighted voting power, panel-access invariant, FLOOR_WEIGHT as security parameter.
**Amended:** 2026-07-14 (session 2) — Layer 2: chained clawback replaced with bounded per-vouch clawback.
  Chained clawback was correct for the abandoned collusion-detection trigger; it becomes a griefing
  vector under the provable-misbehavior trigger. See §4 INVERSION NOTE for the full reasoning.
**Depends on:** existing CBOR `Transaction` pipeline, `LedgerState`, Fisher-Yates sortition filter.
**Core principle:** Sybil resistance and routing preference are the *same* mechanism — thickness.
The defense is not "make Sybils impossible" but "make Sybil influence never exceed the attacker's
real contribution," so the attack is never *profitable*.

---

## 0. The security levels this delivers (read this first — it's the honest boundary)

- **Layer 1 alone** delivers: *Cheap (floor-weight) Sybil influence is bounded by the attacker's
  real contribution.* Naive multiplicity attacks (many cheap keys) are defeated outright —
  multiplicity buys thinness, not influence. A patient attacker who becomes established
  and vouches their own swarm is **capped** (swarm influence ≤ voucher's own thickness) but
  **not slashed**. Layer 1's measured security boundary: honest_T > N_sybil × FLOOR_WEIGHT
  for panel access; once on panel, weighted voting gives honest nodes dominance proportional to T.
- **Layer 2 (gated)** adds: *bounded per-vouch clawback* on individually provable protocol
  violations (double-checkpoint, conflicting attestations, deadline abuse). A voucher loses
  exactly their chosen `stake_committed` on the cheater — no more, no chain propagation.
  This enforces the risk-is-chosen principle: you stake your judgment; bad judgment costs
  you exactly your stake, never someone else's.
- **What neither layer claims:** defense against an attacker willing to genuinely contribute
  at scale indefinitely. That attacker converges to "a real participant who happens to be
  adversarial," and the accepted equilibrium is that they have funded the mesh to attack it.
  This is a *decision*, not a gap — see §7.
- **SCOPE NOTE:** Every Layer 1 security number in this spec models Sybils at floor weight
  (zero contribution, no vouches, competing at FLOOR_WEIGHT = 0.01 each). The patient attacker
  who earns thickness beyond floor weight is **not addressed by Layer 1** — that attacker is
  the domain of Layer 2's clawback + detection. Do not read "97.1% honest dominance" as
  "governance is solved." It means "governance is solved against floor-weight Sybils,
  assuming Layer 2 handles patient ones."

Do not let Layer 1 be mistaken for the full defense. It is the 80% that stands alone,
measured and confirmed. The patient-attacker slash requires Layer 2, which requires detection,
which is not started.

---

## LAYER 1 — Thickness graph + vouch mechanics + weighted voting (BUILT, MEASURED)

### 1. The ThicknessGraph data structure

Thickness is **not a scalar**. It is a directed provenance graph, first-class from line one,
because chained clawback (Layer 2) requires derivation lineage that cannot be retrofitted.

```rust
struct ThicknessGraph {
    edges: HashMap<PeerId, Vec<ThicknessEdge>>,
    encumbered: HashMap<PeerId, f64>,
}

struct ThicknessEdge {
    source: ThicknessSource,
    amount: f64,
    created: Timestamp,
}

enum ThicknessSource {
    VerifiedContribution { receipt_id: ReceiptId },
    Vouch { voucher: PeerId, vouch_nonce: u64 },
}
```

**Key invariants of the graph itself:**

- **Only `VerifiedContribution` mints new thickness.** Vouches *move* thickness, they don't
  create it. This is what makes a self-talking Sybil farm build nothing: with no verified
  contribution, there is no thickness in the cluster to vouch *with*.
- **A node's usable thickness = Σ(incoming edges) − Σ(thickness currently encumbered on
  outgoing vouches).** You cannot vouch with thickness you've already staked elsewhere.
- **Provenance is never collapsed.** Even when thickness is summed for a sortition weight, the
  edges are retained, because Layer 2 traverses them. Never reduce the graph to a scalar at rest.

### 2. The Vouch transaction (new variant, existing pipeline)

```rust
Transaction::Vouch {
    voucher: String,      // PeerId string form
    vouchee: String,      // PeerId string form
    staked_fraction: f64, // fraction of voucher's own unencumbered thickness
    nonce: u64,
    timestamp: Timestamp,
}
```

**Validation:** same pipeline as Transfer/Mint — signature, timestamp freshness, nonce replay.
One new check, same shape as the Transfer balance check: the voucher must hold **sufficient
unencumbered thickness** (voucher_total × staked_fraction ≤ usable_thickness).

**apply_transaction — the `Vouch` arm (invariants #1 and #2 live here):**

1. Compute `stake = staked_fraction × voucher_total_thickness`. (Invariant #1: stake is a
   fraction of the voucher's OWN thickness, so stake scales with the voucher's capacity.)
2. Count existing vouchees `k`. Per-vouchee derived thickness = `stake / (k + 1)`, and
   **existing vouchees are re-divided down** to `stake / (k+1)` each. (Invariant #2: the
   staked pool is split across ALL vouchees. You can have many, or you can have
   influential — not both, from a fixed stake pool.)
3. Write a `ThicknessSource::Vouch` edge into the vouchee's entry, and mark `stake` of the
   voucher's thickness as encumbered.

### 3. Sortition + weighted voting (AMENDMENT #1 — thickness-weighted voting power)

**Panel selection** uses the existing Fisher-Yates sortition filter, weighted by thickness.
Thin / new / unvouched nodes (floor-clamped, see §3a) **can** join, be reached, relay, and
contribute — openness preserved. But their **selection probability** is proportional to
thickness. Near-zero thickness → near-zero selection weight.

**Panel voting is thickness-proportional, NOT one-vote-per-seat.** This is the mechanism
that dissolves the seat-ceiling identified by measurement: weighted selection without
replacement limits any single honest node to at most one panel seat, but thickness-weighted
voting power decouples influence from seat count. A single honest node with T=50 holding
1 of 5 seats against 10,000 floor-weight Sybils commands 86.8% of the panel's aggregate
voting power. With T=100: 97.1%. With T≥500: >99.9%.

Concretely: after the panel is selected, each member's vote weight = their thickness /
total panel thickness. The panel's aggregate decision is the thickness-weighted sum of
member votes. This means 4 floor-weight Sybil seats (4 × 0.01 = 0.04 total weight) are
overwhelmed by 1 honest seat carrying T=50 (vote share = 50/50.04 ≈ 99.9%).

**Implementation:** the `select_weighted_witness_panel` function produces the panel PeerIds.
The caller (witness protocol handler) looks up each panel member's thickness from the
`ThicknessGraph`, computes vote weights, and uses them for attestation quorum and governance
decisions. This is a protocol-layer computation, not a sortition-layer one — the sortition
only controls who gets a seat; voting power is computed post-selection.

### 3a. FLOOR_WEIGHT as a security parameter (AMENDMENT #3)

```rust
pub const FLOOR_WEIGHT: f64 = 0.01;
```

This is **not** an aesthetic choice. It is a security parameter controlling the linear
threshold at which combined Sybil floor weight can challenge honest thickness.

**Measured relationship:** combined Sybil weight = N × FLOOR_WEIGHT. The boundary at
which Sybils begin to compete for panel access is N ≈ honest_T / FLOOR_WEIGHT.

| Honest T | Break-even N (Sybils needed to match T) |
|----------|--------------------------------------|
| 10       | 1,000                                 |
| 50       | 5,000                                 |
| 100      | 10,000                                |
| 500      | 50,000                                |
| 1,000    | 100,000                               |

**Recommended pinning:** set FLOOR_WEIGHT = 1 / T_min where T_min is the expected minimum
thickness of an established honest node in the mesh. This ties the floor to actual mesh
economics rather than an arbitrary constant. If the smallest honest contributor is expected
to have T=100 after a week of relay+storage, FLOOR_WEIGHT=0.01 gives 100× headroom before
floor accumulation threatens panel access.

**Tuning policy:** FLOOR_WEIGHT is a CLI parameter (`--floor-weight`), not hardcoded.
The default is 0.01. Meshes with fewer expected honest nodes or higher security requirements
should lower it. The measurement harness in `src/swarm_measure.rs` can be re-run with any
value to verify the linear boundary.

### 3b. Panel-access invariant (AMENDMENT #2 — continuous runtime property)

**Invariant:** a witness panel SHALL NOT be convened unless the total honest thickness
in the eligible pool exceeds N_sybil × FLOOR_WEIGHT × MARGIN, where:
- N_sybil is the estimated worst-case Sybil count (configurable, default: number of
  floor-weight peers in the peer table)
- MARGIN is a safety factor ≥ 2.0 (recommended: 2.0 for early mesh, 1.5 for mature mesh)

**This is NOT a one-time startup check.** It is a **continuous runtime property**
evaluated at every panel formation, because:
- The mesh grows and shrinks
- New floor-weight peers join (honest newcomers AND Sybils)
- Honest thickness accumulates but so can Sybil count

If the invariant fails: the mesh remains in participation-only mode (relay, storage,
routing — no governance/witness panels). This is the "not enough trust density to
govern safely" state, and it's the correct posture for a sparse young mesh.

**Implementation:** before calling `select_weighted_witness_panel`, compute:
```rust
let total_honest_t: f64 = peer_pool.iter()
    .map(|(_, w)| w)
    .filter(|w| *w > FLOOR_WEIGHT)  // count only nodes above floor
    .sum();
let floor_count = peer_pool.iter()
    .filter(|(_, w)| *w <= FLOOR_WEIGHT || *w == FLOOR_WEIGHT)
    .count();
let sybil_floor_total = floor_count as f64 * FLOOR_WEIGHT * estimated_sybil_fraction;
if total_honest_t < sybil_floor_total * MARGIN {
    // Abort panel formation — mesh too sparse to govern safely
    return;
}
```

The `estimated_sybil_fraction` acknowledges that not every floor-weight peer is a Sybil —
honest newcomers also sit at floor weight. Default: 1.0 (conservative — assume all floor
peers could be Sybils). Tunable down as the mesh matures and newcomer honesty is established.

---

## LAYER 2 — Bounded per-vouch clawback (DESIGNED, GATED on provable violations)

**INVERSION NOTE (2026-07-14):** The original spec (§4, drafted in a prior session) specified
*chained clawback* — burning derived thickness upward through the entire vouch chain on
detected collusion. That mechanism was correct for a *cluster-level collusion-detection*
trigger. The trigger was subsequently abandoned because cluster-level collusion is
undecidable: honest-tight groups and Sybil farms are structurally identical, and no
graph-topology or behavioral signal can separate them without false-positiving honest
clusters. Chained clawback under a *per-node provable-misbehavior* trigger becomes a
**griefing vector** — an attacker can get vouched deep in a chain, self-cheat, and burn
thickness belonging to people who never chose to stake on them. The traversal was
therefore removed *deliberately*. This section replaces it with bounded per-vouch clawback
using the same `stake_committed` field the provenance graph already carries.

### 4. The clawback mechanic (AMENDED — bounded, no chain traversal)

**Trigger:** individually provable protocol violations — verifiable from single-node evidence,
requiring no intent-detection, no cluster heuristics, no topology analysis:
- Double-checkpoint: a node submits two conflicting checkpoints for the same task step
- Deadline-migration abuse: a node repeatedly forces task migration by missing deadlines
- Conflicting attestations: a node signs contradictory witness statements
- (The set of provable violations grows as the protocol hardens; this is bounded and enumerable)

**On confirmed violation by `cheater`:**

1. Find all `ThicknessSource::Vouch` edges where `vouchee == cheater`.
2. For each such edge: release its `stake_committed` from the voucher's encumbrance.
3. Remove the cheater's derived thickness edges from the graph.
4. For each affected voucher: re-divide their remaining active vouchees upward
   (total encumbered / remaining count — same reverse-division as expiration).

**CRITICAL: No chain traversal.** The clawback stops at the direct voucher. The voucher
loses exactly `stake_committed` — the amount they *chose* to stake on the cheater.
It does NOT propagate upward to the voucher's own vouchers. The provenance graph exists
for *traceability* (inspect who-vouched-whom for audit), not for *propagation* (burning
upward through the chain).

**Bound:** a voucher's maximum loss from any cheater = exactly their chosen `stake_committed`
on that cheater. Griefing surface = zero beyond the voucher's own risk decision.
An attacker who gets vouched with X can destroy at most X of the voucher's thickness —
and X was the voucher's judgment call, priced by the vouch-stake mechanism.

**Economic logic:** a voucher who stakes on a cheater loses their stake. A voucher who
stakes on an honest node keeps it. This is the vouch-stake working as designed: you stake
your judgment; bad judgment costs you exactly your stake, no more, no less, and never
someone else's stake.

### 5. The blocking dependency: provable-violation detection (NOT STARTED)

Clawback is inert without protocol-level detection of individually provable violations.
This is a protocol-completeness task (enumerate the violations the protocol can
cryptographically prove from single-node evidence), not an open-ended intent-detection
problem. It is bounded, enumerable, and incremental — the set grows as the protocol hardens.

**Dissolved, not solved:** the honest-tight-vs-collusive distinction that opened the
detection design session was *never addressed* — because clawback fires on provable
individual *acts*, not cluster *shapes*. The mechanism never has to tell them apart.
The undecidable question was simply routed around.

**This is a named prerequisite, not yet built.** Layer 2 ships dark (code present,
trigger unwired) until per-violation detection exists and is verified feeding §4.

---

## 6. Build order

1. `ThicknessGraph` + `ThicknessEdge` + `ThicknessSource` in `LedgerState`. Provenance from line one.
2. `VerifiedContribution` thickness minting: wire Phase 6 receipts → new thickness edges.
3. `Transaction::Vouch` variant + validation check + `apply_transaction` arm (§2).
4. Sortition weighting by thickness (§3), floor-clamped (§3a).
5. **STOP. Layer 1 is a complete, honest, shippable defense here.** Verify it:
   many cheap keys get negligible combined weight; vouched swarms are capped by voucher
   thickness; weighted voting gives honest nodes dominance at T > N×FLOOR_WEIGHT.
   Run `cargo test --lib swarm_distribution -- --nocapture` to measure the actual
   security boundary against current parameters.
6. **Wire node.rs with the panel-access invariant (§3b).** Do NOT wire weighted sortition
   into the witness panel loop without the access guard — a mesh that convenes panels
   before satisfying honest_T > N×FLOOR×MARGIN is convening capturable panels.
7. (Gated) Bounded per-vouch clawback (§4) — build the mechanic, leave the trigger unwired.
   Uses existing `stake_committed` field. No chain traversal. No new graph machinery needed.
8. (Blocked) Per-violation detection — enumerate and detect individually provable protocol
   violations. Bounded, enumerable design task; grows as protocol hardens.

## 7. The accepted boundary (decision, not gap)

An attacker who contributes genuine verified work at scale, indefinitely, across many keys,
can accumulate real thickness and real influence. Neither layer stops this. The equilibrium:
such an attacker has paid the full cost of honest participation — funded relay, storage, and
routing the mesh actually used — to earn that influence. The cost of attack has converged with
the cost of contribution. We accept this equilibrium deliberately: a system that forces
adversaries to fund the infrastructure they would attack has priced the attack correctly.
This is stated so no future reader mistakes it for an oversight.

---

## Verification checklist (read-real-state discipline)

- [x] After Layer 1: spin up N cheap keys, confirm their *combined* sortition weight is
      negligible — measured via `swarm_distribution_measurement`. Sybil seat share = 0.80
      (seat-ceiling property, not a defense failure). Honest vote share with weighted voting =
      97.1% at T=100 vs N=10,000. ✓ MEASURED
- [x] Confirm a vouch encumbers the voucher's thickness (unencumbered drops by `stake`) and
      that a second vouch re-divides existing vouchees down — tested in `thickness.rs`. ✓
- [x] Confirm floor-clamp: a brand-new honest node retains nonzero selection probability —
      tested in `test_weighted_floor_clamp_gives_new_nodes_a_chance`. ✓
- [x] Confirm weighted voting dissolves the seat-ceiling: 1 honest heavy node outvotes
      4 floor-weight Sybil seats despite holding 1 seat — measured at T=50 (86.8%), T=100
      (97.1%), T=500+ (>99.9%). ✓ MEASURED
- [x] Document the linear safety boundary: honest_T > N_sybil × FLOOR_WEIGHT × MARGIN
      for panel access. ✓ ENCODED in §3b
- [ ] Wire node.rs with the panel-access invariant guard. Panel formation aborts if the
      mesh is too sparse to govern safely.
- [ ] Layer 2 remains OFF (trigger unwired) and is documented as such until §5 exists.
- [ ] Do not claim patient-attacker slashing works until detection (§5) is built and verified
      feeding clawback (§4).
