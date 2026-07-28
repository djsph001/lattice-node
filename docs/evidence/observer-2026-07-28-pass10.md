# Observer Evidence Record — 2026-07-28 (Pass 10)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture timestamps:** 2026-07-28T02:07:41Z (start), T02:12:14Z (three-way match), T02:09Z (file inventory)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200)
**Session type:** Tenth observation pass of Jul 28. ~16 min since pass 9 (01:55:26Z). Same processes since 14:48 EDT Jul 27 (~11h runtime at capture start).

**Summary:** All-clear continuation. Epochs advanced 855→890 (+35 on both nodes, ~17 min window). Three-way epoch match clean on both nodes (socket=890, grep=890, last_line=890). Balance locked at 20/0. Snapshot epoch advanced 850→890 on both nodes (+40, 4 rotations). Zero queues, zero fetches, zero sweep/evict/zombie activity. Git HEAD unchanged. All three persistent deviations unchanged. No new deviations.

**Delta from pass 9:** +35 epochs, +4 snapshot rotations, no new anomalies.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs since pass 1 (Jul 27). Both sockets responding. 2 lattice-node processes. Transient extra bash PID (3066512) observed in one pgrep pass but absent on re-check — likely a non-node build process.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 26683 (~7.41h) | — | None (pass 9: 25526; Δ = +1157s ≈ 19.3 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** Same as pass 1. 9 commits behind. |
| thickness | 992.94 | ~1000, slowly decaying | None (pass 9: 993.20; Δ = -0.26) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 26668 (~7.41h) | — | None (pass 9: 25549; Δ = +1119s ≈ 18.7 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=2667, silence_secs=8, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.
**Delta from pass 9:** heartbeats +116 (2551→2667). silence_secs 8→8 (stable).

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=2668, silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.
**Delta from pass 9:** heartbeats +112 (2556→2668). silence_secs 5→7 (normal variation).

---

## Epoch State

### morning-api (~02:12:14Z single capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 890 (socket), 890 (grep count), 890 (last log line) | Cycling ~30s cadence. +35 since pass 9 (855→890). | None — **clean three-way match** |
| ratio | 1.01959 | ~1.01–1.02 steady state (pass 9: 1.01957) | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (~02:12:14Z):**
- Socket epoch: 890
- `grep -c` count: 890
- Last log line epoch: 890

**CLEAN MATCH.**

### local-witness (~02:12:14Z single capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 890 (socket), 890 (grep count), 890 (last log line) | Same cadence. +35 since pass 9 (855→890). | None — **clean three-way match** |
| ratio | 1.12035 | Continuing asymptotic decline (pass 9: 1.12513; Δ = -0.00478) | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (~02:12:14Z):** Socket=890, grep=890, last_line=890. **CLEAN MATCH.**

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
**OBSERVED:** last_snapshot_epoch=890, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every ~10 epochs. WAL drained after rotation.
**DEVIATION:** Snapshot epoch advanced 850→890 (+40, 4 rotations) since pass 9. wal_bytes=0 (endpoint bug).

**Byte-equality check (~02:09Z):** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes. Verifier Mission 2 identified root cause (get_stats reads legacy `transactions.wal` path).

**File inventory (~02:09Z):**

| File | Size | mtime (EDT) | Delta from pass 9 | Notes |
|------|------|-------------|-------------------|-------|
| `persistence/state.snapshot` | 893 bytes | Jul 27 22:07 | mtime advanced 21:52→22:07; size unchanged from pass 9 (894→893, ±1 trivial) | Snapshot epoch 890 (now 890 at three-way). 4 rotations since pass 9. |
| `persistence/wal.log` | 379 bytes | Jul 27 22:07 | mtime advanced from 21:52 | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 22:02 | mtime advanced from 21:47 | Prior rotation's WAL backup |

**Snapshot epoch progression:** 850→860→870→880→890 (+40 in ~17 min. 4 rotations. ~10 epochs/rotation, consistent cadence).

### local-witness
**OBSERVED:** last_snapshot_epoch=890, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** Snapshot epoch advanced 850→890 (+40, 4 rotations) since pass 9. Both nodes converged at 890.

**File inventory (~02:09Z):**

| File | Size | mtime (EDT) | Delta from pass 9 |
|------|------|-------------|-------------------|
| `persistence/state.snapshot` | 569 bytes | Jul 27 22:08 | mtime advanced 21:53→22:08; size unchanged (569) |
| `persistence/wal.log` | 379 bytes | Jul 27 22:08 | mtime advanced from 21:53 |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 22:03 | mtime advanced from 21:48 |

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
- 89 Kademlia bootstrap warnings — benign, expected with `--no-mdns`. Every ~5 min. Last at ~20:23Z Jul 27 (KAD bootstrap stops after timeout).
- 3 WARN `No snapshot found` at startup — expected for fresh state.
- 2 startup WARNs (`InsufficientPeers` on genesis gossip/publish) — expected initial conditions, all resolved on peer connect.
- 1 `Connection from non-mDNS peer` at first peer connect — expected, 2-node mesh.
- **0 unexpected WARN/ERROR lines after filtering (healthy).**
- No sweep/evict/zombie activity.

**local-witness (/tmp/lw.log):**
- 118 WARN `insufficient balance` lines — the known redistribution rejection. Last at 2026-07-27T19:47:26Z. No new rejections in ~6.5h. Count unchanged from passes 1-9 (118).
- 3 `No snapshot found` at startup — expected for fresh state.
- 1 `Connection from non-mDNS peer` at first peer connect — expected.
- **0 unexpected WARN/ERROR lines after filtering (healthy).**
- No sweep/evict/zombie activity.

---

## Metrics (Last 4 Tick Lines at ~02:11Z)

### morning-api
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
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
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```
**All clean:** zero fetches, zero queues, silence=6s.

---

## Recent Epoch Activity (Last 5 Lines Each at ~02:11Z)

**morning-api (at ~02:11Z):**
```
Epoch complete epoch=882 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=883 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=884 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=885 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=886 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at ~1.02.

**local-witness (at ~02:11Z):**
```
Epoch complete epoch=882 balance_before=0 balance_after=0 ratio=1.12
Epoch complete epoch=883 balance_before=0 balance_after=0 ratio=1.12
Epoch complete epoch=884 balance_before=0 balance_after=0 ratio=1.12
Epoch complete epoch=885 balance_before=0 balance_after=0 ratio=1.12
Epoch complete epoch=886 balance_before=0 balance_after=0 ratio=1.12
```
Ratio: 1.12035 at three-way match (continuing decline from 1.12513 at pass 9). Asymptotic decay continues.

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 9 commits behind HEAD `cb5d4b1`) | Low — docs + test changes only, no wire-format change | **Persistent** since pass 1 (Jul 27 18:48 EDT). Unchanged. |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint reads wrong path | **Persistent** since pass 1 (Jul 27 18:48 EDT). Verifier Mission 2 root cause identified. Unchanged. |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 (Jul 27 14:48 EDT). Locked at 20/0 since ~epoch 121. Unchanged. |

**Deviations resolved since pass 9:** None.

**Observations since pass 9:**
- Three-way epoch match **CLEAN** on both nodes (socket=890, grep=890, last_line=890).
- Snapshot epoch advanced 850→890 on both nodes (+40, 4 rotations in ~17 min).
- No new log warnings or metrics anomalies.

---

## Delta from Pass 9 (~01:55:26Z → ~02:12:14Z Jul 28)

| Metric | Pass 9 (~01:55:26Z) | Pass 10 (~02:12:14Z) | Delta |
|--------|---------------------|----------------------|-------|
| Uptime (morning-api) | 25526s | 26683s | +1157s (~19.3 min) |
| Uptime (witness) | 25549s | 26668s | +1119s (~18.7 min) |
| Epoch (morning-api socket) | 855 | 890 | +35 |
| Epoch (witness socket) | 855 | 890 | +35 |
| Heartbeats (morning-api) | 2551 | 2667 | +116 |
| Heartbeats (witness) | 2556 | 2668 | +112 |
| Silence (morning-api) | 8s | 8s | 0 (stable) |
| Silence (witness) | 5s | 7s | +2s (normal variation) |
| Thickness (api) | 993.20 | 992.94 | -0.26 (expected decay) |
| Balance (api) | 20 | 20 | 0 |
| Balance (witness) | 0 | 0 | 0 |
| Nonce (api) | 120 | 120 | 0 |
| Nonce (witness) | 2 | 2 | 0 |
| Snapshot epoch (morning-api) | 850 | 890 | +40 (4 rotations) |
| Snapshot epoch (witness) | 850 | 890 | +40 (4 rotations) |
| Snapshot size (morning-api) | 894 bytes | 893 bytes | -1 (trivial) |
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

2. **witness sees morning-api balance as 0** (actual: 20). Persistent across all 40 passes. Mesh stays healthy. Functional impact: incorrect balance display on the witness's EconomicState endpoint. This is the supply conservation contradiction from VERIFIED-BEHAVIOR.md.

3. **MESH.md topology stale.** Header reads "No production nodes running" but both nodes have been running since 14:48 EDT Jul 27 (~11h of uptime). Same as passes 1-9.

4. **Stale sockets from historical test runs.** 15 stale sockets under /tmp/ from prior test sessions. All are dead processes (confirmed via nc probe — all return no response or empty). Same set as prior passes. No new stale sockets observed.

---

## Raw Capture Bundle

Single-capture queries from ~02:07:41–02:12:14Z:

```
// === Three-way Epoch Match (02:12:14Z) ===
// morning-api: socket=890, grep=890, last line=890 — CLEAN MATCH
// witness:     socket=890, grep=890, last line=890 — CLEAN MATCH

// === GetNodeInfo (morning-api, ~02:09Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":26472,"build_commit":"71aa16b-dirty","thickness":992.941135733297}

// === GetEpochState (morning-api, ~02:09Z) ===
{"type":"EpochState","epoch":883,"ratio":1.0195892606894048,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetPeers (morning-api, ~02:09Z) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":2645,"silence_secs":9,"is_dead":false,"queue_depth":0}]}

// === GetEconomicState (morning-api) — balance=20, nonce=120 ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetPersistenceState (morning-api, ~02:09Z) — wal_bytes=0, snapshot=880 ===
{"type":"PersistenceState","last_snapshot_epoch":880,"wal_bytes":0,"wal_entries":0}

// === GetHeight (morning-api, ~02:09Z) ===
{"type":"Height","height":1}

// === Three-way capture uptime (02:12:14Z) ===
// morning-api: uptime_secs=26683, epoch=890, snapshot=890
// witness: uptime_secs=26668, epoch=890, snapshot=890

// === File inventory (morning-api, ~02:09Z EDT) ===
state.snapshot  893 bytes  mtime: Jul 27 22:07
wal.log         379 bytes  mtime: Jul 27 22:07
wal.wal.old     379 bytes  mtime: Jul 27 22:02

// === GetNodeInfo (witness, ~02:09Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":26467,"build_commit":"71aa16b-dirty"}

// === GetEpochState (witness, ~02:09Z) ===
{"type":"EpochState","epoch":883,"ratio":1.1204979847382148,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetPeers (witness, ~02:09Z) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":2647,"silence_secs":8,"is_dead":false,"queue_depth":0}]}

// === GetEconomicState (witness) — own_balance=0, sees api balance=0 (persistent UNKNOWN divergence) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// === GetPersistenceState (witness, ~02:09Z) — wal_bytes=0, snapshot=880 ===
{"type":"PersistenceState","last_snapshot_epoch":880,"wal_bytes":0,"wal_entries":0}

// === File inventory (witness, ~02:09Z EDT) ===
state.snapshot  569 bytes  mtime: Jul 27 22:08
wal.log         379 bytes  mtime: Jul 27 22:08
wal.wal.old     379 bytes  mtime: Jul 27 22:03

// === Git HEAD ===
cb5d4b1 docs: Observer evidence corpus + Verifier missions 1 and 2

// === Metrics (morning-api last 5 at ~02:11Z) ===
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s

// === Metrics (witness last 5 at ~02:11Z) ===
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```
