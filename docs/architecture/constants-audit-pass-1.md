# Constants Audit — Pass 1: Arithmetic Coherence

**Status:** Complete
**Scope:** Pass 1 only — arithmetic coherence, no behavioral claims
**Date:** 2026-07-25
**Baseline:** main @ 36d4eb0

---

## Scope Discipline

This document asks exactly one question, constant by constant:

> **Are the numerical relationships among the constants internally consistent with their stated intent?**

A Pass 1 failure looks like: a prune floor set to exactly the value of a single
credit, so no earned edge can survive one decay pass. Catching that required no
economics — only two numbers compared.

---

## Baseline Values

These are the ground-truth figures all derived checks depend on:

| Parameter | Value | Source |
|-----------|-------|--------|
| Epoch interval | 30 s | `src/main.rs:81` — `#[arg(long, default_value_t = 30)]` |
| Heartbeat interval | ~10 s | Observed in event loop |
| Epochs per day | 2,880 | 86,400 s / 30 s |
| Epochs per 30 days | 86,400 | 30 × 2,880 |

---

## A. Economic Constants

### A1. Thickness Decay Factor

| Field | Value |
|-------|-------|
| Constant | `DECAY_PER_EPOCH` |
| Value | `0.999_982_885_4` |
| Defined at | `src/claims/mod.rs:117` |
| Read by | `apply_edge_decay` in `src/ledger/thickness.rs`; `run_economic_epoch` in `src/node.rs:2072` |
| Must be consistent with | Epoch interval (30s); stated 30-day half-life |
| Arithmetic check | 30 days = 2,592,000 s / 30 s per epoch = 86,400 epochs. 0.9999828854^86400. **PASS.** The factor was chosen to produce exactly 0.5 at 86,400 epochs. Ratio = ln(2) / (−ln(0.9999828854)) ≈ 40,500 epochs ≈ 14 days at current epoch interval. The half-life is ~14 days, not 30. See A1-Open below. |
| Result | **NEEDS REVIEW** — the factor is mathematically correct for some half-life duration, but the relationship between that duration and the stated "30-day" figure depends on what epoch duration value the factor was computed against. If it was computed against a 64s epoch instead of the actual 30s, the effective half-life is not 30 days. |

### A1-Open: What epoch duration was the decay factor calibrated against?

The current epoch interval is 30 s (CLI default). At 30 s, 0.9999828854^86400 ≈
0.230, not 0.5. The factor 0.9999828854^x = 0.5 solves to x = 40,500 epochs =
~14 days. For the stated 30-day half-life to hold, the epoch duration at time
of calibration would need to have been 2,592,000 / 40,500 ≈ 64 s.

The 64 s figure appears to be an older design value, not the current implementation
default. If that is correct, the effective half-life under actual running conditions
is ~14 days, not the stated 30. This is a **factual discrepancy between stated
design intent and implemented behavior** — pure arithmetic, no behavioral claim.

### A2. Prune Floor

| Field | Value |
|-------|-------|
| Constant | `MIN_THICKNESS` (thickness module) |
| Value | `1e-6` |
| Defined at | `src/ledger/thickness.rs:32` |
| Read by | `apply_edge_decay` (prune pass) in `src/ledger/thickness.rs:~262` |
| Must be consistent with | Smallest possible credit (0.001 per heartbeat) |
| Arithmetic check | Ratio: 0.001 / 1e-6 = 1000×. A one-heartbeat edge survives ~403,600 epochs ≈ 140 days before pruning. **PASS.** |
| Result | **PASS** (post-fix; pre-fix value 0.001 gave ratio 1× → 0 epochs survival → FAIL) |

### A2-A: Duplicate Constant

There is a **second** `MIN_THICKNESS` constant:

| Field | Value |
|-------|-------|
| Constant | `MIN_THICKNESS` (claims module) |
| Value | `0.001` |
| Defined at | `src/claims/mod.rs:120` |
| Read by | Imported in `src/claims/acceptance.rs:10` but **never referenced in any function body** — only in a doc comment on `PairHistory` (line 24) |
| Arithmetic check | If this constant were ever used, it would reintroduce the original prune-at-credit bug. Currently inert. |
| Result | **NEEDS REVIEW** — dead import. Remove to prevent accidental future use. |

### A3. Credit Multiplier

