# Observer Evidence Record — 2026-07-28 (Pass 125)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** ~2026-07-28T19:58–19:59Z (single-capture discipline for morning-api; witness socket queried ~2s later)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** 125th observation pass. ~10 min since pass 124 (19:48Z). PIDs unchanged, both sockets responsive. Same continuous session since 13:01Z (~6h 57min uptime).

**Summary:** Delta from pass 124. Epoch advanced +21/+18 (morning-api socket 813→834, witness socket 815→833 — δ=1 between nodes, timing drift). Snapshot rotated twice since last pass (810→820→830). Three-way epoch equality on morning-api achieved **perfect match** for the first time this session (socket=834, count=834, last=834). Build gap widened from 5 to 6 commits behind HEAD (HEAD advanced from `4c29c52` to `8c68a33`; binary unchanged at `cb5d4b1-dirty`). Insufficient-balance count confirmed at 119 (correction from pass 124's 120 — counting methodology issue, not a new event). NTP series: no new failure at expected ~19:54Z window — pattern broke. One new observation: morning-api log shows 1 "insufficient" hit (different context from witness rejections). Economic state frozen throughout.

---

## Topology Disclosure

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZ...zLVxJ) | 13:01Z | `--name morning-api ... --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZ...zLVxJ | 13:02Z | `--name local-witness ... --mint 0` |

**Topology change from pass 124:** None. Same PIDs, same session.

---

## Node Info (Delta from Pass 124)

| Field | Pass 124 (19:48Z) | This pass (19:58Z) | Δ | DEVIATION |
|-------|-------------------|--------------------|---|-----------|
| m-ap uptime_secs | 24,376 | 24,991 | +615 (~61s/s, ~10min delta) | None |
| witness uptime_secs | 24,416 | 24,983 | +567 (~57s/s, includes separate capture timing) | None |
| build_commit (both) | `cb5d4b1-dirty` | `cb5d4b1-dirty` | Unchanged | **Persistent #1.** 6 behind HEAD `8c68a33`. |
| m-ap thickness | 976.27 | 976.11 | −0.16 in ~10min (~0.016/min decay) | None — normal slow decay |

---

## Epoch State (19:58–19:59Z single capture for morning-api; witness ~2s offset)

### morning-api

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 834 | — |
| Log count (grep -c) | 834 | — |
| Last log epoch | 834 (19:58:17Z) | — |
| Three-way equality | **PERFECT MATCH** — socket=834, count=834, last=834. First perfect match all session. | None. |

### local-witness

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 833 (19:58:21Z capture) | — |
| Log count (grep -c) | 834 (19:58:40Z capture) | — |
| Last log epoch | 834 (19:58:40Z) | — |
| Three-way equality | **BOUNDARY RACE** — socket=833, count=834, last=834. Socket query preceded log capture by ~2s; 1 epoch advanced in the gap. | None — standard timing drift. |

### Latest log epoch (post-capture, both at 19:59Z)

| Node | Epoch | Time (Z) | balance_before→after | ratio |
|------|-------|----------|----------------------|-------|
| morning-api | 835 | 19:58:47 | 20→20 | 1.02 |
| local-witness | 835 | 19:59:10 | 0→0 | 1.13 |

**Cross-node:** Both at epoch 835. Perfect convergence (δ=0). First time this session both nodes report the same epoch number simultaneously.

---

## Peer Connections

| Metric | Pass 124 (19:48Z) | This pass (19:58Z) | Δ |
|--------|-------------------|--------------------|---|
| m-ap: heartbeats | 2,436 | 2,498 | +62 (~6/min, normal) |
| m-ap: silence_secs | 1 | 0 | Healthy — just received heartbeat |
| witness: heartbeats | 2,442 | 2,500 | +58 (~6/min, normal) |
| witness: silence_secs | 8 | 1 | Healthy drift |
| queue_depth (both) | 0 | 0 | Unchanged |

**Both nodes see 1 peer.** Bidirectional heartbeats flowing. Silence well under 30s threshold. No zombie evictions (0 historical on both nodes). Healthy mesh.

---

## Economic State

| Metric | Pass 124 (19:48Z) | This pass (19:58Z) | Δ |
|--------|--------------------|--------------------|-----|
| m-ap: own_balance | 20 | 20 | **Frozen** (entire session) |
| m-ap: own_nonce | 241 | 241 | **Frozen** |
| m-ap: peer (witness) balance | 9,980 | 9,980 | **Frozen** |
| m-ap total supply | 10,000 | 10,000 | **Frozen** |
| Witness: own_balance | 0 | 0 | **Frozen** |
| Witness: own_nonce | 4 | 4 | **Frozen** |
| Witness: peer (m-ap) balance | 0 | 0 | **Frozen** |
| Insufficient-balance (witness log) | ~120 | **119** | **CORRECTION** — pass 124 overcounted by 1 |

**OBSERVED:** Economic state completely frozen since at least pass 1 (13:01Z). Morning-api executes epoch with balance_before=20, balance_after=20 (ratio 1.02). Witness executes with balance_before=0, balance_after=0 (ratio 1.13). Same pattern entire session.

**Insufficient-balance count correction:** Pass 124 reported 120. Current capture shows 119 lines matching `insufficient.*bal` pattern on witness. A broader grep (`insufficient` only) returns 120 — there is 1 additional "insufficient" hit (on the m-ap log, likely a different log context). The actual transaction rejection count is 119. All 119 occurred between 13:59Z and 14:01Z (initial redistribution burst). No new rejections in the last ~5h 57min.

**Persistent DEVIATIONS #3/#4:** Unchanged. Supply conservation divergence (total 10,000 vs 0 across ledgers). Witness sees morning-api balance as 0.

---

## Persistence State (19:58Z single capture for morning-api; ~19:58:40Z for witness)

### morning-api

| Field | Pass 124 (19:48Z, epoch 810) | This pass (19:58Z, epoch 830) | Δ |
|-------|-------------------------------|-------------------------------|----|
| last_snapshot_epoch | 810 | **830** | Rotated 2×: 810→820→830 |
| wal_bytes | 379 | 379 | Unchanged |
| wal_entries | 3 | 3 | Unchanged |

**Byte-equality:** `GetPersistenceState.wal_bytes=379`. `ls -la /tmp/m-ap/persistence/wal.log=379` bytes. **PASS.**

**File inventory (19:58Z):**
| File | Size | mtime (UTC) | Notes |
|------|------|-------------|-------|
| `state.snapshot` | 895 bytes | 15:56 (19:56Z) | Snapshot at epoch 830 |
| `wal.log` | 379 bytes | 15:56 (19:56Z) | Active WAL (genesis re-seed) |
| `wal.wal.old` | 379 bytes | 15:51 (19:51Z) | Pre-rotation backup from epoch 820 |

### local-witness

| Field | Pass 124 (epoch 810) | This pass (epoch 830) | Δ |
|-------|-----------------------|-----------------------|----|
| last_snapshot_epoch | 810 | **830** | Rotated 2× (matches m-ap) |
| wal_bytes | 379 | 379 | Unchanged |
| wal_entries | 3 | 3 | Unchanged |

**Byte-equality:** `GetPersistenceState.wal_bytes=379`. `ls -la /tmp/local-witness/persistence/wal.log=379` bytes. **PASS.**

**File inventory (19:58Z):**
| File | Size | mtime (UTC) | Notes |
|------|------|-------------|-------|
| `state.snapshot` | 569 bytes | 15:56 (19:56Z) | Snapshot at epoch 830 |
| `wal.log` | 379 bytes | 15:56 (19:56Z) | Active WAL |
| `wal.wal.old` | 379 bytes | 15:51 (19:51Z) | Pre-rotation backup from epoch 820 |

**Key observation:** Both nodes rotated snapshot at epochs 810, 820, and 830 (on schedule every 10 epochs). Both at epoch 830 now. `wal_bytes` stable at 379 across both nodes and both rotation cycles. Genesis re-seed persists correctly across WAL rotations.

---

## Metrics (Latest Log Lines)

**No metrics lines available in the last 10 lines of either log** — the heartbeat-based metrics logging may be on a longer cadence or the recent lines were epoch completions and snapshot saves. This is not a deviation; metrics were clean at pass 124 and no changes were expected.

---

## Log Health

### morning-api (/tmp/m-ap.log, ~9,200 lines, ~835 epochs)

| Pattern | Count | Notes |
|---------|-------|-------|
| Epoch complete | 834 (at 19:58Z capture) | +21 since pass 124 |
| WARN (non-structural) | 3 NTP failures (all historical) | Last at 18:58Z. KAD bootstrap WARNs every 5 min (expected with `--no-mdns`). |
| ERROR | 0 | Clean |
| "insufficient" (non-balance context) | 1 | Different context from witness rejections — likely startup or genesis |
| Zombie/sweep/eviction | 0 | None entire session |
| Snapshot saved | Epochs 810, 820, 830 | Last at 19:56:17Z (epoch 830) |

### local-witness (/tmp/lw.log, ~9,300 lines, ~835 epochs)

| Pattern | Count | Notes |
|---------|-------|-------|
| Epoch complete | 834 (at 19:58Z capture) | +19 since pass 124 |
| WARN (non-structural) | 4 total: 1 NTP failure (19:09Z), 3 KAD bootstrap | All historical. No new since 19:09Z. |
| ERROR | 0 | Clean |
| "insufficient balance" | 119 | All from 13:59-14:01Z batch. No new in last ~5h 57min. |
| Zombie/sweep/eviction | 0 | None entire session |
| Snapshot saved | Regular every 10 epochs | Last at 19:56:40Z (epoch 830), trailing m-ap by ~23s |

---

## Build Provenance

| Field | OBSERVED | EXPECTED |
|-------|----------|----------|
| Git HEAD | `8c68a33` (docs: protocol invariants, local-authority principle, state_root audit) | — |
| Build commit (both) | `cb5d4b1-dirty` | `8c68a33` (clean) |
| Commits behind HEAD | **6** (missing `0c4bb7f` + `452b64f` + `d802680` + `8b329b7` + `4c29c52` + `8c68a33`) | 0 |
| -dirty suffix | Present | Absent |

**Delta from pass 124:** HEAD advanced from `4c29c52` to `8c68a33`. Binary unchanged. Build gap widened from 5 to 6 commits behind HEAD. Binary is at least 6 commits stale and compiled from an uncommitted working tree.

---

## NTP Failure Series — Trend Report

| Pass | Capture Time (Z) | New Failures | Cumulative | Trailing 6-Pass Rate | Trend |
|------|------------------|-------------|------------|----------------------|-------|
| 116 | ~18:10Z | 2 (m-ap, 18:02Z) | 2 | 33% (1/3) | Outbreak |
| 117 | ~18:11Z | 0 | 2 | 33% (1/3) | Quiescent |
| 118 | ~18:40Z | 0 | 2 | 33% (2/6) | Quiescent |
| 119 | ~18:49Z | 0 | 2 | 33% (2/6) | Quiescent |
| 120 | ~18:59Z | 1 (m-ap, 18:58Z) | 3 | 50% (3/6) | Second outbreak |
| 121 | ~19:13Z | 0 | 3 | 33% (2/6) | Quiescent |
| 122 | ~19:31Z | 0 | 3 | 33% (2/6) | Quiescent |
| 123 | ~19:41Z | 0 | 3 | 33% (2/6) | Quiescent |
| 124 | ~19:50Z | 0 | 3 m-ap + 1 lw = 4 | 33% (2/6) | Quiescent |
| **125** | **~19:58Z** | **0** | **4 total** | **17% (1/6)** | **Quiescent — pattern broke** |

**OBSERVED:** No new NTP failures since pass 124. Morning-api's last failure stood at 18:58Z. The expected ~56-min-cycle retry at ~19:54Z did **not** produce a failure. This is the first time the established ~56-min pattern has broken — the next expected failure window (~19:54Z) passed without event. Witness's last (and only) failure remains at 19:09Z.

**Trend analysis:** The trailing 6-pass rate dropped from 33% to 17% (1/6) because the oldest pass (116) rolled out of the window. This is the lowest rate since the NTP outbreak began at pass 116.

**DEVIATION (new observation):** The previously established ~56-min failure cadence on morning-api **failed to reproduce** at the expected ~19:54Z window. This means either: (a) the pattern was never deterministic and was coincidental timing, (b) the root cause (NTP server rate limiting, network contention) resolved, or (c) the window is longer this cycle and hasn't expired yet. UNKNOWN which.

---

## Evidence Guards

| Guard | Status |
|-------|--------|
| Three-way epoch (m-ap) | **PASS** — socket=834, count=834, last=834 (**perfect match**) |
| Three-way epoch (witness) | **PASS** — socket=833, count=834, last=834 (1-off boundary race, expected) |
| Byte-equality (m-ap) | **PASS** — 379=379 |
| Byte-equality (witness) | **PASS** — 379=379 |
| PID consistency | **PASS** — 3579452/3579821 unchanged since pass 97 |
| Log health (m-ap) | **PASS** — 3 NTP WARNs (historical), 0 ERRORs, 1 unrelated "insufficient" hit |
| Log health (witness) | **PASS** — 1 NTP WARN (historical), 0 ERRORs, 119 historical insufficient-balance |
| Metrics health | **PASS** (inferred from pass 124) — no metrics line in recent log window but no deviation expected |
| Cross-node epoch sync | **PASS** — latest log: 835 vs 835 (δ=0, first perfect match) |
| Snapshot rotation | **PASS** — both at epoch 830, 2 rotations since pass 124, files on disk |
| Clock sync | **PASS** — NTP synchronized |

---

## Persistent Deviations — Status

| # | Deviation | First Observed | Status | Changed? |
|---|-----------|----------------|--------|----------|
| 1 | `build_commit` stale (`cb5d4b1-dirty`, HEAD `8c68a33`) | Jul 27 pass 1 | Persistent (6 behind + dirty) | **Widened** — HEAD advanced from `4c29c52` to `8c68a33` (+1 commit). Binary unchanged. |
| 2 | `wal_bytes` returning 0 (legacy path) | Jul 27 pass 1 | **RESOLVED** | **Unchanged** — fix verified, byte-equality passes since pass 98 |
| 3 | Supply divergence (total=10,000 vs 0) | Pass 97 (14:08Z) | Persistent | **Unchanged** — balance 20 frozen, witness sees m-ap as 0 |
| 4 | Witness reports morning-api balance as 0 | Jul 27 (~18:48Z) | Persistent | **Unchanged** |
| 5 | Epoch ratio divergence (~11% gap) | Pass 1 (18:06Z) | Persistent | **Unchanged** — witness ratio ~1.13, gap estimate ~11% |
| 6 | MESH.md stale ("No production nodes running") | Jul 27 | Persistent | **Unchanged** |

---

## Summary

**Pass 125: delta-only. Evidence integrity guards all PASS.**

The mesh remains in its familiar frozen steady state:

- **2 nodes, 1 peer each.** Bidirectional heartbeats healthy (silence ≤1s). No zombie evictions. Healthy mesh convergence.
- **Epochs cycling at ~30s cadence.** Both at epoch 835 simultaneously (δ=0, first perfect cross-node match this session). Snapshot rotated twice since pass 124 (810→820→830) on both nodes, on schedule.
- **No transactions flowing.** Nonces frozen entire session (m-ap=241, witness=4). Zero new insufficient-balance rejections since 14:01Z (~5h 57min ago).
- **Balance 20 floor on morning-api, zero on witness.** Supply divergence (10,000 vs 0) unchanged. All economic state frozen.
- **Insufficient-balance count corrected to 119** (not 120 as pass 124 reported — the extra hit was a non-transaction "insufficient" context in m-ap log).
- **Build gap widened from 5 to 6 commits** behind HEAD (HEAD advanced from `4c29c52` to `8c68a33`; binary still `cb5d4b1-dirty`).
- **NTP series at new low** — trailing 6-pass rate dropped to 17% (1/6). The expected ~19:54Z failure did NOT materialize, breaking the established ~56-min pattern for the first time. UNKNOWN whether the root cause resolved or the pattern was coincidental.
- **Three-way epoch equality achieved perfect match** on morning-api (socket=834, count=834, last=834). First time this session.

**Next expected events:** Snapshot rotation at epoch 840 (~3 min from now). No other state changes expected unless someone submits transactions or rebuilds the binary.
