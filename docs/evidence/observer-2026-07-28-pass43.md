# Observer Evidence Record — 2026-07-28 (Pass 43)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T04:33:33Z (04:33Z; three-way bundles at 04:33Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Forty-third observation pass. Same processes since 2026-07-27T18:48Z (~9.7h runtime). ~7.7 min since pass 42 (04:25Z).

**Summary:** Routine continuation. Both nodes at epoch 1171. Three-way PASS on both. Snapshot epoch 1170 (recently rotated from 1150, normal). No new deviations. All three persistent deviations unchanged.

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
| uptime_secs | 35106 (~9.75h) | — | None (pass 42: 34666; Δ = +440s ≈ 7.3 min — consistent with ~7.7 min real time) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9+ commits behind. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 990.650 | ~1000, slowly decaying | None (pass 42: 990.801; Δ = -0.151 over ~7.7 min — normal decay) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 35103 (~9.75h) | — | None (pass 42: 34652; Δ = +451s — within ~11s of api uptime delta) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=3509, silence_secs=4, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=3511, silence_secs=4, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 42:** Heartbeats api +43 (3466→3509), witness +44 (3467→3511). Silence: api 6s→4s, witness 6s→4s (both well within <30s threshold). Queue depth 0 on both. Normal variance.

---

## Epoch State

### morning-api (04:33Z three-way bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1171 | Cycling. +16 since pass 42 (1155→1171). | **PASS — three-way match.** Socket=1171, grep -c=1171, last log line=1171. All agree. |
| ratio | 1.01968 | ~1.01–1.02 steady state (pass 42: 1.01968) | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (04:33Z simultaneous capture):**
- Socket epoch: **1171**
- `grep -c` count: **1171**
- Last log line epoch: **1171** (at 04:33:26Z)
- **PASS.** All three agree.

### local-witness (04:33Z three-way bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1171 | Cycling. +16 since pass 42 (1155→1171). | **PASS — three-way match.** Socket, grep count, and last log line all at 1171. |
| ratio | 1.08922 | Continuing asymptotic decline (pass 42: 1.09086; Δ = -0.00164) | None (monotonic decay expected) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (04:33Z simultaneous capture):**
- Socket epoch: **1171**
- `grep -c` count: **1171**
- Last log line epoch: **1171** (at 04:33:43Z)
- **PASS.** All three agree.

### Epoch synchronization

**OBSERVED:** Both nodes at epoch 1171 at capture time.
**CLASSIFICATION:** Synchronized. No offset.

### Epoch cadence

+16 epochs in ~7.7 minutes. ~29s/epoch. Close to the 30s tick with natural variation.

---

