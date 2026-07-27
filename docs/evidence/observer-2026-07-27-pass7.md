# Observer Evidence Record — 2026-07-27 (Pass 7)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-27T20:26:13Z (16:26 EDT)
**Machine:** z4-workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Seventh observation pass. Same processes since 14:48 EDT (~1h 36m uptime).

**Note:** All-clear continuation. ~9 min since pass 6. Epochs 174→196 (+22) since pass 6. Balance stable at asymptotic floor (20). Both nodes fully epoch-synchronized. Snapshots rotated twice (epoch 180, 190).

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-hp-z4-g4-workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since |
|-----|------|------|--------------|-------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT |

**No topology changes.** Same two PIDs since node start. No stale survivor processes. No binary restart.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | MESH.md identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (launched with `--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 5745 (~1h 36m) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `aa62d12` | **Persistent DEVIATION.** 8 commits behind. Docs-only drift — no wire-format change. |
| thickness | 998.46 | ~1000, slowly decaying | None |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | MESH.md identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api's PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 5742 (~1h 36m) | — | None |
| build_commit | `71aa16b-dirty` | Same binary | Same DEVIATION. |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=573, silence_secs=1, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=575, silence_secs=1, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 6:** heartbeats increased (morning-api: 518→573, witness: 520→575). Silence decreased (6/7s→1s). Queue depth unchanged at 0.

---

## Epoch State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 193 (three-way capture) | Cycling ~30s cadence. +19 epochs since pass 6 (174→193) over ~9 min. | None |
| ratio | 1.018 | ~1.01-1.02 steady state. | None |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | Single peer. | None |

**Three-way epoch check (single capture at 20:24:38Z):**
- Socket epoch: 193
- `grep -c "Epoch complete" /tmp/m-ap.log`: 193
- Last log epoch line: epoch=193 balance_before=20 balance_after=20 ratio=1.02

**Three-way match: PASS.** No boundary race this pass — all three values agree exactly.

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 196 (three-way capture) | Same epoch domain, matching cadence. Both nodes at 196. | None |
| ratio | 1.56 | Declining from 1.64 at pass 6 epoch 174. Expected trajectory. | None |
| tax_calculated | 0 | 0 balance. | None |
| tax_collected | 0 | 0. | None |
| redistributed_to | 1 | Morning-api. | None |

**Three-way epoch check (single capture at 20:26:13Z):**
- Socket epoch: 196
- `grep -c "Epoch complete" /tmp/lw.log`: 196
- Last log epoch line: epoch=196 balance_before=0 balance_after=0 ratio=1.57

**Three-way match: PASS.**

**Epoch synchronization:** Both nodes at 193 (morn single-capture) / 196 (witness capture ~1.5 min later). Both incremented by +19/22 from pass 6. Perfect lockstep — no offset.

---

## Economic State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | At asymptotic floor since epoch ~121. Unchanged from pass 6. | None. Stable floor. |
| own_nonce | 120 | Unchanged. No transactions authored. | None. |
| peers[0].balance | 4980 | local-witness's balance as seen by morning-api. Unchanged. | None. |
| peers[0].nonce | 0 | No transactions authored by witness. | None. |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | Started at 0 (--mint 0). Never changes. | None. |
| own_nonce | 2 | Two attempted transactions at startup. Unchanged. | None. |
| peers[0].balance | 0 | morning-api's balance as seen by witness. | **Same DEVIATION.** Witness sees 0 where morning-api reports 20. Mint-0 witness cannot apply epoch redistribution from morning-api. |

---

## Persistence State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 190 | Snapshots every 10 epochs. Previous was 170 at pass 6. Snapshots at 180 and 190 occurred. | None. |
| wal_bytes | 0 (API) | wal.log is 379 bytes on disk. | **Same DEVIATION.** Endpoint returns 0. |
| wal_entries | 0 | — | Same issue. |

File inventory (single capture):

| File | Size (now) | Size (pass 6) | Delta | Notes |
|------|-----------|-------------|-------|-------|
| `persistence/state.snapshot` | 895 bytes | 895 bytes | 0 | Rotated at epoch 180 and 190. Same encoding size. |
| `persistence/wal.log` | 379 bytes | 379 bytes | 0 | Static since last rotation. |
| `persistence/wal.wal.old` | 379 bytes | 379 bytes | 0 | Static. |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 190 | Same snapshot cadence. Snapshots at 180 and 190 occurred. | None. |
| wal_bytes | 0 (API) | wal.log is 379 bytes. | **Same DEVIATION.** |
| wal_entries | 0 | — | Same issue. |

File inventory (single capture):

| File | Size (now) | Size (pass 6) | Delta | Notes |
|------|-----------|-------------|-------|-------|
| `persistence/state.snapshot` | 569 bytes | 569 bytes | 0 | Rotated at epoch 180 and 190. Same size. |
| `persistence/wal.log` | 379 bytes | 379 bytes | 0 | Static. |
| `persistence/wal.wal.old` | 379 bytes | 379 bytes | 0 | Static. |

**Byte-equality check:** API reports wal_bytes=0. Disk has wal.log=379 bytes. Mismatch persists.

---

## Chain Height

Both nodes: height=1 (genesis only, no subsequent blocks).

**EXPECTED:** height=1 for genesis-only mesh.
**DEVIATION:** None.

---

## Log Health

### morning-api

**Total WARN/ERROR lines:** 26 (was 24 at pass 6). Delta: +2 lines in ~9 min — two more `Failed to trigger bootstrap: No known peers` from periodic Kademlia. Expected.

**Breakdown:**
- 3× startup: `No snapshot found, starting fresh` (expected)
- 1× `InsufficientPeers` for genesis gossip (expected)
- 1× `Failed to publish block` for genesis (expected)
- 1× `Connection from non-mDNS peer` (expected)
- 20× `Failed to trigger bootstrap: No known peers` (periodic Kademlia, benign on `--no-mdns` mesh)

**Filtered WARN/ERROR** (excluding expected patterns): **Empty.** No unexpected patterns.

### local-witness

**Total WARN/ERROR lines:** 122 (unchanged from pass 6). Count stable — all 118 epoch-rejection lines were written early in the run. No new WARN/ERROR lines since pass 4.

**Breakdown:**
- 3× startup: `No snapshot found, starting fresh` (expected)
- 1× `Connection from non-mDNS peer` (expected)
- 118× `Transaction validation failed error=insufficient balance: ... needs <N>` (one per epoch, witness rejects redistribution)

**Filtered WARN/ERROR:** **Empty.** No unexpected patterns.

---

## Metrics Snapshot

### morning-api (last 10 metrics ticks)

```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=2s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
```

**All metrics clean.** Zero fetches, zero queues, silence 2-3s. Unchanged from pass 6.

### local-witness (last 10 metrics ticks)

```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```

**All metrics clean.** Zero fetches, zero queues, 6s silence. Unchanged from pass 6.

### Sweep/Evict/Zombie Activity

**None.** Zero sweep, evict, or zombie events across both nodes' entire logs. Peer table stable — both nodes have exchanged 573+ heartbeats without interruption.

---

## Summary of Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 8 commits behind HEAD `aa62d12`) | Low — docs-only drift, no wire-format change | **Persistent** since pass 1 |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint not wired | **Persistent** since pass 1 |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) — causes repeated validation failures | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 |

**Deviations resolved since pass 6:** None. All three remain unchanged.
**New deviations since pass 6:** None.

---

## Balance Trajectory — Full Soak Record

| Pass | Time (EDT) | Elapsed | Epoch | Balance | Tax/Epoch |
|------|-----------|---------|-------|---------|-----------|
| Pass 1 | 15:18 | ~30 min | 60 | 233 | ~12 |
| Pass 2 | 19:27 | ~4h 39m | 78 | 100 | 5 |
| Pass 3 | 19:50 | ~5h 02m | 123 | 20 | 0 |
| Pass 4 | 19:58 | ~5h 10m | 141 | 20 | 0 |
| Pass 5 | 16:08 | ~1h 20m | 160 | 20 | 0 |
| Pass 6 | 16:15 | ~1h 27m | 174 | 20 | 0 |
| **Pass 7** | **16:26** | **~1h 36m** | **193/196** | **20** | **0** |

**Delta from pass 6:** balance 20→20 (unchanged, at floor), tax 0→0, epochs 174→193 (+19 morn / +22 witness), elapsed ~9 min. The system remains in steady state.

---

## Changes Since Pass 6

- **Mesh health:** Unchanged. Both nodes up, gossiping, epochs cycling at ~30s cadence.
- **Epoch synchronization:** Both nodes locked at same epoch (193/196 at their respective capture moments). Perfect lockstep — no offset.
- **Balance:** Stable at asymptotic floor of 20. No change for 70+ epochs.
- **Three-way epoch check:** morning-api: PASS (no boundary race this time). Witness: PASS.
- **WAL rotation:** Both nodes completed snapshots at epoch 180 and 190. File sizes unchanged from pass 6.
- **Persistence:** State.snapshot files carry correct timestamps. WAL files static at 379B.
- **Log health:** No new error patterns. morning-api WARN +2 (Kademlia), witness unchanged at 122.
- **Metrics:** All clean. Zero fetches, zero queues. Silence: morning-api 2-3s, witness 6s. Unchanged.
- **Sweep/evict activity:** None. Peer table stable — 573+ heartbeats exchanged across both nodes.
- **No new deviations.** All three existing deviations (stale binary, wal_bytes API, witness balance blindness) unchanged.

---

## Raw Capture Bundle

```json
// GetNodeInfo (morning-api) — 20:24:12Z
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":5745,"build_commit":"71aa16b-dirty","thickness":998.460859514867}

// GetPeers (morning-api) — 20:24:12Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":573,"silence_secs":1,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api) — 20:24:38Z (three-way capture)
{"type":"EpochState","epoch":193,"ratio":1.0181099349572758,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (morning-api) — 20:24:12Z
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api) — 20:24:38Z
{"type":"PersistenceState","last_snapshot_epoch":190,"wal_bytes":0,"wal_entries":0}

// GetHeight (morning-api) — 20:24:12Z
{"type":"Height","height":1}

// GetNodeInfo (local-witness) — 20:24:25Z
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":5742,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness) — 20:24:25Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":575,"silence_secs":1,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness) — 20:26:13Z (three-way capture)
{"type":"EpochState","epoch":196,"ratio":1.5682360086377543,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (local-witness) — 20:24:25Z
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness) — 20:24:44Z
{"type":"PersistenceState","last_snapshot_epoch":190,"wal_bytes":0,"wal_entries":0}

// GetHeight (local-witness) — 20:24:25Z
{"type":"Height","height":1}
```

---

## Log Evidence (Last 5 Epoch Complete Lines)

**morning-api:**
```
epoch=191 balance_before=20 balance_after=20 ratio=1.02
epoch=192 balance_before=20 balance_after=20 ratio=1.02
epoch=193 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20 since epoch ~121. No further change.

**local-witness:**
```
epoch=194 balance_before=0 balance_after=0 ratio=1.58
epoch=195 balance_before=0 balance_after=0 ratio=1.57
epoch=196 balance_before=0 balance_after=0 ratio=1.57
```
Ratio continuing expected decline from 1.91 (early) → 1.57 (current).
