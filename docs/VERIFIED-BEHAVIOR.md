# Verified Protocol Behavior

**Updated:** 2026-07-28

Tiers distinguish between "seen working on a live mesh," "proven by unit
test only," and "shipped, never exercised." Conflating them is how
"verified" degrades into "we wrote a test."

---

## Protocol Invariants

### Supply Conservation (PROPOSED)

> A debit recorded on any node's ledger without a corresponding credit on
> the recipient's ledger is a violation of supply conservation unless the
> debit is reversed.

This invariant was assumed but never stated in any design document. It is
recorded here as PROPOSED — the governance layer decides whether to adopt
it. Currently CONTRADICTED by evidence (see Not Verified — Confirmed
Protocol-Level Findings).

### Convergence (PROPOSED, provisional bound)

> Divergence in economically relevant state between nodes must be
> detectable within 10 epochs.

The 10-epoch bound is provisional — testable without pretending the value
is permanently settled. No detection mechanism currently exists.

---

## Architectural Principle: Local Authority

> No local state, action, or assertion becomes authoritative merely because
> it succeeded locally.

This principle has been independently observed in four separate instances
across the Lattice codebase and the Engineering Cell:

1. **Local mint never propagated.** `Minting starting balance to local node
   amount=5000` — applied locally, never entered the wire on a 2-node mesh.
   Witness had no path to learn the root's balance.

2. **Sender debited before recipient confirmation.** Redistribution applied
   `validate_and_apply` locally, then `flush_outbound`. 4,980 DUU debited,
   0 credited. The debit was authoritative locally; the credit never
   materialized.

3. **`publish()` returned Ok before delivery.** `gossipsub.publish()` returns
   a `message_id` for local queue insertion. The injector's process exited
   before gossipsub forwarded the message. `message_id` was read as delivery
   confirmation; it wasn't.

4. **Agent verification didn't confirm delivery.** Both the Observer
   (balance-divergence diagnosis) and the Verifier (APPROVED verdict)
   produced claims that exceeded the evidence they were based on. Each was
   caught and corrected by a subsequent pass.

In all four cases, a subsystem reported success locally. In all four cases,
that success did not establish the corresponding distributed property. The
reflex that caught them — checking whether the verification mechanism itself
ran — is the operational form of this principle.

---

## Verified End-to-End on Running Nodes

### Genesis Lifecycle

Full sequence: author → persist → gossip → receive → validate → persist
on peer → snapshot at epoch 10 → WAL rotation → Genesis re-seed → restart
→ recovered.

**What was observed (Jul 27, ae89fbd):**
- `morning-api` with `--auto-genesis` writes `SignedGenesis` to unified WAL
- `Genesis committed — era one begins` in startup log
- `local-witness` receives genesis via gossip, validates, persists
- At epoch 10: `Snapshot saved epoch=10`, fresh `wal.log` is 806 bytes
  (Genesis re-seed before any transactions)
- After kill-9: `Genesis recovered from WAL`, `verify_consistency passed`
- Build: `ae89fbd`, verified via `GetNodeInfo.build_commit`

**Propagation (0fbba1e):** Initial `re_gossip_genesis` fired on
`ConnectionEstablished` before gossipsub mesh had peers on
`/lattice/genesis/v1`. Fixed by retrying on `InsufficientPeers` via
`pending_genesis_gossip` set drained on each metrics tick (~10s).
Harness at `~/genesis-propagation-test.sh` exercises this on isolated
ports 4105/4110.

### Objection Pipeline Pass 1

Full sequence: submit via UDS → sign locally → validate → dedup → cap →
persist → gossip → recover on restart.

**What was observed (Jul 27-28):**
- `SubmitObjection { target_claim_id, reason }` → `ObjectionSubmitted` (Jul 27)
- `GetAllObjections` → returns correct objector/reason/timestamp (Jul 27)
- Same submission repeated → `Error: Duplicate { claim_id, objector }` (Jul 27)
- Kill-9 + restart → objection recovered and queryable (Jul 27)
- **Receive via gossipsub** → first real exercise (Jul 28, EXP-CAP-002).
  65+ objections published, deserialized, validated, and correctly
  dispositioned. Transport delivery required injector fix (8b329b7).
- **Cap enforcement** → 64 distinct accepted, 65th rejected with limit
  named, duplicate after cap is no-op not cap rejection (Jul 28, EXP-CAP-002).

---

## Verified by Test Only (Never Exercised on a Live Mesh)

### Objection Validation

10 tests in `validation.rs` (6ec9470): bad signature, duplicate
same-claim-same-objector, different-objector-same-claim OK, empty reason,
future timestamp, reason over 1024 bytes, missing public key, missing
signature.

### Witness Protocol + Two-Swarm Harness

4 tests in `two_swarm_witness_harness` and `witness_seam_tests` (fixed Jul 27,
32efcf1): two-swarm witness round-trip, orchestrate and accept service claim,
witness response to acceptance, witness response with decline rejected.

These tests had never passed since their commit — fixtures were written with
`submitted_epoch=0` against a validation rule (`submitted_epoch > end_epoch`)
that predated the tests. Found and fixed when the binary test target was
restored (214eb73).

---

## Implemented, Unverified

