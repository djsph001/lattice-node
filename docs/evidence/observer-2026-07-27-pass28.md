# Observer Evidence Record — 2026-07-27 (Pass 28)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-27T23:36:07Z (bundle A: morning-api single-capture) / 23:36:23Z (bundle B: cross-node three-way)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Twenty-eighth observation pass. Same processes since 14:48 EDT (~8.8h runtime).

**Summary:** All-clear continuation. ~10 min since pass 27 (23:25Z). Epochs 556→576 (+20 morning-api), 556→576 (+20 witness). Three-way epoch match PASS on both nodes (boundary race noted below). Balance locked at 20. Snapshot epoch gap closed (both at 570). Zero queues, zero fetches, zero sweep/evict/zombie activity. Git HEAD advanced by 2 docs commits since pass 27. Three persistent deviations unchanged.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs as all prior passes. Both sockets responding. 2 lattice-node processes.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 17282 (~4.8h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind. Docs-only + test fixes since binary build. |
| thickness | 995.38 | ~1000, slowly decaying | None (pass 27: 995.55; Δ = -0.17) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 17213 (~4.8h) | — | None (slightly lower due to capture timing) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=1717, silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=1722, silence_secs=9, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 27:** Heartbeats morning-api +54 (1663→1717), witness +56 (1666→1722). Silence: morning-api 7s→7s (stable), witness 2s→9s (normal variation). Queue depth 0 on both.

---

## Epoch State

### morning-api (~23:36:07Z bundle A)
| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 576 (socket), 576 (grep at capture), 576 (last log line at capture) | Cycling ~30s cadence. +20 since pass 27 (556→576). | **PASS — three-way match.** |
| ratio | 1.01936 | ~1.01–1.02 steady state (pass 27: 1.01933) | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (~23:36:07Z):**
- Socket epoch: 576
- `grep -c` count: 576
- Last log line epoch: 576

**PASS.** All three agree.

**NOTE — Epoch boundary race:** By ~23:36:23Z (bundle B), morning-api had advanced to 577 (socket=576, grep=577, last line=577 — transition occurred between socket query and grep in the same bundle). This is a normal race at an epoch boundary. The three-way check from the earlier dedicated bundle (A) passed cleanly.

### local-witness (~23:36:23Z bundle B)
| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 576 (socket), 576 (grep at capture), 576 (last log line at capture) | Same cadence. +20 since pass 27 (556→576). | **PASS — three-way match.** |
| ratio | 1.18836 | Continuing asymptotic decline (pass 27: 1.19539; Δ = -0.0070) | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (~23:36:23Z):**
- Socket epoch: 576
- `grep -c` count: 576
- Last log line epoch: 576

**PASS** on witness individually. Both nodes at epoch 576 at capture — witness 1 epoch behind morning-api (which advanced to 577 during the bundle). Normal producer phase lead.

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
**OBSERVED:** last_snapshot_epoch=570, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every ~10 epochs. WAL drained after rotation.
**DEVIATION:** None on the endpoint epoch values.

**Byte-equality check (~23:36:07Z):** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes, wal.wal.old at 379 bytes.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes.

**File inventory (composite, ~23:36:07Z):**

| File | Size | mtime | Delta from pass 27 | Notes |
|------|------|-------|-------------------|-------|
| `persistence/state.snapshot` | 895 bytes | Jul 27 19:32:56 | mtime advanced 19:22→19:32; size unchanged (895) | Snapshot epoch advanced 550→570 (2 rotations) |
| `persistence/wal.log` | 379 bytes | Jul 27 19:32:56 | mtime advanced from 19:22 | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 19:27 | mtime advanced from 19:17 | Prior rotation's WAL backup |

**Snapshot epoch progression:** 550→570 (pass 27→28). +20 in ~10 min. 2 rotations. Consistent cadence.

### local-witness
**OBSERVED:** last_snapshot_epoch=570, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** None. Snapshot epochs now synchronized (both at 570).

**File inventory:**

| File | Size | mtime | Delta from pass 27 |
|------|------|-------|-------------------|
| `persistence/state.snapshot` | 569 bytes | Jul 27 19:33:13 | mtime advanced 19:23→19:33; size unchanged |
| `persistence/wal.log` | 379 bytes | Jul 27 19:33:13 | mtime advanced from 19:23 |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 19:28 | mtime advanced from 19:18 |

---

## Build Commit & Binary Freshness

**OBSERVED:** `71aa16b-dirty` on both nodes (unchanged since pass 1).
**EXPECTED (VERIFIED-BEHAVIOR.md):** Should match git HEAD.
**DEVIATION:** **Persistent.** 9 commits behind HEAD (`cb5d4b1`). All docs and test-only changes — no wire-format, codec, or protocol changes.

Git HEAD: `cb5d4b1` ("docs: Observer evidence corpus + Verifier missions 1 and 2")
Running binary: `71aa16b` ("wip: update Cargo.lock") + `-dirty`

Commits since binary: 9 total — 7 docs-only, 2 test fixes (witness harness fixture bugs + witness harness test declaration), 1 test addition (cap enforcement).

---

## Log Health

**morning-api (/tmp/m-ap.log):**
- 0 unexpected WARN/ERROR lines after filtering (healthy).
- Periodic `libp2p_kad WARN Failed to trigger bootstrap: No known peers` every ~5 min — benign with `--no-mdns`.
- 2 startup WARNs at 14:48: gossipsub `InsufficientPeers` — expected on initial connect.

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
**All clean:** zero fetches, zero queues, silence=3s. Unchanged from pass 27.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```
**All clean:** zero fetches, zero queues, silence=6s. Unchanged from pass 27.

---

## Recent Epoch Activity (Last 5 Lines Each)

**morning-api (at ~23:36Z):**
```
Epoch complete epoch=573 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=574 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=575 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=576 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=577 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at ~1.02.

**local-witness (at ~23:36Z):**
```
Epoch complete epoch=572 balance_before=0 balance_after=0 ratio=1.19
Epoch complete epoch=573 balance_before=0 balance_after=0 ratio=1.19
Epoch complete epoch=574 balance_before=0 balance_after=0 ratio=1.19
Epoch complete epoch=575 balance_before=0 balance_after=0 ratio=1.19
Epoch complete epoch=576 balance_before=0 balance_after=0 ratio=1.19
```
Ratio: 1.18836 at socket query (declining from 1.19539 at pass 27). Expected asymptotic decay. Phase relationship: morning-api at 577, witness at 576 (normal producer 1-epoch lead).

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 9 commits behind HEAD `cb5d4b1`) | Low — docs + test changes only, no wire-format change | **Persistent** since pass 1 |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint not wired | **Persistent** since pass 1 |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 |

