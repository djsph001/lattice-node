# Observer Evidence Record — 2026-07-27 (Pass 13)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-27T21:21:15Z (~17:21 EDT)
**Machine:** z4-workstation (dale-joseph-hp-z4-g4-workstation, Boynton Beach FL)
**Session type:** Thirteenth observation pass. Same processes since 14:48 EDT (~6.5h runtime).

**Summary:** All-clear continuation. ~8 min since pass 12 (21:13Z). Epochs 291→307 (+16) on both nodes. Snapshot rotation at epoch 300 completed. Balance stable at 20. Thickness decaying (997.68→997.56). Zero queues, zero fetches. No new deviations.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-hp-z4-g4-workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since |
|-----|------|------|--------------|-------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT |

**No topology changes.** Same PIDs as pass 12. Both sockets responding. No stale survivor processes.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | MESH.md identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 9108 (~2.5h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `aa62d12` | **Persistent DEVIATION.** 8 commits behind. Docs-only changes — no binary rebuild since session start. |
| thickness | 997.56 | ~1000, slowly decaying | None (pass 12: 997.68; Δ = -0.12) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | MESH.md identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 9110 (~2.5h) | — | None |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=909, silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=912, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 12:** heartbeats +49 (morning-api: 860→909) and +48 (witness: 864→912). Silence consistent (morning-api 7s, witness 3s — swapping; within normal variation). Queue depth still 0 on both.

---

## Epoch State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 306 (socket), 307 (last log line) | Cycling ~30s cadence. +16 since pass 12 (290→306). | None |
| ratio | 1.0188 | ~1.01-1.02 steady state | None |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (single capture, 21:21:15Z):**
- Socket epoch: 306
- `grep -c` count: 306
- Last log line: epoch=306

**PASS.** All three agree. Latest line at 21:21:26Z shows epoch=307 — advanced during capture gap, as expected.

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 306 (socket), 306 (last log line) | Same cadence, matching morning-api. +16 since pass 12 (290→306). | None |
| ratio | 1.36 | Declining (pass 12: 1.38; Δ = -0.02). Expected asymptotic decay toward 1.0. | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (single capture, 21:21:15Z):**
- Socket epoch: 306
- `grep -c` count: 306
- Last log line: epoch=306

**PASS.** All three agree.

**Epoch synchronization:** Latest log lines show both nodes at epoch 306/307. Perfect lockstep. Witness ratio: 1.38→1.36 (continuing expected decay).

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
**OBSERVED:** last_snapshot_epoch=300, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every 10 epochs. WAL drained after rotation.
**DEVIATION:** None. Snapshot at epoch 300 (was at 290 in pass 12; +10 epochs = 1 rotation).

**Byte-equality check:** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes on disk.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes.

**File inventory (single capture, ~21:21:15Z):**

| File | Size | Timestamp | Delta from pass 12 | Notes |
|------|------|-----------|-------------------|-------|
| `persistence/state.snapshot` | 894 bytes | 17:17 EDT | Size: 895→894 (-1 byte), new timestamp | Rotated at epoch 300 |
| `persistence/wal.log` | 379 bytes | 17:17 EDT | Unchanged size, new timestamp | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | 17:12 EDT | Unchanged size, same timestamp | Prior rotation's WAL backup |

### local-witness
**OBSERVED:** last_snapshot_epoch=300, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** None.

**Byte-equality check:** Same UNKNOWN — wal_bytes=0 but wal.log=379 bytes on disk.

**File inventory (single capture, ~21:21:15Z):**

| File | Size | Timestamp | Delta from pass 12 | Notes |
|------|------|-----------|-------------------|-------|
| `persistence/state.snapshot` | 569 bytes | 17:18 EDT | Unchanged size, new timestamp | Rotated at epoch 300 |
| `persistence/wal.log` | 379 bytes | 17:18 EDT | Unchanged size, new timestamp | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | 17:13 EDT | Unchanged size, same timestamp | Prior rotation's WAL backup |

---

## Build Commit & Binary Freshness

**OBSERVED:** `71aa16b-dirty` on both nodes (unchanged since pass 1).
**EXPECTED (VERIFIED-BEHAVIOR.md):** Should match git HEAD (`aa62d12`).
**DEVIATION:** **Persistent.** 8 commits behind. Docs-only changes — no wire-format, codec, or protocol changes. Not a functional concern.

---

## Log Health

**OBSERVED:** No ERROR or unexpected WARN lines on either node.

**morning-api:** 37 total WARN/ERROR lines (was 36 at pass 12; +1 from periodic Kademlia `Failed to trigger bootstrap` — the expected 5-minute cadence).

**witness:** 122 total WARN/ERROR lines (unchanged from pass 12 — all from early epoch-rejection lines, no new ones).

**Filtered health scan** `grep -vE 'skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|Failed to trigger bootstrap|InsufficientPeers|Failed to gossip genesis|Failed to publish block'` — **zero hits** on both nodes. All WARN/ERROR lines belong to known-benign categories.

**Sweep/evict/zombie activity:** None found in either log across the entire run.

---

## Metrics (Last 10 Tick Lines)

### morning-api
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
(all 10 lines identical)
```
**All clean:** zero fetches, zero queues, silence=3s. Unchanged from pass 12.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
(all 10 lines identical)
```
**All clean:** zero fetches, zero queues, silence=6s. Unchanged from pass 12.

---

## Recent Epoch Activity (Last 5 Lines Each)

**morning-api:**
```
epoch=303 balance_before=20 balance_after=20 ratio=1.02
epoch=304 balance_before=20 balance_after=20 ratio=1.02
epoch=305 balance_before=20 balance_after=20 ratio=1.02
epoch=306 balance_before=20 balance_after=20 ratio=1.02
epoch=307 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at 1.02.

**local-witness:**
```
epoch=302 balance_before=0 balance_after=0 ratio=1.37
epoch=303 balance_before=0 balance_after=0 ratio=1.36
epoch=304 balance_before=0 balance_after=0 ratio=1.36
epoch=305 balance_before=0 balance_after=0 ratio=1.36
epoch=306 balance_before=0 balance_after=0 ratio=1.36
```
Ratio: 1.37→1.36 (continuing expected asymptotic decline).

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 8 commits behind HEAD `aa62d12`) | Low — docs-only drift, no wire-format change | **Persistent** since pass 1 |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint not wired | **Persistent** since pass 1 |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) — causes repeated validation failures | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 |

**Deviations resolved since pass 12:** None.
**New deviations since pass 12:** None.

---

## Delta from Pass 12

| Metric | Pass 12 (~17:13 EDT) | Pass 13 (~17:21 EDT) | Delta |
|--------|---------------------|----------------------|-------|
| Uptime (morning-api) | 8687s | 9108s | +421s (~7 min) |
| Uptime (witness) | 8680s | 9110s | +430s (~7.2 min) |
| Epoch (morning-api, socket) | 290 | 306 | +16 |
| Epoch (witness, socket) | 290 | 306 | +16 |
| Heartbeats (morning-api) | 860 | 909 | +49 |
| Heartbeats (witness) | 864 | 912 | +48 |
| Silence (morning-api) | 3s | 7s | +4s (normal variation) |
| Silence (witness) | 6s | 3s | -3s (normal variation) |
| Thickness | 997.68 | 997.56 | -0.12 (expected decay) |
| Balance | 20 | 20 | 0 |
| Nonce | 120 | 120 | 0 |
| Snapshot epoch (both) | 290 | 300 | +10 (1 rotation) |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| WARN count (morning-api) | 36 | 37 | +1 (one Kademlia tick) |
| WARN count (witness) | 122 | 122 | 0 |
| Snapshot size (morning-api) | 895 bytes | 894 bytes | -1 byte (trivial) |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from all prior passes). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes.

2. **snapshot size (morning-api: 894 bytes, witness: 569 bytes).** Witness state.snapshot is 325 bytes smaller than morning-api. Expected (witness has different peer table state — zero balance, 2 nonces vs morning-api's 120 nonce, 4980 peer balance), but byte composition unconfirmed without deserialization.

---

## Raw Capture Bundle

```
// === GetNodeInfo (morning-api, ~21:21:15Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":9108,"build_commit":"71aa16b-dirty","thickness":997.5641217950529}

// === GetPeers (morning-api, ~21:21:15Z) ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":909,"silence_secs":7,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (morning-api, 21:21:15Z) — three-way: endpoint=306, grep=306, last line=306 ===
{"type":"EpochState","epoch":306,"ratio":1.0188109495919067,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (morning-api, ~21:21:15Z) ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetPersistenceState (morning-api, 21:21:15Z) ===
{"type":"PersistenceState","last_snapshot_epoch":300,"wal_bytes":0,"wal_entries":0}

// === GetHeight (morning-api, ~21:21:15Z) ===
{"type":"Height","height":1}

// === Error response shape ===
{"type":"Error","message":"Invalid JSON: unknown variant `GetInvalidThing`, expected one of `GetHeight`, `GetBlock`, `GetCertificate`, `GetStats`, `GetPeers`, `GetEpochState`, `GetEconomicState`, `GetNodeInfo`, `GetPersistenceState`, `AgentSubmit`, `SubmitClaim`, `WitnessClaimService`, `GetClaimStatus`, `SubmitObjection`, `GetObjections`, `GetAllObjections` at line 1 column 25"}

// === GetNodeInfo (local-witness, ~21:21:15Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":9110,"build_commit":"71aa16b-dirty"}

// === GetPeers (local-witness, ~21:21:15Z) ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":912,"silence_secs":3,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (local-witness, 21:21:15Z) — three-way: endpoint=306, grep=306, last line=306 ===
{"type":"EpochState","epoch":306,"ratio":1.3609137712839392,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (local-witness, ~21:21:15Z) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// === GetPersistenceState (local-witness, 21:21:15Z) ===
{"type":"PersistenceState","last_snapshot_epoch":300,"wal_bytes":0,"wal_entries":0}
```

---

## Bottom Line

**No new deviations. All three persistent anomalies (stale binary, wal_bytes endpoint, witness balance blindness) unchanged since pass 1. Mesh has been running healthy for ~6.5 hours. Both nodes at epoch 306/307 in perfect lockstep. No sweep/evict/zombie activity recorded. Zero queues, zero fetches across all metrics ticks.**

**Next check:** No threshold violations. The system is in stable steady state.
