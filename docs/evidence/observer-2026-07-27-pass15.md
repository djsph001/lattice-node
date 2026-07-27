# Observer Evidence Record — 2026-07-27 (Pass 15)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-27T21:37:25Z (~17:37 EDT)
**Machine:** z4-workstation (dale-joseph-hp-z4-g4-workstation, Boynton Beach FL)
**Session type:** Fifteenth observation pass. Same processes since 14:48 EDT (~6.8h runtime).

**Summary:** All-clear continuation. ~8 min since pass 14 (21:29Z). Epochs 322→338 (+16) on both nodes. Snapshot rotated at epoch 330. Balance stable at 20. Thickness decaying (997.44→997.31). Zero queues, zero fetches. No sweep/evict/zombie activity. Both nodes in lockstep at epoch 338. No new deviations.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-hp-z4-g4-workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs as pass 14. Both sockets responding (morning-api at /tmp/m-ap/lattice.sock, witness at /tmp/local-witness/lattice.sock). No stale survivor processes detected. Logs: `/tmp/m-ap.log` and `/tmp/lw.log`.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | MESH.md identity (not present; VERIFIED-BEHAVIOR.md) | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 10056 (~2.8h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `aa62d12` | **Persistent DEVIATION.** 8 commits behind. Docs-only changes — no binary rebuild since session start. |
| thickness | 997.31 | ~1000, slowly decaying | None (pass 14: 997.44; Δ = -0.13) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | VERIFIED-BEHAVIOR.md identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 10054 (~2.8h) | — | None |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=1004, silence_secs=2, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=1006, silence_secs=6, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 14:** Heartbeats +48 (morning-api: 956→1004) and +45 (witness: 961→1006). Silence: morning-api dropped from 7s→2s, witness rose from 0s→6s (normal variation within ~10s metrics tick). Queue depth still 0 on both.

---

## Epoch State

### morning-api (single capture, 21:36:56Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 338 (socket, grep, log all agree) | Cycling ~30s cadence. +16 since pass 14 (322→338). | None |
| ratio | 1.0189 | ~1.01-1.02 steady state (pass 14: 1.0199) | None |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (21:36:56Z):**
- Socket epoch: 338
- `grep -c` count: 338
- Last log line: epoch=338

**PASS.** All three agree.

### local-witness (single capture, 21:37:13Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 338 (socket, grep, log all agree) | Same cadence, matching morning-api. +16 since pass 14 (322→338). | None |
| ratio | 1.326 | Declining (pass 14: 1.34; Δ = -0.014). Expected asymptotic decay toward 1.0. | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (21:37:13Z):**
- Socket epoch: 338
- `grep -c` count: 338
- Last log line: epoch=338

**PASS.** All three agree.

**Epoch synchronization:** Both nodes at epoch 338. Perfect lockstep.

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
**OBSERVED:** last_snapshot_epoch=330, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every 10 epochs. WAL drained after rotation.
**DEVIATION:** None. Snapshot at epoch 330 (was at 320 in pass 14; +10 epochs = 1 rotation).

**Byte-equality check:** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes on disk.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes.

**File inventory (single capture, ~21:37Z):**

| File | Size | Timestamp | Delta from pass 14 (21:29Z) | Notes |
|------|------|-----------|-------------------|-------|
| `persistence/state.snapshot` | 895 bytes | 17:32 EDT | Same size, new timestamp | Rotated at epoch 330 |
| `persistence/wal.log` | 379 bytes | 17:32 EDT | Same size, new timestamp | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | 17:27 EDT | Same size, new timestamp | Prior rotation's WAL backup |

### local-witness
**OBSERVED:** last_snapshot_epoch=330, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** None.

**Byte-equality check:** Same UNKNOWN — wal_bytes=0 but wal.log=379 bytes on disk.

**File inventory (single capture, ~21:37Z):**

| File | Size | Timestamp | Delta from pass 14 (21:29Z) | Notes |
|------|------|-----------|-------------------|-------|
| `persistence/state.snapshot` | 569 bytes | 17:33 EDT | Same size, new timestamp | Rotated at epoch 330 |
| `persistence/wal.log` | 379 bytes | 17:33 EDT | Same size, new timestamp | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | 17:28 EDT | Same size, new timestamp | Prior rotation's WAL backup |

---

## Build Commit & Binary Freshness

**OBSERVED:** `71aa16b-dirty` on both nodes (unchanged since pass 1).
**EXPECTED (VERIFIED-BEHAVIOR.md):** Should match git HEAD (`aa62d12`).
**DEVIATION:** **Persistent.** 8 commits behind. Docs-only changes — no wire-format, codec, or protocol changes. Not a functional concern.

---

## Log Health

**OBSERVED:** No ERROR or unexpected WARN lines on either node.

**morning-api:** 40 total WARN/ERROR lines (was 39 at pass 14; +1 from periodic Kademlia `Failed to trigger bootstrap` — the expected 5-minute cadence).

**witness:** 122 total WARN/ERROR lines (unchanged from pass 14 — all from early epoch-rejection lines, no new ones).

**Filtered health scan** `grep -vE 'skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|Failed to trigger bootstrap|InsufficientPeers|Failed to gossip genesis|Failed to publish block'` — **zero hits** on both nodes. All WARN/ERROR lines belong to known-benign categories.

**Sweep/evict/zombie activity:** None found in either log across the entire run.

---

## Metrics (Last 10 Tick Lines)

### morning-api
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
(all 10 lines identical)
```
**All clean:** zero fetches, zero queues, silence=3s. Unchanged from pass 14.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
(all 10 lines identical)
```
**All clean:** zero fetches, zero queues, silence=6s. Unchanged from pass 14.

---

## Recent Epoch Activity (Last 5 Lines Each)

**morning-api:**
```
epoch=334 balance_before=20 balance_after=20 ratio=1.02
epoch=335 balance_before=20 balance_after=20 ratio=1.02
epoch=336 balance_before=20 balance_after=20 ratio=1.02
epoch=337 balance_before=20 balance_after=20 ratio=1.02
epoch=338 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at 1.02.

**local-witness:**
```
epoch=334 balance_before=0 balance_after=0 ratio=1.33
epoch=335 balance_before=0 balance_after=0 ratio=1.33
epoch=336 balance_before=0 balance_after=0 ratio=1.33
epoch=337 balance_before=0 balance_after=0 ratio=1.33
epoch=338 balance_before=0 balance_after=0 ratio=1.33
```
Ratio: 1.34→1.33 (continuing expected asymptotic decline).

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 8 commits behind HEAD `aa62d12`) | Low — docs-only drift, no wire-format change | **Persistent** since pass 1 |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint not wired | **Persistent** since pass 1 |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) — causes repeated validation failures | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 |

**Deviations resolved since pass 14:** None.
**New deviations since pass 14:** None.

---

## Delta from Pass 14 (21:29Z → 21:37Z)

| Metric | Pass 14 (~17:29 EDT) | Pass 15 (~17:37 EDT) | Delta |
|--------|---------------------|----------------------|-------|
| Uptime (morning-api) | 9581s | 10056s | +475s (~7.9 min) |
| Uptime (witness) | 9600s | 10054s | +454s (~7.6 min) |
| Epoch (morning-api, socket) | 322 | 338 | +16 |
| Epoch (witness, socket) | 322 | 338 | +16 |
| Heartbeats (morning-api) | 956 | 1004 | +48 |
| Heartbeats (witness) | 961 | 1006 | +45 |
| Silence (morning-api) | 7s | 2s | -5s (normal variation) |
| Silence (witness) | 0s | 6s | +6s (normal variation) |
| Thickness | 997.44 | 997.31 | -0.13 (expected decay) |
| Balance | 20 | 20 | 0 |
| Nonce | 120 | 120 | 0 |
| Snapshot epoch (both) | 320 | 330 | +10 (1 rotation) |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| WARN count (morning-api) | 39 | 40 | +1 (Kademlia tick) |
| WARN count (witness) | 122 | 122 | 0 |
| Snapshot size (morning-api) | 895 bytes | 895 bytes | 0 |
| Snapshot size (witness) | 569 bytes | 569 bytes | 0 |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from all prior passes). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes.

2. **snapshot size (morning-api: 895 bytes, witness: 569 bytes).** Witness state.snapshot is 326 bytes smaller than morning-api. Expected (witness has different peer table state — zero balance, 2 nonces vs morning-api's 120 nonce, 4980 peer balance), but byte composition unconfirmed without deserialization.

---

## Raw Capture Bundle

Single-capture queries from ~21:37Z:

```
// === GetNodeInfo (morning-api, ~21:36:03Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":10056,"build_commit":"71aa16b-dirty","thickness":997.3080589762023}

// === GetPeers (morning-api, ~21:36:03Z) ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":1004,"silence_secs":2,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (morning-api, 21:36:56Z) — three-way: endpoint=338, grep=338, last line=338 ===
{"type":"EpochState","epoch":338,"ratio":1.0189239674985644,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (morning-api, ~21:36:03Z) ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetPersistenceState (morning-api, ~21:36:03Z) ===
{"type":"PersistenceState","last_snapshot_epoch":330,"wal_bytes":0,"wal_entries":0}

// === GetHeight (morning-api, ~21:36:03Z) ===
{"type":"Height","height":1}

// === Error response shape ===
{"type":"Error","message":"Invalid JSON: unknown variant `GetInvalidThing`, expected one of `GetHeight`, `GetBlock`, `GetCertificate`, `GetStats`, `GetPeers`, `GetEpochState`, `GetEconomicState`, `GetNodeInfo`, `GetPersistenceState`, `AgentSubmit`, `SubmitClaim`, `WitnessClaimService`, `GetClaimStatus`, `SubmitObjection`, `GetObjections`, `GetAllObjections` at line 1 column 25"}

// === GetNodeInfo (local-witness, ~21:37:Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":10054,"build_commit":"71aa16b-dirty"}

// === GetPeers (local-witness, ~21:37:Z) ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":1006,"silence_secs":6,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (local-witness, 21:37:13Z) — three-way: endpoint=338, grep=338, last line=338 ===
{"type":"EpochState","epoch":338,"ratio":1.3260165035401916,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (local-witness, ~21:37:Z) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// === GetPersistenceState (local-witness, ~21:37:Z) ===
{"type":"PersistenceState","last_snapshot_epoch":330,"wal_bytes":0,"wal_entries":0}
```

---

## Bottom Line

**No new deviations. All three persistent anomalies (stale binary, wal_bytes endpoint, witness balance blindness) unchanged since pass 1. Mesh has been running healthy for ~6.8 hours. Both nodes at epoch 338 in perfect lockstep (three-way PASS on both). One snapshot rotation completed (epoch 330). No sweep/evict/zombie activity recorded. Zero queues, zero fetches across all metrics ticks.**

**Next check:** No threshold violations. The system is in stable steady state.
