# Observer Evidence Record — 2026-07-28 (Pass 6)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T01:20:51Z (initial queries)
**Capture completed:** 2026-07-28T01:21:13Z (three-way epoch match confirmation)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.68.10.200, Boynton Beach FL)
**Session type:** Sixth observation pass of Jul 28. ~14 min since pass 5 (00:57:24Z ~01:09:00Z, but pass 5 captured at 00:58:00Z). Same processes since 14:48 EDT Jul 27 (~11.5h runtime at capture start).

**Summary:** All-clear continuation. Epochs 738→785 (+47 socket, matching ~23 min window since pass 5). Three-way epoch match shows same boundary race as passes 3-5 (socket=785, log=786). Balance locked at 20/0. Snapshot epochs advanced 730→780 on both nodes (+50, 5 rotations). Zero queues, zero fetches, zero sweep/evict/zombie activity. Git HEAD unchanged. All three persistent deviations unchanged. One minor note: morning-api snapshot size returned to 894 bytes (was 893 in pass 5, 894 in passes 2-4). No new deviations.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs as passes 1-5. Both sockets responding (morning-api at `/tmp/m-ap/lattice.sock`, witness at `/tmp/local-witness/lattice.sock`). 2 lattice-node processes.

**Note:** Earlier attempt to query witness at `/tmp/lw-id/lattice.sock` failed — that directory contains only `identity.key`. The witness socket lives at `/tmp/local-witness/lattice.sock` (matching `--storage-dir /tmp/local-witness`).

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 23544 (~6.5h) | — | None (pass 5: 22116; Δ = +1428s ≈ 23.8 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** Same as passes 1-5. 9 commits behind. |
| thickness | 993.72 | ~1000, slowly decaying | None (pass 5: 994.10; Δ = -0.38) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 23542 (~6.5h) | — | None (pass 5: 22102; Δ = +1440s ≈ 24 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=2353, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.
**Delta from pass 5:** heartbeats +143 (2210→2353). silence_secs 5→3 (normal variation, both <10s).

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=2355, silence_secs=5, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.
**Delta from pass 5:** heartbeats +144 (2211→2355). silence_secs 5→5 (same).

---

## Epoch State

### morning-api (~01:20:51Z single capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 785 (socket at 01:20:51Z), 786 (grep count), 786 (last log line) | Cycling ~30s cadence. +47 since pass 5 (738→785). | **BORDERLINE** — Same race-at-boundary as passes 3-5. |
| ratio | 1.01954 | ~1.01–1.02 steady state (pass 5: 1.01951) | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (~01:20:51Z):**
- Socket epoch: 785
- `grep -c` count: 786
- Last log line epoch: 786

**BORDERLINE (race at boundary).** Same pattern as passes 3-5. Socket read crossed an epoch boundary.

### local-witness (~01:20:51Z single capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 785 (socket at 01:20:51Z), 786 (grep count), 786 (last log line) | Same cadence. +47 since pass 5 (738→785 socket, 739→786 log). | **BORDERLINE** — Socket crossed boundary same as morning-api. |
| ratio | 1.13638 | Continuing asymptotic decline (pass 5: 1.14550; Δ = -0.00912) | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (~01:20:51Z):** Socket=785, grep=786, last line=786. **BORDERLINE** — same boundary race as morning-api.

---

## Economic State

### morning-api
**OBSERVED:** own_balance=20, own_nonce=120. Peer (witness) balance=4980, nonce=0.
**EXPECTED:** Balance at asymptotic floor (20) since ~epoch 121. Nonce at 120 since no new transactions.
**DEVIATION:** None (unchanged from all prior passes).

### local-witness
**OBSERVED:** own_balance=0, own_nonce=2. Peer (morning-api) balance=0, nonce=0.
**EXPECTED:** Zero-balance witness with no mint grant. Nonce 2 (max nonce applied).
**DEVIATION:** **Persistent** — witness sees morning-api balance as 0 (actual: 20). Same since pass 1 (Jul 27, ~14:48 EDT). Unchanged from pass 5.

**Supply accounting (morning-api view):** 20 + 4980 = 5000. Matches `--mint 5000`. Total supply conserved in morning-api's ledger.
**Supply accounting (witness view):** 0 + 0 = 0. Witness does not see the mint or redistribution transfers. This is the supply conservation contradiction identified in VERIFIED-BEHAVIOR.md (Section: Not Verified — Confirmed Protocol-Level Findings).

---

## Persistence State

### morning-api
**OBSERVED:** last_snapshot_epoch=780, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every ~10 epochs. WAL drained after rotation.
**DEVIATION:** Snapshot epoch advanced 730→780 (+50, 5 rotations) since pass 5. wal_bytes=0 (endpoint bug).

**Byte-equality check (~01:20:51Z):** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes, wal.wal.old at 379 bytes.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes. Verifier Mission 2 identified the root cause (`get_stats()` reads legacy `transactions.wal` instead of `wal.log`). Not yet fixed.

**File inventory (~01:20:51Z):**

| File | Size | mtime | Delta from pass 5 | Notes |
|------|------|-------|-------------------|-------|
| `persistence/state.snapshot` | 894 bytes | Jul 27 21:17 | mtime advanced 20:52→21:17; size **increased** 893→894 (+1B) | Snapshot epoch advanced 730→780 (5 rotations). Size returned to 894 (was 893 in pass 5, 894 in passes 2-4) |
| `persistence/wal.log` | 379 bytes | Jul 27 21:17 | mtime advanced from 20:52 | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 21:12 | mtime advanced from 20:47 | Prior rotation's WAL backup |

**Snapshot epoch progression:** 730→780 (+50 in ~23 min. 5 rotations. ~10 epochs/rotation consistent cadence).

**Snapshot size note:** 894 bytes, up from 893 in pass 5 (was 894 in passes 2-4). The size has oscillated between 893 and 894. Not a progressive drift.

### local-witness
**OBSERVED:** last_snapshot_epoch=780, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** None. Snapshot epochs synchronized at 780 (both nodes). +50 since pass 5, 5 rotations.

**File inventory (~01:20:51Z):**

| File | Size | mtime | Delta from pass 5 |
|------|------|-------|-------------------|
| `persistence/state.snapshot` | 569 bytes | Jul 27 21:18 | mtime advanced 20:53→21:18; size unchanged (569) |
| `persistence/wal.log` | 379 bytes | Jul 27 21:18 | mtime advanced from 20:53 |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 21:13 | mtime advanced from 20:48 |

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
- ~68 KAD bootstrap warnings — benign, expected with `--no-mdns`. Every ~5 min. Last at 01:18:26Z.
- 2 startup WARNs at 14:48 (InsufficientPeers on genesis gossip/publish) — expected initial conditions, all resolved on peer connect.
- **0 unexpected WARN/ERROR lines after filtering (healthy).**
- No sweep/evict/zombie activity.

**local-witness (/tmp/lw.log):**
- 118 WARN `insufficient balance` lines — the known redistribution rejection. Last occurrence at `2026-07-27T19:47:26.411589Z` — no new rejections in the last ~5.5h. Count unchanged from pass 5 (118).
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
**All clean:** zero fetches, zero queues, silence=3s. Unchanged from passes 1-5.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```
**All clean:** zero fetches, zero queues, silence=6s. Unchanged from passes 1-5.

---

## Recent Epoch Activity (Last 5 Lines Each)

**morning-api (at ~01:20Z):**
```
Epoch complete epoch=782 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=783 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=784 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=785 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=786 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at ~1.02.

**local-witness (at ~01:20Z):**
```
Epoch complete epoch=782 balance_before=0 balance_after=0 ratio=1.14
Epoch complete epoch=783 balance_before=0 balance_after=0 ratio=1.14
Epoch complete epoch=784 balance_before=0 balance_after=0 ratio=1.14
Epoch complete epoch=785 balance_before=0 balance_after=0 ratio=1.14
Epoch complete epoch=786 balance_before=0 balance_after=0 ratio=1.14
```
Ratio: 1.13638 at socket query (declining from 1.14550 at pass 5). Asymptotic decay continues.

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 9 commits behind HEAD `cb5d4b1`) | Low — docs + test changes only, no wire-format change | **Persistent** since pass 1 (Jul 27 18:48 EDT). Unchanged. |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint reads wrong path | **Persistent** since pass 1 (Jul 27 18:48 EDT). Verifier Mission 2 root cause identified. Unchanged. |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 (Jul 27 14:48 EDT). Locked at 20/0 since ~epoch 121. Unchanged. |

**Deviations resolved since pass 5:** None.

**New observations since pass 5:**
- Snapshot size on morning-api returned to 894 bytes (was 893 in pass 5, 894 in passes 2-4). Not a progressive drift.

---

## Delta from Pass 5 (~00:58:00Z → ~01:20:51Z Jul 28)

| Metric | Pass 5 (~00:58:00Z) | Pass 6 (~01:20:51Z) | Delta |
|--------|---------------------|---------------------|-------|
| Uptime (morning-api) | 22116s | 23544s | +1428s (~23.8 min) |
| Uptime (witness) | 22102s | 23542s | +1440s (~24 min) |
| Epoch (morning-api socket) | 738 | 785 | +47 (log at 786) |
| Epoch (witness socket) | 738 | 785 | +47 (log at 786) |
| Heartbeats (morning-api) | 2210 | 2353 | +143 |
| Heartbeats (witness) | 2211 | 2355 | +144 |
| Silence (morning-api) | 5s | 3s | -2s (normal variation) |
| Silence (witness) | 5s | 5s | 0 (same) |
| Thickness | 994.10 | 993.72 | -0.38 (expected decay) |
| Balance (api) | 20 | 20 | 0 |
| Balance (witness) | 0 | 0 | 0 |
| Nonce (api) | 120 | 120 | 0 |
| Nonce (witness) | 2 | 2 | 0 |
| Snapshot epoch (morning-api) | 730 | 780 | +50 (5 rotations) |
| Snapshot epoch (witness) | 730 | 780 | +50 (5 rotations) |
| Snapshot size (morning-api) | 893 bytes | 894 bytes | +1 (oscillated back to 894) |
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

2. **witness sees morning-api balance as 0** (actual: 20). Persistent across all 36 passes. Mesh stays healthy. Functional impact: incorrect balance display on the witness's EconomicState endpoint. This is the supply conservation contradiction from VERIFIED-BEHAVIOR.md.

3. **MESH.md topology stale.** Header reads "No production nodes running" but both nodes have been running since 14:48 EDT Jul 27 (~11.5h of uptime). Same as passes 1-5.

4. **Stale sockets from historical test runs.** 20 stale sockets under /tmp/ from prior test sessions (plus 1 live: morning-api at `/tmp/m-ap/lattice.sock`). All are dead processes (verified via pgrep). Same set as prior passes.

---

## Raw Capture Bundle

Single-capture queries from ~01:20:51–01:21:13Z:

```
// === Three-way Epoch Match (01:20:51Z) ===
// morning-api: socket=785, grep=786, last line=786 — BORDERLINE (boundary race)
// witness:     socket=785, grep=786, last line=786 — BORDERLINE (boundary race)

// === GetNodeInfo (morning-api, ~01:20:51Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":23544,"build_commit":"71aa16b-dirty","thickness":993.7221011107803}

// === GetEpochState (morning-api, ~01:20:51Z) ===
{"type":"EpochState","epoch":785,"ratio":1.0195378961457318,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetPeers (morning-api, ~01:20:51Z) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":2353,"silence_secs":3,"is_dead":false,"queue_depth":0}]}

// === GetEconomicState (morning-api) — balance=20, nonce=120 ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetPersistenceState (morning-api) — wal_bytes=0, wal.log=379 bytes (UNKNOWN) ===
{"type":"PersistenceState","last_snapshot_epoch":780,"wal_bytes":0,"wal_entries":0}

// === File inventory (morning-api, ~01:20:51Z) ===
state.snapshot  894 bytes  mtime: Jul 27 21:17
wal.log         379 bytes  mtime: Jul 27 21:17
wal.wal.old     379 bytes  mtime: Jul 27 21:12

// === GetNodeInfo (witness, ~01:20:51Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":23542,"build_commit":"71aa16b-dirty"}

// === GetEpochState (witness, ~01:20:51Z) ===
{"type":"EpochState","epoch":785,"ratio":1.1363830900185459,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetPeers (witness, ~01:20:51Z) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":2355,"silence_secs":5,"is_dead":false,"queue_depth":0}]}

// === GetEconomicState (witness) — own_balance=0, sees api balance=0 (persistent UNKNOWN divergence) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// === GetPersistenceState (witness) — wal_bytes=0, snapshot=780 (synchronized) ===
{"type":"PersistenceState","last_snapshot_epoch":780,"wal_bytes":0,"wal_entries":0}

// === File inventory (witness, ~01:20:51Z) ===
state.snapshot  569 bytes  mtime: Jul 27 21:18
wal.log         379 bytes  mtime: Jul 27 21:18
wal.wal.old     379 bytes  mtime: Jul 27 21:13

// === Git HEAD ===
cb5d4b1 docs: Observer evidence corpus + Verifier missions 1 and 2

// === Metrics (morning-api last 3 at ~01:18:26Z) ===
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s

// === Metrics (witness last 3 at ~01:18:23Z) ===
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```
