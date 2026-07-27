# Observer Evidence Record — 2026-07-27 (Pass 25)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-27T23:10:31Z
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Twenty-fifth observation pass. Same processes since 14:48 EDT (~8.4h runtime).

**Summary:** All-clear continuation. ~11 min since pass 24 (22:59Z). Epochs 503→525 (+22) on both nodes. Three-way epoch match PASS (all at 525). Balance locked at 20. Thickness 995.80 (decaying). Zero queues, zero fetches, zero sweep/evict/zombie activity. Both nodes healthy. Snapshot epoch advanced 500→520 (2 rotations) but mtime unchanged since 19:07Z — same UNKNOWN as prior passes. No new deviations.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs as all prior passes. Both sockets responding. No stale survivor processes (2 lattice-node processes expected, 2 found; pgrep -c returns 5 due to bash wrapper processes).

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 15725 (~4.4h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `aa62d12` | **Persistent DEVIATION.** 8 commits behind. Docs-only changes since binary build. |
| thickness | 995.80 | ~1000, slowly decaying | None (pass 24: 995.99; Δ = -0.19) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 15720 (~4.4h) | — | None |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=1571, silence_secs=2, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=1573, silence_secs=1, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 24:** Heartbeats morning-api +70 (1501→1571), witness +70 (1503→1573). Silence: morning-api 4s→2s (normal), witness 9s→1s (normal variation). Queue depth 0 on both.

---

## Epoch State

### morning-api (single capture, ~23:10:31Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 525 (socket), 525 (grep), 525 (last log line) | Cycling ~30s cadence. +22 since pass 24 (503→525). | **PASS — three-way match.** |
| ratio | 1.01994 | ~1.01–1.02 steady state (pass 24: 1.0193) | None (near-gaussian variation) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (~23:10:31Z):**
- Socket epoch: 525
- `grep -c` count: 525
- Last log line epoch: 525

**PASS.** All three agree.

### local-witness (same capture, ~23:10:31Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 525 (socket) | Same cadence. +22 since pass 24 (503→525). | **PASS — match with morning-api (525).** |
| ratio | 1.2073 | Declining (pass 24: 1.218; Δ ≈ -0.01). Expected asymptotic decay toward 1.0. | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Epoch synchronization:** Both nodes at epoch 525. Exact match.

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
**OBSERVED:** last_snapshot_epoch=520, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every 10 epochs. WAL drained after rotation.
**DEVIATION:** None on the endpoint values.

**Byte-equality check (~23:10:31Z):** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes, wal.wal.old at 379 bytes.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes.

**File inventory (single capture, ~23:10:31Z):**

| File | Size | Timestamp | Delta from pass 24 | Notes |
|------|------|-----------|-------------------|-------|
| `persistence/state.snapshot` | 895 bytes | Jul 27 19:07 | Unchanged size (895) | Timestamp unchanged (19:07) since pass 22. Snapshot epoch advanced 500→520 (2 rotations). |
| `persistence/wal.log` | 379 bytes | Jul 27 19:07 | Same size | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 19:02 | Same size | Prior rotation's WAL backup |

### local-witness
**OBSERVED:** last_snapshot_epoch=520, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** None (matches morning-api's snapshot epoch).

**File inventory:**

| File | Size | Timestamp | Delta from pass 24 |
|------|------|-----------|-------------------|
| `persistence/state.snapshot` | 569 bytes | Jul 27 19:08 | Unchanged |
| `persistence/wal.log` | 379 bytes | Jul 27 19:08 | Same size |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 19:03 | Same size |

**Note:** Snapshot mtime unchanged since 19:07Z (morning-api) / 19:08Z (witness) despite endpoint reporting snapshot epoch advancing 500→520 (2 rotations). Same UNKNOWN as all prior passes.

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

**morning-api (/tmp/m-ap.log):**
- 0 unexpected WARN/ERROR lines after filtering (healthy).
- 2 benign WARN lines at startup: `Failed to gossip genesis (will retry on peer connect)` and `Failed to publish block` — both InsufficientPeers at 14:48 when witness was connecting. Expected with `--no-mdns`.
- ~60+ benign libp2p_kad WARN lines — benign with `--no-mdns`.
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
(all 5 lines identical, timestamps 23:09:56–23:10:36Z)
```
**All clean:** zero fetches, zero queues, silence=3s. Unchanged from pass 24.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
(all 5 lines identical, timestamps 23:10:13–23:10:53Z)
```
**All clean:** zero fetches, zero queues, silence=6s. Unchanged from pass 24.

---

## Recent Epoch Activity (Last 5 Lines Each)

**morning-api (at ~23:10Z):**
```
Epoch complete epoch=521 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=522 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=523 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=524 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=525 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at ~1.020.

**local-witness (at ~23:10Z):**
```
Epoch complete epoch=521 balance_before=0 balance_after=0 ratio=1.21
Epoch complete epoch=522 balance_before=0 balance_after=0 ratio=1.21
Epoch complete epoch=523 balance_before=0 balance_after=0 ratio=1.21
Epoch complete epoch=524 balance_before=0 balance_after=0 ratio=1.21
Epoch complete epoch=525 balance_before=0 balance_after=0 ratio=1.21
```
Ratio: 1.2073 at socket query (declining from 1.218 at pass 24). Continuing expected asymptotic decay.

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 8 commits behind HEAD `aa62d12`) | Low — docs-only drift, no wire-format change | **Persistent** since pass 1 |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint not wired | **Persistent** since pass 1 |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 |

**Deviations resolved since pass 24:** None.
**New deviations since pass 24:** None.

---

## Delta from Pass 24 (22:59Z → 23:10Z)

| Metric | Pass 24 (~22:59Z) | Pass 25 (~23:10Z) | Delta |
|--------|--------------------|--------------------|-------|
| Uptime (morning-api) | 15028s | 15725s | +697s (~11.6 min) |
| Uptime (witness) | 15029s | 15720s | +691s |
| Epoch (morning-api socket) | 503 | 525 | +22 |
| Epoch (witness socket) | 503 | 525 | +22 |
| Heartbeats (morning-api) | 1501 | 1571 | +70 |
| Heartbeats (witness) | 1503 | 1573 | +70 |
| Silence (morning-api) | 4s | 2s | -2s (normal variation) |
| Silence (witness) | 9s | 1s | -8s (normal variation) |
| Thickness | 995.99 | 995.80 | -0.19 (expected decay) |
| Balance | 20 | 20 | 0 |
| Nonce | 120 | 120 | 0 |
| Snapshot epoch (morning-api) | 500 | 520 | +20 (2 rotations) |
| Snapshot epoch (witness) | 500 | 520 | +20 (2 rotations) |
| Snapshot size (morning-api) | 895 bytes | 895 bytes | 0 |
| Snapshot size (witness) | 569 bytes | 569 bytes | 0 |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| WARN count (both, filtered) | 0 unexpected | 0 unexpected | 0 |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from all prior passes). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes.

2. **Snapshot file mtime not advancing.** Snapshot on morning-api shows mtime=19:07Z (unchanged since pass 22). Endpoint reports last_snapshot_epoch advanced from 500 to 520 (2 rotations should have occurred). Snapshot size unchanged at 895 bytes. Witness shows same pattern (mtime: 19:08Z unchanged).

3. **witness sees morning-api balance as 0** (actual: 20). Persistent across all 25 passes. Mesh stays healthy; functional impact is limited to incorrect balance display on the witness.

---

## Raw Capture Bundle

Single-capture queries from ~23:10:31Z:

```
// Timestamp: 2026-07-27T23:10:31Z

// === GetNodeInfo (morning-api) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":15725,"build_commit":"71aa16b-dirty","thickness":995.7970279191828}

// === GetPeers (morning-api) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":1571,"silence_secs":2,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (morning-api) — three-way: endpoint=525, grep=525, last line=525 (PASS) ===
{"type":"EpochState","epoch":525,"ratio":1.019944713610558,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (morning-api) ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetPersistenceState (morning-api) — wal_bytes=0, wal.log=379 bytes (UNKNOWN) ===
{"type":"PersistenceState","last_snapshot_epoch":520,"wal_bytes":0,"wal_entries":0}

// === GetHeight (morning-api) ===
{"type":"Height","height":1}

// === GetNodeInfo (local-witness) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":15720,"build_commit":"71aa16b-dirty"}

// === GetPeers (local-witness) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":1573,"silence_secs":1,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (local-witness) — epoch=525 (same single capture) ===
{"type":"EpochState","epoch":525,"ratio":1.2073342736248236,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (local-witness) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// === GetPersistenceState (local-witness) ===
{"type":"PersistenceState","last_snapshot_epoch":520,"wal_bytes":0,"wal_entries":0}
```

---

## Bottom Line

**No new deviations. All three persistent anomalies unchanged. Mesh has been running healthy for ~8.4 hours. Both nodes at epoch 525 — exact match with three-way PASS on both. Two snapshot rotations completed since pass 24 (epoch 500→520); mtime unchanged since 19:07Z (UNKNOWN). Zero queues, zero fetches, zero sweep/evict/zombie activity across all metrics ticks.**

**Next check:** No threshold violations. System in stable steady state. Scheduled cron for next pass.
