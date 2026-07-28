# Observer Evidence Record — 2026-07-28 (Pass 73)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T09:30Z bundle (simultaneous capture with ~30s window between node queries)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Seventy-third observation pass. Same processes since 2026-07-27T18:48Z (~14.7h runtime). ~8 min since pass 72 (09:22Z).

**Summary:** Routine continuation. Both nodes cycling normally. Epoch endpoint shows both at 1767/1768 (+18-19 since pass 72). Snapshot rotated once (1750→1760). All three persistent deviations unchanged. No new deviations detected.

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

### morning-api (~09:30Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 53026 (~14.7h) | — | None (pass 72: 52463; Δ = +563s ≈ 9.4 min — covers bundle interval and margins) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree (64 unstaged files). First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 985.92 | ~1000, slowly decaying | None (pass 72: 986.067; Δ = −0.147 over ~8 min — consistent decay ~0.018/min) |

### local-witness (~09:30Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 52993 (~14.7h) | — | None (pass 72: 52466; Δ = +527s ≈ 8.8 min — bundle timing variation from m-ap) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api (~09:30Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=5301, silence_secs=5, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~09:30Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=5300, silence_secs=6, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 72 (~09:22Z):** Heartbeats api +56 (5245→5301 ~7.0/min), witness +52 (5248→5300 ~6.5/min). Both at expected rate. Silence: api 5s (pass 72: 8s — normal variation), witness 6s (pass 72: 5s — normal). Queue depth 0 on both.

**No zombie eviction events** detected in either log.

---

## Epoch State

### morning-api (~09:30Z socket capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1768 (endpoint) | Cycling. +19 from pass 72 (1749→1768) in ~8 min. | None. Normal cadence (~25-30s/epoch). |
| ratio | 1.019788 | ~1.01–1.02 steady state | None (pass 72: 1.019785; unchanged within precision) |
| tax_calculated | 0 | Balance 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (~09:30Z socket capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1767 (endpoint) | Cycling. +18 from pass 72 (1749→1767) in ~8 min. | None. Normal cadence. Bundle capture sequential (witness queried ~30s later). |
| ratio | 1.057094 | Continuing asymptotic decline | None (pass 72: 1.057548; Δ = −0.000454 over ~8 min — continued approach to 1.0) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** Bundle showed m-ap at 1768, witness at 1767 (Δ=1 — sequential capture with ~30s gap, normal at ~30s/epoch cadence).

### Three-way epoch check (single-capture bundle caveat — log queries ran ~90s after socket queries)
- **morning-api:** Socket=1768 (~09:30Z), grep count=1770 (run ~09:32Z), last_log epoch=1770 (09:32:56Z). Δ=2 between endpoint and log — 2 epochs elapsed during the ~2.5 min between socket and log queries. Normal race. ROs NOT a deviation.
- **local-witness:** Socket=1767 (~09:30Z), grep count=1768 (run ~09:32Z), last_log epoch=1769 (09:32:43Z). Δ=2 — 2 epochs elapsed during capture window. Normal race.

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

### morning-api (~09:30Z socket + filesystem, sequential capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1760 | Incrementing by 10 (pass 72: 1750; +10 = 1 rotation) | None (normal — 1 rotation since pass 72) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (~09:30Z single capture, fs queried ~60s after socket):**
- `state.snapshot`: 895 bytes (mtime: 2026-07-28T05:32:56 EDT — epoch 1760 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T05:32:56 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T05:27:56 EDT — previous epoch 1750 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 05:22 (pass 72) to 05:32 — confirmed 1 rotation occurred. ✓
- Snapshot size 895 bytes (pass 72: 895; unchanged) ✓

### local-witness (~09:30Z socket + filesystem, sequential capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1760 | Incrementing by 10 (pass 72: 1750; +10 = 1 rotation) | None (normal — 1 rotation since pass 72) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~09:30Z):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T05:28:13 EDT — epoch 1760 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T05:28:13 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T05:23:13 EDT — previous epoch 1750 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 05:23 (pass 72) to 05:28 — confirmed 1 rotation occurred. ✓
- Snapshot size 569 bytes (pass 72: 569; unchanged) ✓

---

## Metrics Instrumentation

**OBSERVED (from m-ap metrics line at ~09:32Z):**
- `outstanding_fetches=0` — no pending fetches
- `aged=0` — no stale fetches
- `outbound_queues=[]` — all peer queues empty
- `max_peer_silence=3s` — well under 30s threshold

**OBSERVED (from witness metrics line at ~09:32Z):**
- `outstanding_fetches=0`
- `aged=0`
- `outbound_queues=[]`
- `max_peer_silence=6s`

**EXPECTED:** All gauges near zero on a settled 2-node mesh with no new transactions.
**DEVIATION:** None. Mesh is quiescent.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **WARN (177 total, every 5 min since Jul 27 18:48Z):** `libp2p_kad::behaviour: Failed to trigger bootstrap: No known peers.` — expected on 2-node loopback with `--no-mdns`. +1 since pass 72 (176→177). Benign and continuous.
- **WARN (Jul 28 ~07:37Z):** `NTP query to pool.ntp.org failed: Input/output error` — single transient occurrence. Unchanged.
- **WARN (Jul 27 18:48Z, historic):** `Failed to gossip genesis (will retry on peer connect) error=InsufficientPeers` — startup artifact, no recurrence.
- **WARN (Jul 27 18:48Z, historic):** `[block-publish] Failed to publish block proposal_id="genesis" error=InsufficientPeers` — startup artifact, no recurrence.
- **ERROR:** None.
- **Zombie eviction events:** None.
- **Sweep/eviction events:** None.

### local-witness (/tmp/lw.log)
- **WARN (118 total, all historic Jul 27):** `Transaction validation failed error=insufficient balance` — all from original redistribution test. No new occurrences since Jul 27.
- **WARN (Jul 28 ~08:00Z):** `NTP query to pool.ntp.org failed: Input/output error` — single transient occurrence. Unchanged.
- **ERROR:** None.

### Log filter (WARN/ERROR excluded as benign)
- `skip-ntp-check`: No hits.
- `No snapshot`: No hits (both have snapshots).
- `zombie`: No hits.
- `insufficient balance`: 118 hits on witness (historic, all Jul 27). 0 on m-ap.
- `panicked`: No hits since pass 72 review.

---

## Build Commit Verification

| Check | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| git HEAD | `cb5d4b1` | — | — |
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind + dirty (64 unstaged files). Unchanged since pass 1. |

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 72 Status | Pass 73 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

**No new deviations detected in this pass.**

---

## Minor Observations (Not Deviations)

| Observation | First Noted | Status |
|-------------|------------|--------|
| morning-api snapshot size stabilized at 895 bytes (was 894 in pass 71, 895 in pass 72) | Pass 72 | Persisting. UNKNOWN: why it grew from 894→895 and has since stabilized. |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (all three agree) | **PASS WITH CAVEAT** — socket queries and log queries ran ~90s apart. Socket 1768/1767 vs grep 1770/1768. Δ=2 for both, consistent with 2-3 epochs elapsed during capture window. Normal race, not a deviation. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2) |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 |
| Log health (WARN/ERROR filtered) | **PASS** — only continuous KAD bootstrap warnings (benign, 177 count), 118 historic insufficient-balance entries, and single NTP failure each. No new anomalous events. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** |
| Cross-node epoch sync | **PASS WITH CAVEAT** — m-ap at 1768, witness at 1767 (Δ=1, sequential capture timing) |