| Field | Value |
|-------|-------|
| Constant | (inline literal) |
| Value | `0.001` per observed heartbeat |
| Defined at | `src/node.rs:2054` — `w.observed_heartbeats as f64 * 0.001` |
| Read by | Claim accrual in `run_economic_epoch` (credit to thickness graph) |
| Must be consistent with | `MIN_THICKNESS` (1e-6); decay factor; nadir table from design |
| Arithmetic check | Single-heartbeat credit: 0.001. After one decay at `DECAY_PER_EPOCH`: 0.001 × 0.9999828854 = 0.00099998. Above prune floor of 1e-6. **PASS** for survival. |
| Result | **PASS** |

### A4. Genesis Amortization

| Field | Value |
|-------|-------|
| Constant | `amortize_over` parameter |
| Value | `Some(50)` (typical) |
| Defined at | Call site: `add_genesis_thickness` in `src/ledger/thickness.rs` |
| Read by | `derive_genesis_amount` in `src/ledger/thickness.rs:~635` |
| Must be consistent with | Contribution cadence; decay half-life |
| Arithmetic check | At 50 contributions, genesis fully amortizes to zero. The `contribution_count()` counter is **network-wide** — adding contributions to *any* peer increments it. Test evidence (`earned_thickness_independent_of_genesis_decay`): 30 contributions to random peers reduce genesis from 1000×(50/50) to 1000×(17/50)=340. |
| Result | **OPEN — see A4-Open** |

### A4-Open: Is the global counter deliberate?

`contribution_count()` is incremented by every `add_verified_contribution` call
regardless of target peer. A peer's own genesis decays when **anyone** in the
network earns a contribution. At two peers claiming daily, `k` reaches 50 in ~25
days — genesis fully evaporates in roughly one thickness half-life.

This may be exactly the intent (genesis is a bootstrap that washes out as the
network becomes productive). But it must be confirmed: the docstring on
`add_genesis_thickness` does not specify whether amortization is per-peer or
network-wide. If per-peer was intended, `contribution_count` should be scoped to
the genesis-bearing peer.

---

## B. Tax and Mint Constants

### B1. Tax Rate

| Field | Value |
|-------|-------|
| Constant | `base_tax_rate` (CLI parameter) |
| Value | `5` (percent) → `500` bps |
| Defined at | `src/main.rs:94` — `#[arg(long, default_value_t = 5)]` |
| Read by | `TaxEngine::execute_epoch` via `run_epoch` in `src/economics/mod.rs:166` |
| Must be consistent with | Mint rate; balance scale |
| Arithmetic check | Line 112: `tax_owed = balance.0 × tax_rate_bps / 10,000`. At 5% base rate on 272,317 DUU balance (observed live): tax_owed = 272,317 × 500 / 10,000 = 13,615 DUU. Effective rate scales inversely with contribution ratio — a ratio of 2.0 (contributor) halves the rate; ratio of 0.5 (consumer) doubles it. Redistribution (line 138): `share_per_peer = tax_owed / peer_count`. Equal split. |
| Result | **PASS** — arithmetic is internally coherent. Tax = balance-based, contribution-ratio-adjusted, equally redistributed. |

### B2. Mint Rate

| Field | Value |
|-------|-------|
| Constant | `base_mint_rate` (CLI parameter) |
| Value | `1` (DUU per epoch base) |
| Defined at | `src/main.rs` — `#[arg(long, default_value_t = 1)]` |
| Read by | `calculate_mint_from_receipts` in `src/economics/mint.rs:73` |
| Must be consistent with | Tax rate; balance scale |
| Arithmetic check | Mint = `base_mint_rate × verified_score` where verified_score is receipt-gated (relay work observed by a third party). At current mesh size (2 peers), verified relay score is ≈0, so mint is 0. Tax at 5% on 272,317 balance is 13,615 DUU. Net: outflow ≈ 13,615, inflow ≈ 0 per epoch. |
| Result | **PASS** — arithmetic is coherent. The tax-mint asymmetry (tax flowing, mint gated) is a design property, not an arithmetic flaw. |

### B3. Tax-Mint Arithmetic Relationship

| Check | Result |
|-------|--------|
| Does mint inflow exceed tax outflow? | **No, at current mesh size.** Mint is receipt-gated (requires third-party relay observation), and a 2-node mesh rarely produces verified relay receipts. Tax is balance-based and fires every epoch regardless. A node in a small mesh is a net payer. |
| Is there arithmetic conservation? | Tax collection (line 150): `actual_collected = share_per_peer × peer_count`. The remainder `tax_owed − actual_collected` is absorbed at line 151-157. Value is conserved (collected + remainder = owed) but remainder is lost to the taxpayer rather than redistributed. |
| Result | **PASS** — arithmetic is internally consistent. The tax-mint gap is a structural property of the receipt gate, not a numerical error. |

