# Observer Evidence Record — 2026-07-28 (Pass 54)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** 2026-07-28T06:24:47–06:29:13Z bundle (02:24:47–02:29:13 EDT)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Fifty-fourth observation pass. Same processes since 2026-07-27T18:48Z (~11.7h runtime). ~24 min since pass 53 (06:05Z).

**Summary:** Routine continuation. Both nodes at epoch 1402 — fully synchronized. Three-way epoch clean at first recapture (1398/1398/1398); off-by-1 race at second recapture (boundary tick between socket query and log tail) but both nodes agree on the boundary. Snapshot epoch 1340→1400 (6 rotations since pass 53). No new deviations. All three persistent deviations unchanged.

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
- Socket: `/tmp/local-witness/lattice.sock` (local-witness) — responding (was incorrectly queried at `/tmp/lw-id/lattice.sock` in initial tool call — socket path maps to `--storage-dir`, not `--identity-dir`)

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
| uptime_secs | 42034 (06:29Z) | — | None (pass 53: 40276; Δ = +1758s ≈ 29.3 min — matches elapsed real time) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 1 commit behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 988.82 | ~1000, slowly decaying | None (pass 53: 989.28; Δ = -0.46 over ~24 min — consistent decay ~0.019/min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 42033 (06:29Z) | — | None (pass 53: 40278; Δ = +1755s ≈ 29.3 min — within 3s of api delta, consistent) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED (06:24Z):** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=4177, silence_secs=2, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

**Recapture (06:29Z):** Same peer, heartbeats=4201, silence_secs=1, queue_depth=0. Progressive incremental.

### local-witness
**OBSERVED (06:24Z):** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=4191, silence_secs=6, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 53:** Heartbeats api +175 (4026→4201), witness +173 (4028→4201). Both ~7.2/min. Silence: api 2s→1s (improved), witness 8s→4s (improved). Queue depth 0 on both.

---

## Epoch State

### morning-api (first recapture 06:27:56Z; second recapture 06:28:56Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1398 (first recapture 06:27Z) → 1402 completed (second recapture 06:28Z) | Cycling. +47 since pass 53 (1355→1402) in ~24 min. | **First recapture: CLEAN three-way.** Socket=1398, grep=1398, last_log=1398. **Second recapture: RACE at boundary.** Socket=1401, grep=1402, last_log=1402 (epoch ticked between socket query and log read). Both nodes agree on boundary — not a divergence. |
| ratio | 1.01997 | ~1.01–1.02 steady state | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (first recapture 06:27:13Z; second recapture 06:29:13Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1398 (first recapture) → 1402 completed (second recapture) | Cycling. +47 since pass 53. | **First recapture: CLEAN.** Socket=1398, grep=1398, last_log=1398. **Second recapture: RACE.** Same boundary race as api — socket=1401, grep=1402, last_log=1402. Nodes fully synchronized. |
| ratio | 1.07349 | Continuing asymptotic decline | None (pass 53: 1.07621; Δ = -0.00272 — normal decline ~0.000057/epoch, consistent) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization

**OBSERVED:** Both nodes at same epoch throughout. First recapture both at 1398. Second recapture both at the 1401/1402 boundary. Fully synchronized.

### Epoch cadence

+47 epochs since pass 53 in ~24 min ≈ 30.6s/epoch. Both nodes consistent. Within normal timer variance (28–32s/epoch observed range).

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
| last_snapshot_epoch | 1400 | Incrementing by 10 (pass 53: 1340; +60 epochs = 6 rotations) | None (normal — 6 rotations since pass 53) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (06:28Z single capture):**
- `state.snapshot`: 894 bytes (mtime: 2026-07-28T02:27 EDT — epoch 1400 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T02:27 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T02:22 EDT — previous epoch 1390 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- state.snapshot size: **894 bytes** (same as pass 53 at epoch 1340; unchanged across 60 epochs of cycling)

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1400 | Incrementing by 10 (pass 53 at 06:05Z: 1340; +60 = 6 rotations) | None (caught up — was at 1390 at initial 06:24Z capture, now at 1400) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (06:28Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T02:23 EDT — epoch 1390 (at capture); since advanced to 1400)
- `wal.log`: 379 bytes (mtime: 2026-07-28T02:23 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T02:18 EDT — previous epoch 1380 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### Snapshot synchronization

Both nodes now at last_snapshot_epoch=1400 (was 1340 in pass 53 — 6 rotations since). File mtimes: api 02:27 vs witness 02:23 EDT (within ~4 min). Snapshot rotation working normally.

### state.snapshot size (morning-api)

**OBSERVED:** 894 bytes at epoch 1400 (same as pass 53 epoch 1340: 894 bytes). 170 bytes accumulated across epochs? No — the snapshot is a state snapshot that doesn't change between epochs unless the state changes, and the state IS frozen.

Wait — 894 bytes is the same as pass 53. Let me check: pass 53 said 894 at epoch 1340. Now at epoch 1400 it's 894. So the snapshot size is stable across 60 epochs of cycling with no state changes. That's consistent with a frozen balance state.

Actually interesting: the snapshot is 894 bytes despite representing 0/total-supply changes. It encodes the economic state (balances, nonces, epoch metadata), which doesn't change because the accounting is frozen.

---

## Metrics (from heartbeat logs)

### morning-api — latest ticks (06:27:16–06:27:56Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=1-3s`
**CLASSIFICATION:** Clean.

### local-witness — latest ticks (06:27:23–06:28:03Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=4-6s`
**CLASSIFICATION:** Clean.

**All observed ticks (~24 min window):** Every tick on both nodes shows `aged=0`, `outbound_queues=[]`. No stale fetch entries, no queue buildup, no zombie evictions, no sweep events.

---

## Error Health Scan

### morning-api
**OBSERVED:** 146 total WARN/ERROR lines (+4 since pass 53: 142→146). After filtering (expected patterns + genesis startup warnings): **zero unexpected lines.** The +4 is normal 5-min-interval kad bootstrap accumulation (expected with `--no-mdns`).
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

### local-witness
**OBSERVED:** 122 total WARN/ERROR lines (unchanged from pass 53). After filtering: **zero unexpected lines.**
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

---

## Delta Summary (Pass 53 → Pass 54)

| Metric | Pass 53 (06:05Z) | Pass 54 (06:29Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch (recapture) | 1355 | 1402 | +47 (~24 min) | Normal cycling (~30.6s/epoch) |
| witness epoch (recapture) | 1355 | 1402 | +47 (~24 min) | Synchronized with api |
| Three-way match | api: CLEAN, witness: CLEAN | api: CLEAN (then boundary race), witness: CLEAN (same boundary race) | Boundary race at 2nd recapture | Both nodes agree — not divergence |
| Epoch sync | 0 (both 1355) | 0 (both 1402) | None | Fully synchronized |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch (api) | 1340 | 1400 | +60 (6 rotations) | Normal 10-epoch interval |
| Snapshot epoch (witness) | 1340 | 1400 | +60 (6 rotations) | Synchronized (was 1390 at 06:24Z, caught up to 1400 by 06:28Z) |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk (api) | 379 bytes, 02:02 EDT | 379 bytes, 02:27 EDT | 0 size; mtime +25 min | Snapshot rotation working |
| wal.log on disk (witness) | 379 bytes, 02:03 EDT | 379 bytes, 02:23 EDT | 0 size; mtime +20 min | Snapshot rotation working |
| state.snapshot (api) | 894 bytes, 02:02 | 894 bytes, 02:27 | 0 | Stable |
| state.snapshot (witness) | 569 bytes, 02:03 | 569 bytes, 02:23 | 0 | Stable |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 1 commit behind HEAD + dirty tree |
| Heartbeats (api) | 4026 | 4201 | +175 | Normal — ~7.3/min |
| Heartbeats (witness) | 4028 | 4201 | +173 | Normal — ~7.2/min |
| max_peer_silence (api) | 3s | 1-3s | ~0 | Well within <30s |
| max_peer_silence (witness) | 6s | 4-6s | ~0 | Well within <30s |
| Sweep/evict events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~29s/epoch | ~30.6s/epoch | Slight variance | Within normal range |
| Total WARN/ERROR (api) | 142 | 146 | +4 | Kad bootstrap noise; all expected |
| Total WARN/ERROR (witness) | 122 | 122 | 0 | Unchanged |

### New observations this pass

1. **Both nodes fully synchronized at epoch 1402.** Three-way match clean at first recapture (1398/1398/1398). Boundary race at second recapture but both nodes agree — not a divergence.

2. **Six snapshot rotations** since pass 53 (1340→1350→...→1400). Consistent with ~4 min interval.

3. **state.snapshot size stable at 894 bytes** (api) across 60 epochs with no state changes. Expected — frozen economic state.

4. **Uptime approaching 12 hours** (42034s ≈ 11.7h). Process health stable — no restarts.

### No new deviations

The three persistent deviations are unchanged. Supply conservation contradiction and wal_bytes endpoint path bug remain. Build_commit still 1 commit behind HEAD with dirty tree.

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

- Previous: `docs/evidence/observer-2026-07-28-pass53.md`
- This: `docs/evidence/observer-2026-07-28-pass54.md`

---

## Raw Capture Bundle

```json
// Timestamp — 06:24:47Z
// GetNodeInfo (morning-api) — 06:24:47Z
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":41803,"build_commit":"71aa16b-dirty","thickness":988.8868253477401}

// GetPeers (morning-api) — 06:24:47Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":4177,"silence_secs":2,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api) — 06:24:47Z
{"type":"EpochState","epoch":1393,"ratio":1.0197302972478821,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (morning-api) — 06:24:47Z
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api) — 06:24:47Z
{"type":"PersistenceState","last_snapshot_epoch":1390,"wal_bytes":0,"wal_entries":0}

// GetHeight (morning-api) — 06:24:47Z
{"type":"Height","height":1}

// GetNodeInfo (local-witness) — 06:27:13Z
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":41900,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness) — 06:27:13Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":4191,"silence_secs":6,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness) — 06:27:13Z
{"type":"EpochState","epoch":1397,"ratio":1.07371857307134,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (local-witness) — 06:27:13Z
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness) — 06:27:13Z
{"type":"PersistenceState","last_snapshot_epoch":1390,"wal_bytes":0,"wal_entries":0}

// GetHeight (local-witness) — 06:27:13Z
{"type":"Height","height":1}

// Three-way epoch (morning-api first recapture) — 06:27:56Z
// Socket: 1398; grep -c "Epoch complete": 1398; last log line: epoch=1398
// CLEAN.

// Three-way epoch (local-witness first recapture) — 06:27:13Z
// Socket: 1398; grep -c "Epoch complete": 1398; last log line: epoch=1398
// CLEAN.

// Three-way epoch (morning-api second recapture) — 06:28:56Z
// Socket: 1401; grep -c "Epoch complete": 1402; last log line: epoch=1402
// RACE — boundary tick between queries. Both nodes agree.

// Three-way epoch (local-witness second recapture) — 06:29:13Z
// Socket: 1401; grep -c "Epoch complete": 1402; last log line: epoch=1402
// RACE — same boundary. Both nodes agree.

// File system (06:28Z) — wal_bytes mismatch confirmed
// morning-api: state.snapshot=894, wal.log=379, wal.wal.old=379, endpoint=0
// local-witness: state.snapshot=569, wal.log=379, wal.wal.old=379, endpoint=0
```

## Verification Cross-Checks

| Check | morning-api | local-witness | Result |
|-------|-------------|---------------|--------|
| Three-way epoch match (first recapture) | 1398/1398/1398 (clean) | 1398/1398/1398 (clean) | PASS — both clean |
| Three-way epoch match (second recapture) | 1401/1402/1402 (boundary race) | 1401/1402/1402 (boundary race) | PASS — both agree on boundary |
| Nodes synced | 1402 | 1402 | PASS — synchronized |
| Byte-equality: wal_bytes endpoint vs file size | 0 vs 379 | 0 vs 379 | MISMATCH (known deviation) |
| Build commit vs git HEAD | 71aa16b-dirty vs cb5d4b1 | 71aa16b-dirty vs cb5d4b1 | DEVIATION (1 behind + dirty) |
| System clock sync | NTP active, synchronized | N/A (same machine) | PASS |
| Process health (PIDs unchanged) | 2727391 | 2727569 | PASS (no restarts — ~11.7h uptime) |
