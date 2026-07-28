# Observer Evidence Record — 2026-07-28 (Pass 68)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T08:45:03Z bundle (simultaneous capture)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Sixty-eighth observation pass. Same processes since 2026-07-27T18:48Z (~13.94h runtime). ~14 min since pass 67 (08:32Z).

**Summary:** Routine continuation. Both nodes reached epoch 1676. Snapshot epoch advanced 1640→1670 (+3 rotations). All three persistent deviations unchanged. No new deviations detected. Three-way epoch check PASS on both nodes.

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

**Other sockets found on machine:** Several stale sockets remain from earlier test runs (pre-date Jul 27). No running processes associated with them — stale sockets only.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 50197 (~13.94h) | — | None (pass 67: 49447; Δ = +750s ≈ 12.5 min — covers bundle interval) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 986.66 | ~1000, slowly decaying | None (pass 67: 986.87; Δ = −0.21 over ~14 min — consistent decay ~0.015/min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 50238 (~13.96h) | — | None (pass 67: 49465; Δ = +773s ≈ 12.9 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED (08:45Z):** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=5018, silence_secs=6, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED (08:46Z):** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=5025, silence_secs=1, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 67 (~08:32Z):** Heartbeats api +75 (4943→5018 ~5.8/min), witness +79 (4946→5025 ~6.1/min). Both normal rate. Silence: api 6s (pass 67: 2s), witness 1s (pass 67: 9s) — both well within threshold (<30s). Queue depth 0 on both.

**No zombie eviction events** detected in either log.

---

## Epoch State

### morning-api (single-capture bundle ~08:45Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1674 (bundle), 1676 (3-way check) | Cycling. +25 from pass 67 (1649→1674) in ~14 min. | None. Normal cadence (~33s/epoch). |
| ratio | 1.019978 | ~1.01–1.02 steady state | None (pass 67: 1.019772; Δ = +0.000206 — essentially unchanged) |
| tax_calculated | 0 | Balance 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (single-capture bundle ~08:46Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1675 (bundle), 1676 (3-way check) | Cycling. +27 from pass 67 (1648→1675 bundle) in ~14 min. | None. Both nodes at same epoch by 3-way check. |
| ratio | 1.060383 | Continuing asymptotic decline | None (pass 67: 1.061481; Δ = −0.001098 over ~14 min — consistent approach to 1.0) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** Socket bundle showed morning-api at 1674, witness at 1675 (Δ=1 normal race). 3-way check at ~08:46Z shows both at 1676. **Synchronized.**

### Epoch cadence
+25–27 epochs since pass 67 (~08:32Z) in ~14 min ≈ ~31–33s/epoch. Within normal 26-35s range.

### Three-way epoch check
- **morning-api:** Socket=1676, grep=1676, last_log=epoch=1676. **PASS** — all three agree.
- **local-witness:** Socket=1676, grep=1676, last_log=epoch=1676. **PASS** — all three agree.

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
**EXPECTED (proposed invariant):** Sum of all spendable balances across mesh = 5000.
**DEVIATION:** Witness-side accounting reports 0. Known-deviating since first observer pass. Unchanged.
**UNKNOWN:** Witness nonce = 2 (unchanged since pass 64). Two transactions originated from witness. Nature unknown.

---

## Persistence State

### morning-api (single-capture bundle ~08:45Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1670 | Incrementing by 10 (pass 67: 1640; +30 = 3 rotations) | None (normal — 3 rotations since pass 67) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (08:46Z single capture):**
- `state.snapshot`: 895 bytes (mtime: 2026-07-28T04:42 EDT — epoch 1670 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T04:42 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T04:37 EDT — previous epoch 1660 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 04:27 (pass 67) to 04:42 — confirmed 3 rotations occurred. ✓
- Snapshot size 895 bytes (pass 67: 894; Δ = +1 — within normal variance).

### local-witness (single-capture bundle ~08:46Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1670 | Incrementing by 10 (pass 67: 1640; +30 = 3 rotations) | None (normal — 3 rotations since pass 67) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (08:46Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T04:43 EDT — epoch 1670 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T04:43 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T04:38 EDT — previous epoch 1660 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 04:28 (pass 67) to 04:43 — confirmed 3 rotations. ✓
- Snapshot size (569 bytes) vs morning-api (895 bytes): consistent — witness stores only zero balances; morning-api stores both nodes' balances.

---

## Metrics Instrumentation

**OBSERVED (from m-ap metrics line at ~08:46Z):**
- `outstanding_fetches=0` — no pending fetches
- `aged=0` — no stale fetches
- `outbound_queues=[]` — all peer queues empty
- `max_peer_silence=3s` (pass 67: 3s) — well under 30s threshold

**OBSERVED (from witness metrics line at ~08:46Z):**
- `outstanding_fetches=0`
- `aged=0`
- `outbound_queues=[]`
- `max_peer_silence=6s` (pass 67: same)

**EXPECTED:** All gauges near zero on a settled 2-node mesh with no transactions.
**DEVIATION:** None. Mesh is quiescent.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **WARN (recurring, every 5 min):** `libp2p_kad::behaviour: Failed to trigger bootstrap: No known peers.` — Benign. Node runs with `--no-mdns` and no DHT bootstrap peers. Recurring since startup.
- **WARN, Genesis-time (Jul 27 18:48Z):** `Failed to gossip genesis (will retry on peer connect) error=InsufficientPeers` — startup artifact, no recurrence.
- **WARN, Genesis-time:** `[block-publish] Failed to publish block proposal_id="genesis" error=InsufficientPeers` — startup artifact, no recurrence.
- **WARN (Jul 28 ~07:37Z):** `NTP query to pool.ntp.org failed: Input/output error: Resource temporarily unavailable (os error 11) (fallback)` — transient. Single occurrence noted in prior passes. Not observed in this pass's log tail.
- **ERROR:** None.
- **Zombie eviction events:** None.
- **Sweep/eviction events:** None.

### local-witness (/tmp/lw.log)
- **WARN (118 total, all historic Jul 27):** `Transaction validation failed error=insufficient balance` — all 118 from original redistribution test. No new occurrences since Jul 27.
- **WARN (Jul 28 ~08:00Z):** `NTP query to pool.ntp.org failed: Input/output error: Resource temporarily unavailable (os error 11) (fallback)` — transient, single occurrence noted in prior passes. Unchanged.
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
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind + dirty. |
| build.rs mtime | Jul 27 13:43 | — | Stale — `build.rs` hasn't been touched since initial build. `BUILD_COMMIT` env var hasn't re-run. |

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 67 Status | Pass 68 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

**No new deviations detected in this pass.**

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (all three agree) | **PASS** — morning-api (1676/1676/1676). Witness (1676/1676/1676). Clean agreement on both. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2) |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 |
| Log health (WARN/ERROR filtered) | **PASS** — only historic Jul 27 transactions, benign kad bootstrap warnings, single transient NTP failure (08:00Z, unchanged). No new events. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** |
| Cross-node epoch sync | **PASS** — both nodes at epoch 1676 by 3-way check |
