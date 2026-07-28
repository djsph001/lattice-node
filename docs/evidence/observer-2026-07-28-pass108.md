# Observer Evidence Record — 2026-07-28 (Pass 108)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** 2026-07-28T16:34:38Z (socket queries), 16:35:21Z (log/file queries)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** 108th observation pass of Jul 28. ~8 min since pass 107 (16:26:46Z). Sockets responsive, PIDs unchanged.

**Summary:** Delta-only from pass 107. All evidence guards PASS (with small timing offset). Epochs advanced +15/+16. Snapshot rotated one notch (410→420). Economic state completely frozen — unchanged entire session (>3.5h). Metrics clean. No new deviations.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 13:01Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 13:02Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs since pass 1 (13:01Z). Both sockets responding. 2 lattice-node processes + 2 bash wrappers (+1 hermes shell ephemeral for this capture).

---

## Evidence Integrity Guards — Near-Simultaneous Capture (socket 16:34:38Z, log 16:35:21Z)

*Note: Socket and log timestamps differ by ~43s. The three-way check below accounts for epochs that ticked during the capture gap.*

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch (morning-api) | Socket=426 (16:34:38Z), Log count=428, Last log epoch=428 (16:35:17Z) | All three should match at a single instant | **PASS** — log advanced +2 during 39s capture gap (~28s cadence). 426→428 consistent. |
| Three-way epoch (witness) | Socket=426 (16:34:57Z), Log count=427, Last log epoch=427 (16:35:10Z) | All three should match | **PASS** — log advanced +1 during 13s gap. Consistent. |
| Byte-equality (morning-api) | wal_bytes=379 (socket), `stat wal.log`=379 (ls) | Must match | **PASS** |
| Byte-equality (witness) | wal_bytes=379 (socket), `stat wal.log`=379 (ls) | Must match | **PASS** |
| Cross-node epoch sync | morning-api=426, witness=426 (socket, δ=0) | Should be ≤1-2 | **PASS** — both at 426 at capture time |
| PID consistency | 3579452/3579821 unchanged | Same since 13:01Z | **PASS** |
| Log health (morning-api) | 0 non-KAD/non-expected WARN/ERROR (KAD bootstrap warnings expected with --no-mdns) | Clean | **PASS** |
| Log health (witness) | 0 non-KAD/non-insufficient-balance WARN/ERROR | Clean | **PASS** |
| Metrics health | aged=0, queues=[], max_peer_silence 6s/3s | aged≈0, silence<30s | **PASS** |

---

## Node Info

### morning-api (16:34:38Z)

| Field | OBSERVED (this pass) | DEVIATION |
|-------|---------------------|-----------|
| peer_id | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | None — matches genesis |
| name | morning-api | — |
| genesis_root_id | auto | — |
| chain_tip | 1 | None — genesis-only mesh |
| uptime_secs | 12771 | None — 3h33min consistent with 13:01Z start |
| build_commit | **cb5d4b1-dirty** | **PERSISTENT** — 2 commits behind HEAD 452b64f. Dirty from markdown evidence files only (untracked). Unchanged since Jul 27. |
| thickness | 979.31 | No expected value documented (was 979.44 in pass 107) |

### local-witness (16:34:57Z)

| Field | OBSERVED (this pass) | DEVIATION |
|-------|---------------------|-----------|
| peer_id | 12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch | None |
| name | local-witness | — |
| genesis_root_id | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | Correct — matches morning-api |
| chain_tip | 1 | None |
| uptime_secs | 12767 | — |
| build_commit | **cb5d4b1-dirty** | Same PERSISTENT deviation |

---

## Epoch State — Delta from Pass 107

### morning-api (socket 16:34:38Z, log 16:35:21Z)

| Check | Pass 107 (16:26:46Z) | This pass | Δ | DEVIATION |
|-------|----------------------|-----------|----|-----------|
| Socket epoch | 411 | **426** | +15 | None — ~31.5s/epoch cadence (consistent with ~28-32s) |
| Log count | 411 | **428** | +17 | None — 2 more epochs elapsed during capture gap |
| Last log epoch | 411 (16:26:47Z) | **428** (16:35:17Z) | +17 | None |
| Three-way equality | PASS (411=411=411) | **PASS** (426→428 consistent with capture gap) | — | None |

### local-witness (socket 16:34:57Z, log 16:35:21Z)

