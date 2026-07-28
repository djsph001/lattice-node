# Observer Evidence Record — 2026-07-28 (Pass 79)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T10:26–10:28Z bundle (socket queries + log/metrics)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Seventy-ninth observation pass. Same processes since 2026-07-27T18:48Z (~15.7h runtime). ~9 min since pass 78 (10:18Z).

**Summary:** Routine continuation. Both nodes cycling normally. m-ap at epoch 1879 (+17 from pass 78's 1862 in ~9 min). Three persistent deviations unchanged. Zero new WARN/ERROR events. Witness now caught up to epoch 1879 (was 1 behind in pass 78). **NTP runtime failure has NOT recurred** (last at 09:40:16Z, now 47 min ago). Single-capture three-way epoch check on m-ap PASSED (all three = 1879).

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

### morning-api (~10:26Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | ~56364 (~15.7h) | — | None (pass 78: 55776; Δ = +588s ≈ 9.8 min — consistent with wall time) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree (70 unstaged files, up from 69 in pass 78). First observed: pass 1 (Jul 27). Unchanged. |
| thickness | ~985.062 | ~1000, slowly decaying | None (pass 78: 985.189; Δ = −0.127 over ~9 min ≈ −0.014/min — consistent decay rate) |

### local-witness (~10:26Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | ~56350 (~15.7h) | — | None (pass 78: 55794; Δ = +556s ≈ 9.3 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api (~10:26Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=5624, silence_secs=6, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~10:26Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=5628, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 78 (~10:18Z):** Heartbeats: api +48 (5576→5624 ~5.3/min), witness +47 (5581→5628 ~5.2/min). Both at expected rate. Silence: api 6s (pass 78: 7s), witness 3s (pass 78: 0s). Queue depth 0 on both. **No zombie eviction events. No sweep events.**

---

## Epoch State

### morning-api (~10:26Z socket capture → 10:27Z log check)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1879 (endpoint) | Cycling. +17 from pass 78 (1862→1879) in ~9 min. | None. Normal cadence (~32s/epoch range). |
| ratio | 1.02 | ~1.01–1.02 steady state | None (pass 78: 1.019977; essentially unchanged) |
| tax_calculated | 0 | Balance 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (~10:27Z socket capture → 10:27Z log check)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1879 (endpoint — after sequential capture caught up) | Cycling. +18 from pass 78 (1861→1879) in ~9 min. | None. Witness caught up — was 1 behind m-ap in pass 78. |
| ratio | 1.05 | Continuing asymptotic decline toward 1.0 | None (pass 78: 1.053723; continued gradual decline) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** Both nodes reached epoch 1879 (log data). Witness now in sync with m-ap (was δ=1 behind in pass 78).

### Three-way epoch check (sequential capture bundle)
- **morning-api:** Socket=1879, log_count=1879, last_log epoch=1879 (10:27:26Z). **Δ=0 — PASS.**
- **local-witness:** Socket=1879 (re-check), log_count=1878, last_log epoch=1879 (10:27:43Z). **PASS WITH CAVEAT** — log_count=1878 vs last_log epoch=1879 (δ=1 — epoch completed between count read and last-line read).

---

## Economic State

### morning-api (~10:26Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged) |
| own_nonce | 120 | 120 | None (unchanged) |
| witness_balance (reported) | 4980 | 5000 - morning_api_balance = 4980 | None (mesh consensus on peer balance — unchanged) |
| witness_nonce (reported) | 0 | 0 | None |

### local-witness (~10:27Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 | None |
| own_nonce | 2 | 2 | None (unchanged since pass 64) |
| morning_api_balance (reported) | 0 | 5000 | **Persistent DEVIATION.** Witness reports morning-api balance as 0. First observed: observer pass 1 (Jul 27 18:48Z). |

### Supply divergence
**OBSERVED:** morning-api sees total supply = 20 + 4980 = 5000. Witness sees total supply = 0 + 0 = 0.
**DEVIATION:** Witness-side accounting reports 0. Unchanged since first observer pass.

---

## Persistence State

### morning-api (~10:26Z socket + filesystem, sequential capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1870 | Incrementing by 10 (pass 78: 1860; +10 = 1 rotation) | None (normal — 1 rotation from 1860→1870) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause) |

**File system cross-check (~10:26Z):**
- `state.snapshot`: **895 bytes** (mtime: 2026-07-28T06:17:56 EDT — epoch 1860 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T06:17:56 EDT — unchanged since pass 78)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T06:12:56 EDT — unchanged since pass 78)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime unchanged since pass 78 (06:17). 1 rotation (1860→1870) occurred between passes 78 and 79 but the snapshot file is the one from epoch 1860 — the 1870 snapshot was already present at pass 78 time.
- **Recalculation needed:** Pass 78 reported last_snapshot_epoch=1860, current is 1870. That's +10 = 1 rotation in ~9 min. But the file mtimes are unchanged (06:17 is when both current snapshot and WAL were last written). So the 1860→1870 rotation happened sometime between ~06:17 and ~10:18Z (pass 78). The next snapshot (at epoch 1880) will trigger new file writes.

