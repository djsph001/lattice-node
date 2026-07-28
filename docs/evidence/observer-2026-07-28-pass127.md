# Observer Evidence Record — 2026-07-28 (Pass 127)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** ~2026-07-28T20:18–20:20Z
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** 127th observation pass. ~10 min since pass 126 (20:10Z).

**Summary:** Post-restart session continues normally. Epochs progressing (23 on m-ap). Snapshot fired at epoch 20 as expected. Build gap REAPPEARED — binary at `9f604ed-dirty` now 2 commits behind new HEAD `475607a`. All 7 persistent deviations unchanged. 1 new observation: witness strings reveal zombie eviction events occurred during the ~1-min gap when morning-api was down at restart. 1 provisional deviation: cross-node epoch state shows δ=1 (m-ap 23, witness 24) — likely a boundary race (~30s between queries). Metrics clean on m-ap. No new NTP failures.

---

## Topology Disclosure

**This machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, 100.93.232.107 Tailscale). Boynton Beach FL. Ubuntu 24.04.

| PID | Name | Port | Genesis Root | Since (UTC) | Since (elapsed) | State |
|-----|------|------|--------------|-------------|-----------------|-------|
| 3946739 | morning-api | 4005 | auto (12D3KooWPfrZ...zLVxJ) | 20:07:43Z | ~12 min | Running |
| **3579821 (STALE)** | local-witness | 4010 | 12D3KooWPfrZ...zLVxJ | **13:02Z** | **~7h 18min** | **Running — should have been killed** |
| 3947000 | local-witness | 4010 | 12D3KooWPfrZ...zLVxJ | 20:07:55Z | ~12 min | Running |

**Topology change from pass 126:** None. Stale witness (PID 3579821) was NOT cleaned up.

---

## Node Info

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| m-ap peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Same identity (persistent key) | None |
| m-ap uptime_secs | 622 (~20:17Z) | ~630s (since 20:07:43Z) | None — matches clock |
| m-ap build_commit | `9f604ed-dirty` | Git HEAD `475607a` | **DEVIATION — 2 behind HEAD** |
| m-ap thickness | 975.82 | ~976 (slow decay) | None — consistent decay |
| witness peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Same identity | None |
| witness uptime_secs | 709 (~20:20Z) | ~725s (since 20:07:55Z) | None — matches clock |
| witness build_commit | `9f604ed-dirty` | Git HEAD `475607a` | **DEVIATION — same binary** |

### Build Provenance — GAP REAPPEARED

| Field | Pass 126 (20:10Z) | Pass 127 (20:19Z) | Δ |
|-------|-------------------|--------------------|-------|
| Git HEAD | `9f604ed` | **`475607a`** | +2 commits: `8bf05b3`, `475607a` (both docs commits) |
| Build commit | `9f604ed-dirty` | `9f604ed-dirty` | Unchanged |
| Behind HEAD | **0** | **2** | **GAP REAPPEARED** — binary 2 behind new HEAD |
| -dirty | Present | Present | Unchanged |

**OBSERVED:** The running binary (`9f604ed-dirty`) was at HEAD in pass 126. Two new doc commits (`8bf05b3`, `475607a`) landed between capture time 20:10Z and 20:19Z. The binary is now 2 commits behind.

**DEVIATION #1 (build gap): REAPPEARED.** Was RESOLVED in pass 126 (20:10Z). Now 2 behind again. This is the second occurrence of this deviation — first observed Jul 27 pass 1, resolved at pass 126, now recurred at pass 127.

---

## Epoch State (Single Capture ~20:18Z)

### morning-api

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | **23** | — |
| Log count (grep -c) | 23 | — |
| Last log epoch | 23 (20:18:43Z) | — |
| Three-way equality | **MATCH** — socket=23, count=23, last=23 | None |

### local-witness

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | **24** | **δ=1 vs m-ap** (see below) |
| Log count (strings-based) | 48 | Contaminated by dual-writer (old + new process logs) |
| Last log epoch | Incomplete — log is binary file | Contamination |
| Three-way equality | **UNKNOWN** — log corrupted | Contamination |

