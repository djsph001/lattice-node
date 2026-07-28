# Observer Evidence Record — 2026-07-28 (Pass 55)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** 2026-07-28T06:37:06–06:37:56Z bundle (02:37:06–02:37:56 EDT)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Fifty-fifth observation pass. Same processes since 2026-07-27T18:48Z (~11.9h runtime). ~8 min since pass 54 (06:29Z).

**Summary:** Routine continuation. Both nodes at epoch 1418 — fully synchronized. Three-way epoch: recapture boundary race (expected — ~50s gap between socket and log queries). Snapshot epoch 1400→1410 (1 rotation since pass 54). No new deviations. All three persistent deviations unchanged.

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
| uptime_secs | 42510 (06:37Z) | — | None (pass 54: 42034; Δ = +476s ≈ 7.9 min — matches elapsed real time) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 1 commit behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 988.69 | ~1000, slowly decaying | None (pass 54: 988.89; Δ = -0.20 over ~8 min — consistent decay ~0.025/min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 42526 (06:37Z) | — | None (pass 54: 42033; Δ = +493s ≈ 8.2 min — within 17s of api delta, consistent) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED (06:37Z):** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=4250, silence_secs=2, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED (06:37Z):** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=4254, silence_secs=6, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 54:** Heartbeats api +49 (4201→4250), witness +53 (4201→4254). Both ~6.2–6.6/min (~8min window). Silence: api 1-3s, witness 6s — well within threshold. Queue depth 0 on both.

---

## Epoch State

### morning-api (recapture 06:37:06Z; log grep 06:37:56Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1418 (socket 06:37Z) | Cycling. +16 since pass 54 (1402→1418) in ~8 min. | **RACE at recapture.** Socket=1418, grep=1420, last_log=1420 (epochs ticked during ~50s gap between queries). Log and grep agree. |
| ratio | 1.019735 | ~1.01–1.02 steady state | None (pass 54: 1.019730; Δ = +0.000005 — essentially unchanged) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (recapture 06:37:13Z; log grep 06:37:43Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1418 (socket 06:37Z) | Cycling. +16 since pass 54 (1402→1418) in ~8 min. | **RACE at recapture.** Same boundary — socket=1418, grep=1419, last_log=1419. Nodes fully synchronized. |
| ratio | 1.07253 | Continuing asymptotic decline | None (pass 54: 1.07372; Δ = -0.00119 — normal decline ~0.000056/epoch, consistent) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** Both nodes at same epoch (1418) throughout capture. Synchronized.

### Epoch cadence
+16 epochs since pass 54 (06:29Z) in ~8 min ≈ 30s/epoch. Both nodes consistent. Within normal variance (28–32s/epoch observed range).

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
| last_snapshot_epoch | 1410 | Incrementing by 10 (pass 54: 1400; +10 = 1 rotation) | None (normal — 1 rotation since pass 54) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (06:37Z single capture):**
- `state.snapshot`: 895 bytes (mtime: 2026-07-28T02:37 EDT — epoch 1410 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T02:37 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T02:32 EDT — previous epoch 1400 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- state.snapshot size: **895 bytes** (pass 54 at epoch 1400: 894 bytes; Δ = +1 byte across 10 epochs — expected as epoch metadata changes)

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1410 | Incrementing by 10 (pass 54: 1400; +10 = 1 rotation) | None (normal — 1 rotation since pass 54) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (06:37Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T02:33 EDT — epoch 1410 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T02:33 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T02:28 EDT — previous epoch 1400 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### Snapshot synchronization
Both nodes at last_snapshot_epoch=1410 (pass 54: 1400). File mtimes: api 02:37 vs witness 02:33 EDT (~4 min offset). One rotation since pass 54. Normal.

### state.snapshot size (morning-api)
895 bytes vs 894 at pass 54 (epoch 1400). +1 byte across 10 epochs. Expected — epoch metadata encodes different epoch number. Not a concern.

---

## Metrics (from heartbeat logs)

### morning-api — latest ticks (06:37:26–06:37:46Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s`
**CLASSIFICATION:** Clean.

### local-witness — latest ticks (06:37:33–06:37:53Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s`
**CLASSIFICATION:** Clean.

**All observed ticks (~8 min window):** Every tick on both nodes shows `aged=0`, `outbound_queues=[]`. No stale fetch entries, no queue buildup, no zombie evictions, no sweep events.

---

## Error Health Scan

### morning-api
**OBSERVED:** 148 total WARN/ERROR lines (+2 since pass 54: 146→148). After filtering (expected patterns + genesis startup warnings): **zero unexpected lines.** The +2 is normal kad bootstrap accumulation (expected with `--no-mdns`).
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

### local-witness
**OBSERVED:** 122 total WARN/ERROR lines (unchanged from pass 54). After filtering: **zero unexpected lines.**
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

---

## Delta Summary (Pass 54 → Pass 55)

| Metric | Pass 54 (06:29Z) | Pass 55 (06:37Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch (recapture) | 1402 | 1418 | +16 (~8 min) | Normal cycling (~30s/epoch) |
| witness epoch (recapture) | 1402 | 1418 | +16 (~8 min) | Synchronized with api |
| Three-way match | Boundary race at 2nd recapture — both nodes agree | Socket=1418, grep=1420, last log=1420 (api); socket=1418, grep=1419, last log=1419 (witness) — boundary race at ~50s recapture gap | — | Both nodes agree — not divergence |
| Epoch sync | 0 (both 1402) | 0 (both 1418) | None | Fully synchronized |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch (api) | 1400 | 1410 | +10 (1 rotation) | Normal 10-epoch interval |
| Snapshot epoch (witness) | 1400 | 1410 | +10 (1 rotation) | Synchronized |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk (api) | 379 bytes, 02:27 EDT | 379 bytes, 02:37 EDT | 0 size; mtime +10 min | Snapshot rotation working |
| wal.log on disk (witness) | 379 bytes, 02:23 EDT | 379 bytes, 02:33 EDT | 0 size; mtime +10 min | Snapshot rotation working |
| state.snapshot (api) | 894 bytes, 02:27 | 895 bytes, 02:37 | +1 byte | Expected — epoch metadata change |
| state.snapshot (witness) | 569 bytes, 02:23 | 569 bytes, 02:33 | 0 | Stable |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 1 commit behind HEAD + dirty tree |
| Heartbeats (api) | 4201 | 4250 | +49 | Normal — ~6.1/min |
| Heartbeats (witness) | 4201 | 4254 | +53 | Normal — ~6.6/min |
| max_peer_silence (api) | 1-3s | 2-3s | ~0 | Well within <30s |
| max_peer_silence (witness) | 4-6s | 6s | ~0 | Well within <30s |
| Sweep/evict events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~30.6s/epoch | ~30s/epoch | Slight variance | Within normal range |
| Total WARN/ERROR (api) | 146 | 148 | +2 | Kad bootstrap noise; all expected |
| Total WARN/ERROR (witness) | 122 | 122 | 0 | Unchanged |

### New observations this pass
1. **Both nodes fully synchronized at epoch 1418.** Recapture boundary race at ~50s gap between socket and log queries, but both nodes agree on the boundary.
2. **One snapshot rotation** since pass 54 (1400→1410). Consistent with ~4 min epoch cadence × 10 epochs.
3. **state.snapshot size +1 byte** (api: 894→895 at epoch 1400→1410). Expected — epoch metadata incremented.
4. **Uptime approaching 12 hours** (42510s ≈ 11.8h). Process health stable — no restarts.

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

- Previous: `docs/evidence/observer-2026-07-28-pass54.md`
- This: `docs/evidence/observer-2026-07-28-pass55.md`

---

## Raw Capture Bundle

```json
// Timestamp — 06:37:06Z
// GetNodeInfo (morning-api) — 06:37:06Z
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":42510,"build_commit":"71aa16b-dirty","thickness":988.6885108262346}

// GetPeers (morning-api) — 06:37:06Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":4250,"silence_secs":2,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api) — 06:37:06Z
{"type":"EpochState","epoch":1418,"ratio":1.0197350567455432,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (morning-api) — 06:37:06Z
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api) — 06:37:06Z
{"type":"PersistenceState","last_snapshot_epoch":1410,"wal_bytes":0,"wal_entries":0}

// GetNodeInfo (local-witness) — 06:37:13Z
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":42526,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness) — 06:37:13Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":4254,"silence_secs":6,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness) — 06:37:13Z
{"type":"EpochState","epoch":1418,"ratio":1.0725285235834163,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (local-witness) — 06:37:13Z
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness) — 06:37:13Z
{"type":"PersistenceState","last_snapshot_epoch":1410,"wal_bytes":0,"wal_entries":0}

// Three-way epoch (morning-api) — 06:37:56Z
// Socket: 1418; grep -c "Epoch complete": 1420; last log line: epoch=1420
// RACE — ~50s gap between socket and log queries.

// Three-way epoch (local-witness) — 06:37:43Z
// Socket: 1418; grep -c "Epoch complete": 1419; last log line: epoch=1419
// RACE — same phenomenon. Both nodes fully synchronized.

// File system (06:37Z) — wal_bytes mismatch confirmed
// morning-api: state.snapshot=895, wal.log=379, wal.wal.old=379, endpoint=0
// local-witness: state.snapshot=569, wal.log=379, wal.wal.old=379, endpoint=0
```

## Verification Cross-Checks

| Check | morning-api | local-witness | Result |
|-------|-------------|---------------|--------|
| Three-way epoch match | 1418/1420/1420 (boundary race — 50s gap) | 1418/1419/1419 (boundary race — 30s gap) | PASS — both agree on synchronous epochs |
| Nodes synced | 1418 | 1418 | PASS — synchronized |
| Byte-equality: wal_bytes endpoint vs file size | 0 vs 379 | 0 vs 379 | MISMATCH (known deviation) |
| Build commit vs git HEAD | 71aa16b-dirty vs cb5d4b1 | 71aa16b-dirty vs cb5d4b1 | DEVIATION (1 behind + dirty) |
| System clock sync | NTP active, synchronized | N/A (same machine) | PASS |
| Process health (PIDs unchanged) | 2727391 | 2727569 | PASS (no restarts — ~11.9h uptime) |
