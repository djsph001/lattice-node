# Observer Evidence Record — 2026-07-27 (Pass 19)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-27T22:19Z–22:20Z (~18:19 EDT)
**Machine:** z4-workstation (dale-joseph-HP-Z4-G4-Workstation, Boynton Beach FL)
**Session type:** Nineteenth observation pass. Same processes since 14:48 EDT (~7.5h runtime).

**Summary:** All-clear continuation. ~8 min since pass 18 (22:12Z). Epochs 408→424 (+16) on both nodes. Snapshot rotated twice (400→410→420). Balance locked at 20. Thickness 996.62 (decaying). Zero queues, zero fetches, zero sweep/evict/zombie activity. Both nodes in perfect lockstep — three-way PASS on both in the same capture. No new deviations.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs as pass 1 through 18. Both sockets responding. No stale survivor processes (`pgrep -fl lattice-node` shows only 2 PIDs: 2727391, 2727569). Logs: `/tmp/m-ap.log` (morning-api) and `/tmp/lw.log` (witness).

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 12636 (~3.5h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `aa62d12` | **Persistent DEVIATION.** 8 commits behind. Docs-only changes — no binary rebuild since session start. |
| thickness | 996.62 | ~1000, slowly decaying | None (pass 18: 996.74; Δ = -0.12) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 12639 (~3.5h) | — | None |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=1262, silence_secs=2, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=1264, silence_secs=8, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 18:** Heartbeats morning-api +45 (1217→1262), witness +45 (1219→1264). Silence: morning-api 3s→2s, witness 3s→8s (normal variation). Queue depth still 0 on both.

---

## Epoch State

### morning-api (single capture, 22:20:11Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 424 (socket, grep, log all agree) | Cycling ~30s cadence. +16 since pass 18 (408→424). | None |
| ratio | 1.0191 | ~1.01–1.02 steady state (pass 18: 1.0191) | None |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (22:20:11Z):**
- Socket epoch: 424
- `grep -c` count: 424
- Last log line: epoch=424 (22:19:56Z)

**PASS.** All three agree.

### local-witness (same capture, ~22:20:11Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 424 (socket, grep, log all agree) | Same cadence. +16 since pass 18 (408→424). | None |
| ratio | 1.258 | Declining (pass 18: 1.269; Δ ≈ -0.011). Expected asymptotic decay toward 1.0. | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (~22:20:11Z):**
- Socket epoch: 424
- `grep -c` count: 424
- Last log line: epoch=424 (22:20:13Z)

**PASS.** All three agree.

**Epoch synchronization:** Both nodes at epoch 424 in the same capture. Perfect lockstep.

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
**OBSERVED:** last_snapshot_epoch=420, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every 10 epochs. WAL drained after rotation.
**DEVIATION:** None. Snapshot at epoch 420 (was 400 in pass 18; +20 epochs = 2 rotations: 410, 420).

**Byte-equality check (22:19:Z):** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes on disk.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes.

**File inventory (single capture, ~22:19Z):**

| File | Size | Timestamp | Delta from pass 18 (22:12Z) | Notes |
|------|------|-----------|-------------------|-------|
| `persistence/state.snapshot` | 894 bytes | 18:17 EDT | 895→894 (-1) | New rotation (epoch 420, was 400) |
| `persistence/wal.log` | 379 bytes | 18:17 EDT | Same size, new timestamp | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | 18:12 EDT | Same size, new timestamp | Prior rotation's WAL backup |

### local-witness
**OBSERVED:** last_snapshot_epoch=420, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** None.

**Byte-equality check (22:19Z):** Same UNKNOWN — wal_bytes=0 but wal.log=379 bytes on disk.

**File inventory (single capture, ~22:19Z):**

| File | Size | Timestamp | Delta from pass 18 (22:12Z) | Notes |
|------|------|-----------|-------------------|-------|
| `persistence/state.snapshot` | 569 bytes | 18:18 EDT | Same size, new timestamp | Rotated at epoch 420 (was 400) |
| `persistence/wal.log` | 379 bytes | 18:18 EDT | Same size, new timestamp | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | 18:13 EDT | Same size, new timestamp | Prior rotation's WAL backup |

---

## Build Commit & Binary Freshness

**OBSERVED:** `71aa16b-dirty` on both nodes (unchanged since pass 1).
**EXPECTED (VERIFIED-BEHAVIOR.md):** Should match git HEAD (`aa62d12`).
**DEVIATION:** **Persistent.** 8 commits behind. Docs-only changes — no wire-format, codec, or protocol changes. Not a functional concern.

---

## Log Health

**OBSERVED:** No ERROR or unexpected WARN lines on either node from filtered scan.

**morning-api:** 49 total WARN/ERROR lines (was 46 at pass 18; +3 from periodic Kademlia `Failed to trigger bootstrap` at ~5-min cadence, expected over ~8 min).

**witness (/tmp/lw.log):** 122 total WARN/ERROR lines (unchanged from pass 18 — all from early epoch-rejection lines prior to this session, no new ones).

**Filtered health scan** (`grep -vE 'skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|Failed to trigger bootstrap|InsufficientPeers|Failed to gossip genesis|Failed to publish block'`) — **zero hits** on both nodes. All WARN/ERROR lines belong to known-benign categories.

**Sweep/evict/zombie activity:** None found in either log across the entire run. `grep -ciE 'sweep|evict|zombie' /tmp/m-ap.log /tmp/lw.log` = 0 hits on both.

---

## Metrics (Last 5 Tick Lines)

### morning-api
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
(all 5 lines identical, timestamps 22:19:16–22:19:56Z)
```
**All clean:** zero fetches, zero queues, silence=3s. Unchanged from pass 18.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
(all 5 lines identical, timestamps 22:19:13–22:19:53Z)
```
**All clean:** zero fetches, zero queues, silence=6s. Unchanged from pass 18.

---

## Recent Epoch Activity (Last 5 Lines Each)

**morning-api:**
```
Epoch complete epoch=420 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=421 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=422 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=423 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=424 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at 1.02.

**local-witness:**
```
Epoch complete epoch=419 balance_before=0 balance_after=0 ratio=1.26
Epoch complete epoch=420 balance_before=0 balance_after=0 ratio=1.26
Epoch complete epoch=421 balance_before=0 balance_after=0 ratio=1.26
Epoch complete epoch=422 balance_before=0 balance_after=0 ratio=1.26
Epoch complete epoch=423 balance_before=0 balance_after=0 ratio=1.26
```
Ratio: 1.28→1.26 continuing expected asymptotic decline (pass 18: 1.28, pass 17: 1.30).

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 8 commits behind HEAD `aa62d12`) | Low — docs-only drift, no wire-format change | **Persistent** since pass 1 |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint not wired | **Persistent** since pass 1 |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) — causes repeated validation failures | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 |

**Deviations resolved since pass 18:** None.
**New deviations since pass 19:** None.

---

## Delta from Pass 18 (22:12Z → 22:20Z)

| Metric | Pass 18 (~18:12 EDT) | Pass 19 (~18:19 EDT) | Delta |
|--------|---------------------|----------------------|-------|
| Uptime (morning-api) | 12186s | 12636s | +450s (~7.5 min) |
| Uptime (witness) | 12182s | 12639s | +457s (~7.6 min) |
| Epoch (morning-api socket) | 408 | 424 | +16 |
| Epoch (witness socket) | 408 | 424 | +16 |
| Heartbeats (morning-api) | 1217 | 1262 | +45 |
| Heartbeats (witness) | 1219 | 1264 | +45 |
| Silence (morning-api) | 3s | 2s | -1s (normal variation) |
| Silence (witness) | 3s | 8s | +5s (normal variation) |
| Thickness | 996.74 | 996.62 | -0.12 (expected decay) |
| Balance | 20 | 20 | 0 |
| Nonce | 120 | 120 | 0 |
| Snapshot epoch (both) | 400 | 420 | +20 (2 rotations) |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| WARN count (morning-api) | 46 | 49 | +3 (Kademlia ticks) |
| WARN count (witness) | 122 | 122 | 0 |
| Snapshot size (morning-api) | 895 bytes | 894 bytes | -1 (negligible) |
| Snapshot size (witness) | 569 bytes | 569 bytes | 0 |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from all prior passes). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes.

2. **snapshot size (morning-api: 894 bytes, witness: 569 bytes).** Witness state.snapshot is 325 bytes smaller than morning-api. Expected (witness has different peer table state — zero balance, 2 nonces vs morning-api's 120 nonce, 4980 peer balance), but byte composition unconfirmed without deserialization.

---

## Raw Capture Bundle

Single-capture queries from ~22:19–22:20Z:

```
// === GetNodeInfo (morning-api, ~22:19:03Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":12636,"build_commit":"71aa16b-dirty","thickness":996.6202157922484}

// === GetPeers (morning-api, ~22:19:03Z) ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":1262,"silence_secs":2,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (morning-api, 22:20:11Z) — three-way: endpoint=424, grep=424, last line=424 ===
{"type":"EpochState","epoch":424,"ratio":1.0191429067144757,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (morning-api, ~22:19:03Z) ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetPersistenceState (morning-api, ~22:19:03Z) ===
{"type":"PersistenceState","last_snapshot_epoch":420,"wal_bytes":0,"wal_entries":0}

// === GetHeight (morning-api, ~22:19:03Z) ===
{"type":"Height","height":1}

// === GetNodeInfo (local-witness, ~22:19:22Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":12639,"build_commit":"71aa16b-dirty"}

// === GetPeers (local-witness, ~22:19:22Z) ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":1264,"silence_secs":8,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (local-witness, 22:20:11Z) — three-way: endpoint=424, grep=424, last line=424 ===
{"type":"EpochState","epoch":424,"ratio":1.2583938199917046,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (local-witness, ~22:19:22Z) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// === GetPersistenceState (local-witness, ~22:19:22Z) ===
{"type":"PersistenceState","last_snapshot_epoch":420,"wal_bytes":0,"wal_entries":0}
```

---

## Bottom Line

**No new deviations. All three persistent anomalies (stale binary, wal_bytes endpoint, witness balance blindness) unchanged since pass 1. Mesh has been running healthy for ~7.5 hours. Both nodes at epoch 424 in perfect lockstep — three-way PASS on both nodes simultaneously. 2 snapshot rotations completed since pass 18 (epoch 400→420). No sweep/evict/zombie activity recorded. Zero queues, zero fetches across all metrics ticks.**

**Next check:** No threshold violations. System in stable steady state.
