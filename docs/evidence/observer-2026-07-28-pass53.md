# Observer Evidence Record — 2026-07-28 (Pass 53)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** 2026-07-28T05:59:41–06:05:49Z bundle (01:59:41–02:05:49 EDT)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Fifty-third observation pass. Same processes since 2026-07-27T18:48Z (~11.2h runtime). ~13 min since pass 52 (05:52Z).

**Summary:** Routine continuation. Both nodes at epoch 1355 — fully synchronized. Three-way epoch clean on both nodes. Snapshot epoch 1320→1340 (2 rotations since pass 52). No new deviations. All three persistent deviations unchanged (build_commit, wal_bytes endpoint, supply divergence).

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
- Socket: `/tmp/m-ap/lattice.sock` (morning-api) — responding
- Socket: `/tmp/local-witness/lattice.sock` (local-witness) — responding

Logs at `/tmp/m-ap.log` (morning-api) and `/tmp/lw.log` (local-witness).

**Clock:** System clock synchronized via NTP. No deviation.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 40276 (05:59Z) | — | None (pass 52: 39780; Δ = +496s ≈ 8.3 min — matches elapsed real time) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 1 commit behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 989.28 | ~1000, slowly decaying | None (pass 52: 989.41; Δ = -0.13 over ~8 min — consistent decay ~0.016/min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 40278 (05:59Z) | — | None (pass 52: 39781; Δ = +497s — within 1s of api delta, consistent) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED (05:59Z):** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=4026, silence_secs=2, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED (05:59Z):** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=4028, silence_secs=8, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 52:** Heartbeats api +50 (3976→4026), witness +49 (3979→4028). Both ~5.8/min. Silence: api 9s→2s (improved), witness 4s→8s (well within <30s threshold). Queue depth 0 on both.

---

## Epoch State

### morning-api (05:59:41Z capture; 06:05:45Z recapture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1343 (socket 05:59Z) → 1355 (recapture 06:05Z) | Cycling. +27 since pass 52 (1328→1355) in ~13 min. | **PASS — clean three-way match at recapture.** Socket=1355, grep count=1355, last log line=epoch=1355. Clean. |
| ratio | 1.01972 | ~1.01–1.02 steady state | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (05:59:41Z; 06:05:49Z recapture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1343 (socket 05:59Z) → 1355 (recapture 06:05Z) | Cycling. +27 since pass 52 (1328→1355) in ~13 min. | **PASS — clean three-way match at recapture.** Socket=1355, grep count=1355, last log line=epoch=1355. Clean. |
| ratio | 1.07621 | Continuing asymptotic decline | None (pass 52: 1.07789; Δ = -0.00168 — normal decline ~0.00013/epoch, consistent) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization

**OBSERVED:** At recapture (06:05Z): both nodes at epoch 1355. Fully synchronized.

### Epoch cadence

+27 epochs since pass 52 in ~13 min ≈ 28.9s/epoch. Both nodes consistent. Slightly faster than 31.9s/epoch observed between passes 51→52, but within normal timer variance.

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
**EXPECTED (proposed invariant, per VERIFIED-BEHAVIOR.md):** Sum of all spendable balances across mesh = 5000.
**DEVIATION:** Witness-side accounting reports 0. Known-deviating since first observer pass. Unchanged.

---

## Persistence State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1340 | Incrementing by 10 (pass 52: 1320; +20 epochs = 2 rotations) | None (normal — 2 rotations since pass 52) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (06:05Z single capture):**
- `state.snapshot`: 894 bytes (mtime: 2026-07-28T02:02 EDT — epoch 1340 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T02:02 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T01:57 EDT — previous epoch 1330 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- state.snapshot size: **894 bytes** (pass 52 at epoch 1320: 895; now at epoch 1340: 894. Content-dependent variation: 893→895→894 across 3 recent boundaries.)

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1340 | Incrementing by 10 (pass 52: 1320; +20 epochs = 2 rotations) | None (both nodes synchronized) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (06:05Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T02:03 EDT — epoch 1340 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T02:03 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T01:58 EDT — previous epoch 1330 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### Snapshot synchronization

Both nodes at last_snapshot_epoch=1340 (was 1320 in pass 52 — 2 rotations since). File mtimes within ~1 min (02:02 vs 02:03 EDT). Snapshot rotation working normally.

### state.snapshot size (morning-api)

**OBSERVED:** 894 bytes at epoch 1340 (was 895 at epoch 1320, 893 at epoch 1300, 895 at epoch 1310, 895 at epoch 1320, 894 at epoch 1340).
**CLASSIFICATION:** Normal content-dependent variation. Not a deviation.

---

## Metrics (from heartbeat logs)

### morning-api — latest ticks (06:01:16–06:01:36Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s`
**CLASSIFICATION:** Clean.

### local-witness — latest ticks (06:04:13–06:04:33Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s`
**CLASSIFICATION:** Clean.

**All observed ticks (~13 min window):** Every tick on both nodes shows `aged=0`, `outbound_queues=[]`. No stale fetch entries, no queue buildup, no zombie evictions, no sweep events.

---

## Error Health Scan

### morning-api
**OBSERVED:** 142 total WARN/ERROR lines (+3 since pass 52: 139→142). After filtering (expected patterns + genesis startup warnings): **zero unexpected lines.** The +3 is normal 5-min-interval kad bootstrap accumulation (expected with `--no-mdns`). Two startup-time genesis gossip warnings are also expected (InsufficientPeers at first startup before peers connected).
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

### local-witness
**OBSERVED:** 122 total WARN/ERROR lines (unchanged from pass 52). After filtering: **zero unexpected lines.**
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

---

## Delta Summary (Pass 52 → Pass 53)

| Metric | Pass 52 (05:52Z) | Pass 53 (06:05Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch (recapture) | 1328 | 1355 | +27 (~13 min) | Normal cycling (~29s/epoch) |
| witness epoch (recapture) | 1328 | 1355 | +27 (~13 min) | Synchronized with api |
| Three-way match | api: CLEAN, witness: CLEAN | api: CLEAN, witness: CLEAN | None | Both clean |
| Epoch sync | 0 (both 1328) | 0 (both 1355) | None | Fully synchronized |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch (api) | 1320 | 1340 | +20 (2 rotations) | Normal 10-epoch interval |
| Snapshot epoch (witness) | 1320 | 1340 | +20 (2 rotations) | Synchronized |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk (api) | 379 bytes, 01:47 EDT | 379 bytes, 02:02 EDT | 0 size; mtime +15 min | Snapshot rotation working |
| wal.log on disk (witness) | 379 bytes, 01:48 EDT | 379 bytes, 02:03 EDT | 0 size; mtime +15 min | Snapshot rotation working |
| state.snapshot (api) | 895 bytes, 01:47 | 894 bytes, 02:02 | -1 byte | Content variation: 893→895→894 |
| state.snapshot (witness) | 569 bytes, 01:48 | 569 bytes, 02:03 | 0 | Stable |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 1 commit behind HEAD + dirty tree |
| Heartbeats (api) | 3976 | 4026 | +50 | Normal — ~5.8/min |
| Heartbeats (witness) | 3979 | 4028 | +49 | Normal — ~5.8/min |
| max_peer_silence (api) | 3s | 3s | 0 | Well within <30s |
| max_peer_silence (witness) | 6s | 6s | 0 | Well within <30s |
| Sweep/evict events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~32s/epoch | ~29s/epoch | Slight variance | Within normal range |
| Total WARN/ERROR (api) | 139 | 142 | +3 | Kad bootstrap noise; all expected |
| Total WARN/ERROR (witness) | 122 | 122 | 0 | Unchanged |

### New observations this pass

1. **Both nodes fully synchronized at epoch 1355.** Clean three-way match on both at recapture (1355/1355/1355).

2. **Two snapshot rotations** since pass 52 (1320→1330→1340). Consistent with ~5 min interval.

3. **state.snapshot size** shifted from 895→894 (content-dependent variation, normal).

### No new deviations

The three persistent deviations are unchanged.

---

## Persistent Deviations (unchanged)

| # | Observation | First seen | Status |
|---|------------|-----------|--------|
| 1 | build_commit 71aa16b-dirty vs HEAD cb5d4b1 (1 commit behind + dirty tree) | Pass 1 (Jul 27) | Persistent — binary not rebuilt since those commits |
| 2 | GetPersistenceState wal_bytes=0 (reads transactions.wal instead of wal.log) | Pass 1 (Jul 27) | Persistent — Verifier Mission 2 confirmed one-line fix |
| 3 | Local-witness reports morning-api balance as 0 (supply divergence); nonces frozen | Pass 1 (Jul 27) | Persistent — supply conservation CONTRADICTED per Verifier Mission 1 |

---

## UNKNOWN Items

None.

---

## Evidence Files

- Previous: `docs/evidence/observer-2026-07-28-pass52.md`
- This: `docs/evidence/observer-2026-07-28-pass53.md`

---

## Raw Capture Bundle

```json
// Timestamp — 05:59:41Z
// GetNodeInfo (morning-api) — 05:59:41Z
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":40276,"build_commit":"71aa16b-dirty","thickness":989.2835737345447}

// GetPeers (morning-api) — 05:59:41Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":4026,"silence_secs":2,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api) — 05:59:41Z
{"type":"EpochState","epoch":1343,"ratio":1.0197202461304027,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (morning-api) — 05:59:41Z
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api) — 05:59:41Z
{"type":"PersistenceState","last_snapshot_epoch":1340,"wal_bytes":0,"wal_entries":0}

// GetHeight (morning-api) — 05:59:41Z
{"type":"Height","height":1}

// GetNodeInfo (local-witness) — 05:59:41Z
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":40278,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness) — 05:59:41Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":4028,"silence_secs":8,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness) — 05:59:41Z
{"type":"EpochState","epoch":1343,"ratio":1.0762094187922333,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (local-witness) — 05:59:41Z
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness) — 05:59:41Z
{"type":"PersistenceState","last_snapshot_epoch":1340,"wal_bytes":0,"wal_entries":0}

// GetHeight (local-witness) — 05:59:41Z
{"type":"Height","height":1}

// Three-way epoch (morning-api recapture) — 06:05:45Z
// Socket: 1355; grep -c "Epoch complete": 1355; last log line: epoch=1355
// CLEAN.

// Three-way epoch (local-witness recapture) — 06:05:49Z
// Socket: 1355; grep -c "Epoch complete": 1355; last log line: epoch=1355
// CLEAN.

// File system (06:05Z) — wal_bytes mismatch confirmed
// morning-api: state.snapshot=894, wal.log=379, wal.wal.old=379, endpoint=0
// local-witness: state.snapshot=569, wal.log=379, wal.wal.old=379, endpoint=0
```

## Verification Cross-Checks

| Check | morning-api | local-witness | Result |
|-------|-------------|---------------|--------|
| Three-way epoch match | 1355/1355/1355 (clean) | 1355/1355/1355 (clean) | PASS — both clean |
| Nodes synced | 1355 | 1355 | PASS — synchronized |
| Byte-equality: wal_bytes endpoint vs file size | 0 vs 379 | 0 vs 379 | MISMATCH (known deviation) |
| Build commit vs git HEAD | 71aa16b-dirty vs cb5d4b1 | 71aa16b-dirty vs cb5d4b1 | DEVIATION (1 behind + dirty) |
| System clock sync | NTP active, synchronized | N/A (same machine) | PASS |
| Process health (PIDs unchanged) | 2727391 | 2727569 | PASS (no restarts — ~11.2h uptime) |