## Economic State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged since pass 42) |
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
| last_snapshot_epoch | 1170 | Incrementing by 10 each rotation (pass 42: 1150; +20 epochs = 2 rotations) | None (normal — recently rotated to 1170) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (04:33Z single capture):**
- `state.snapshot`: 895 bytes (mtime: 2026-07-28T00:32 EDT — epoch 1170 snapshot, just rotated)
- `wal.log`: 379 bytes (mtime: 2026-07-28T00:32 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T00:27 EDT — previous epoch 1160 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- state.snapshot size: 895 bytes (pass 42: 894; Δ = +1 byte — normal content variance across rewrites)

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1170 | Incrementing by 10 (pass 42: 1150; +20 epochs = 2 rotations) | None (both nodes at 1170; synchronized) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (04:33Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T00:33 EDT — epoch 1170 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T00:33 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T00:28 EDT — previous epoch 1160 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### Snapshot synchronization

Both nodes at last_snapshot_epoch=1170. File mtimes within ~1 min (00:32 vs 00:33 EDT). Snapshot rotation working normally. wal.wal.old at 00:27/00:28 confirms epoch 1160 → 1170 rotation occurred as expected.

---

## Metrics (from heartbeat logs)

### morning-api — latest tick (04:34:06Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s`
**CLASSIFICATION:** Clean. Zero fetches, zero queues, max peer silence well within threshold.

### local-witness — latest tick (04:34:13Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s`
**CLASSIFICATION:** Clean. Consistent pattern — witness tick timing lags api by ~7s.

**All observed ticks (04:25–04:34Z):** Every tick on both nodes shows `aged=0`, `outbound_queues=[]`. No stale fetch entries, no queue buildup, no zombie evictions.

---

## Error Health Scan

### morning-api
**OBSERVED:** 124 total WARN/ERROR lines (+2 since pass 42: 122→124). After filtering (expected patterns: `skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|No known peers|Failed to trigger`): **only 2 startup warnings remain** — `Failed to gossip genesis` and `Failed to publish block` at 18:48:26 (both expected for initial no-peer state). +2 total lines consistent with ~2 more Kademlia bootstrap warnings in ~8 min of runtime.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

### local-witness
**OBSERVED:** 122 total WARN/ERROR lines (unchanged from pass 42). After filtering: **zero unexpected lines.** All 122 are expected: 3× startup messages + 118× `insufficient balance` rejections + 1× `Connection from non-mDNS peer`.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

---

## Delta Summary (Pass 42 → Pass 43)

| Metric | Pass 42 (04:25Z) | Pass 43 (04:33Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch | 1155 | 1171 | +16 (~7.7 min) | Normal cycling (~29s/epoch) |
| witness epoch | 1155 | 1171 | +16 (~7.7 min) | Nodes synchronized |
| Three-way match | PASS both | **PASS BOTH** — all three match on each node | — | Clean |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch (api) | 1150 | 1170 | +20 (2 rotations) | Normal 10-epoch interval |
| Snapshot epoch (witness) | 1150 | 1170 | +20 (2 rotations) | Synchronized |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk (api) | 379 bytes, 04:22Z | 379 bytes, 04:32Z | 0 size; mtime +10min | Snapshot rotation working |
| wal.log on disk (witness) | 379 bytes, 04:23Z | 379 bytes, 04:33Z | 0 size; mtime +10min | Snapshot rotation working |
| state.snapshot (api) | 894 bytes | 895 bytes | +1 byte | Normal content variance across rewrites |
| state.snapshot (witness) | 569 bytes | 569 bytes | 0 | Format stable |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 9+ commits behind HEAD |
| Heartbeats (api) | 3466 | 3509 | +43 | Normal — ~5.6/min |
| Heartbeats (witness) | 3467 | 3511 | +44 | Normal |
| max_peer_silence (api) | 3s | 3s | 0 | Well within <30s |
| max_peer_silence (witness) | 6s | 6s | 0 | Consistent |
| Sweep/evict events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~27s/epoch | ~29s/epoch | — | Within normal variance |

### New observations this pass

1. **No new observations.** Routine continuation. Nothing changed since pass 42.

### No new deviations

The three persistent deviations are unchanged.

---

## Persistent Deviations (unchanged)

| # | Observation | First seen | Status |
|---|------------|-----------|--------|
| 1 | build_commit 9+ commits behind HEAD (71aa16b-dirty vs cb5d4b1) | Pass 1 (Jul 27) | Persistent — binary not rebuilt since those commits |
| 2 | GetPersistenceState wal_bytes=0 (reads transactions.wal instead of wal.log) | Pass 1 (Jul 27) | Persistent — Verifier Mission 2 confirmed one-line fix |
| 3 | Local-witness reports morning-api balance as 0 (supply divergence); nonces frozen | Pass 1 (Jul 27) | Persistent — supply conservation CONTRADICTED per Verifier Mission 1 |

---

## UNKNOWN Items

**(None.)** No new UNKNOWNs this pass.

---

## Evidence Files

- Previous: `docs/evidence/observer-2026-07-28-pass42.md`
- This: `docs/evidence/observer-2026-07-28-pass43.md`

## Raw Capture Bundle

```json
// GetNodeInfo (morning-api) — 04:33Z
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":35106,"build_commit":"71aa16b-dirty","thickness":990.6496041944039}

// GetPeers (morning-api)
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":3509,"silence_secs":4,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api) — three-way capture
// Socket: {"type":"EpochState","epoch":1171,...}
// grep -c "Epoch complete": 1171
// Last log line: epoch=1171 balance_before=20 balance_after=20 ratio=1.02

// GetEconomicState (morning-api)
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api)
{"type":"PersistenceState","last_snapshot_epoch":1170,"wal_bytes":0,"wal_entries":0}

// GetHeight (morning-api)
{"type":"Height","height":1}


// GetNodeInfo (local-witness)
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":35103,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness)
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":3511,"silence_secs":4,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness) — three-way capture
// Socket: {"type":"EpochState","epoch":1171,...}
// grep -c "Epoch complete": 1171
// Last log line: epoch=1171 balance_before=0 balance_after=0 ratio=1.09

// GetEconomicState (local-witness)
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness)
{"type":"PersistenceState","last_snapshot_epoch":1170,"wal_bytes":0,"wal_entries":0}

// GetHeight (local-witness)
{"type":"Height","height":1}
```
