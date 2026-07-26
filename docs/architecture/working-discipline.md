# The Lattice's Working Discipline

A governance document for how this project should be worked on.
Read this before you read the architecture diagrams.

---

## Root Principle

> **The system must never claim to know more than it has actually established.**

This applies to **outputs** — logs, metrics, endpoints, UI, publications. Internal
computation may speculate, explore, hold provisional state, generate competing
hypotheses. None of that may be presented as established knowledge until it has
survived the failure modes relevant to the claim.

---

## Seven Working Principles

### 1. Information is not evidence.

**Information** is data available to the system.
**Evidence** is information that has survived the failure modes relevant to the
claim being made. A fact is not established until it has been tested against the
conditions that would disprove it.

*How this showed up.* The oracle discriminator test. The system might observe a
build result — but that's information, not evidence. Evidence requires
independent verification: the discriminator that distinguishes a real verifier
from one that merely echoes. The test ensures that a *claimed* build result is
not treated as a *verified* one.

### 2. Transport is not verification. Verification is not economic consequence.

The pipeline has distinct stages: if the network carries the data, that's
transport. If the signature checks out, that's authentication. Neither is
evidence the claimed property holds. And even verified evidence does not
automatically earn economic standing — each step must be independently justified.

*How this showed up.* The witness pipeline. A claim arriving at a peer means
transport succeeded. A signed response means the peer acknowledged it. But
verification (did the witness actually observe what they signed?) is a separate
check against the witness's own heartbeat table. Economic consequence (does the
claim accrue thickness?) comes only after verification meets the threshold
conditions.

### 3. Measure before intervening.

If you don't have a baseline, you don't know whether your fix fixed anything.
Build the metric before the sweep. Build the sensor before the actuator.

*How this showed up.* The decay derivation decision. Before committing to
changing how thickness is computed, we had to establish what persisted values
currently represent — they're already decayed. Without that measurement,
"deriving decay from epoch" would silently change the economic meaning of every
existing edge while appearing to be a refactor.

### 4. Fix the invariant, not the symptom.

Symptoms are specific (a peer gets evicted, a witness count disappears).
Invariants are general (every new peer gets a minimum residency grace period,
evidence survives the epoch boundary). A symptom fix patches one path. An
invariant repair covers all paths.

*How this showed up.* The eviction gap. New peers kept getting evicted on a
tight timescale. Four times. Mint-seeding worked around it — once. The real fix
was making the grace window universal across all eviction paths, addressing the
root cause (Layer 2b bypassing the invariant) rather than papering over the
symptom with economic seeding.

### 5. Historical state is part of the protocol.

A value persisted to the snapshot is a commitment the mesh made to itself.
Changing what it means without a versioned migration is a protocol change, not
a refactor.

**Operational consequence.** Changes to the meaning of persisted values are
protocol migrations requiring coordinated mesh events, not local refactors.
A contributor who does not recognize this distinction will silently reinterpret
history.

*How this showed up.* The decay migration decision. Persisted thickness amounts
are already decayed — multiplying by a new factor would incorrectly double-decay
them. The current state represents real history, even if that history has
uncertainty baked in. The system must preserve "we no longer know exactly what
happened here" rather than filling the gap with a confident reconstruction.

### 6. Separate evidence from its consequences.

A decision and the evidence it was based on are different artifacts and must
survive independently. The evidence must remain available even if the decision
is later revised, else revision becomes a mechanism for destroying inconvenient
information.

*How this showed up.* The C1b persistence gap. Witness evidence was stored in
accepted_claims, which drained on snapshot. The decision (thickness crediting)
had already been applied, but the evidence supporting that decision disappeared
10 epochs later. Evidence must survive the decisions it once justified, or the
system cannot later re-examine those decisions.

### 7. Hide complexity, not evidence.

The user doesn't need to see the machinery. But the system must never present
uncertainty as certainty merely because the complexity has been hidden. The
interface can simplify the experience; it cannot fabricate assurance.

*How this showed up.* The site-copy thread. "Live infrastructure" appeared in a
page whose own header said data came from placeholder telemetry. The fix kept
the design language while grounding it in what the running system actually
produces. Same pattern: keep what's designed, name what's still designed.

---

## When Principles Conflict

The principles are not in fixed priority order. When they conflict — as
principle 5 (don't reinterpret history) conflicted with principle 1 (persist
causes, derive consequences) during the decay investigation — the resolution
returns to the root principle:

> **The system must not claim to know more than it has actually established.**

The project asks: what is actually established? What may the system legitimately
claim? Then decides. The decay resolution: the system did not have the
historical evidence to establish exact original thickness, so it accepted the
loss and documented it. That is not a dominance of one principle over another.
It is the root principle arbitrating.

---

## Status Vocabulary

The system distinguishes between levels of epistemic status. Contributors must
use these terms precisely.

| **Term**       | **Meaning**                                                                     |
|----------------|---------------------------------------------------------------------------------|
| Observed       | The system received the information.                                            |
| Persisted      | The information survived a durability boundary.                                 |
| Verified       | The required validation conditions were satisfied.                              |
| Eligible       | The verified artifact satisfies conditions for a consequence.                   |
| Applied        | The consequence was actually executed.                                          |
| Established    | The system has sufficient evidence to make the corresponding claim externally.  |

**The system must not skip levels.** A claim that has been observed is not yet
persisted. A claim that has been verified is not yet eligible. An eligible claim
has not yet had its consequence applied. Outputs must reflect the actual status,
not a more advanced one.

*How this showed up.* The persistence gate exposed the gap between *observed*
and *persisted*. The genesis gossip gap exposed the gap between *local*
establishment and *mesh-wide* establishment. The oracle discriminator test
ensured that *claimed* verification is not treated as *actual* verification.

---

## Collaboration Protocol

### The collaborator analyzes; the project decides.

The AI collaborator offers analysis — it surfaces alternatives, identifies
tradeoffs, applies adversarial pressure to human assumptions. The human decides
which analysis holds up against the evidence. The evidence arbitrates both.

Specific principles:

1. **Analysis offered, judgment retained.** The collaborator provides analysis;
   the project retains decision authority.

2. **The collaborator's confidence is not evidence.** A well-reasoned
   recommendation is input to the project's reasoning, not a substitute for it.

3. **The human's authority is not evidence.** "I designed it" is not a
   guarantee that it works. Only the running system's behavior is.

4. **The evidence arbitrates both.** When collaborator and human disagree,
   both turn to the same arbiter: what does the code actually do?

*How this showed up.* The wall-clock vs. epoch decision. The collaborator
recommended epoch-based; the human chose wall-clock based on a distinction
between operational time and economic time. Both analyses were offered; the
evidence and the project's reasoning determined the outcome.

---

## Operational Test

Does this matter, practically? Ask:

1. Do the logs distinguish between "observed" and "established"?
2. Do the metrics report what was measured, or what was inferred?
3. Do the API responses label the status of returned data?
4. Does the UI show uncertainty that the system actually has?
5. Do the publications present design as established fact?

If the answer to any question is "no," the root principle is being violated in
the system's outputs — where it matters most.

---

## Document Purpose

This is a governance document for the project, not a philosophy essay. It exists
to constrain future decisions, guide new contributors, and define what the
project is committed to before those constraints are tested by a specific
engineering choice.

**Read this before the architecture diagrams.** The architecture tells you how
the Lattice works. This document tells you what you are allowed to claim about
how it works.