---

## C. Liveness Constants

### C1. Wall-Clock Silence Threshold

| Field | Value |
|-------|-------|
| Constant | `ZOMBIE_EVICT_THRESHOLD_SECS` |
| Value | `90` |
| Defined at | `src/node.rs:76` |
| Read by | `check_peer_liveness` Layer 1 in `src/node.rs:~3832` |
| Must be consistent with | Heartbeat interval (~10 s) |
| Arithmetic check | 90 s / ~10 s per heartbeat ≈ 9 missed heartbeats. **PASS.** |
| Result | **PASS** |

### C2. Epoch Silence Threshold

| Field | Value |
|-------|-------|
| Constant | `ZOMBIE_EPOCH_THRESHOLD` |
| Value | `30` epochs |
| Defined at | `src/node.rs:104` |
| Read by | Layer 2a eviction (heartbeat-specific) |
| Must be consistent with | Layer 1 threshold (90 s); epoch interval (30 s) |
| Arithmetic check | 30 epochs × 30 s = 900 s = 15 min. Layer 1 fires at 90 s on the same signal. Ratio: 900 / 90 = 10×. Layer 1 always fires first. |
| Result | **NEEDS REVIEW — see C2-Open** |

### C2-Open: Are the two layers 10× apart by intent?

Layer 1 tracks `last_seen` (any activity) at 90 s. Layer 2a tracks
`last_heartbeat_epoch` (heartbeats specifically) at 30 epochs = 900 s. Layer 1
always fires first on the same peer. Layer 2a exists to catch a peer that is
otherwise active (passing Layer 1) but has silently stopped heartbeating.

The 10× gap means a heartbeat-silent peer that is otherwise chatty survives
for 15 minutes after the Layer 1 threshold. Whether this is deliberate is a
design-intent question; the arithmetic is coherent but the two thresholds
are not comparable in magnitude.

### C3. Cold-Start Grace Period

| Field | Value |
|-------|-------|
| Constant | `COLD_START_GRACE_SECS` |
| Value | `300` |
| Defined at | `src/node.rs:85` |
| Read by | Both eviction layers (guarded by `in_grace` check) |
| Must be consistent with | Observed connect-to-first-heartbeat interval (~10-15 s) |
| Arithmetic check | 300 / 15 = 20× the worst observed interval. **PASS.** |
| Result | **PASS** |

---

## D. Claim Lifecycle Constants

### D1. Collection Window

| Field | Value |
|-------|-------|
| Constant | `CLAIM_COLLECTION_EPOCHS` |
| Value | `5` epochs |
| Defined at | `src/node.rs:120` |
| Read by | `sweep_claims` expiry check in `src/node.rs:2707` |
| Must be consistent with | Witness RPC round-trip time |
| Arithmetic check | 5 epochs × 30 s = 150 s = 2.5 min. Local witness round-trips are sub-second. **PASS** with wide margin. |
| Result | **PASS** |

### D2. Retention Window

| Field | Value |
|-------|-------|
| Constant | `CLAIM_RETENTION_EPOCHS` |
| Value | `10` epochs |
| Defined at | `src/node.rs:125` |
| Read by | `sweep_claims` eviction in `src/node.rs:2723`; indirectly affects `count_distinct_witnesses` via `accepted_claims` |
| Must be consistent with | Thickness edge lifetime (indefinite) |
| Arithmetic check | 10 epochs × 30 s = 300 s = 5 min. A claim is retained for 5 minutes; its thickness edge persists indefinitely. `distinct_witnesses` is computed from `accepted_claims`, which is drained by the sweep — so the witness count drops to absent ~5 minutes after each claim, while `earned_thickness` persists. |
| Result | **FAIL** — two numbers describing the same event (a credited claim) have different lifetimes: 5 minutes vs. indefinitely. The witness count decays to absent while the earned thickness it was meant to qualify remains visible. |

### D3. Maximum Claim Window

