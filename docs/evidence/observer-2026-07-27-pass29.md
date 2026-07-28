# Observer Evidence Record — 2026-07-27 (Pass 29)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-27T23:53:05Z (single-capture bundle)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Twenty-ninth observation pass. Same processes since 14:48 EDT (~9.1h runtime). ~17 min since pass 28 (23:36Z).

**Summary:** All-clear continuation. Epochs 576→608 (+32 on both nodes). Three-way epoch match PASS on both nodes. Balance locked at 20. Snapshot epoch synchronized at 600 on both. Zero queues, zero fetches, zero sweep/evict/zombie activity. Git HEAD unchanged. All three persistent deviations unchanged.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs as pass 28 (2727391, 2727569). Both sockets responding. 2 lattice-node processes.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 18214 (~5.1h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind. Docs-only + test fixes since binary build. |
| thickness | 995.13 | ~1000, slowly decaying | None (pass 28: 995.38; Δ = -0.25) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 18233 (~5.1h) | — | None (slightly higher than api due to capture order) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=1820, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=1824, silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 28:** Heartbeats morning-api +103 (1717→1820), witness +102 (1722→1824). Silence: morning-api 7s→3s (normal variation), witness 9s→7s (normal variation). Queue depth 0 on both.

---

## Epoch State

### morning-api (~23:53:05Z single capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 608 (socket), 608 (grep at capture), 608 (last log line at capture) | Cycling ~19-20s cadence. +32 since pass 28 (576→608). | **PASS — three-way match.** |
| ratio | 1.01939 | ~1.01–1.02 steady state (pass 28: 1.01936) | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (~23:53:05Z):**
- Socket epoch: 608
- `grep -c` count: 608
- Last log line epoch: 608

**PASS.** All three agree.

### local-witness (~23:53:05Z bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 608 (socket), 608 (grep at capture), 608 (last log line at capture) | Same cadence. +32 since pass 28 (576→608). | **PASS — three-way match.** |
| ratio | 1.17808 | Continuing asymptotic decline (pass 28: 1.18836; Δ = -0.0103) | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (~23:53:05Z):**
- Socket epoch: 608
- `grep -c` count: 608
- Last log line epoch: 608

**PASS** on witness. Both nodes at epoch 608 at capture — synchronized (no producer phase lead visible at this capture instant).

---

## Economic State

### morning-api
**OBSERVED:** own_balance=20, own_nonce=120. Peer (witness) balance=4980, nonce=0.
**EXPECTED:** Balance at asymptotic floor (20) since ~epoch 121. Nonce at 120 since no new transactions.
**DEVIATION:** None.

### local-witness
**OBSERVED:** own_balance=0, own_nonce=2. Peer (morning-api) balance=0, nonce=0.
**EXPECTED:** Zero-balance witness with no mint grant. Nonce 2 (max nonce applied).
**DEVIATION:** **Persistent** — witness sees morning-api balance as 0 (actual: 20). Same since pass 1.

---

## Persistence State

### morning-api
**OBSERVED:** last_snapshot_epoch=600, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every ~10 epochs. WAL drained after rotation.
**DEVIATION:** None on the endpoint epoch values.

**Byte-equality check (~23:53:05Z):** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes, wal.wal.old at 379 bytes.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes.

**File inventory (~23:53:05Z):**

| File | Size | mtime | Delta from pass 28 | Notes |
|------|------|-------|-------------------|-------|
| `persistence/state.snapshot` | 895 bytes | Jul 27 19:47:xx | mtime advanced 19:32→19:47; size unchanged (895) | Snapshot epoch advanced 570→600 (3 rotations) |
| `persistence/wal.log` | 379 bytes | Jul 27 19:47:xx | mtime advanced from 19:32 | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 19:42 | mtime advanced from 19:27 | Prior rotation's WAL backup |

**Snapshot epoch progression:** 570→600 (pass 28→29). +30 in ~17 min. 3 rotations. Consistent cadence.

### local-witness
**OBSERVED:** last_snapshot_epoch=600, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** None. Snapshot epochs synchronized (both at 600).

**File inventory (~23:53:05Z):**

| File | Size | mtime | Delta from pass 28 |
|------|------|-------|-------------------|
| `persistence/state.snapshot` | 569 bytes | Jul 27 19:48 | mtime advanced 19:33→19:48; size unchanged |
| `persistence/wal.log` | 379 bytes | Jul 27 19:48 | mtime advanced from 19:33 |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 19:43 | mtime advanced from 19:28 |

---

## Build Commit & Binary Freshness

**OBSERVED:** `71aa16b-dirty` on both nodes (unchanged since pass 1).
**EXPECTED (VERIFIED-BEHAVIOR.md):** Should match git HEAD.
**DEVIATION:** **Persistent.** 9 commits behind HEAD (`cb5d4b1`). All docs and test-only changes — no wire-format, codec, or protocol changes.

Git HEAD: `cb5d4b1` ("docs: Observer evidence corpus + Verifier missions 1 and 2")
Running binary: `71aa16b` ("wip: update Cargo.lock") + `-dirty`

**No change in HEAD since pass 28.** Git has not advanced.

---

## Log Health

**morning-api (/tmp/m-ap.log):**
- 0 unexpected WARN/ERROR lines after filtering (healthy).
- Startup WARNs at 14:48: `InsufficientPeers` on genesis gossip — expected initial condition.
- No `libp2p_kad WARN Failed to trigger bootstrap` observed in current filter window (previous passes noted it every ~5 min benign with `--no-mdns`).

**local-witness (/tmp/lw.log):**
- 0 WARN, 0 ERROR lines after filtering.

**No sweep/evict/zombie activity** on either node (grep returns zero hits).

---

## Metrics (Last 3 Tick Lines)

### morning-api
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
```
**All clean:** zero fetches, zero queues, silence=3s. Unchanged from pass 28.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```
**All clean:** zero fetches, zero queues, silence=6s. Unchanged from pass 28 (was 6s then too).

---

## Recent Epoch Activity (Last 5 Lines Each)

**morning-api (at ~23:53Z):**
```
Epoch complete epoch=604 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=605 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=606 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=607 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=608 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at ~1.02.

**local-witness (at ~23:53Z):**
```
Epoch complete epoch=604 balance_before=0 balance_after=0 ratio=1.19
Epoch complete epoch=605 balance_before=0 balance_after=0 ratio=1.19
Epoch complete epoch=606 balance_before=0 balance_after=0 ratio=1.19
Epoch complete epoch=607 balance_before=0 balance_after=0 ratio=1.19
Epoch complete epoch=608 balance_before=0 balance_after=0 ratio=1.19
```
Ratio: 1.17808 at socket query (declining from 1.18836 at pass 28). Asymptotic decay continues. Both nodes synchronized at epoch 608 at capture — no producer phase lead visible.

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 9 commits behind HEAD `cb5d4b1`) | Low — docs + test changes only, no wire-format change | **Persistent** since pass 1 |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint not wired | **Persistent** since pass 1 |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 |

**Deviations resolved since pass 28:** None.

**New observations since pass 28:** None.

---

## Delta from Pass 28 (23:36Z → 23:53Z)

| Metric | Pass 28 (~23:36Z) | Pass 29 (~23:53Z) | Delta |
|--------|--------------------|--------------------|-------|
| Uptime (morning-api) | 17282s | 18214s | +932s (~15.5 min) |
| Uptime (witness) | 17213s | 18233s | +1020s (~17 min, capture timing difference) |
| Epoch (morning-api socket) | 576 | 608 | +32 |
| Epoch (witness socket) | 576 | 608 | +32 (synchronized) |
| Heartbeats (morning-api) | 1717 | 1820 | +103 |
| Heartbeats (witness) | 1722 | 1824 | +102 |
| Silence (morning-api) | 7s | 3s | -4s (normal variation) |
| Silence (witness) | 9s | 7s | -2s (normal variation) |
| Thickness | 995.38 | 995.13 | -0.25 (expected decay) |
| Balance | 20 | 20 | 0 |
| Nonce | 120 | 120 | 0 |
| Snapshot epoch (morning-api) | 570 | 600 | +30 (3 rotations) |
| Snapshot epoch (witness) | 570 | 600 | +30 (3 rotations, synchronized) |
| Snapshot size (morning-api) | 895 bytes | 895 bytes | 0 |
| Snapshot size (witness) | 569 bytes | 569 bytes | 0 |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| Git HEAD | `cb5d4b1` | `cb5d4b1` | Unchanged |
| WARN count (both, filtered) | 0 unexpected | 0 unexpected | 0 |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from all prior passes). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes. Verifier Mission 2 (WAL Bytes Audit, `docs/evidence/verifier-walbytes-audit-2026-07-27.md`) identified the root cause: `get_stats()` reads `self.wal_path` (legacy `transactions.wal`, retired in unified WAL migration) instead of `self.unified_wal_path` (`wal.log`). One-line fix identified but not applied. Marked as UNKNOWN because the Observer does not fix — the endpoint continues to report 0.

2. **witness sees morning-api balance as 0** (actual: 20). Persistent across all 29 passes. Mesh stays healthy; functional impact limited to incorrect balance display on the witness.

---

## Raw Capture Bundle

Single-capture queries from ~23:53:05Z:

```
// === GetNodeInfo (morning-api) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":18214,"build_commit":"71aa16b-dirty","thickness":995.1341770927352}

// === GetEpochState (morning-api) — three-way: endpoint=608, grep=608, last line=608 (PASS) ===
{"type":"EpochState","epoch":608,"ratio":1.0193918703594493,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === grep -c Epoch complete (morning-api) ===
608

// === Last log line (morning-api) ===
2026-07-27T23:51:56.405167Z  INFO lattice_node::node: Epoch complete epoch=608 balance_before=20 balance_after=20 ratio=1.02

// === GetPersistenceState (morning-api) — wal_bytes=0, wal.log=379 bytes (UNKNOWN) ===
{"type":"PersistenceState","last_snapshot_epoch":600,"wal_bytes":0,"wal_entries":0}

// === File inventory (morning-api) ===
state.snapshot  895 bytes  mtime: 19:47
wal.log         379 bytes  mtime: 19:47
wal.wal.old     379 bytes  mtime: 19:42

// === GetPeers (morning-api) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":1820,"silence_secs":3,"is_dead":false,"queue_depth":0}]}

// === GetEconomicState (morning-api) — balance=20, nonce=120 ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetNodeInfo (witness) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":18233,"build_commit":"71aa16b-dirty"}

// === GetEpochState (witness) — three-way: endpoint=608, grep=608, last line=608 (PASS) ===
{"type":"EpochState","epoch":608,"ratio":1.1780830331561776,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === grep -c Epoch complete (witness) ===
608

// === Last log line (witness) ===
2026-07-27T23:52:13.262241Z  INFO lattice_node::node: Epoch complete epoch=608 balance_before=0 balance_after=0 ratio=1.18

// === GetPersistenceState (witness) ===
{"type":"PersistenceState","last_snapshot_epoch":600,"wal_bytes":0,"wal_entries":0}

// === File inventory (witness) ===
state.snapshot  569 bytes  mtime: 19:48
wal.log         379 bytes  mtime: 19:48
wal.wal.old     379 bytes  mtime: 19:43

// === GetPeers (witness) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":1824,"silence_secs":7,"is_dead":false,"queue_depth":0}]}

// === GetEconomicState (witness) — own_balance=0, sees api balance=0 (persistent UNKNOWN divergence) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// === Git HEAD ===
cb5d4b1 docs: Observer evidence corpus + Verifier missions 1 and 2
```

---

## Bottom Line

**No new deviations. All three persistent anomalies unchanged (build_commit stale, wal_bytes=0 on endpoint, balance divergence).** Mesh running at ~9.1h with zero active issues: zero queues, zero fetches, zero sweep/evict/zombie activity. Three-way epoch match PASS on both nodes at capture. Epochs synchronized at 608 (no producer phase lead visible in this capture). Git HEAD unchanged since pass 28 — no protocol-impacting changes committed.

**Next observation pass:** Scheduled cron. No threshold violations.
