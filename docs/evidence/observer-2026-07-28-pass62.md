# Observer Evidence Record — 2026-07-28 (Pass 62)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** 2026-07-28T07:44Z bundle (sequential queries ~07:43–07:45Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Sixty-second observation pass. Same processes since 2026-07-27T18:48Z (~13.0h runtime). ~9 min since pass 61 (07:36Z).

**Summary:** Routine continuation. Both nodes at epoch 1554/1553 (sequential capture, 1-epoch gap). Three-way epoch unanimous within each node. Snapshot epoch advanced 1530→1550 (+2 rotations since pass 61). Metrics healthy: all queues empty, silence <5s, aged=0, no zombie evictions. All three persistent deviations unchanged. No new deviations detected.

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
| uptime_secs | 46537 (~12.93h) | — | None (pass 61: 45911; Δ = +626s ≈ 10.4 min — covers full bundle) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 987.63 | ~1000, slowly decaying | None (pass 61: 987.79; Δ = −0.16 over ~9 min — consistent decay ~0.018/min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 46532 (~12.93h) | — | None (pass 61: 45951; Δ = +581s ≈ 9.7 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |
| thickness | N/A | — | N/A (witness does not report thickness via GetNodeInfo) |

---

## Peer Connections

### morning-api
**OBSERVED (first query ~07:44Z):** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=4652, silence_secs=2, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED (sequential query ~07:44Z):** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=4654, silence_secs=1, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 61:** Heartbeats api +63 (4589→4652), witness +58 (4596→4654). Both ~6.5–7.0/min (~9 min window). Silence: api 2s, witness 1s — both well within threshold (<30s). Queue depth 0 on both.

**No zombie eviction events** detected in either log since last pass.

---

## Epoch State

### morning-api (single-capture bundle ~07:44Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1554 (socket); 1554 (log grep); 1554 (last log line) | Cycling. +17 since pass 61 (1537→1554) in ~9 min. | **None.** Three-way unanimous. |
| ratio | 1.019977 | ~1.01–1.02 steady state | None (pass 61: 1.019756; Δ = +0.000221 — minimal floating drift) |
| tax_calculated | 0 | Balance 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (sequential capture ~07:44Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1553 (socket); 1553 (log grep); 1553 (last log line) | Cycling. +17 since pass 61 (1536→1553) in ~9 min. | **None.** Three-way unanimous. 1‑epoch gap from morning-api is sequential capture artifact. |
| ratio | 1.065694 | Continuing asymptotic decline | None (pass 61: 1.066447; Δ = −0.000753 over ~9 min — normal decline) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** morning-api at 1554, witness at 1553 during single-capture bundle (sequential capture — api queried first, witness second, 1 epoch elapsed between). Both log greps confirm unanimous agreement within each node's capture.

### Epoch cadence
+17 epochs since pass 61 (~07:36Z) in ~9 min ≈ 31s/epoch. Both nodes consistent. Within normal variance (28–35s observed range).

### Three-way epoch check
- **morning-api:** Socket=1554, grep=1554, last_log=1554. **PASS — unanimous.**
- **local-witness:** Socket=1553, grep=1553, last_log=1553. **PASS — unanimous.**

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
| own_nonce | 2 | 2 | None |
| morning_api_balance (reported) | 0 | 5000 | **Persistent DEVIATION.** Witness reports morning-api balance as 0. First observed: observer pass 1 (Jul 27 18:48Z). Supply conservation: CONTRADICTED per Verifier Mission 1. |

### Supply divergence
**OBSERVED:** morning-api sees total supply = 20 + 4980 = 5000. Witness sees total supply = 0 + 0 = 0.
**EXPECTED (proposed invariant):** Sum of all spendable balances across mesh = 5000.
**DEVIATION:** Witness-side accounting reports 0. Known-deviating since first observer pass. Unchanged.

---

## Persistence State

### morning-api (single-capture bundle ~07:44Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1550 | Incrementing by 10 (pass 61: 1530; +20 = 2 rotations) | None (normal — 2 rotations since pass 61) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (07:44Z single capture):**
- `state.snapshot`: 895 bytes (mtime: 2026-07-28T03:42 EDT — epoch 1550 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T03:42 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T03:37 EDT — previous epoch 1540 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 03:32 (pass 61) to 03:42 — confirmed 2 rotations occurred.
- `wal.wal.old` naming: noted cosmetic (expected `wal.log.old`). Known-provisional.

### local-witness (sequential capture ~07:44Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1550 | Incrementing by 10 (pass 61: 1530; +20 = 2 rotations) | None (normal — 2 rotations since pass 61) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (07:44Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T03:43 EDT — epoch 1550 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T03:43 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T03:38 EDT — previous epoch 1540 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 03:33 (pass 61) to 03:43 — confirmed 2 rotations.

---

## Metrics Instrumentation

**OBSERVED (from m-ap metrics lines at ~07:44Z):**
- `outstanding_fetches=0` — no pending fetches
- `aged=0` — no stale fetches
- `outbound_queues=[]` — all peer queues empty
- `max_peer_silence=3s` — last heartbeat from witness 3 seconds ago
- No sweep/eviction events logged (zero stale entries to evict)

**EXPECTED:** All gauges near zero on a settled 2-node mesh with no transactions.
**DEVIATION:** None. Mesh is quiescent — no transactions flowing, no fetch pressure, no queue buildup.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **WARN, Genesis-time (Jul 27 18:48Z):** `Failed to gossip genesis (will retry on peer connect) error=InsufficientPeers` — startup artifact, no recurrence.
- **WARN, Genesis-time:** `[block-publish] Failed to publish block proposal_id="genesis" error=InsufficientPeers` — startup artifact, no recurrence.
- **WARN, Recurrent (every 5 min):** `libp2p_kad::behaviour: Failed to trigger bootstrap: No known peers.` — ~165 total (+~11 since pass 61). Benign on a 2-node `--no-mdns` explicit-bootstrap mesh.
- **WARN, Single:** `NTP query to pool.ntp.org failed: Input/output error` (07:37:59Z) — transient failure; no recurrence.
- **ERROR:** None.
- **Zombie eviction events:** None.
- **Sweep/eviction events:** None.

### local-witness (/tmp/lw.log)
- **WARN (118 total, all historic Jul 27):** `Transaction validation failed error=insufficient balance` — all 118 from the original redistribution test (Verifier Mission 1). No new occurrences since Jul 27.

### Log filter (WARN/ERROR excluded as benign)
- `skip-ntp-check`: No hits (neither node uses this flag).
- `No snapshot`: No hits (both have snapshots).
- `zombie`: No hits.
- `insufficient balance`: 118 hits on witness (historic, all Jul 27). 0 on m-ap.

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 61 Status | Pass 62 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

**No new deviations detected in this pass.**

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (all three agree) | **PASS** — both nodes unanimous internally (1554/1554/1554 api; 1553/1553/1553 witness). 1-epoch gap between nodes is sequential capture artifact. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2) |
| Sequential capture artifact (apparent desync) | Noted — 1-epoch gap between nodes; each node's own data is self-consistent |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 |
| Log health (WARN/ERROR filtered) | **PASS** — only benign kad bootstrap warnings and historic Jul 27 transactions |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** |