| Field | Value |
|-------|-------|
| Constant | `MAX_CLAIM_WINDOW` |
| Value | `1000` epochs |
| Defined at | `src/claims/mod.rs:108` |
| Read by | `accept_claim` validation in `src/claims/acceptance.rs:105` |
| Must be consistent with | Epoch interval (30 s); realistic claim cadence |
| Arithmetic check | 1000 epochs × 30 s = 30,000 s = 8.3 hours. A claim window of up to 8.3 hours is well above any realistic service attestation window. The constant acts as an upper bound on malformed-input rather than a realistic constraint. |
| Result | **PASS** — acts as a malformed-input guard, not an active constraint. |

### D4. Minimum Witnesses

| Field | Value |
|-------|-------|
| Constant | `MIN_WITNESSES` |
| Value | `1` |
| Defined at | `src/claims/mod.rs:113` |
| Read by | `accept_claim` in `src/claims/acceptance.rs:122-129` |
| Must be consistent with | Established peer count |
| Arithmetic check | Clamp logic: `effective = min(MIN_WITNESSES, established_peers − 1)`. At 2 nodes (self + 1 peer): effective = min(1, 1) = 1. At 1 node (isolated): effective = min(1, 0) = 0, claim blocked. |
| Result | **PASS** — arithmetic is coherent. See capability caveat below. |

### D4-Caveat: Old-Binary Peers

`established_peers` counts all peers with `heartbeats_received > 0`. Peers on
old binaries that don't speak `/lattice/witness/v1` are counted as established
but cannot witness. At `MIN_WITNESSES = 1` this is benign. If the constant
rises, the clamp may count nodes structurally unable to sign. This is a
capability-negotiation gap (stabilization gate 4), not an arithmetic flaw.

---

## E. Constants Consistency Matrix

| Constant pair | Relationship | Consistent? |
|---------------|-------------|-------------|
| `DECAY_PER_EPOCH` vs epoch interval | Half-life calibration | **NEEDS REVIEW** — computed for ~64s epoch, current is 30s |
| `MIN_THICKNESS` (thickness) vs credit (0.001) | Prune floor vs smallest credit | **PASS** — 1000× ratio |
| `MIN_THICKNESS` (claims, 0.001) vs credit (0.001) | Duplicate constant, dead import | **NEEDS REVIEW** — remove |
| `CLAIM_RETENTION_EPOCHS` vs edge lifetime | Claim visibility vs thickness persistence | **FAIL** — 5 min vs indefinite |
| `ZOMBIE_EVICT_THRESHOLD` vs heartbeat interval | Wall-clock silence tolerance | **PASS** — ~9 heartbeats |
| `COLD_START_GRACE_SECS` vs connect interval | Startup protection | **PASS** — 20× margin |
| `base_tax_rate` vs `base_mint_rate` | Tax-mint balance | **PASS** — coherent but receipt-gated mint means net outflow in small mesh |
| `ZOMBIE_EPOCH_THRESHOLD` vs Layer 1 | Two eviction layers | **NEEDS REVIEW** — 10× gap |

---

## Pass 1 Summary

| Result | Count | Rows |
|--------|-------|------|
| PASS | 7 | decay factor (math), MIN_THICKNESS (prune), credit multiplier, tax rate, mint rate, wall-clock silence, cold-start grace, claim window, MIN_WITNESSES |
| FAIL | 1 | CLAIM_RETENTION_EPOCHS vs edge lifetime |
| NEEDS REVIEW | 3 | decay half-life vs stated 30-day (epoch interval mismatch), dead MIN_THICKNESS import (claims/mod.rs), eviction layer gap |

---

## Open Arithmetic Questions

1. **What epoch duration was the decay factor calibrated against?** 0.9999828854 produces half-life at 40,500 epochs. At current 30 s epochs that's ~14 days. At the older 64 s epochs that's ~30 days. Determine the design baseline and either adjust the constant or document the effective half-life.

2. **Is the global `contribution_count()` for genesis amortization deliberate?** The counter is network-wide, not per-peer. Confirm against design intent.

3. **Why two eviction layers 10× apart?** Layer 1 (90 s wall-clock) and Layer 2a (30 epochs = 900 s) differ by an order of magnitude. If the intent was "roughly comparable tolerance for two signal types," the constants disagree.

---

## Explicitly Deferred to Pass 2

- Whether the tax actually reduces concentration
- Whether standing correlates with useful contribution
- Whether genesis amortization produces a fair transition
- Whether any constant's value is *right* rather than *coherent*
- Economic desirability of the tax-mint asymmetry
- Behavioral effect of the 14-day vs 30-day half-life difference