**Deviations resolved since pass 27:** None.

**New observations since pass 27:** None.

---

## Delta from Pass 27 (23:25Z → 23:36Z)

| Metric | Pass 27 (~23:25Z) | Pass 28 (~23:36Z) | Delta |
|--------|--------------------|--------------------|-------|
| Uptime (morning-api) | 16651s | 17282s | +631s (~10.5 min) |
| Uptime (witness) | 16670s | 17213s | +543s (~9.0 min) |
| Epoch (morning-api socket) | 556 | 576 | +20 |
| Epoch (witness socket) | 556 | 576 | +20 (now equal) |
| Heartbeats (morning-api) | 1663 | 1717 | +54 |
| Heartbeats (witness) | 1666 | 1722 | +56 |
| Silence (morning-api) | 7s | 7s | 0 |
| Silence (witness) | 2s | 9s | +7s (normal variation) |
| Thickness | 995.55 | 995.38 | -0.17 (expected decay) |
| Balance | 20 | 20 | 0 |
| Nonce | 120 | 120 | 0 |
| Snapshot epoch (morning-api) | 550 | 570 | +20 (2 rotations) |
| Snapshot epoch (witness) | 550 | 570 | +20 (2 rotations, synchronized) |
| Snapshot size (morning-api) | 895 bytes | 895 bytes | 0 |
| Snapshot size (witness) | 569 bytes | 569 bytes | 0 |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| Git HEAD | `aa62d12` | `cb5d4b1` | +2 docs commits |
| WARN count (both, filtered) | 0 unexpected | 0 unexpected | 0 |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from all prior passes). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes. Verifier Mission 2 (WAL Bytes Audit, `docs/evidence/verifier-walbytes-audit-2026-07-27.md`) identified the root cause: `get_stats()` reads `self.wal_path` (legacy `transactions.wal`, retired in unified WAL migration) instead of `self.unified_wal_path` (`wal.log`). One-line fix identified but not applied. Marked as UNKNOWN because the Observer does not fix — the endpoint continues to report 0.

