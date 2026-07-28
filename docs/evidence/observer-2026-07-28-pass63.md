# Observer Evidence Record — 2026-07-28 (Pass 63)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T07:53:53–07:54:13Z bundle (simultaneous capture)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Sixty-third observation pass. Same processes since 2026-07-27T18:48Z (~13.1h runtime). ~10 min since pass 62 (07:44Z).

**Summary:** Routine continuation. Both nodes at epoch 1571/1571 (simultaneous capture, no gap). Snapshot epoch advanced 1550→1570 (+2 rotations). Metrics healthy: all queues empty, silence <10s, aged=0, no zombie evictions. All three persistent deviations unchanged. No new deviations detected.

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
| uptime_secs | 47127 (~13.09h) | — | None (pass 62: 46537; Δ = +590s ≈ 9.8 min — covers bundle interval) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 987.48 | ~1000, slowly decaying | None (pass 62: 987.63; Δ = −0.15 over ~10 min — consistent decay ~0.015/min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 47120 (~13.09h) | — | None (pass 62: 46532; Δ = +588s ≈ 9.8 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED (07:53:53Z):** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=4711, silence_secs=8, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED (07:54:13Z):** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=4713, silence_secs=5, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 62 (~07:44Z):** Heartbeats api +59 (4652→4711), witness +59 (4654→4713). Both ~5.9/min (~10 min window). Silence: api 8s, witness 5s — both well within threshold (<30s). Queue depth 0 on both.

**No zombie eviction events** detected in either log since last pass.

---

## Epoch State

### morning-api (single-capture bundle ~07:53:53Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1571 (socket); 1572 (log grep); 1572 (last log line) | Cycling. +17–18 since pass 62 (1554→1571/1572) in ~10 min. | **Minor.** Socket vs log differ by 1 — race at epoch boundary (log line fired between query and grep). Within capture interval, not a real divergence. |
| ratio | 1.019977 | ~1.01–1.02 steady state | None (pass 62: 1.019977; Δ = 0 — unchanged from pass 62 value) |
| tax_calculated | 0 | Balance 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (single-capture bundle ~07:54:13Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1571 (socket); 1571 (log grep); 1571 (last log line) | Cycling. +18 since pass 62 (1553→1571) in ~10 min. | **None.** Three-way unanimous. |
| ratio | 1.065045 | Continuing asymptotic decline | None (pass 62: 1.065694; Δ = −0.000649 over ~10 min — normal decline) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** Both nodes at epoch 1571 in simultaneous capture. Morning-api's log count was 1572 (1-ahead due to mid-capture epoch boundary crossing). The core state — each node's recorded epoch — is in sync at 1571/1571.

### Epoch cadence
+17–18 epochs since pass 62 (~07:44Z) in ~10 min ≈ 33–35s/epoch. Slightly slower than pass 62 (31s/epoch) but within normal variance.

### Three-way epoch check
- **morning-api:** Socket=1571, grep=1572, last_log=1572. **MINOR — 1-epoch race at boundary** (log line fired mid-capture). Not a deviation.
- **local-witness:** Socket=1571, grep=1571, last_log=1571. **PASS — unanimous.**

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
**UNKNOWN:** The witness nonce advanced from 0 to 2 (as of this pass). Two transactions originated from the witness but neither appears in morning-api's peer nonce (which reports 0). The nature of these transactions is unknown — they may be internal-only operations or data that did not propagate.

---

## Persistence State

### morning-api (single-capture bundle ~07:53:53Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1570 | Incrementing by 10 (pass 62: 1550; +20 = 2 rotations) | None (normal — 2 rotations since pass 62) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (07:53:53Z single capture):**
- `state.snapshot`: 895 bytes (mtime: 2026-07-28T03:52 EDT — epoch 1570 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T03:52 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T03:42 EDT — previous epoch 1560 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 03:42 (pass 62) to 03:52 — confirmed 2 rotations occurred. ✓
- `wal.wal.old` naming: noted cosmetic (expected `wal.log.old`). Known-provisional.

### local-witness (sequential capture ~07:54:13Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1570 | Incrementing by 10 (pass 62: 1550; +20 = 2 rotations) | None (normal — 2 rotations since pass 62) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (07:54:13Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T03:53 EDT — epoch 1570 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T03:53 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T03:43 EDT — previous epoch 1560 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 03:43 (pass 62) to 03:53 — confirmed 2 rotations. ✓
- Snapshot size (569 bytes) vs morning-api (895 bytes): The witness snapshot is smaller because it stores only zero balances; morning-api stores both nodes' balances (20 + 4980).

---

## Metrics Instrumentation

**OBSERVED (from m-ap metrics lines at ~07:53Z):**
- `outstanding_fetches=0` — no pending fetches
- `aged=0` — no stale fetches
- `outbound_queues=[]` — all peer queues empty
- `max_peer_silence=3s` pass 62 → 8s now. Still well under 30s threshold. Slight increase but no cause for concern.

**EXPECTED:** All gauges near zero on a settled 2-node mesh with no transactions.
**DEVIATION:** None. Mesh is quiescent.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **WARN, Genesis-time (Jul 27 18:48Z):** `Failed to gossip genesis (will retry on peer connect) error=InsufficientPeers` — startup artifact, no recurrence.
- **WARN, Genesis-time:** `[block-publish] Failed to publish block proposal_id="genesis" error=InsufficientPeers` — startup artifact, no recurrence.
- **WARN, Recurrent (every 5 min):** `libp2p_kad::behaviour: Failed to trigger bootstrap: No known peers.` — ~180 total (+~15 since pass 62). Benign on a 2-node `--no-mdns` explicit-bootstrap mesh.
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

| # | Deviation | First Observed | Pass 62 Status | Pass 63 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

**No new deviations detected in this pass.**

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (all three agree) | **PARTIAL PASS** — witness unanimous (1571/1571/1571). Morning-api has 1-epoch race (1571 vs 1572/1572) — within capture window, not real divergence. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2) |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 |
| Log health (WARN/ERROR filtered) | **PASS** — only benign kad bootstrap warnings, historic Jul 27 transactions, and one transient NTP failure |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** |
| Cross-node simultaneous capture | **PASS** — both nodes queried within 20s window; epoch state matches at 1571/1571 |
