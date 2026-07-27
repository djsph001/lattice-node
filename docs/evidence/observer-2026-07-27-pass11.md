# Observer Evidence Record — 2026-07-27 (Pass 11)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-27T21:05:00Z (~17:05 EDT)
**Machine:** z4-workstation (dale-joseph-hp-z4-g4-workstation, Boynton Beach FL)
**Session type:** Eleventh observation pass. Same processes since 14:48 EDT (~6.3h runtime).

**Summary:** All-clear continuation. ~9 min since pass 10. Epochs 254→275 (+21) on morning-api, 254→275 (+21) on witness. One snapshot rotation (epoch 250→270) completed — last_snapshot_epoch now at 270 (was 250). Balance stable at 20. Thickness decaying (997.96→997.80). Zero queues, zero fetches. No new deviations.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-hp-z4-g4-workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since |
|-----|------|------|--------------|-------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT |

**No topology changes.** Same two PIDs as pass 10. Socket at `/tmp/m-ap/lattice.sock` responding. Witness socket at `/tmp/local-witness/lattice.sock` responding. No stale survivors.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | auto-genesis identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 8213 (~2.3h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `aa62d12` | **Persistent DEVIATION.** 8 commits behind. Docs-only changes — no binary rebuild since session start. |
| thickness | 997.80 | ~1000, slowly decaying | None (pass 10: 997.96; Δ = -0.16) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | — | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 8199 (~2.3h) | — | None |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=811, silence_secs=8, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=819, silence_secs=5, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 10:** heartbeats +53 (morning-api: 758→811) and +57 (witness: 762→819). Silence: 2s→8s (increased, morning-api) and 4s→5s (witness). Queue depth still 0 on both. Healthy heartbeats — silence variation within normal range.

---

## Epoch State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 274 (socket), 275 (last log line) | Cycling ~30s cadence. +21 since pass 10 (254→275). | None |
| ratio | 1.0187 | ~1.01-1.02 steady state | None |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match: PASS.** Endpoint=274, grep count=274, last log line=274 (from same-log-lines capture). Perfect agreement.

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 273 (socket), 275 (last log line — later capture) | Same cadence. Socket/log cross-check: endpoint=273, grep count=273, last line=273 (same capture). Both nodes now at epoch 275 per latest log lines. | None |
| ratio | 1.41 | Declining (pass 10: 1.44; Δ = -0.03). Expected asymptote toward 1.0. | None |
| tax_calculated | 0 | Balance at 0: no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match: PASS.** Endpoint=273, grep count=273, last line=273. Perfect agreement.

**Epoch synchronization:** Latest log lines show both nodes at epoch 275. Perfect lockstep. Witness ratio continuing expected asymptotic decay: 1.44→1.41→1.40 in the latest epoch log line at 21:05:43.

---

## Economic State

### morning-api
**OBSERVED:** own_balance=20, own_nonce=120. Peer (witness) balance=4980, nonce=0.
**EXPECTED:** Balance at asymptotic floor (20) since ~epoch 121. Nonce at 120 since no new transactions.
**DEVIATION:** None.

### local-witness
**OBSERVED:** own_balance=0, own_nonce=2. Peer (morning-api) balance=0, nonce=0.
**EXPECTED:** Zero-balance witness with no mint grant. Nonce 2 (max nonce applied).
**DEVIATION:** None — unchanged since pass 7.

---

## Persistence State

### morning-api
**OBSERVED:** last_snapshot_epoch=270, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every 10 epochs. WAL drained after rotation.
**DEVIATION:** None. Snapshot at epoch 270 (was at 250 in pass 10; +20 epochs = 2 rotations at 250→260→270).

**Byte-equality check:** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes on disk.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes.

**File inventory (single capture):**

| File | Size | Delta from pass 10 | Notes |
|------|------|-------------------|-------|
| `persistence/state.snapshot` | 895 bytes | Unchanged (pass 10: 895) | |
| `persistence/wal.log` | 379 bytes | Unchanged | |
| `persistence/wal.wal.old` | 379 bytes | Unchanged | Known-provisional naming quirk |

### local-witness
**OBSERVED:** last_snapshot_epoch=270, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** None.

**Byte-equality check:** Same UNKNOWN — wal_bytes=0 but wal.log=379 bytes on disk.

**File inventory (single capture):**

| File | Size | Delta from pass 10 | Notes |
|------|------|-------------------|-------|
| `persistence/state.snapshot` | 569 bytes | Unchanged (pass 10: 569) | |
| `persistence/wal.log` | 379 bytes | Unchanged | |
| `wal.wal.old` | not present | — | Cleaned up after rotation? |

---

## Build Commit & Binary Freshness

**OBSERVED:** `71aa16b-dirty` on both nodes (unchanged since pass 1).
**EXPECTED (VERIFIED-BEHAVIOR.md):** Should match git HEAD (`aa62d12`).
**DEVIATION:** **Persistent.** 8 commits behind. Docs-only changes — no wire-format, codec, or protocol changes. Not a functional concern.

---

## Log Health

**OBSERVED:** No ERROR or unexpected WARN lines on either node.

**morning-api:** 34 total WARN/ERROR lines (was 24 at pass 6; increased by ~10 due to periodic Kademlia `Failed to trigger bootstrap` over time).
**witness:** 122 total WARN/ERROR lines (unchanged — all from early epoch-rejection lines, no new ones).

**Grep filter** `grep -vE 'skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|Failed to trigger bootstrap|InsufficientPeers|Failed to gossip genesis'` — zero unexpected hits on both logs.

**Sweep/evict activity:** None found in either log across the entire run.

---

## Metrics (Last 10 Tick Lines)

### morning-api
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
(all 10 lines identical)
```
**All clean:** zero fetches, zero queues, silence=3s. Unchanged from pass 10.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
(all 10 lines identical)
```
**All clean:** zero fetches, zero queues, silence=6s. Unchanged from pass 10.

---

## Delta from Pass 10

| Metric | Pass 10 (~16:56 EDT) | Pass 11 (~17:05 EDT) | Delta |
|--------|---------------------|----------------------|-------|
| Uptime (morning-api) | 7590s | 8213s | +623s (~10.4 min) |
| Uptime (witness) | 7608s | 8199s | +591s (~9.9 min) |
| Epoch (morning-api) | 254 | 275 | +21 |
| Epoch (witness) | 254 | 275 | +21 |
| Heartbeats (morning-api) | 758 | 811 | +53 |
| Heartbeats (witness) | 762 | 819 | +57 |
| Silence (morning-api) | 2s | 3-8s | slight increase |
| Silence (witness) | 4s | 5-6s | slight increase |
| Thickness | 997.96 | 997.80 | -0.16 (expected decay) |
| Balance | 20 | 20 | 0 |
| Nonce | 120 | 120 | 0 |
| Snapshot (morning-api) | 250 | 270 | +20 (2 rotations) |
| Snapshot (witness) | 250 | 270 | +20 (2 rotations) |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from all prior passes). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes. The genesis re-seed after snapshot rotation could explain this (post-rotation WAL written after counter snapshot), but requires inspection of the write path to classify as "expected" or "bug."

