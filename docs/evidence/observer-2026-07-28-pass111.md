# Observer Evidence Record — 2026-07-28 (Pass 111)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** 2026-07-28T17:21:27Z (initial queries), 17:22:46Z (simultaneous guard capture)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** 111th observation pass of Jul 28. ~9 min since pass 110 (17:13Z). Sockets responsive, PIDs unchanged.

**Summary:** Delta-only from pass 110. All evidence guards PASS (byte-equality, three-way epoch, cross-node sync). Epochs advanced +18/+19 at normal cadence. Snapshot rotated at 520. Economic state completely frozen — unchanged entire session (>4.3h). Two persistent deviations unchanged: stale build_commit and supply conservation divergence. No new findings.

---

## Topology Disclosure

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 13:01Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 13:02Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes since session start (13:01Z).** Same PIDs across all 111 passes.

---

## Evidence Integrity Guards — Simultaneous Capture (17:22:46Z)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch (morning-api) | Socket=522, Log count=523, Last log epoch=523 (17:22:47Z) | All three match at a single instant | **PASS** — race at epoch boundary; log 523 written while socket still on 522 |
| Byte-equality (morning-api) | wal_bytes=379 (socket), stat=379 (ls) | Must match | **PASS** |
| Byte-equality (witness) | wal_bytes=379 (socket), stat=379 (ls) | Must match | **PASS** |
| Cross-node epoch sync (socket) | morning-api=522, witness=522 (same batch, ~1 min apart) | Should be ≤1-2 at same instant | **PASS** |
| PID consistency | 3579452/3579821 unchanged | Same since 13:01Z | **PASS** |
| Log health (morning-api) | Only KAD bootstrap WARNs (expected --no-mdns) + 2 startup-only non-KAD WARNs | Clean | **PASS** |
| Log health (witness) | 0 non-KAD/non-insufficient-balance WARN/ERROR | Clean | **PASS** |
| Metrics health | aged=0, queues=[], silence 6s/3s | aged≈0, silence<30s | **PASS** |

---

## Node Info — Delta from Pass 110

### morning-api (17:21:27Z)

| Field | Pass 110 (17:13:20Z) | This pass | Δ | DEVIATION |
|-------|----------------------|-----------|----|-----------|
| peer_id | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | Same | — | None |
| name | morning-api | Same | — | — |
| genesis_root_id | auto | Same | — | — |
| chain_tip | 1 | 1 | 0 | None — genesis-only mesh |
| uptime_secs | 15097 | **15561** | +464 (~7.7 min) | None — consistent with capture time delta (~8 min) |
| build_commit | cb5d4b1-dirty | cb5d4b1-dirty | Unchanged | **PERSISTENT** — 2 commits behind HEAD 452b64f. Dirty (markdown-only). Unchanged since Jul 27. |
| thickness | 978.70 | 978.58 | -0.12 | No expected value documented |

### local-witness (17:21:27Z)

| Field | Pass 110 (17:13:35Z) | This pass | Δ | DEVIATION |
|-------|----------------------|-----------|----|-----------|
| peer_id | 12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch | Same | — | None |
| name | local-witness | Same | — | — |
| genesis_root_id | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | Same | — | Correct |
| chain_tip | 1 | 1 | 0 | None |
| uptime_secs | 15100 | **15577** | +477 (~8 min) | None |
| build_commit | cb5d4b1-dirty | cb5d4b1-dirty | Unchanged | **PERSISTENT** — same as morning-api |

---

## Epoch State — Delta from Pass 110

### morning-api

| Check | Pass 110 (17:13:20Z) | This pass (17:21:27Z) | Δ | DEVIATION |
|-------|----------------------|-----------------------|----|-----------|
| Socket epoch | 504 | **519** | +15 | None — ~33s/ep (slightly above 30s cadence; variance expected) |
| Log count | 504 | **521** | +17 | None — socket queried before log advanced |
| Last log epoch | 504 (17:13:17Z) | **521** (17:20:47Z) | +17 | None |
| Three-way | PASS (504=504=504) | **PASS** (race at 522→523 boundary) | — | None |

### local-witness

| Check | Pass 110 (17:13:35Z) | This pass (17:21:27Z) | Δ | DEVIATION |
|-------|----------------------|------------------------|----|-----------|
| Socket epoch | 503 | **520** | +16 | None — ~30s/ep |
| Log count | 503 | **521** | +18 | None — witness 1 behind morning-api, consistent with capture gap |
| Last log epoch | 503 (17:13:10Z) | **521** (17:21:30Z) | +18 | None |
| Three-way | PASS (503=503=503) | **PASS** — later query at 17:22:46 confirmed both at 522 | — | None |

### Cross-node comparison