**Cross-node sync:** m-ap=23, witness=24 (δ=1 at ~20:19–20:20Z). This is likely a boundary race: m-ap was captured ~30-60s before witness, and an epoch transition occurred between queries. At ~30s/epoch cadence, a 1-epoch gap from sequential queries is expected. Not classified as a persistent divergence unless repeat observations show a persistent gap aligned on the same query order.

---

## Peer Connections (~20:19Z)

### morning-api

| Metric | OBSERVED | DEVIATION |
|--------|----------|-----------|
| Peers | 1 (`12D3KooWDNNZm...9sch`) | None |
| Heartbeats | 61 | ~1/10s for 622s uptime — plausible |
| silence_secs | 9 | Healthy |
| queue_depth | 0 | None |
| is_dead | false | None |

### local-witness

| Metric | OBSERVED | DEVIATION |
|--------|----------|-----------|
| Peers | 1 (`12D3KooWPfrZ...zLVxJ`) | None |
| Uptime | 709s | Matches new session start |
| is_dead | false | None |

**Both nodes see 1 peer.** Bidirectional peer table entries exist.

---

## Economic State (~20:19Z)

### morning-api (morning-api's books)

| Metric | OBSERVED | vs Pass 126 (epoch 7) | DEVIATION |
|--------|----------|----------------------|-----------|
| own_balance | **1,674** | was 3,632 (Δ=-1,958) | Dropping ~122/epoch via redistribution tax |
| own_nonce | 264 | was 249 (Δ=+15) | Incrementing |
| peer (witness) balance | **13,326** | was 11,368 (Δ=+1,958) | Witness accumulating rejected redistribution |
| Total supply (m-ap books) | **15,000** | was 15,000 | **UNCHANGED** — 15,000 vs 5,000 minted |

### local-witness (witness's books)

| Metric | OBSERVED | vs Pass 126 | DEVIATION |
|--------|----------|-------------|-----------|
| own_balance | **0** | 0 | None (--mint 0) |
| own_nonce | **6** | 6 | **Frozen** — unchanged since pass 126 (was 6 at epoch 7, now 24 epochs in) |
| peer (m-ap) balance | **0** | 0 | **DEVIATION #4** — witness sees m-ap as 0 (actual ~1,674) |

**OBSERVED:** Witness nonce (6) is **frozen** since pass 126 (epoch 7). 16 epochs later, still at 6. Every epoch, morning-api sends a redistribution transfer that witness rejects with "insufficient balance." The rejection does NOT increment the witness's nonce. Verified against the nonce table in VERIFIED-BEHAVIOR.md: "nonce asymmetry across peers" is a known C-classified mechanism limitation.

**Supply distribution at epoch 23 (m-ap):**
- Morning-api books: m-ap=1,674, witness=13,326, total=15,000
- Witness books: witness=0, m-ap=0, total=0

**Supply conservation:** CONTRADICTED (same pattern, same magnitude since pass 126). 15,000 total in m-ap books vs 5,000 minted. Divergence total (10,000) unchanged.

---

## Persistence State (Single Capture ~20:19Z)

### morning-api

| Field | OBSERVED | Pass 126 (epoch 7) | Δ |
|-------|----------|--------------------|----|
| last_snapshot_epoch | **20** | 0 | **Snapshot fired** at epoch 20 ✓ |
| wal_bytes | 1,255 | 3,437 | **-2,182** (WAL shrank after snapshot + rotation) |
| wal_entries | 10 | 28 | **-18** (entries reset after rotation, Genesis re-seed + 9 post-snapshot) |

**Byte-equality:** `GetPersistenceState.wal_bytes=1,255`. `ls -la wal.log=1,255` bytes. **PASS.**

| File | Size | mtime (UTC) | Notes |
|------|------|-------------|-------|
| `state.snapshot` | 899 bytes | 20:17Z | Fresh snapshot at epoch 20 |
| `wal.log` | 1,255 bytes | 20:18Z | Active WAL (post-rotation, growing) |
| `wal.wal.old` | 3,437 bytes | 20:17Z | Pre-rotation WAL from pre-snapshot session |

