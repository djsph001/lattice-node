# Observer Evidence Record — 2026-07-28 (Pass 110)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** 2026-07-28T17:13:20Z (initial socket queries), 17:13:35Z (simultaneous guard capture)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** 110th observation pass of Jul 28. ~23 min since pass 109 (16:50:10Z). Sockets responsive, PIDs unchanged.

**Summary:** Delta-only from pass 109. All evidence guards PASS. Epochs advanced +47/+46. Snapshot rotated from 450→500. Economic state completely frozen — unchanged entire session (>4h). No transaction activity since redistribution phase ended before Jul 27 session recording. Mesh is a steady-state heartbeat daemon with no transaction flow. Two persistent deviations unchanged: stale build_commit and supply conservation divergence.

---

## Topology Disclosure

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 13:01Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 13:02Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes since session start (13:01Z).** Same PIDs across all 110 passes.

---

## Evidence Integrity Guards — Simultaneous Capture (17:13:35Z)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch (morning-api) | Socket=504, Log count=504, Last log epoch=504 (17:13:17Z) | All three match at a single instant | **PASS** |
| Byte-equality (morning-api) | wal_bytes=379 (socket), stat=379 (ls) | Must match | **PASS** |
| Byte-equality (witness) | wal_bytes=379 (socket), stat=379 (ls) | Must match | **PASS** |
| Cross-node epoch sync (socket) | morning-api=504, witness=503 (same batch, ~30s apart) | Should be ≤1-2 at same instant | **PASS** — normal drift within capture gap |
| PID consistency | 3579452/3579821 unchanged | Same since 13:01Z | **PASS** |
| Log health (morning-api) | 2 non-KAD WARN (both startup-only: "No snapshot found, starting fresh" + "Connection from non-mDNS peer") | Clean expected warnings only | **PASS** |
| Log health (witness) | 0 non-KAD/non-insufficient-balance WARN/ERROR | Clean | **PASS** |
| Metrics health | aged=0, queues=[], silence 8s/7s | aged≈0, silence<30s | **PASS** |

---

## Node Info — Delta from Pass 109

### morning-api (17:13:20Z)

| Field | Pass 109 (16:50:10Z) | This pass | Δ | DEVIATION |
|-------|----------------------|-----------|----|-----------|
| peer_id | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | Same | — | None |
| name | morning-api | Same | — | — |
| genesis_root_id | auto | Same | — | — |
| chain_tip | 1 | 1 | 0 | None — genesis-only mesh |
| uptime_secs | 13702 | 15097 | +1395 (~23.3 min) | None — consistent with capture time delta (~23 min) |
| build_commit | cb5d4b1-dirty | cb5d4b1-dirty | Unchanged | **PERSISTENT** — 2 commits behind HEAD 452b64f. Dirty (markdown-only). Unchanged since Jul 27. |
| thickness | 979.07 | 978.70 | -0.37 | No expected value documented |

### local-witness (17:13:35Z)

| Field | Pass 109 (16:50:27Z) | This pass | Δ | DEVIATION |
|-------|----------------------|-----------|----|-----------|
| peer_id | 12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch | Same | — | None |
| name | local-witness | Same | — | — |
| genesis_root_id | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | Same | — | Correct |
| chain_tip | 1 | 1 | 0 | None |
| uptime_secs | 13704 | 15100 | +1396 (~23.3 min) | None |
| build_commit | cb5d4b1-dirty | cb5d4b1-dirty | Unchanged | **PERSISTENT** — same as morning-api |

---

## Epoch State — Delta from Pass 109

### morning-api

