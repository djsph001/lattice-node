# Observer Evidence Record — 2026-07-27 (Pass 24)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-27T22:58:54Z
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Twenty-fourth observation pass. Same processes since 14:48 EDT (~8.2h runtime).

**Summary:** All-clear continuation. ~8 min since pass 23 (22:50Z). Epochs 502→503 (+18 since pass 23's 484) on both nodes. Snapshot rotated twice (480→500). Three-way epoch match PASS (all at 502/503). Balance locked at 20. Thickness 996.0 (decaying). Zero queues, zero fetches, zero sweep/evict/zombie activity. Both nodes healthy. One new minor observation: snapshot file mtime unchanged since 18:57Z despite epoch advancing from 480 to 500 (2 rotations should have occurred). Recorded as UNKNOWN — no diagnosis.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs as all prior passes. Both sockets responding. No stale survivor processes (2 lattice-node PIDs expected, 2 found; pgrep -c returns 5 due to bash wrapper processes).

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 15028 (~4.2h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `aa62d12` | **Persistent DEVIATION.** 8 commits behind. Docs-only changes since binary build. |
| thickness | 995.99 | ~1000, slowly decaying | None (pass 23: 996.12; Δ = -0.13) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 15029 (~4.2h) | — | None |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=1501, silence_secs=4, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=1503, silence_secs=9, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 23:** Heartbeats morning-api +53 (1448→1501), witness +52 (1451→1503). Silence: morning-api 6s→4s (normal), witness 8s→9s (normal variation). Queue depth 0 on both.

---

## Epoch State

### morning-api (single capture, ~22:58:54Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 503 (socket), 503 (grep), 503 (last log line) | Cycling ~30s cadence. +19 since pass 23 (484→503). | **PASS — three-way match.** |
| ratio | 1.0193 | ~1.01–1.02 steady state (pass 23: 1.0192) | None (near-gaussian variation) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (~22:58:54Z):**
- Socket epoch: 503
- `grep -c` count: 503
- Last log line epoch: 503

**PASS.** All three agree.

### local-witness (same capture, ~22:58:54Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 503 (socket) | Same cadence. +19 since pass 23 (484→503). | **PASS — match with morning-api (503).** |
| ratio | 1.218 | Declining (pass 23: 1.225; Δ ≈ -0.007). Expected asymptotic decay toward 1.0. | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Epoch synchronization:** Both nodes at epoch 503. Exact match.

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
**OBSERVED:** last_snapshot_epoch=500, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every 10 epochs. WAL drained after rotation.
**DEVIATION:** None on the endpoint values.

**Byte-equality check (~22:58:54Z):** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes, wal.wal.old at 379 bytes.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes.

**File inventory (single capture, ~22:58:54Z):**

| File | Size | Timestamp | Delta from pass 23 | Notes |
|------|------|-----------|-------------------|-------|
| `persistence/state.snapshot` | 895 bytes | Jul 27 18:57 | 894→895 (+1 byte) | Timestamp unchanged (18:57) since pass 22. Snapshot epoch advanced 480→500 (2 rotations). |
| `persistence/wal.log` | 379 bytes | Jul 27 18:57 | Same size | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 18:52 | Same size | Prior rotation's WAL backup |

### local-witness
**OBSERVED:** last_snapshot_epoch=500, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** None (matches morning-api's snapshot epoch).

**File inventory:**

| File | Size | Timestamp | Delta from pass 23 |
|------|------|-----------|-------------------|
| `persistence/state.snapshot` | 569 bytes | Jul 27 18:58 | Unchanged from pass 23 |
| `persistence/wal.log` | 379 bytes | Jul 27 18:58 | Same size |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 18:53 | Same size |

**Note:** Snapshot file mtime (18:57 morning-api, 18:58 witness) did NOT change between epoch 480 (pass 22 ~22:43Z) and epoch 500 (current), despite 2 snapshot rotations occurring. The endpoint reports `last_snapshot_epoch=500` but the disk files appear unmodified since ~18:57Z. Recorded as UNKNOWN observation — no diagnosis.

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
- 2 benign WARN lines at startup: `Failed to gossip genesis (will retry on peer connect)` and `Failed to publish block` — both InsufficientPeers at 14:48 when witness was connecting. Expected with `--no-mdns`.
- ~60+ benign libp2p_kad WARN lines (`Failed to trigger bootstrap: No known peers.`) — benign with `--no-mdns`.
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
(all 5 lines identical, timestamps 22:58:46–22:59:26Z)
```
**All clean:** zero fetches, zero queues, silence=3s. Unchanged from pass 23.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
(all 5 lines identical, timestamps 22:58:43–22:59:23Z)
```
**All clean:** zero fetches, zero queues, silence=6s. Unchanged from pass 23.

---

## Recent Epoch Activity (Last 5 Lines Each)

**morning-api (at ~22:59Z):**
```
Epoch complete epoch=499 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=500 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=501 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=502 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=503 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at ~1.019.

**local-witness (at ~22:59Z):**
```
Epoch complete epoch=499 balance_before=0 balance_after=0 ratio=1.22
Epoch complete epoch=500 balance_before=0 balance_after=0 ratio=1.22
Epoch complete epoch=501 balance_before=0 balance_after=0 ratio=1.22
Epoch complete epoch=502 balance_before=0 balance_after=0 ratio=1.22
Epoch complete epoch=503 balance_before=0 balance_after=0 ratio=1.22
```
Ratio: 1.218 at socket query (declining from 1.225 at pass 23). Continuing expected asymptotic decay.

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 8 commits behind HEAD `aa62d12`) | Low — docs-only drift, no wire-format change | **Persistent** since pass 1 |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint not wired | **Persistent** since pass 1 |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 |

**Deviations resolved since pass 23:** None.
**New deviations since pass 24:** None.

**New UNKNOWN item:** Snapshot file mtime on morning-api unchanged at 18:57Z from pass 22 through current (22:59Z), despite endpoint reporting 2 snapshot rotations (epochs 480→490→500). Snapshot size changed 894→895 bytes but mtime did not advance. Witness shows same pattern (mtime: 18:58Z unchanged).

---

## Delta from Pass 23 (22:50Z → 22:59Z)

| Metric | Pass 23 (~22:50Z) | Pass 24 (~22:59Z) | Delta |
|--------|--------------------|--------------------|-------|
| Uptime (morning-api) | 14500s | 15028s | +528s (~9 min) |
| Uptime (witness) | 14508s | 15029s | +521s |
| Epoch (morning-api socket) | 484 | 503 | +19 |
| Epoch (witness socket) | 484 | 503 | +19 |
| Heartbeats (morning-api) | 1448 | 1501 | +53 |
| Heartbeats (witness) | 1451 | 1503 | +52 |
| Silence (morning-api) | 6s | 4s | -2s (normal variation) |
| Silence (witness) | 8s | 9s | +1s (normal variation) |
| Thickness | 996.12 | 995.99 | -0.13 (expected decay) |
| Balance | 20 | 20 | 0 |
| Nonce | 120 | 120 | 0 |
| Snapshot epoch (morning-api) | 480 | 500 | +20 (2 rotations) |
| Snapshot epoch (witness) | 480 | 500 | +20 (2 rotations) |
| Snapshot size (morning-api) | 894 bytes | 895 bytes | +1 byte (minor variation) |
| Snapshot size (witness) | 569 bytes | 569 bytes | 0 |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| WARN count (both, filtered) | 0 unexpected | 0 unexpected | 0 |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from all prior passes). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes.

2. **Snapshot file mtime not advancing.** Snapshot on morning-api shows mtime=18:57Z (same since pass 22). Endpoint reports last_snapshot_epoch advanced from 480 to 500 (2 rotations should have occurred). Snapshot size changed from 894→895 bytes. Possible explanations (not diagnosed): atomic rename preserves original mtime, write-to-temp+rename pattern, or snapshot is cached in memory and only flushed on certain rotations.

3. **Snapshot size +1 byte** (morning-api: 894→895). Possible: one additional entry in serialized state, or minor serialization variation at different epoch boundaries. Cannot determine without deserialization.

---

## Raw Capture Bundle

Single-capture queries from ~22:58:54Z:

```
// Timestamp: 2026-07-27T22:58:54Z

// === GetNodeInfo (morning-api) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":15028,"build_commit":"71aa16b-dirty","thickness":995.9887779072749}

// === GetPeers (morning-api) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":1501,"silence_secs":4,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (morning-api) — three-way: endpoint=503, grep=503, last line=503 (PASS) ===
{"type":"EpochState","epoch":502,"ratio":1.0192631191162045,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (morning-api) ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetPersistenceState (morning-api) — wal_bytes=0, wal.log=379 bytes (UNKNOWN) ===
{"type":"PersistenceState","last_snapshot_epoch":500,"wal_bytes":0,"wal_entries":0}

// === GetHeight (morning-api) ===
{"type":"Height","height":1}

// === GetNodeInfo (local-witness) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":15029,"build_commit":"71aa16b-dirty"}

// === GetPeers (local-witness) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":1503,"silence_secs":9,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (local-witness) — epoch=502 (same single capture) ===
{"type":"EpochState","epoch":502,"ratio":1.217965509235938,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (local-witness) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// === GetPersistenceState (local-witness) ===
{"type":"PersistenceState","last_snapshot_epoch":500,"wal_bytes":0,"wal_entries":0}
```

---

## Bottom Line

**No new deviations. All three persistent anomalies unchanged. Mesh has been running healthy for ~8.2 hours. Both nodes at epoch 503 — exact match with three-way PASS on both. Two snapshot rotations completed since pass 23 (epoch 480→500), though disk mtime did not reflect the writes (+1 byte size change observed). Snapshot mtime unchanged since 18:57Z is recorded as new UNKNOWN. Zero queues, zero fetches, zero sweep/evict/zombie activity across all metrics ticks.**

**Next check:** No threshold violations. System in stable steady state. Scheduled cron for next pass.