This is the bucket that matters when someone asks "what's safe to rely on."

- **Objection propagation.** The `publish_objection` call executes after
  `process_objection`, but no test verified that a second node receives
  an objection submitted via UDS on the first. (Verified Jul 28: cross-node
  propagation not tested, but gossip receive path verified via injector in
  EXP-CAP-002.)
- **GetAllObjections at scale.** Unbounded; returns everything. Not tested
  with >64 entries or across claims.
- **Observer-mode genesis Sybil surface.** An observer node can publish
  a different genesis with the same root signer (acceptance was
  unimplemented as of Commit 5). Not a current threat on a two-node
  trusted mesh; would matter on an open mesh.

---

## Known-Provisional

Items known to be incomplete or suboptimal, documented so future work
doesn't rediscover them.

| Item | Category | When |
|---|---|---|
| `wal.wal.old` naming (should be `wal.log.old`) | Cosmetic | ae89fbd |
| `wal_bytes` endpoint read legacy `transactions.wal` | Fixed | cb5d4b1 |
| `wal_entries` reports `size/120` heuristic — plausible but unrelated to actual entry count | Known-provisional | cb5d4b1 |
| `GetAllObjections` unbounded — needs pagination | Scale | e7d9e1e |
| No per-claim cap on total objections archive | Scale | 6ec9470 |
| Widget down — Netlify blobs read failing | Infra | Jul 27 |
| `build_commit` lags if no proto changes | Fixed | 6c29f97 |

---

## Not Verified — Confirmed Protocol-Level Findings

### Transfer Path Integrity

**Status:** CONFIRMED — wire path works correctly  
**Verified by:** Verifier Mission 1 (Jul 27, snapshot 23:23:12Z)

118/118 redistribution transactions transmitted, received, parsed, and
validated with exact amount preservation (500, 277, 236, …, 1). Gossip,
serialization, receipt, and parse all functioned correctly. Zero transaction
loss or wire-level corruption across 549 epochs.

### Supply Conservation

**Status:** CONTRADICTED — mesh-wide accounting divergence  
**Verified by:** Verifier Mission 1 (Jul 27), Observers passes 1-24

In the tested redistribution sequence, morning-api's ledger decreased by
4,980 DUU while local-witness credited 0 DUU. All 118 transfers were
received (transfer path confirmed) and all 118 were correctly rejected
by the witness's validation. The sender debited before recipient
confirmation. Total supply by morning-api's accounting: 5,000. By
witness's accounting: 0.

The relevant conservation invariant was not previously specified and is
now recorded as violated under the tested conditions.

**Candidate invariant (proposed, pending governance):**

> Supply Conservation Invariant: For every valid ledger state, the sum of
> all spendable balances across the mesh must equal the network's recognized
> total supply, subject only to explicitly defined issuance, destruction, or
> escrow states.

### Redistribution Supply Divergence — Original

**Status:** SUPERSEDED by the decomposition above  
**Observer pass:** #3 (Jul 27, 18:48 EDT)

Originally classified as "not a bug — predictable consequence of minimal
genesis." Both halves of that were wrong. Not predictable (no design
document predicted it), and the transfer-path integrity was never
separately tested. The divergences have now been decomposed into:

- CONFIRMED: transfer path works (positive finding)
- CONTRADICTED: supply conservation (negative finding)

---

## Causes / Contributing Conditions (separate from findings)

These explain observed failures but are not themselves verified findings.
They prevent the codebase from conflating explanation with evidence.

1. **Initial mint is local and not propagated.** `Minting starting balance
   to local node amount=5000` — applied locally, never enters the wire on
   a 2-node mesh with no third-party relay.

2. **Sender debits before recipient confirmation.** `src/node.rs:2032-2057`
   — `validate_and_apply` runs before `flush_outbound`. The debit is
   unconditional.

3. **No reconciliation mechanism.** The state_root and thickness_root work
   from Era Two exists but nothing currently uses it to detect or correct
   cross-node divergence.

---

## The Pattern That Found the Bugs

Jul 27 started as "verify the objection cap" and uncovered four production
defects, a broken diagnostic, and a dark test target. None were on any board
at the start. Every one surfaced by refusing a plausible explanation.

| What was said | What it was | How it was found |
|---|---|---|
| "Pre-existing test failure, unrelated" | `populated_wal_recovery_with_snapshot` was a real bug — genesis lost at every snapshot rotation | ae89fbd |
| "Production is fine, enable_persistence carries genesis" | `from_state()` hardcoded `genesis: None` | ae89fbd |
| "Silently dead code, pre-existing" | `two_swarm_witness_harness` wasn't gated — compilation errors killed the entire binary test target since fe33971 | 214eb73 |
| "Build hash lags, known issue" | `build_commit` hadn't updated since 294da2c — the running binary was genuinely stale, and the fix caught it | 6c29f97 |

The reflex that kept finding them: checking whether the verification
mechanism itself ran. A harness script that executes on a pre-fix binary
proves nothing. A test count reported against a target that doesn't compile
proves nothing. A diagnostic that hasn't fired since the last proto change
proves nothing.

A future contributor reading this before their own first "pre-existing
failure, unrelated" is the useful artifact.
