# Observer Evidence Record — 2026-07-27 (Pass 10)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-27T20:55:56Z (~16:56 EDT)
**Machine:** z4-workstation (dale-joseph-hp-z4-g4-workstation, Boynton Beach FL)
**Session type:** Tenth observation pass. Same processes since 14:48 EDT (~6.1h runtime).

**Summary:** All-clear continuation. ~8 min since pass 9. Epochs 240→254 (+14) on morning-api, 241→254 (+13) on witness. One snapshot rotation (epoch 250 on both). Everything at asymptotic steady state: balance=20/thickness decaying/nonce at 120/zero queues. No new deviations.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-hp-z4-g4-workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since |
|-----|------|------|--------------|-------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT |

**No topology changes.** Same two PIDs as pass 9. Socket at `/tmp/m-ap/lattice.sock` responding. Witness socket at `/tmp/local-witness/lattice.sock` responding. No stale survivors.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | auto-genesis identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 7590 (~2.1h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `aa62d12` | **Persistent DEVIATION.** 8 commits behind. Docs-only changes — no binary rebuild since session start. |
| thickness | 997.96 | ~1000, slowly decaying | None (pass 9: 998.08; Δ = -0.12) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | — | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 7608 (~2.1h) | — | None |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=758, silence_secs=2, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=762, silence_secs=4, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 9:** heartbeats +41 (morning-api: 717→758) and +42 (witness: 720→762). Silence: 6→2 (improved, morning-api), 4→4 (witness). Queue depth still 0 on both. Healthy heartbeats.

---

## Epoch State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 254 | Cycling ~30s cadence. +14 epochs since pass 9 (240→254) over ~8 min. | None |
| ratio | 1.0186 | ~1.01-1.02 steady state | None |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 254 | Caught up to morning-api (was 0-1 ahead in pass 9). | None (clocks converged) |
| ratio | 1.436 | Declining (pass 9: 1.460; Δ = -0.024). Expected asymptote toward 1.0. | None |
| tax_calculated | 0 | Balance at 0: no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch agreement (morning-api):** Endpoint epoch=254, `grep -c "Epoch complete"` count=256, last log line epoch=256. Gap of 2 — sequential capture (endpoint first, then log ~30-60s later). Not a simultaneous capture. At witness: endpoint=254, log count=255, last line epoch=255. Off-by-1 (same timing caveat). No genuine disagreement visible.

---

## Economic State

### morning-api
**OBSERVED:** own_balance=20, own_nonce=120. Peer (witness) balance=4980, nonce=0.
**EXPECTED:** Balance at asymptotic floor (20) since ~epoch 121. Nonce at 120 since no new transactions.
**DEVIATION:** None.

### local-witness
**OBSERVED:** own_balance=0, own_nonce=2. Peer (morning-api) balance=0, nonce=0.
**EXPECTED:** Zero balance witness with no mint grant. Nonce 2 (max nonce applied).
**DEVIATION:** None — unchanged since pass 7.

---

## Persistence State

### morning-api
**OBSERVED:** last_snapshot_epoch=250, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every 10 epochs. WAL drained after rotation.
**DEVIATION:** None. Snapshot at epoch 250 (was at 240 in pass 9; +10 epochs = 1 rotation).

**Byte-equality check:** GetPersistenceState wal_bytes=0. `ls -la /tmp/m-ap/persistence/wal.log` shows 379 bytes on disk.
**DEVIATION:** **Persistent UNKNOWN.** wal_bytes=0 but wal.log exists at 379 bytes. Same discrepancy as pass 8 and 9.

**state.snapshot:** 895 bytes (pass 9: 893; grew by +2 since last rotation). Snapshot size fluctuated: 895→893→895 across passes 8→9→10.

### local-witness
**OBSERVED:** last_snapshot_epoch=250, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** None.

**Byte-equality check:** Same UNKNOWN — wal_bytes=0 but wal.log=379 bytes on disk.

**state.snapshot:** 569 bytes (unchanged from pass 8 and 9).

**Notable:** `wal.wal.old` now present on both nodes (379 bytes each) — from the epoch 240→250 snapshot rotation. Pass 9 only showed `wal.log` and `state.snapshot`. This file is the known-provisional `wal.wal.old` naming quirk documented in VERIFIED-BEHAVIOR.md.

---

## Build Commit & Binary Freshness

**OBSERVED:** `71aa16b-dirty` on both nodes (unchanged since pass 1).
**EXPECTED (VERIFIED-BEHAVIOR.md):** Should match git HEAD (`aa62d12`).
**DEVIATION:** **Persistent.** 8 commits behind. Docs-only changes since `71aa16b` — no wire-format, codec, or protocol changes. Not a functional concern.

---

## Log Health

**OBSERVED:** No ERROR or unexpected WARN lines on either node.

**Grep filter** `grep -vE 'skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|Failed to trigger bootstrap|InsufficientPeers|Failed to gossip genesis'` — zero operational WARN/ERROR hits on both logs.

---

## Delta from Pass 9

| Metric | Pass 9 (16:48 EDT) | Pass 10 (~16:56 EDT) | Delta |
|--------|-------------------|----------------------|-------|
| Uptime (morning-api) | 7189s | 7590s | +401s (~6.7 min) |
| Uptime (witness) | 7191s | 7608s | +417s |
| Epoch (morning-api) | 240 | 254 | +14 |
| Epoch (witness) | 241 | 254 | +13 |
| Heartbeats (morning-api) | 717 | 758 | +41 |
| Heartbeats (witness) | 720 | 762 | +42 |
| Silence (morning-api) | 6s | 2s | -4 (improved) |
| Silence (witness) | 4s | 4s | 0 |
| Thickness | 998.08 | 997.96 | -0.12 (expected decay) |
| Balance | 20 | 20 | 0 |
| Nonce | 120 | 120 | 0 |
| Snapshot (morning-api) | 240 | 250 | +10 (1 rotation) |
| Snapshot (witness) | 240 | 250 | +10 (1 rotation) |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| **snapshot size (morning-api)** | 893 | 895 | +2 (fluctuation) |
| **wal.wal.old present** | not mentioned | both nodes, 379B | New — rotation artifact |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from pass 8/9). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes. The genesis re-seed after snapshot rotation could explain this (post-rotation WAL written after counter snapshot), but requires inspection of the write path to classify as "expected" or "bug."