| Check | Pass 107 (16:26:40Z) | This pass | Δ | DEVIATION |
|-------|----------------------|-----------|----|-----------|
| Socket epoch | 410 | **426** | +16 | None — ~30.7s/epoch |
| Log count | 410 | **427** | +17 | None — 1 more epoch during capture gap |
| Last log epoch | 410 (16:26:40Z) | **427** (16:35:10Z) | +17 | None |
| Three-way equality | PASS (410=410=410) | **PASS** (426→427 consistent) | — | None |

### Cross-node comparison

| Metric | Pass 107 | This pass | Δ | DEVIATION |
|--------|----------|-----------|----|-----------|
| morning-api epoch | 411 | 426 | +15 | — |
| witness epoch | 410 | 426 | +16 | — |
| Cross-node δ | 1 | **0** | -1 | **RESOLVED** — both nodes at epoch 426 at socket capture time. Not a deviation (normal drift within ±1). |
| Epoch rate | ~28s/ep | ~31s/ep | Slightly slower cadence | None — within normal range |

---

## Peers

### morning-api (16:34:38Z)

| Peer | Name | Heartbeats | Silence (s) | Dead | Queue Depth | DEVIATION |
|------|------|-----------|-------------|------|-------------|-----------|
| 12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch | null | 1275 | 0 | false | 0 | None — healthy 1-peer mesh |

### local-witness (16:34:57Z)

| Peer | Name | Heartbeats | Silence (s) | Dead | Queue Depth | DEVIATION |
|------|------|-----------|-------------|------|-------------|-----------|
| 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | null | 1277 | 3 | false | 0 | None — healthy |

---

## Economic State — Completely Frozen (Unchanged Since Pass 97, ~14:08Z, Now >2.5h)

| Metric | Pass 107 (16:26Z) | This pass (16:34Z) | Δ | DEVIATION |
|--------|-------------------|--------------------|----|-----------|
| morning-api: own_balance | 20 | 20 | **Frozen** | None — steady-state floor |
| morning-api: own_nonce | 241 | 241 | **Frozen** | None |
| morning-api: sees witness balance | 9980 | 9980 | **Frozen** | **PERSISTENT DEVIATION #2a** |
| morning-api: sees witness nonce | 0 | 0 | **Frozen** | **PERSISTENT DEVIATION #2b** |
| Witness: own_balance | 0 | 0 | **Frozen** | None — --mint 0 |
| Witness: own_nonce | 4 | 4 | **Frozen** | None |
| Witness: sees morning-api balance | 0 | 0 | **Frozen** | **PERSISTENT DEVIATION #2b** |
| Witness: sees morning-api nonce | 0 | 0 | **Frozen** | **PERSISTENT DEVIATION #2b** |

**OBSERVED:** Economic state frozen since pass 97 (~14:08Z), now >2.5 hours. No transactions flowing. Balance 20 floor cycles with ratio ~1.02. Witness balance 0 cycles with ratio ~1.26.

---

## Persistence State

### morning-api (simultaneous capture, 16:34:38Z socket + 16:35:21Z ls)

| Field | Pass 107 (16:26Z) | This pass | Δ | DEVIATION |
|-------|-------------------|-----------|----|-----------|
| last_snapshot_epoch | 410 | **420** | +10 (1 rotation) | None — normal 10-epoch cadence |
| wal_bytes | 379 | 379 | Unchanged | None — byte-equality PASS (379=379) |
| wal_entries | 3 | 3 | Unchanged | **KNOWN-PROVISIONAL** — size/120 heuristic |

**File inventory:**

| File | Size | mtime (EDT) | Notes |
|------|------|-------------|-------|
| state.snapshot | 894 bytes | Jul 28 12:31 (epoch 420 snapshot) | Size nearly identical (895→894, probably 1-byte timestamp diff) |
| wal.log | 379 bytes | Jul 28 12:31 | Active WAL (genesis re-seed only, 3 entries) |
| wal.wal.old | 379 bytes | Jul 28 12:26 | Pre-rotation backup |

### local-witness (simultaneous capture, 16:34:57Z socket + 16:35:21Z ls)

| Field | Pass 107 (16:26Z) | This pass | Δ | DEVIATION |
|-------|-------------------|-----------|----|-----------|
| last_snapshot_epoch | 410 | **420** | +10 | None — normal rotation |
| wal_bytes | 379 | 379 | Unchanged | Byte-equality PASS (379=379) |
| wal_entries | 3 | 3 | Unchanged | KNOWN-PROVISIONAL |

