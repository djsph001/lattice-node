# Observer Evidence Record — 2026-07-28 (Pass 48)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T05:16Z (05:16:28–05:17:26 bundle)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Forty-eighth observation pass. Same processes since 2026-07-27T18:48Z (~10.4h runtime). ~8 min since pass 47 (05:08Z).

**Summary:** Routine continuation. Both nodes at epoch 1257–1259 (1-off capture-time artifact, not genuine divergence). Snapshot epoch 1240→1250 (1 rotation). No new deviations. All three persistent deviations unchanged. Three-way epoch clean on both nodes — first clean match on api side since ~pass 40 boundary races ceased.

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

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 37682 (~10.5h) | — | None (pass 47: 37171; Δ = +511s ≈ 8.5 min — consistent with ~8 min real time + startup-report compensation) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 1 commit behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 989.97 | ~1000, slowly decaying | None (pass 47: 990.10; Δ = -0.13 over ~8.5 min — normal decay ~0.015/min, consistent) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 37683 (~10.5h) | — | None (pass 47: 37170; Δ = +513s ≈ 8.6 min — within 2s of api delta) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=3766, silence_secs=8, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=3769, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 47:** Heartbeats api +51 (3715→3766), witness +51 (3718→3769). Both ~6.0/min. Silence: api 7s→8s, witness 0s→3s (both well within <30s threshold, normal variance). Queue depth 0 on both.

---

## Epoch State

### morning-api (05:16:28Z socket, 05:17:26Z log)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1257 (socket 05:16); 1259 (socket 05:17); 1259 (log 05:17) | Cycling. +17 since pass 47 (1240→1257 socket via 05:16 capture). | **PASS — clean three-way match at 05:17.** Socket=1259, grep count=1259, last log line=1259. No boundary race — first clean api match since pass 40. |
| ratio | 1.019706 | ~1.01–1.02 steady state | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (05:16Z socket bundle, 05:17:13Z log)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1257 (socket 05:16); 1258 (socket 05:17); 1258 (log 05:17) | Cycling. +17 since pass 47 (1240→1257 socket). | **PASS — clean three-way match.** Socket=1258, grep count=1258, last log line=1258. Three-way clean. |
| ratio | 1.08266 | Continuing asymptotic decline | None (pass 47: 1.08389; Δ = -0.00123 — normal decline ~0.00015/epoch) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization

**OBSERVED:** At 05:16Z capture: both nodes at 1257 (socket). At 05:17Z recapture: api at 1259, witness at 1258. The 1-off gap is explained by capture timing (api captured ~13s after witness — enough for witness to be 1 epoch behind at 30s cadence, but api has ticked past). No genuine divergence.

**CLASSIFICATION:** Normal — capture-timing artifact.

### Epoch cadence

