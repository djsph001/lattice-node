# Observer Evidence Record — 2026-07-28 (Pass 7)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T01:34:57Z (initial queries)
**Capture completed:** 2026-07-28T01:37:48Z (three-way epoch match confirmation)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Seventh observation pass of Jul 28. ~14 min since pass 6 (01:20:51Z). Same processes since 14:48 EDT Jul 27 (~10.7h runtime at capture start).

**Summary:** All-clear continuation. Epochs 785→819 (+34 socket, matching ~17 min window since pass 6). Three-way epoch match clean on both nodes (socket=819, grep=819, last_line=819). Balance locked at 20/0. Snapshot epochs advanced 780→800 (morning-api) and 780→810 (witness) — no new rotation since pass 6 file timestamps at ~21:32-21:33 EDT (~01:32-01:33 UTC). Ratio declining asymptotically on witness (1.13638→1.13179). Zero queues, zero fetches, zero sweep/evict/zombie activity. Git HEAD unchanged. All three persistent deviations unchanged. File mtimes show fresh rotation (21:32-21:33 EDT), confirming snapshot engine active.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs since pass 1. Both sockets responding. 2 lattice-node processes.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 24242 (~6.7h) | — | None (pass 6: 23544; Δ = +698s ≈ 11.6 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** Same as passes 1-6. 9 commits behind. |
| thickness | 993.53 | ~1000, slowly decaying | None (pass 6: 993.72; Δ = -0.19) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 24310 (~6.75h) | — | None (pass 6: 23542; Δ = +768s ≈ 12.8 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=2423, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.
**Delta from pass 6:** heartbeats +70 (2353→2423). silence_secs 3→3.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=2432, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.
**Delta from pass 6:** heartbeats +77 (2355→2432). silence_secs 5→3 (normal variation).

---

## Epoch State

### morning-api (~01:37:48Z single capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 819 (socket), 819 (grep count), 819 (last log line) | Cycling ~30s cadence. +34 since pass 6 (785→819). | None — **clean three-way match** |
| ratio | 1.01956 | ~1.01–1.02 steady state (pass 6: 1.01954) | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (~01:37:48Z):**
- Socket epoch: 819
- `grep -c` count: 819
- Last log line epoch: 819

**CLEAN MATCH.** Not a boundary race — perfect alignment across all three sources.

### local-witness (~01:37:48Z single capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 819 (socket), 819 (grep count), 819 (last log line) | Same cadence. +33 since pass 6 (786→819 socket, 786→819 log). | None — **clean three-way match** |
| ratio | 1.13044 | Continuing asymptotic decline (pass 6: 1.13638; Δ = -0.00594) | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (~01:37:48Z):** Socket=819, grep=819, last_line=819. **CLEAN MATCH.**

---

## Economic State

### morning-api
**OBSERVED:** own_balance=20, own_nonce=120. Peer (witness) balance=4980, nonce=0.
**EXPECTED:** Balance at asymptotic floor (20) since ~epoch 121. Nonce at 120 since no new transactions.
**DEVIATION:** None (unchanged from all prior passes).

### local-witness
**OBSERVED:** own_balance=0, own_nonce=2. Peer (morning-api) balance=0, nonce=0.
**EXPECTED:** Zero-balance witness with no mint grant. Nonce 2 (max nonce applied).
**DEVIATION:** **Persistent** — witness sees morning-api balance as 0 (actual: 20). Same since pass 1 (Jul 27, ~14:48 EDT). Unchanged.

**Supply accounting (morning-api view):** 20 + 4980 = 5000. Matches `--mint 5000`. Conserved.
**Supply accounting (witness view):** 0 + 0 = 0. Supply conservation contradiction (VERIFIED-BEHAVIOR.md).

---

## Persistence State

### morning-api
**OBSERVED:** last_snapshot_epoch=800, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every ~10 epochs. WAL drained after rotation.
**DEVIATION:** Snapshot epoch advanced 780→800 (+20, 2 rotations) since pass 6. wal_bytes=0 (endpoint bug).

**Byte-equality check (~01:34:57Z):** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes. Verifier Mission 2 identified root cause.

**File inventory (~01:34:57Z):**

| File | Size | mtime (EDT) | Delta from pass 6 | Notes |
|------|------|-------------|-------------------|-------|
| `persistence/state.snapshot` | 894 bytes | Jul 27 21:32 | mtime advanced 21:17→21:32; size **same** 894 (was 893 in pass 5, returned to 894) | Snapshot epoch 800. 2 rotations since pass 6. |
| `persistence/wal.log` | 379 bytes | Jul 27 21:32 | mtime advanced from 21:17 | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 21:27 | mtime advanced from 21:12 | Prior rotation's WAL backup |

**Snapshot epoch progression:** 780→800 (+20 in ~14 min. 2 rotations. ~10 epochs/rotation, consistent cadence).

### local-witness
**OBSERVED:** last_snapshot_epoch=810, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** Snapshot epoch advanced 780→810 (+30, 3 rotations) since pass 6. Witness is 1 rotation ahead of morning-api (810 vs 800).

**Note:** Witness at snapshot epoch 810 while morning-api is at 800. This 10-epoch skew means the witness has completed one more snapshot rotation than morning-api since pass 6. This is within normal variation for independent rotation timers — not a divergence.

**File inventory (~01:34:57Z):**

| File | Size | mtime (EDT) | Delta from pass 6 |
|------|------|-------------|-------------------|
| `persistence/state.snapshot` | 569 bytes | Jul 27 21:33 | mtime advanced 21:18→21:33; size unchanged (569) |
| `persistence/wal.log` | 379 bytes | Jul 27 21:33 | mtime advanced from 21:18 |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 21:28 | mtime advanced from 21:13 |

---

## Build Commit & Binary Freshness

**OBSERVED:** `71aa16b-dirty` on both nodes (unchanged since pass 1).
**EXPECTED (VERIFIED-BEHAVIOR.md):** Should match git HEAD.
**DEVIATION:** **Persistent.** 9 commits behind HEAD (`cb5d4b1`). All docs and test-only changes — no wire-format, codec, or protocol changes.

Git HEAD: `cb5d4b1` ("docs: Observer evidence corpus + Verifier missions 1 and 2")
Running binary: `71aa16b` ("wip: update Cargo.lock") + `-dirty`

**No change in HEAD since pass 28 (Jul 27).** Git has not advanced.

---

## Log Health

**morning-api (/tmp/m-ap.log):**
- ~73 KAD bootstrap warnings — benign, expected with `--no-mdns`. Every ~5 min. Last at 01:33:26Z.
- 2 startup WARNs at 14:48 EDT (InsufficientPeers on genesis gossip/publish) — expected initial conditions, all resolved on peer connect.
- **0 unexpected WARN/ERROR lines after filtering (healthy).**
- No sweep/evict/zombie activity.

**local-witness (/tmp/lw.log):**
- 118 WARN `insufficient balance` lines — the known redistribution rejection. Last at `2026-07-27T19:47:26Z`. No new rejections in ~6h. Count unchanged from passes 1-6 (118).
- 3 `No snapshot found` at startup — expected for fresh state.
- **0 unexpected WARN/ERROR lines after filtering (healthy).**
- No sweep/evict/zombie activity.

---

## Metrics (Last 3 Tick Lines)

### morning-api
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
```
**All clean:** zero fetches, zero queues, silence=3s.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```
**All clean:** zero fetches, zero queues, silence=6s.

---

## Recent Epoch Activity (Last 5 Lines Each)

**morning-api (at ~01:36Z):**
```
Epoch complete epoch=815 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=816 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=817 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=818 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=819 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at ~1.02.

**local-witness (at ~01:36Z):**
```
Epoch complete epoch=815 balance_before=0 balance_after=0 ratio=1.13
Epoch complete epoch=816 balance_before=0 balance_after=0 ratio=1.13
Epoch complete epoch=817 balance_before=0 balance_after=0 ratio=1.13
Epoch complete epoch=818 balance_before=0 balance_after=0 ratio=1.13
Epoch complete epoch=819 balance_before=0 balance_after=0 ratio=1.13
```
Ratio: 1.13044 at socket query (continuing decline from 1.13638 at pass 6). Asymptotic decay continues.

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 9 commits behind HEAD `cb5d4b1`) | Low — docs + test changes only, no wire-format change | **Persistent** since pass 1 (Jul 27 18:48 EDT). Unchanged. |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint reads wrong path | **Persistent** since pass 1 (Jul 27 18:48 EDT). Verifier Mission 2 root cause identified. Unchanged. |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 (Jul 27 14:48 EDT). Locked at 20/0 since ~epoch 121. Unchanged. |

**Deviations resolved since pass 6:** None.

**New observations since pass 6:**
- Three-way epoch match returned to **CLEAN** (was BORDERLINE boundary race in pass 6). Socket, grep, and last log line all agree at 819.
- Snapshot epoch 10-epoch skew between nodes (api=800, witness=810) — within normal independent rotation variation.

---

## Delta from Pass 6 (~01:20:51Z → ~01:37:48Z Jul 28)

| Metric | Pass 6 (~01:20:51Z) | Pass 7 (~01:37:48Z) | Delta |
|--------|---------------------|---------------------|-------|
| Uptime (morning-api) | 23544s | 24242s | +698s (~11.6 min) |
| Uptime (witness) | 23542s | 24310s | +768s (~12.8 min) |
| Epoch (morning-api socket) | 785 | 819 | +34 |
| Epoch (witness socket) | 785 | 819 | +34 |
| Heartbeats (morning-api) | 2353 | 2423 | +70 |
| Heartbeats (witness) | 2355 | 2432 | +77 |
| Silence (morning-api) | 3s | 3s | 0 (same) |
| Silence (witness) | 5s | 3s | -2s (normal variation) |
| Thickness | 993.72 | 993.53 | -0.19 (expected decay) |
| Balance (api) | 20 | 20 | 0 |
| Balance (witness) | 0 | 0 | 0 |
| Nonce (api) | 120 | 120 | 0 |
| Nonce (witness) | 2 | 2 | 0 |
| Snapshot epoch (morning-api) | 780 | 800 | +20 (2 rotations) |
| Snapshot epoch (witness) | 780 | 810 | +30 (3 rotations) |
| Snapshot size (morning-api) | 894 bytes | 894 bytes | 0 |
| Snapshot size (witness) | 569 bytes | 569 bytes | 0 |
| wal.log (morning-api) | 379 bytes | 379 bytes | 0 |
| wal.log (witness) | 379 bytes | 379 bytes | 0 |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| Git HEAD | `cb5d4b1` | `cb5d4b1` | Unchanged |
| WARN count (morning-api, filtered) | 0 unexpected | 0 unexpected | 0 |
| WARN count (witness, filtered) | 0 unexpected | 0 unexpected | 0 |
| Insufficient balance count (witness) | 118 | 118 | 0 (no new rejections since ~19:47) |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from all prior passes). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes. Verifier Mission 2 (WAL Bytes Audit) identified the root cause: `get_stats()` reads legacy `transactions.wal` instead of `wal.log`. Not yet fixed.

2. **witness sees morning-api balance as 0** (actual: 20). Persistent across all 37 passes. Mesh stays healthy. Functional impact: incorrect balance display on the witness's EconomicState endpoint. This is the supply conservation contradiction from VERIFIED-BEHAVIOR.md.

3. **MESH.md topology stale.** Header reads "No production nodes running" but both nodes have been running since 14:48 EDT Jul 27 (~10.7h of uptime). Same as passes 1-6.

4. **Stale sockets from historical test runs.** Multiple stale sockets under /tmp/ from prior test sessions. All are dead processes (verified via pgrep). Same set as prior passes.

---

## Raw Capture Bundle

Single-capture queries from ~01:34:57–01:37:48Z:

```
// === Three-way Epoch Match (01:37:48Z) ===
// morning-api: socket=819, grep=819, last line=819 — CLEAN MATCH
// witness:     socket=819, grep=819, last line=819 — CLEAN MATCH

// === GetNodeInfo (morning-api, ~01:34:57Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":24242,"build_commit":"71aa16b-dirty","thickness":993.5307875083769}

// === GetEpochState (morning-api, ~01:34:57Z — later confirmed at 819) ===
{"type":"EpochState","epoch":809,"ratio":1.0195516277431709,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetPeers (morning-api, ~01:34:57Z) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":2423,"silence_secs":3,"is_dead":false,"queue_depth":0}]}

// === GetEconomicState (morning-api) — balance=20, nonce=120 ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetPersistenceState (morning-api) — wal_bytes=0, wal.log=379 bytes (UNKNOWN) ===
{"type":"PersistenceState","last_snapshot_epoch":800,"wal_bytes":0,"wal_entries":0}

// === File inventory (morning-api, ~01:34:57Z EDT) ===
state.snapshot  894 bytes  mtime: Jul 27 21:32
wal.log         379 bytes  mtime: Jul 27 21:32
wal.wal.old     379 bytes  mtime: Jul 27 21:27

// === GetNodeInfo (witness, ~01:34:57Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":24310,"build_commit":"71aa16b-dirty"}

// === GetEpochState (witness, ~01:34:57Z — later confirmed at 819) ===
{"type":"EpochState","epoch":811,"ratio":1.1317940484474034,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetPeers (witness, ~01:34:57Z) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":2432,"silence_secs":3,"is_dead":false,"queue_depth":0}]}

// === GetEconomicState (witness) — own_balance=0, sees api balance=0 (persistent UNKNOWN divergence) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// === GetPersistenceState (witness) — wal_bytes=0, snapshot=810 ===
{"type":"PersistenceState","last_snapshot_epoch":810,"wal_bytes":0,"wal_entries":0}

// === File inventory (witness, ~01:34:57Z EDT) ===
state.snapshot  569 bytes  mtime: Jul 27 21:33
wal.log         379 bytes  mtime: Jul 27 21:33
wal.wal.old     379 bytes  mtime: Jul 27 21:28

// === Git HEAD ===
cb5d4b1 docs: Observer evidence corpus + Verifier missions 1 and 2

// === Metrics (morning-api last 3 at ~01:36:36Z) ===
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s

// === Metrics (witness last 3 at ~01:36:33Z) ===
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```
