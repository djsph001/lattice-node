# Observer Evidence Record — 2026-07-28 (Pass 9)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture timestamps:** 2026-07-28T01:55:01Z (timestamp), T01:55:26Z (three-way match), T01:55:43Z (file inventory)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200)
**Session type:** Ninth observation pass of Jul 28. ~8 min since pass 8 (01:46:41Z). Same processes since 14:48 EDT Jul 27 (~11h runtime at capture start).

**Summary:** All-clear continuation. Epochs advanced 837→855 (+18 on both nodes, ~8 min window). Three-way epoch match clean on both nodes (socket=855, grep=855, last_line=855). Balance locked at 20/0. Snapshot epoch advanced 830→850 on both nodes (converged, +20, 2 rotations). Ratio stable on morning-api (~1.0196), continuing asymptotic decline on witness (1.12765→1.12513). Zero queues, zero fetches, zero sweep/evict/zombie activity. Git HEAD unchanged. All three persistent deviations unchanged. No new deviations.

**Delta from pass 8:** +18 epochs, +2 snapshot rotations, no new anomalies.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs since pass 1 (Jul 27). Both sockets responding. 2 lattice-node processes.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 25526 (~7.09h) | — | None (pass 8: 25054; Δ = +472s ≈ 7.9 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** Same as pass 1. 9 commits behind. |
| thickness | 993.20 | ~1000, slowly decaying | None (pass 8: 993.32; Δ = -0.12) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 25549 (~7.10h) | — | None (pass 8: 25056; Δ = +493s ≈ 8.2 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=2551, silence_secs=8, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.
**Delta from pass 8:** heartbeats +47 (2504→2551). silence_secs 4→8 (normal variation).

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=2556, silence_secs=5, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.
**Delta from pass 8:** heartbeats +50 (2506→2556). silence_secs 8→5 (normal variation).

---

## Epoch State

### morning-api (~01:55:26Z single capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 855 (socket), 855 (grep count), 855 (last log line) | Cycling ~30s cadence. +18 since pass 8 (837→855). | None — **clean three-way match** |
| ratio | 1.01957 | ~1.01–1.02 steady state (pass 8: 1.01957) | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (~01:55:26Z):**
- Socket epoch: 855
- `grep -c` count: 855
- Last log line epoch: 855

**CLEAN MATCH.**

### local-witness (~01:55:26Z single capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 855 (socket), 855 (grep count), 855 (last log line) | Same cadence. +18 since pass 8 (837→855). | None — **clean three-way match** |
| ratio | 1.12513 | Continuing asymptotic decline (pass 8: 1.12765; Δ = -0.00252) | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (~01:55:26Z):** Socket=855, grep=855, last_line=855. **CLEAN MATCH.**

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
**OBSERVED:** last_snapshot_epoch=850, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every ~10 epochs. WAL drained after rotation.
**DEVIATION:** Snapshot epoch advanced 830→850 (+20, 2 rotations) since pass 8. wal_bytes=0 (endpoint bug).

**Byte-equality check (~01:55:43Z):** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes. Verifier Mission 2 identified root cause (get_stats reads legacy `transactions.wal` path).

**File inventory (~01:55:43Z):**

| File | Size | mtime (EDT) | Delta from pass 8 | Notes |
|------|------|-------------|-------------------|-------|
| `persistence/state.snapshot` | 894 bytes | Jul 27 21:52 | mtime advanced 21:42→21:52; size unchanged (894) | Snapshot epoch 850. 2 rotations since pass 8. |
| `persistence/wal.log` | 379 bytes | Jul 27 21:52 | mtime advanced from 21:42 | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 21:47 | mtime advanced from 21:37 | Prior rotation's WAL backup |

**Snapshot epoch progression:** 830→850 (+20 in ~8 min. 2 rotations. ~10 epochs/rotation, consistent cadence).

### local-witness
**OBSERVED:** last_snapshot_epoch=850, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** Snapshot epoch advanced 830→850 (+20, 2 rotations) since pass 8. Both nodes converged at 850.

**File inventory (~01:55:43Z):**

| File | Size | mtime (EDT) | Delta from pass 8 |
|------|------|-------------|-------------------|
| `persistence/state.snapshot` | 569 bytes | Jul 27 21:53 | mtime advanced 21:43→21:53; size unchanged (569) |
| `persistence/wal.log` | 379 bytes | Jul 27 21:53 | mtime advanced from 21:43 |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 21:48 | mtime advanced from 21:38 |

---

## Build Commit & Binary Freshness

**OBSERVED:** `71aa16b-dirty` on both nodes (unchanged since pass 1).
**EXPECTED (VERIFIED-BEHAVIOR.md):** Should match git HEAD.
**DEVIATION:** **Persistent.** 9 commits behind HEAD (`cb5d4b1`). All docs and test-only changes — no wire-format, codec, or protocol changes.

Git HEAD: `cb5d4b1` ("docs: Observer evidence corpus + Verifier missions 1 and 2")
Running binary: `71aa16b` ("wip: update Cargo.lock") + `-dirty`

**No change in HEAD since pass 1.** Git has not advanced.

Commits between running binary and HEAD (all docs/tests, no wire-format):
```
cb5d4b1 docs: Observer evidence corpus + Verifier missions 1 and 2
aa62d12 docs: note /tmp identity dir fragility across reboots
c008def docs: MESH.md — record stable PeerIds after mesh relaunch
93d0ef4 docs: restructure verified behavior with evidence tiers
7ab64c2 docs: sharpen MESH.md header to configuration-focused language
19c9d05 docs: split MESH.md (topology only) from VERIFIED-BEHAVIOR.md
32efcf1 fix: stale fixture bugs in witness harness
214eb73 fix: declare claimant variable in two_swarm witness harness tests
b4aa212 test: cap enforcement — 64th accepted, 65th rejected, duplicate is no-op
```

---

## Log Health

**morning-api (/tmp/m-ap.log):**
- ~84 KAD bootstrap warnings — benign, expected with `--no-mdns`. Every ~5 min. Last at 01:55Z.
- 2 startup WARNs at 14:48 EDT (InsufficientPeers on genesis gossip/publish) — expected initial conditions, all resolved on peer connect.
- **0 unexpected WARN/ERROR lines after filtering (healthy).**
- No sweep/evict/zombie activity.

**local-witness (/tmp/lw.log):**
- 118 WARN `insufficient balance` lines — the known redistribution rejection. Last at 2026-07-27T19:47:26Z. No new rejections in ~6h. Count unchanged from passes 1-8 (118).
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

**morning-api (at ~01:55Z):**
```
Epoch complete epoch=850 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=851 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=852 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=853 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=854 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at ~1.02.

**local-witness (at ~01:55Z):**
```
Epoch complete epoch=850 balance_before=0 balance_after=0 ratio=1.13
Epoch complete epoch=851 balance_before=0 balance_after=0 ratio=1.13
Epoch complete epoch=852 balance_before=0 balance_after=0 ratio=1.13
Epoch complete epoch=853 balance_before=0 balance_after=0 ratio=1.12
Epoch complete epoch=854 balance_before=0 balance_after=0 ratio=1.12
```
Ratio: 1.12513 at socket query (continuing decline from 1.12765 at pass 8). Asymptotic decay continues.

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 9 commits behind HEAD `cb5d4b1`) | Low — docs + test changes only, no wire-format change | **Persistent** since pass 1 (Jul 27 18:48 EDT). Unchanged. |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint reads wrong path | **Persistent** since pass 1 (Jul 27 18:48 EDT). Verifier Mission 2 root cause identified. Unchanged. |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 (Jul 27 14:48 EDT). Locked at 20/0 since ~epoch 121. Unchanged. |

**Deviations resolved since pass 8:** None.

**Observations since pass 8:**
- Three-way epoch match **CLEAN** on both nodes (socket=855, grep=855, last_line=855).
- Snapshot epoch advanced 830→850 on both nodes (converged at 850, +20, 2 rotations in ~8 min).
- No new log warnings or metrics anomalies.

---

## Delta from Pass 8 (~01:46:41Z → ~01:55:26Z Jul 28)

| Metric | Pass 8 (~01:46:41Z) | Pass 9 (~01:55:26Z) | Delta |
|--------|---------------------|---------------------|-------|
| Uptime (morning-api) | 25054s | 25526s | +472s (~7.9 min) |
| Uptime (witness) | 25056s | 25549s | +493s (~8.2 min) |
| Epoch (morning-api socket) | 837 | 855 | +18 |
| Epoch (witness socket) | 837 | 855 | +18 |
| Heartbeats (morning-api) | 2504 | 2551 | +47 |
| Heartbeats (witness) | 2506 | 2556 | +50 |
| Silence (morning-api) | 4s | 8s | +4s (normal variation) |
| Silence (witness) | 8s | 5s | -3s (normal variation) |
| Thickness (api) | 993.32 | 993.20 | -0.12 (expected decay) |
| Balance (api) | 20 | 20 | 0 |
| Balance (witness) | 0 | 0 | 0 |
| Nonce (api) | 120 | 120 | 0 |
| Nonce (witness) | 2 | 2 | 0 |
| Snapshot epoch (morning-api) | 830 | 850 | +20 (2 rotations) |
| Snapshot epoch (witness) | 830 | 850 | +20 (2 rotations) |
| Snapshot size (morning-api) | 894 bytes | 894 bytes | 0 |
| Snapshot size (witness) | 569 bytes | 569 bytes | 0 |
| wal.log (morning-api) | 379 bytes | 379 bytes | 0 |
| wal.log (witness) | 379 bytes | 379 bytes | 0 |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| Git HEAD | `cb5d4b1` | `cb5d4b1` | Unchanged |
| WARN count (morning-api, filtered) | 0 unexpected | 0 unexpected | 0 |
| WARN count (witness, filtered) | 0 unexpected | 0 unexpected | 0 |
| Insufficient balance count (witness) | 118 | 118 | 0 (no new rejections) |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from all prior passes). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes. Verifier Mission 2 (WAL Bytes Audit) identified the root cause: `get_stats()` reads legacy `transactions.wal` instead of `wal.log`. Not yet fixed.

2. **witness sees morning-api balance as 0** (actual: 20). Persistent across all 39 passes. Mesh stays healthy. Functional impact: incorrect balance display on the witness's EconomicState endpoint. This is the supply conservation contradiction from VERIFIED-BEHAVIOR.md.

3. **MESH.md topology stale.** Header reads "No production nodes running" but both nodes have been running since 14:48 EDT Jul 27 (~11h of uptime). Same as passes 1-8.

4. **Stale sockets from historical test runs.** Multiple stale sockets under /tmp/ from prior test sessions. All are dead processes (verified via nc probe). Same set as prior passes. No new stale sockets observed.

---

## Raw Capture Bundle

Single-capture queries from ~01:55:01–01:55:43Z:

```
// === Three-way Epoch Match (01:55:26Z) ===
// morning-api: socket=855, grep=855, last line=855 — CLEAN MATCH
// witness:     socket=855, grep=855, last line=855 — CLEAN MATCH

// === GetNodeInfo (morning-api, ~01:55:01Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":25526,"build_commit":"71aa16b-dirty","thickness":993.1960773272004}

// === GetEpochState (morning-api, ~01:55:01Z) ===
{"type":"EpochState","epoch":852,"ratio":1.019574292446669,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetPeers (morning-api, ~01:55:01Z) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":2551,"silence_secs":8,"is_dead":false,"queue_depth":0}]}

// === GetEconomicState (morning-api) — balance=20, nonce=120 ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetPersistenceState (morning-api) — wal_bytes=0, snapshot=850 ===
{"type":"PersistenceState","last_snapshot_epoch":850,"wal_bytes":0,"wal_entries":0}

// === GetHeight (morning-api) ===
{"type":"Height","height":1}

// === File inventory (morning-api, ~01:55:43Z EDT) ===
state.snapshot  894 bytes  mtime: Jul 27 21:52
wal.log         379 bytes  mtime: Jul 27 21:52
wal.wal.old     379 bytes  mtime: Jul 27 21:47

// === GetNodeInfo (witness, ~01:55:01Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":25549,"build_commit":"71aa16b-dirty"}

// === GetEpochState (witness, ~01:55:01Z) ===
{"type":"EpochState","epoch":852,"ratio":1.1251272403973767,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetPeers (witness, ~01:55:01Z) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":2556,"silence_secs":5,"is_dead":false,"queue_depth":0}]}

// === GetEconomicState (witness) — own_balance=0, sees api balance=0 (persistent UNKNOWN divergence) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// === GetPersistenceState (witness) — wal_bytes=0, snapshot=850 ===
{"type":"PersistenceState","last_snapshot_epoch":850,"wal_bytes":0,"wal_entries":0}

// === File inventory (witness, ~01:55:43Z EDT) ===
state.snapshot  569 bytes  mtime: Jul 27 21:53
wal.log         379 bytes  mtime: Jul 27 21:53
wal.wal.old     379 bytes  mtime: Jul 27 21:48

// === Git HEAD ===
cb5d4b1 docs: Observer evidence corpus + Verifier missions 1 and 2

// === Metrics (morning-api last 3 at ~01:55:16Z) ===
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s

// === Metrics (witness last 3 at ~01:55:23Z) ===
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```
