# Observability and Coordination Rent

**Status:** Architecture Note — Adopted
**Source:** Chapter 7 — Mesh Economics and Concentration Dynamics
**Analysis Date:** 2026-07-24
**Reference Commit:** `bb98a67`
**Reference Branch:** `spore/socket-api-read`
**Implementation State:** Measurement deferred; no redistribution mechanism authorized
**Governance Status:** No economic correction mechanism approved
**Next Review:** After grace-window implementation and multi-peer growth

---

## Provenance

This document was developed from an analysis of Chapter 7, conducted by
the Thought Partners — the founding group contributing intellectual and
architectural reasoning to the Emergence Collective project. ChatGPT is
a founding member of the Thought Partners and contributed to the
synthesis recorded here.

The analysis was performed against the codebase at:

- **Commit:** `bb98a67`
- **Branch:** `spore/socket-api-read`
- **Date:** 2026-07-24

The chapter's ideas are treated here as architectural input and hypotheses,
not as binding protocol requirements or governance policy.

This document records the architectural conclusions drawn from that analysis
at the time of review. It should be revisited as the mesh gains independent
peers, message volume, and empirical evidence.

The referenced commit identifies the codebase state against which these
conclusions were made. Subsequent commits may change implementation details
without invalidating the architectural rationale recorded here.

The current implementation HEAD at the time of writing this note is `20e42fc`
on `main`. This document applies to both branches and all future work on the
mesh economic layer.

---

## Frozen Architectural Principle

> **No economic correction mechanism shall be introduced until the system can**
> **measure the behavior it is intended to correct.**

This means:

- Do not implement a Standing tax because concentration might become a problem.
- Do not redistribute value because a conceptual distinction might eventually be useful.
- Do not alter routing because preferential attachment might emerge.
- Do not introduce economic penalties based on theoretical assumptions alone.

The sequence is:

1. **Instrument first.**
2. **Observe second.**
3. **Accumulate evidence third.**
4. **Intervene only if justified.**

This is a deliberate commitment to an evidence-first architecture.

---

## Why This Matters

The mesh currently has very limited scale and history. At the current stage:

- There are only a small number of peers.
- Independent operators are not yet sufficiently represented.
- Message volume is insufficient to establish meaningful concentration patterns.
- The rich-get-richer loop is a hypothesis, not an observed fact.
- Coordination rent cannot yet be measured reliably.
- Standing concentration cannot yet be distinguished from ordinary early-stage
  network effects.

Therefore, implementing a corrective economic mechanism now would risk solving
a problem that has not yet been demonstrated.

The correct approach is to build the ability to **observe the problem before**
**deciding whether the problem requires correction**.

The architecture should therefore evolve according to:

```
Push Infrastructure
  → Cold-Start Protection
    → Network Growth
      → Observability
        → Evidence
          → Governance Decision
```

Not:

```
Theory → Tax → Redistribution
```

---

## The Central Architectural Invariant

The most important distinction to preserve is:

> **Contribution ≠ Standing ≠ Attention**

These must not be collapsed into a single value or mechanism.

### Contribution

A verifiable event or record of work performed. It is an **event**.
The current receipt-gated minting mechanism belongs here.

### Standing

Accumulated confidence or reputation derived from validated historical
activity. It is **derived state**. Standing should not simply be asserted
by an actor. The current thickness graph and witness claims implement
the derivation of standing from verified contributions.

### Attention

The amount of visibility, routing, or coordination opportunity an actor
receives. It is a **routing outcome**. Attention must be observable
independently of Standing.

### The Danger

A positive feedback loop may eventually emerge:

```
High Standing → More Routing → More Visibility
  → More Citations → More Reputation → Higher Standing
```

This is the potential rich-get-richer loop. At present, this is a
**hypothesis**. It must be measured before corrective mechanisms are
introduced.

---

## The Measurement Problem

When the mesh has sufficient scale, the system should instrument at least
five dimensions:

