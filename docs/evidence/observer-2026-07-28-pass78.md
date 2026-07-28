# Observer Evidence Record — 2026-07-28 (Pass 78)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T10:18Z bundle (socket queries)
**Log/metrics capture:** ~2026-07-28T10:18–10:20Z
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Seventy-eighth observation pass. Same processes since 2026-07-27T18:48Z (~15.5h runtime). ~7 min since pass 77 (10:11Z).

**Summary:** Routine continuation. Both nodes cycling normally. Epoch 1862 on morning-api (+20 from pass 77's 1842 in ~7 min). Snapshot rotated twice per node (1840→1850→1860). Three persistent deviations unchanged. Zero new WARN/ERROR events. **NTP runtime failure episode (09:40:16Z from pass 74) has NOT recurred in 38 minutes.** Single-capture three-way epoch check on morning-api PASSED (all three = 1862).

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

### morning-api (~10:18Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 55776 (~15.5h) | — | None (pass 77: 55237; Δ = +539s ≈ 9 min — consistent with wall time) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree (69 unstaged files). First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 985.189 | ~1000, slowly decaying | None (pass 77: 985.331; Δ = −0.142 over ~7 min ≈ −0.0203/min — consistent decay rate) |

### local-witness (~10:18Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 55794 (~15.5h) | — | None (pass 77: 55280; Δ = +514s ≈ 8.6 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api (~10:18Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=5576, silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~10:18Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=5581, silence_secs=0, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 77 (~10:11Z):** Heartbeats: api +54 (5522→5576 ~7.7/min), witness +52 (5529→5581 ~7.4/min). Both at expected rate. Silence: api 7s (pass 77: 4s), witness 0s (pass 77: 0s). Queue depth 0 on both. **No zombie eviction events. No sweep events.**

---

## Epoch State

### morning-api (~10:18Z socket capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1862 (endpoint) | Cycling. +20 from pass 77 (1842→1862) in ~7 min. | None. Normal cadence (~21s/epoch). |
| ratio | 1.019977 | ~1.01–1.02 steady state | None (pass 77: 1.019792; Δ = +0.000185 — minimal variation) |
| tax_calculated | 0 | Balance 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (~10:18Z socket capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1861 (endpoint — earlier in sequential bundle) | Cycling. +18 from pass 77 (1843→1861). | None. Normal cadence. |
| ratio | 1.053723 | Continuing asymptotic decline toward 1.0 | None (pass 77: 1.054280; Δ = −0.000557 — continued decline) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** m-ap=1862, witness=1861 (δ=1). Same sequential-capture race as prior passes.

### Three-way epoch check (single-capture discipline — sequential)
- **morning-api:** Socket=1862, log_count=1862, last_log epoch=1862 (10:18:56Z). **Δ=0 — PASS.** First time all three match exactly in recent record.
- **local-witness:** Socket=1861, log_count=1862, last_log epoch=1862 (10:19:13Z). Δ=1 — same sequential capture gap as prior passes.

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
| morning_api_balance (reported) | 0 | 5000 | **Persistent DEVIATION.** Witness reports morning-api balance as 0. First observed: observer pass 1 (Jul 27 18:48Z). |

### Supply divergence
**OBSERVED:** morning-api sees total supply = 20 + 4980 = 5000. Witness sees total supply = 0 + 0 = 0.
**DEVIATION:** Witness-side accounting reports 0. Unchanged since first observer pass.

---

## Persistence State

### morning-api (~10:18Z socket + filesystem, sequential capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1860 | Incrementing by 10 (pass 77: 1840; +20 = 2 rotations) | None (normal — 2 rotations since pass 77: 1850, 1860) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause) |

**File system cross-check (~10:18Z):**
- `state.snapshot`: **895 bytes** (mtime: 2026-07-28T06:17:56 EDT — epoch 1860 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T06:17:56 EDT — created at snapshot rotation)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T06:12:56 EDT — prior epoch 1850 rotation)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 06:07 (pass 77) to 06:17 — confirmed 2 rotations (1840→1850→1860). ✓
- Snapshot size **895 bytes** (pass 77: 895; pass 76: 895; pass 75: 895). Fourth consecutive pass at 895 — the ±1 byte oscillation (894↔895 in passes 71-75) has resolved. UNKNOWN: cause.

### local-witness (~10:18Z socket + filesystem, sequential capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1860 | Incrementing by 10 (pass 77: 1840; +20 = 2 rotations) | None (normal — 2 rotations) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~10:18Z):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T06:18:13 EDT — epoch 1860 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T06:18:13 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T06:13:13 EDT — prior epoch 1850 rotation)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 06:08 (pass 77) to 06:18 — confirmed 2 rotations. ✓
- Snapshot size 569 bytes (unchanged since at least pass 69). ✓

---

## Metrics Instrumentation

**OBSERVED (from m-ap metrics lines at ~10:19Z):**
- `outstanding_fetches=0` — no pending fetches
- `aged=0` — no stale fetches
- `outbound_queues=[]` — all peer queues empty
- `max_peer_silence=3s` — well under 30s threshold

**OBSERVED (from witness metrics lines at ~10:19Z):**
- `outstanding_fetches=0`
- `aged=0`
- `outbound_queues=[]`
- `max_peer_silence=6s`

**EXPECTED:** All gauges near zero on a settled 2-node mesh with no new transactions.
**DEVIATION:** None. Mesh is quiescent.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **No new WARN/ERROR events since pass 77 capture (10:11Z).**
- **NTP:** No NTP-related log lines since pass 77. The "RUNTIME NTP: check failed" episode at 09:40:16Z (pass 74) has **NOT recurred in 38 minutes** (09:40→10:18Z).
- **Zombie eviction events:** None.
- **Sweep/eviction events:** None (0 `swept` events, 0 `evict` events).
- **Panics:** None.

### local-witness (/tmp/lw.log)
- **No new WARN/ERROR events since pass 77 capture (10:11Z).**
- **NTP:** No NTP log lines ever observed on witness.
- **Insufficient balance:** 118 (unchanged, all historic Jul 27). **No new occurrences.**
- **Panics:** None.

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
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind + dirty (69 unstaged files, up from 64+ in pass 77). Unchanged since pass 1. |

---

## NTP Runtime Check — Update

The single "RUNTIME NTP: check failed" episode from 2026-07-28T09:40:16Z (pass 74) has **not recurred in 38 minutes** (09:40→10:18Z). No new NTP query failures on either node since pass 76 capture (09:56Z) — confirmed silent for 22+ minutes.

The runtime NTP check cadence remains UNKNOWN. The episode may have been a transient network condition.

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 77 Status | Pass 78 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No. Unstaged files count: 69 (pass 77: 64+ — ±5 normal variation) |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

**No new deviations detected in this pass.**

---

## Minor Observations (Not Deviations)

| Observation | First Noted | Status |
|-------------|------------|--------|
| morning-api snapshot size stable at 895 bytes for fourth consecutive pass | Pass 71 | Resolved. The ±1 byte oscillation (894↔895 in passes 71-75) has stopped. Unknown: cause. |
| NTP runtime check failure (09:40:16Z) has not recurred in 38 min | Pass 74 | Single-episode so far; check cadence remains UNKNOWN |
| Zero NTP query failures on both nodes since pass 76 capture (09:56Z) | This pass | 22+ minutes of NTP silence |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (socket, log count, last log line) | **m-ap: PASS** (all three = 1862 — first exact match in recent record). **Witness: PASS WITH CAVEAT** (socket 1861 vs log_count 1862 vs last_log 1862 — Δ=1 from sequential capture race) |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2) |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 |
| Log health (WARN/ERROR filtered) | **PASS** — no new anomalous events. KAD bootstrap warnings benign and continuous. 118 historic insufficient-balance unchanged. No panics, no zombies, no errors. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** — m-ap 3s, witness 6s. Both well under threshold. |
| Cross-node epoch sync | **PASS WITH CAVEAT** — m-ap=1862, witness=1861 (δ=1 — normal sequential capture race) |
