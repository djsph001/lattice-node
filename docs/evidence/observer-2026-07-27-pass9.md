# Observer Evidence Record — 2026-07-27 (Pass 9)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-27T20:48:16Z (16:48 EDT)
**Machine:** z4-workstation (dale-joseph-hp-z4-g4-workstation, Boynton Beach FL)
**Session type:** Ninth observation pass. Same processes since 14:48 EDT (~6h runtime).

**Summary:** All-clear continuation. ~14 min since pass 8. Epochs 213→240 (+27) since pass 8. Snapshot rotation at epoch 240. All metrics stable. No new deviations.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-hp-z4-g4-workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since |
|-----|------|------|--------------|-------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT |

**No topology changes.** Same two PIDs. Socket at `/tmp/m-ap/lattice.sock` responding. Witness socket at `/tmp/local-witness/lattice.sock` responding. No stale survivors.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | auto-genesis identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 7189 (~2h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `aa62d12` | **Persistent DEVIATION.** 8 commits behind. Docs-only drift — no binary rebuild since session start. |
| thickness | 998.08 | ~1000, slowly decaying | None (pass 8: 998.30; Δ = -0.22) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | — | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 7191 (~2h) | — | None |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=717, silence_secs=6, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=720, silence_secs=4, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 8:** heartbeats +83 (morning-api: 634→717) and +83 (witness: 637→720). Silence: 2→6 (morning-api), 4→4 (witness). Queue depth still 0 on both. Healthy heartbeat flow.

---

## Epoch State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 240 | Cycling ~30s cadence. +27 epochs since pass 8 (213→240) over ~14 min. | None |
| ratio | 1.018 | ~1.01-1.02 steady state | None |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 241 | Cycling ~30s cadence. Caught up to/1 ahead of morning-api. | None (witness runs 0-1 epoch ahead after clock convergence) |
| ratio | 1.460 | Declining (pass 8: 1.522). Expected asymptote toward 1.0 as thickness decays. | None |
| tax_calculated | 0 | Balance at 0: no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch agreement (morning-api):** Endpoint epoch=240, `grep -c "Epoch complete"` count=241, last log line epoch=241. Off-by-1 race (new epoch completed between endpoint query and log grep; sequential not parallel). At witness capture: Endpoint epoch=241, log count=241, last line epoch=241. ✓ no gap.

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
**OBSERVED:** last_snapshot_epoch=240, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every 10 epochs. WAL drained after rotation.
**DEVIATION:** None. Snapshot at epoch 240 (was at 210 in pass 8; +30 epochs = 3 rotations 210→220→230→240).

**Byte-equality check:** GetPersistenceState wal_bytes=0. `ls -la /tmp/m-ap/persistence/wal.log` shows 379 bytes on disk.
**DEVIATION:** wal_bytes=0 but wal.log exists with 379 bytes. **Same UNKNOWN as pass 8.** The 379-byte WAL is the genesis re-seed written after the latest snapshot rotation. The wal_bytes counter may track only entries added after the current snapshot epoch, or was reset at rotation. Cannot classify from endpoint data alone.

**State.snapshot:** 893 bytes (pass 8: 895; shrunk by 2 bytes — likely due to snapshot re-encoding at rotation).

### local-witness
**OBSERVED:** last_snapshot_epoch=240, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** None.

**State.snapshot:** 569 bytes (unchanged from pass 8).

---

## Build Commit & Binary Freshness

**OBSERVED:** `71aa16b-dirty` on both nodes (unchanged since pass 1).
**EXPECTED (VERIFIED-BEHAVIOR.md):** Should match git HEAD (`aa62d12`).
**DEVIATION:** **Persistent.** 8 commits behind. All docs-only changes since `71aa16b` — no wire-format, codec, or protocol changes. Binary was not rebuilt after docs-only commits. Not a functional concern.

---

## Log Health

**OBSERVED:** No ERROR or unexpected WARN lines on either node. The only WARNs are `Failed to gossip genesis (will retry on peer connect)` and `[block-publish] Failed to publish block` — both from startup (14:48 EDT) when the first node had no peers. No new WARN lines since then.

**Grep filter** `grep -vE 'skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|Failed to trigger bootstrap|InsufficientPeers'` — zero operational WARN/ERROR hits on both logs.

---

## Delta from Pass 8

| Metric | Pass 8 (16:34 EDT) | Pass 9 (16:48 EDT) | Delta |
|--------|-------------------|-------------------|-------|
| Uptime (morning-api) | 6356s | 7189s | +833s (~14 min, correct) |
| Uptime (witness) | 6365s | 7191s | +826s |
| Epoch (morning-api) | 213 | 240 | +27 |
| Epoch (witness) | 213 | 241 | +28 |
| Heartbeats (morning-api) | 634 | 717 | +83 |
| Heartbeats (witness) | 637 | 720 | +83 |
| Silence (morning-api) | 2s | 6s | +4s (still well under threshold) |
| Silence (witness) | 4s | 4s | 0 |
| Thickness | 998.30 | 998.08 | -0.22 (expected decay) |
| Balance | 20 | 20 | 0 |
| Nonce | 120 | 120 | 0 |
| Snapshot (morning-api) | 210 | 240 | +30 (3 rotations) |
| Snapshot (witness) | 210 | 240 | +30 (3 rotations) |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| **Epoch three-way** | ✓ (213=213=213) | ✓ (241=241=241 witness; morning-api within race tolerance) | ✓ |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from pass 8). GetPersistenceState reports wal_bytes=0, but `ls -la` shows `/tmp/m-ap/persistence/wal.log` at 379 bytes. The genesis re-seed after snapshot rotation could explain this (post-rotation WAL written after counter snapshot), but this requires inspection of the write path to classify as "expected" or "bug."

