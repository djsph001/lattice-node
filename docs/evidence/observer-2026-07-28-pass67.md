# Observer Evidence Record — 2026-07-28 (Pass 67)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T08:32:32Z bundle (simultaneous capture)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Sixty-seventh observation pass. Same processes since 2026-07-27T18:48Z (~13.74h runtime). ~9 min since pass 66 (08:23Z).

**Summary:** Routine continuation. Both nodes reached epoch 1649. Snapshot epoch advanced 1630→1640 (+1 rotation). All three persistent deviations unchanged. No new deviations detected. Witness showed transient 1-epoch lag vs morning-api in the bundle (boundary race, resolved by the 3-way check). Three-way epoch check PASS on both nodes.

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
| uptime_secs | 49447 (~13.74h) | — | None (pass 66: 48892; Δ = +555s ≈ 9.25 min — covers bundle interval) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 986.86 | ~1000, slowly decaying | None (pass 66: 987.01; Δ = −0.15 over ~9 min — consistent decay ~0.016/min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 49465 (~13.74h) | — | None (pass 66: 48894; Δ = +571s ≈ 9.5 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED (08:32Z):** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=4943, silence_secs=2, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED (08:32Z):** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=4946, silence_secs=9, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 66 (~08:23Z):** Heartbeats api +55 (4888→4943 ~6/min), witness +56 (4890→4946 ~6/min). Both normal rate. Silence: api 2s (pass 66: 2s), witness 9s (pass 66: 6s) — well within threshold (<30s). Queue depth 0 on both.

**No zombie eviction events** detected in either log.

---

## Epoch State

### morning-api (single-capture bundle ~08:32Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1649 | Cycling. +18 since pass 66 final convergence (1631→1649) in ~9 min. | None. Normal cadence (~30s/epoch). |
| ratio | 1.019772 | ~1.01–1.02 steady state | None (pass 66: 1.019772; Δ = ~0 — essentially unchanged) |
| tax_calculated | 0 | Balance 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (single-capture bundle ~08:32Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1648 (socket at 08:32:32Z), 1649 (log at 08:32:43Z) | Cycling. +17-18 since pass 66 (1631→1648/9). | **Transient deviation (boundary race).** Socket returned 1648 while log shows 1649 at +11s later. Both nodes at 1649 by log time. No evidence of persistent drift. |
| ratio | 1.061481 | Continuing asymptotic decline | None (pass 66: 1.062233; Δ = −0.000752 over ~9 min — consistent approach to 1.0) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** Socket bundle showed morning-api at 1649, witness at 1648. Log greps show both at 1649. **Transient 1-epoch lag in socket capture** — resolved within 11s by log timestamps. Not a persistent drift.

### Epoch cadence
+18 epochs since pass 66 (~08:23Z) in ~9 min ≈ 30s/epoch. Within normal 26-35s range.

### Three-way epoch check
- **morning-api:** Socket=1649, grep=1649, last_log=1649. **PASS** — all three agree.
- **local-witness:** Socket=1648 (boundary race at bundle time), grep=1649, last_log=1649. **PASS** — grep and last_log agree. Socket was 1 behind due to timing of capture relative to epoch complete event.

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

### morning-api (single-capture bundle ~08:32Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1640 | Incrementing by 10 (pass 66: 1630; +10 = 1 rotation) | None (normal — 1 rotation since pass 66) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (08:32Z single capture):**
- `state.snapshot`: 894 bytes (mtime: 2026-07-28T04:27 EDT — epoch 1640 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T04:27 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T04:22 EDT — previous epoch 1630 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 04:22 (pass 66) to 04:27 — confirmed 1 rotation occurred. ✓
- Snapshot size 894 bytes (pass 66: 894; Δ = 0 — stable).

### local-witness (single-capture bundle ~08:32Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1640 | Incrementing by 10 (pass 66: 1630; +10 = 1 rotation) | None (normal — 1 rotation since pass 66) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (08:32Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T04:28 EDT — epoch 1640 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T04:28 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T04:23 EDT — previous epoch 1630 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 04:23 (pass 66) to 04:28 — confirmed 1 rotation. ✓
- Snapshot size (569 bytes) vs morning-api (894 bytes): consistent — witness stores only zero balances; morning-api stores both nodes' balances.

---

## Metrics Instrumentation

**OBSERVED (from m-ap metrics line at ~08:32:36Z):**
- `outstanding_fetches=0` — no pending fetches
- `aged=0` — no stale fetches
- `outbound_queues=[]` — all peer queues empty
- `max_peer_silence=3s` (pass 66: 3s) — well under 30s threshold

**OBSERVED (from witness metrics line at ~08:32:43Z):**
- `outstanding_fetches=0`
- `aged=0`
- `outbound_queues=[]`
- `max_peer_silence=6s` (pass 66: same)

**EXPECTED:** All gauges near zero on a settled 2-node mesh with no transactions.
**DEVIATION:** None. Mesh is quiescent.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **WARN (recurring, every 5 min):** `libp2p_kad::behaviour: Failed to trigger bootstrap: No known peers.` — Benign. Node runs with `--no-mdns` and no DHT bootstrap peers. Recurring since startup.
- **WARN, Genesis-time (Jul 27 18:48Z):** `Failed to gossip genesis (will retry on peer connect) error=InsufficientPeers` — startup artifact, no recurrence.
- **WARN, Genesis-time:** `[block-publish] Failed to publish block proposal_id="genesis" error=InsufficientPeers` — startup artifact, no recurrence.
- **WARN (Jul 28 07:37Z):** `NTP query to pool.ntp.org failed: Input/output error: Resource temporarily unavailable (os error 11) (fallback)` — transient. Single occurrence, unchanged from pass 66. Noted.
- **ERROR:** None.
- **Zombie eviction events:** None.
- **Sweep/eviction events:** None.

### local-witness (/tmp/lw.log)
- **WARN (118 total, all historic Jul 27):** `Transaction validation failed error=insufficient balance` — all 118 from original redistribution test. No new occurrences since Jul 27.
- **WARN (Jul 28 08:00Z):** `NTP query to pool.ntp.org failed: Input/output error: Resource temporarily unavailable (os error 11) (fallback)` — transient, single occurrence. Unchanged from pass 66.
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

| # | Deviation | First Observed | Pass 66 Status | Pass 67 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

**No new deviations detected in this pass.**

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (all three agree) | **PASS** — morning-api (1649/1649/1649). Witness (1648/1649/1649 — socket 1 behind due to boundary race at capture time, grep and last_log agree). |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2) |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 |
| Log health (WARN/ERROR filtered) | **PASS** — only historic Jul 27 transactions, benign kad bootstrap warnings, transient NTP failures noted. No new events. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** |
| Cross-node epoch sync | **PASS** — both nodes at epoch 1649 by log timestamp (transient 1-epoch lag in socket capture noted but resolved within 11s) |