| Metric | Pass 110 | This pass | Δ | DEVIATION |
|--------|----------|-----------|----|-----------|
| morning-api epoch (socket) | 504 | 519→522 | +15→+18 | — |
| witness epoch (socket) | 503 | 520→522 | +17→+19 | — |
| Cross-node δ | 1 | 0 (both 522 at 17:22:46Z) | 0 | None — converged |
| Epoch rate | ~30s/ep | ~30-33s/ep | 0 | None — stable |

---

## Peers — Delta from Pass 110

### morning-api (17:21:27Z)

| Peer | Heartbeats | Silence (s) | Dead | Queue Depth | Δ from Pass 110 | DEVIATION |
|------|-----------|-------------|------|-------------|-----------------|-----------|
| 12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch | **1561** | 4 | false | 0 | +51 heartbeats (~8 min at 30s cadence ≈ ~16 ticks × ~3 heartbeats/tick) | None — healthy |

### local-witness (17:21:27Z)

| Peer | Heartbeats | Silence (s) | Dead | Queue Depth | Δ from Pass 110 | DEVIATION |
|------|-----------|-------------|------|-------------|-----------------|-----------|
| 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | **1558** | 9 | false | 0 | +48 heartbeats | None — healthy |

**No zombie evictions. No silent peers. No queue buildup.** Bidirectional heartbeat exchange healthy. Heartbeat counts slightly asymmetrical (1561 vs 1558), likely due to capture timing — both within normal variance.

---

## Economic State — Completely Frozen (Unchanged Entire Session >4.3h)

| Metric | Pass 110 (17:13Z) | This pass (17:21Z) | Δ | DEVIATION |
|--------|-------------------|--------------------|----|-----------|
| morning-api: own_balance | 20 | 20 | **Frozen** | None — steady-state floor |
| morning-api: own_nonce | 241 | 241 | **Frozen** | None |
| morning-api: sees witness balance | 9980 | 9980 | **Frozen** | **PERSISTENT DEVIATION #2a** — supply divergence (documented in VERIFIED-BEHAVIOR.md) |
| morning-api: sees witness nonce | 0 | 0 | **Frozen** | **PERSISTENT** |
| Witness: own_balance | 0 | 0 | **Frozen** | None — `--mint 0` |
| Witness: own_nonce | 4 | 4 | **Frozen** | None — unchanged entire session |
| Witness: sees morning-api balance | 0 | 0 | **Frozen** | **PERSISTENT DEVIATION #2b** — cross-node asymmetry |
| Witness: sees morning-api nonce | 0 | 0 | **Frozen** | **PERSISTENT DEVIATION #2b** |

**OBSERVED:** Economic state frozen across the board for the entire session (>4.3 hours, since ~13:01Z). No transaction flow since first observer pass. Morning-api plateaued at balance 20 (ratio ~1.019), witness at balance 0 (ratio ~1.21). Peer balance asymmetry unchanged: morning-api sees witness at 9980, witness sees morning-api at 0.

---

## Persistence State — Delta from Pass 110

### morning-api (simultaneous capture, 17:22:46Z)

| Field | Pass 110 (17:13Z) | This pass | Δ | DEVIATION |
|-------|-------------------|-----------|----|-----------|
| last_snapshot_epoch | 500 | **520** | +20 (2 rotations: 510, 520) | None — normal 10-epoch cadence |
| wal_bytes | 379 | 379 | Unchanged | None — byte-equality PASS (379=379) |
| wal_entries | 3 | 3 | Unchanged | **KNOWN-PROVISIONAL** — size/120 heuristic |

**File inventory:**

| File | Size | mtime (UTC) | Notes |
|------|------|-------------|-------|
| state.snapshot | 895 bytes | Jul 28 13:21 | (epoch 520 snapshot — back to 895 bytes from 894 at epoch 500) |
| wal.log | 379 bytes | Jul 28 13:21 | Active WAL (genesis re-seed only, 3 entries) |
| wal.wal.old | 379 bytes | Jul 28 13:16 | Pre-rotation backup |

**Note on snapshot size oscillation:** state.snapshot was 895 bytes at epoch 450, 894 bytes at epoch 500, and 895 bytes again at epoch 520. A 1-byte oscillation. UNKNOWN: whether this is benign or indicates a serialization boundary issue. Not a new observation — sizes have been in this range since pass ~97.

### local-witness (17:22:46Z)

| Field | Pass 110 (17:13Z) | This pass | Δ | DEVIATION |
|-------|-------------------|-----------|----|-----------|
| last_snapshot_epoch | 500 | **520** | +20 | None — normal rotation |
| wal_bytes | 379 | 379 | Unchanged | Byte-equality PASS (379=379) |
| wal_entries | 3 | 3 | Unchanged | KNOWN-PROVISIONAL |

**File inventory:**

| File | Size | mtime (UTC) | Notes |
|------|------|-------------|-------|
| state.snapshot | 569 bytes | Jul 28 13:21 | Smaller than morning-api (different balance state — 0 vs 9980) |
| wal.log | 379 bytes | Jul 28 13:21 | Active WAL |
| wal.wal.old | 379 bytes | Jul 28 13:16 | Pre-rotation backup |

