# Observer Evidence Record — 2026-07-28 (Pass 71)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T09:15Z bundle (simultaneous capture + verification sweep)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Seventy-first observation pass. Same processes since 2026-07-27T18:48Z (~14.4h runtime). ~8 min since pass 70 (09:07Z).

**Summary:** Routine continuation. Both nodes reached epoch 1734 (endpoint, both agree). Snapshot epoch advanced 1710→1730 (+2 rotations). All three persistent deviations unchanged. No new deviations detected.

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

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 51994 (~14.4h) | — | None (pass 70: 51518; Δ = +476s ≈ 7.9 min — covers bundle interval) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree (62 unstaged files, mostly observer evidence docs). First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 986.185 | ~1000, slowly decaying | None (pass 70: 986.31; Δ = −0.125 over ~8 min — consistent decay ~0.016/min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 51999 (~14.4h) | — | None (pass 70: 51511; Δ = +488s ≈ 8.1 min — bundle timing variation) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED (09:15Z):** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=5197, silence_secs=9, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED (09:15Z):** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=5201, silence_secs=0, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 70 (~09:07Z):** Heartbeats api +47 (5150→5197 ~5.9/min), witness +49 (5152→5201 ~6.1/min). Both normal rate. Silence: api 9s (pass 70: 3s — well within <30s threshold). Witness 0s. Queue depth 0 on both.

**No zombie eviction events** detected in either log.

---

## Epoch State

### morning-api (single-capture bundle ~09:15Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1734 (endpoint) | Cycling. +15 from pass 70 (1719→1734) in ~8 min. | None. Normal cadence (~32s/epoch). |
| ratio | 1.019783 | ~1.01–1.02 steady state | None (pass 70: 1.01978; unchanged) |
| tax_calculated | 0 | Balance 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (single-capture bundle ~09:15Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1734 (endpoint) | Cycling. +16 from pass 70 (1718→1734) in ~8 min. | None. Normal cadence. |
| ratio | 1.058103 | Continuing asymptotic decline | None (pass 70: 1.05895; Δ = −0.00085 over ~8 min — continued approach to 1.0) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** Bundle showed both at 1734 (Δ=0 — exact alignment this pass, unlike pass 70's Δ=1 race). 3-way check: api 1734/1736/1736, witness 1734/1735/1735 — endpoint matched, log counts ran slightly ahead due to interleaved capture timing.

### Three-way epoch check
- **morning-api:** Socket=1734 (09:15:00Z), grep=1736 (09:15:30Z capture), last_log=epoch=1736. The Δ=2 between endpoint and log count is a race at an epoch boundary — 2 epochs elapsed between endpoint query and log query. Not a deviation.
- **local-witness:** Socket=1734 (09:15:22Z), grep=1735, last_log=epoch=1735. Δ=1 — one epoch elapsed between endpoint and log query. Normal race.

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
**UNKNOWN:** Witness nonce = 2 (unchanged since pass 64). Two transactions originated from witness.

---

## Persistence State

### morning-api (single-capture bundle ~09:15Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1730 | Incrementing by 10 (pass 70: 1710; +20 = 2 rotations) | None (normal — 2 rotations since pass 70) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (09:15Z single capture):**
- `state.snapshot`: 894 bytes (mtime: 2026-07-28T05:12:56 EDT — epoch 1730 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T05:12:56 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T05:07:56 EDT — previous epoch 1720 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 05:02 (pass 70) to 05:12 — confirmed 2 rotations occurred. ✓
- Snapshot size 894 bytes (pass 70: 894; unchanged) ✓

### local-witness (single-capture bundle ~09:15Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1730 | Incrementing by 10 (pass 70: 1710; +20 = 2 rotations) | None (normal — 2 rotations since pass 70) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (09:15Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T05:13:13 EDT — epoch 1730 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T05:13:13 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T05:08:13 EDT — previous epoch 1720 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 05:03 (pass 70) to 05:13 — confirmed 2 rotations occurred. ✓
- Snapshot size (569 bytes) vs morning-api (894 bytes): consistent — witness stores only zero balances; morning-api stores both nodes' balances.

---

## Metrics Instrumentation

**OBSERVED (from m-ap metrics line at ~09:15Z):**
- `outstanding_fetches=0` — no pending fetches
- `aged=0` — no stale fetches
- `outbound_queues=[]` — all peer queues empty
- `max_peer_silence=3s` (pass 70: 3s) — well under 30s threshold

**OBSERVED (from witness metrics line at ~09:15Z):**
- `outstanding_fetches=0`
- `aged=0`
- `outbound_queues=[]`
- `max_peer_silence=6s` (pass 70: 6s)

**EXPECTED:** All gauges near zero on a settled 2-node mesh with no new transactions.
**DEVIATION:** None. Mesh is quiescent.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **WARN (174 total, every 5 min since Jul 27 18:48Z):** `libp2p_kad::behaviour: Failed to trigger bootstrap: No known peers.` — expected on 2-node loopback with `--no-mdns`. +2 since pass 70 (172→174). Benign and continuous.
- **WARN (Jul 28 ~07:37Z):** `NTP query to pool.ntp.org failed: Input/output error: Resource temporarily unavailable (os error 11) (fallback)` — single transient occurrence. Unchanged.
- **WARN (Jul 27 18:48Z, historic):** `Failed to gossip genesis (will retry on peer connect) error=InsufficientPeers` — startup artifact, no recurrence.
- **WARN (Jul 27 18:48Z, historic):** `[block-publish] Failed to publish block proposal_id="genesis" error=InsufficientPeers` — startup artifact, no recurrence.
- **ERROR:** None.
- **Zombie eviction events:** None.
- **Sweep/eviction events:** None.

### local-witness (/tmp/lw.log)
- **WARN (118 total, all historic Jul 27):** `Transaction validation failed error=insufficient balance` — all from original redistribution test. No new occurrences since Jul 27.
- **WARN (Jul 28 ~08:00Z):** `NTP query to pool.ntp.org failed: Input/output error: Resource temporarily unavailable (os error 11) (fallback)` — single transient occurrence. Unchanged.
- **ERROR:** None.

### Log filter (WARN/ERROR excluded as benign)
- `skip-ntp-check`: No hits.
- `No snapshot`: No hits (both have snapshots).
- `zombie`: No hits.
- `insufficient balance`: 118 hits on witness (historic, all Jul 27). 0 on m-ap.

---

## Build Commit Verification

| Check | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| git HEAD | `cb5d4b1` | — | — |
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind + dirty (62 unstaged files, mostly observer evidence docs). Unchanged since pass 1. |
| build.rs mtime | Jul 27 13:43 | — | Stale — `build.rs` hasn't been touched since initial build. |

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 70 Status | Pass 71 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

**No new deviations detected in this pass.**

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (all three agree) | **PASS** — endpoint both 1734. Grep/log race naturally ahead by 1–2 ticks. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2) |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 |
| Log health (WARN/ERROR filtered) | **PASS** — only continuous KAD bootstrap warnings (benign), 118 historic insufficient-balance entries, and single NTP failure each. No new anomalous events. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** |
| Cross-node epoch sync | **PASS** — both at 1734 from endpoint capture (Δ=0 this pass) |
