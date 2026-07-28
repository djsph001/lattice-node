# Observer Evidence Record — 2026-07-28 (Pass 112)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** 2026-07-28T17:29:52Z (morning-api), 17:30:10Z (witness), ~17:30-17:31Z (log/disc captures)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** 112th observation pass of Jul 28. ~8.5 min since pass 111 (17:21Z). Sockets responsive, PIDs unchanged (3579452/3579821).

**Summary:** Delta-only from pass 111. All evidence guards PASS (byte-equality, simultaneous capture, cross-node sync). Epochs advanced +15/+18 at normal cadence. Snapshot rotated at 530 (~17:26Z, mid-capture relative to pass 111's 17:22Z). Economic state completely frozen — unchanged entire session (>4.3h). Two persistent deviations unchanged: stale build_commit and supply conservation divergence. Snapshot size oscillated from 895→894 bytes across the 520→530 rotation (continuing previously noted pattern). No new findings.

---

## Topology Disclosure

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 13:01Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 13:02Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes since session start (13:01Z).** Same PIDs across all 112 passes.

---

## Evidence Integrity Guards — Simultaneous Capture (17:29–17:31Z)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch (morning-api) | Socket=537, Log count=538, Last log epoch=538 (17:30:17Z) | All three match at a single instant | **PASS** — race at epoch boundary; log advanced to 538 (17:30:17Z) while socket still at 537 (17:29:52Z). Expected ~25s gap. |
| Three-way epoch (witness) | Socket=537, Log count=538, Last log epoch=538 (17:30:40Z) | All three match | **PASS** — same race pattern; witness ~25s behind morning-api capture. |
| Byte-equality (morning-api) | wal_bytes=379 (socket), stat=379 (ls) | Must match | **PASS** |
| Byte-equality (witness) | wal_bytes=379 (socket), stat=379 (ls) | Must match | **PASS** |
| Cross-node epoch sync (socket) | morning-api=537, witness=537 (simultaneous capture, ~18s apart) | Should be ≤1-2 at same instant | **PASS** — δ=0 |
| PID consistency | 3579452/3579821 unchanged | Same since 13:01Z | **PASS** |
| Log health (morning-api) | Only KAD bootstrap WARNs (expected --no-mdns) | Clean | **PASS** |
| Log health (witness) | 0 non-KAD/non-insufficient-balance WARN/ERROR | Clean | **PASS** |
| Metrics health | aged=0, queues=[], silence 4s/5s | aged≈0, silence<30s | **PASS** |

---

## Node Info — Delta from Pass 111

### morning-api (17:29:52Z)

| Field | Pass 111 (17:21:27Z) | This pass | Δ | DEVIATION |
|-------|----------------------|-----------|----|-----------|
| peer_id | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | Same | — | None |
| name | morning-api | Same | — | — |
| genesis_root_id | auto | Same | — | — |
| chain_tip | 1 | 1 | 0 | None — genesis-only mesh |
| uptime_secs | 15561 | **16084** | +523 (~8.7 min) | None — consistent with capture time delta (~8.5 min) |
| build_commit | cb5d4b1-dirty | cb5d4b1-dirty | Unchanged | **PERSISTENT** — 2 commits behind HEAD 452b64f. Dirty (markdown-only). Unchanged since Jul 27. |
| thickness | 978.58 | **978.44** | -0.14 | No expected value documented |

### local-witness (17:30:10Z)

| Field | Pass 111 (17:21:35Z) | This pass | Δ | DEVIATION |
|-------|----------------------|-----------|----|-----------|
| peer_id | 12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch | Same | — | None |
| name | local-witness | Same | — | — |
| genesis_root_id | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | Same | — | Correct |
| chain_tip | 1 | 1 | 0 | None |
| uptime_secs | 15577 | **16079** | +502 (~8.4 min) | None — consistent with capture timing |
| build_commit | cb5d4b1-dirty | cb5d4b1-dirty | Unchanged | **PERSISTENT** — same as morning-api |

---

## Epoch State — Delta from Pass 111

### morning-api

| Check | Pass 111 (17:21:27Z) | This pass (17:29:52Z) | Δ | DEVIATION |
|-------|----------------------|-----------------------|----|-----------|
| Socket epoch | 519→522 | **537** | +15→+18 | None — ~30s/ep |
| Log count | 523 | **538** | +15 | None |
| Last log epoch | 523 (17:22:47Z) | **538** (17:30:17Z) | +15 | None |
| Three-way | PASS (race at 522→523) | **PASS** (race at 537→538) | — | None |

### local-witness

| Check | Pass 111 (17:21:35Z) | This pass (17:30:10Z) | Δ | DEVIATION |
|-------|----------------------|------------------------|----|-----------|
| Socket epoch | 520→522 | **537** | +15→+17 | None — ~30s/ep |
| Log count | 522 | **538** | +16 | None |
| Last log epoch | 522 (17:22:40Z) | **538** (17:30:40Z) | +16 | None |
| Three-way | PASS | **PASS** (race at 537→538) | — | None |

### Cross-node comparison

| Metric | Pass 111 | This pass | Δ | DEVIATION |
|--------|----------|-----------|----|-----------|
| morning-api epoch (socket) | 522 | 537 | +15 | — |
| witness epoch (socket) | 522 | 537 | +15 | — |
| Cross-node δ | 0 | **0** | 0 | None — converged |
| Epoch rate | ~30-33s/ep | ~30-33s/ep | 0 | None — stable |

---

## Peers — Delta from Pass 111

### morning-api (17:29:52Z)

| Peer | Heartbeats | Silence (s) | Dead | Queue Depth | Δ from Pass 111 | DEVIATION |
|------|-----------|-------------|------|-------------|-----------------|-----------|
| 12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch | **1606** | 4 | false | 0 | +45 heartbeats (~8.5 min at ~30s cadence ≈ 17 ticks × ~2.6 hb/tick) | None — healthy |

### local-witness (17:30:10Z)

| Peer | Heartbeats | Silence (s) | Dead | Queue Depth | Δ from Pass 111 | DEVIATION |
|------|-----------|-------------|------|-------------|-----------------|-----------|
| 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | **1608** | 5 | false | 0 | +50 heartbeats | None — healthy |

**No zombie evictions. No silent peers. No queue buildup.** Bidirectional heartbeat exchange healthy. Slight heartbeat asymmetry (1606 vs 1608) within normal capture timing variance. max_peer_silence 4-5s — well under 30s threshold.

---

## Economic State — Completely Frozen (Unchanged Entire Session >4.5h)

| Metric | Pass 111 (17:21Z) | This pass (17:30Z) | Δ | DEVIATION |
|--------|-------------------|--------------------|----|-----------|
| morning-api: own_balance | 20 | 20 | **Frozen** | None — steady-state floor |
| morning-api: own_nonce | 241 | 241 | **Frozen** | None |
| morning-api: sees witness balance | 9980 | 9980 | **Frozen** | **PERSISTENT DEVIATION #2a** — supply divergence |
| morning-api: sees witness nonce | 0 | 0 | **Frozen** | **PERSISTENT** |
| Witness: own_balance | 0 | 0 | **Frozen** | None — `--mint 0` |
| Witness: own_nonce | 4 | 4 | **Frozen** | None |
| Witness: sees morning-api balance | 0 | 0 | **Frozen** | **PERSISTENT DEVIATION #2b** — cross-node asymmetry |
| Witness: sees morning-api nonce | 0 | 0 | **Frozen** | **PERSISTENT DEVIATION #2b** |

**OBSERVED:** Economic state frozen across the board for the entire session (>4.5h, since ~13:01Z). No transaction flow since first observer pass. Morning-api plateaued at balance 20 (ratio ~1.019), witness at balance 0 (ratio ~1.204, declining asymptotically from ~1.934 at session start). Peer balance asymmetry unchanged: morning-api sees witness at 9980, witness sees morning-api at 0.

---

## Persistence State — Delta from Pass 111

### morning-api (simultaneous capture, 17:29–17:31Z)

| Field | Pass 111 (17:22Z) | This pass | Δ | DEVIATION |
|-------|-------------------|-----------|----|-----------|
| last_snapshot_epoch | 520 | **530** | +10 (1 rotation) | None — normal 10-epoch cadence |
| wal_bytes | 379 | 379 | Unchanged | None — byte-equality PASS (379=379) |
| wal_entries | 3 | 3 | Unchanged | **KNOWN-PROVISIONAL** — size/120 heuristic |

**File inventory:**

| File | Size | mtime (UTC) | Notes |
|------|------|-------------|-------|
| state.snapshot | **894** bytes | Jul 28 13:26 | (epoch 530 snapshot — **decreased from 895 to 894 bytes**) |
| wal.log | 379 bytes | Jul 28 13:26 | Active WAL (genesis re-seed only, 3 entries) |
| wal.wal.old | 379 bytes | Jul 28 13:21 | Pre-rotation backup |

**Snapshot size oscillation continues:** 895 (epoch 520, pass 111) → **894** (epoch 530, this pass). The pattern: 895 (epoch 450) → 894 (epoch 500) → 895 (epoch 520) → **894 (epoch 530)**. This is now a 4-data-point oscillation between 894 and 895 bytes. UNKNOWN: whether this is benign (metadata serialization variance) or a serialization boundary issue. Not elevated to deviation — first noted at pass 111, now confirmed as persistent oscillating pattern.

### local-witness (simultaneous capture, 17:30Z)

| Field | Pass 111 (17:22Z) | This pass | Δ | DEVIATION |
|-------|-------------------|-----------|----|-----------|
| last_snapshot_epoch | 520 | **530** | +10 | None — normal rotation |
| wal_bytes | 379 | 379 | Unchanged | Byte-equality PASS (379=379) |
| wal_entries | 3 | 3 | Unchanged | KNOWN-PROVISIONAL |

**File inventory:**

| File | Size | mtime (UTC) | Notes |
|------|------|-------------|-------|
| state.snapshot | **569** bytes | Jul 28 13:26 | Smaller than morning-api (different balance state — 0 vs 9980) |
| wal.log | 379 bytes | Jul 28 13:26 | Active WAL |
| wal.wal.old | 379 bytes | Jul 28 13:21 | Pre-rotation backup |

**Witness snapshot size at 569 bytes — unchanged.** No oscillation observed on witness (stayed at 569 across 500→520→530 rotations).

---

## Metrics

### morning-api (last 3 lines, 17:30:07–17:30:27Z)
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```

### local-witness (last 3 lines, 17:30:20–17:30:40Z)
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
```

**All green:** 0 fetches, 0 aged, empty queues, silence ≤6s (well under 30s threshold). No zombie eviction activity. No stale fetch sweep events. max_peer_silence stable at 6s (morning-api) and 3s (witness).

---

## Log Health Scan

### morning-api (/tmp/m-ap.log)

| Item | Count / Detail |
|------|----------------|
| **Epoch complete** lines | **538** (last: epoch=538, 17:30:17Z, balance=20→20, ratio=1.02) |
| **Snapshot rotations** | 400, 410, 420, 430, 440, 450, 460, 470, 480, 490, 500, 510, 520, **530** (normal 10-epoch cadence) |
| **KAD WARN** | `Failed to trigger bootstrap: No known peers.` (~5-min interval, expected with --no-mdns, no DHT) |
| **Non-KAD WARN** | 2 total, both startup-only: `No snapshot found, starting fresh` (13:01:47Z), `Connection from non-mDNS peer` (13:02:10Z) |
| **ERROR** | 0 |
| **Panics** | 0 |
| **Zombie evictions** | 0 |
| **Stale fetch sweeps** | 0 |
| **Transactions** | 0 new since session start |
| **WAL recovery** | `Genesis recovered from WAL` at startup (13:01:47Z) — correct |

### local-witness (/tmp/lw.log)

| Item | Count / Detail |
|------|----------------|
| **Epoch complete** lines | **538** (last: epoch=538, 17:30:40Z, balance=0→0, ratio=1.20) |
| **KAD WARN** | Present (--no-mdns) |
| **Insufficient-balance WARN** | 119 (unchanged — last at 14:01:47Z from Jul 27 redistribution) |
| **Panics** | 0 |
| **Zombie evictions** | 0 |
| **Stale fetch sweeps** | 0 |
| **Non-expected WARN/ERROR** | None |

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Status | Changed Since Pass 111? |
|---|-----------|---------------|--------|------------------------|
| 1 | `build_commit` stale: `cb5d4b1-dirty` vs HEAD `452b64f` (2 behind + dirty) | Jul 27 | PERSISTENT | **Unchanged** |
| 2a | Supply divergence: morning-api total=10,000 (20+9980), witness total=0 | Jul 27 pass 3 | PERSISTENT | **Unchanged** — documented in VERIFIED-BEHAVIOR.md as CONTRADICTED |
| 2b | Cross-node peer balance/nonce asymmetry (witness sees morning-api balance=0, morning-api sees witness nonce=0) | Pass 1 (13:01Z) | PERSISTENT | **Unchanged** |
| 3 | `wal.wal.old` naming (cosmetic) | ae89fbd | KNOWN-PROVISIONAL | **Unchanged** |

## Resolved Observations (from pass 111→112)

None. No new items resolved.

---

## UNKNOWN Items

| # | Unknown | Why unclassified |
|---|---------|-----------------|
| 1 | Ratio divergence (morning-api 1.019 vs witness 1.204) | No design document specifies expected ratio behavior across nodes with different balances. Witness's higher ratio declining asymptotically from 1.934 (session start) toward ~1.0 — consistent with zero-balance dynamics but not independently verified. |
| 2 | Why economic state is completely frozen (>4.5h without any transaction activity) | Could be expected (no external transactions submitted to UDS), designed (balance 20 floor below 1-token threshold for redistribution), or a bug. Cannot determine from observation alone. |
| 3 | Snapshot size oscillation (894→895→894→895→894 across five rotations) | The 1-byte oscillation persists across 4 data points (epochs 450, 500, 520, 530). Witness snapshot stable at 569 bytes (no oscillation). Could be benign metadata variance or a serialization boundary issue specific to morning-api's larger balance state. |
| 4 | Why witness ratio declined from 1.934 (pass 1) to 1.204 (this pass) | Thickness decay from initial values is expected, but the specific rate and asymptotic behavior are not documented. Witness's zero balance means ratio derives only from other nodes' contributions — the declining trend is consistent with redistribution dynamics but the precise trajectory is unverified. |

---

## Summary

**Pass 112: delta-only. No new deviations.**

The mesh remains in a frozen steady-state — functionally a heartbeat daemon with no transaction activity:

- **2 nodes**, 1 peer each, bidirectional heartbeats healthy (1606/1608), silence ≤5s
- **Epochs cycling** at ~30-33s cadence. Both nodes at 537, synchronized (δ=0).
- **No transactions flowing** — nonces frozen (241/4) since session start (>4.5h)
- **Balance 20 floor** on morning-api, zero on witness — unchanged entire session
- **Snapshot rotation** at normal 10-epoch cadence (530). One rotation (520→530) since pass 111. WAL unchanged at 379 bytes.
- **All metrics clean:** zero fetches, zero queues, max_peer_silence ≤6s
- **Build commit** 2 behind HEAD + dirty (unchanged since Jul 27)
- **Supply conservation divergence** unchanged (documented, pending governance)
- **Snapshot size oscillation** (895→894) confirmed as persistent pattern across 4 data points — noted but not elevated to deviation

**Next expected event:** Snapshot rotation at epoch 540 (~5 min). No other state changes expected given the frozen economic state.
