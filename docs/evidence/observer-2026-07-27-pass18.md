# Observer Evidence Record — 2026-07-27 (Pass 18)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-27T22:11Z–22:12Z (~18:11 EDT)
**Machine:** z4-workstation (dale-joseph-HP-Z4-G4-Workstation, Boynton Beach FL)
**Session type:** Eighteenth observation pass. Same processes since 14:48 EDT (~7.3h runtime).

**Summary:** All-clear continuation. ~20 min since pass 17 (21:52Z). Epochs 369→408 (+39) on morning-api, witness at 408 (+39). Snapshot rotated 4 times: 360→370→380→390→400. Balance stable at 20. Thickness decaying (997.05→996.74). Zero queues, zero fetches. No sweep/evict/zombie activity. Both nodes in perfect lockstep — first three-way PASS on both nodes in the same capture (socket=408, grep=408, last line=408). No new deviations.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs as pass 1. Both sockets responding. No stale survivor processes (`pgrep -fl lattice-node` shows only 2 PIDs: 2727391, 2727569). Logs: `/tmp/m-ap.log` (morning-api) and `/tmp/lw.log` (witness).

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 12186 (~3.4h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `aa62d12` | **Persistent DEVIATION.** 8 commits behind. Docs-only changes — no binary rebuild since session start. |
| thickness | 996.74 | ~1000, slowly decaying | None (pass 17: 997.05; Δ = -0.31) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 12182 (~3.4h) | — | None |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=1217, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=1219, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 17:** Heartbeats morning-api +117 (1100→1217), witness +115 (1104→1219). Silence: both at 3s (was 5s/8s in pass 17; normal variation). Queue depth still 0 on both.

---

## Epoch State

### morning-api (single capture, 22:12:21Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 408 (socket, grep, log all agree) | Cycling ~30s cadence. +39 since pass 17 (369→408). | None |
| ratio | 1.0191 | ~1.01–1.02 steady state (pass 17: 1.0190) | None |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (22:12:21Z):**
- Socket epoch: 408
- `grep -c` count: 408
- Last log line: epoch=408 (22:11:56Z)

**PASS.** All three agree. First perfect three-way match on morning-api since pass 1 (previous passes had off-by-1 races at tick boundaries).

### local-witness (same capture, ~22:12:21Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 408 (socket, grep, log all agree) | Same cadence. +39 since pass 17 (369→408). | None — first perfect match across all three signals. |
| ratio | 1.269 | Declining (pass 17: 1.298; Δ ≈ -0.029). Expected asymptotic decay toward 1.0. | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (~22:12:21Z):**
- Socket epoch: 408
- `grep -c` count: 408
- Last log line: epoch=408 (22:12:13Z)

**PASS.** All three agree.

**Epoch synchronization:** Both nodes at epoch 408 in the same capture. Perfect lockstep.

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
**OBSERVED:** last_snapshot_epoch=400, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every 10 epochs. WAL drained after rotation.
**DEVIATION:** None. Snapshot at epoch 400 (was 360 in pass 16/17; +40 epochs = 4 rotations: 370, 380, 390, 400).

**Byte-equality check (22:12:26Z):** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes on disk.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes.

**File inventory (single capture, ~22:12Z):**

| File | Size | Timestamp | Delta from pass 17 (21:52Z) | Notes |
|------|------|-----------|-------------------|-------|
| `persistence/state.snapshot` | 895 bytes | 18:07 EDT | 894→895 (+1) | New rotation (epoch 400, was 360) |
| `persistence/wal.log` | 379 bytes | 18:07 EDT | Same size, new timestamp | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | 18:02 EDT | Same size, new timestamp | Prior rotation's WAL backup |

### local-witness
**OBSERVED:** last_snapshot_epoch=400, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** None.

**Byte-equality check (22:12:26Z):** Same UNKNOWN — wal_bytes=0 but wal.log=379 bytes on disk.

**File inventory (single capture, ~22:12Z):**

| File | Size | Timestamp | Delta from pass 17 (21:52Z) | Notes |
|------|------|-----------|-------------------|-------|
| `persistence/state.snapshot` | 569 bytes | 18:08 EDT | Same size, new timestamp | Rotated at epoch 400 (was 360) |
| `persistence/wal.log` | 379 bytes | 18:08 EDT | Same size, new timestamp | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | 18:03 EDT | Same size, new timestamp | Prior rotation's WAL backup |

---

## Build Commit & Binary Freshness

**OBSERVED:** `71aa16b-dirty` on both nodes (unchanged since pass 1).
**EXPECTED (VERIFIED-BEHAVIOR.md):** Should match git HEAD (`aa62d12`).
**DEVIATION:** **Persistent.** 8 commits behind. Docs-only changes — no wire-format, codec, or protocol changes. Not a functional concern.

---

## Log Health

**OBSERVED:** No ERROR or unexpected WARN lines on either node from filtered scan.

**morning-api:** 46 total WARN/ERROR lines (was 43 at pass 17; +3 from periodic Kademlia `Failed to trigger bootstrap` at ~5-min cadence, expected over 20 min).

**witness (/tmp/lw.log):** 122 total WARN/ERROR lines (unchanged from pass 17 — all from early epoch-rejection lines, no new ones).

**Filtered health scan** (`grep -vE 'skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|Failed to trigger bootstrap|InsufficientPeers|Failed to gossip genesis|Failed to publish block'`) — **zero hits** on both nodes. All WARN/ERROR lines belong to known-benign categories.

**Sweep/evict/zombie activity:** None found in either log across the entire run. `grep -iE 'sweep|evict|zombie' /tmp/m-ap.log /tmp/lw.log` = 0 hits.

---

## Metrics (Last 5 Tick Lines)

### morning-api
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
(all 5 lines identical, timestamps 22:04:56–22:05:36Z)
```
**All clean:** zero fetches, zero queues, silence=3s. Unchanged from pass 17.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
(all 5 lines identical, timestamps 22:04:53–22:05:33Z)
```
**All clean:** zero fetches, zero queues, silence=6s. Unchanged from pass 17.

---

## Recent Epoch Activity (Last 5 Lines Each)

**morning-api:**
```
Epoch complete epoch=400 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=401 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=402 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=403 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=404 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at 1.02.

**local-witness:**
```
Epoch complete epoch=395 balance_before=0 balance_after=0 ratio=1.28
Epoch complete epoch=396 balance_before=0 balance_after=0 ratio=1.28
Epoch complete epoch=397 balance_before=0 balance_after=0 ratio=1.28
Epoch complete epoch=398 balance_before=0 balance_after=0 ratio=1.28
Epoch complete epoch=399 balance_before=0 balance_after=0 ratio=1.28
```
Ratio: 1.30→1.28 continuing expected asymptotic decline (pass 17: 1.30, pass 14: 1.31).

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 8 commits behind HEAD `aa62d12`) | Low — docs-only drift, no wire-format change | **Persistent** since pass 1 |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint not wired | **Persistent** since pass 1 |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) — causes repeated validation failures | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 |

**Deviations resolved since pass 17:** None.
**New deviations since pass 18:** None.

---

## Delta from Pass 17 (21:52Z → 22:12Z)

| Metric | Pass 17 (~17:52 EDT) | Pass 18 (~18:12 EDT) | Delta |
|--------|---------------------|----------------------|-------|
| Uptime (morning-api) | 11013s | 12186s | +1173s (~19.6 min) |
| Uptime (witness) | 11038s | 12182s | +1144s (~19.1 min) |
| Epoch (morning-api socket) | 369 | 408 | +39 |
| Epoch (witness socket) | 369 | 408 | +39 |
| Heartbeats (morning-api) | 1100 | 1217 | +117 |
| Heartbeats (witness) | 1104 | 1219 | +115 |
| Silence (morning-api) | 5s | 3s | -2s (normal variation) |
| Silence (witness) | 8s | 3s | -5s (normal variation) |
| Thickness | 997.05 | 996.74 | -0.31 (expected decay) |
| Balance | 20 | 20 | 0 |
| Nonce | 120 | 120 | 0 |
| Snapshot epoch (both) | 360 | 400 | +40 (4 rotations) |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| WARN count (morning-api) | 43 | 46 | +3 (Kademlia ticks) |
| WARN count (witness) | 122 | 122 | 0 |
| Snapshot size (morning-api) | 894 bytes | 895 bytes | +1 (negligible) |
| Snapshot size (witness) | 569 bytes | 569 bytes | 0 |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from all prior passes). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes.

2. **snapshot size (morning-api: 895 bytes, witness: 569 bytes).** Witness state.snapshot is 326 bytes smaller than morning-api. Expected (witness has different peer table state — zero balance, 2 nonces vs morning-api's 120 nonce, 4980 peer balance), but byte composition unconfirmed without deserialization.

---

## Raw Capture Bundle

Single-capture queries from ~22:11–22:12Z:

```
// === GetNodeInfo (morning-api, ~22:11:44Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":12186,"build_commit":"71aa16b-dirty","thickness":996.7401542749736}

// === GetPeers (morning-api, ~22:11:44Z) ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":1217,"silence_secs":3,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (morning-api, 22:12:21Z) — three-way: endpoint=408, grep=408, last line=408 ===
{"type":"EpochState","epoch":408,"ratio":1.0191091851957093,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (morning-api, ~22:11:44Z) ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetPersistenceState (morning-api, ~22:11:44Z) ===
{"type":"PersistenceState","last_snapshot_epoch":400,"wal_bytes":0,"wal_entries":0}

// === GetHeight (morning-api, ~22:11:44Z) ===
{"type":"Height","height":1}

// === GetNodeInfo (local-witness, ~22:11:56Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":12182,"build_commit":"71aa16b-dirty"}

// === GetPeers (local-witness, ~22:11:56Z) ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":1219,"silence_secs":3,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (local-witness, 22:12:21Z) — three-way: endpoint=408, grep=408, last line=408 ===
{"type":"EpochState","epoch":408,"ratio":1.2688108888697303,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (local-witness, ~22:11:56Z) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// === GetPersistenceState (local-witness, ~22:11:56Z) ===
{"type":"PersistenceState","last_snapshot_epoch":400,"wal_bytes":0,"wal_entries":0}
```

---

## Bottom Line

**No new deviations. All three persistent anomalies (stale binary, wal_bytes endpoint, witness balance blindness) unchanged since pass 1. Mesh has been running healthy for ~7.3 hours. Both nodes at epoch 408 in perfect lockstep — first three-way PASS on both nodes simultaneously. 4 snapshot rotations completed (epoch 360→400). No sweep/evict/zombie activity recorded. Zero queues, zero fetches across all metrics ticks.**

**Next check:** No threshold violations. System in stable steady state.