Morning-api: +19 epochs in ~9.5 min ≈ 30s/epoch. Witness: +18 epochs ≈ 30s/epoch. Consistent with 30s default timer.

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
| last_snapshot_epoch | 1250 | Incrementing by 10 each rotation (pass 47: 1240; +10 epochs = 1 rotation) | None (normal — 1 rotation since pass 47) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (05:16Z single capture):**
- `state.snapshot`: 894 bytes (mtime: 2026-07-28T01:12 EDT — epoch 1250 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T01:12 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T01:07 EDT — previous epoch 1240 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- state.snapshot size: **894 bytes** (pass 47: 894 — stable. The 894↔895 oscillation is resolved; now stable through 3 rotations.)

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1250 | Incrementing by 10 (pass 47: 1240; +10 epochs = 1 rotation) | None (both nodes synchronized) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (05:17Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T01:13 EDT — epoch 1250 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T01:13 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T01:08 EDT — previous epoch 1240 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### Snapshot synchronization

Both nodes at last_snapshot_epoch=1250 (was 1240 in pass 47 — 1 rotation since). File mtimes within ~1 min (01:12 vs 01:13 EDT). Snapshot rotation working normally. **Note:** 1 rotation in 8 min vs 2 rotations in 9 min (pass 46→47). Consistent with the 8 min vs 9 min interval difference.

### state.snapshot size (morning-api)

**OBSERVED:** 894 bytes (unchanged from pass 47). Stable through 3+ snapshot rotations. The earlier 894↔895 oscillation has not recurred since pass 46. Convergence confirmed.

**CLASSIFICATION:** Not a deviation. Normal serialization precision variation that has stabilized.

---

## Metrics (from heartbeat logs)

### morning-api — latest ticks (05:16:56–05:17:06Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=2-3s`
**CLASSIFICATION:** Clean.

### local-witness — latest ticks (05:16:53–05:17:03Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s`
**CLASSIFICATION:** Clean.

**All observed ticks (05:08–05:17Z):** Every tick on both nodes shows `aged=0`, `outbound_queues=[]`. No stale fetch entries, no queue buildup, no zombie evictions, no sweep events.

---

## Error Health Scan

### morning-api
**OBSERVED:** 132 total WARN/ERROR lines (+1 since pass 47: 131→132). After filtering (expected patterns: `skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|No known peers|Failed to trigger`): **2 startup-only lines** from Jul 27 18:48Z (`Failed to gossip genesis`, `Failed to publish block proposal_id=genesis` — expected at bootstrap with zero peers). No new unexpected lines.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean. The 2 startup lines are pre-existing from initial launch and have not changed.

### local-witness
**OBSERVED:** 122 total WARN/ERROR lines (unchanged from pass 47). After filtering (same expected patterns): **zero unexpected lines.** All expected: startup messages + `insufficient balance` rejections + non-mDNS note.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

---

## Delta Summary (Pass 47 → Pass 48)

| Metric | Pass 47 (05:08Z) | Pass 48 (05:16Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch (socket) | 1240 | 1257→1259 | +19 (~9.5 min) | Normal cycling (30s/epoch) |
| witness epoch (socket) | 1240 | 1257→1258 | +18 (~9.5 min) | Capture-timing 1-off, not real divergence |
| Three-way match | api: 1-off race, witness: clean | api: CLEAN, witness: CLEAN | **Improved** | First clean api match since pass 40 |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch (api) | 1240 | 1250 | +10 (1 rotation) | Normal 10-epoch interval |
| Snapshot epoch (witness) | 1240 | 1250 | +10 (1 rotation) | Synchronized |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk (api) | 379 bytes, 01:07 EDT | 379 bytes, 01:12 EDT | 0 size; mtime +5 min | Snapshot rotation working |
| wal.log on disk (witness) | 379 bytes, 01:08 EDT | 379 bytes, 01:13 EDT | 0 size; mtime +5 min | Snapshot rotation working |
| state.snapshot (api) | 894 bytes, 01:07 | 894 bytes, 01:12 | 0 | Stable (oscillation resolved) |
| state.snapshot (witness) | 569 bytes, 01:08 | 569 bytes, 01:13 | 0 | Size stable |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 1 commit behind HEAD + dirty tree |
| Heartbeats (api) | 3715 | 3766 | +51 | Normal — ~6.0/min |
| Heartbeats (witness) | 3718 | 3769 | +51 | Normal — ~6.0/min |
| max_peer_silence (api) | 3s | 2-3s | 0 | Well within <30s |
| max_peer_silence (witness) | 6s | 6s | 0 | Well within <30s |
| Sweep/evict events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~30s/epoch | ~30s/epoch | — | Within normal variance |
| Total WARN/ERROR (api) | 131 | 132 | +1 | Gradual accumulation; all expected |
| Total WARN/ERROR (witness) | 122 | 122 | 0 | Unchanged |
| RSS (api) | ~22.5 MB | ~22.5 MB | 0 | Stable |
| RSS (witness) | ~23.0 MB | ~23.0 MB | 0 | Stable |

### New observations this pass

1. **Clean three-way epoch match on morning-api.** Socket=1259, grep count=1259, last log line=epoch=1259. All three agree. This is the first clean match on the api side since boundary races were common (~pass 40). The pass 47 1-off race is resolved by coincident capture timing.

2. **1-off epoch between nodes at 05:17 capture.** Api at 1259, witness at 1258. Causal: witness captured ~13s before api ticked over. Not a real divergence — both were at 1257 in the earlier 05:16 capture. Verified as capture-timing artifact.

3. **state.snapshot (api) at 894 bytes for 3rd consecutive rotation.** The 894↔895 oscillation from passes 44–46 is definitively resolved. Now stable through 3 snapshot rotations (epochs 1230, 1240, 1250).

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

None this pass.

---

## Evidence Files

- Previous: `docs/evidence/observer-2026-07-28-pass47.md`
- This: `docs/evidence/observer-2026-07-28-pass48.md`

---

## Raw Capture Bundle

```json
// GetNodeInfo (morning-api) — 05:16:28Z
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":37682,"build_commit":"71aa16b-dirty","thickness":989.966353345482}

// GetPeers (morning-api) — 05:16:28Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":3766,"silence_secs":8,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api) — 05:16:28Z
{"type":"EpochState","epoch":1257,"ratio":1.0197063950658798,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (morning-api) — 05:16:28Z
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api) — 05:16:28Z
{"type":"PersistenceState","last_snapshot_epoch":1250,"wal_bytes":0,"wal_entries":0}

// Three-way epoch (morning-api) — 05:17Z recapture
// Socket: 1259
// grep -c "Epoch complete": 1259
// Last log line: epoch=1259 balance_before=20 balance_after=20 ratio=1.02 (05:17:26Z)

// GetNodeInfo (local-witness) — 05:16Z bundle
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":37683,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness) — 05:16Z bundle
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":3769,"silence_secs":3,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness) — 05:16Z bundle
{"type":"EpochState","epoch":1257,"ratio":1.0826635749817974,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (local-witness) — 05:16Z bundle
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness) — 05:16Z bundle
{"type":"PersistenceState","last_snapshot_epoch":1250,"wal_bytes":0,"wal_entries":0}

// Three-way epoch (local-witness) — 05:17Z recapture
// Socket: 1258
// grep -c "Epoch complete": 1258
// Last log line: epoch=1258 balance_before=0 balance_after=0 ratio=1.08 (05:17:13Z)

// File system (05:16Z) — wal_bytes mismatch confirmed
// morning-api: state.snapshot=894, wal.log=379, wal.wal.old=379, endpoint=0
// local-witness: state.snapshot=569, wal.log=379, wal.wal.old=379, endpoint=0
```

## Verification Cross-Checks

| Check | morning-api | local-witness | Result |
|-------|-------------|---------------|--------|
| Three-way epoch match | 1259/1259/1259 (clean) | 1258/1258/1258 (clean) | PASS — both clean |
| Nodes synced | 1259 | 1258 | PASS — 1-off is capture-timing artifact, not genuine divergence |
| Byte-equality: wal_bytes endpoint vs file size | 0 vs 379 | 0 vs 379 | MISMATCH (known deviation) |
| Build commit vs git HEAD | 71aa16b-dirty vs cb5d4b1 | 71aa16b-dirty vs cb5d4b1 | DEVIATION (1 behind + dirty) |
| System clock sync | NTP active, synchronized | N/A (same machine) | PASS |
| Process health (RSS) | ~22.5 MB | ~23.0 MB | Stable |
| PIDs unchanged | 2727391 | 2727569 | PASS (no restarts) |