---

## Raw Capture (20:48:16Z)

```json
// GetNodeInfo (morning-api)
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":7189,"build_commit":"71aa16b-dirty","thickness":998.0764446850817}

// GetPeers (morning-api)
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":717,"silence_secs":6,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api)
{"type":"EpochState","epoch":240,"ratio":1.01848213703783,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (morning-api)
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api)
{"type":"PersistenceState","last_snapshot_epoch":240,"wal_bytes":0,"wal_entries":0}

// GetHeight (morning-api)
{"type":"Height","height":1}

// GetNodeInfo (local-witness)
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":7191,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness)
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":720,"silence_secs":4,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness)
{"type":"EpochState","epoch":241,"ratio":1.460450325316178,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (local-witness)
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness)
{"type":"PersistenceState","last_snapshot_epoch":240,"wal_bytes":0,"wal_entries":0}
```

---

## Log Evidence (Last 3 Epoch Complete Lines)

**morning-api (/tmp/m-ap.log):**
```
epoch=239 balance_before=20 balance_after=20 ratio=1.02
epoch=240 balance_before=20 balance_after=20 ratio=1.02
epoch=241 balance_before=20 balance_after=20 ratio=1.02
```

**local-witness (/tmp/lw.log):**
```
epoch=239 balance_before=0 balance_after=0 ratio=1.46
epoch=240 balance_before=0 balance_after=0 ratio=1.46
epoch=241 balance_before=0 balance_after=0 ratio=1.46
```

Epoch counts match: morning-api 241, witness 241. Witness ratio declined from 1.47 (pass 8) to 1.46 — continuing expected asymptotic decay.

---

## Lines Triaged as Benign (no change from pass 8)

- `Failed to gossip genesis (will retry on peer connect)` — Startup only (14:48 EDT), first node launched before any peers existed.
- `[block-publish] Failed to publish block proposal_id="genesis" error=InsufficientPeers` — Same startup context.

Both are from the initial bootstrap sequence and have not re-occurred since peer connections were established. Not relevant to current mesh health.

---

**Next check:** No threshold violations. All clear. No new UNKNOWN items beyond the persistent wal_bytes discrepancy. Mesh has been stable for ~6 hours.
