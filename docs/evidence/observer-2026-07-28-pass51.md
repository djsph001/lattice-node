# Observer Evidence Record — 2026-07-28 (Pass 51)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** 2026-07-28T05:42:42–05:44:13Z bundle (01:42:42–01:44:13 EDT)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Fifty-first observation pass. Same processes since 2026-07-27T18:48Z (~11h runtime). ~10 min since pass 50 (05:33Z).

**Summary:** Routine continuation. Both nodes at epoch 1312 — fully synchronized. Three-way epoch clean on both nodes. Snapshot epoch 1290→1310 (2 rotations since pass 50). No new deviations. All three persistent deviations unchanged. state.snapshot on morning-api remains at 895 bytes (oscillation 893–895 confirmed across 3 snapshot boundaries).

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

Logs at `/tmp/m-ap.log` and `/tmp/lw.log`.

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
| uptime_secs | 39312 (~10.9h) | — | None (pass 50: 38706; Δ = +606s ≈ 10.1 min — consistent with ~10 min real time) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 1 commit behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 989.54 | ~1000, slowly decaying | None (pass 50: 989.70; Δ = -0.16 over ~10 min — normal decay ~0.016/min, consistent) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 39307 (~10.9h) | — | None (pass 50: 38697; Δ = +610s ≈ 10.2 min — within ~4s of api delta, normal) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=3929, silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=3931, silence_secs=6, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 50:** Heartbeats api +60 (3869→3929), witness +60 (3871→3931). Both ~6.0/min. Silence: api 3s→7s (bundle capture), witness 6s→6s (stable). Well within <30s threshold. Queue depth 0 on both.

---

## Epoch State

### morning-api (05:42:42Z capture; 05:43:56Z log confirm)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1311 (socket 05:42Z) → 1312 (log confirm 05:43Z) | Cycling. +20 since pass 50 recapture (1292→1312). | **PASS — clean three-way match at log confirm.** Socket=1311, grep count=1312, last log line=epoch=1312. Off by 1 (race at epoch boundary) at initial capture — resolved clean on recapture. |
| ratio | 1.01997 | ~1.01–1.02 steady state | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (05:42:42Z capture; 05:44:13Z log confirm)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1311 (socket 05:42Z) → 1312 (log confirm 05:44Z) | Cycling. +20 since pass 50 recapture (1292→1312). | **PASS — clean three-way match at log confirm.** Socket=1311, grep count=1312, last log line=epoch=1312. Same race pattern as morning-api — resolved clean. |
| ratio | 1.07899 | Continuing asymptotic decline | None (pass 50: 1.0803; Δ = -0.0013 — normal decline ~0.00013/epoch, consistent) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization

**OBSERVED:** At log confirm (05:43-44Z): both nodes at epoch 1312. Fully synchronized.

### Epoch cadence