2. **witness sees morning-api balance as 0** (actual: 20). Persistent across all 28 passes. Mesh stays healthy; functional impact limited to incorrect balance display on the witness.

---

## Raw Capture Bundle

Single-capture queries from ~23:36:07Z (bundle A) and ~23:36:23Z (bundle B):

```
// Bundle A — Timestamp: 2026-07-27T23:36:07Z (morning-api focused)

// === GetEpochState (morning-api) — three-way: endpoint=576, grep=576, last line=576 (PASS) ===
{"type":"EpochState","epoch":576,"ratio":1.0193580071229422,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === grep -c Epoch complete (morning-api) ===
576

// === Last log line (morning-api) ===
2026-07-27T23:35:56.404801Z  INFO lattice_node::node: Epoch complete epoch=576 balance_before=20 balance_after=20 ratio=1.02

// === GetPersistenceState (morning-api) — wal_bytes=0, wal.log=379 bytes (UNKNOWN) ===
{"type":"PersistenceState","last_snapshot_epoch":570,"wal_bytes":0,"wal_entries":0}

// === File inventory (morning-api) ===
state.snapshot  895 bytes  mtime: 19:32:56
wal.log         379 bytes  mtime: 19:32:56
wal.wal.old     379 bytes  mtime: 19:27

// Bundle B — Timestamp: 2026-07-27T23:36:23Z (cross-node three-way)

// === GetEpochState (witness) ===
{"type":"EpochState","epoch":576,"ratio":1.1883603049708416,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEpochState (morning-api) ===
{"type":"EpochState","epoch":576,"ratio":1.0193580071229422,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === grep -c Epoch complete (witness) ===
576

// === Last log line (witness) ===
2026-07-27T23:36:13.262365Z  INFO lattice_node::node: Epoch complete epoch=576 balance_before=0 balance_after=0 ratio=1.19

// === grep -c Epoch complete (morning-api) — advanced to 577 between queries ===
577

// === Last log line (morning-api) ===
2026-07-27T23:36:26.404909Z  INFO lattice_node::node: Epoch complete epoch=577 balance_before=20 balance_after=20 ratio=1.02

// === File inventory (witness) ===
state.snapshot  569 bytes  mtime: 19:33:13
wal.log         379 bytes  mtime: 19:33:13
wal.wal.old     379 bytes  mtime: 19:28

// === GetNodeInfo (morning-api) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":17282,"build_commit":"71aa16b-dirty","thickness":995.381696245026}

// === GetNodeInfo (witness) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":17213,"build_commit":"71aa16b-dirty"}

// === GetPeers (morning-api) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":1717,"silence_secs":7,"is_dead":false,"queue_depth":0}]}

// === GetPeers (witness) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":1722,"silence_secs":9,"is_dead":false,"queue_depth":0}]}

// === Git HEAD ===
cb5d4b1 docs: Observer evidence corpus + Verifier missions 1 and 2
```

---

## Bottom Line

**No new deviations. All three persistent anomalies unchanged (build_commit stale, wal_bytes=0 on endpoint, balance divergence).** Mesh running at ~8.8h with zero active issues: zero queues, zero fetches, zero sweep/evict/zombie activity. Three-way epoch match PASS on both nodes at capture. Epoch boundary race between bundle A and bundle B is normal phase variation (producer morning-api 1 epoch ahead). Git HEAD advanced by 2 docs commits since pass 27 — no protocol-impacting changes.

**Next observation pass:** Scheduled cron. No threshold violations.
