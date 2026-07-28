# Observer Evidence Record — 2026-07-28 (Pass 126)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** ~2026-07-28T20:07–20:10Z (restart event interrupted first capture; all final values from 20:09–20:10Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** 126th observation pass. ~10 min since pass 125 (19:58Z).

**Summary:** RESTART EVENT DURING OBSERVATION. Both nodes were restarted between ~20:07:43Z (morning-api) and ~20:07:55Z (witness) while this observer was actively capturing data. The first socket queries hit the old session (uptime=25,464s, epoch=849, balance=20). Subsequent queries hit the new session (uptime=14s, epoch=1, balance=5,020). This is the first restart since the session began at 13:01Z (~7h 6min uptime). Key outcomes: build gap RESOLVED (binary now at HEAD `9f604ed-dirty` vs 6 behind before), but supply divergence WIDENED (total supply now 15,000 vs 5,000 minted — previous mint layer stacked on new mint). Stale process contamination detected: old witness (PID 3579821) NOT killed before new witness started — both bound to same UDS socket. Witness log corrupted by dual-writer interleaving. Old session had all 6 persistent deviations; 1 resolved (build gap), 5 remain.

---

## Topology Disclosure

**This machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, 100.93.232.107 Tailscale). Boynton Beach FL. Ubuntu 24.04.

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3946739 | morning-api | 4005 | auto (12D3KooWPfrZ...zLVxJ) | 20:07:43Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| **3579821 (STALE)** | local-witness | 4010 | 12D3KooWPfrZ...zLVxJ | **13:02Z** | `--name local-witness --port 4010 --identity-dir /tmp/lw-id...` |
| 3947000 | local-witness | 4010 | 12D3KooWPfrZ...zLVxJ | 20:07:55Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id...` |

**Topology change from pass 125:** RESTART EVENT.
- morning-api: killed (PID 3579452) and replaced by PID 3946739 (20:07:43Z)
- local-witness: new process started (PID 3947000 at 20:07:55Z) but **old process (PID 3579821) NOT killed** — both running simultaneously, bound to same UDS socket. This is a **stale process contamination** incident.

---

## RESTART EVENT — Observations

### Timeline (UTC)
| Time | Event |
|------|-------|
| ~20:07:40Z | First observer queries hit old session (m-ap uptime 25,464s, epoch 849, balance 20) |
| 20:07:43.561Z | morning-api: NTP failure at startup |
| 20:07:43.699Z | morning-api: Genesis recovered from WAL, Mint 5,000 applied |
| 20:07:43.709Z | morning-api: Epoch 1 complete (balance 5,020, ratio 1.00) |
| 20:07:55.672Z | local-witness: Lattice node starting (new process) |
| 20:07:55.730Z | local-witness: Genesis recovered from WAL, balance=0 |
| 20:07:55Z | **OLD witness (3579821) still running** — bound to same socket |
| ~20:08:16Z | Second observer query batch detects new session (uptime=32s, epoch=2) |
| ~20:08:44Z | Both nodes at epoch 3, m-ap balance=4,478 |
| 20:09:13Z | morning-api epoch 5: balance=4,047, witness epoch 5 (δ=0) |
| 20:09:56Z | morning-api epoch 6: balance=3,845, witness epoch 6 |
| ~20:10:18Z | morning-api epoch 7: balance=3,632, witness epoch 7 (δ=0) |

### Morning-api startup log
```
Genesis recovered from WAL network_name=lattice-morning-api root=12D3KooWPfr...
Recovered balances from economic snapshot count=2
Minting starting balance to local node amount=5000
```
Old persistence snapshot had: m-ap=20, witness=9,980. New mint adds 5,000 → m-ap=5,020. **Total supply: 15,000 DUU.**

### Witness startup log
```
Genesis recovered from WAL (same root as morning-api)
Recovered balances from economic snapshot count=1
Minting starting balance to local node amount=0
```
Witness recovered its own balance (0) from snapshot. Has no knowledge of morning-api's 5,020 balance.

### Stale Process Detection
```bash
fuser /tmp/local-witness/lattice.sock
# Output: /tmp/local-witness/lattice.sock: 3579821 3947000
```
Both PIDs are bound to the same UDS socket. Process 3579821 started at 13:02Z (~7h old) was never killed before the new process started.

### Witness Log Contamination
The witness log file (`/tmp/lw.log`) is written to by both processes simultaneously. File is 2.16MB with 206 `wc -l` lines (average ~10KB/line — binary interleaving). `grep` returns "binary file matches". `strings`-based queries required for analysis.

---

## Node Info (Post-Restart, ~20:10Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| m-ap peer_id | `12D3KooWPfrZ...zLVxJ` | Same identity (persistent key) | None |
| m-ap uptime_secs | 189 | ~190s (since 20:07:43Z) | None — matches clock |
| m-ap build_commit | `9f604ed-dirty` | Git HEAD `9f604ed` | Binary at HEAD (RESOLVED), but **-dirty** present |
| m-ap thickness | 975.93 | ~976 (slow decay) | None — consistent decay rate |
| witness peer_id | `12D3KooWDNNZm...9sch` | Same identity | None |
| witness uptime_secs | 184 | ~185s (since 20:07:55Z) | None — matches clock |
| witness build_commit | `9f604ed-dirty` | Git HEAD `9f604ed` | Same as m-ap |

**Build provenance:**
| Field | Pass 125 (19:58Z) | Pass 126 (20:10Z) | Δ |
|-------|-------------------|--------------------|-------|
| Git HEAD | `8c68a33` | **`9f604ed`** | +3 commits: `df5bc26`, `9f604ed` (docs + GetStateRoot API) |
| Build commit | `cb5d4b1-dirty` | **`9f604ed-dirty`** | **GAP RESOLVED** — binary now at HEAD |
| Behind HEAD | **6** | **0** | **Build gap closed** |
| -dirty | Present | Present | Still dirty |

**DEVIATION #1 (build gap): RESOLVED.** Binary now matches HEAD `9f604ed`. Still compiled from dirty working tree. First time build has been current since session started at pass 1 (13:01Z).

---

## Epoch State (Single Capture ~20:10Z)

### morning-api
| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | **7** | — |
| Log count (grep -c) | 7 | — |
| Last log epoch | 7 (20:10:12Z) | — |
| Three-way equality | **MATCH** — socket=7, count=7, last=7 | None |

### local-witness
| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | **7** | — |
| Log count (strings-based) | 12 | **DEVIATION** — log contaminated by dual-writer (old + new processes) |
| Last epoch | Unknown — log is binary file | — |
| Three-way equality | **UNKNOWN** — log corrupted by stale process | Contamination |

**Cross-node sync:** Both at epoch 7 (δ=0). Converged.

---

## Peer Connections (~20:10Z)

### morning-api
| Metric | OBSERVED | DEVIATION |
|--------|----------|-----------|
| Peers | 1 (`12D3KooWDNNZm...9sch`) | None |
| Heartbeats | 17 | Expected ~6/min for ~3min of peer connection |
| silence_secs | 2 | Healthy |
| queue_depth | 0 | None |
| is_dead | false | None |

### local-witness
| Metric | OBSERVED | DEVIATION |
|--------|----------|-----------|
| Peers | 1 (`12D3KooWPfrZ...zLVxJ`) | None |
| Heartbeats | 18 | Expected |
| silence_secs | 8 | Healthy |
| queue_depth | 0 | None |
| is_dead | false | None |

**Both nodes see 1 peer.** Bidirectional heartbeats flowing. No zombie evictions.

---

## Economic State (~20:10Z)

### morning-api (morning-api's view)
| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| own_balance | **3,632** | ~5,020 (mint) minus redistribution tax | Balance dropping ~200-300/epoch |
| own_nonce | 249 | Incrementing with redistribution txs | None |
| peer (witness) balance | **11,368** | Unknown (accumulating rejected tax) | **Persistent #3** |
| Total supply (m-ap books) | **15,000** | 5,000 (--mint 5000) | **WIDENED** — was 10,000 before restart |

### local-witness (witness's view)
| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| own_balance | **0** | 0 (--mint 0) | None |
| own_nonce | 6 | Incrementing | None |
| peer (m-ap) balance | **0** | ~3,632 | **Persistent #4** — witness sees m-ap as 0 |

### Redistribution Activity (New Session)
Epoch-by-epoch balance trajectory (morning-api):
| Epoch | balance_before | balance_after | ratio | Tax | Redistributed |
|-------|---------------|---------------|-------|-----|---------------|
| 1 | 5,020 | 5,020 | 1.00 | 251 (calculated, 0 collected — no peers yet) | 0 |
| 2 | 5,020 | 4,772 | 1.01 | 248 | 1 (witness) |
| 3 | 4,772 | 4,478 | 0.81 | 294 | 1 |
| 4 | 4,478 | 4,257 | 1.02 | 221 | 1 |
| 5 | 4,257 | 4,047 | 1.02 | 210 | 1 |
| 6 | 4,047 | 3,845 | — | — | — |
| 7 | 3,845 | 3,632 | 0.96 | 201 | 1 |

**Witness log (via strings):** 5 `insufficient balance` rejections — one per redistribution epoch (epochs 2-6). All redistribution transfers sent by morning-api are rejected by witness (witness balance=0, can't accept transfer).

**Supply distribution at epoch 7:**
- Morning-api books: m-ap=3,632, witness=11,368, total=15,000
- Witness books: witness=0, m-ap=0, total=0

**Supply conservation:** CONTRADICTED (same pattern as previous session; now with 15,000 total diverging from 5,000 minted).

---

## Persistence State (~20:10Z single capture)

### morning-api
| Field | OBSERVED | PASS 125 | Δ |
|-------|----------|----------|----|
| last_snapshot_epoch | **0** (fresh) | 830 | Reset — new process, no snapshot yet |
| wal_bytes | 3,437 | 379 | Growing — accumulating transactions |
| wal_entries | 28 | 3 | Growing — includes genesis + 7 epochs + redistribution |

**Byte-equality:** `GetPersistenceState.wal_bytes=3,437`. `ls -la wal.log=3,437` bytes. **PASS.**

**File inventory:**
| File | Size | mtime (UTC) | Notes |
|------|------|-------------|-------|
| `state.snapshot` | 895 bytes | 20:06Z | Old snapshot from previous session |
| `wal.log` | 3,437 bytes | 20:10Z | Active WAL (new session) |
| `wal.wal.old` | 379 bytes | 20:01Z | Pre-rotation from old session |
| `transactions.wal` | 0 bytes | 20:07Z | Legacy — empty (correct, unified WAL active) |

**Note:** `last_snapshot_epoch=0` while old `state.snapshot` (895 bytes, epoch 830 snapshot) still on disk. New process reads it for recovery but hasn't written its own snapshot yet. Expected — first snapshot at epoch 10.

### local-witness
| Field | OBSERVED | DEVIATION |
|-------|----------|-----------|
| last_snapshot_epoch | **0** (fresh) | Reset (same as m-ap) |
| wal_bytes | 813 | New WAL growing |
| wal_entries | 6 | Includes genesis re-seed |

**File inventory:**
| File | Size | mtime (UTC) | Notes |
|------|------|-------------|-------|
| `state.snapshot` | 569 bytes | 20:06Z | Old snapshot from previous session |
| `wal.log` | 813 bytes | 20:07Z | Active WAL (new session — short, few entries) |
| `wal.wal.old` | 379 bytes | 20:01Z | Pre-rotation from old session |
| `transactions.wal` | 0 bytes | 20:07Z | Legacy — empty |

---

## Metrics (Log Lines, ~20:10Z)

### morning-api
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=7s
```
All clean. Same pattern as old session.

