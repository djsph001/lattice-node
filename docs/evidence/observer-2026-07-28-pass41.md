# Observer Evidence Record — 2026-07-28 (Pass 41)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T04:17:32Z (04:17Z; three-way bundles at 04:16Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Forty-first observation pass. Same processes since 2026-07-27T18:48Z (~9.5h runtime). ~15 min since pass 40 (04:02Z).

**Summary:** Routine continuation. Both nodes cycling normally. Three-way PASS on both. Snapshot epoch gap from pass 40 is RESOLVED — both nodes now at last_snapshot_epoch=1130. No new deviations. All three persistent deviations unchanged.

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
| uptime_secs | 34115 (~9.5h) | — | None (pass 40: 33218; Δ = +897s ≈ 15 min — consistent with ~15 min real time) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9+ commits behind. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 990.912 | ~1000, slowly decaying | None (pass 40: 991.150; Δ = -0.238 over ~15 min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 34073 (~9.5h) | — | None (matches api within 42s) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=3404, silence_secs=2, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=3408, silence_secs=6, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 40:** Heartbeats api +84 (3320→3404). Silence: api 3s→2s, witness 6s→6s (both well within <30s threshold). Queue depth 0 on both. Normal variance.

---

## Epoch State

### morning-api (04:16Z three-way bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1137 | Cycling. +23 since pass 40 (1114→1137). | **PASS — three-way match.** Socket=1137, grep -c=1137, last log line=1137. All agree. |
| ratio | 1.01968 | ~1.01–1.02 steady state (pass 40: 1.01998) | None (stable; small variance from tick timing) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (04:16Z simultaneous capture):**
- Socket epoch: **1137**
- `grep -c` count: **1137**
- Last log line epoch: **1137** (at 04:16:26Z)
- **PASS.** All three agree.

### local-witness (04:16Z three-way bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1137 | Cycling. +24 since pass 40 (1113→1137). | **PASS — three-way match.** Socket, grep count, and last log line all at 1137. |
| ratio | 1.09208 | Continuing asymptotic decline (pass 40: 1.09421; Δ = -0.00213) | None (monotonic decay expected) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (04:16Z simultaneous capture):**
- Socket epoch: **1137**
- `grep -c` count: **1137**
- Last log line epoch: **1137** (at 04:16:43Z)
- **PASS.** All three agree.

### Epoch synchronization

**OBSERVED:** Both nodes at epoch 1137 at capture time.
**CLASSIFICATION:** Synchronized. Pass 40's 1-epoch offset was a timing artifact (witness log line at 04:04:43Z, api at 04:04:56Z — same boundary race). In this pass, the three-way bundles on both nodes converge at epoch 1137.

### Epoch cadence

+23/+24 epochs in ~15 minutes. ~37-39s/epoch. Slower than pass 40's ~24s/epoch, but consistent with natural variance of the 30s tick over short windows.

---

## Economic State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged since pass 40) |
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
| last_snapshot_epoch | 1130 | Incrementing by 10 each rotation (pass 40: 1100; +30 epochs = 3 rotations) | **RESOLVED from pass 40.** Snapshot epoch gap between nodes is now closed. Both at 1130. |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (04:17Z single capture):**
- `state.snapshot`: 895 bytes (mtime: 2026-07-28T04:12:56Z EDT — epoch 1130 snapshot, just rotated)
- `wal.log`: 379 bytes (mtime: 2026-07-28T04:12:56Z EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T04:07:56Z EDT — previous epoch 1120 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- state.snapshot grew 1 byte since pass 40 (894→895). Format stable.

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1130 | Incrementing by 10 (pass 40: 1110; +20 epochs = 2 rotations) | **RESOLVED from pass 40.** Both nodes now at 1130. The +10 gap was a timing artifact (witness queried ~1 min after api, catching a rotation boundary). |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (04:17Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T04:13:13Z EDT — epoch 1130 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T04:13:13Z EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T04:08:13Z EDT — previous epoch 1120 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### Snapshot timing — resolved

The snapshot epoch gap flagged in pass 40 (witness=1110, api=1100) is now closed. Both nodes report last_snapshot_epoch=1130, with file mtimes within ~17s of each other (04:12:56 vs 04:13:13). The gap was a timing artifact — the witness happened to be captured just after a snapshot rotation.

---

## Metrics (from heartbeat logs)

### morning-api — latest tick (04:16:56Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s`
**CLASSIFICATION:** Clean. Zero fetches, zero queues, max peer silence well within threshold.

### local-witness — latest tick (04:17:03Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s`
**CLASSIFICATION:** Clean. Consistent pattern — witness tick timing lags api by ~7s.

**All observed ticks (04:02–04:17Z):** Every tick on both nodes shows `aged=0`, `outbound_queues=[]`. No stale fetch entries, no queue buildup, no zombie evictions.

---

## Error Health Scan

### morning-api
**OBSERVED:** 120 total WARN/ERROR lines. After filtering (expected patterns: `skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|No known peers`): **only 2 startup warnings remain** — `Failed to gossip genesis` and `Failed to publish block` at 18:48:26 (both expected for initial no-peer state). 2 more WARN/ERROR lines than pass 40 (120 vs 118), consistent with 23 more epochs cycling (+1-2 Kademlia bootstrap warnings per pass).
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

### local-witness
**OBSERVED:** 122 total WARN/ERROR lines. After filtering: **zero unexpected lines.** All 122 are expected: 3× startup messages + 118× `insufficient balance` rejections (one per epoch, witness rejects redistribution) + 1× `Connection from non-mDNS peer`.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

---

## Delta Summary (Pass 40 → Pass 41)

| Metric | Pass 40 (04:02Z) | Pass 41 (04:17Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch | 1114 | 1137 | +23 (~15 min) | Normal cycling (~38s/epoch) |
| witness epoch | 1113 | 1137 | +24 (~15 min) | Nodes synchronized this pass |
| Three-way match | PASS both | **PASS BOTH** — all three values match on each node | — | Clean |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch (api) | 1100 | 1130 | +30 (3 rotations) | Normal 10-epoch interval |
| Snapshot epoch (witness) | 1110 | 1130 | +20 (2 rotations) | **RESOLVED** — gap closed, both at 1130 |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk (api) | 379 bytes, 04:02Z | 379 bytes, 04:12Z | 0 size; mtime +10min | Snapshot rotation working |
| wal.log on disk (witness) | 379 bytes, 04:03Z | 379 bytes, 04:13Z | 0 size; mtime +10min | Snapshot rotation working |
| state.snapshot (api) | 894 bytes | 895 bytes | +1 byte | Format stable (rotation rewrite) |
| state.snapshot (witness) | 569 bytes | 569 bytes | 0 | Format stable |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 9+ commits behind HEAD |
| Heartbeats (api) | 3320 | 3404 | +84 | Normal — ~5.6/min |
| Heartbeats (witness) | 3322 | 3408 | +86 | Normal |
| max_peer_silence (api) | 3s | 3s | 0 | Well within <30s |
| max_peer_silence (witness) | 6s | 6s | 0 | Consistent |
| Sweep/evict events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~24s/epoch | ~38s/epoch | — | Within normal variance |

### New observations this pass

1. **Snapshot epoch gap RESOLVED.** Pass 40 flagged witness=1110 vs api=1100 as a +10 gap requiring verification. Both nodes now at last_snapshot_epoch=1130 with file mtimes within 17s. Classification: timing artifact confirmed.

2. **No other new observations.** Routine continuation. Nothing changed.

### No new deviations

The three persistent deviations are unchanged. The one flag from pass 40 is resolved.

---

## Persistent Deviations (unchanged)

| # | Observation | First seen | Status |
|---|------------|-----------|--------|
| 1 | build_commit 9+ commits behind HEAD (71aa16b-dirty vs cb5d4b1) | Pass 1 (Jul 27) | Persistent — binary not rebuilt since those commits |
| 2 | GetPersistenceState wal_bytes=0 (reads transactions.wal instead of wal.log) | Pass 1 (Jul 27) | Persistent — Verifier Mission 2 confirmed one-line fix |
| 3 | Local-witness reports morning-api balance as 0 (supply divergence); nonces frozen | Pass 1 (Jul 27) | Persistent — supply conservation CONTRADICTED per Verifier Mission 1 |

---

## UNKNOWN Items

**(None.)** No new UNKNOWNs this pass. The snapshot epoch gap from pass 40 is now resolved to a timing artifact.

---

## Evidence Files

- Previous: `docs/evidence/observer-2026-07-28-pass40.md`
- This: `docs/evidence/observer-2026-07-28-pass41.md`

## Raw Capture Bundle

```json
// GetNodeInfo (morning-api) — 04:17Z
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":34115,"build_commit":"71aa16b-dirty","thickness":990.911907019226}

// GetPeers (morning-api)
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":3404,"silence_secs":2,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api) — three-way capture
// Socket: {"type":"EpochState","epoch":1137,...}
// grep -c "Epoch complete": 1137
// Last log line: epoch=1137 balance_before=20 balance_after=20 ratio=1.02

// GetEconomicState (morning-api)
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api)
{"type":"PersistenceState","last_snapshot_epoch":1130,"wal_bytes":0,"wal_entries":0}

// GetHeight (morning-api)
{"type":"Height","height":1}


// GetNodeInfo (local-witness)
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":34073,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness)
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":3408,"silence_secs":6,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness) — three-way capture
// Socket: {"type":"EpochState","epoch":1137,...}
// grep -c "Epoch complete": 1137
// Last log line: epoch=1137 balance_before=0 balance_after=0 ratio=1.09

// GetEconomicState (local-witness)
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness)
{"type":"PersistenceState","last_snapshot_epoch":1130,"wal_bytes":0,"wal_entries":0}

// GetHeight (local-witness)
{"type":"Height","height":1}
```