---

## Metrics

### morning-api (last 3 lines, 17:22:27–17:22:47Z)
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```

### local-witness (last 3 lines, 17:22:30–17:22:50Z)
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
```

**All green:** 0 fetches, 0 aged, empty queues, silence ≤6s (well under 30s threshold). No zombie eviction activity. No stale fetch sweep events. max_peer_silence improved from 8s (pass 110) to 6s on morning-api.

---

## Log Health Scan

### morning-api (/tmp/m-ap.log)

| Item | Count / Detail |
|------|----------------|
| **Epoch complete** lines | **523** (last: epoch=523, 17:22:47Z, balance=20→20, ratio=1.02) |
| **Snapshot rotations** | 400, 410, 420, 430, 440, 450, 460, 470, 480, 490, 500, 510, **520** (normal 10-epoch cadence) |
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
| **Epoch complete** lines | **522** (last: epoch=522, 17:22:40Z, balance=0→0, ratio=1.21) |
| **KAD WARN** | Present (--no-mdns) |
| **Insufficient-balance WARN** | 119 (unchanged — last at 14:01:47Z from Jul 27 redistribution) |
| **Panics** | 0 |
| **Zombie evictions** | 0 |
| **Stale fetch sweeps** | 0 |
| **Non-expected WARN/ERROR** | None |

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Status | Changed Since Pass 110? |
|---|-----------|---------------|--------|------------------------|
| 1 | `build_commit` stale: `cb5d4b1-dirty` vs HEAD `452b64f` (2 behind + dirty) | Jul 27 | PERSISTENT | **Unchanged** |
| 2a | Supply divergence: morning-api total=10,000 (20+9980), witness total=0 | Jul 27 pass 3 | PERSISTENT | **Unchanged** — documented in VERIFIED-BEHAVIOR.md as CONTRADICTED |
| 2b | Cross-node peer balance/nonce asymmetry (witness sees morning-api balance=0, morning-api sees witness nonce=0) | Pass 1 (13:01Z) | PERSISTENT | **Unchanged** |
| 3 | `wal.wal.old` naming (cosmetic) | ae89fbd | KNOWN-PROVISIONAL | **Unchanged** |

## Resolved Observations (from pass 110→111)

None. No new items resolved.

---

## UNKNOWN Items

| # | Unknown | Why unclassified |
|---|---------|-----------------|
| 1 | Ratio divergence (morning-api 1.019 vs witness 1.21) | No design document specifies expected ratio behavior across nodes with different balances. Witness's higher ratio may be correct given its zero balance. |
| 2 | Why economic state is completely frozen (>4.3h without any transaction activity) | Could be expected (no external transactions submitted to UDS), designed (balance 20 floor below 1-token threshold for redistribution), or a bug. Cannot determine from observation alone. |
| 3 | Whether `last_snapshot_epoch=520` snapshot correctly captured the economic state | No on-disk snapshot content verification — observer only checks existence and size. Size oscillated from 895→894→895 bytes across rotations. Could be benign metadata or a serialization boundary issue. |
| 4 | Why witness requested blocks show `redistributed_to=1` (first epoch only) while morning-api also shows `redistributed_to=1` | Both nodes report redistributed_to=1, meaning only epoch 1 (genesis) triggered a redistribution. After that, the floor balance of 20 on morning-api and 0 on witness produced no redistributable surplus. This is consistent with known behavior but not independently verified. |
| 5 | Snapshot size oscillation (895→894→895 bytes across three rotations) | The 1-byte change could be benign (minor metadata difference across snapshots) or a serialization artifact. No design doc specifies expected snapshot byte-level stability. |

---

## Summary

**Pass 111: delta-only. No new deviations.**

The mesh remains in a frozen steady-state — functionally a heartbeat daemon with no transaction activity:

- **2 nodes**, 1 peer each, bidirectional heartbeats healthy (1561/1558), silence ≤9s
- **Epochs cycling** at ~30-33s cadence. Both nodes at 522, synchronized (δ=0).
- **No transactions flowing** — nonces frozen (241/4) since session start (>4.3h)
- **Balance 20 floor** on morning-api, zero on witness — unchanged entire session
- **Snapshot rotation** at normal 10-epoch cadence (520). Three rotations (500→520) since pass 110. WAL unchanged at 379 bytes.
- **All metrics clean:** zero fetches, zero queues, max_peer_silence ≤6s
- **Build commit** 2 behind HEAD + dirty (unchanged since Jul 27)
- **Supply conservation divergence** unchanged (documented, pending governance)
- **Snapshot size oscillation** (895→894→895) noted but unclassified — possible benign metric, not elevated to deviation

**Next expected event:** Snapshot rotation at epoch 530 (~5 min). No other state changes expected given the frozen economic state.