**Snapshot rotation verified.** `state.snapshot` mtime is 20:17Z, matching epoch 20 timing (~5 min before epoch 21 at 20:18:43Z). `wal.wal.old` is the rotated WAL (3,437 bytes = previous wal_bytes from pass 126).

### local-witness

| Field | OBSERVED | Pass 126 | Δ |
|-------|----------|----------|----|
| last_snapshot_epoch | **20** | 0 | Same as m-ap — both snapshotted at epoch 20 |
| wal_bytes | 379 | 813 | **-434** (post-rotation) |
| wal_entries | 3 | 6 | **-3** (post-rotation) |

**Byte-equality:** `GetPersistenceState.wal_bytes=379`. `ls -la wal.log=379` bytes. **PASS.**

| File | Size | mtime (UTC) | Notes |
|------|------|-------------|-------|
| `state.snapshot` | 569 bytes | 20:17Z | Fresh snapshot at epoch 20 |
| `wal.log` | 379 bytes | 20:17Z | Active WAL (post-rotation) |
| `wal.wal.old` | 379 bytes | 20:16Z | Pre-rotation WAL |

**OBSERVED:** Both nodes fired snapshot at epoch 20 (expected — first at 10, then every 10). WAL rotated cleanly. Genesis re-seeded.

---

## State Root

| Node | State Root | Epoch |
|------|-----------|-------|
| morning-api | `00520feb4414d41b262bc32750c91ddc2e62b61c2d238a7aa8c1ace034644c3c` | 23 |
| local-witness | `4bd573e62bc2531b833c6faa1d5a91c64c64fa2b2d3ec219810160a176ad5654` | 24 |

**OBSERVED:** Roots differ. Epochs also differ by 1. Per VERIFIED-BEHAVIOR.md Mission A finding: `state_root` is classified **C** — structurally incapable of cross-node convergence detection due to nonce asymmetry. Divergent roots are expected even when balances match. No diagnostic value here.

---

## Metrics (Log Lines, ~20:19Z)

