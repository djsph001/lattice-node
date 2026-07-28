# Observer Evidence Record — 2026-07-28 (Pass 37)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T03:33:51Z (single-capture bundle: 03:34Z simultaneous three-way)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Thirty-seventh observation pass. Same processes since 2026-07-27T18:48Z (~8.7h runtime). ~11 min since pass 36 (03:23Z).

**Summary:** All-clear continuation. Epoch 1051/1050 (1-epoch timing offset — same pattern as pass 35). Three-way match PASS on both. Balance locked at 20/0. Snapshots cycled twice (1030→1050). Zero queues, zero fetches, zero zombie/evict activity. Build unchanged. All 3 persistent deviations unchanged. Witness 1-epoch offset re-emerged (same half-cycle timing as pass 35, not a chronic lag). Delta: routine continuation; no new observations.

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
| uptime_secs | 31413 (~8.7h) | — | None (pass 36: 30864; Δ = +549s ≈ 9.1 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 991.628 | ~1000, slowly decaying | None (pass 36: 991.779; Δ = -0.151 over ~11 min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 31401 (~8.7h) | — | None (matches api within 12s) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=3141, silence_secs=4, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=3142, silence_secs=5, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 36:** Heartbeats api +56 (3085→3141), witness +55 (3087→3142). Silence: 0s→4s (api — pass 36 captured at a heartbeat moment; now at normal 4s), 3s→5s (witness). Queue depth 0 on both. Normal variance; all silence within <10s threshold.

---

## Epoch State

### morning-api (03:34Z three-way bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1051 | Cycling. +20 since pass 36 (1031→1051). | **PASS — three-way match.** |
| ratio | 1.01999 | ~1.01–1.02 steady state (pass 36: 1.01966) | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (03:34Z simultaneous capture):**
- Socket epoch: **1051**
- `grep -c` count: **1051**
- Last log line epoch: **1051**
- **PASS.** All three agree.

### local-witness (03:34Z three-way bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1050 | Cycling. +20 since pass 36 (1030→1050). | **PASS — three-way match.** Socket, grep count, and last log line all at 1050. |
| ratio | 1.10026 | Continuing asymptotic decline (pass 36: 1.10223; Δ = -0.00197) | None (monotonic decay expected) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (03:34Z simultaneous capture):**
- Socket epoch: **1050**
- `grep -c` count: **1050**
- Last log line epoch: **1050**
- **PASS.** All three agree.

### Pass 36's resolved lag is back — reconfirmed as transient timing

**OBSERVED (pass 36):** Both nodes at 1031 during same-capture bundle — no gap.
**OBSERVED (pass 37):** api=1051, witness=1050 — 1-epoch gap re-emerged.
**CLASSIFICATION:** This is the same half-cycle timing offset first seen in pass 35 (api=1015, witness=1014). Both nodes cycle at identical ~30s cadence. The gap appears or disappears depending on whether the simultaneous capture catches them on the same tick or 13s apart. Confirmed as transient — no chronic witness lag.

### Epoch cadence — reconfirmed

**OBSERVED:** +20 epochs in ~11 minutes. ~33s/epoch. Close to the established 30s cadence (small variance from start/end of window timing). Consistent with previous passes.

---

## Economic State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged since pass 36) |
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
| last_snapshot_epoch | 1050 | 1050 | None — snapshot at 10-epoch interval (pass 36: 1030, now: 1050; two rotations). File mtime confirmed: 2026-07-28T03:32:56Z (~1 min before capture). |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (03:34Z):**
- `state.snapshot`: 895 bytes (mtime: 03:32:56Z — fresh, epoch 1050 snapshot). Size unchanged from pass 36's 895; snapshot content format stable.
- `wal.log`: 379 bytes (mtime: 03:32:56Z — reflects latest snapshot rotation)
- `wal.wal.old`: 379 bytes (mtime: 03:27:56Z — previous epoch 1040 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1050 | 1050 | None — matches log. Same 10-epoch interval. File mtime confirmed: 2026-07-28T03:33:13Z (~1 min before capture). |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (03:34Z):**
- `state.snapshot`: 569 bytes (mtime: 03:33:13Z — fresh). Size unchanged from pass 36.
- `wal.log`: 379 bytes (mtime: 03:33:13Z)
- `wal.wal.old`: 379 bytes (mtime: 03:28:13Z)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

---

## Metrics (from heartbeat logs)

### morning-api — latest tick (03:33:26Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s`
**CLASSIFICATION:** Clean. Zero fetches, zero queues, max peer silence well within threshold.

### local-witness — latest tick (03:33:13Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s`
**CLASSIFICATION:** Clean. Slightly higher max_peer_silence (6s vs api's 3s) but well under 10s threshold. Expected for the 1-epoch timing offset.

**All observed ticks (03:28–03:33Z):** Every tick on both nodes shows `aged=0`, `outbound_queues=[]`. No stale fetch entries, no queue buildup, no zombie evictions. Metrics tick is reliable at ~10s cadence.

---

## Error Health Scan

### morning-api
**OBSERVED:** WARN/ERROR lines after filtering: **Kademlia bootstrap WARNs only** (harmless — no Kademlia DHT configured). Recurring every 5 min since node start.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — Kademlia WARNs are benign and present since node start (Jul 27 18:48).

### local-witness
**OBSERVED:** WARN/ERROR lines after filtering: **zero**.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

**Filter applied:** `grep -vE 'skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|Kademlia|Connection from|Failed to gossip genesis|Failed to publish block'`

---

## Delta Summary (Pass 36 → Pass 37)

| Metric | Pass 36 (03:23Z) | Pass 37 (03:34Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch | 1031 | 1051 | +20 (~11 min) | Normal cycling (~33s/epoch) |
| witness epoch | 1031 | 1050 | +19 (~11 min) | 1-epoch offset — timing, not chronic lag |
| Three-way match | PASS both | PASS both | — | Stable |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch | 1030 | 1050 | +20 (two rotations) | Normal 10-epoch interval |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk (api) | 379 bytes, 03:32:56Z | 379 bytes, 03:32:56Z | 0 size; mtime live | Snapshot rotation working correctly |
| wal.log on disk (witness) | 379 bytes, 03:33:13Z | 379 bytes, 03:33:13Z | 0 size; mtime live | Snapshot rotation working correctly |
| state.snapshot (api) | 895 bytes | 895 bytes | 0 | Format stable across two rotations |
| state.snapshot (witness) | 569 bytes | 569 bytes | 0 | Format stable |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 9 commits behind HEAD |
| Heartbeats (api) | 3085 | 3141 | +56 | Normal — ~5.1/min |
| Heartbeats (witness) | 3087 | 3142 | +55 | Normal — ~5.0/min |
| max_peer_silence (api) | 0s (heartbeat moment) | 4s | +4s | Normal variance; well within <10s |
| max_peer_silence (witness) | 3s | 5s | +2s | Normal variance |
| Kademlia bootstrap WARNs | Present (harmless) | Present (harmless) | — | Unchanged |
| Zombie/reconnect events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~30s | ~33s | — | Consistent (variance from window boundaries) |

### New observations this pass

1. **Snapshot rotation verified live.** Pass 36 noted state.snapshot grew 1 byte (894→895). Now at 895 bytes with fresh mtime (03:32:56Z). Both file timestamps updated with last_snapshot_epoch. Snapshot rotation is working correctly despite the `wal_bytes=0` endpoint bug.

2. **Pass 36's resolved lag re-emerged — confirming transient.** Both nodes were synchronized at 1031 in pass 36's bundle. Now api=1051, witness=1050 — same 1-epoch half-cycle offset as pass 35. This is the expected pattern: they catch up when sampled at a moment of alignment, otherwise show 1-epoch offset.

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

- Previous: `docs/evidence/observer-2026-07-28-pass36.md`
- This record: `docs/evidence/observer-2026-07-28-pass37.md`
