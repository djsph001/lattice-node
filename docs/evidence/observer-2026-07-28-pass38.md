# Observer Evidence Record — 2026-07-28 (Pass 38)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T03:43:22Z (single-capture bundle: 03:43Z simultaneous three-way)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Thirty-eighth observation pass. Same processes since 2026-07-27T18:48Z (~8.9h runtime). ~9 min since pass 37 (03:34Z).

**Summary:** Routine continuation. Epoch 1070 on both nodes (1-epoch offset from pass 37's 1050/1051 resolved — timing, not chronic lag). Three-way match PASS on witness; api shows 1-epoch race at boundary (expected — epoch turned over during capture). Snapshot rotated 1060→1070 (10-epoch interval confirmed). Balance locked at 20/0. Zero queues, zero fetches, zero zombie/evict activity. Build unchanged. All 3 persistent deviations unchanged. Delta: routine continuation; no new observations.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 2026-07-27T18:48Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 2026-07-27T18:48Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs (2727391, 2727569). Both sockets responding. 2 lattice-node processes.
- Socket: `/tmp/m-ap/lattice.sock` (morning-api) — responding
- Socket: `/tmp/local-witness/lattice.sock` (local-witness) — responding

Logs at `/tmp/m-ap.log` and `/tmp/lw.log`.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 32120 (~8.9h) | — | None (pass 37: 31413; Δ = +707s ≈ 11.8 min — includes observation window + wait) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 991.445 | ~1000, slowly decaying | None (pass 37: 991.628; Δ = -0.183 over ~9 min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 32106 (~8.9h) | — | None (matches api within 14s) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=3209, silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=3210, silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 37:** Heartbeats api +68 (3141→3209), witness +68 (3142→3210). Silence: 4s→7s (api), 5s→7s (witness). Queue depth 0 on both. Normal variance; all silence within <10s threshold. Both nodes report identical silence_secs for first time — suggests aligned heartbeat tick timing.

---

## Epoch State

### morning-api (03:43Z bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1070 | Cycling. +19 since pass 37 (1051→1070). | **PASS — three-way near-match.** Socket returned 1070 at 03:43:22Z. grep -c=1071, last log line at 1071 (03:43:26Z). Epoch turned over during ~4s window between socket query and grep. Expected race at epoch boundary. |
| ratio | 1.01967 | ~1.01–1.02 steady state (pass 37: 1.01999) | None (stable; small variance from tick timing) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (03:43Z simultaneous capture):**
- Socket epoch: **1070**
- `grep -c` count: **1071** (epoch turned over during capture — see race note above)
- Last log line epoch: **1071** (at 03:43:26Z)
- **MINOR RACE at epoch boundary.** Socket captured 1070; by the time grep ran, epoch 1071 had been logged. Not a deviation.

### local-witness (03:43Z bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1070 | Cycling. +20 since pass 37 (1050→1070). | **PASS — three-way match.** Socket, grep count, and last log line all at 1070. |
| ratio | 1.09826 | Continuing asymptotic decline (pass 37: 1.10026; Δ = -0.00200) | None (monotonic decay expected) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (03:43Z simultaneous capture):**
- Socket epoch: **1070**
- `grep -c` count: **1070**
- Last log line epoch: **1070** (at 03:43:13Z)
- **PASS.** All three agree.

### Pass 37's 1-epoch offset resolved — reconfirmed as transient timing

**OBSERVED (pass 37):** api=1051, witness=1050 — 1-epoch gap.
**OBSERVED (pass 38):** api=1070, witness=1070 — no gap in the socket bundle. The later grep showed the api had incremented to 1071 by the time epoch turned over.
**CLASSIFICATION:** Same transient pattern as passes 35-37. When captured at the right moment, both nodes are aligned. The offset appears or disappears depending on whether the capture catches them on the same tick or mid-cycle. Confirmed as benign timing — no chronic witness lag.

### Epoch cadence — reconfirmed

**OBSERVED:** +19/+20 epochs in ~9 minutes. ~27–31s/epoch. Consistent with the established ~30s cadence (small variance from window boundaries). Normal.

---

## Economic State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged since pass 37) |
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
**EXPECTED (proposed invariant, per VERIFIED-BEHAVIOR.md):** Sum of all spendable balances across mesh = 5000.
**DEVIATION:** Witness-side accounting reports 0. Known-deviating since first observer pass.

---

## Persistence State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1070 | 1070 | None — snapshot at 10-epoch interval (pass 37: 1060, now: 1070; one rotation). File mtime confirmed: 2026-07-28T03:42Z (~1 min before capture). |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (03:43Z):**
- `state.snapshot`: 895 bytes (mtime: 2026-07-28T03:42Z — fresh, epoch 1070 snapshot). Size unchanged from pass 37 (895); snapshot format stable across rotation.
- `wal.log`: 379 bytes (mtime: 2026-07-28T03:42Z — reflects latest snapshot rotation)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-27T23:37Z — previous epoch 1060 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1070 | 1070 | None — matches log. Same 10-epoch interval. File mtime confirmed: 2026-07-28T03:43Z (~0 min before capture). |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (03:43Z):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T03:43Z — fresh). Size unchanged from pass 37.
- `wal.log`: 379 bytes (mtime: 2026-07-28T03:43Z)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-27T23:38Z)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

