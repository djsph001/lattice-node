# Epistemic Invariants

Each invariant is a **philosophical constraint expressed as executable
code**. A violation means the system is claiming more than its evidence
supports.

## I1: Presence ≠ Relationship

A peer can be present in the peer table without having any cell
relationships. The peer table proves liveness; relationships require
a separate attestation.

**Test:** `add_peer` → `cell_relationships` is empty → no relationship
exists for that peer.

## I2: Relationship ≠ Trust

Two cells can have a declared relationship (Collaboration, WitnessPair)
without one trusting the other. Relationship is a claim about topology,
not about honesty.

**Test:** `CellRelationshipMsg::Accept` does not change any peer's
thickness, eviction status, or witness eligibility.

## I3: Observation ≠ Endorsement

A cell can observe and report an experiment's outcome without endorsing
its correctness. Observation is a claim about what was seen, not a
judgment about its validity.

**Test:** An `CellExperimentMsg` from an honest cell and a malicious cell
are structurally identical at the protocol level — both carry identical
evidence fields. The system never says one is "more true."

## I4: Witness ≠ Certification

A witness attests "I saw this happen," not "this is correct." Three
witnesses mean three attestations of observation, not a certified truth.

**Test:** Dashboard displays `witness_count: 3` — never `verified: true`
or `certified: true`.

## I5: Claim ≠ Truth

A submitted `WitnessedClaim` is a claim about work performed. It may
be honest, mistaken, or fraudulent. The protocol records it; it does
not verify it.

**Test:** A claim with zero witnesses is accepted by the system and
displayed as `witnesses: 0`. The system never rejects a claim for
insufficient truth.

## I6: Evidence ≠ Interpretation

Evidence (receipts, payload references) is recorded separately from
interpretation (reflections, insights). One cell's interpretation of
an experiment is not authoritative over another's.

**Test:** `CellReflectionMsg` and `CellExperimentMsg` are separate
types on separate gossipsub topics. The reflection topic carries
interpretation; the experiment topic carries raw evidence pointers.

## I7: Discovery ≠ Recognition

A cell discovering another cell via mesh protocols means only that it
received information about the cell's existence. It does not imply
recognition of legitimacy.

**Test:** After receiving a heartbeat with `cell_type: Some(...)`,
the receiving peer stores the cell type. It does NOT propose a
relationship, submit a claim, or modify thickness.

## Implementing the invariants

Each invariant maps to one or more unit tests in the `claims::invariants`
module (or a dedicated test file). The tests are part of the standard
`cargo test --lib` suite and are run on every build.

A test that fails is not a bug in the usual sense — it's an **epistemic
leak**, meaning the system is now claiming something its architecture
doesn't support. Fixing it may require reverting a feature or tightening
a type constraint, not adding more code.
