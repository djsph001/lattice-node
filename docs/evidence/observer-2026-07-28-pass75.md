# Observer Evidence Record — 2026-07-28 (Pass 75)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T09:48Z bundle (socket queries)
**Log/metrics capture:** ~2026-07-28T09:48Z
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Seventy-fifth observation pass. Same processes since 2026-07-27T18:48Z (~15.0h runtime). ~9 min since pass 74 (09:39Z).

**Summary:** Routine continuation. Both nodes cycling normally. Epoch 1799 (+17 from pass 74). One snapshot rotation on each node (1780→1790). The runtime NTP check failure observed in pass 74 (09:40:16Z) did NOT recur — appears to have been transient. No new WARN/ERROR events. All three persistent deviations unchanged.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 2026-07-27T18:48Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 2026-07-27T18:48Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs (2727391, 2727569). Both sockets responding.

---

## Node Info

### morning-api (~09:48Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 53952 (~15.0h) | — | None (pass 74: 53429; Δ = +523s ≈ 8.7 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 985.671 | ~1000, slowly decaying | None (pass 74: 985.813; Δ = −0.142 over ~9 min — consistent decay ~0.016/min) |

### local-witness (~09:48Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 53957 (~15.0h) | — | None (pass 74: 53463; Δ = +494s ≈ 8.2 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api (~09:48Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=5393, silence_secs=8, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~09:48Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=5397, silence_secs=0, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 74 (~09:39Z):** Heartbeats api +51 (5342→5393 ~5.7/min), witness +50 (5347→5397 ~5.6/min). Both at expected rate. Silence: api 8s (pass 74: 1s — normal variation), witness 0s (pass 74: 9s). Queue depth 0 on both.

**No zombie eviction events** detected in either log.

---

## Epoch State

