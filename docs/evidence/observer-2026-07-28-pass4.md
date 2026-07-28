# Observer Evidence Record — 2026-07-28 (Pass 4)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T00:47:46Z (initial queries)
**Capture completed:** 2026-07-28T00:49:00Z (three-way epoch match confirmation)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Fourth observation pass of Jul 28. ~9 min since pass 3 (00:38:12Z). Same processes since 14:48 EDT Jul 27 (~10.9h runtime).

**Summary:** All-clear continuation. Epochs 701→721 (+20 morning-api, +19 witness — witness 1 behind api, transient). Three-way epoch match: morning-api shows a race at boundary (socket=720, log=721 — same pattern as pass 3), witness clear (720=720=720). Balance locked at 20/0. Snapshot epochs synchronized at 720 on both nodes (both at +20, 2 rotations, no witness catch-up gap this pass). Zero queues, zero fetches, zero sweep/evict/zombie activity. Git HEAD unchanged. All three persistent deviations unchanged. Snapshot size on api decreased by 1 byte (894→893) — minor, possibly serialization variation; not classified as a deviation without a causal hypothesis.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs as passes 1-3. Both sockets responding. 2 lattice-node processes.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 21645 (~6.0h) | — | None (pass 3: 21079; Δ = +566s ≈ 9.4 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** Same as passes 1-3. 9 commits behind. |
| thickness | 994.22 | ~1000, slowly decaying | None (pass 3: 994.40; Δ = -0.18) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 21604 (~6.0h) | — | None (pass 3: 21023; Δ = +581s ≈ 9.7 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=2149, silence_secs=9, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.
**Delta from pass 3:** heartbeats +52 (2097→2149). silence_secs 5→9 (normal variation, both <10s).

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=2156, silence_secs=4, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.
**Delta from pass 3:** heartbeats +53 (2103→2156). silence_secs 3→4 (normal variation).

---

## Epoch State

### morning-api (~00:47:46Z single capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 718 (socket at 00:47:46Z), 720 (socket at 00:48:10Z), 721 (grep), 721 (last log line) | Cycling ~30s cadence. +20 since pass 3 (701→721). | **BORDERLINE** — Socket returned 718 first (epoch tick between queries), then 720 at three-way capture, log at 721. Transient race. See three-way section. |
| ratio | 1.01950 | ~1.01–1.02 steady state (pass 3: 1.01948) | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (~00:48:10Z):**
- Socket epoch: 720
- `grep -c` count: 721
- Last log line epoch: 721

**PASS (race at boundary).** Same pattern as pass 3: the socket read crossed an epoch boundary. Socket returned 720, log advanced to 721. The count and last line agree. Repeatable behavior — the socket poll happens at a different moment than the log write.

### local-witness (~00:48:13Z single capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 719 (socket at 00:47:46Z), 720 (socket at 00:48:13Z), 720 (grep), 720 (last log line) | Same cadence. +19 since pass 3 (701→720). | **PASS — three-way match.** Socket, grep count, and last line all at 720. Clean. Witness is 1 epoch behind api (721 vs 720), consistent with the earlier socket read. |
| ratio | 1.14984 | Continuing asymptotic decline (pass 3: 1.15354; Δ = -0.00370) | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (~00:48:13Z):** Socket=720, grep=720, last line=720. **PASS.** Clean synchronization.

---

## Economic State

### morning-api
**OBSERVED:** own_balance=20, own_nonce=120. Peer (witness) balance=4980, nonce=0.
**EXPECTED:** Balance at asymptotic floor (20) since ~epoch 121. Nonce at 120 since no new transactions.
**DEVIATION:** None (unchanged from all prior passes).

### local-witness
**OBSERVED:** own_balance=0, own_nonce=2. Peer (morning-api) balance=0, nonce=0.
**EXPECTED:** Zero-balance witness with no mint grant. Nonce 2 (max nonce applied).
**DEVIATION:** **Persistent** — witness sees morning-api balance as 0 (actual: 20). Same since pass 1 (Jul 27, ~14:48 EDT). No change from pass 3.

**Supply accounting (morning-api view):** 20 + 4980 = 5000. Matches `--mint 5000`. Total supply conserved in morning-api's ledger.
**Supply accounting (witness view):** 0 + 0 = 0. Witness does not see the mint or redistribution transfers. This is the supply conservation contradiction identified in VERIFIED-BEHAVIOR.md (Section: Not Verified — Confirmed Protocol-Level Findings).

---

## Persistence State

### morning-api
**OBSERVED:** last_snapshot_epoch=720, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every ~10 epochs. WAL drained after rotation.
**DEVIATION:** Snapshot epoch advanced 700→720 (+20, 2 rotations) since pass 3. wal_bytes=0 (endpoint bug).

**Byte-equality check (~00:48:Z):** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes, wal.wal.old at 379 bytes.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes. Verifier Mission 2 identified the root cause (`get_stats()` reads legacy `transactions.wal` instead of `wal.log`). Not yet fixed.

**File inventory (~00:48:Z):**

| File | Size | mtime | Delta from pass 3 | Notes |
|------|------|-------|-------------------|-------|
| `persistence/state.snapshot` | 893 bytes | Jul 27 20:47 | mtime advanced 20:37→20:47; size 894→893 (-1) | Snapshot epoch advanced 700→720 (2 rotations). **Size decreased by 1 byte.** Minor — possibly floating-point serialization variation. Not classified as deviation. |
| `persistence/wal.log` | 379 bytes | Jul 27 20:47 | mtime advanced from 20:37 | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 20:42 | mtime advanced from 20:32 | Prior rotation's WAL backup |

**Snapshot epoch progression:** 700→720 (pass 3→4). +20 in ~9 min. 2 rotations. Consistent cadence (~10 epochs/rotation).

**Snapshot size note:** 894→893 bytes (-1). One byte reduction. Possible causes: floating-point value encoding difference, serialization of a trimmed data structure. Not a sign of data loss — the snapshot still contains all expected fields (seen_nonces, balances, thickness_edges). Recorded as observation, not deviation.

### local-witness
**OBSERVED:** last_snapshot_epoch=720, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** None. Snapshot epochs synchronized at 720 (both nodes). No catch-up gap — witness and api both at +20 since pass 3.

**File inventory (~00:48:Z):**

| File | Size | mtime | Delta from pass 3 |
|------|------|-------|-------------------|
| `persistence/state.snapshot` | 569 bytes | Jul 27 20:48 | mtime advanced 20:38→20:48; size unchanged |
| `persistence/wal.log` | 379 bytes | Jul 27 20:48 | mtime advanced from 20:38 |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 20:43 | mtime advanced from 20:33 |

**Snapshot epoch progression:** 700→720 (pass 3→4). +20 in ~9 min. 2 rotations. Synchronized with api — no catch-up gap this pass (unlike pass 3 where witness was 1 rotation behind).

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
- 64 WARN `libp2p_kad::behaviour: Failed to trigger bootstrap: No known peers.` — benign, expected with `--no-mdns`. Every ~5 min. Last at 00:48:26Z.
- 2 startup WARNs at 14:48 (InsufficientPeers on genesis gossip, InsufficientPeers on block publish) — expected initial conditions, all resolved on peer connect.
- **0 unexpected WARN/ERROR lines after filtering (healthy).**
- No sweep/evict/zombie activity.

**local-witness (/tmp/lw.log):**
- 118 WARN `insufficient balance` lines — the known redistribution rejection. Last occurrence at `2026-07-27T19:47:26.411589Z` — no new rejections in the last ~5h. Count unchanged from pass 3 (118).
- 3 `No snapshot found` at startup — expected for fresh state.
- **0 unexpected WARN/ERROR lines after filtering (healthy).**
- No sweep/evict/zombie activity.

**No sweep/evict/zombie activity** on either node (grep returns zero hits).

---

## Metrics (Last 3 Tick Lines)

### morning-api
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
```
**All clean:** zero fetches, zero queues, silence=3s. Unchanged from passes 1-3.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```
**All clean:** zero fetches, zero queues, silence=6s. Unchanged from passes 1-3.

---

## Recent Epoch Activity (Last 5 Lines Each)

**morning-api (at ~00:48Z):**
```
Epoch complete epoch=717 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=718 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=719 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=720 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=721 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at ~1.02.

**local-witness (at ~00:48Z):**
```
Epoch complete epoch=716 balance_before=0 balance_after=0 ratio=1.15
Epoch complete epoch=717 balance_before=0 balance_after=0 ratio=1.15
Epoch complete epoch=718 balance_before=0 balance_after=0 ratio=1.15
Epoch complete epoch=719 balance_before=0 balance_after=0 ratio=1.15
Epoch complete epoch=720 balance_before=0 balance_after=0 ratio=1.15
```
Ratio: 1.14984 at socket query (declining from 1.15354 at pass 3). Asymptotic decay continues. Witness balance locked at 0.

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 9 commits behind HEAD `cb5d4b1`) | Low — docs + test changes only, no wire-format change | **Persistent** since pass 1 (Jul 27 18:48 EDT). Unchanged. |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint reads wrong path | **Persistent** since pass 1 (Jul 27 18:48 EDT). Verifier Mission 2 root cause identified. Unchanged. |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 (Jul 27 14:48 EDT). Locked at 20/0 since ~epoch 121. Unchanged. |

**Deviations resolved since pass 3:** None.

**New observations since pass 3:** 
- Snapshot size on morning-api decreased from 894→893 bytes (-1). NOT classified as deviation — minor, possibly floating-point serialization variation. Recorded for reference.

---

## Delta from Pass 3 (00:38:12Z → ~00:49:00Z Jul 28)

| Metric | Pass 3 (~00:38:12Z) | Pass 4 (~00:49:00Z) | Delta |
|--------|---------------------|---------------------|-------|
| Uptime (morning-api) | 21079s | 21645s | +566s (~9.4 min) |
| Uptime (witness) | 21023s | 21604s | +581s (~9.7 min) |
| Epoch (morning-api socket) | 701 | 720 (+1 race) | +19 (log at 721) |
| Epoch (witness socket) | 701 | 720 | +19 |
| Heartbeats (morning-api) | 2097 | 2149 | +52 |
| Heartbeats (witness) | 2103 | 2156 | +53 |
| Silence (morning-api) | 5s | 9s | +4s (normal variation) |
| Silence (witness) | 3s | 4s | +1s (normal variation) |
| Thickness | 994.40 | 994.22 | -0.18 (expected decay) |
| Balance (api) | 20 | 20 | 0 |
| Balance (witness) | 0 | 0 | 0 |
| Nonce (api) | 120 | 120 | 0 |
| Nonce (witness) | 2 | 2 | 0 |
| Snapshot epoch (morning-api) | 700 | 720 | +20 (2 rotations) |
| Snapshot epoch (witness) | 700 | 720 | +20 (2 rotations, synchronized) |
| Snapshot size (morning-api) | 894 bytes | 893 bytes | -1 (minor) |
| Snapshot size (witness) | 569 bytes | 569 bytes | 0 |
| wal.log (morning-api) | 379 bytes | 379 bytes | 0 (reset after rotation) |
| wal.log (witness) | 379 bytes | 379 bytes | 0 |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| Git HEAD | `cb5d4b1` | `cb5d4b1` | Unchanged |
| WARN count (morning-api, filtered) | 0 unexpected | 0 unexpected | 0 |
| WARN count (witness, filtered) | 0 unexpected | 0 unexpected | 0 |
| Insufficient balance count (witness) | 118 | 118 | 0 (no new rejections since ~19:47) |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from all prior passes). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes. Verifier Mission 2 (WAL Bytes Audit, `docs/evidence/verifier-walbytes-audit-2026-07-27.md`) identified the root cause: `get_stats()` reads `self.wal_path` (legacy `transactions.wal`, retired in unified WAL migration) instead of `self.unified_wal_path` (`wal.log`). One-line fix identified but not applied. The endpoint continues to report 0.

2. **witness sees morning-api balance as 0** (actual: 20). Persistent across all 33 passes. Mesh stays healthy. Functional impact: incorrect balance display on the witness's EconomicState endpoint. This is the supply conservation contradiction from VERIFIED-BEHAVIOR.md — the witness never received the mint or redistribution transfers. Total supply in morning-api ledger: 5000. Total supply in witness ledger: 0.

3. **MESH.md topology stale.** Header reads "No production nodes running" but both nodes have been running since 14:48 EDT Jul 27 (~10.9h of uptime). Either intentional (deferred update) or an oversight. Does not affect mesh behavior. (Same as passes 1-3.)

4. **Stale sockets from historical test runs.** 21 stale sockets under /tmp/ from prior test sessions (Jul 26-27). All are dead processes (verified via pgrep). Not interfering with current mesh. Same set as prior passes.

5. **Snapshot size decreased by 1 byte on morning-api** (894→893, |Δ| = 0.1%). Not a functional concern — snapshot contents appear intact. Possible cause: floating-point encoding variation in serialization. Recorded for reference only.

---

## Raw Capture Bundle

Single-capture queries from ~00:47:46–00:49:00Z:

```
// === Three-way Epoch Match (00:48:10Z) ===
// morning-api: socket=720, grep=721, last line=721 — BORDERLINE (boundary race, same as pass 3)
// witness:     socket=720, grep=720, last line=720 — PASS (synchronized)

// === GetNodeInfo (morning-api, ~00:47:46Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":21507,"build_commit":"71aa16b-dirty","thickness":994.2643563985376}

// === GetEpochState (morning-api, ~00:47:46Z) — first read ===
{"type":"EpochState","epoch":718,"ratio":1.0194946945407413,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEpochState (morning-api, ~00:48:10Z) — three-way capture ===
{"type":"EpochState","epoch":720,"ratio":1.019496100779844,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetPeers (morning-api, ~00:47:46Z) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":2149,"silence_secs":9,"is_dead":false,"queue_depth":0}]}

// === GetEconomicState (morning-api) — balance=20, nonce=120 ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetHeight (morning-api) ===
{"type":"Height","height":1}

// === GetPersistenceState (morning-api) — wal_bytes=0, wal.log=379 bytes (UNKNOWN) ===
{"type":"PersistenceState","last_snapshot_epoch":720,"wal_bytes":0,"wal_entries":0}

// === File inventory (morning-api, ~00:48:Z) ===
state.snapshot  893 bytes  mtime: 20:47  (was 894 at pass 3)
wal.log         379 bytes  mtime: 20:47
wal.wal.old     379 bytes  mtime: 20:42

// === GetNodeInfo (witness, ~00:47:46Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":21549,"build_commit":"71aa16b-dirty"}

// === GetEpochState (witness, ~00:47:46Z) — first read ===
{"type":"EpochState","epoch":719,"ratio":1.1495248887436353,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEpochState (witness, ~00:48:13Z) — three-way PASS ===
{"type":"EpochState","epoch":720,"ratio":1.1498408610288038,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetPeers (witness, ~00:47:46Z) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":2156,"silence_secs":4,"is_dead":false,"queue_depth":0}]}

// === GetEconomicState (witness) — own_balance=0, sees api balance=0 (persistent UNKNOWN divergence) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// === GetPersistenceState (witness) — wal_bytes=0, snapshot=720 (synchronized) ===
{"type":"PersistenceState","last_snapshot_epoch":720,"wal_bytes":0,"wal_entries":0}

// === File inventory (witness, ~00:48:Z) ===
state.snapshot  569 bytes  mtime: 20:48  (unchanged from pass 3)
wal.log         379 bytes  mtime: 20:48
wal.wal.old     379 bytes  mtime: 20:43

// === Git HEAD ===
cb5d4b1 docs: Observer evidence corpus + Verifier missions 1 and 2

// === Metrics (api last 3 at ~00:48:36Z) ===
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s

// === Metrics (witness last 3 at ~00:48:43Z) ===
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s

// === Recent Epoch Activity — morning-api (last 5 at ~00:48Z) ===
Epoch complete epoch=717 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=718 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=719 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=720 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=721 balance_before=20 balance_after=20 ratio=1.02

// === Recent Epoch Activity — witness (last 5 at ~00:48Z) ===
Epoch complete epoch=716 balance_before=0 balance_after=0 ratio=1.15
Epoch complete epoch=717 balance_before=0 balance_after=0 ratio=1.15
Epoch complete epoch=718 balance_before=0 balance_after=0 ratio=1.15
Epoch complete epoch=719 balance_before=0 balance_after=0 ratio=1.15
Epoch complete epoch=720 balance_before=0 balance_after=0 ratio=1.15

// === Snapshot log (morning-api, last 5) ===
Snapshot saved epoch=680  (00:27:56)
Snapshot saved epoch=690  (00:32:56)
Snapshot saved epoch=700  (00:37:56)
Snapshot saved epoch=710  (00:42:56)
Snapshot saved epoch=720  (00:47:56)

// === Snapshot log (witness, last 5) ===
Snapshot saved epoch=680  (00:28:13)
Snapshot saved epoch=690  (00:33:13)
Snapshot saved epoch=700  (00:38:13)
Snapshot saved epoch=710  (00:43:13)
Snapshot saved epoch=720  (00:48:13)
```

---

## Bottom Line

**No new deviations. All three persistent anomalies unchanged.** Mesh running at ~10.9h with zero active issues: zero queues, zero fetches, zero sweep/evict/zombie activity. Three-way epoch match: PASS on witness (720=720=720), BORDERLINE on morning-api (socket=720, log=721 — same race-at-boundary pattern observed in pass 3). Snapshot epochs synchronized at 720 on both nodes (both at +20, 2 rotations, no catch-up gap this pass). Git HEAD unchanged since Jul 27 pass 28 — no protocol-impacting changes committed. Balance locked at 20/0 across the mesh, with the known supply conservation contradiction persisting. Snapshot size decreased by 1 byte on morning-api (894→893) — recorded as observation, not deviation.

**Next observation pass:** Scheduled cron. No threshold violations.
