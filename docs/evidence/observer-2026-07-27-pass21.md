# Observer Evidence Record — 2026-07-27 (Pass 21)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-27T22:34Z–22:36Z (~18:34 EDT)
**Machine:** z4-workstation (dale-joseph-HP-Z4-G4-Workstation, Boynton Beach FL)
**Session type:** Twenty-first observation pass. Same processes since 14:48 EDT (~7.8h runtime).

**Summary:** All-clear continuation. ~7 min since pass 20 (22:27Z). Epochs 439→454/455 (+15/16) on both nodes. Snapshot rotated twice (430→440→450). Balance locked at 20. Thickness 996.36 (decaying). Zero queues, zero fetches, zero sweep/evict/zombie activity. Both nodes healthy — three-way PASS on both in same capture (witness shows race at epoch boundary: socket=454, grep=454, last-line=455). No new deviations.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs as pass 1 through 20. Both sockets responding. No stale survivor processes (`pgrep -fl lattice-node` shows only 2 lattice-node PIDs). Logs: `/tmp/m-ap.log` (morning-api) and `/tmp/lw.log` (witness).

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 13628 (~3.8h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `aa62d12` | **Persistent DEVIATION.** 8 commits behind. Docs-only changes — no binary rebuild since session start. |
| thickness | 996.36 | ~1000, slowly decaying | None (pass 20: 996.49; Δ = -0.13) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 13614 (~3.8h) | — | None |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=1355, silence_secs=2, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=1358, silence_secs=4, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 20:** Heartbeats morning-api +45 (1310→1355), witness +46 (1312→1358). Silence: morning-api 3s→2s, witness 6s→4s (normal variation). Queue depth still 0 on both.

---

## Epoch State

### morning-api (simultaneous capture, ~22:35Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 454 (socket), 454 (grep), 455 (last log line) | Cycling ~30s cadence. +15 since pass 20 (439→454). | **RACE at boundary:** last log line is +1 from socket/grep — expected at epoch transition. |
| ratio | 1.0192 | ~1.01–1.02 steady state (pass 20: 1.0192) | None |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (~22:35Z):**
- Socket epoch: 454
- `grep -c` count: 454
- Last log line: epoch=455 (22:35:26Z)

**BORDERLINE — race at epoch transition.** Socket+grep agree at 454. Last log at 455 is normal — the epoch completed ~30s after the socket query.

### local-witness (same capture, ~22:35Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 454 (socket), 454 (grep), 454 (last log line) | Same cadence. +15 since pass 20 (439→454). | None |
| ratio | 1.241 | Declining (pass 20: 1.249; Δ ≈ -0.008). Expected asymptotic decay toward 1.0. | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (~22:35Z):**
- Socket epoch: 454
- `grep -c` count: 454
- Last log line: epoch=454 (22:35:13Z)

**PASS.** All three agree.

**Epoch synchronization:** morning-api at 454/455 (boundary), witness at 454. ~30s skew normal.

---

## Economic State

### morning-api
**OBSERVED:** own_balance=20, own_nonce=120. Peer (witness) balance=4980, nonce=0.
**EXPECTED:** Balance at asymptotic floor (20) since ~epoch 121. Nonce at 120 since no new transactions.
**DEVIATION:** None.

### local-witness
**OBSERVED:** own_balance=0, own_nonce=2. Peer (morning-api) balance=0, nonce=0.
**EXPECTED:** Zero-balance witness with no mint grant. Nonce 2 (max nonce applied).
**DEVIATION:** Persistent — witness sees morning-api balance as 0 (actual: 20). Same since pass 1.

---

## Persistence State

### morning-api
**OBSERVED:** last_snapshot_epoch=450, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every 10 epochs. WAL drained after rotation.
**DEVIATION:** None. Snapshot at epoch 450 (was 430 in pass 20; +20 epochs = 2 rotations: 430→440→450).

**Byte-equality check (~22:34Z):** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes on disk.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes.

**File inventory (single capture, ~22:34Z):**

| File | Size | Timestamp | Delta from pass 20 (22:27Z) | Notes |
|------|------|-----------|-------------------|-------|
| `persistence/state.snapshot` | 895 bytes | 18:32 EDT | 895→895 (size same, timestamp changed) | New rotation (epoch 450, was 430) |
| `persistence/wal.log` | 379 bytes | 18:32 EDT | Same size, new timestamp | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | 18:27 EDT | Same size, new timestamp | Prior rotation's WAL backup |

### local-witness
**OBSERVED:** last_snapshot_epoch=450, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** None.

**Byte-equality check (~22:34Z):** Same UNKNOWN — wal_bytes=0 but wal.log=379 bytes on disk.

**File inventory (single capture, ~22:34Z):**

| File | Size | Timestamp | Delta from pass 20 (22:27Z) | Notes |
|------|------|-----------|-------------------|-------|
| `persistence/state.snapshot` | 569 bytes | 18:33 EDT | Same size, new timestamp | Rotated at epoch 450 (was 430) |
| `persistence/wal.log` | 379 bytes | 18:33 EDT | Same size, new timestamp | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | 18:28 EDT | Same size, new timestamp | Prior rotation's WAL backup |

---

## Build Commit & Binary Freshness

**OBSERVED:** `71aa16b-dirty` on both nodes (unchanged since pass 1).
**EXPECTED (VERIFIED-BEHAVIOR.md):** Should match git HEAD (`aa62d12`).
**DEVIATION:** **Persistent.** 8 commits behind. Docs-only changes — no wire-format, codec, or protocol changes. Not a functional concern.

---

## Log Health

**OBSERVED:** No ERROR or unexpected WARN lines on either node from filtered scan.

**morning-api:** 52 total WARN/ERROR lines (was 50 at pass 20; +2 from periodic Kademlia `Failed to trigger bootstrap` ticks).

**witness (/tmp/lw.log):** 122 total WARN/ERROR lines (unchanged from pass 20 — all from early epoch-rejection lines prior to this session, no new ones).

**Filtered health scan** (`grep -vE 'skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|Failed to trigger bootstrap|InsufficientPeers|Failed to gossip genesis|Failed to publish block'`) — **zero hits** on both nodes. All WARN/ERROR lines belong to known-benign categories.

**Sweep/evict/zombie activity:** None found in either log across the entire run. `grep -ciE 'sweep|evict|zombie' /tmp/m-ap.log /tmp/lw.log` = 0 hits on both.

---

## Metrics (Last 5 Tick Lines)

### morning-api
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
(all 5 lines identical, timestamps 22:34:06–22:34:46Z)
```
**All clean:** zero fetches, zero queues, silence=3s. Unchanged from pass 20.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
(all 5 lines identical, timestamps 22:34:33–22:35:13Z)
```
**All clean:** zero fetches, zero queues, silence=6s. Unchanged from pass 20.

---

## Recent Epoch Activity (Last 5 Lines Each)

**morning-api:**
```
Epoch complete epoch=451 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=452 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=453 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=454 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=455 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at 1.02.

**local-witness:**
```
Epoch complete epoch=450 balance_before=0 balance_after=0 ratio=1.24
Epoch complete epoch=451 balance_before=0 balance_after=0 ratio=1.24
Epoch complete epoch=452 balance_before=0 balance_after=0 ratio=1.24
Epoch complete epoch=453 balance_before=0 balance_after=0 ratio=1.24
Epoch complete epoch=454 balance_before=0 balance_after=0 ratio=1.24
```
Ratio: 1.24 continuing expected asymptotic decline (pass 20: 1.25→1.24, trending down).

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 8 commits behind HEAD `aa62d12`) | Low — docs-only drift, no wire-format change | **Persistent** since pass 1 |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint not wired | **Persistent** since pass 1 |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) — causes repeated validation failures | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 |

**Deviations resolved since pass 20:** None.
**New deviations since pass 21:** None.

---

## Delta from Pass 20 (22:27Z → 22:35Z)

| Metric | Pass 20 (~18:27 EDT) | Pass 21 (~18:34 EDT) | Delta |
|--------|---------------------|-----------------------|-------|
| Uptime (morning-api) | 13118s | 13628s | +510s (~8.5 min) |
| Uptime (witness) | 13117s | 13614s | +497s (~8.3 min) |
| Epoch (morning-api socket) | 439 | 454 | +15 |
| Epoch (witness socket) | 439 | 454 | +15 |
| Heartbeats (morning-api) | 1310 | 1355 | +45 |
| Heartbeats (witness) | 1312 | 1358 | +46 |
| Silence (morning-api) | 3s | 2s | -1s (normal variation) |
| Silence (witness) | 6s | 4s | -2s (normal variation) |
| Thickness | 996.49 | 996.36 | -0.13 (expected decay) |
| Balance | 20 | 20 | 0 |
| Nonce | 120 | 120 | 0 |
| Snapshot epoch (both) | 430 | 450 | +20 (2 rotations) |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| WARN count (morning-api) | 50 | 52 | +2 (Kademlia ticks) |
| WARN count (witness) | 122 | 122 | 0 |
| Snapshot size (morning-api) | 895 bytes | 895 bytes | 0 |
| Snapshot size (witness) | 569 bytes | 569 bytes | 0 |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from all prior passes). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes.

2. **snapshot size (morning-api: 895 bytes, witness: 569 bytes).** Witness state.snapshot is 326 bytes smaller than morning-api. Expected (witness has different peer table state — zero balance, 2 nonces vs morning-api's 120 nonce, 4980 peer balance), but byte composition unconfirmed without deserialization.

---

## Raw Capture Bundle

Single-capture queries from ~22:34–22:36Z:

```
// === GetNodeInfo (morning-api, ~22:35Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":13628,"build_commit":"71aa16b-dirty","thickness":996.3564019294564}

// === GetPeers (morning-api, ~22:34Z) ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":1355,"silence_secs":2,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (morning-api, ~22:35Z) — three-way: endpoint=454, grep=454, last line=455 (boundary race) ===
{"type":"EpochState","epoch":454,"ratio":1.019950774007497,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (morning-api, ~22:34Z) ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetPersistenceState (morning-api, ~22:34Z) ===
{"type":"PersistenceState","last_snapshot_epoch":450,"wal_bytes":0,"wal_entries":0}

// === GetHeight (morning-api, ~22:34Z) ===
{"type":"Height","height":1}

// === GetNodeInfo (local-witness, ~22:35Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":13614,"build_commit":"71aa16b-dirty"}

// === GetPeers (local-witness, ~22:35Z) ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":1358,"silence_secs":4,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (local-witness, ~22:35Z) — three-way: endpoint=454, grep=454, last line=454 (PASS) ===
{"type":"EpochState","epoch":454,"ratio":1.2408452749806351,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (local-witness, ~22:35Z) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// === GetPersistenceState (local-witness, ~22:35Z) ===
{"type":"PersistenceState","last_snapshot_epoch":450,"wal_bytes":0,"wal_entries":0}
```

---

## Bottom Line

**No new deviations. All three persistent anomalies (stale binary, wal_bytes endpoint, witness balance blindness) unchanged since pass 1. Mesh has been running healthy for ~7.8 hours. Both nodes at epoch 454/455 — healthy lockstep with expected boundary race. 2 snapshot rotations completed since pass 20 (epoch 430→440→450). No sweep/evict/zombie activity recorded. Zero queues, zero fetches across all metrics ticks.**

**Next check:** No threshold violations. System in stable steady state.
