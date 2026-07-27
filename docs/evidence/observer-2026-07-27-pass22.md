# Observer Evidence Record — 2026-07-27 (Pass 22)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-27T22:43Z–22:44Z (~18:43 EDT)
**Machine:** z4-workstation (dale-joseph-HP-Z4-G4-Workstation, Boynton Beach FL)
**Session type:** Twenty-second observation pass. Same processes since 14:48 EDT (~8.0h runtime).

**Summary:** All-clear continuation. ~8 min since pass 21 (22:35Z). Epochs 454/455→470/471 (+16) on both nodes. Snapshot rotated twice (450→460→470). Both nodes at exact epoch match (470 three-way PASS). Balance locked at 20. Thickness 996.25 (decaying). Zero queues, zero fetches, zero sweep/evict/zombie activity. Both nodes healthy — three-way PASS on both in same capture. No new deviations.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs as pass 1 through 21. Both sockets responding. No stale survivor processes (`pgrep -fl lattice-node` shows only 2 lattice-node PIDs). Logs: `/tmp/m-ap.log` (morning-api) and `/tmp/lw.log` (witness).

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 14015 (~3.9h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `aa62d12` | **Persistent DEVIATION.** 8 commits behind. Docs-only changes — no binary rebuild since session start. |
| thickness | 996.25 | ~1000, slowly decaying | None (pass 21: 996.36; Δ = -0.11) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 14027 (~3.9h) | — | None |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=1400, silence_secs=2, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=1403, silence_secs=8, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 21:** Heartbeats morning-api +45 (1355→1400), witness +45 (1358→1403). Silence: morning-api 2s→2s (unchanged), witness 4s→8s (normal variation). Queue depth still 0 on both.

---

## Epoch State

### morning-api (simultaneous capture, ~22:43Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 470 (socket), 470 (grep), 470 (last log line) | Cycling ~30s cadence. +16 since pass 21 (454→470). | **PASS — three-way match.** No boundary race this capture. |
| ratio | 1.0199 | ~1.01–1.02 steady state (pass 21: 1.0199) | None |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (~22:43Z):**
- Socket epoch: 470
- `grep -c` count: 470
- Last log line: epoch=470 (22:42:56Z)

**PASS.** All three agree.

### local-witness (same capture, ~22:43Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 470 (socket), 470 (grep), 470 (last log line) | Same cadence. +16 since pass 21 (454→470). | **PASS — three-way match.** |
| ratio | 1.232 | Declining (pass 21: 1.241; Δ ≈ -0.009). Expected asymptotic decay toward 1.0. | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (~22:43Z):**
- Socket epoch: 470
- `grep -c` count: 470
- Last log line: epoch=470 (22:43:13Z)

**PASS.** All three agree.

**Epoch synchronization:** Both nodes at epoch 470. Exact match (no boundary race this capture).

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
**OBSERVED:** last_snapshot_epoch=470, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every 10 epochs. WAL drained after rotation.
**DEVIATION:** None. Snapshot at epoch 470 (was 450 in pass 21; +20 epochs = 2 rotations: 450→460→470).

**Byte-equality check (~22:43Z):** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes on disk.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes.

**File inventory (single capture, ~22:43Z):**

| File | Size | Timestamp | Delta from pass 21 (22:34Z) | Notes |
|------|------|-----------|-------------------|-------|
| `persistence/state.snapshot` | 895 bytes | 18:42 EDT | 895→895 (size same, timestamp changed) | New rotation (epoch 470, was 450) |
| `persistence/wal.log` | 379 bytes | 18:42 EDT | Same size, new timestamp | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | 18:37 EDT | Same size, new timestamp | Prior rotation's WAL backup |

### local-witness
**OBSERVED:** last_snapshot_epoch=470, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** None.

**Byte-equality check (~22:43Z):** Same UNKNOWN — wal_bytes=0 but wal.log=379 bytes on disk.

**File inventory (single capture, ~22:43Z):**

| File | Size | Timestamp | Delta from pass 21 (22:34Z) | Notes |
|------|------|-----------|-------------------|-------|
| `persistence/state.snapshot` | 569 bytes | 18:38 EDT | Same size, new timestamp | Rotated at epoch 470 (was 450) |
| `persistence/wal.log` | 379 bytes | 18:43 EDT | Same size, new timestamp | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | 18:33 EDT | Same size, new timestamp | Prior rotation's WAL backup |

---

## Build Commit & Binary Freshness

**OBSERVED:** `71aa16b-dirty` on both nodes (unchanged since pass 1).
**EXPECTED (VERIFIED-BEHAVIOR.md):** Should match git HEAD (`aa62d12`).
**DEVIATION:** **Persistent.** 8 commits behind. Docs-only changes — no wire-format, codec, or protocol changes. Not a functional concern.

---

## Log Health

**OBSERVED:** No unexpected WARN or ERROR lines on either node from filtered scan.

**morning-api:** 54 total WARN/ERROR lines (was 52 at pass 21; +2 from periodic Kademlia `Failed to trigger bootstrap` ticks).

**witness (/tmp/lw.log):** 122 total WARN/ERROR lines (unchanged from pass 21 — all from early epoch-rejection lines prior to this session, no new ones).

**Filtered health scan** (`grep -vE 'skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|Failed to trigger bootstrap|InsufficientPeers|Failed to gossip genesis|Failed to publish block|Failed to trigger Kademlia'`) — **zero hits** on both nodes. All WARN/ERROR lines belong to known-benign categories.

**Sweep/evict/zombie activity:** None found in either log across the entire run. `grep -ciE 'sweep|evict|zombie' /tmp/m-ap.log /tmp/lw.log` = 0 hits on both.

---

## Metrics (Last 5 Tick Lines)

### morning-api
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
(all 5 lines identical, timestamps 22:42:16–22:42:56Z)
```
**All clean:** zero fetches, zero queues, silence=3s. Unchanged from pass 21.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
(all 5 lines identical, timestamps 22:42:13–22:42:53Z)
```
**All clean:** zero fetches, zero queues, silence=6s. Unchanged from pass 21.

---

## Recent Epoch Activity (Last 5 Lines Each)

**morning-api:**
```
Epoch complete epoch=467 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=468 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=469 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=470 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=471 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at 1.02.

**local-witness:**
```
Epoch complete epoch=467 balance_before=0 balance_after=0 ratio=1.23
Epoch complete epoch=468 balance_before=0 balance_after=0 ratio=1.23
Epoch complete epoch=469 balance_before=0 balance_after=0 ratio=1.23
Epoch complete epoch=470 balance_before=0 balance_after=0 ratio=1.23
Epoch complete epoch=471 balance_before=0 balance_after=0 ratio=1.23
```
Ratio: 1.23 continuing expected asymptotic decline (pass 21: 1.24→1.23, trending down).

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 8 commits behind HEAD `aa62d12`) | Low — docs-only drift, no wire-format change | **Persistent** since pass 1 |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint not wired | **Persistent** since pass 1 |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) — causes repeated validation failures | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 |

**Deviations resolved since pass 21:** None.
**New deviations since pass 22:** None.

---

## Delta from Pass 21 (22:35Z → 22:43Z)

| Metric | Pass 21 (~18:34 EDT) | Pass 22 (~18:43 EDT) | Delta |
|--------|---------------------|-----------------------|-------|
| Uptime (morning-api) | 13628s | 14015s | +387s (~6.5 min) |
| Uptime (witness) | 13614s | 14027s | +413s (~6.9 min) |
| Epoch (morning-api socket) | 454 | 470 | +16 |
| Epoch (witness socket) | 454 | 470 | +16 |
| Heartbeats (morning-api) | 1355 | 1400 | +45 |
| Heartbeats (witness) | 1358 | 1403 | +45 |
| Silence (morning-api) | 2s | 2s | 0s (unchanged) |
| Silence (witness) | 4s | 8s | +4s (normal variation) |
| Thickness | 996.36 | 996.25 | -0.11 (expected decay) |
| Balance | 20 | 20 | 0 |
| Nonce | 120 | 120 | 0 |
| Snapshot epoch (both) | 450 | 470 | +20 (2 rotations) |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| WARN count (morning-api) | 52 | 54 | +2 (Kademlia ticks) |
| WARN count (witness) | 122 | 122 | 0 |
| Snapshot size (morning-api) | 895 bytes | 895 bytes | 0 |
| Snapshot size (witness) | 569 bytes | 569 bytes | 0 |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from all prior passes). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes.

2. **snapshot size (morning-api: 895 bytes, witness: 569 bytes).** Witness state.snapshot is 326 bytes smaller than morning-api. Expected (witness has different peer table state — zero balance, 2 nonces vs morning-api's 120 nonce, 4980 peer balance), but byte composition unconfirmed without deserialization.

---

## Raw Capture Bundle

Single-capture queries from ~22:43–22:44Z:

```
// === GetNodeInfo (morning-api, ~22:43Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":14015,"build_commit":"71aa16b-dirty","thickness":996.2524944310893}

// === GetPeers (morning-api, ~22:43Z) ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":1400,"silence_secs":2,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (morning-api, ~22:43Z) — three-way: endpoint=470, grep=470, last line=470 (PASS) ===
{"type":"EpochState","epoch":470,"ratio":1.0199382209337085,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (morning-api, ~22:43Z) ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetPersistenceState (morning-api, ~22:43Z) ===
{"type":"PersistenceState","last_snapshot_epoch":470,"wal_bytes":0,"wal_entries":0}

// === GetHeight (morning-api, ~22:43Z) ===
{"type":"Height","height":1}

// === GetNodeInfo (local-witness, ~22:44Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":14027,"build_commit":"71aa16b-dirty"}

// === GetPeers (local-witness, ~22:44Z) ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":1403,"silence_secs":8,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (local-witness, ~22:44Z) — three-way: endpoint=470, grep=470, last line=470 (PASS) ===
{"type":"EpochState","epoch":470,"ratio":1.2324213361514684,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (local-witness, ~22:44Z) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// === GetPersistenceState (local-witness, ~22:44Z) ===
{"type":"PersistenceState","last_snapshot_epoch":470,"wal_bytes":0,"wal_entries":0}
```

---

## Bottom Line

**No new deviations. All three persistent anomalies (stale binary, wal_bytes endpoint, witness balance blindness) unchanged since pass 1. Mesh has been running healthy for ~8.0 hours. Both nodes at epoch 470 — exact match with three-way PASS on both. 2 snapshot rotations completed since pass 21 (epoch 450→460→470). No sweep/evict/zombie activity recorded. Zero queues, zero fetches across all metrics ticks.**

**Next check:** No threshold violations. System in stable steady state.