### morning-api (~09:48Z socket capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1799 (endpoint) | Cycling. +17 from pass 74 (1782→1799) in ~9 min. | None. Normal cadence (~32s/epoch). |
| ratio | 1.019980 | ~1.01–1.02 steady state | None (pass 74: 1.019789; tiny drift +0.00019) |
| tax_calculated | 0 | Balance 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (~09:48Z socket capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1799 (endpoint) | Cycling. +16 from pass 74 (1783→1799) in ~9 min. | None. Normal cadence. Bundle capture sequential. |
| ratio | 1.056 | Continuing asymptotic decline | None (pass 74: 1.056325; Δ = −0.00036 over ~8 min — continued approach to 1.0) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** Both at 1799 in this bundle (pass 74 showed m-ap=1782, witness=1783 with Δ=1). Now synchronized. Normal — happens when capture timing aligns across an epoch boundary.

### Three-way epoch check (single-capture — sequential, not simultaneous)
- **morning-api:** Socket=1799 (~09:48Z), grep count=1800, last_log epoch=1800 (09:47:56Z). Δ=1 between endpoint and log — 1 epoch elapsed between socket query and log grep within the capture window. Normal race.
- **local-witness:** Socket=1799 (~09:48Z), grep count=1799, last_log epoch=1799 (09:47:43Z). Consistent (no elapsed epoch between queries).

---

## Economic State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged) |
| own_nonce | 120 | 120 | None (unchanged) |
| witness_balance (reported) | 4980 | 5000 - morning_api_balance = 4980 | None (mesh consensus on peer balance) |
| witness_nonce (reported) | 0 | 0 | None |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 | None |
| own_nonce | 2 | 2 | None (unchanged since pass 64) |
| morning_api_balance (reported) | 0 | 5000 | **Persistent DEVIATION.** Witness reports morning-api balance as 0. First observed: observer pass 1 (Jul 27 18:48Z). Supply conservation: CONTRADICTED per Verifier Mission 1. |

### Supply divergence
**OBSERVED:** morning-api sees total supply = 20 + 4980 = 5000. Witness sees total supply = 0 + 0 = 0.
**DEVIATION:** Witness-side accounting reports 0. Known since first observer pass. Unchanged.

---

## Persistence State

### morning-api (~09:48Z socket + filesystem, sequential capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1790 | Incrementing by 10 (pass 74: 1780; +10 = 1 rotation) | None (normal — 1 rotation since pass 74) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (~09:48Z):**
- `state.snapshot`: **895 bytes** (mtime: 2026-07-28T05:47 EDT — epoch 1790 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T05:47 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T05:42 EDT — previous epoch 1780 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 05:37 (pass 74) to 05:47 — confirmed 1 rotation (1780→1790). ✓
- Snapshot size **895 bytes** (pass 74: 894; back to 895 after one-pass regression). The ±1 byte oscillation continues: pass 71=894, pass 72-73=895, pass 74=894, pass 75=895. UNKNOWN: cause. Consistent with serialization of a threshold/boundary value.

### local-witness (~09:48Z socket + filesystem, sequential capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1790 | Incrementing by 10 (pass 74: 1780; +10 = 1 rotation) | None (normal — 1 rotation since pass 74) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~09:48Z):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T05:43 EDT — epoch 1790 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T05:43 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T05:38 EDT — previous epoch 1780 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 05:38 (pass 74) to 05:43 — confirmed 1 rotation. ✓
- Snapshot size 569 bytes (pass 74: 569; unchanged) ✓

---

## Metrics Instrumentation

**OBSERVED (from m-ap metrics line at ~09:48Z):**
- `outstanding_fetches=0` — no pending fetches
- `aged=0` — no stale fetches
- `outbound_queues=[]` — all peer queues empty
- `max_peer_silence=3s` — well under 30s threshold

**OBSERVED (from witness metrics line at ~09:48Z):**
- `outstanding_fetches=0`
- `aged=0`
- `outbound_queues=[]`
- `max_peer_silence=6s`

**EXPECTED:** All gauges near zero on a settled 2-node mesh with no new transactions.
**DEVIATION:** None. Mesh is quiescent.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **WARN (191 total, +1 since pass 74):** The +1 is the NTP runtime check failure at 09:40:16Z (pass 74 was captured slightly before that log line was written). **No new WARNs since 09:40:16Z.** The single NTP failure from pass 74 (09:40:16Z) did NOT recur in the ~8 min window between pass 74 and pass 75 — appears to have been transient.
- **ERROR:** 0 (unchanged).
- **Zombie eviction events:** None.
- **Sweep/eviction events:** None.
- **Panics:** None.
- **NTP runtime failures since pass 74:** 0. (The single occurrence from pass 74 was transient.)

### local-witness (/tmp/lw.log)
- **WARN (123 total, unchanged since pass 69):** All historic startup artifacts. The witness has never produced a runtime NTP check failure.
- **ERROR:** 0 (unchanged).
- **Insufficient balance:** 118 (unchanged, all historic Jul 27). No new occurrences.
- **NTP runtime failures:** 0 total (never observed).

### Log filter (WARN/ERROR excluded as benign)

| Pattern | m-ap | lw | Status |
|---------|------|----|--------|
| `skip-ntp-check` | 0 | 0 | Clean |
| `No snapshot` | 0 | 3 (startup) | Clean |
| `zombie` | 0 | 0 | Clean |
| `insufficient balance` | 0 | 118 (historic) | No new occurrences |
| `panicked` | 0 | 0 | Clean |

---

## Build Commit Verification

| Check | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| git HEAD | `cb5d4b1` | — | — |
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind + dirty (64 unstaged files). Unchanged since pass 1. |

---

## New Observation: Runtime NTP Failure Did NOT Recur

| Field | Detail |
|-------|--------|
| **OBSERVED** | The runtime NTP check failure from pass 74 (2026-07-28T09:40:16Z) did NOT recur in the ~8 min window between pass 74 and pass 75. morning-api continues cycling normally. Witness has never produced a runtime NTP failure. |
| **EXPECTED** | Not documented. No runtime NTP check cadence is specified in VERIFIED-BEHAVIOR.md or MESH.md. |
| **CLASSIFICATION** | The episode from pass 74 appears to have been a transient DNS failure (3 back-to-back `Temporary failure in name resolution` errors, one per NTP server, at the same timestamp). The runtime NTP check cadence is unknown — may be on a multi-minute timer, not every tick. |
| **NOTE** | This is an observation record, not a diagnosis. Causal claim ("transient DNS failure") is stated as a classification, not a causal explanation of root cause. The OBSERVED facts: (1) it happened once at 09:40:16Z, (2) it has not happened again in ~8 min, (3) the witness — running the same binary — has never produced one. |

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 74 Status | Pass 75 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

**No new deviations detected in this pass.** No new observations beyond the NTP runtime failure non-recurrence (classified under pass 74 observations).

---

## Minor Observations (Not Deviations)

| Observation | First Noted | Status |
|-------------|------------|--------|
| morning-api snapshot size back at 895 bytes (was 894 in pass 74, 895 in passes 72-73, 894 in pass 71) | Pass 71 | Fluctuating 894↔895. UNKNOWN: cause of ±1 byte oscillation. Possibly serialization of a threshold/boundary value. |
| Runtime NTP check failure (pass 74 at 09:40:16Z) did NOT recur in ~8 min window | Pass 74 | Single occurrence, no repeat. UNKNOWN: whether the runtime check cadence is longer than 8 min or the failure was transient. |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (all three agree) | **PASS WITH CAVEAT** — socket queries and log queries sequential, not simultaneous. m-ap socket 1799 vs log 1800 (Δ=1, 1 epoch elapsed during capture). Witness consistent (socket 1799, log 1799). Normal race. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2) |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 |
| Log health (WARN/ERROR filtered) | **PASS** — no new anomalous events beyond continuous KAD bootstrap warnings, 118 historic insufficient-balance, and the single pass-74 NTP runtime failure (no recurrence). No errors, no panics, no zombies. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** |
| Cross-node epoch sync | **PASS** — both at 1799 (synchronized this pass) |
