# Verified Protocol Behavior

**Updated:** 2026-07-27
**Authority:** Every entry sourced to an observation on real nodes or a
specific test run. No claims without evidence.

Tiers distinguish between "seen working on a live mesh," "proven by unit
test only," and "shipped, never exercised." Conflating them is how
"verified" degrades into "we wrote a test."

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

**What was observed (Jul 27, e7d9e1e):**
- `SubmitObjection { target_claim_id, reason }` → `ObjectionSubmitted`
- `GetAllObjections` → returns correct objector/reason/timestamp
- Same submission repeated → `Error: Duplicate { claim_id, objector }`
- Kill-9 + restart → objection recovered and queryable
- Build: `ae89fbd` (hash lag, no code changes between ae89fbd and e7d9e1e
  in the persistence path)

---

## Verified by Test Only (Never Exercised on a Live Mesh)

### Objection Cap Enforcement

Three unit tests in `persistence.rs` (b4aa212):

- `cap_accepts_64th_distinct_objector` — 64 distinct objectors, 64th accepted
- `cap_rejects_65th_distinct_objector` — 65 distinct objectors, 65th rejected
- `cap_duplicate_after_full_is_noop_not_rejection` — duplicate from existing
  objector after cap reached is a no-op, not a cap rejection

The cap logic (inside `process_objection`) operates on the in-memory
`HashMap` the same way regardless of source — receive via gossip or submit
via UDS. The map operations are identical. But the receive boundary was never
exercised with 64 distinct objectors on a running mesh.

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

- **Objection cap at 64 on a live receive boundary.** Unit tests cover the
  map operations; no test exercises the gossip handler receiving objection
  #65 from a real peer.
- **Objection propagation.** The `publish_objection` call executes after
  `process_objection`, but no test verified that a second node receives
  an objection submitted via UDS on the first.
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
