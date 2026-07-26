# Engineering Discipline

Rules that emerged from the Pass 1 audit and earned their place by
preventing real bugs. None was written before the work — each was
discovered in the specific moment it became load-bearing.

---

## 1. Don't confuse historical state with current state.

A value persisted to the snapshot is a commitment the mesh made to itself.
Changing what it means without a versioned migration is a protocol change,
not a refactor. This applies to thickness amounts, decay factors, claim
status, and any field with economic meaning.

The acid test: if a future reader who only has the snapshot file needs to
interpret a value, and your change would make them interpret it differently,
that's a protocol migration. Design the migration, name the boundary,
and don't paper over it.

## 2. Don't call a mechanism durable until crash recovery proves it.

Short form: compiled ≠ deployed, deployed ≠ verified, verified ≠ durable.
A claim exists in memory (Layer 1). A claim survives a restart (Layer 2).
A claim survives a crash between acceptance and snapshot (Layer 3). Each
is a different guarantee and needs a different test. Treat "it compiles"
and "the test passes in memory" as building blocks, not evidence of
durability.

## 3. Don't infer protocol failure from configuration failure.

When an observed failure has multiple possible causes, check the
configuration first. The mesh reports configuration silently (e.g.
`--no-mdns` breaking without warning). A protocol that works as
specified under known config is not the same as a protocol that doesn't
work. Blaming the protocol for a config issue wastes an investigation
cycle and conceals the real fix.

## 4. Don't fix symptoms when the invariant can be repaired.

Symptoms are specific (a peer gets evicted, a witness count disappears).
Invariants are general (every new peer gets a minimum residency grace
period, evidence survives the epoch boundary). A symptom fix patches
one path. An invariant repair covers all paths. If you're about to add
a per-condition guard, ask whether the invariant itself should be
universal instead.

## 5. Don't economicize before you observe.

Measurement before intervention. A metric before a sweep. An observation
window before a distribution rule. The mesh doesn't have enough
independent operators yet to validate any behavioral-economic hypothesis.
Build the sensor before the actuator. This has a specific operational
form for this codebase: Pass 1 (arithmetic coherence) before Pass 2
(behavioral desirability), and Pass 2 gates on sufficient network growth.

## 6. Don't hide uncertainty behind polished interfaces.

The widget shows "0.001 earned · 1 witness" with the earned explicitly
labeled as "self-attested pending independent operators." The methodology
page states which metrics are running and which are designed. The
honesty rule: if the system can't prove it, the interface doesn't claim
it. A polished UI that omits uncertainty is a more effective lie than a
confusing one — harder to spot, easier to trust.

## 7. Don't let external authority substitute for evidence.

A collaborator's confidence, a literature citation, a precedent from
another system — all of these can inform design. None constitutes
evidence that the thing works here, in this codebase, under these
conditions. The running system is the authority. Every rule above is
a specific form of this one.
