# Observer Evidence Record — 2026-07-28 (Pass 8)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture timestamps:** 2026-07-28T01:46:01Z (socket queries), 01:46:41Z (three-way epoch + file inventory)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200)
**Session type:** Eighth observation pass of Jul 28. ~12 min since pass 7 (01:34:57Z). Same processes since 14:48 EDT Jul 27 (~7h runtime at capture start).

**Summary:** All-clear continuation. Epochs advanced 819→837 (+18 on both nodes, ~12 min window). Three-way epoch match clean on both nodes (socket=837, grep=837, last_line=837). Balance locked at 20/0. Snapshot convergence: both nodes now at epoch 830 (morning-api advanced 800→830, witness 810→830 — the 10-epoch skew from pass 7 has resolved). Ratio stable on morning-api (~1.0196), continuing asymptotic decline on witness (1.13044→1.12765). Zero queues, zero fetches, zero sweep/evict/zombie activity. Git HEAD unchanged. All three persistent deviations continue unchanged.

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
| uptime_secs | 25054 (~6.96h) | — | None (pass 7: 24242; Δ = +812s ≈ 13.5 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** Same as passes 1-7. 9 commits behind. |
| thickness | 993.32 | ~1000, slowly decaying | None (pass 7: 993.53; Δ = -0.21) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 25056 (~6.96h) | — | None (pass 7: 24310; Δ = +746s ≈ 12.4 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=2504, silence_secs=4, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.
**Delta from pass 7:** heartbeats +81 (2423→2504). silence_secs 3→4 (normal variation).

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=2506, silence_secs=8, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.
**Delta from pass 7:** heartbeats +74 (2432→2506). silence_secs 3→8 (normal variation).

---

## Epoch State

### morning-api (~01:46:41Z single capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 837 (socket), 837 (grep count), 837 (last log line) | Cycling ~30s cadence. +18 since pass 7 (819→837). | None — **clean three-way match** |
| ratio | 1.01957 | ~1.01–1.02 steady state (pass 7: 1.01956) | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (~01:46:41Z):**
- Socket epoch: 837
- `grep -c` count: 837
- Last log line epoch: 837

**CLEAN MATCH.**

### local-witness (~01:46:41Z single capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 837 (socket), 837 (grep count), 837 (last log line) | Same cadence. +18 since pass 7 (819→837). | None — **clean three-way match** |
| ratio | 1.12765 | Continuing asymptotic decline (pass 7: 1.13044; Δ = -0.00279) | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (~01:46:41Z):** Socket=837, grep=837, last_line=837. **CLEAN MATCH.**

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
**OBSERVED:** last_snapshot_epoch=830, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every ~10 epochs. WAL drained after rotation.
**DEVIATION:** Snapshot epoch advanced 800→830 (+30, 3 rotations) since pass 7. wal_bytes=0 (endpoint bug).

**Byte-equality check (~01:46:41Z):** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes. Verifier Mission 2 identified root cause.

**File inventory (~01:46:41Z):**

| File | Size | mtime (EDT) | Delta from pass 7 | Notes |
|------|------|-------------|-------------------|-------|
| `persistence/state.snapshot` | 894 bytes | Jul 27 21:42 | mtime advanced 21:32→21:42; size unchanged (894) | Snapshot epoch 830. 3 rotations since pass 7. |
| `persistence/wal.log` | 379 bytes | Jul 27 21:42 | mtime advanced from 21:32 | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 21:37 | mtime advanced from 21:27 | Prior rotation's WAL backup |

**Snapshot epoch progression:** 800→830 (+30 in ~12 min. 3 rotations. ~10 epochs/rotation, consistent cadence).

### local-witness
**OBSERVED:** last_snapshot_epoch=830, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** Snapshot epoch advanced 810→830 (+20, 2 rotations) since pass 7.

**Note:** Witness at snapshot epoch 830 and morning-api at 830. The 10-epoch skew from pass 7 (api=800, witness=810) has resolved. Both nodes now converged at snapshot epoch 830.

**File inventory (~01:46:41Z):**

| File | Size | mtime (EDT) | Delta from pass 7 |
|------|------|-------------|-------------------|
| `persistence/state.snapshot` | 569 bytes | Jul 27 21:43 | mtime advanced 21:33→21:43; size unchanged (569) |
| `persistence/wal.log` | 379 bytes | Jul 27 21:43 | mtime advanced from 21:33 |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 21:38 | mtime advanced from 21:28 |

---

## Build Commit & Binary Freshness

**OBSERVED:** `71aa16b-dirty` on both nodes (unchanged since pass 1).
**EXPECTED (VERIFIED-BEHAVIOR.md):** Should match git HEAD.
**DEVIATION:** **Persistent.** 9 commits behind HEAD (`cb5d4b1`). All docs and test-only changes — no wire-format, codec, or protocol changes.

Git HEAD: `cb5d4b1` ("docs: Observer evidence corpus + Verifier missions 1 and 2")
Running binary: `71aa16b` ("wip: update Cargo.lock") + `-dirty`

**No change in HEAD since pass 28 (Jul 27).** Git has not advanced.

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
- ~84 KAD bootstrap warnings — benign, expected with `--no-mdns`. Every ~5 min. Last at 01:43:26Z (up from ~73 in pass 7).
- 2 startup WARNs at 14:48 EDT (InsufficientPeers on genesis gossip/publish) — expected initial conditions, all resolved on peer connect.
- **0 unexpected WARN/ERROR lines after filtering (healthy).**
- No sweep/evict/zombie activity.

**local-witness (/tmp/lw.log):**
- 118 WARN `insufficient balance` lines — the known redistribution rejection. Last at `2026-07-27T19:47:26Z`. No new rejections in ~6h. Count unchanged from passes 1-7 (118).
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

**morning-api (at ~01:46Z):**
```
Epoch complete epoch=833 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=834 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=835 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=836 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=837 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at ~1.02.

**local-witness (at ~01:46Z):**
```
Epoch complete epoch=833 balance_before=0 balance_after=0 ratio=1.13
Epoch complete epoch=834 balance_before=0 balance_after=0 ratio=1.13
Epoch complete epoch=835 balance_before=0 balance_after=0 ratio=1.13
Epoch complete epoch=836 balance_before=0 balance_after=0 ratio=1.13
Epoch complete epoch=837 balance_before=0 balance_after=0 ratio=1.13
```
Ratio: 1.12765 at socket query (continuing decline from 1.13044 at pass 7). Asymptotic decay continues.

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 9 commits behind HEAD `cb5d4b1`) | Low — docs + test changes only, no wire-format change | **Persistent** since pass 1 (Jul 27 18:48 EDT). Unchanged. |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint reads wrong path | **Persistent** since pass 1 (Jul 27 18:48 EDT). Verifier Mission 2 root cause identified. Unchanged. |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 (Jul 27 14:48 EDT). Locked at 20/0 since ~epoch 121. Unchanged. |

**Deviations resolved since pass 7:** None.

**Observations since pass 7:**
- Three-way epoch match **CLEAN** on both nodes (socket=837, grep=837, last_line=837).
- Snapshot epoch skew resolved: both nodes now converged at epoch 830 (was api=800, witness=810 in pass 7).
- No new log warnings or metrics anomalies.

---

## Delta from Pass 7 (~01:34:57Z → ~01:46:41Z Jul 28)

| Metric | Pass 7 (~01:34:57Z) | Pass 8 (~01:46:41Z) | Delta |
|--------|---------------------|---------------------|-------|
| Uptime (morning-api) | 24242s | 25054s | +812s (~13.5 min) |
| Uptime (witness) | 24310s | 25056s | +746s (~12.4 min) |
| Epoch (morning-api socket) | 819 | 837 | +18 |
| Epoch (witness socket) | 819 | 837 | +18 |
| Heartbeats (morning-api) | 2423 | 2504 | +81 |
| Heartbeats (witness) | 2432 | 2506 | +74 |
| Silence (morning-api) | 3s | 4s | +1s (normal variation) |
| Silence (witness) | 3s | 8s | +5s (normal variation) |
| Thickness (api) | 993.53 | 993.32 | -0.21 (expected decay) |
| Balance (api) | 20 | 20 | 0 |
| Balance (witness) | 0 | 0 | 0 |
| Nonce (api) | 120 | 120 | 0 |
| Nonce (witness) | 2 | 2 | 0 |
| Snapshot epoch (morning-api) | 800 | 830 | +30 (3 rotations) |
| Snapshot epoch (witness) | 810 | 830 | +20 (2 rotations) |
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

2. **witness sees morning-api balance as 0** (actual: 20). Persistent across all 38 passes. Mesh stays healthy. Functional impact: incorrect balance display on the witness's EconomicState endpoint. This is the supply conservation contradiction from VERIFIED-BEHAVIOR.md.

3. **MESH.md topology stale.** Header reads "No production nodes running" but both nodes have been running since 14:48 EDT Jul 27 (~7h of uptime). Same as passes 1-7.

4. **Stale sockets from historical test runs.** Multiple stale sockets under /tmp/ from prior test sessions. All are dead processes (verified via nc probe). Same set as prior passes. No new stale sockets observed.

---

## Raw Capture Bundle

Single-capture queries from ~01:46:01–01:46:41Z:

```
// === Three-way Epoch Match (01:46:41Z) ===
// morning-api: socket=837, grep=837, last line=837 — CLEAN MATCH
// witness:     socket=837, grep=837, last line=837 — CLEAN MATCH

// === GetNodeInfo (morning-api, ~01:46:01Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":25054,"build_commit":"71aa16b-dirty","thickness":993.3156037312166}

// === GetEpochState (morning-api, ~01:46:01Z) ===
{"type":"EpochState","epoch":836,"ratio":1.0195661318750366,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetPeers (morning-api, ~01:46:01Z) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":2504,"silence_secs":4,"is_dead":false,"queue_depth":0}]}

// === GetEconomicState (morning-api) — balance=20, nonce=120 ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetPersistenceState (morning-api) — wal_bytes=0, snapshot=830 ===
{"type":"PersistenceState","last_snapshot_epoch":830,"wal_bytes":0,"wal_entries":0}

// === GetHeight (morning-api) ===
{"type":"Height","height":1}

// === File inventory (morning-api, ~01:46:41Z EDT) ===
state.snapshot  894 bytes  mtime: Jul 27 21:42
wal.log         379 bytes  mtime: Jul 27 21:42
wal.wal.old     379 bytes  mtime: Jul 27 21:37

// === GetNodeInfo (witness, ~01:46:01Z) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":25056,"build_commit":"71aa16b-dirty"}

// === GetEpochState (witness, ~01:46:01Z) ===
{"type":"EpochState","epoch":836,"ratio":1.127651024690807,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetPeers (witness, ~01:46:01Z) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":2506,"silence_secs":8,"is_dead":false,"queue_depth":0}]}

// === GetEconomicState (witness) — own_balance=0, sees api balance=0 (persistent UNKNOWN divergence) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// === GetPersistenceState (witness) — wal_bytes=0, snapshot=830 ===
{"type":"PersistenceState","last_snapshot_epoch":830,"wal_bytes":0,"wal_entries":0}

// === File inventory (witness, ~01:46:41Z EDT) ===
state.snapshot  569 bytes  mtime: Jul 27 21:43
wal.log         379 bytes  mtime: Jul 27 21:43
wal.wal.old     379 bytes  mtime: Jul 27 21:38

// === Git HEAD ===
cb5d4b1 docs: Observer evidence corpus + Verifier missions 1 and 2

// === Metrics (morning-api last 3 at ~01:46:46Z) ===
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s

// === Metrics (witness last 3 at ~01:46:43Z) ===
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```