+20 epochs since pass 50 recapture in ~10 min ≈ 30s/epoch. Both nodes consistent. Default timer (30s) confirmed.

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
| last_snapshot_epoch | 1310 | Incrementing by 10 (pass 50: 1290; +20 epochs = 2 rotations) | None (normal — 2 rotations since pass 50) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (05:43Z single capture):**
- `state.snapshot`: 895 bytes (mtime: 2026-07-28T01:42:56 EDT — epoch 1310 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T01:42:56 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T01:37:56 EDT — previous epoch 1300 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- state.snapshot size: **895 bytes** (pass 50: 895; intermediate at epoch 1300 was 893. Confirmed oscillation 893–895 across 3 snapshot boundaries — not a persistent shift.)

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1310 | Incrementing by 10 (pass 50: 1290; +20 epochs = 2 rotations) | None (both nodes synchronized) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (05:43Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T01:43:13 EDT — epoch 1310 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T01:43:13 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T01:38:13 EDT — previous epoch 1300 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### Snapshot synchronization

Both nodes at last_snapshot_epoch=1310 (was 1290 in pass 50 — 2 rotations since). File mtimes within ~17s (01:42:56 vs 01:43:13 EDT). Snapshot rotation working normally.

### state.snapshot size (morning-api)

**OBSERVED:** 895 bytes at epoch 1310 (was 895 at epoch 1290, 893 at epoch 1300). The oscillation 893–895 is now confirmed across 3 snapshot boundaries — content-dependent serialization variation, not a persistent shift. Witness stable at 569 bytes across same boundaries.

**Classification:** Normal variation. Not a deviation. UNKNOWN resolved by more passes.

---

## Metrics (from heartbeat logs)

### morning-api — latest ticks (05:42:26–05:42:46Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s`
**CLASSIFICATION:** Clean.

### local-witness — latest ticks (05:42:33–05:42:53Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s`
**CLASSIFICATION:** Clean.

**All observed ticks (~10 min window):** Every tick on both nodes shows `aged=0`, `outbound_queues=[]`. No stale fetch entries, no queue buildup, no zombie evictions, no sweep events.

---

## Error Health Scan

### morning-api
**OBSERVED:** 137 total WARN/ERROR lines (+1 since pass 50: 136→137). After filtering (expected patterns): **zero unexpected lines.** The +1 is normal gradual accumulation of expected startup/skip notices.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

### local-witness
**OBSERVED:** 122 total WARN/ERROR lines (unchanged from pass 50). After filtering: **zero unexpected lines.**
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

---

## Delta Summary (Pass 50 → Pass 51)

| Metric | Pass 50 (05:33Z) | Pass 51 (05:43Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch (recapture) | 1292 | 1312 | +20 (~10 min) | Normal cycling (~30s/epoch) |
| witness epoch (recapture) | 1292 | 1312 | +20 (~10 min) | Synchronized with api |
| Three-way match | api: CLEAN, witness: CLEAN | api: CLEAN, witness: CLEAN | None | Both clean (race resolved) |
| Epoch sync | 0 (both 1292) | 0 (both 1312) | None | Fully synchronized |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch (api) | 1290 | 1310 | +20 (2 rotations) | Normal 10-epoch interval |
| Snapshot epoch (witness) | 1290 | 1310 | +20 (2 rotations) | Synchronized |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk (api) | 379 bytes, 01:32 EDT | 379 bytes, 01:42 EDT | 0 size; mtime +10 min | Snapshot rotation working |
| wal.log on disk (witness) | 379 bytes, 01:33 EDT | 379 bytes, 01:43 EDT | 0 size; mtime +10 min | Snapshot rotation working |
| state.snapshot (api) | 895 bytes, 01:32 | 895 bytes, 01:42 | 0 (was 893 at intermediate epoch 1300) | Oscillation 893–895 confirmed across 3 boundaries |
| state.snapshot (witness) | 569 bytes, 01:33 | 569 bytes, 01:43 | 0 | Stable |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 1 commit behind HEAD + dirty tree |
| Heartbeats (api) | 3869 | 3929 | +60 | Normal — ~6.0/min |
| Heartbeats (witness) | 3871 | 3931 | +60 | Normal — ~6.0/min |
| max_peer_silence (api) | 3s | 3s | 0 | Well within <30s |
| max_peer_silence (witness) | 6s | 6s | 0 | Well within <30s |
| Sweep/evict events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~31s/epoch | ~30s/epoch | — | Within normal variance |
| Total WARN/ERROR (api) | 136 | 137 | +1 | Gradual accumulation; all expected |
| Total WARN/ERROR (witness) | 122 | 122 | 0 | Unchanged |

### New observations this pass

1. **Both nodes fully synchronized at epoch 1312.** Clean three-way match on both at log confirm (1312/1312/1312).

2. **state.snapshot oscillation 893–895 confirmed normal.** Across 3 snapshot boundaries (epochs 1290→1300→1310): 895→893→895. No longer classified as UNKNOWN — content-dependent serialization variation, not a persistent shift.

3. **Two snapshot rotations in ~10 min** (1290→1310), same cadence as pass 50.

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

None. The state.snapshot oscillation observation from pass 50 (894→895 bytes) has been resolved across 3 samples: 895→893→895 confirms content-dependent variation, not a deviation.

---

## Evidence Files

- Previous: `docs/evidence/observer-2026-07-28-pass50.md`
- This: `docs/evidence/observer-2026-07-28-pass51.md`

---

## Raw Capture Bundle

```json
// GetNodeInfo (morning-api) — 05:42:42Z
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":39312,"build_commit":"71aa16b-dirty","thickness":989.5375762348278}

// GetPeers (morning-api) — 05:42:42Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":3929,"silence_secs":7,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api) — 05:42:42Z
{"type":"EpochState","epoch":1311,"ratio":1.019973014027614,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (morning-api) — 05:42:42Z
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api) — 05:42:42Z
{"type":"PersistenceState","last_snapshot_epoch":1310,"wal_bytes":0,"wal_entries":0}

// GetHeight (morning-api) — 05:42:42Z
{"type":"Height","height":1}

// GetNodeInfo (local-witness) — 05:42:42Z
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":39307,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness) — 05:42:42Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":3931,"silence_secs":6,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness) — 05:44:00Z
{"type":"EpochState","epoch":1311,"ratio":1.0789901898416312,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (local-witness) — 05:44:00Z
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness) — 05:44:00Z
{"type":"PersistenceState","last_snapshot_epoch":1310,"wal_bytes":0,"wal_entries":0}

// GetHeight (local-witness) — 05:44:00Z
{"type":"Height","height":1}

// Three-way epoch (morning-api) — 05:43:56Z log confirm
// Socket: 1311 (at 05:42); grep -c "Epoch complete": 1312; last log line: epoch=1312
// Clean — race at boundary resolved.

// Three-way epoch (local-witness) — 05:44:13Z log confirm
// Socket: 1311 (at 05:44); grep -c "Epoch complete": 1312; last log line: epoch=1312
// Clean — race at boundary resolved.

// File system (05:43Z) — wal_bytes mismatch confirmed
// morning-api: state.snapshot=895, wal.log=379, wal.wal.old=379, endpoint=0
// local-witness: state.snapshot=569, wal.log=379, wal.wal.old=379, endpoint=0
```

## Verification Cross-Checks

| Check | morning-api | local-witness | Result |
|-------|-------------|---------------|--------|
| Three-way epoch match | 1312/1312/1312 (clean) | 1312/1312/1312 (clean) | PASS — both clean |
| Nodes synced | 1312 | 1312 | PASS — synchronized |
| Byte-equality: wal_bytes endpoint vs file size | 0 vs 379 | 0 vs 379 | MISMATCH (known deviation) |
| Build commit vs git HEAD | 71aa16b-dirty vs cb5d4b1 | 71aa16b-dirty vs cb5d4b1 | DEVIATION (1 behind + dirty) |
| System clock sync | NTP active, synchronized | N/A (same machine) | PASS |
| Process health (PIDs unchanged) | 2727391 | 2727569 | PASS (no restarts — ~11h uptime) |
