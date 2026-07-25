# Pass 1 Closeout — Arithmetic Coherence Audit

**Date:** 2026-07-25
**Status:** Complete
**Scope:** Pass 1 only — arithmetic coherence and lifecycle correctness
**Repository:** main @ e16381c (claims WAL fix)

---

## Method

The audit applied three separations throughout:

- **Row 0 first** — establish clocks and units before interpreting
  any constant denominated in them. Epoch duration, heartbeat interval,
  WAL format — the coordinate system before the measurements.
- **Invariant before check** — every row states what property the
  constant is supposed to hold before checking whether it holds. A
  check without a pass criterion is just a number.
- **Code fact / arithmetic fact / design intent / Pass 2** kept in
  separate columns. An unresolved implementation question cannot
  quietly become an architectural conclusion, and an empirical claim
  (Pass 2) cannot be smuggled into an arithmetic finding (Pass 1).

These separations are load-bearing. Pass 2's rows will need all
three, and so will any future subsystem audited against the same
standard.

---

## Resolved

| Finding | Disposition | Commit |
|---|---|---|
| Duplicate `MIN_THICKNESS` (0.001 vs 1e-6) | Removed stale definition; single authority at thickness.rs | f0dae2d |
| Decay factor miscalibrated for 64 s epoch | Corrected to 30 s / 30-day; value 0.9999919775 | 1a8bdb3 |
| C1a — witness count display invariant | Snapshot drain eliminated; count reads durable source | 294da2c |
| C1b — evidence retention across epochs | `accepted_claims` survives drain cycles; verified Level 1 | 294da2c |
| C1b — evidence retention across restart | `import_accepted_claims` wired in recovery; verified Level 2 | d6964bb |
| `last_claimed` reset on restart | Rebuilt from recovered `accepted_claims`; single authority | 1e88080 |
| C2 — crash durability of accepted claims | Claims WAL-logged at acceptance; verified Level 3 | e16381c |
| Claim replay idempotent across repeated restarts | Dedup on (claimant, claim_type, end_epoch); verified | e16381c |

---

## Known fix, not applied

These are deferred for explicit triggers, not as a permanent backlog.

| Fix | Trigger | Rationale |
|---|---|---|
| Derive decay factor from epoch + half-life | None — unblocked | Correct value with hardcoded literal; structural fix eliminates recurrence of the miscalibration class |
| Edge-keyed claim retention | When `accepted_claims` growth is measurable | Eliminated drain means unbounded growth; prune claims when their edges prune, so evidence and standing share a lifetime by construction |
| WAL truncation at snapshot | **Now** — C2 closed, WAL is complete event stream | Snapshot at time T contains full state; pre-T WAL entries are redundant. Bounds 6.6 MB/day growth and shortens replay to post-snapshot window |

---

## Open design question

| Question | Status |
|---|---|
| Eviction-gap intent — why is heartbeat-silence tolerated 10× longer (900 s) than activity-silence (90 s)? | Requires explicit design decision |

---

## Deferred to Pass 2

These require independent network participants and real history. They
are empirical questions, not code questions.

- Whether the cadence griefing vector (claim frequency drives genesis
  destruction 1:1 while producing no additional earned value) matters
- Whether throughput-dependent genesis equilibrium is desirable
- Whether the anti-concentration tax actually reduces concentration
- Whether the `established_peers` clamp correctly accounts for
  protocol-incapable peers at higher `MIN_WITNESSES`
- Whether any constant's value is *right* rather than *coherent*

---

## What the audit actually found

Nine findings. Three were actively losing data in production:

- The **snapshot drain** destroyed witness evidence every 10 epochs
- The **missing recovery import** silently zeroed `accepted_claims`
  on every restart, overwriting good data with empty state
- The **credit-before-snapshot tick ordering** produced a torn
  recovery where transactions advanced to crash-time but claims and
  thickness stayed at snapshot-time

None had any symptom anyone had noticed, except a witness count
disappearing from a webpage five minutes after each claim.

An audit that began as "are the constants arithmetically coherent"
found a persistence layer that silently destroyed evidence on every
restart. The constants were fine. The persistence invariants weren't.

---

## Revision History

| Date | Event |
|---|---|
| 2026-07-25 | Pass 1 opened |
| 2026-07-25 | Pass 1 closed — 8 resolved, 3 deferred, 1 open design question |