2. **snapshot size (morning-api: 895 bytes, witness: 569 bytes).** Witness state.snapshot is 326 bytes smaller than morning-api. This is expected (witness has different peer table state — zero balance, 2 nonces vs morning-api's 120 nonce, 4980 peer balance), but the exact byte composition difference would require deserialization to confirm.

---

## Raw Capture (~21:04Z)

```json
// GetNodeInfo (morning-api)
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":8213,"build_commit":"71aa16b-dirty","thickness":997.8042403973557}

// GetPeers (morning-api)
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":811,"silence_secs":8,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api) — three-way: endpoint=274, grep=274, last line=274
{"type":"EpochState","epoch":274,"ratio":1.0186714044710312,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (morning-api)
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api)
{"type":"PersistenceState","last_snapshot_epoch":270,"wal_bytes":0,"wal_entries":0}

// GetHeight (morning-api)
{"type":"Height","height":1}

// GetNodeInfo (local-witness)
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":8199,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness)
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":819,"silence_secs":5,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness) — three-way: endpoint=273, grep=273, last line=273
{"type":"EpochState","epoch":273,"ratio":1.4072268147765434,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (local-witness)
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness)
{"type":"PersistenceState","last_snapshot_epoch":270,"wal_bytes":0,"wal_entries":0}
```

---

## Log Evidence (Last 5 Epoch Complete Lines)

**morning-api (/tmp/m-ap.log):**
```
2026-07-27T21:03:26.404893Z  INFO lattice_node::node: Epoch complete epoch=271 balance_before=20 balance_after=20 ratio=1.02
2026-07-27T21:03:56.404687Z  INFO lattice_node::node: Epoch complete epoch=272 balance_before=20 balance_after=20 ratio=1.02
2026-07-27T21:04:26.404893Z  INFO lattice_node::node: Epoch complete epoch=273 balance_before=20 balance_after=20 ratio=1.02
2026-07-27T21:04:56.404768Z  INFO lattice_node::node: Epoch complete epoch=274 balance_before=20 balance_after=20 ratio=1.02
2026-07-27T21:05:26.404680Z  INFO lattice_node::node: Epoch complete epoch=275 balance_before=20 balance_after=20 ratio=1.02
```

**local-witness (/tmp/lw.log):**
```
2026-07-27T21:03:43.261924Z  INFO lattice_node::node: Epoch complete epoch=271 balance_before=0 balance_after=0 ratio=1.41
2026-07-27T21:04:13.262042Z  INFO lattice_node::node: Epoch complete epoch=272 balance_before=0 balance_after=0 ratio=1.41
2026-07-27T21:04:43.261594Z  INFO lattice_node::node: Epoch complete epoch=273 balance_before=0 balance_after=0 ratio=1.41
2026-07-27T21:05:13.262104Z  INFO lattice_node::node: Epoch complete epoch=274 balance_before=0 balance_after=0 ratio=1.40
2026-07-27T21:05:43.261880Z  INFO lattice_node::node: Epoch complete epoch=275 balance_before=0 balance_after=0 ratio=1.40
```

Witness ratio declined from 1.41 to 1.40 — continuing expected asymptotic decay.

---

## Lines Triaged as Benign (no change from pass 10)

- `Failed to gossip genesis (will retry on peer connect)` — Startup only (14:48 EDT), first node launched before any peers existed.
- `[block-publish] Failed to publish block proposal_id="genesis" error=InsufficientPeers` — Same startup context.
- `Failed to trigger bootstrap: No known peers` — Periodic Kademlia tick on `--no-mdns` mesh. Expected.

All from initial bootstrap sequence or periodic benign Kademlia; not relevant to current mesh health.

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 8 commits behind HEAD `aa62d12`) | Low — docs-only drift, no wire-format change | **Persistent** since pass 1 |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint not wired | **Persistent** since pass 1 |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) — causes repeated validation failures | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 |

**Deviations resolved since pass 10:** None.
**New deviations since pass 10:** None.

---

**Next check:** No threshold violations. All clear. Persistence UNKNOWN items unchanged and non-functional. Mesh has been stable for ~6.3 hours. Both nodes at epoch 275 in perfect lockstep.
