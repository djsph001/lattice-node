# Epistemic Integrity

**Principle:** The system must represent the epistemic status of
information exactly as it exists — no stronger, no weaker.

## What the system says

The system records:

- Who claimed what
- Who witnessed it
- What evidence exists
- How the evidence can be examined

The system does **not** say:

- This claim is true
- This witness is honest
- This evidence is sufficient
- This relationship is legitimate

## Why it matters

Truth is not produced by protocol. A distributed network cannot certify
that an observation happened — it can only record that a participant
said it happened, that other participants said they saw it happen, and
that certain artifacts exist. Any system that claims more is over-
selling its own epistemic reach.

## Enforcement in code

1. **Type names never overclaim.** `WitnessedClaim`, never `Proof` or
   `Certified`. The type system is the first line of defense against
   epistemic drift.

2. **The dashboard shows what was claimed, not what was verified.**
   `witness_count: 3` is displayed honestly; `verified: true` never
   appears. The UI is the second line of defense.

3. **No single number summarizes trust.** No "reputation score,"
   no "trust rank," no "credibility index." Thickness is a derived
   structural property of the claim graph, not an authoritative
   judgment about a peer's honesty.

## The seven invariants

1.  Presence ≠ Relationship
2.  Relationship ≠ Trust
3.  Observation ≠ Endorsement
4.  Witness ≠ Certification
5.  Claim ≠ Truth
6.  Evidence ≠ Interpretation
7.  Discovery ≠ Recognition

Each transition between these states requires explicit action or
additional evidence. No higher-order property is assumed from a
lower-order primitive.
