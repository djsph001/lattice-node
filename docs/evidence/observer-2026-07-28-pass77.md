# Observer Evidence Record — 2026-07-28 (Pass 77)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T10:11Z bundle (socket queries)
**Log/metrics capture:** ~2026-07-28T10:11Z
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Seventy-seventh observation pass. Same processes since 2026-07-27T18:48Z (~15.4h runtime). ~15 min since pass 76 (09:56Z).

**Summary:** Routine continuation. Both nodes cycling normally. Epoch 1842 (+28 from pass 76's 1814 at capture). Snapshot rotated thrice (1820→1830→1840). Three persistent deviations unchanged. Zero new WARN/ERROR events beyond KAD bootstrap spam. No new NTP query failures since pass 76 capture (09:56Z). **NTP runtime failure episode (09:40:16Z from pass 74) has NOT recurred in 31 minutes.** Witness socket was at `/tmp/local-witness/lattice.sock` (not `/tmp/lv-quick/lattice.sock` as prior passes may have recorded — the witness socket is at its storage dir, not an arbitrary path).

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
**Witness socket:** `/tmp/local-witness/lattice.sock` (in storage dir, consistent with `x-socket` path).

---

## Node Info

### morning-api (~10:11Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 55237 (~15.4h) | — | None (pass 76: 54408; Δ = +829s ≈ 13.8 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree (64+ unstaged files). First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 985.331 | ~1000, slowly decaying | None (pass 76: 985.553; Δ = −0.222 over ~15 min ≈ −0.0148/min — consistent decay) |

### local-witness (~10:11Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 55280 (~15.4h) | — | None (pass 76: 54422; Δ = +858s ≈ 14.3 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api (~10:11Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=5522, silence_secs=4, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~10:11Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=5529, silence_secs=0, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 76 (~09:56Z):** Heartbeats api +83 (5439→5522 ~5.5/min), witness +86 (5443→5529 ~5.7/min). Both at expected rate. Silence: api 4s (pass 76: 9s — normal variation), witness 0s (pass 76: 7s). Queue depth 0 on both. **No zombie eviction events.**

---

## Epoch State

### morning-api (~10:11Z socket capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1842 (endpoint) | Cycling. +28 from pass 76 (1814→1842) in ~15 min. | None. Normal cadence (~32s/epoch). |
| ratio | 1.019792 | ~1.01–1.02 steady state | None (pass 76: 1.019789; Δ = +0.000003 — stable) |
| tax_calculated | 0 | Balance 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (~10:11Z socket capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1843 (endpoint) | Cycling. +28 from pass 76 (1815→1843) in ~15 min. | None. Normal cadence. |
| ratio | 1.054280 | Continuing asymptotic decline toward 1.0 | None (pass 76: 1.055219; Δ = −0.000939 — continued approach to 1.0) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** m-ap=1842, witness=1843 (δ=1). Same sequential-capture race as pass 76.

### Three-way epoch check (single-capture discipline — sequential)
- **morning-api:** Socket=1842 (~10:11Z), log_count=1845, last_log epoch=1842 (10:08:56Z). Δ=3 between endpoint and log count — 3 epochs elapsed between socket query and log count within the capture window. Consistent with pass 76's Δ=3.
- **local-witness:** Socket=1843 (~10:11Z), log_count=1844, last_log epoch=1843 (10:09:43Z). Δ=1 — same tight gap as pass 76.

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

### morning-api (~10:11Z socket + filesystem, sequential capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1840 | Incrementing by 10 (pass 76: 1810; +30 = 3 rotations) | None (normal — 3 rotations since pass 76: 1820, 1830, 1840) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (~10:11Z):**
- `state.snapshot`: **895 bytes** (mtime: 2026-07-28T06:07 EDT — epoch 1840 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T06:07 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T06:02 EDT — previous epoch 1830 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 05:52 (pass 76) to 06:07 — confirmed 3 rotations (1810→1820→1830→1840). ✓
- Snapshot size **895 bytes** (pass 76: 895; pass 75: 895; pass 74: 894). Back to 895 for third consecutive pass. The ±1 byte oscillation may have stabilized. UNKNOWN: cause.

### local-witness (~10:11Z socket + filesystem, sequential capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1840 | Incrementing by 10 (pass 76: 1810; +30 = 3 rotations) | None (normal — 3 rotations since pass 76) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~10:11Z):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T06:08 EDT — epoch 1840 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T06:08 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T06:03 EDT — previous epoch 1830 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 05:53 (pass 76) to 06:08 — confirmed 3 rotations. ✓
- Snapshot size 569 bytes (pass 76: 569; unchanged since at least pass 69). ✓

---

## Metrics Instrumentation

**OBSERVED (from m-ap metrics line at ~10:11Z):**
- `outstanding_fetches=0` — no pending fetches
- `aged=0` — no stale fetches
- `outbound_queues=[]` — all peer queues empty
- `max_peer_silence=3s` — well under 30s threshold

**OBSERVED (from witness metrics line at ~10:11Z):**
- `outstanding_fetches=0`
- `aged=0`
- `outbound_queues=[]`
- `max_peer_silence=6s`

**EXPECTED:** All gauges near zero on a settled 2-node mesh with no new transactions.
**DEVIATION:** None. Mesh is quiescent.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **WARN:** Repeating KAD bootstrap warnings (`No known peers` every 5 min). No other WARNs beyond the two startup `InsufficientPeers` warnings from Jul 27 (18:48:26).
- **NTP query failures since pass 76:** **Zero.** The last NTP query failures were at 09:50:29–32Z (pool.ntp.org and time.apple.com, recorded in pass 76). No new failures in the 15+ minutes since pass 76 capture.
- **RUNTIME NTP: check failed:** The single episode at 2026-07-28T09:40:16Z (pass 74) has **NOT recurred in 31 minutes** (09:40→10:11Z).
- **ERROR:** 0 (unchanged).
- **Zombie eviction events:** None.
- **Sweep/eviction events:** None (0 `swept` events, 0 `evict` events).
- **Panics:** None.

### local-witness (/tmp/lw.log)
- **WARN (123 total, unchanged since pass 69):** All historic startup artifacts.
- **NTP query failures:** **None detected.** No NTP-related log lines in witness logs at all since the single occurrence at 08:00:06Z (recorded in pass 76).
- **ERROR:** 0 (unchanged).
- **Insufficient balance:** 118 (unchanged, all historic Jul 27). **No new occurrences.**
- **RUNTIME NTP failures:** 0 total (never observed on witness).
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
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind + dirty (67 unstaged files). Unchanged since pass 1. |

---

## NTP Runtime Check — Update

The single "RUNTIME NTP: check failed" episode from 2026-07-28T09:40:16Z (recorded in pass 74) has **not recurred**. It has been 31 minutes since the episode, during which two subsequent NTP query cycles occurred (including the 09:50Z individual failures recorded in pass 76). The runtime check cadence is UNKNOWN — it may run on a longer cycle (>15 min) than the inter-pass interval.

**Observation:** No new NTP query failures since 09:50Z. Both nodes currently have NTP working (or at least not failing).

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 76 Status | Pass 77 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

**No new deviations detected in this pass.**

---

## Minor Observations (Not Deviations)

| Observation | First Noted | Status |
|-------------|------------|--------|
| morning-api snapshot size at 895 bytes for third consecutive pass (was oscillating 894↔895 in passes 71-75) | Pass 71 | Stable at 895. The ±1 byte oscillation may have resolved. UNKNOWN: cause. |
| NTP runtime check failure (09:40:16Z) has not recurred in 31 min | Pass 74 | Single-episode observation; check cadence remains UNKNOWN |
| Zero new NTP query failures on both nodes since pass 76 capture (09:56Z) | This pass | Intermittent pattern: failures cluster in ~2/hour bursts, then silence |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (socket, log count, last log line) | **PASS WITH CAVEAT** — sequential capture, not simultaneous. m-ap socket 1842 vs log count 1845 (Δ=3 — 3 epochs elapsed during capture bundle). Witness socket 1843 vs log count 1844 (Δ=1). Both within expected timing gap. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2) |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 |
| Log health (WARN/ERROR filtered) | **PASS** — no new anomalous events. KAD bootstrap warnings benign and continuous. 118 historic insufficient-balance unchanged. No panics, no zombies, no errors. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** |
| Cross-node epoch sync | **PASS WITH CAVEAT** — m-ap=1842, witness=1843 (δ=1 — normal sequential capture race) |