### local-witness
Log is binary/contaminated by stale process. Metrics lines not reliably extractable. **New observation:** witness log monitoring is degraded until stale process is cleaned up.

---

## Log Health

### morning-api (/tmp/m-ap.log, clean, 7 epochs)
| Pattern | Count | Notes |
|---------|-------|-------|
| Epoch complete | 7 | All match socket epoch |
| WARN | 4 | ~1 NTP failure + ~3 KAD bootstrap WARNs (expected with --no-mdns, every 5 min) |
| ERROR | **0** | None |
| NTP failures | **1** | At startup (20:07:43Z): `pool.ntp.org failed: Resource temporarily unavailable (os error 11)` |
| "insufficient" | 0 | None on morning-api |
| Zombie/sweep/eviction | 0 | None |
| Snapshot saved | 0 | First snapshot expected at epoch 10 |

### local-witness (/tmp/lw.log, contaminated by stale process)
| Pattern | Count | Method | Notes |
|---------|-------|--------|-------|
| Epoch complete | 12 | strings-based | 7 from new + 5 residual from old process's truncated log |
| insufficient balance | **5** | strings-based | All from new session (epochs 2-6) |
| NTP failures | 0 visible | strings-based | New process startup had 0 NTP failures |
| ERROR | Unknown | — | Log not reliably parseable |
| Zombie/sweep | 0 | strings-based | None |

