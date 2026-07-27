# Observer Evidence Record — 2026-07-27 (Pass 16)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-27T21:44Z–21:45Z (~17:44 EDT)
**Machine:** z4-workstation (dale-joseph-hp-z4-g4-workstation, Boynton Beach FL)
**Session type:** Sixteenth observation pass. Same processes since 14:48 EDT (~7.0h runtime).

**Summary:** All-clear continuation. ~8 min since pass 15 (21:37Z). Epochs 338→355 (+17) on morning-api. Snapshot rotated at epoch 350. Balance stable at 20. Thickness decaying (997.31→997.18). Zero queues, zero fetches. No sweep/evict/zombie activity. Both nodes in lockstep (epoch 354–355, expected racing by ~1 at tick boundary). No new deviations.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-hp-z4-g4-workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs as pass 15. Both sockets responding. No stale survivor processes detected. Logs: `/tmp/m-ap.log` (morning-api) and `/tmp/lw.log` (witness).

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | MESH.md identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 10538 (~2.9h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `aa62d12` | **Persistent DEVIATION.** 8 commits behind. Docs-only changes — no binary rebuild since session start. |
| thickness | 997.18 | ~1000, slowly decaying | None (pass 15: 997.31; Δ = -0.13) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | MESH.md identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 10543 (~2.9h) | — | None |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=1052, silence_secs=4, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=1055, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 15:** Heartbeats +48 (morning-api: 1004→1052) and +49 (witness: 1006→1055). Silence: morning-api 2s→4s, witness 6s→3s (normal variation within ~10s metrics tick). Queue depth still 0 on both.

---

## Epoch State

### morning-api (single capture, 21:45:26Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 355 (socket, grep, log all agree) | Cycling ~30s cadence. +17 since pass 15 (338→355). | None |
| ratio | 1.0199 | ~1.01–1.02 steady state (pass 15: 1.0189) | None |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (21:45:26Z):**
- Socket epoch: 355
- `grep -c` count: 355
- Last log line: epoch=355

**PASS.** All three agree.

### local-witness (single capture, ~21:45:13Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 354 (socket, grep, log all agree) | Same cadence, matching morning-api. +16 since pass 15 (338→354). | None (off by 1 at tick boundary) |
| ratio | 1.31 | Declining (pass 15: 1.326; Δ ≈ -0.016). Expected asymptotic decay toward 1.0. | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (21:45:13Z):**
- Socket epoch: 354
- `grep -c` count: 354
- Last log line: epoch=354

**PASS.** All three agree.

**Epoch synchronization:** morning-api at 355, witness at 354 when captured ~13s apart. Expected racing by ~1 at tick boundary (< 30s epoch cadence). No lockstep deviation.

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
**OBSERVED:** last_snapshot_epoch=350, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every 10 epochs. WAL drained after rotation.
**DEVIATION:** None. Snapshot at epoch 350 (was at 330 in pass 15; +20 epochs = 2 rotations).

**Byte-equality check:** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes on disk.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes.

**File inventory (single capture, ~21:44Z):**

| File | Size | Timestamp | Delta from pass 15 (21:37Z) | Notes |
|------|------|-----------|-------------------|-------|
| `persistence/state.snapshot` | 895 bytes | 17:42 EDT | Same size, new timestamp | Rotated at epoch 350 (was 340) |
| `persistence/wal.log` | 379 bytes | 17:42 EDT | Same size, new timestamp | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | 17:37 EDT | Same size, new timestamp | Prior rotation's WAL backup |

### local-witness
**OBSERVED:** last_snapshot_epoch=350, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** None.

**Byte-equality check:** Same UNKNOWN — wal_bytes=0 but wal.log=379 bytes on disk.

**File inventory (single capture, ~21:44Z):**

| File | Size | Timestamp | Delta from pass 15 (21:37Z) | Notes |
|------|------|-----------|-------------------|-------|
| `persistence/state.snapshot` | 569 bytes | 17:43 EDT | Same size, new timestamp | Rotated at epoch 350 (was 340) |
| `persistence/wal.log` | 379 bytes | 17:43 EDT | Same size, new timestamp | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | 17:38 EDT | Same size, new timestamp | Prior rotation's WAL backup |

---

## Build Commit & Binary Freshness

**OBSERVED:** `71aa16b-dirty` on both nodes (unchanged since pass 1).
**EXPECTED (VERIFIED-BEHAVIOR.md):** Should match git HEAD (`aa62d12`).
**DEVIATION:** **Persistent.** 8 commits behind. Docs-only changes — no wire-format, codec, or protocol changes. Not a functional concern.

---

## Log Health

**OBSERVED:** No ERROR or unexpected WARN lines on either node.

**morning-api:** 42 total WARN/ERROR lines (was 40 at pass 15; +2 from periodic Kademlia `Failed to trigger bootstrap` at 5-min cadence).

**witness:** 122 total WARN/ERROR lines (unchanged from pass 15 — all from early epoch-rejection lines, no new ones).

**Filtered health scan** (`grep -vE 'skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|Failed to trigger bootstrap|InsufficientPeers|Failed to gossip genesis|Failed to publish block'`) — **zero hits** on both nodes. All WARN/ERROR lines belong to known-benign categories.

**Sweep/evict/zombie activity:** None found in either log across the entire run.

---

## Metrics (Last 10+ Tick Lines)

### morning-api
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
(all 15 lines identical)
```
**All clean:** zero fetches, zero queues, silence=3s. Unchanged from pass 15.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
(all 5 lines identical)
```
**All clean:** zero fetches, zero queues, silence=6s. Unchanged from pass 15.

---

## Recent Epoch Activity (Last 5 Lines Each)

**morning-api:**
```
epoch=351 balance_before=20 balance_after=20 ratio=1.02
epoch=352 balance_before=20 balance_after=20 ratio=1.02
epoch=353 balance_before=20 balance_after=20 ratio=1.02
epoch=354 balance_before=20 balance_after=20 ratio=1.02
epoch=355 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at 1.02.

**local-witness:**
```
epoch=350 balance_before=0 balance_after=0 ratio=1.31
epoch=351 balance_before=0 balance_after=0 ratio=1.31
epoch=352 balance_before=0 balance_after=0 ratio=1.31
epoch=353 balance_before=0 balance_after=0 ratio=1.31
epoch=354 balance_before=0 balance_after=0 ratio=1.31
```
Ratio: 1.326→1.31 (continuing expected asymptotic decline; pass 15 was 1.33).

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 8 commits behind HEAD `aa62d12`) | Low — docs-only drift, no wire-format change | **Persistent** since pass 1 |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint not wired | **Persistent** since pass 1 |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) — causes repeated validation failures | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 |

**Deviations resolved since pass 15:** None.
**New deviations since pass 16:** None.

---

## Delta from Pass 15 (21:37Z → 21:45Z)

| Metric | Pass 15 (~17:37 EDT) | Pass 16 (~17:44 EDT) | Delta |
|--------|---------------------|----------------------|-------|
| Uptime (morning-api) | 10056s | 10538s | +482s (~8.0 min) |
| Uptime (witness) | 10054s | 10543s | +489s (~8.1 min) |
| Epoch (morning-api, socket) | 338 | 355 | +17 |
| Epoch (witness, socket) | 338 | 354 | +16 |
| Heartbeats (morning-api) | 1004 | 1052 | +48 |
| Heartbeats (witness) | 1006 | 1055 | +49 |
| Silence (morning-api) | 2s | 4s | +2s (normal variation) |
| Silence (witness) | 6s | 3s | -3s (normal variation) |
| Thickness | 997.31 | 997.18 | -0.13 (expected decay) |
| Balance | 20 | 20 | 0 |
| Nonce | 120 | 120 | 0 |
| Snapshot epoch (both) | 330 | 350 | +20 (2 rotations) |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| WARN count (morning-api) | 40 | 42 | +2 (Kademlia ticks) |
| WARN count (witness) | 122 | 122 | 0 |
| Snapshot size (morning-api) | 895 bytes | 895 bytes | 0 |
| Snapshot size (witness) | 569 bytes | 569 bytes | 0 |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from all prior passes). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes.

2. **snapshot size (morning-api: 895 bytes, witness: 569 bytes).** Witness state.snapshot is 326 bytes smaller than morning-api. Expected (witness has different peer table state — zero balance, 2 nonces vs morning-api's 120 nonce, 4980 peer balance), but byte composition unconfirmed without deserialization.

---

## Raw Capture Bundle

Single-capture queries from ~21:44–21:45Z:

```
// === GetNodeInfo (morning-api, ~21:44:04Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":10538,"build_commit":"71aa16b-dirty","thickness":997.1800522159343}

// === GetPeers (morning-api, ~21:44:04Z) ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":1052,"silence_secs":4,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (morning-api, 21:45:26Z) — three-way: endpoint=355, grep=355, last line=355 ===
{"type":"EpochState","epoch":355,"ratio":1.0199364418874604,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (morning-api, ~21:44:04Z) ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetPersistenceState (morning-api, ~21:44:04Z) ===
{"type":"PersistenceState","last_snapshot_epoch":350,"wal_bytes":0,"wal_entries":0}

// === GetHeight (morning-api, ~21:44:04Z) ===
{"type":"Height","height":1}

// === Error response shape ===
{"type":"Error","message":"Invalid JSON: unknown variant `GetInvalidThing`, expected one of `GetHeight`, `GetBlock`, `GetCertificate`, `GetStats`, `GetPeers`, `GetEpochState`, `GetEconomicState`, `GetNodeInfo`, `GetPersistenceState`, `AgentSubmit`, `SubmitClaim`, `WitnessClaimService`, `GetClaimStatus`, `SubmitObjection`, `GetObjections`, `GetAllObjections` at line 1 column 25"}

// === GetNodeInfo (local-witness, ~21:44:27Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":10543,"build_commit":"71aa16b-dirty"}

// === GetPeers (local-witness, ~21:44:27Z) ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":1055,"silence_secs":3,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (local-witness, 21:45:13Z) — three-way: endpoint=354, grep=354, last line=354 ===
{"type":"EpochState","epoch":354,"ratio":1.3127499250224932,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (local-witness, ~21:44:27Z) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// === GetPersistenceState (local-witness, ~21:44:27Z) ===
{"type":"PersistenceState","last_snapshot_epoch":350,"wal_bytes":0,"wal_entries":0}
```

---

## Bottom Line

**No new deviations. All three persistent anomalies (stale binary, wal_bytes endpoint, witness balance blindness) unchanged since pass 1. Mesh has been running healthy for ~7.0 hours. morning-api at epoch 355, witness at epoch 354 (expected ~1-off at tick boundary). Both three-way PASS. Two snapshot rotations completed (epoch 340, 350). No sweep/evict/zombie activity recorded. Zero queues, zero fetches across all metrics ticks.**

**Next check:** No threshold violations. System in stable steady state.