---

## Metrics (from heartbeat logs)

### morning-api — latest tick (03:42:56Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s`
**CLASSIFICATION:** Clean. Zero fetches, zero queues, max peer silence well within threshold.

### local-witness — latest tick (03:42:53Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s`
**CLASSIFICATION:** Clean. Slightly higher max_peer_silence (6s vs api's 3s) but well under 10s threshold. Consistent with epoch timing offset pattern.

**All observed ticks (03:33–03:43Z):** Every tick on both nodes shows `aged=0`, `outbound_queues=[]`. No stale fetch entries, no queue buildup, no zombie evictions.

---

## Error Health Scan

### morning-api
**OBSERVED:** WARN/ERROR lines after filtering: **Kademlia bootstrap WARNs only** (harmless — no Kademlia DHT configured). Recurring every 5 min since node start. No new error types since pass 37.
**EXPECTED:** No actionable errors.
**DEVIATION:** None.

### local-witness
**OBSERVED:** WARN/ERROR lines after filtering: **zero**.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

**Filter applied:** `grep -vE 'skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|Kademlia|Connection from|Failed to gossip genesis|Failed to publish block'`

---

## Delta Summary (Pass 37 → Pass 38)

| Metric | Pass 37 (03:34Z) | Pass 38 (03:43Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch | 1051 | 1070 | +19 (~9 min) | Normal cycling (~28s/epoch) |
| witness epoch | 1050 | 1070 | +20 (~9 min) | Gap resolved — both at 1070 |
| Three-way match | PASS both | PASS (witness); minor race at boundary (api) | — | Resolved 1-epoch offset; boundary race expected |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch | 1050→1060 → 1070 | 1060→1070 | +10 (one rotation) | Normal 10-epoch interval; both nodes rotated cleanly |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk (api) | 379 bytes, Jul 27 23:37 | 379 bytes, Jul 28 03:42 | 0 size; mtime live | Snapshot rotation working correctly |
| wal.log on disk (witness) | 379 bytes, Jul 27 23:38 | 379 bytes, Jul 28 03:43 | 0 size; mtime live | Snapshot rotation working correctly |
| state.snapshot (api) | 895 bytes, Jul 27 23:37 | 895 bytes, Jul 28 03:42 | 0 | Format stable across two rotations |
| state.snapshot (witness) | 569 bytes, Jul 27 23:38 | 569 bytes, Jul 28 03:43 | 0 | Format stable |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 9 commits behind HEAD |
| Heartbeats (api) | 3141 | 3209 | +68 | Normal — ~7.6/min |
| Heartbeats (witness) | 3142 | 3210 | +68 | Normal — ~7.6/min |
| max_peer_silence (api) | 4s | 3s | -1s | Normal variance; well within <10s |
| max_peer_silence (witness) | 5s | 6s | +1s | Normal variance |
| Kademlia bootstrap WARNs | Present (harmless) | Present (harmless) | — | Unchanged |
| Zombie/reconnect events | None | None | — | Clean |
| Sweep/evict events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~30–33s | ~27–31s | — | Consistent within window variance |

### New observations this pass

1. **Snapshot rotation at epoch 1070 confirmed.** Both nodes rotated from epoch 1060→1070 on the 10-epoch schedule. State file sizes unchanged from pass 37 (api: 895 bytes; witness: 569 bytes). Format stable across two consecutive rotations.

2. **Witness epoch gap resolved.** Pass 37 showed api=1051, witness=1050. Now both at 1070 in the same bundle. Confirms the 1-epoch offset is a transient timing artifact, not chronic lag.

### No new observations this pass

No new findings, deviations, or anomalous metrics. The mesh is in a stable steady state: epoch cycles at ~30s, snapshot rotations at 10-epoch intervals, zero transaction activity, zero fetches, zero queues, locked balances.

---

## Persistent Deviations (unchanged)

| # | Observation | First seen | Status |
|---|------------|-----------|--------|
| 1 | build_commit 9 commits behind HEAD (71aa16b-dirty vs cb5d4b1) | Pass 1 (Jul 27) | Persistent — binary not rebuilt since those commits |
| 2 | GetPersistenceState wal_bytes=0 (reads transactions.wal instead of wal.log) | Pass 1 (Jul 27) | Persistent — Verifier Mission 2 confirmed one-line fix |
| 3 | Local-witness reports morning-api balance as 0 (supply divergence); nonces frozen | Pass 1 (Jul 27) | Persistent — supply conservation CONTRADICTED per Verifier Mission 1 |

---

## UNKNOWN Items

**(None.)** No new UNKNOWNS this pass.

---

## Evidence Files

- Previous: `docs/evidence/observer-2026-07-28-pass37.md`
- This record: `docs/evidence/observer-2026-07-28-pass38.md`