**OBSERVED:** Witness log monitoring capability is degraded. Stale process cleanup required.

---

## NTP Failure Series — Updated Trend

| Pass | Capture Time (Z) | New Failures | Cumulative | Trailing 6-Pass Rate | Trend |
|------|------------------|-------------|------------|----------------------|-------|
| 121 | ~19:13Z | 0 | 3 | 33% (2/6) | Quiescent |
| 122 | ~19:31Z | 0 | 3 | 33% (2/6) | Quiescent |
| 123 | ~19:41Z | 0 | 3 | 33% (2/6) | Quiescent |
| 124 | ~19:50Z | 0 | 3 m-ap + 1 lw = 4 | 33% (2/6) | Quiescent |
| 125 | ~19:58Z | 0 | 4 total | 17% (1/6) | Quiescent — pattern broke |
| **126** | **~20:10Z** | **1 (m-ap startup)** | **5 total** | **17% (1/6)** | **Session restart reset NTP state** |

**OBSERVED:** Pass 126 has 1 new NTP failure (at morning-api startup, 20:07:43Z). The old session's NTP failure series is superseded by the restart — both nodes have fresh UDS and fresh NTP state. The restart changes the baseline: the ~56-min failure cadence observed in the old session may or may not apply to the new session. The new session is only ~2 min old; no conclusions can be drawn yet.

