# Observer Evidence Record — 2026-07-28 (Pass 64)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T08:03:30–08:08:00Z bundle (simultaneous capture)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Sixty-fourth observation pass. Same processes since 2026-07-27T18:48Z (~13.3h runtime). ~14 min since pass 63 (07:53Z).

**Summary:** Routine continuation. Both nodes reached epoch 1598. Snapshot epoch advanced 1570→1590 (+2 rotations). All three persistent deviations unchanged. No new deviations detected. One transient NTP failure on witness.

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

**Other sockets present but NOT part of this mesh** (historic test nodes): `/tmp/gr-an/`, `/tmp/lv-quick/`, `/tmp/as/`, `/tmp/as3/`, `/tmp/ktr/`, `/tmp/witness-b/`, `/tmp/lv-none/`, `/tmp/m-an/`, `/tmp/bc/`, `/tmp/ktz/`, `/tmp/witness-a/`, `/tmp/as2/`, `/tmp/genesis-test/`, `/tmp/lv-an/`, `/tmp/gr-ap/`, `/tmp/api-live/`, `/tmp/kta/`, `/tmp/as1/`, plus `~/Projects/lattice-node/lattice-storage-*`.

**Clock:** System clock synchronized via NTP. No drift detected.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 47930 (~13.31h) | — | None (pass 63: 47127; Δ = +803s ≈ 13.4 min — covers bundle interval) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 987.26 | ~1000, slowly decaying | None (pass 63: 987.48; Δ = −0.22 over ~14 min — consistent decay ~0.016/min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 47915 (~13.31h) | — | None (pass 63: 47120; Δ = +795s ≈ 13.25 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED (08:03Z):** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=4768, silence_secs=9, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED (08:04Z):** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=4782, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 63 (~07:53Z):** Heartbeats api +57 (4711→4768), witness +69 (4713→4782). Both ~5–6/min consistent with pass 63. Silence: api 9s (pass 63: 8s), witness 3s (pass 63: 5s) — both well within threshold (<30s). Queue depth 0 on both.

**No zombie eviction events** detected in either log since last pass.

---

## Epoch State

### morning-api (single-capture bundle ~08:03:30Z → re-query at ~08:08:00Z confirms epoch 1598)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1591 (socket @ 08:03:30); 1598 (socket @ 08:08:00); 1597 (log grep); 1597 (last log line) | Cycling. +20–27 since pass 63 (1571→1591/1598) in ~14 min. | **Minor.** The 6-epoch initial gap (socket 1591 vs log count 1597) was a timing artifact — initial query at 08:03:30 captured epoch mid-cycle; by 08:08 both socket and log had converged. Within capture interval, not a real divergence. |
| ratio | 1.019765 | ~1.01–1.02 steady state | None (pass 63: 1.019977; Δ = −0.000212 — normal decline) |
| tax_calculated | 0 | Balance 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (single-capture bundle ~08:04:00Z → re-query at ~08:08:00Z confirms epoch 1598)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1594 (socket @ 08:04:00); 1598 (socket @ 08:08:00); 1596 (log grep); 1596 (last log line) | Cycling. +23–27 since pass 63 (1571→1594/1598) in ~14 min. | **Minor.** Initial gap (socket 1594 vs log count 1596) was a 2-epoch timing artifact. By 08:08 socket and log converged. |
| ratio | 1.063789 | Continuing asymptotic decline | None (pass 63: 1.065694; Δ = −0.001905 over ~14 min — faster decline, but consistent with approach to 1.0) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** Both nodes reached epoch 1598 by the time of re-query (~08:08:00Z). Within capture interval, the 3–4 epoch gap between m-ap (1591→1598) and witness (1594→1598) was a timing artifact of sequential queries across a fast-cycling mesh.

### Epoch cadence
+27 epochs since pass 63 (~07:53Z) in ~14 min ≈ 31s/epoch. Consistent with normal rate (31–35s/epoch). No cadence anomaly.

### Three-way epoch check
- **morning-api:** Socket=1598, grep=1597 (as of 08:06), last_log=1597. **MINOR — 1-epoch race at boundary.** Expected.
- **local-witness:** Socket=1598, grep=1596 (as of 08:06), last_log=1596. **MINOR — 2-epoch race.** Both within capture window, not real divergence.

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
| own_nonce | 2 | 2 | None (unchanged since pass 63) |
| morning_api_balance (reported) | 0 | 5000 | **Persistent DEVIATION.** Witness reports morning-api balance as 0. First observed: observer pass 1 (Jul 27 18:48Z). Supply conservation: CONTRADICTED per Verifier Mission 1. |

### Supply divergence
**OBSERVED:** morning-api sees total supply = 20 + 4980 = 5000. Witness sees total supply = 0 + 0 = 0.
**EXPECTED (proposed invariant):** Sum of all spendable balances across mesh = 5000.
**DEVIATION:** Witness-side accounting reports 0. Known-deviating since first observer pass. Unchanged.
**UNKNOWN:** Witness nonce = 2 (unchanged since pass 63). Two transactions originated from witness. Nature unknown — may be internal-only or non-propagating.

---

## Persistence State

### morning-api (single-capture bundle ~08:03:30Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1590 | Incrementing by 10 (pass 63: 1570; +20 = 2 rotations) | None (normal — 2 rotations since pass 63) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (08:03:30Z single capture):**
- `state.snapshot`: 895 bytes (mtime: 2026-07-28T04:02 EDT — epoch 1590 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T04:02 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T03:57 EDT — previous epoch 1580 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 03:52 (pass 63) to 04:02 — confirmed 2 rotations occurred. ✓
- `wal.wal.old` naming: noted cosmetic (expected `wal.log.old`). Known-provisional.

### local-witness (sequential capture ~08:04:00Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1590 | Incrementing by 10 (pass 63: 1570; +20 = 2 rotations) | None (normal — 2 rotations since pass 63) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (08:04:00Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T04:03 EDT — epoch 1590 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T04:03 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T03:58 EDT — previous epoch 1580 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 03:53 (pass 63) to 04:03 — confirmed 2 rotations. ✓
- Snapshot size (569 bytes) vs morning-api (895 bytes): consistent — witness stores only zero balances; morning-api stores both nodes' balances.

---

## Metrics Instrumentation

**OBSERVED (from m-ap metrics line at ~08:06Z):**
- `outstanding_fetches=0` — no pending fetches
- `aged=0` — no stale fetches
- `outbound_queues=[]` — all peer queues empty
- `max_peer_silence=3s` (pass 63: 8s) — decreased; well under 30s threshold

**EXPECTED:** All gauges near zero on a settled 2-node mesh with no transactions.
**DEVIATION:** None. Mesh is quiescent.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **WARN, Genesis-time (Jul 27 18:48Z):** `Failed to gossip genesis (will retry on peer connect) error=InsufficientPeers` — startup artifact, no recurrence.
- **WARN, Genesis-time:** `[block-publish] Failed to publish block proposal_id="genesis" error=InsufficientPeers` — startup artifact, no recurrence.
- **WARN, Recurrent (every 5 min):** `libp2p_kad::behaviour: Failed to trigger bootstrap: No known peers.` — ~190 total (+~10 since pass 63). Benign on a 2-node `--no-mdns` explicit-bootstrap mesh.
- **ERROR:** None.
- **Zombie eviction events:** None.
- **Sweep/eviction events:** None.

### local-witness (/tmp/lw.log)
- **WARN (118 total, all historic Jul 27):** `Transaction validation failed error=insufficient balance` — all 118 from the original redistribution test (Verifier Mission 1). No new occurrences since Jul 27.
- **WARN (Single, new since pass 63):** `NTP query to pool.ntp.org failed: Input/output error: Resource temporarily unavailable (os error 11) (fallback)` — recorded at 2026-07-28T08:00:06Z. Transient failure, similar to pass 63's NTP warning on morning-api. No recurrence.

### Log filter (WARN/ERROR excluded as benign)
- `skip-ntp-check`: No hits.
- `No snapshot`: No hits (both have snapshots).
- `zombie`: No hits.
- `insufficient balance`: 118 hits on witness (historic, all Jul 27). 0 on m-ap.

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 63 Status | Pass 64 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

**No new deviations detected in this pass.**

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (all three agree) | **PARTIAL PASS** — both nodes within 1–2 epoch race at boundary. Socket vs log gap resolved on re-query. Within capture window, not real divergence. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2) |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 |
| Log health (WARN/ERROR filtered) | **PASS** — only benign kad bootstrap warnings, historic Jul 27 transactions, and one transient NTP failure on witness |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** |
| Cross-node simultaneous capture | **PASS** — both nodes confirmed at epoch 1598 on re-query |
