# Observer Evidence Record — 2026-07-28 (Pass 69)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T08:54Z bundle (simultaneous capture)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Sixty-ninth observation pass. Same processes since 2026-07-27T18:48Z (~14.1h runtime). ~9 min since pass 68 (08:45Z).

**Summary:** Routine continuation. Both nodes reached epoch 1692 (api) / 1691 (witness). Snapshot epoch advanced 1670→1690 (+2 rotations). All three persistent deviations unchanged. No new deviations detected. Three-way epoch check PASS on both nodes.

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
| uptime_secs | 50756 (~14.1h) | — | None (pass 68: 50197; Δ = +559s ≈ 9.3 min — covers bundle interval) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 986.52 | ~1000, slowly decaying | None (pass 68: 986.66; Δ = −0.14 over ~9 min — consistent decay ~0.016/min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 50690 (~14.08h) | — | None (pass 68: 50238; Δ = +452s ≈ 7.5 min — slightly smaller Δ, within normal clock skew) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED (08:54Z):** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=5070, silence_secs=1, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED (08:54Z):** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=5070, silence_secs=1, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 68 (~08:45Z):** Heartbeats api +52 (5018→5070 ~5.8/min), witness +45 (5025→5070 ~5.0/min). Both normal rate. Silence: api 1s (pass 68: 6s), witness 1s (pass 68: 1s) — both well within threshold (<30s). Queue depth 0 on both.

**No zombie eviction events** detected in either log.

---

## Epoch State

### morning-api (single-capture bundle ~08:54Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1692 (endpoint + 3-way check) | Cycling. +18 from pass 68 (1674→1692 bundle) in ~9 min. | None. Normal cadence (~30-33s/epoch). |
| ratio | 1.019778 | ~1.01–1.02 steady state | None (pass 68: 1.019978; Δ = −0.0002 — essentially unchanged) |
| tax_calculated | 0 | Balance 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (single-capture bundle ~08:54Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1691 (endpoint), 1691 (3-way check) | Cycling. +16 from pass 68 (1675→1691 bundle) in ~9 min. | None. Witness runs ~1 epoch behind api — normal race. |
| ratio | 1.059749 | Continuing asymptotic decline | None (pass 68: 1.060383; Δ = −0.000634 over ~9 min — consistent approach to 1.0) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** Socket bundle showed morning-api at 1692, witness at 1691 (Δ=1 normal race). 3-way check: api 1692/1692/1692, witness 1691/1691/1691. **Synchronized.**

### Epoch cadence
+16–18 epochs since pass 68 (~08:45Z) in ~9 min ≈ ~30–34s/epoch. Within normal 26-35s range.

### Three-way epoch check
- **morning-api:** Socket=1692, grep=1692, last_log=epoch=1692. **PASS** — all three agree.
- **local-witness:** Socket=1691, grep=1691, last_log=epoch=1691. **PASS** — all three agree.

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

### morning-api (single-capture bundle ~08:54Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1690 | Incrementing by 10 (pass 68: 1670; +20 = 2 rotations) | None (normal — 2 rotations since pass 68) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (08:54Z single capture):**
- `state.snapshot`: 895 bytes (mtime: 2026-07-28T04:52 EDT — epoch 1690 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T04:52 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T04:47 EDT — previous epoch 1680 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 04:42 (pass 68) to 04:52 — confirmed 2 rotations occurred. ✓
- Snapshot size 895 bytes (pass 68: 895; unchanged).

### local-witness (single-capture bundle ~08:54Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1690 | Incrementing by 10 (pass 68: 1670; +20 = 2 rotations) | None (normal — 2 rotations since pass 68) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (08:54Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T04:53 EDT — epoch 1690 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T04:53 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T04:48 EDT — previous epoch 1680 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 04:43 (pass 68) to 04:53 — confirmed 2 rotations occurred. ✓
- Snapshot size (569 bytes) vs morning-api (895 bytes): consistent — witness stores only zero balances; morning-api stores both nodes' balances.

---

## Metrics Instrumentation

**OBSERVED (from m-ap metrics line at ~08:54Z):**
- `outstanding_fetches=0` — no pending fetches
- `aged=0` — no stale fetches
- `outbound_queues=[]` — all peer queues empty
- `max_peer_silence=3s` (pass 68: 3s) — well under 30s threshold

**OBSERVED (from witness metrics line at ~08:54Z):**
- `outstanding_fetches=0`
- `aged=0`
- `outbound_queues=[]`
- `max_peer_silence=6s` (pass 68: 6s)

**EXPECTED:** All gauges near zero on a settled 2-node mesh with no transactions.
**DEVIATION:** None. Mesh is quiescent.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **WARN (Jul 28 ~07:37Z):** `NTP query to pool.ntp.org failed: Input/output error: Resource temporarily unavailable (os error 11) (fallback)` — transient. Single occurrence. Unchanged from prior passes.
- **WARN (Jul 27 18:48Z, historic):** `Failed to gossip genesis (will retry on peer connect) error=InsufficientPeers` — startup artifact, no recurrence.
- **WARN (Jul 27 18:48Z, historic):** `[block-publish] Failed to publish block proposal_id="genesis" error=InsufficientPeers` — startup artifact, no recurrence.
- **ERROR:** None.
- **Zombie eviction events:** None.
- **Sweep/eviction events:** None.

### local-witness (/tmp/lw.log)
- **WARN (118 total, all historic Jul 27):** `Transaction validation failed error=insufficient balance` — all from original redistribution test. No new occurrences since Jul 27.
- **WARN (Jul 28 ~08:00Z):** `NTP query to pool.ntp.org failed: Input/output error: Resource temporarily unavailable (os error 11) (fallback)` — transient. Single occurrence. Unchanged from prior passes.
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

| # | Deviation | First Observed | Pass 68 Status | Pass 69 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

**No new deviations detected in this pass.**

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (all three agree) | **PASS** — morning-api (1692/1692/1692). Witness (1691/1691/1691). Clean agreement on both. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2) |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 |
| Log health (WARN/ERROR filtered) | **PASS** — only historic Jul 27 transactions and benign NTP failure. No new events. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** |
| Cross-node epoch sync | **PASS** — witness at 1691, api at 1692 (Δ=1 normal race) |
