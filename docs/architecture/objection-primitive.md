# Objection Primitive — Design Note

**Status:** Draft for experiment design
**Date:** 2026-07-25
**Scope:** Minimum viable mechanism — no economic layer

---

## 1. What is an objection?

An objection is a `WitnessedClaim` with `claim_type = ClaimType::Objection = 1`,
targeting a specific existing claim by ID. It encodes a structured dissent:
"I disagree with claim X, and here is why."

An objection is explicitly an **attested assertion** — the witness signs that
the objector asserted it, not that the objection is substantively correct. The
epistemic ladder already distinguishes this case (Model 3 boundary: cell claims
propagate but don't accrue thickness).

### Structurally

The existing `WitnessedClaim` envelope carries everything needed:

- `claimant` — who objects
- `claim_type = Objection`
- `start_epoch` / `end_epoch` — window (likely single-epoch for an objection)
- `witnesses` — signed attestations that the objection was made
- `evidence` — optional field; could carry a `target_claim_id` or free-text reason

No new struct, no new RPC, no new persistence path. The objection rides the
same envelope, same witness flow, same acceptance, same durability guarantees.

---

## 2. What is the acceptance boundary?

Same as service attestation: the claim passes through the existing witness gates
(established peer, no self-witness, within window). Acceptance means "the
network records that this objection was made" — not "the network agrees the
objection is correct."

The ladder applies:

| Level | Meaning |
|---|---|
| Recorded | Objection submitted, persisted, retrievable |
| Witnessed | Attested by an established peer |
| Engaged | Referenced by a later claim or proposal |
| Consequential | Led to a changed decision |
| Valued | (Future) Retroactive credit |

---

## 3. What prevents the archive from becoming unusable?

Start with protocol-level constraints before adding economic ones:

1. **Established-participant gate** — only peers with `heartbeats_received > 0`
   can submit (already enforced by the witness infrastructure).
2. **One objection per (claimant, target_claim_id)** — enforced by `last_claimed`
   monotonic window defense, same as service claims.
3. **No thickness accrual** — objections don't earn standing; there's no economic
   incentive to flood the system with them.

If spam still occurs at scale, *then* measure the problem and decide whether
rate-limiting or staking is warranted.

---

## 4. What would constitute success?

The experiment tests one question: *Does the lattice naturally make use of
preserved disagreement when that disagreement is available?*

| Signal | What it tells us |
|---|---|
| Objections submitted | Is there demand for structured dissent? |
| Objections retrieved | Is the archive discoverable? |
| Objections engaged | Is the content useful? |
| Objections resolved | Can disagreement produce movement? |
| Proposals changed | Does dissent have consequence? |
| Same objection recurred | Is the archive failing at retrieval? |

The last metric is the most diagnostic. If five participants independently
raise the same objection, the archive exists but the system hasn't learned
from it — a retrieval problem, not a reasoning problem.

---

## Implementation checklist

- [ ] Add `ClaimType::Objection = 1` to the enum
- [ ] Extend `name()` to return `"objection"`
- [ ] Confirm `accept_claim` handles `Objection` without changes
- [ ] Confirm `queue_claim` / claims WAL persist objections
- [ ] Confirm `count_distinct_witnesses` works for objections
- [ ] Confirm `thickness != 0` guard: objections don't accrue standing
- [ ] Add test: objection accepted, persisted, survives restart
- [ ] Add test: objection's witnesses are counted correctly
- [ ] Add test: objection does NOT generate thickness or earned credit

---

## What this is NOT

- It is NOT a retroactive-nutrient system
- It is NOT an economic mechanism
- It is NOT a governance decision protocol
- It is NOT a proposal/amendment workflow

It is the smallest possible extension of the existing claim infrastructure
that makes the chapter's core insight — "memory of disagreements, not merely
decisions" — testable against the running lattice. The economic questions
come later, if and only if the archive demonstrates utility.

---

## Relationship to Chapter 12

The chapter's principle: "A conscious lattice remembers what challenged the
decision." The mechanism described here tests that principle at minimum
viable scale. If objections are preserved, retrieved, and engaged, the
principle is demonstrated. The retroactive-credit question is deferred to
Pass 2 and is not addressed by this design.