**File inventory:**

| File | Size | mtime (EDT) | Notes |
|------|------|-------------|-------|
| state.snapshot | 569 bytes | Jul 28 12:31 | Smaller than morning-api (different balance state) |
| wal.log | 379 bytes | Jul 28 12:31 | Active WAL |
| wal.wal.old | 379 bytes | Jul 28 12:26 | Pre-rotation backup |

---

## Metrics

### morning-api (last 3 lines, 16:34:57–16:35:17Z)
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```

### local-witness (last 3 lines, 16:35:00–16:35:20Z)
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
```

**All green:** 0 fetches, 0 aged, empty queues, silence well under 30s threshold. No zombie eviction activity. No stale fetch sweep events. Same pattern since pass 97.

---

## Log Health Scan

### morning-api (/tmp/m-ap.log)
- **428 Epoch complete** lines (last: epoch=428, 16:35:17Z, balance=20→20)
- **Snapshot rotations:** At epochs 400, 410, 420 (normal 10-epoch cadence)
- **Non-KAD WARN:** `No snapshot found, starting fresh` (startup at 13:01Z, expected)
- **KAD WARN:** `Failed to trigger bootstrap: No known peers.` (~5-min interval, expected with --no-mdns, no DHT)
- **Panics: 0. Zombie evictions: 0. Stale fetch sweeps: 0. Transactions: 0. Non-expected WARN/ERROR: None.**

### local-witness (/tmp/lw.log)
- **427 Epoch complete** lines (last: epoch=427, 16:35:10Z, balance=0→0)
- **119 insufficient-balance** WARNs (unchanged — last at 14:01:47Z, no new rejections)
- **Non-KAD WARN:** `No snapshot found, starting fresh` (startup, expected)
- **Panics: 0. Zombie evictions: 0. Stale fetch sweeps: 0. Non-KAD/non-insufficient-balance WARN/ERROR: None.**

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Status | Changed Since Pass 107? |
|---|-----------|---------------|--------|------------------------|
| 1 | `build_commit` stale: `cb5d4b1-dirty` vs HEAD `452b64f` (2 behind + dirty) | Jul 27 | PERSISTENT | **Unchanged** |
| 2a | Supply divergence: morning-api total=10,000 (20+9980), witness total=0 | Jul 27 pass 3 | PERSISTENT | **Unchanged** — documented in VERIFIED-BEHAVIOR.md as CONTRADICTED |
| 2b | Cross-node peer balance/nonce asymmetry | Pass 1 (13:01Z) | PERSISTENT | **Unchanged** |
| 3 | `wal.wal.old` naming (cosmetic) | ae89fbd | KNOWN-PROVISIONAL | **Unchanged** |

## Resolved Observations (from pass 107→108)

| # | Observation (from pass 107) | Resolution |
|---|-----------------------------|-----------|
| 1 | Cross-node epoch δ=1 (411/410) | **NO LONGER DEVIANT** — both nodes at 426 at socket capture. δ=0 is normal variance. |

## UNKNOWN Items

| # | Unknown | Why unclassified |
|---|---------|-----------------|
| 1 | Ratio divergence (morning-api 1.020 vs witness 1.260) | Both are asymptotic from their respective balance states. No design document specifies expected ratio behavior across nodes with different balances. |
| 2 | Why economic state is completely frozen (>3.5h without any transaction activity) | Could be expected (no external transactions submitted), designed (balance 20 floor below 1-token threshold for redistribution), or a bug. Cannot determine from observation alone. |

---

## Summary

**Pass 108: delta-only. No new deviations.**

The mesh remains in a frozen steady-state:

- **2 nodes**, 1 peer each, bidirectional heartbeats healthy (1275/1277), silence ≤6s
- **Epochs cycling** at ~31s cadence. Both nodes at 426 at socket capture time (δ=0).
- **No transactions flowing** — nonces frozen (241/4) since session start (~3.5h)
- **Balance 20 floor** on morning-api, zero on witness — unchanged entire session
- **Snapshot rotation** at normal 10-epoch cadence (420). WAL unchanged at 379 bytes.
- **All metrics clean:** zero fetches, zero queues, max_peer_silence ≤6s
- **Build commit** 2 behind HEAD + dirty (unchanged since Jul 27)
- **Supply conservation divergence** unchanged (documented, pending governance)

**Next expected event:** Snapshot rotation at epoch 430 (~5 min). No other state changes expected.