| Check | Pass 109 (16:50:10Z) | This pass (17:13:20Z) | Δ | DEVIATION |
|-------|----------------------|-----------------------|----|-----------|
| Socket epoch | 457 | **504** | +47 | None — ~29.8s/epoch (consistent with ~30s cadence) |
| Log count | 459 | **504** | +45 | None — 2 fewer in log query (pass 109's count included 2 from capture gap) |
| Last log epoch | 459 (16:50:47Z) | **504** (17:13:17Z) | — | None |
| Three-way | PASS (459=459=459 simultaneous) | **PASS** (504=504=504) | — | None |

### local-witness

| Check | Pass 109 (16:50:30Z) | This pass (17:13:35Z) | Δ | DEVIATION |
|-------|----------------------|------------------------|----|-----------|
| Socket epoch | 457 | **503** | +46 | None — ~30.2s/epoch (consistent) |
| Log count | 458 | **503** | +45 | None — 1 less in log query due to capture gap |
| Last log epoch | 458 (16:50:40Z) | **503** (17:13:10Z) | — | None |
| Three-way | PASS (457→458 gap) | **PASS** (503=503=503) | — | None |

### Cross-node comparison

| Metric | Pass 109 | This pass | Δ | DEVIATION |
|--------|----------|-----------|----|-----------|
| morning-api epoch (socket) | 457 | 504 | +47 | — |
| witness epoch (socket) | 457 | 503 | +46 | — |
| Cross-node δ | 0 | **1** | +1 | None — normal drift within capture gap (morning-api ahead by ~30s) |
| Epoch rate | ~30s/ep | ~30s/ep | 0 | None — stable |

---

## Peers — Delta from Pass 109

### morning-api (17:13:20Z)

| Peer | Heartbeats | Silence (s) | Dead | Queue Depth | Δ from Pass 109 | DEVIATION |
|------|-----------|-------------|------|-------------|-----------------|-----------|
| 12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch | **1510** | 8 | false | 0 | +142 heartbeats (~23 min at 30s cadence ≈ ~46 ticks × ~3 heartbeats/tick) | None — healthy |

### local-witness (17:13:35Z)

| Peer | Heartbeats | Silence (s) | Dead | Queue Depth | Δ from Pass 109 | DEVIATION |
|------|-----------|-------------|------|-------------|-----------------|-----------|
| 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | **1510** | 7 | false | 0 | +139 heartbeats | None — healthy |

**No zombie evictions. No silent peers. No queue buildup.** Bidirectional heartbeat exchange healthy. Both nodes report identical heartbeat counts (1510), indicating symmetric mesh health.

---

## Economic State — Completely Frozen (Unchanged Entire Session >4.3h)

| Metric | Pass 109 (16:50Z) | This pass (17:13Z) | Δ | DEVIATION |
|--------|-------------------|--------------------|----|-----------|
| morning-api: own_balance | 20 | 20 | **Frozen** | None — steady-state floor |
| morning-api: own_nonce | 241 | 241 | **Frozen** | None |
| morning-api: sees witness balance | 9980 | 9980 | **Frozen** | **PERSISTENT DEVIATION #2a** — supply divergence (documented in VERIFIED-BEHAVIOR.md) |
| morning-api: sees witness nonce | 0 | 0 | **Frozen** | **PERSISTENT DEVIATION #2b** |
| Witness: own_balance | 0 | 0 | **Frozen** | None — `--mint 0` |
| Witness: own_nonce | 4 | 4 | **Frozen** | None — unchanged entire session |
| Witness: sees morning-api balance | 0 | 0 | **Frozen** | **PERSISTENT DEVIATION #2b** |
| Witness: sees morning-api nonce | 0 | 0 | **Frozen** | **PERSISTENT DEVIATION #2b** |

**OBSERVED:** Economic state frozen across the board for the entire session (>4.3 hours, since ~13:01Z). No transaction flow since first observer pass. Morning-api plateaued at balance 20 (ratio ~1.019), witness at balance 0 (ratio ~1.22). Peer balance asymmetry unchanged: morning-api sees witness at 9980, witness sees morning-api at 0.

**Note on total supply divergence:** morning-api's ledger accounts for 10,000 total (20 own + 9980 peer), but initial mint was 5,000. Witness's ledger accounts for 0 total. This is the known supply conservation contradiction (VERIFIED-BEHAVIOR.md, CONTRADICTED). Neither number has changed since the Jul 27 redistribution phase.

---

## Persistence State — Delta from Pass 109

### morning-api (simultaneous capture, 17:13:35Z)

| Field | Pass 109 (16:50Z) | This pass | Δ | DEVIATION |
|-------|-------------------|-----------|----|-----------|
| last_snapshot_epoch | 450 | **500** | +50 (5 rotations: 460, 470, 480, 490, 500) | None — normal 10-epoch cadence |
| wal_bytes | 379 | 379 | Unchanged | None — byte-equality PASS (379=379) |
| wal_entries | 3 | 3 | Unchanged | **KNOWN-PROVISIONAL** — size/120 heuristic |

**File inventory:** Snapshot rotated since pass 109 (mtime updated from 12:46 EDT to 13:11 EDT).

| File | Size | mtime (EDT) | Notes |
|------|------|-------------|-------|
| state.snapshot | 894 bytes | Jul 28 13:11 | (epoch 500 snapshot — 1 byte smaller than 895 at 450) |
| wal.log | 379 bytes | Jul 28 13:11 | Active WAL (genesis re-seed only, 3 entries) |
| wal.wal.old | 379 bytes | Jul 28 13:06 | Pre-rotation backup |

### local-witness (17:13:35Z)

| Field | Pass 109 (16:50Z) | This pass | Δ | DEVIATION |
|-------|-------------------|-----------|----|-----------|
| last_snapshot_epoch | 450 | **500** | +50 | None — normal rotation |
| wal_bytes | 379 | 379 | Unchanged | Byte-equality PASS (379=379) |
| wal_entries | 3 | 3 | Unchanged | KNOWN-PROVISIONAL |

**File inventory:**

| File | Size | mtime (EDT) | Notes |
|------|------|-------------|-------|
| state.snapshot | 569 bytes | Jul 28 13:11 | Smaller than morning-api (different balance state — 0 vs 9980) |
| wal.log | 379 bytes | Jul 28 13:11 | Active WAL |
| wal.wal.old | 379 bytes | Jul 28 13:06 | Pre-rotation backup |

---

## Metrics

### morning-api (last 3 lines, 17:13:17–17:13:47Z approx)
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=8s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=8s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=8s
```

### local-witness (last 3 lines, 17:13:10–17:13:40Z approx)
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=7s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=7s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=7s
```

**All green:** 0 fetches, 0 aged, empty queues, silence ≤8s (well under 30s threshold). No zombie eviction activity. No stale fetch sweep events. Same pattern since ~pass 97 (14:08Z). max_peer_silence slightly increased from 6s/3s (pass 109) to 8s/7s — minor variance, no concern.

---

## Log Health Scan

### morning-api (/tmp/m-ap.log)

| Item | Count / Detail |
|------|----------------|
| **Epoch complete** lines | **504** (last: epoch=504, 17:13:17Z, balance=20→20, ratio=1.02) |
| **Snapshot rotations** | 400, 410, 420, 430, 440, 450, 460, 470, 480, 490, **500** (normal 10-epoch cadence) |
| **KAD WARN** | `Failed to trigger bootstrap: No known peers.` (~5-min interval, expected with --no-mdns, no DHT) |
| **Non-KAD WARN** | 2 total, both startup-only: `No snapshot found, starting fresh` (13:01:47Z), `Connection from non-mDNS peer` (13:02:10Z) |
| **ERROR** | 0 |
| **Panics** | 0 |
| **Zombie evictions** | 0 |
| **Stale fetch sweeps** | 0 |
| **Transactions** | 0 new since session start |

### local-witness (/tmp/lw.log)

| Item | Count / Detail |
|------|----------------|
| **Epoch complete** lines | **503** (last: epoch=503, 17:13:10Z, balance=0→0, ratio=1.22) |
| **KAD WARN** | Present (--no-mdns) |
| **Insufficient-balance WARN** | 119 (unchanged — last at 14:01:47Z from Jul 27 redistribution) |
| **Panics** | 0 |
| **Zombie evictions** | 0 |
| **Stale fetch sweeps** | 0 |
| **Non-expected WARN/ERROR** | None |

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Status | Changed Since Pass 109? |
|---|-----------|---------------|--------|------------------------|
| 1 | `build_commit` stale: `cb5d4b1-dirty` vs HEAD `452b64f` (2 behind + dirty) | Jul 27 | PERSISTENT | **Unchanged** |
| 2a | Supply divergence: morning-api total=10,000 (20+9980), witness total=0 | Jul 27 pass 3 | PERSISTENT | **Unchanged** — documented in VERIFIED-BEHAVIOR.md as CONTRADICTED |
| 2b | Cross-node peer balance/nonce asymmetry (witness sees morning-api balance=0, morning-api sees witness nonce=0) | Pass 1 (13:01Z) | PERSISTENT | **Unchanged** |
| 3 | `wal.wal.old` naming (cosmetic) | ae89fbd | KNOWN-PROVISIONAL | **Unchanged** |

## Resolved Observations (from pass 109→110)

None. No new items resolved.

## UNKNOWN Items

| # | Unknown | Why unclassified |
|---|---------|-----------------|
| 1 | Ratio divergence (morning-api 1.019 vs witness 1.241) | No design document specifies expected ratio behavior across nodes with different balances. Witness's higher ratio may be correct given its zero balance. |
| 2 | Why economic state is completely frozen (>4.3h without any transaction activity) | Could be expected (no external transactions submitted to UDS), designed (balance 20 floor below 1-token threshold for redistribution), or a bug. Cannot determine from observation alone. |
| 3 | Whether `last_snapshot_epoch=500` snapshot correctly captured the economic state | No on-disk snapshot content verification — observer only checks existence and size. Size changed from 895 bytes (epoch 450) to 894 bytes (epoch 500) — a 1-byte decrease. Could be benign (no new state to serialize reduces metadata) or a silent error. |
| 4 | Why witness requested blocks show `redistributed_to=1` (first epoch only) while morning-api also shows `redistributed_to=1` | Both nodes report redistributed_to=1, meaning only epoch 1 (genesis) triggered a redistribution. After that, the floor balance of 20 on morning-api and 0 on witness produced no redistributable surplus. This is consistent with known behavior but not independently verified. |

---

## Summary

**Pass 110: delta-only. No new deviations.**

The mesh remains in a frozen steady-state — functionally a heartbeat daemon with no transaction activity:

- **2 nodes**, 1 peer each, bidirectional heartbeats healthy (1510/1510), silence ≤8s
- **Epochs cycling** at ~30s cadence. Morning-api at 504, witness at 503 (δ=1 due to capture gap).
- **No transactions flowing** — nonces frozen (241/4) since session start (>4.3h)
- **Balance 20 floor** on morning-api, zero on witness — unchanged entire session
- **Snapshot rotation** at normal 10-epoch cadence (500). Five rotations (460→500) since pass 109. WAL unchanged at 379 bytes.
- **All metrics clean:** zero fetches, zero queues, max_peer_silence ≤8s
- **Build commit** 2 behind HEAD + dirty (unchanged since Jul 27)
- **Supply conservation divergence** unchanged (documented, pending governance)

**Next expected event:** Snapshot rotation at epoch 510 (~5 min). No other state changes expected given the frozen economic state.
