# Observer Evidence Record — 2026-07-28 (Pass 76)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T09:56Z bundle (socket queries)
**Log/metrics capture:** ~2026-07-28T09:56Z
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Seventy-sixth observation pass. Same processes since 2026-07-27T18:48Z (~15.1h runtime). ~8 min since pass 75 (09:48Z).

**Summary:** Routine continuation. Both nodes cycling normally. Epoch 1814 (+15 from pass 75's 1799 at capture). Snapshot rotated twice (1790→1800→1810). Three persistent deviations unchanged. **Classification update on NTP runtime check:** the 09:40:16Z failure from pass 74 has NOT recurred as a full runtime failure (all-3-servers-down), but individual NTP server query failures are ongoing on both nodes. Witness confirmed to also experience NTP query failures (08:00:06Z), previously not recorded. Metrics healthy: aged=0, queues=[], silence<10s.

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

### morning-api (~09:56Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 54408 (~15.1h) | — | None (pass 75: 53952; Δ = +456s ≈ 7.6 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree (64 unstaged files). First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 985.553 | ~1000, slowly decaying | None (pass 75: 985.671; Δ = −0.118 over ~8 min — consistent decay ~0.015/min) |

### local-witness (~09:56Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 54422 (~15.1h) | — | None (pass 75: 53957; Δ = +465s ≈ 7.75 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api (~09:56Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=5439, silence_secs=9, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~09:56Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=5443, silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 75 (~09:48Z):** Heartbeats api +46 (5393→5439 ~5.75/min), witness +46 (5397→5443 ~5.75/min). Both at expected rate. Silence: api 9s (pass 75: 8s — normal), witness 7s (pass 75: 0s — normal variation). Queue depth 0 on both.

**No zombie eviction events** detected in either log.

---

## Epoch State

### morning-api (~09:56Z socket capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1814 (endpoint) | Cycling. +15 from pass 75 (1799→1814) in ~8 min. | None. Normal cadence (~32s/epoch). |
| ratio | 1.019789 | ~1.01–1.02 steady state | None (pass 75: 1.019980; Δ = −0.00019 — normal drift) |
| tax_calculated | 0 | Balance 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (~09:56Z socket capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1815 (endpoint) | Cycling. +16 from pass 75 (1799→1815) in ~8 min. | None. Normal cadence. Bundle capture sequential. |
| ratio | 1.055219 | Continuing asymptotic decline toward 1.0 | None (pass 75: 1.056; Δ = −0.00078 — continued approach to 1.0) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** m-ap=1814, witness=1815 (δ=1). Common race between sequential captures. Pass 75 showed both synchronized at 1799 — the δ=1 here is normal timing.

### Three-way epoch check (single-capture discipline — sequential, not simultaneous)
- **morning-api:** Socket=1814 (~09:55Z), grep count=1817, last_log epoch=1817 (09:56:26Z). Δ=3 between endpoint and log — 3 epochs elapsed between socket query and log grep within the capture window. Normal timing gap.
- **local-witness:** Socket=1815 (~09:56Z), grep count=1816, last_log epoch=1816 (09:56:13Z). Δ=1 — narrower gap, consistent with tighter capture timing.

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

### morning-api (~09:56Z socket + filesystem, sequential capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1810 | Incrementing by 10 (pass 75: 1790; +20 = 2 rotations) | None (normal — 2 rotations since pass 75: 1800 and 1810) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (~09:56Z):**
- `state.snapshot`: **895 bytes** (mtime: 2026-07-28T05:52 EDT — epoch 1810 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T05:52 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T05:47 EDT — previous epoch 1800 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 05:47 (pass 75) to 05:52 — confirmed 2 rotations (1790→1800→1810). ✓
- Snapshot size **895 bytes** (pass 75: 895; pass 74: 894; pass 73: 895; pass 72: 895; pass 71: 894). Back to 895. The ±1 byte oscillation continues. UNKNOWN: cause. Consistent with serialization of a threshold/boundary value.

### local-witness (~09:56Z socket + filesystem, sequential capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1810 | Incrementing by 10 (pass 75: 1790; +20 = 2 rotations) | None (normal — 2 rotations since pass 75) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~09:56Z):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T05:53 EDT — epoch 1810 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T05:53 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T05:48 EDT — previous epoch 1800 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 05:43 (pass 75) to 05:53 — confirmed 2 rotations. ✓
- Snapshot size 569 bytes (pass 75: 569; unchanged since at least pass 69). ✓

---

## Metrics Instrumentation

**OBSERVED (from m-ap metrics line at ~09:56Z):**
- `outstanding_fetches=0` — no pending fetches
- `aged=0` — no stale fetches
- `outbound_queues=[]` — all peer queues empty
- `max_peer_silence=3s` — well under 30s threshold

**OBSERVED (from witness metrics line at ~09:56Z):**
- `outstanding_fetches=0`
- `aged=0`
- `outbound_queues=[]`
- `max_peer_silence=6s`

**EXPECTED:** All gauges near zero on a settled 2-node mesh with no new transactions.
**DEVIATION:** None. Mesh is quiescent.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **WARN (195 total, +4 since pass 75's 191):**
  - +2 KAD bootstrap WARNs (`No known peers`) — repeating every ~5 min, benign
  - +2 individual NTP server query failures at 09:50:29–32Z (pool.ntp.org and time.apple.com with `Resource temporarily unavailable (os error 11)`) — see NTP section below
  - **No new "RUNTIME NTP: check failed" line at 09:50Z** — only 2 of 3 servers queried failed (time.google.com was not retried or succeeded silently), so the runtime check did NOT trigger.
- **ERROR:** 0 (unchanged).
- **Zombie eviction events:** None.
- **Sweep/eviction events:** None.
- **Panics:** None.
- **RUNTIME NTP failures since pass 75:** 0 (the single episode at 09:40:16Z from pass 74 remains the only full runtime check failure).

### local-witness (/tmp/lw.log)
- **WARN (123, unchanged since pass 69):** All historic startup artifacts.
  - **NTP query failure revealed:** The witness had an individual NTP query failure at 2026-07-28T08:00:06Z (`pool.ntp.org: Resource temporarily unavailable (os error 11)`). This was not recorded in prior passes because witness was classified as "never observed" for NTP failures — that was accurate only for RUNTIME check failures, not individual query failures. This pass corrects the record.
- **ERROR:** 0 (unchanged).
- **Insufficient balance:** 118 (unchanged, all historic Jul 27). No new occurrences.
- **RUNTIME NTP failures:** 0 total (never observed on witness — the individual query failures never triggered a full check failure).

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
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind + dirty (67 unstaged files). Unchanged since pass 1. |

---

## Updated Observation: NTP Runtime Check Failure — Single Episode

| Field | Detail |
|-------|--------|
| **OBSERVED** | The "RUNTIME NTP: check failed" from pass 74 (2026-07-28T09:40:16Z) has NOT recurred as a full runtime failure. Two new individual NTP server query failures at 09:50:29–32Z (pool.ntp.org and time.apple.com, both `os error 11` — `Resource temporarily unavailable`) did NOT produce a runtime check failure line. Only 2 of 3 servers failed; time.google.com either succeeded silently or was not retried. Pattern: a full runtime failure requires ALL 3 servers to fail simultaneously. Individual query failures are intermittent (~1–2/hour) but do not trigger the alert. |
| **EXPECTED** | Not documented. No runtime NTP check cadence is specified in VERIFIED-BEHAVIOR.md or MESH.md. The observer-reconstructed pattern suggests: (a) individual server queries are independent and may fail without alert; (b) the runtime check fires on some cycle >=10 min; (c) all 3 must fail to trigger. |
| **CLASSIFICATION** | The 09:40:16Z episode remains a single observed occurrence. The runtime check has not recurred with the same pattern in ~16 min of subsequent observation (09:40→09:56Z). Individual query failures are ongoing (~2 in pass 76 window) but benign. The check cadence is UNKNOWN — may be longer than the inter-pass interval. |

### New Observation: Witness NTP Query Failures

| Field | Detail |
|-------|--------|
| **OBSERVED** | Witness had an NTP query failure at 2026-07-28T08:00:06Z: `pool.ntp.org: Input/output error: Resource temporarily unavailable (os error 11) (fallback)`. This is the same failure mode as morning-api's individual query failures. |
| **EXPECTED** | Not documented. Prior passes classified witness as "0 NTP runtime failures (never observed)" — accurate for runtime check failures but missed individual query failures. |
| **CLASSIFICATION** | This correction updates the record: witness IS affected by NTP query failures, though like morning-api, they are individual server failures and have never triggered a full runtime check failure for the witness. First occurrence: 08:00:06Z (known since this pass). No others detected. |

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 75 Status | Pass 76 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

**No new deviations detected in this pass.**

---

## Minor Observations (Not Deviations)

| Observation | First Noted | Status |
|-------------|------------|--------|
| morning-api snapshot size back at 895 bytes (was 894 in pass 74, 895 in pass 75) | Pass 71 | Fluctuating 894↔895. UNKNOWN: cause of ±1 byte oscillation. Possibly serialization of a threshold/boundary value. |
| Witness NTP query failure at 08:00:06Z — previously not recorded | This pass | Single occurrence on witness, same pattern as m-ap individual query failures. No runtime check triggered. |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (all three agree) | **PASS WITH CAVEAT** — socket queries and log grep sequential, not simultaneous. m-ap socket 1814 vs log 1817 (Δ=3 — 3 epochs elapsed during single-capture bundle). Witness socket 1815 vs log 1816 (Δ=1). Both within expected timing gap. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2) |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 |
| Log health (WARN/ERROR filtered) | **PASS** — no new anomalous events beyond continuous KAD bootstrap warnings, 118 historic insufficient-balance, and ongoing individual NTP query failures (no runtime check failure recurrence). No errors, no panics, no zombies. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** |
| Cross-node epoch sync | **PASS WITH CAVEAT** — m-ap=1814, witness=1815 (δ=1 — normal sequential capture race) |
