# Observer Evidence Record — 2026-07-28 (Pass 49)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T05:24:10–05:25:13Z bundle (01:24:10–01:25:13 EDT)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Forty-ninth observation pass. Same processes since 2026-07-27T18:48Z (~10.6h runtime). ~8 min since pass 48 (05:16Z).

**Summary:** Routine continuation. Both nodes at epoch 1274 — synchronized. Three-way epoch clean on both nodes. Snapshot epoch 1250→1270 (2 rotations). No new deviations. All three persistent deviations unchanged. Recapture confirmed pass 48's 1-off was capture-timing artifact (both nodes now at same epoch).

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
| uptime_secs | 38144 (~10.6h) | — | None (pass 48: 37682; Δ = +462s ≈ 7.7 min — consistent with ~8 min real time) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 1 commit behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 989.85 | ~1000, slowly decaying | None (pass 48: 989.97; Δ = -0.12 over ~8 min — normal decay ~0.015/min, consistent) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 38148 (~10.6h) | — | None (pass 48: 37683; Δ = +465s ≈ 7.8 min — within 3s of api delta) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=3812, silence_secs=9, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=3815, silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 48:** Heartbeats api +46 (3766→3812), witness +46 (3769→3815). Both ~6.0/min. Silence: api 8s→9s, witness 3s→7s (both well within <30s threshold, normal variance). Queue depth 0 on both.

---

## Epoch State