**Trailing 6-pass rate:** 17% (1/6) — unchanged from pass 125. The single failure is the startup event. If no further failures occur, the rate will drop to 0% as this pass ages out of the window.

---

## Evidence Guards

| Guard | Status |
|-------|--------|
| Three-way epoch (m-ap) | **PASS** — socket=7, count=7, last=7 (clean) |
| Three-way epoch (witness) | **DEGRADED** — socket=7, but log is binary file from stale process contamination |
| Byte-equality (m-ap) | **PASS** — 3,437=3,437 |
| Byte-equality (witness) | **PASS** — 813=813 |
| PID consistency | **SESSION RESET** — m-ap PID changed (3579452→3946739), witness has dual PIDs (3579821+3947000) |
| Log health (m-ap) | **PASS** — 4 WARNs, 0 ERRORs, 0 zombie evictions |
| Log health (witness) | **DEGRADED** — log corrupted by stale process |
| Metrics health (m-ap) | **PASS** — aged=0, queues=[], silence<10s |
| Metrics health (witness) | **UNKNOWN** — log corrupted |
| Cross-node epoch sync | **PASS** — both at epoch 7 (δ=0) |
| Snapshot rotation | **NOT YET** — session too new (first snapshot at epoch 10 expected ~2.5 min from now) |
| Clock sync | **PASS** — NTP synchronized, drift 0s at startup |

---

## Persistent Deviations — Status

| # | Deviation | First Observed | Status | Delta from Pass 125 |
|---|-----------|----------------|--------|---------------------|
| 1 | `build_commit` stale (`cb5d4b1-dirty`, 6 behind HEAD) | Jul 27 pass 1 | **RESOLVED** — binary now at HEAD `9f604ed-dirty` (0 behind, dirty) | **FIXED.** Build gap closed. Dirty remains. |
| 2 | `wal_bytes` returning 0 (legacy path) | Jul 27 pass 1 | **RESOLVED** (since pass 98) | Unchanged |
| 3 | Supply divergence (total=15,000 vs 5,000 minted) | Pass 97 (14:08Z) | **WIDENED** — was 10,000, now 15,000 | New mint layer (5,000) stacked on old 10,000 divergence. |
| 4 | Witness reports morning-api balance as 0 | Jul 27 (~18:48Z) | Persistent (0 vs ~3,632 actual at epoch 7) | Same pattern, new numbers |
| 5 | Epoch ratio divergence (~11% gap) | Pass 1 (18:06Z) | Persistent — witness ratio ~1.95, m-ap ~0.96-1.02 | Same structural gap (witness has 0 balance, ratio is meaningless) |
| 6 | MESH.md stale ("No production nodes running") | Jul 27 | **SUPERSEDED** — nodes running (just restarted) | Update MESH.md with new topology |

