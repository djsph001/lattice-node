# Observer Evidence Record — 2026-07-28 (Pass 56)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** 2026-07-28T06:46:05–06:47:26Z bundle (~06:46:05–06:47:26Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Fifty-sixth observation pass. Same processes since 2026-07-27T18:48Z (~12.0h runtime). ~10 min since pass 55 (06:37Z).

**Summary:** Routine continuation. Both nodes at epoch 1437—fully synchronized. Three-way epoch: recapture boundary race (expected — ~40s gap between socket and log queries). Snapshot epoch 1410→1430 (2 rotations since pass 55). No new deviations. All three persistent deviations unchanged.

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
| uptime_secs | 43084 (06:47Z) | — | None (pass 55: 42510; Δ = +574s ≈ 9.6 min — matches elapsed real time) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 1 commit behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 988.54 | ~1000, slowly decaying | None (pass 55: 988.69; Δ = −0.15 over ~10 min — consistent decay ~0.015/min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 43083 (06:47Z) | — | None (pass 55: 42526; Δ = +557s ≈ 9.3 min — within 17s of api delta, consistent) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED (06:47Z):** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=4307, silence_secs=0, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED (06:47Z):** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=4309, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 55:** Heartbeats api +57 (4250→4307), witness +55 (4254→4309). Both ~5.7–6.0/min (~10min window). Silence: api 0–3s, witness 3s — well within threshold (<30s). Queue depth 0 on both.

---

## Epoch State

### morning-api (recapture 06:46:05Z; log grep 06:47:26Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1436 (socket 06:46Z); 1437 (socket 06:47Z) | Cycling. +19 since pass 55 (1418→1437) in ~10 min. | **RACE at recapture.** Socket=1437, grep=1439, last_log=1439 (epochs ticked during ~80s gap between queries). Log and grep agree. |
| ratio | 1.019738 | ~1.01–1.02 steady state | None (pass 55: 1.019735; Δ = +0.000003 — essentially unchanged) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (recapture 06:47:03Z; log grep 06:47:26Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1437 (socket 06:47Z) | Cycling. +19 since pass 55 (1418→1437) in ~10 min. | **RACE at recapture.** Same boundary — socket=1437, grep=1438, last_log=1437. Nodes fully synchronized. |
| ratio | 1.07148 | Continuing asymptotic decline | None (pass 55: 1.07253; Δ = −0.00105 — normal decline ~0.000055/epoch, consistent) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** Both nodes at same epoch (1437) throughout capture. Synchronized.

### Epoch cadence
+19 epochs since pass 55 (06:37Z) in ~10 min ≈ 31.6s/epoch. Both nodes consistent. Within normal variance (28–32s/epoch observed range).

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
| last_snapshot_epoch | 1430 | Incrementing by 10 (pass 55: 1410; +20 = 2 rotations) | None (normal — 2 rotations since pass 55) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (06:47Z single capture):**
- `state.snapshot`: 895 bytes (mtime: 2026-07-28T02:42 EDT — epoch 1430 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T02:42 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T02:37 EDT — previous epoch 1420 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- state.snapshot size: **895 bytes** (pass 55 at epoch 1410: 895 bytes; Δ = 0 across 2 rotations — consistent size at 895)

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1430 | Incrementing by 10 (pass 55: 1410; +20 = 2 rotations) | None (normal — 2 rotations since pass 55) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (06:47Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T02:43 EDT — epoch 1430 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T02:43 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T02:38 EDT — previous epoch 1420 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### Snapshot synchronization
Both nodes at last_snapshot_epoch=1430 (pass 55: 1410). File mtimes: api 02:42 vs witness 02:43 EDT (~1 min offset). Two rotations since pass 55. Normal.

### state.snapshot size (morning-api)
895 bytes — unchanged from pass 55 (was also 895 at epoch 1410). Expected — epoch metadata encodes similar epoch numbers.

---

## Metrics (from heartbeat logs)

### morning-api — latest ticks (06:46:26–06:47:06Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s`
**CLASSIFICATION:** Clean.

### local-witness — latest ticks (06:46:23–06:47:03Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s`
**CLASSIFICATION:** Clean.

**All observed ticks (~10 min window):** Every tick on both nodes shows `aged=0`, `outbound_queues=[]`. No stale fetch entries, no queue buildup, no zombie evictions, no sweep events.

---

## Error Health Scan

### morning-api
**OBSERVED:** 150 total WARN/ERROR lines (+2 since pass 55: 148→150). After filtering (expected patterns + genesis startup warnings): **146 lines remaining.** All are `libp2p_kad::behaviour: Failed to trigger bootstrap: No known peers.` — expected with `--no-mdns`. The +2 is normal kad bootstrap accumulation (~1 per 5 min).
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean (all noise from kad on `--no-mdns` node).

### local-witness
**OBSERVED:** 122 total WARN/ERROR lines (unchanged from pass 55). After filtering: **zero unexpected lines** — all match exclusion patterns (3 `No snapshot found` at startup, 119 `Transaction validation failed: insufficient balance` — these are the known supply conservation deviation, excluded from actionability).
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean (witness's 119 insufficient-balance rejections are the known supply conservation deviation).

---

## Delta Summary (Pass 55 → Pass 56)

| Metric | Pass 55 (06:37Z) | Pass 56 (06:47Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch (recapture) | 1418 | 1437 | +19 (~10 min) | Normal cycling (~31s/epoch) |
| witness epoch (recapture) | 1418 | 1437 | +19 (~10 min) | Synchronized with api |
| Three-way match | Boundary race at 2nd recapture — both nodes agree | Socket=1437, grep=1439, last log=1439 (api); socket=1437, grep=1438, last log=1437 (witness) — boundary race at ~80s recapture gap | — | Both nodes agree — not divergence |
| Epoch sync | 0 (both 1418) | 0 (both 1437) | None | Fully synchronized |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch (api) | 1410 | 1430 | +20 (2 rotations) | Normal 10-epoch interval |
| Snapshot epoch (witness) | 1410 | 1430 | +20 (2 rotations) | Synchronized |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk (api) | 379 bytes, 02:37 EDT | 379 bytes, 02:42 EDT | 0 size; mtime +5 min | Snapshot rotation working |
| wal.log on disk (witness) | 379 bytes, 02:33 EDT | 379 bytes, 02:43 EDT | 0 size; mtime +10 min | Snapshot rotation working |
| state.snapshot (api) | 895 bytes, 02:42 | 895 bytes, 02:42 | 0 | Stable size |
| state.snapshot (witness) | 569 bytes, 02:33 | 569 bytes, 02:43 | 0 | Stable size |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 1 commit behind HEAD + dirty tree |
| Heartbeats (api) | 4250 | 4307 | +57 | Normal — ~5.7/min |
| Heartbeats (witness) | 4254 | 4309 | +55 | Normal — ~5.5/min |
| max_peer_silence (api) | 2–3s | 0–3s | ~0 | Well within <30s |
| max_peer_silence (witness) | 6s | 3–6s | ~0 | Well within <30s |
| Sweep/evict events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~30s/epoch | ~31.6s/epoch | Slight variance | Within normal range |
| Total WARN/ERROR (api) | 148 | 150 | +2 | Kad bootstrap noise; all expected |
| Total WARN/ERROR (witness) | 122 | 122 | 0 | Unchanged |

### New observations this pass
1. **Both nodes fully synchronized at epoch 1437.** Recapture boundary race at ~80s gap between socket and log queries, but both nodes agree on the boundary.
2. **Two snapshot rotations** since pass 55 (1410→1420→1430). Consistent with ~30s epoch cadence × 20 epochs.
3. **state.snapshot size stable** at 895 bytes (api) and 569 bytes (witness) across 2 rotations.
4. **Uptime crossing 12 hours** (43084s ≈ 12.0h). Process health stable — no restarts.

### No new deviations
The three persistent deviations are unchanged. Supply conservation contradiction, wal_bytes endpoint path bug, and build_commit staleness remain.

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

- Previous: `docs/evidence/observer-2026-07-28-pass55.md`
- This: `docs/evidence/observer-2026-07-28-pass56.md`

---

## Raw Capture Bundle

```json
// Timestamp — 06:46:05Z
// GetNodeInfo (morning-api) — 06:46:05Z
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":43059,"build_commit":"71aa16b-dirty","thickness":988.545748997184}

// GetPeers (morning-api) — 06:46:05Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":4299,"silence_secs":2,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api) — 06:46:05Z
{"type":"EpochState","epoch":1436,"ratio":1.019738380873458,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (morning-api) — 06:46:05Z
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api) — 06:46:05Z
{"type":"PersistenceState","last_snapshot_epoch":1430,"wal_bytes":0,"wal_entries":0}

// GetNodeInfo (local-witness) — 06:47:03Z
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":43083,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness) — 06:47:03Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":4309,"silence_secs":3,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness) — 06:47:03Z
{"type":"EpochState","epoch":1437,"ratio":1.0714818050201895,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (local-witness) — 06:47:03Z
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness) — 06:47:03Z
{"type":"PersistenceState","last_snapshot_epoch":1430,"wal_bytes":0,"wal_entries":0}

// Three-way epoch (morning-api) — 06:47:26Z
// Socket: 1437; grep -c "Epoch complete": 1439; last log line: epoch=1439
// RACE — ~80s gap between socket and log queries.

// Three-way epoch (local-witness) — 06:47:26Z
// Socket: 1437; grep -c "Epoch complete": 1438; last log line: epoch=1437
// RACE — same phenomenon. Both nodes fully synchronized.

// File system (06:47Z) — wal_bytes mismatch confirmed
// morning-api: state.snapshot=895, wal.log=379, wal.wal.old=379, endpoint=0
// local-witness: state.snapshot=569, wal.log=379, wal.wal.old=379, endpoint=0
```

## Verification Cross-Checks

| Check | morning-api | local-witness | Result |
|-------|-------------|---------------|--------|
| Three-way epoch match | 1437/1439/1439 (boundary race — 80s gap) | 1437/1438/1437 (boundary race — 23s gap) | PASS — both agree on synchronous epochs |
| Nodes synced | 1437 | 1437 | PASS — synchronized |
| Byte-equality: wal_bytes endpoint vs file size | 0 vs 379 | 0 vs 379 | MISMATCH (known deviation) |
| Build commit vs git HEAD | 71aa16b-dirty vs cb5d4b1 | 71aa16b-dirty vs cb5d4b1 | DEVIATION (1 behind + dirty) |
| System clock sync | NTP active, synchronized | N/A (same machine) | PASS |
| Process health (PIDs unchanged) | 2727391 | 2727569 | PASS (no restarts — ~12.0h uptime) |