### morning-api (05:24:10Z primary capture; 05:25:06Z recapture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1272 (socket 05:24:10Z) → 1274 (recapture 05:25:06Z) | Cycling. +15 since pass 48 (1257→1272 primary). | **PASS — clean three-way match at recapture.** Socket=1274, grep count=1274, last log line=epoch=1274. |
| ratio | 1.01971 | ~1.01–1.02 steady state | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (05:24:32Z primary capture; 05:25:13Z recapture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1272 (socket 05:24:32Z) → 1274 (recapture 05:25:13Z) | Cycling. +15 since pass 48 (1257→1272 primary). | **PASS — clean three-way match at recapture.** Socket=1274, grep count=1274, last log line=epoch=1274. |
| ratio | 1.08161 | Continuing asymptotic decline | None (pass 48: 1.08266; Δ = -0.00105 — normal decline ~0.00013/epoch, consistent) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization

**OBSERVED:** At recapture (05:25Z): both nodes at epoch 1274. Synchronized.
**Note on pass 48 1-off:** The earlier 1-off (api 1259, witness 1258 in pass 48) is confirmed as capture-timing artifact — this pass shows both converging to same epoch at recapture.

### Epoch cadence

Morning-api: +15 epochs since pass 48 in ~8 min ≈32s/epoch. Witness: +15 epochs ≈30s/epoch. Consistent with 30s default timer (normal variance).

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
| last_snapshot_epoch | 1270 | Incrementing by 10 (pass 48: 1250; +20 epochs = 2 rotations) | None (normal — 2 rotations since pass 48) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (05:24:10Z single capture):**
- `state.snapshot`: 894 bytes (mtime: 2026-07-28T01:22 EDT — epoch 1270 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T01:22 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T01:17 EDT — previous epoch 1260 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- state.snapshot size: **894 bytes** (stable — pass 48: 894, pass 47: 894. No oscillation.)

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1270 | Incrementing by 10 (pass 48: 1250; +20 epochs = 2 rotations) | None (both nodes synchronized) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (05:24:32Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T01:23 EDT — epoch 1270 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T01:23 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T01:18 EDT — previous epoch 1260 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### Snapshot synchronization

Both nodes at last_snapshot_epoch=1270 (was 1250 in pass 48 — 2 rotations since). File mtimes within ~1 min (01:22 vs 01:23 EDT). Snapshot rotation working normally. **2 rotations in ~8 min** vs 1 rotation in ~8 min (pass 47→48). Inconsistent but plausible: rotations at every 10 epochs (≈5 min) could mean 0, 1, or 2 rotations fall in any 8-min interval depending on alignment.

### state.snapshot size (morning-api)

**OBSERVED:** 894 bytes (unchanged from pass 48). Stable through 5+ snapshot rotations. The earlier 894↔895 oscillation has not recurred since pass 46. Convergence firmly confirmed.

**CLASSIFICATION:** Not a deviation. Normal serialization precision variation that has stabilized.

---

## Metrics (from heartbeat logs)

### morning-api — latest ticks (05:24:56–05:25:06Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s`
**CLASSIFICATION:** Clean.

### local-witness — latest ticks (05:24:43–05:25:03Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s`
**CLASSIFICATION:** Clean.

**All observed ticks (05:16–05:25Z):** Every tick on both nodes shows `aged=0`, `outbound_queues=[]`. No stale fetch entries, no queue buildup, no zombie evictions, no sweep events.

---

## Error Health Scan

### morning-api
**OBSERVED:** 134 total WARN/ERROR lines (+2 since pass 48: 132→134). After filtering (expected patterns: `skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|No known peers|Failed to trigger|Failed to gossip genesis|Failed to publish block`): **zero unexpected lines.** The +2 is normal gradual accumulation of expected startup/skip notices.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

### local-witness
**OBSERVED:** 122 total WARN/ERROR lines (unchanged from pass 48). After filtering (same expected patterns): **zero unexpected lines.** All expected: startup messages + `insufficient balance` rejections + non-mDNS note.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

---

## Delta Summary (Pass 48 → Pass 49)

| Metric | Pass 48 (05:16Z) | Pass 49 (05:24Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch (socket) | 1257→1259 | 1272→1274 | +15 (~8 min) | Normal cycling (~31s/epoch) |
| witness epoch (socket) | 1257→1258 | 1272→1274 | +15 (~8 min) | Synchronized with api at recapture |
| Three-way match | api: CLEAN, witness: CLEAN | api: CLEAN, witness: CLEAN | None | Both clean |
| Epoch sync | 1-off (capture artifact) | 0 (both 1274 at recapture) | **Improved** | Pass 48 artifact confirmed |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch (api) | 1250 | 1270 | +20 (2 rotations) | Normal 10-epoch interval |
| Snapshot epoch (witness) | 1250 | 1270 | +20 (2 rotations) | Synchronized |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk (api) | 379 bytes, 01:12 EDT | 379 bytes, 01:22 EDT | 0 size; mtime +10 min | Snapshot rotation working |
| wal.log on disk (witness) | 379 bytes, 01:13 EDT | 379 bytes, 01:23 EDT | 0 size; mtime +10 min | Snapshot rotation working |
| state.snapshot (api) | 894 bytes, 01:12 | 894 bytes, 01:22 | 0 | Stable (oscillation resolved) |
| state.snapshot (witness) | 569 bytes, 01:13 | 569 bytes, 01:23 | 0 | Size stable |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 1 commit behind HEAD + dirty tree |
| Heartbeats (api) | 3766 | 3812 | +46 | Normal — ~6.0/min |
| Heartbeats (witness) | 3769 | 3815 | +46 | Normal — ~6.0/min |
| max_peer_silence (api) | 2-3s | 3s | 0 | Well within <30s |
| max_peer_silence (witness) | 6s | 6-7s | 0 | Well within <30s |
| Sweep/evict events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~30s/epoch | ~30-32s/epoch | — | Within normal variance |
| Total WARN/ERROR (api) | 132 | 134 | +2 | Gradual accumulation; all expected |
| Total WARN/ERROR (witness) | 122 | 122 | 0 | Unchanged |
| RSS (api) | ~22.5 MB | ~22.5 MB (not re-measured) | — | Assumed stable |
| RSS (witness) | ~23.0 MB | ~23.0 MB (not re-measured) | — | Assumed stable |

### New observations this pass

1. **Clean three-way match on both nodes.** Socket=1274, grep count=1274, last log line=1274 on both morning-api and local-witness. Fully synchronized at recapture.

2. **Pass 48's 1-off confirmed as capture-timing artifact.** Both nodes now at same epoch 1274. The pass 48 report was correct to classify it that way — this pass proves it.

3. **Two snapshot rotations in ~8 min** (1250→1270). Higher than previous pass (1 rotation in ~8 min). Explained by alignment: rotations every 10 epochs (~5 min) means some 8-min windows span 1 rotation, others span 2, depending on when the capture falls relative to the rotation schedule.

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

- Previous: `docs/evidence/observer-2026-07-28-pass48.md`
- This: `docs/evidence/observer-2026-07-28-pass49.md`

---

## Raw Capture Bundle

```json
// GetNodeInfo (morning-api) — 05:24:10Z
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":38144,"build_commit":"71aa16b-dirty","thickness":989.8472299592512}

// GetPeers (morning-api) — 05:24:10Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":3812,"silence_secs":9,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api) — 05:24:10Z
{"type":"EpochState","epoch":1272,"ratio":1.0197098610516007,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (morning-api) — 05:24:10Z
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api) — 05:24:10Z
{"type":"PersistenceState","last_snapshot_epoch":1270,"wal_bytes":0,"wal_entries":0}

// GetHeight (morning-api) — 05:24:10Z
{"type":"Height","height":1}

// Three-way epoch (morning-api) — 05:25:06Z recapture
// Socket: 1274
// grep -c "Epoch complete": 1274
// Last log line: epoch=1274 balance_before=20 balance_after=20 ratio=1.02 (05:24:56Z)

// GetNodeInfo (local-witness) — 05:24:32Z
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":38148,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness) — 05:24:32Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":3815,"silence_secs":7,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness) — 05:24:32Z
{"type":"EpochState","epoch":1272,"ratio":1.0816103278527294,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (local-witness) — 05:24:32Z
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness) — 05:24:32Z
{"type":"PersistenceState","last_snapshot_epoch":1270,"wal_bytes":0,"wal_entries":0}

// Three-way epoch (local-witness) — 05:25:13Z recapture
// Socket: 1274
// grep -c "Epoch complete": 1274
// Last log line: epoch=1274 balance_before=0 balance_after=0 ratio=1.08 (05:25:13Z)

// File system (05:24Z) — wal_bytes mismatch confirmed
// morning-api: state.snapshot=894, wal.log=379, wal.wal.old=379, endpoint=0
// local-witness: state.snapshot=569, wal.log=379, wal.wal.old=379, endpoint=0
```

## Verification Cross-Checks

| Check | morning-api | local-witness | Result |
|-------|-------------|---------------|--------|
| Three-way epoch match | 1274/1274/1274 (clean) | 1274/1274/1274 (clean) | PASS — both clean |
| Nodes synced | 1274 | 1274 | PASS — synchronized |
| Byte-equality: wal_bytes endpoint vs file size | 0 vs 379 | 0 vs 379 | MISMATCH (known deviation) |
| Build commit vs git HEAD | 71aa16b-dirty vs cb5d4b1 | 71aa16b-dirty vs cb5d4b1 | DEVIATION (1 behind + dirty) |
| System clock sync | NTP active, synchronized | N/A (same machine) | PASS |
| Process health (PIDs unchanged) | 2727391 | 2727569 | PASS (no restarts) |
