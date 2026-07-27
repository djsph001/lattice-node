# Observer Evidence Record — 2026-07-27 (Pass 27)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-27T23:25:57Z (composite bundle, spread ~+60s for log queries)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Twenty-seventh observation pass. Same processes since 14:48 EDT (~8.6h runtime).

**Summary:** All-clear continuation. ~8 min since pass 26 (23:17Z). Epochs 540→558 (+18 morning-api), 539→557 (+18 witness). Three-way epoch match PASS on both nodes — both now at 556/556/556 at capture time (socket). Snapshot epoch divergence from pass 26 (morning-api=540, witness=530, 10-epoch gap) **RESOLVED** — both now at 550. The gap was a phase lag, not a persistent skew. Balance locked at 20. Thickness 995.55 (continuing decay). Zero queues, zero fetches, zero sweep/evict/zombie activity.

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
| uptime_secs | 16651 (~4.6h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `aa62d12` | **Persistent DEVIATION.** 8 commits behind. Docs-only changes since binary build. |
| thickness | 995.55 | ~1000, slowly decaying | None (pass 26: 995.69; Δ = -0.14) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 16670 (~4.6h) | — | None (slightly higher than morning-api due to capture timing) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=1663, silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=1666, silence_secs=2, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 26:** Heartbeats morning-api +50 (1613→1663), witness +51 (1615→1666). Silence: morning-api 9s→7s, witness 7s→2s (normal variation). Queue depth 0 on both.

---

## Epoch State

### morning-api (composite, ~23:25:57Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 556 (socket), 556 (grep at capture), 556 (last log line at capture) | Cycling ~30s cadence. +16 since pass 26 (540→556). | **PASS — three-way match.** |
| ratio | 1.01933 | ~1.01–1.02 steady state (pass 26: 1.0193) | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (~23:25:57Z):**
- Socket epoch: 556
- `grep -c` count: 556
- Last log line epoch: 556

**PASS.** All three agree.

### local-witness (composite, ~23:25:57Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 556 (socket), 556 (grep at capture), 556 (last log line at capture) | Same cadence. +17 since pass 26 (539→556). **Now equal to morning-api.** | **PASS — three-way match.** Socket 556 now matches morning-api. |
| ratio | 1.1954 | Continuing asymptotic decline (pass 26: 1.2025; Δ = -0.0071) | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (~23:25:57Z):**
- Socket epoch: 556
- `grep -c` count: 556
- Last log line epoch: 556

**PASS** on witness individually. Both nodes now at epoch 556 — the 1-epoch phase difference from pass 26 is gone.

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
**OBSERVED:** last_snapshot_epoch=550, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every 10 epochs. WAL drained after rotation.
**DEVIATION:** None on the endpoint values.

**Byte-equality check (~23:25:57Z):** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes, wal.wal.old at 379 bytes.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes.

**File inventory (composite, ~23:25:57Z):**

| File | Size | mtime | Delta from pass 26 | Notes |
|------|------|-------|-------------------|-------|
| `persistence/state.snapshot` | 895 bytes | Jul 27 19:22:56 | mtime advanced 19:17→19:22; size unchanged (895) | Snapshot epoch advanced 540→550 (1 rotation) |
| `persistence/wal.log` | 379 bytes | Jul 27 19:22:56 | mtime advanced from 19:17 | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 19:17 | mtime advanced from 19:12 | Prior rotation's WAL backup |

**Snapshot epoch progression:** 540→550 (pass 26→27). +10 per ~8 min. 1 rotation. Consistent with earlier observations.

### local-witness
**OBSERVED:** last_snapshot_epoch=550, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** **RESOLVED.** Snapshot epoch gap from pass 26 (morning-api=540, witness=530, 10-epoch gap) is now closed — both at 550.

**File inventory:**

| File | Size | mtime | Delta from pass 26 |
|------|------|-------|-------------------|
| `persistence/state.snapshot` | 569 bytes | Jul 27 19:23:13 | mtime advanced 19:18→19:23; size unchanged |
| `persistence/wal.log` | 379 bytes | Jul 27 19:23:13 | mtime advanced from 19:18 |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 19:18 | mtime advanced from 19:08 |

**Snapshot epoch progression:** 530→550 (pass 26→27). +20 in ~8 min. **2 rotations** — witness caught up to morning-api's 550. The divergence was a phase lag, not a persistent skew.

---

## Build Commit & Binary Freshness

**OBSERVED:** `71aa16b-dirty` on both nodes (unchanged since pass 1).
**EXPECTED (VERIFIED-BEHAVIOR.md):** Should match git HEAD (`aa62d12`).
**DEVIATION:** **Persistent.** 8 commits behind. Docs-only changes — no wire-format, codec, or protocol changes.

Git HEAD: `aa62d12` ("docs: note /tmp identity dir fragility across reboots")
Running binary: `71aa16b` ("wip: update Cargo.lock") + `-dirty`

---

## Log Health

**morning-api (/tmp/m-ap.log):**
- 0 unexpected WARN/ERROR lines after filtering (healthy).
- Periodic `libp2p_kad WARN Failed to trigger bootstrap: No known peers` every 5 minutes — benign with `--no-mdns`.
- 2 startup WARNs at 14:48: `Failed to gossip genesis (will retry on peer connect)` / `Failed to publish block` — both InsufficientPeers. Expected with `--no-mdns` and witness connecting.
- No ERROR lines.
- No sweep/evict/zombie activity detected (grep for "sweep|evict|zombie|stale" — zero hits).

**local-witness (/tmp/lw.log):**
- 0 WARN, 0 ERROR lines.
- No sweep/evict/zombie activity.

---

## Metrics (Last 3 Tick Lines)

### morning-api
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
```
**All clean:** zero fetches, zero queues, silence=3s. Unchanged from pass 26.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```
**All clean:** zero fetches, zero queues, silence=6s. Unchanged from pass 26.

---

## Recent Epoch Activity (Last 5 Lines Each)

**morning-api (at ~23:25Z):**
```
Epoch complete epoch=552 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=553 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=554 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=555 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=556 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at ~1.02.

**local-witness (at ~23:25Z):**
```
Epoch complete epoch=553 balance_before=0 balance_after=0 ratio=1.20
Epoch complete epoch=554 balance_before=0 balance_after=0 ratio=1.20
Epoch complete epoch=555 balance_before=0 balance_after=0 ratio=1.20
Epoch complete epoch=556 balance_before=0 balance_after=0 ratio=1.20
Epoch complete epoch=557 balance_before=0 balance_after=0 ratio=1.20
```
Ratio: 1.1954 at socket query (declining from 1.2025 at pass 26). Expected asymptotic decay toward 1.0. Both nodes now at same epoch (556).

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 8 commits behind HEAD `aa62d12`) | Low — docs-only drift, no wire-format change | **Persistent** since pass 1 |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint not wired | **Persistent** since pass 1 |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 |

**Deviations resolved since pass 26:**
- **Snapshot epoch divergence (morning-api=540, witness=530, 10-epoch gap):** RESOLVED. Both now at 550. The gap was a phase lag — witness wrote 2 rotations in this interval while morning-api wrote 1.

**New observations since pass 26:** None.

---

## Delta from Pass 26 (23:17Z → 23:25Z)

| Metric | Pass 26 (~23:17Z) | Pass 27 (~23:25Z) | Delta |
|--------|--------------------|--------------------|-------|
| Uptime (morning-api) | 16153s | 16651s | +498s (~8.3 min) |
| Uptime (witness) | 16149s | 16670s | +521s (~8.7 min) |
| Epoch (morning-api socket) | 540 | 556 | +16 |
| Epoch (witness socket) | 539 | 556 | +17 (now equal to morning-api) |
| Heartbeats (morning-api) | 1613 | 1663 | +50 |
| Heartbeats (witness) | 1615 | 1666 | +51 |
| Silence (morning-api) | 9s | 7s | -2s (normal variation) |
| Silence (witness) | 7s | 2s | -5s (normal variation) |
| Thickness | 995.69 | 995.55 | -0.14 (expected decay) |
| Balance | 20 | 20 | 0 |
| Nonce | 120 | 120 | 0 |
| Snapshot epoch (morning-api) | 540 | 550 | +10 (1 rotation) |
| Snapshot epoch (witness) | 530 | 550 | +20 (2 rotations) — **gap closed** |
| Snapshot size (morning-api) | 895 bytes | 895 bytes | 0 |
| Snapshot size (witness) | 569 bytes | 569 bytes | 0 |
| Snapshot mtime (morning-api) | 19:17:56 | 19:22:56 | +5 min (advancing) |
| Snapshot mtime (witness) | 19:18:13 | 19:23:13 | +5 min (advancing) |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| WARN count (both, filtered) | 0 unexpected | 0 unexpected | 0 |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from all prior passes). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes.

2. **witness sees morning-api balance as 0** (actual: 20). Persistent across all 27 passes. Mesh stays healthy; functional impact limited to incorrect balance display on the witness.

---

## Raw Capture Bundle

Single-capture queries from ~23:25:57Z (composite, spread ~+60s for log):

```
// Timestamp: 2026-07-27T23:25:57Z (bundle)

// === GetNodeInfo (morning-api) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":16651,"build_commit":"71aa16b-dirty","thickness":995.5494054873718}

// === GetPeers (morning-api) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":1663,"silence_secs":7,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (morning-api) — three-way: endpoint=556, grep=556, last line=556 (PASS) ===
{"type":"EpochState","epoch":556,"ratio":1.0193348584340607,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (morning-api) ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetPersistenceState (morning-api) — wal_bytes=0, wal.log=379 bytes (UNKNOWN) ===
{"type":"PersistenceState","last_snapshot_epoch":550,"wal_bytes":0,"wal_entries":0}

// === GetHeight (morning-api) ===
{"type":"Height","height":1}

// === GetNodeInfo (local-witness) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":16670,"build_commit":"71aa16b-dirty"}

// === GetPeers (local-witness) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":1666,"silence_secs":2,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (local-witness) — three-way: endpoint=556, grep=556, last line=556 (PASS) ===
{"type":"EpochState","epoch":556,"ratio":1.1953854482374373,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (local-witness) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// === GetPersistenceState (local-witness) ===
{"type":"PersistenceState","last_snapshot_epoch":550,"wal_bytes":0,"wal_entries":0}

// === File inventory (morning-api) ===
state.snapshot  895 bytes  mtime: 19:22:56
wal.log         379 bytes  mtime: 19:22:56
wal.wal.old     379 bytes  mtime: 19:17

// === File inventory (witness) ===
state.snapshot  569 bytes  mtime: 19:23:13
wal.log         379 bytes  mtime: 19:23:13
wal.wal.old     379 bytes  mtime: 19:18

// === Git HEAD ===
aa62d12 docs: note /tmp identity dir fragility across reboots
```

---

## Bottom Line

**No new deviations. All three persistent anomalies unchanged (build_commit stale, wal_bytes=0 on endpoint, balance divergence). One UNKNOWN RESOLVED: the snapshot epoch gap (morning-api=540, witness=530) from pass 26 is closed — both now at 550. The gap was a phase lag, not a persistent skew. Mesh has been running healthy for ~8.6h. Zero queues, zero fetches, zero sweep/evict/zombie activity. Three-way epoch match PASS on both nodes, now synchronized at the same epoch (556).**

**Next observation pass:** Scheduled cron. No threshold violations.
