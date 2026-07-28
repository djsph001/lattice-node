# Observer Evidence Record — 2026-07-28 (Pass 40)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T04:02:03Z (04:02Z; three-way bundles at 04:04Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Fortieth observation pass. Same processes since 2026-07-27T18:48Z (~9.2h runtime). ~10 min since pass 39 (03:52Z).

**Summary:** Routine continuation. Both nodes cycling normally. Three-way PASS on both (with 1-epoch timing offset, consistent with boundary race). Snapshot rotated, balances frozen, metrics clean. One item flagged for attention: last_snapshot_epoch shows a +10 gap between nodes (morning-api=1100, witness=1110). Possibly a timing artifact; flagged for next-pass comparison.

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
| uptime_secs | 33218 (~9.2h) | — | None (pass 39: 32623; Δ = +595s ≈ 9.9 min — consistent with ~10min real time minus capture window) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 991.150 | ~1000, slowly decaying | None (pass 39: 991.310; Δ = -0.16 over ~10 min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 33213 (~9.2h) | — | None (matches api within 5s) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=3320, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=3322, silence_secs=2, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 39:** Heartbeats api +60 (3260→3320). Silence: api 9s→3s (well within <30s threshold). Queue depth 0 on both. Normal variance.

---

## Epoch State

### morning-api (04:04Z three-way bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1114 | Cycling. +25 since pass 39 (1089→1114). | **PASS — three-way match.** Socket=1114, grep -c=1114, last log line=1114. All agree. |
| ratio | 1.01968 | ~1.01–1.02 steady state (pass 39: 1.01999) | None (stable; small variance from tick timing) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (04:04Z simultaneous capture):**
- Socket epoch: **1114**
- `grep -c` count: **1114**
- Last log line epoch: **1114** (at 04:04:56Z)
- **PASS.** All three agree.

### local-witness (04:04Z three-way bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1113 | Cycling. +24 since pass 39 (1089→1113). | **PASS — three-way match.** Socket, grep count, and last log line all at 1113. |
| ratio | 1.09421 | Continuing asymptotic decline (pass 39: 1.0964; Δ = -0.00219) | None (monotonic decay expected) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (04:04Z simultaneous capture):**
- Socket epoch: **1113**
- `grep -c` count: **1113**
- Last log line epoch: **1113** (at 04:04:43Z)
- **PASS.** All three agree.

### Epoch synchronization

**OBSERVED:** morning-api at 1114, witness at 1113. 1-epoch offset.
**CLASSIFICATION:** Timing artifact — the captures are not simultaneous. Witness log line at 04:04:43Z (epoch 1113), api log line at 04:04:56Z (epoch 1114). Witness transitioned to 1114 ~13s after its capture. Same pattern as pass 38's boundary race.

### Epoch cadence

+25/+24 epochs in ~10 minutes. ~24–25s/epoch. Slightly faster than pass 39's ~28s/epoch, consistent with natural variance in the 30s tick.

---

## Economic State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged since pass 39) |
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
| last_snapshot_epoch | 1100 | Incrementing by 10 each rotation (pass 39: 1080; +20 epochs = 2 rotations) | None — snapshot at 10-epoch interval. |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (04:02Z single capture):**
- `state.snapshot`: 894 bytes (mtime: 2026-07-28T04:02Z EDT — epoch 1100 snapshot, just rotated)
- `wal.log`: 379 bytes (mtime: 2026-07-28T04:02Z EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-27T23:57Z EDT — previous epoch 1090 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1110 | Incrementing by 10 (pass 39: 1080; +30 epochs = 3 rotations) | **FLAG.** Witness last_snapshot_epoch=1110; morning-api last_snapshot_epoch=1100. Gap of +10 epochs. Possibly a timing artifact (witness captured ~1 min later). See Delta Summary for discussion. |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (04:03Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T04:03Z EDT — epoch 1110 snapshot, just rotated)
- `wal.log`: 379 bytes (mtime: 2026-07-28T04:03Z EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-27T23:58Z EDT — previous epoch 1100 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### Snapshot timing divergence — analysis

**OBSERVED:** morning-api last_snapshot_epoch=1100, file mtime 04:02Z. Witness last_snapshot_epoch=1110, file mtime 04:03Z. Gap of +10 epochs (one full snapshot rotation) on witness.
**EXPECTED:** Both nodes on same 10-epoch snapshot schedule.
**DEVIATION:** Witness is one rotation ahead (+10 epochs). Two possible explanations:

1. **Timing artifact.** The witness persistence query (04:03Z) was captured ~1 min after morning-api (04:02Z). In that minute, both nodes crossed epoch 1110. Morning-api's snapshot at 1110 may not have completed yet or may have been overwritten by the in-place write at 1100 before my query. The file mtime (04:02Z both for state.snapshot and wal.log) suggests the 1100 rotation just completed — the 1110 rotation may be pending.

2. **Genuine divergence.** The snapshot trigger logic fires at a different epoch or interval on each node.

**Verification needed:** Next pass — compare last_snapshot_epoch on both nodes. If gap persists beyond one capture cycle, it's a genuine divergence.

---

## Metrics (from heartbeat logs)

### morning-api — latest tick (04:04:56Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s`
**CLASSIFICATION:** Clean. Zero fetches, zero queues, max peer silence well within threshold.

### local-witness — latest tick (04:05:03Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s`
**CLASSIFICATION:** Clean. Slightly higher max_peer_silence (6s vs api's 3s) but well under 10s threshold. Consistent pattern — witness tick timing lags api by ~7s.

**All observed ticks (03:52–04:05Z):** Every tick on both nodes shows `aged=0`, `outbound_queues=[]`. No stale fetch entries, no queue buildup, no zombie evictions.

---

## Error Health Scan

### morning-api
**OBSERVED:** 118 total WARN/ERROR lines. After filtering (expected patterns: `skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|No known peers`): **only 2 startup warnings remain** — `Failed to gossip genesis` and `Failed to publish block` at 18:48:26 (both expected for initial no-peer state). 3 more WARN/ERROR lines than pass 39 (118 vs 115), consistent with 25 more epochs cycling.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

### local-witness
**OBSERVED:** 122 total WARN/ERROR lines. After filtering: **zero unexpected lines.** All 122 are expected: 3× startup messages + 118× `insufficient balance` rejections (one per epoch, witness rejects redistribution) + 1× `Connection from non-mDNS peer`.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

---

## Delta Summary (Pass 39 → Pass 40)

| Metric | Pass 39 (03:52Z) | Pass 40 (04:02Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch | 1089 | 1114 | +25 (~10 min) | Normal cycling (~24s/epoch) |
| witness epoch | 1089 | 1113 | +24 (~10 min) | 1-epoch offset = timing artifact |
| Three-way match | PASS both | **PASS BOTH** — all three values match on each node | — | Clean; boundary race between nodes |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch (api) | 1080 | 1100 | +20 (2 rotations) | Normal 10-epoch interval |
| Snapshot epoch (witness) | 1080 | 1110 | +30 (3 rotations) | **FLAG: +10 gap vs morning-api** |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk (api) | 379 bytes, Jul 27 23:52 | 379 bytes, Jul 28 04:02 | 0 size; mtime +4h 10min | Snapshot rotation working correctly |
| wal.log on disk (witness) | 379 bytes, Jul 27 23:48 | 379 bytes, Jul 28 04:03 | 0 size; mtime +4h 15min | Snapshot rotation working correctly |
| state.snapshot (api) | 894 bytes | 894 bytes | 0 | Format stable |
| state.snapshot (witness) | 569 bytes | 569 bytes | 0 | Format stable |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 9 commits behind HEAD |
| Heartbeats (api) | 3260 | 3320 | +60 | Normal — ~6.0/min |
| Heartbeats (witness) | 3258 | 3322 | +64 | Normal |
| max_peer_silence (api) | 3s | 3s | 0 | Well within <30s |
| max_peer_silence (witness) | 6s | 6s | 0 | Consistent with witness tick lag |
| Kademlia bootstrap WARNs | Present (harmless) | Present in total count | — | Unchanged — always filtered |
| Zombie/reconnect events | None | None | — | Clean |
| Sweep/evict events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~28s/epoch | ~24s/epoch | — | Within normal variance |

### New observations this pass

1. **Snapshot epoch gap.** Witness last_snapshot_epoch=1110 vs morning-api=1100. +10 gap. Witness file mtimes confirm a snapshot at epoch 1110; morning-api file mtimes confirm a snapshot at epoch 1100. See analysis above. Flagged for next-pass comparison.

2. **No other new observations.** Routine continuation.

### No new deviations

The three persistent deviations are unchanged. One flag on snapshot timing needs next-pass verification.

---

## Persistent Deviations (unchanged)

| # | Observation | First seen | Status |
|---|------------|-----------|--------|
| 1 | build_commit 9 commits behind HEAD (71aa16b-dirty vs cb5d4b1) | Pass 1 (Jul 27) | Persistent — binary not rebuilt since those commits |
| 2 | GetPersistenceState wal_bytes=0 (reads transactions.wal instead of wal.log) | Pass 1 (Jul 27) | Persistent — Verifier Mission 2 confirmed one-line fix |
| 3 | Local-witness reports morning-api balance as 0 (supply divergence); nonces frozen | Pass 1 (Jul 27) | Persistent — supply conservation CONTRADICTED per Verifier Mission 1 |

---

## UNKNOWN Items

**(None.)** No new UNKNOWNS this pass. The snapshot epoch gap is flagged but classified as tentative — next pass will resolve whether it's a timing artifact or genuine divergence.

---

## Evidence Files

- Previous: `docs/evidence/observer-2026-07-28-pass39.md`
- This: `docs/evidence/observer-2026-07-28-pass40.md`

## Raw Capture Bundle

```json
// GetNodeInfo (morning-api)
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":33218,"build_commit":"71aa16b-dirty","thickness":991.1504244006355}

// GetPeers (morning-api)
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":3320,"silence_secs":3,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api) — three-way capture
// Socket: {"type":"EpochState","epoch":1114,...}
// grep -c "Epoch complete": 1114
// Last log line: epoch=1114 balance_before=20 balance_after=20 ratio=1.02

// GetEconomicState (morning-api)
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api)
{"type":"PersistenceState","last_snapshot_epoch":1100,"wal_bytes":0,"wal_entries":0}

// GetHeight (morning-api)
{"type":"Height","height":1}

// GetNodeInfo (local-witness)
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":33213,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness)
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":3322,"silence_secs":2,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness) — three-way capture
// Socket: {"type":"EpochState","epoch":1113,...}
// grep -c "Epoch complete": 1113
// Last log line: epoch=1113 balance_before=0 balance_after=0 ratio=1.09

// GetEconomicState (local-witness)
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness)
{"type":"PersistenceState","last_snapshot_epoch":1110,"wal_bytes":0,"wal_entries":0}

// GetHeight (local-witness)
{"type":"Height","height":1}
```
