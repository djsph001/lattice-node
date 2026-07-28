# Observer Evidence Record — 2026-07-28 (Pass 45)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T04:50:00Z (04:50Z; three-way bundles at 04:49Z api, 04:50Z witness)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Forty-fifth observation pass. Same processes since 2026-07-27T18:48Z (~10.0h runtime). ~8.5 min since pass 44 (04:41Z).

**Summary:** Routine continuation. Both nodes at epoch 1203/1204 (reversed 1-epoch boundary race vs pass 44). Three-way PASS on both. Snapshot epoch advanced from 1180→1200 (2 normal rotations). API state.snapshot size changed 895→894 bytes (minor serialization change). No new deviations. All three persistent deviations unchanged.

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
| uptime_secs | 36033 (~10.0h) | — | None (pass 44: 35557; Δ = +476s ≈ 7.9 min — consistent with ~8.5 min real time) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 1 commit behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 990.403 | ~1000, slowly decaying | None (pass 44: 990.530; Δ = -0.127 over ~8.5 min — normal decay ~0.015/min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 36040 (~10.0h) | — | None (pass 44: 35563; Δ = +477s — within ~1s of api delta) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=3602, silence_secs=5, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=3605, silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 44:** Heartbeats api +48 (3554→3602), witness +48 (3557→3605). Silence: api 3s→5s, witness 6s→7s (both well within <30s threshold). Queue depth 0 on both. Normal variance.

---

## Epoch State

### morning-api (04:49Z three-way bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1203 | Cycling. +15 since pass 44 (1188→1203). | **PASS — three-way match.** Socket=1203, grep -c=1203, last log line=1203. All agree. |
| ratio | 1.01969 | ~1.01–1.02 steady state (pass 44: 1.01969) | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (04:49Z simultaneous capture):**
- Socket epoch: **1203**
- `grep -c` count: **1203**
- Last log line epoch: **1203** (at 04:49:26Z)
- **PASS.** All three agree.

### local-witness (04:50Z three-way bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1204 | Cycling. +17 since pass 44 (1187→1204). | **PASS — three-way match.** Socket, grep count, and last log line all at 1204. |
| ratio | 1.08660 | Continuing asymptotic decline (pass 44: 1.08793; Δ = -0.00133 — slightly larger drop than usual ~0.0004/epoch) | None (monotonic decay expected; per-epoch drop can vary) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (04:50Z simultaneous capture):**
- Socket epoch: **1204**
- `grep -c` count: **1204**
- Last log line epoch: **1204** (at 04:50:13Z)
- **PASS.** All three agree.

### Epoch synchronization

**OBSERVED:** morning-api at 1203, local-witness at 1204 at respective capture times. Reversed from pass 44 (where api=1188, witness=1187). Witness now 1 epoch ahead.
**CLASSIFICATION:** Boundary race — reversed direction. Witness ticked at 04:50:13Z, api at 04:49:26Z (~47s gap). Consistent with ~30s epoch cadence and ~10-20s tick offset between nodes. **Not a persistent offset.**

### Epoch cadence

morning-api: +15 epochs in ~8.5 min ≈ ~34s/epoch. Witness: +17 epochs in same window ≈ ~30s/epoch. Variance normal for two independent tokio timers.

---

## Economic State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged since pass 44) |
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
| last_snapshot_epoch | 1200 | Incrementing by 10 each rotation (pass 44: 1180; +20 epochs = 2 rotations) | None (normal — 2 rotations since pass 44) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (04:50Z single capture):**
- `state.snapshot`: 894 bytes (mtime: 2026-07-28T00:47 EDT — epoch 1200 snapshot, recently rotated)
- `wal.log`: 379 bytes (mtime: 2026-07-28T00:47 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T00:42 EDT — previous epoch 1190 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- state.snapshot size: 894 bytes (pass 44: 895; Δ = -1 — minor change, likely serialization rounding difference)

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1200 | Incrementing by 10 (pass 44: 1180; +20 epochs = 2 rotations) | None (both nodes at 1200; synchronized) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (04:50Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T00:48 EDT — epoch 1200 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T00:48 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T00:43 EDT — previous epoch 1190 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### Snapshot synchronization

Both nodes at last_snapshot_epoch=1200 (was 1180 in pass 44 — 2 rotations). File mtimes within ~1 min (00:47 vs 00:48 EDT). Snapshot rotation working normally.

### state.snapshot size change (morning-api)

**OBSERVED:** API state.snapshot changed from 895 bytes (pass 44) to 894 bytes (this pass). Witness snapshot unchanged at 569 bytes.
**CLASSIFICATION:** UNKNOWN. Possible serialization drift in floating-point state field (thickness?). 1-byte change suggests a numerical precision difference, not structural corruption. First observed this pass. No functional impact expected.

---

## Metrics (from heartbeat logs)

### morning-api — latest tick (04:49:56Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s`
**CLASSIFICATION:** Clean. Zero fetches, zero queues, max peer silence well within threshold.

### local-witness — latest tick (04:50:13Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s`
**CLASSIFICATION:** Clean. Consistent pattern — witness tick timing lags api by ~17s.

**All observed ticks (04:42–04:50Z):** Every tick on both nodes shows `aged=0`, `outbound_queues=[]`. No stale fetch entries, no queue buildup, no zombie evictions.

---

## Error Health Scan

### morning-api
**OBSERVED:** 127 total WARN/ERROR lines (+2 since pass 44: 125→127). After filtering (expected patterns: `skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|No known peers|Failed to trigger`): **only 2 startup warnings remain** — `Failed to gossip genesis` and `Failed to publish block` at 18:48:26 (both expected for initial no-peer state). +2 total lines consistent with ~2 more Kademlia/bootstrap warnings in ~9 min of runtime.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

### local-witness
**OBSERVED:** 122 total WARN/ERROR lines (unchanged from pass 44). After filtering: **zero unexpected lines.** All 122 are expected: 3× startup messages + 118× `insufficient balance` rejections + 1× `Connection from non-mDNS peer`.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

---

## Delta Summary (Pass 44 → Pass 45)

| Metric | Pass 44 (04:41Z) | Pass 45 (04:50Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch | 1188 | 1203 | +15 (~8.5 min) | Normal cycling (~34s/epoch) |
| witness epoch | 1187 | 1204 | +17 (~8.5 min) | Boundary race reversed (witness ahead by 1) |
| Three-way match | PASS both | **PASS BOTH** — all three match on each node | — | Clean |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch (api) | 1180 | 1200 | +20 (2 rotations) | Normal 10-epoch interval |
| Snapshot epoch (witness) | 1180 | 1200 | +20 (2 rotations) | Synchronized |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk (api) | 379 bytes, 00:37 EDT | 379 bytes, 00:47 EDT | 0 size; mtime +10 min | Snapshot rotation working |
| wal.log on disk (witness) | 379 bytes, 00:38 EDT | 379 bytes, 00:48 EDT | 0 size; mtime +10 min | Snapshot rotation working |
| state.snapshot (api) | 895 bytes | 894 bytes | -1 byte | Minor serialization change |
| state.snapshot (witness) | 569 bytes | 569 bytes | 0 | Size stable |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 1 commit behind HEAD + dirty tree |
| Heartbeats (api) | 3554 | 3602 | +48 | Normal — ~5.6/min |
| Heartbeats (witness) | 3557 | 3605 | +48 | Normal |
| max_peer_silence (api) | 3s | 5s | +2s | Well within <30s |
| max_peer_silence (witness) | 6s | 7s | +1s | Consistent |
| Sweep/evict events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~30s/epoch | ~30-34s/epoch | — | Within normal variance |

### New observations this pass

1. **Reversed 1-epoch boundary race.** Pass 44: api=1188, witness=1187. This pass: api=1203, witness=1204. Not a persistent offset — the lead flipped. Consistent with independent tokio timers crossing a tick boundary at different capture times.

2. **state.snapshot size change (api only):** 895→894 bytes. Witness unchanged at 569 bytes. UNKNOWN — possible serialization rounding in floating-point field. Noted for future tracking.

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

1. **state.snapshot size regression (api: 895→894 bytes).** First observed this pass (2026-07-28T04:50Z). Most likely explanation: floating-point serialization precision difference (thickness value changes by ~0.127 per epoch interval). Witness snapshot unchanged at 569 bytes — witness thickness decays symmetrically but snapshot is a different structure. No functional concern expected.

---

## Evidence Files

- Previous: `docs/evidence/observer-2026-07-28-pass44.md`
- This: `docs/evidence/observer-2026-07-28-pass45.md`

## Raw Capture Bundle

```json
// GetNodeInfo (morning-api) — 04:49Z
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":36033,"build_commit":"71aa16b-dirty","thickness":990.4032617599664}

// GetPeers (morning-api) — 04:49Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":3602,"silence_secs":5,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api) — 04:49Z three-way capture
// Socket: {"type":"EpochState","epoch":1203,...}
// grep -c "Epoch complete": 1203
// Last log line: epoch=1203 balance_before=20 balance_after=20 ratio=1.02

// GetEconomicState (morning-api) — 04:49Z
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api) — 04:49Z
{"type":"PersistenceState","last_snapshot_epoch":1200,"wal_bytes":0,"wal_entries":0}


// GetNodeInfo (local-witness) — 04:50Z
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":36040,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness) — 04:50Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":3605,"silence_secs":7,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness) — 04:50Z three-way capture
// Socket: {"type":"EpochState","epoch":1204,...}
// grep -c "Epoch complete": 1204
// Last log line: epoch=1204 balance_before=0 balance_after=0 ratio=1.09

// GetEconomicState (local-witness) — 04:50Z
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness) — 04:50Z
{"type":"PersistenceState","last_snapshot_epoch":1200,"wal_bytes":0,"wal_entries":0}
```