### morning-api (clean, every 10s)
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=7s
```
All clean. Same pattern since session start.

### local-witness (degraded — dual-writer log)
Log is binary/contaminated by stale process (PID 3579821). Metrics lines not reliably extractable.

**Strings-based findings from witness log:**
| Pattern | Count | Notes |
|---------|-------|-------|
| "Epoch complete" | 48 | Includes old process entries; unreliable count |
| "insufficient balance" | **24** | One per epoch in new session — correct |
| "approaching zombie threshold" | Multiple | From old process (see below) |
| "Evicting zombie peer" | **1** | From old process during morning-api restart gap |
| "Failed to reconnect after zombie eviction" | **1** | From old process |
| " ERROR" | **0** | None found (strings-based) |
| NTP mentions | **1** | Likely startup failure |

**ZOMBIE EVICTION (WITNESS):** Strings analysis reveals `Evicting zombie peer` and `Failed to reconnect after zombie eviction` in the contaminated witness log. These events occurred during the ~1-min window (20:07:43–20:07:55Z) when morning-api was down for restart and the old witness (PID 3579821) lost its only peer. Timeline:
- 20:07:43Z — morning-api killed
- 20:07:43–20:07:55Z — old witness has no peer; silence timer started
- ~20:09:13Z — zombie threshold (90s) would fire, evicting the dead peer entry
- 20:07:55Z or later — new witness/morning-api start; old witness attempts reconnect but fails (new witness now owns the socket)

This is an observation, not a diagnosis. The zombie eviction fired as designed — a peer went silent for >90s and was evicted. The cleanup action from pass 126 (kill old witness) was not performed, so the contamination and degraded monitoring persist.

---

## Log Health

### morning-api (/tmp/m-ap.log, 23 epochs, clean)

| Pattern | Count | Notes |
|---------|-------|-------|
| Epoch complete | 23 | All match socket epoch |
| WARN | 4 | 1 NTP startup failure + 3 KAD bootstrap (expected every 5 min with --no-mdns) |
| ERROR | **0** | None |
| NTP failures | 1 | At startup (20:07:43Z) — same as pass 126; no NEW failures this pass |
| "insufficient" | 0 | None on morning-api |
| Zombie/sweep/eviction | **0** | None |
| Snapshot saved | 1 | At epoch 20 |
| stale fetch sweep | 0 | None |

### local-witness (/tmp/lw.log, contaminated by dual-writer — strings-based)

| Pattern | Count | Notes |
|---------|-------|-------|
| "insufficient balance" | 24 | One per epoch in new session (epochs 1-23 ≈ 24 responses expected) |
| Zombie eviction | Yes | From old process during morning-api restart gap |
| ERROR | 0 revealed | Strings-based — may miss some |
| Metrics | Unreadable | Log corrupted |

---

## NTP Failure Series

### New Session (since pass 126 restart)

| Pass | Capture Time (Z) | New Failures | Cumulative | Trailing 6-Pass Rate | Trend |
|------|------------------|-------------|------------|----------------------|-------|
| **126** | ~20:10Z | **1** (m-ap startup) | 1 | — (window too short) | Baseline |
| **127** | **~20:19Z** | **0** | **1** | — (only 2 data points) | **Quiescent** |

**OBSERVED:** No new NTP failures this pass. The single failure from pass 126 (morning-api startup at 20:07:43Z) remains the only event in the new session. The trailing 6-pass window is too short (2 passes) for rate calculation. If no failures in the next 4 passes, the rate will be 17% (1/6) — same as pass 126's trailing rate on the old session.

**System NTP status:** Clock synchronized, NTP service active. Per MESH.md clock sanity precondition, the startup failure (`os error 11` — transient resource) was non-fatal because the fallback clock check (`drift 0s`) passed.

---

## Evidence Guards

| Guard | Status |
|-------|--------|
| Three-way epoch (m-ap) | **PASS** — socket=23, count=23, last=23 |
| Three-way epoch (witness) | **DEGRADED** — log contaminated by stale process dual-writer |
| Byte-equality (m-ap) | **PASS** — 1,255=1,255 |
| Byte-equality (witness) | **PASS** — 379=379 |
| Snapshot rotation | **PASS** — both nodes fired snapshot at epoch 20. WAL rotated. |
| Cross-node epoch sync | **PROVISIONAL** — m-ap 23, witness 24 (δ=1, likely boundary race — sequential queries ~30-60s apart) |
| Log health (m-ap) | **PASS** — 4 WARNs (1 NTP + 3 KAD), 0 ERRORs, 0 zombie evictions |
| Log health (witness) | **DEGRADED** — log corrupted by stale process dual-writer |
| Metrics health (m-ap) | **PASS** — aged=0, queues=[], silence<10s |
| Metrics health (witness) | **UNKNOWN** — log corrupted |
| Zombie eviction (m-ap) | **PASS** — none |
| Zombie eviction (witness) | **OBSERVED** — old process evicted peer during restart gap (expected behavior) |
| Clock sync | **PASS** — NTP synchronized, drift 0s at startup |
| State root divergence | **OBSERVED** — roots differ (expected per VERIFIED-BEHAVIOR.md C classification) |

---

## Persistent Deviations — Status

| # | Deviation | First Observed | Since Pass 126 (20:10Z) | Current Status |
|---|-----------|----------------|------------------------|----------------|
| 1 | Build gap — binary behind HEAD | Jul 27 pass 1 | **REAPPEARED** (was RESOLVED at pass 126). Now 2 behind (binary `9f604ed`, HEAD `475607a`). Dirty remains. | **REAPPEARED** |
| 2 | `wal_bytes` returning 0 (legacy path) | Jul 27 pass 1 | **RESOLVED** (since pass 98) | **RESOLVED** ✓ |
| 3 | Supply divergence — total 15,000 vs 5,000 minted | Pass 97 (14:08Z) | Unchanged (still 15,000 total on m-ap books, m-ap=1,674, witness=13,326) | **PERSISTENT** |
| 4 | Witness reports morning-api balance as 0 | Jul 27 (~18:48Z) | Unchanged (witness sees 0, actual ~1,674) | **PERSISTENT** |
| 5 | Epoch ratio divergence | Pass 1 (18:06Z) | Unchanged (m-ap ~1.00, witness ~1.95 — meaningless on 0-balance witness) | **PERSISTENT** |
| 6 | MESH.md topology stale | Jul 27 | Superseded by restart event pass 126 | **SUPERSEDED** |
| 7 | Stale witness process (PID 3579821) | Pass 126 (20:10Z) | **NOT CLEANED UP** — PID 3579821 still running (7h 18min uptime) | **PERSISTENT** |
| 8 | Witness log corrupted (dual-writer) | Pass 126 (20:10Z) | Unchanged — log still binary file from dual-process writing | **PERSISTENT** |

### Deviations table

| # | Deviation | Status | Since | Notes |
|---|-----------|--------|-------|-------|
| 1 | Build gap | **REAPPEARED** | Pass 127 | Was fixed, now 2 behind again. Binary `9f604ed-dirty`, HEAD `475607a` |
| 2 | wal_bytes | RESOLVED | Pass 98 | — |
| 3 | Supply divergence (15K vs 5K) | PERSISTENT | Pass 97 | Unchanged magnitude |
| 4 | Witness sees m-ap=0 | PERSISTENT | Jul 27 | Unchanged |
| 5 | Ratio divergence | PERSISTENT | Pass 1 | Structural (0-balance witness) |
| 6 | MESH.md stale | SUPERSEDED | — | Covered by topology disclosure |
| 7 | Stale witness process | **PERSISTENT** | Pass 126 | **Not cleaned up.** PID 3579821 still running |
| 8 | Witness log corrupted | **PERSISTENT** | Pass 126 | **Consequence of #7** |

### New/recurred this pass: None.

Observation: Zombie eviction events found in witness log (strings-based) are from the old process during the restart gap. These are expected consequences of the stale process contamination. The morning-api process shows no zombie or eviction activity.

---

## Summary

**Pass 127: Post-restart session stable. Build gap REAPPEARED. Stale witness process NOT cleaned up.**

### What changed since pass 126 (~10 min ago)

| Area | Change |
|------|--------|
| Epoch progress | 7→23 (m-ap), 7→24 (witness). 16 epochs in ~9 min — correct cadence |
| Snapshot rotation | **FIRED** at epoch 20 on both nodes. WAL rotated. Byte-equality verified. |
| Build gap | **REAPPEARED** — binary at `9f604ed-dirty` now 2 behind new HEAD `475607a` |
| Stale witness (PID 3579821) | **STILL RUNNING** — not cleaned up. Witness log monitoring still degraded. |
| Supply divergence | UNCHANGED — 15,000 total vs 5,000 minted (same 10,000 gap) |
| Metrics (m-ap) | Clean — aged=0, queues=[], silence=7s |
| Zombie evictions (m-ap) | None |
| Zombie evictions (witness) | Found in strings — from old process during restart gap (expected) |
| NTP | 0 new failures. Single startup failure (pass 126) remains the only event. |
| State root | Divergent (expected per C classification) |
| Witness nonce | Frozen at 6 since pass 126 (epoch 7). 16 epochs with no increment — rejection path doesn't advance nonce. |

### Active deviations (5 persistent + 2 structural)

1. ✅ Build gap — **REAPPEARED** (was resolved at pass 126, now 2 behind again)
2. ✅ wal_bytes — RESOLVED
3. ❌ Supply divergence — PERSISTENT (15K vs 5K)
4. ❌ Witness sees m-ap=0 — PERSISTENT
5. ❌ Ratio divergence — PERSISTENT (structural)
6. ❌ MESH.md stale — SUPERSEDED
7. ❌ Stale witness process — **PERSISTENT (NO CHANGE)**
8. ❌ Witness log corrupted — PERSISTENT (consequence of #7)

### Observation for next action

The stale witness process (PID 3579821, running 7h 18min) is the primary operational issue. It degrades witness log monitoring and may interfere with socket operations. Kill with `pkill -9 -f "lattice-node --name local-witness"` (scoped by name flag) to restore clean monitoring. The remaining deviations (supply, balance visibility, ratio) are structural and preceded the stale-process contamination.