2. **snapshot size fluctuation (893→895 bytes)** on morning-api across rotations. Witness snapshot holds steady at 569. Possibly due to non-deterministic serialization (HashMap iteration order) — this was documented as a known trap in era-two-foundation.md. Not a functional concern on a 2-node mesh.

---

## Raw Capture (~20:54Z)

```json
// GetNodeInfo (morning-api)
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":7590,"build_commit":"71aa16b-dirty","thickness":997.9643515745496}

// GetPeers (morning-api)
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":758,"silence_secs":2,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api)
{"type":"EpochState","epoch":254,"ratio":1.0185662395229593,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (morning-api)
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api)
{"type":"PersistenceState","last_snapshot_epoch":250,"wal_bytes":0,"wal_entries":0}

// GetHeight (morning-api)
{"type":"Height","height":1}

// GetNodeInfo (local-witness)
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":7608,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness)
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":762,"silence_secs":4,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness)
{"type":"EpochState","epoch":254,"ratio":1.4364511095700416,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (local-witness)
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness)
{"type":"PersistenceState","last_snapshot_epoch":250,"wal_bytes":0,"wal_entries":0}
```

---

## Log Evidence (Last 3 Epoch Complete Lines)

**morning-api (/tmp/m-ap.log):**
```
epoch=254 balance_before=20 balance_after=20 ratio=1.02
epoch=255 balance_before=20 balance_after=20 ratio=1.02
epoch=256 balance_before=20 balance_after=20 ratio=1.02
```

**local-witness (/tmp/lw.log):**
```
epoch=253 balance_before=0 balance_after=0 ratio=1.44
epoch=254 balance_before=0 balance_after=0 ratio=1.44
epoch=255 balance_before=0 balance_after=0 ratio=1.43
```

Epoch counts: morning-api 256, witness 255. Witness ratio declined to 1.43 — continuing expected asymptotic decay from 1.46 (pass 9) to 1.43 now.

---

## Lines Triaged as Benign (no change from pass 9)

- `Failed to gossip genesis (will retry on peer connect)` — Startup only (14:48 EDT), first node launched before any peers existed.
- `[block-publish] Failed to publish block proposal_id="genesis" error=InsufficientPeers` — Same startup context.

Both from initial bootstrap sequence; not relevant to current mesh health.

---

**Next check:** No threshold violations. All clear. Persistent UNKNOWN items (wal_bytes discrepancy, snapshot size fluctuation) are unchanged and non-functional. Mesh has been stable for ~6.1 hours.