### local-witness (~10:27Z socket + filesystem, sequential capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1870 | Incrementing by 10 (pass 78: 1860; +10 = 1 rotation) | None (normal — 1 rotation) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~10:27Z):**
- `state.snapshot`: **569 bytes** (mtime: 2026-07-28T06:18:13 EDT — epoch 1860 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T06:18:13 EDT — unchanged)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T06:13:13 EDT — unchanged)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot size 569 bytes — unchanged since at least pass 69. ✓

---

## Metrics Instrumentation

**OBSERVED (from m-ap metrics lines at ~10:27Z):**
- `outstanding_fetches=0` — no pending fetches
- `aged=0` — no stale fetches
- `outbound_queues=[]` — all peer queues empty
- `max_peer_silence=3s` — well under 30s threshold

**OBSERVED (from witness metrics lines at ~10:27Z):**
- `outstanding_fetches=0`
- `aged=0`
- `outbound_queues=[]`
- `max_peer_silence=6s`

**EXPECTED:** All gauges near zero on a settled 2-node mesh with no new transactions.
**DEVIATION:** None. Mesh is quiescent.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **No new WARN/ERROR events since pass 78 capture (10:18Z).**
- **NTP:** No NTP-related log lines since pass 77. The "RUNTIME NTP: check failed" episode at 09:40:16Z (pass 74) has **NOT recurred in 47 minutes** (09:40→10:27Z).
- **Zombie eviction events:** None.
- **Sweep/eviction events:** None (0 `swept` events, 0 `evict` events, 0 `stale fetch` events).
- **Panics:** None.
- **KAD bootstrap warnings:** Continuous (every 5 minutes). Benign — `--no-mdns` with single bootstrap peer. Filtered as known pattern.

### local-witness (/tmp/lw.log)
- **No new WARN/ERROR events since pass 78 capture (10:18Z).**
- **NTP:** No NTP log lines ever observed on witness.
- **Insufficient balance:** 118 (unchanged, all historic Jul 27). **No new occurrences.**
- **Panics:** None.

### Log filter (WARN/ERROR excluded as benign)

| Pattern | m-ap | lw | Status |
|---------|------|----|--------|
| `Failed to trigger bootstrap` | Many (continuous) | 0 | Benign — `--no-mdns` with single bootstrap peer |
| `skip-ntp-check` | 0 | 0 | Clean |
| `No snapshot` | 0 | 3 (startup) | Clean |
| `zombie` | 0 | 0 | Clean |
| `insufficient balance` | 0 | 118 (historic) | No new occurrences |
| `panicked` | 0 | 0 | Clean |
| `NTP.*fail` | 3 (09:40Z, historic) | 1 (00:08Z ~10h ago, historic) | Not recurred |

---

## Build Commit Verification

| Check | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| git HEAD | `cb5d4b1` | — | — |
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree (70 unstaged files, up from 69 in pass 78). Unchanged since pass 1. |

---

## NTP Runtime Check — Update

The single "RUNTIME NTP: check failed" episode from 2026-07-28T09:40:16Z (pass 74) has **not recurred in 47 minutes** (09:40→10:27Z). Zero NTP failures on either node in the last 47 minutes across both nodes. The episode was transient.

System clock: Local time Tue 2026-07-28 06:26 EDT, NTP service active. ✓

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 78 Status | Pass 79 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No. Unstaged files count: 70 (pass 78: 69 — ±1 normal) |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

**No new deviations detected in this pass.**

---

## Minor Observations (Not Deviations)

| Observation | First Noted | Status |
|-------------|------------|--------|
| morning-api snapshot size stable at 895 bytes for fifth consecutive pass | Pass 71 | Resolved. The ±1 byte oscillation (894↔895 in passes 71-75) has stopped. Unknown: cause. |
| NTP runtime check failure (09:40:16Z) has not recurred in 47 min | Pass 74 | Single-episode so far; 47 min of NTP silence confirmed |
| Witness caught up to m-ap epoch (both at 1879) | This pass | Normal — was 1 behind in pass 78 |
| KAD bootstrap warnings on m-ap every 5 min (continuous) | Pass 1 | Benign with `--no-mdns`. Noted for completeness. |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (socket, log count, last log line) | **m-ap: PASS** (all three = 1879 — second consecutive exact match). **Witness: PASS WITH CAVEAT** (log_count 1878 vs last_log 1879 — δ=1 from sequential capture race) |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2) |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 |
| Log health (WARN/ERROR filtered) | **PASS** — no new anomalous events. KAD bootstrap warnings benign and continuous. 118 historic insufficient-balance unchanged. No panics, no zombies, no errors. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** — m-ap 3s, witness 6s. Both well under threshold. |
| Cross-node epoch sync | **PASS** — both nodes at epoch 1879 (log data). Witness caught up. |
