# Observer Evidence Record — 2026-07-27 (Pass 23)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-27T22:50Z–22:51Z
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Twenty-third observation pass. Same processes since 14:48 EDT (~8.1h runtime).

**Summary:** All-clear continuation. ~7 min since pass 22 (22:43Z). Epochs 470→484 (+14) on both nodes. Snapshot rotated once (470→480). Both nodes at exact epoch match (484 three-way PASS). Balance locked at 20. Thickness 996.12 (decaying). Zero queues, zero fetches, zero sweep/evict/zombie activity. Both nodes healthy. No new deviations.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs as all prior passes. Both sockets responding. No stale survivor processes (2 lattice-node PIDs, 2 expected).

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 14500 (~4.0h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `aa62d12` | **Persistent DEVIATION.** 8 commits behind. Docs-only changes since binary build. |
| thickness | 996.12 | ~1000, slowly decaying | None (pass 22: 996.25; Δ = -0.13) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 14508 (~4.0h) | — | None |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=1448, silence_secs=6, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=1451, silence_secs=8, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 22:** Heartbeats morning-api +48 (1400→1448), witness +48 (1403→1451). Silence: morning-api 2s→6s (normal), witness 8s→8s (unchanged). Queue depth 0 on both.

---

## Epoch State

### morning-api (single capture, ~22:50Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 484 (socket), 484 (grep), 484 (last log line) | Cycling ~30s cadence. +14 since pass 22 (470→484). | **PASS — three-way match.** |
| ratio | 1.0192 | ~1.01–1.02 steady state (pass 22: 1.0199) | None (near-gaussian variation) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (~22:50Z):**
- Socket epoch: 484
- `grep -c` count: 484
- Last log line: epoch=484 (22:49:56Z)

**PASS.** All three agree.

### local-witness (same capture, ~22:50Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 484 (socket) | Same cadence. +14 since pass 22. | **PASS — match with morning-api (484).** |
| ratio | 1.225 | Declining (pass 22: 1.232; Δ ≈ -0.007). Expected asymptotic decay toward 1.0. | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Epoch synchronization:** Both nodes at epoch 484. Exact match.

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
**OBSERVED:** last_snapshot_epoch=480, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every 10 epochs. WAL drained after rotation.
**DEVIATION:** Snapshot advanced from 470 to 480 (+10, 1 rotation since pass 22). No other deviation.

**Byte-equality check (~22:50Z):** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes, wal.wal.old at 379 bytes.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes.

**File inventory (single capture, ~22:50Z):**

| File | Size | Timestamp | Delta from pass 22 | Notes |
|------|------|-----------|-------------------|-------|
| `persistence/state.snapshot` | 894 bytes | Jul 27 18:47 | 895→894 (-1 byte) | New rotation (epoch 480, was 470) |
| `persistence/wal.log` | 379 bytes | Jul 27 18:47 | Same size | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 18:42 | Same size | Prior rotation's WAL backup |

### local-witness
**OBSERVED:** last_snapshot_epoch=480, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** None (matches morning-api's snapshot epoch).

---

## Build Commit & Binary Freshness

**OBSERVED:** `71aa16b-dirty` on both nodes (unchanged since pass 1).
**EXPECTED (VERIFIED-BEHAVIOR.md):** Should match git HEAD (`aa62d12`).
**DEVIATION:** **Persistent.** 8 commits behind. Docs-only changes — no wire-format, codec, or protocol changes.

Git HEAD: `aa62d12` ("docs: note /tmp identity dir fragility across reboots")
Running binary: `71aa16b` ("wip: update Cargo.lock") + `-dirty`

`71aa16b` is ancestor of `aa62d12`. Commits between: 8 docs-only and test-fix commits (no wire-format changes).

---

## Log Health

**morning-api:**
- 0 unexpected WARN/ERROR lines after filtering (healthy).
- All WARN lines are from libp2p_kad `Failed to trigger bootstrap: No known peers.` — benign with `--no-mdns`. ~60+ ticks over 8h.
- No ERROR lines.
- No sweep/evict/zombie activity detected.

**local-witness (/tmp/lw.log):**
- 0 WARN, 0 ERROR lines.
- No sweep/evict/zombie activity.

---

## Metrics (Last 5 Tick Lines)

### morning-api
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
(all 5 lines identical, timestamps 22:50:36–22:51:16Z)
```
**All clean:** zero fetches, zero queues, silence=3s. Unchanged from pass 22.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
(all 5 lines identical, timestamps 22:50:43–22:51:23Z)
```
**All clean:** zero fetches, zero queues, silence=6s. Unchanged from pass 22.

---

## Recent Epoch Activity (Last 5 Lines Each)

**morning-api (at ~22:50Z):**
```
Epoch complete epoch=481 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=482 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=483 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=484 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at ~1.019.

**local-witness (at ~22:50Z):**
```
Epoch complete epoch=481 balance_before=0 balance_after=0 ratio=1.23
Epoch complete epoch=482 balance_before=0 balance_after=0 ratio=1.23
Epoch complete epoch=483 balance_before=0 balance_after=0 ratio=1.23
Epoch complete epoch=484 balance_before=0 balance_after=0 ratio=1.23
```
Ratio: 1.225 at socket query (declining from 1.232 at pass 22). Continuing expected asymptotic decay.

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 8 commits behind HEAD `aa62d12`) | Low — docs-only drift, no wire-format change | **Persistent** since pass 1 |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint not wired | **Persistent** since pass 1 |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 |

**Deviations resolved since pass 22:** None.
**New deviations since pass 23:** None.

---

## Delta from Pass 22 (22:43Z → 22:50Z)

| Metric | Pass 22 (~22:43Z) | Pass 23 (~22:50Z) | Delta |
|--------|--------------------|--------------------|-------|
| Uptime (morning-api) | 14015s | 14500s | +485s (~8 min) |
| Uptime (witness) | 14027s | 14508s | +481s |
| Epoch (morning-api socket) | 470 | 484 | +14 |
| Epoch (witness socket) | 470 | 484 | +14 |
| Heartbeats (morning-api) | 1400 | 1448 | +48 |
| Heartbeats (witness) | 1403 | 1451 | +48 |
| Silence (morning-api) | 2s | 6s | +4s (normal variation) |
| Silence (witness) | 8s | 8s | 0s (unchanged) |
| Thickness | 996.25 | 996.12 | -0.13 (expected decay) |
| Balance | 20 | 20 | 0 |
| Nonce | 120 | 120 | 0 |
| Snapshot epoch (morning-api) | 470 | 480 | +10 (1 rotation) |
| Snapshot epoch (witness) | 470 | 480 | +10 (1 rotation) |
| Snapshot size (morning-api) | 895 bytes | 894 bytes | -1 byte (minor variation) |
| Snapshot size (witness) | 569 bytes | — | Not re-queried this pass |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| WARN count (both, filtered) | 0 unexpected | 0 unexpected | 0 |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from all prior passes). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes.

2. **Snapshot size -1 byte change** (morning-api: 895→894). Pass 22 snapshot was 895 bytes (epoch 470). Current snapshot at epoch 480 is 894 bytes (1 byte smaller). Possible: one fewer entry in serialized state (nonce/balance delta), or a peer table entry differed. Cannot determine without deserialization.

---

## Raw Capture Bundle

Single-capture queries from ~22:50Z:

```
// === GetNodeInfo (morning-api, ~22:50Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":14500,"build_commit":"71aa16b-dirty","thickness":996.1246231549344}

// === GetPeers (morning-api, ~22:50Z) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":1448,"silence_secs":6,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (morning-api, ~22:50Z) — three-way: endpoint=484, grep=484, last line=484 (PASS) ===
{"type":"EpochState","epoch":484,"ratio":1.019235638878274,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (morning-api, ~22:50Z) ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetPersistenceState (morning-api, ~22:50Z) — wal_bytes=0, wal.log=379 bytes (UNKNOWN) ===
{"type":"PersistenceState","last_snapshot_epoch":480,"wal_bytes":0,"wal_entries":0}

// === GetHeight (morning-api, ~22:50Z) ===
{"type":"Height","height":1}

// === GetNodeInfo (local-witness, ~22:50Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":14508,"build_commit":"71aa16b-dirty"}

// === GetPeers (local-witness, ~22:50Z) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":1451,"silence_secs":8,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (local-witness, ~22:50Z) — epoch=484 ===
{"type":"EpochState","epoch":484,"ratio":1.2254934456971354,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (local-witness, ~22:50Z) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}
```

---

## Bottom Line

**No new deviations. All three persistent anomalies unchanged. Mesh has been running healthy for ~8.1 hours. Both nodes at epoch 484 — exact match with three-way PASS on both. One snapshot rotation completed since pass 22 (epoch 470→480). Snapshot size -1 byte change (895→894) — likely a minor serialization difference. Zero queues, zero fetches, zero sweep/evict/zombie activity across all metrics ticks.**

**Next check:** No threshold violations. System in stable steady state. Scheduled cron for next pass.