**NEW DEVIATION (Pass 126):**
| # | Deviation | First Observed | Status | Notes |
|---|-----------|----------------|--------|-------|
| 7 | Witness stale process contamination (PID 3579821 not killed) | Pass 126 (20:10Z) | **NEW** | Old witness still bound to socket alongside new process. Witness log corrupted by dual-writer interleaving. Witness monitoring degraded. |
| 8 | Witness log corrupted (binary/garbled) | Pass 126 (20:10Z) | **NEW** | Consequence of #7. `grep` returns "binary file matches". Strings-based queries required. |

---

## Build Provenance — GAP RESOLVED

| Field | Pass 125 (19:58Z) | Pass 126 (20:10Z) |
|-------|-------------------|--------------------|
| Git HEAD | `8c68a33` (protocol invariants doc) | `9f604ed` (GetStateRoot API endpoint) |
| Build commit | `cb5d4b1-dirty` | `9f604ed-dirty` |
| Behind HEAD | 6 | **0** |
| -dirty | Present | Present |

**First time since pass 1** that the running binary matches git HEAD. The new commit `9f604ed` adds a `GetStateRoot` API endpoint. The dirty suffix persists — uncommitted changes in the working tree at build time.

---

## Summary

**Pass 126: RESTART EVENT. Build gap resolved. Supply divergence widened. Stale process contamination detected.**

### What changed since pass 125

The mesh was completely restarted during this observation pass (~20:07Z). Key changes:

- **RESTART EVENT.** Both nodes killed and restarted with new binary. Old session had 7h 6min uptime (849 epochs). New session is ~2 min old.
- **Build gap RESOLVED** — binary now at HEAD `9f604ed-dirty` (was 6 behind `cb5d4b1-dirty`). First time current since session start. Dirty remains.
- **Supply divergence WIDENED** — total supply now 15,000 DUU (was 10,000). The new `--mint 5000` was applied on top of the old persistence snapshot (which already had 10,000 on morning-api's books from the previous mint-only divergence). Morning-api has 3,632, witness has 11,368 on morning-api's books (accumulating rejected redistribution). Witness sees morning-api as 0.
- **Redistribution active** — morning-api losing ~200-300 DUU/epoch to tax, witness rejecting all transfers (balance=0). 5 insufficient-balance rejections in the new session.
- **Stale process contamination (NEW)** — old witness (PID 3579821, running since 13:02Z) was NOT killed before new witness (PID 3947000) started. Both bound to same UDS socket. Witness log corrupted by dual-writer interleaving (2.16MB binary file).
- **Witness log monitoring degraded** — `grep` returns "binary file matches" on /tmp/lw.log. Must use `strings` for queries until stale process is cleaned up.
- **NTP** — 1 startup failure on morning-api (20:07:43Z). New session too short to establish pattern.
- **Metrics** — clean on morning-api. Witness metrics unreadable.

### Active deviations (5 remaining + 2 new)
1. ✅ **RESOLVED** — build gap (binary now at HEAD)
2. ✅ **RESOLVED** — wal_bytes (fixed since pass 98)
3. ❌ **WIDENED** — supply divergence (now 15,000 vs 5,000 minted)
4. ❌ **Unchanged** — witness sees morning-api balance as 0
5. ❌ **Unchanged** — epoch ratio divergence (~11% gap)
6. ❌ **Superseded** — MESH.md topology stale
7. 🆕 **NEW** — stale witness process contamination
8. 🆕 **NEW** — witness log corrupted (consequence of #7)

### Recommendations for next action
- Kill old witness process (PID 3579821) to restore clean log and socket
- Update MESH.md with new topology (fresh session, new build)
- Verify snapshot rotation at epoch 10 (~20:12Z)