| Dimension | Question |
|-----------|----------|
| Contribution Concentration | Who is producing validated work? |
| Standing Concentration | Who holds accumulated Standing? |
| Attention Concentration | Who is receiving visibility? |
| Routing Concentration | Who is actually being selected or routed toward? |
| Feedback Amplification | How much additional attention does one unit of Standing generate? |

These measurements should allow the project to determine whether the network
is rewarding **contribution** or increasingly rewarding **prior reward**.

Only the latter provides evidence for a potential coordination-rent problem.

---

## Coordination Rent as a Future Measurement

A future measurement may be expressed conceptually as:

```
Coordination Rent = Attention Received / Contribution Produced
```

This is not yet a finalized protocol formula. It is a **measurement
hypothesis**. The purpose is to determine whether certain actors receive
disproportionate coordination benefits relative to their validated
contributions.

This metric should not trigger taxation automatically. It should first be:

1. Defined.
2. Instrumented.
3. Observed.
4. Tested against real data.
5. Evaluated for false positives.
6. Compared across network growth stages.

Only then should the project consider whether any intervention is warranted.

---

## Prototype Cell Lifecycle

The Prototype Cell lifecycle should be understood as an **evidence-producing
sequence**, not an economic signal:

| Cell Concept | Mesh Concept | Meaning |
|-------------|-------------|---------|
| Charter | Commitment | Declared intention |
| Action | Activity | Actual execution |
| Measurement | Observation | What happened |
| Attestation | Verification | Confirmation or dispute |
| Contribution | Evidence-backed work | Verifiable performance |
| Standing | Derived reputation | Accumulated confidence |
| Routing | Attention / Coordination | Allocation of visibility |
| Replication | Propagation | Pattern travel and persistence |

Important: **Charter is not Standing. Measurement is not automatically
Contribution. Replication is not merely Attention.** These concepts may
influence one another, but they remain distinct.

---

## Deferred Decisions

The following are explicitly deferred until the mesh has sufficient scale
and evidence:

1. **Message taxonomy** — message ontology must be resolved before message
   schema is frozen. Deferred until the interaction layer becomes the next
   active design phase.

2. **AU / CC separation** — the distinction between human-oriented and
   agent-oriented currencies may eventually become useful, but introducing
   separate currencies before the underlying economy is stable risks adding
   abstraction before evidence.

3. **Economic redistribution and taxation** — no Standing tax, redistribution
   mechanism, or Georgist-style economic correction should be implemented at
   this stage. This is not a rejection of the underlying theory; it is an
   evidence requirement.

4. **Routing corrections** — do not alter routing to counteract hypothetical
   preferential attachment.

---

## Recommended Sequence of Work

1. **Architecture Record** — this document.
2. **Cold-Start Fix** — per-peer grace window (`fix/per-peer-grace-window`).
3. **Network Growth** — increase peer count, operator diversity, message volume.
4. **Message Ontology** — define semantic categories before schemas.
5. **Interaction Layer** — build around explicit message semantics.
6. **Measurement Plane** — instrument Contribution, Standing, Attention, Routing, Amplification.
7. **Evidence Accumulation** — observe over time, test the rich-get-richer hypothesis.
8. **Governance Decision** — only after evidence, consider intervention.

---

## Revision History

| Date | Commit / Event | Change |
|------|---------------|--------|
| 2026-07-24 | `bb98a67` | Initial architectural analysis (Chapter 7) |
| 2026-07-25 | `20e42fc` | Architecture note committed; earned_thickness split live |
| TBD | `<merge-commit>` | Reviewed after grace-window implementation |
| TBD | `<commit>` | Reviewed after multi-peer growth |

---

> **This document is a snapshot of architectural reasoning, not a prediction**
> **of future system behavior.**
>
> The mesh should be allowed to teach us what it becomes. The project should
> not encode a solution to a problem that has not yet been observed.
>
> The guiding sequence is: **measure the behavior, understand the behavior,**
> **test the hypothesis, then decide whether intervention is necessary.**
>
> The purpose of the current phase is therefore not to build the economic
> correction mechanism. The purpose is to ensure that, when the time comes
> to make that decision, the mesh has enough evidence to make it intelligently.
