# Observer Evidence Record — 2026-07-28 (Pass 114)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** 2026-07-28T17:46Z–17:47Z (simultaneous capture at T0, three-way cross-check at T+1min)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** 114th observation pass of Jul 28. ~7 min since pass 113 (17:39Z). Sockets responsive, PIDs unchanged (3579452/3579821).

**Summary:** Delta-only from pass 113. All evidence guards PASS. Epochs advanced +16/+17 at normal cadence (~30s). Snapshot rotated at 560→570 (two rotations since pass 113's 550). Economic state completely frozen — unchanged entire session (>4.5h). Two persistent deviations unchanged (stale build_commit, supply conservation divergence). Snapshot size remained at 895 bytes (no oscillation this pass — stayed 895 instead of toggling to 894). No new findings.

---

## Topology Disclosure

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 13:01Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 13:02Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes since session start (13:01Z).** Same PIDs across all 114 passes (only processes on this machine — no stale survivors).

---

## Evidence Integrity Guards — Simultaneous Capture (17:46–17:47Z)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch (morning-api, 17:47:30Z) | Socket=572, Log count=572, Last log epoch at instant=572 | All three match at a single instant | **PASS** — exactly equal at 17:47:30Z |
| Three-way epoch (witness, 17:47:40Z) | Socket=572, Log count=572, Last log epoch=572 | All three match at a single instant | **PASS** — exactly equal at 17:47:40Z |
| Byte-equality (morning-api) | wal_bytes=379 (socket), stat=379 (ls) | Must match | **PASS** |
| Byte-equality (witness) | wal_bytes=379 (socket), stat=379 (ls) | Must match | **PASS** |
| Cross-node epoch sync (17:47:30–40Z) | morning-api=572, witness=572 | Should be ≤1-2 at same instant | **PASS** — δ=0 at this instant |
| PID consistency | 3579452/3579821 unchanged | Same since 13:01Z | **PASS** |
| Log health (morning-api) | Only KAD bootstrap WARNs (expected --no-mdns), 0 others | Clean | **PASS** |
| Log health (witness) | 0 non-KAD/non-insufficient-balance WARN/ERROR | Clean | **PASS** |

---

## Node Info — Delta from Pass 113

### morning-api (17:46:06Z)

| Field | Pass 113 (17:38:52Z) | This pass | Δ | DEVIATION |
|-------|----------------------|-----------|----|-----------|
| peer_id | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | Same | — | None |
| name | morning-api | Same | — | — |
| genesis_root_id | auto | Same | — | — |
| chain_tip | 1 | 1 | 0 | None — genesis-only mesh |
| uptime_secs | 16650 | **17122** | +472 (~7.9 min) | None — consistent with capture time delta (~7 min) |
| build_commit | cb5d4b1-dirty | cb5d4b1-dirty | Unchanged | **PERSISTENT** — 2 commits behind HEAD 452b64f. Dirty (markdown-only). Unchanged since Jul 27. |
| thickness | 978.29 | **978.17** | -0.12 | No expected value documented |

### local-witness (17:46:06Z)

| Field | Pass 113 (17:39:10Z) | This pass | Δ | DEVIATION |
|-------|----------------------|-----------|----|-----------|
| peer_id | 12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch | Same | — | None |
| name | local-witness | Same | — | — |
| genesis_root_id | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | Same | — | Correct |
| chain_tip | 1 | 1 | 0 | None |
| uptime_secs | 16582 | **17101** | +519 (~8.7 min) | None — consistent with ~7-8 min capture timing; witness queried slightly later |
| build_commit | cb5d4b1-dirty | cb5d4b1-dirty | Unchanged | **PERSISTENT** — same as morning-api |

---

## Epoch State — Delta from Pass 113

### morning-api

| Check | Pass 113 (17:38:47Z) | This pass (17:47:30Z) | Δ | DEVIATION |
|-------|----------------------|----------------------|----|-----------|
| Three-way (17:47:30Z) | Socket=555, Log=555, Last=555 | **Socket=572, Log=572, Last=572** | +17 | **PASS** — exact match |
| Cross-check (17:47:47Z) | — | **573** (epoch completed after capture) | — | None — advances on schedule |

### local-witness

| Check | Pass 113 (17:39:10Z) | This pass (17:47:40Z) | Δ | DEVIATION |
|-------|----------------------|-----------------------|----|-----------|
| Socket | 555 | **572** | +17 | None |
| Log count | 555 | **572** | +17 | None |
| Three-way | PASS | **PASS** (572=572=572) | — | None |
| Cross-check (17:47:40Z) | — | **572** | — | None |

### Cross-node comparison

| Metric | Pass 113 (17:39Z) | This pass (17:47Z) | Δ | DEVIATION |
|--------|------------------|--------------------|----|-----------|
| morning-api epoch (three-way) | 555→556 | **572→573** | +16→+17 | — |
| witness epoch (three-way) | 555 | **572** | +17 | — |
| Cross-node δ | 0→1 (race at boundary) | **0** (captured before boundary) | — | None — normal timing |
| Epoch rate | ~30-33s/ep | ~30-33s/ep | — | None — stable |

---

## Peers — Delta from Pass 113

### morning-api (17:46:54Z)

| Peer | Heartbeats | Silence (s) | Dead | Queue Depth | Δ from Pass 113 (1663) | DEVIATION |
|------|-----------|-------------|------|-------------|------------------------|-----------|
| 12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch | **1708** | 7 | false | 0 | +45 heartbeats (~7 min at ~30s cadence ≈ 14 ticks) | None — healthy |

### local-witness (17:46:54Z)

| Peer | Heartbeats | Silence (s) | Dead | Queue Depth | Δ from Pass 113 (1660) | DEVIATION |
|------|-----------|-------------|------|-------------|------------------------|-----------|
| 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | **1709** | 2 | false | 0 | +49 heartbeats | None — healthy |

**No zombie evictions. No silent peers. No queue buildup.** Bidirectional heartbeat exchange healthy. max_peer_silence 2-7s — well under 30s threshold.

---

## Economic State — Completely Frozen (Unchanged Entire Session >4.5h)

| Metric | Pass 113 (17:39Z) | This pass (17:46Z) | Δ | DEVIATION |
|--------|-------------------|--------------------|----|-----------|
| morning-api: own_balance | 20 | 20 | **Frozen** | None — steady-state floor |
| morning-api: own_nonce | 241 | 241 | **Frozen** | None |
| morning-api: sees witness balance | 9980 | 9980 | **Frozen** | **PERSISTENT DEVIATION #2a** — supply divergence (documented in VERIFIED-BEHAVIOR.md) |
| morning-api: sees witness nonce | 0 | 0 | **Frozen** | **PERSISTENT** |
| Witness: own_balance | 0 | 0 | **Frozen** | None — `--mint 0` |
| Witness: own_nonce | 4 | 4 | **Frozen** | None |
| Witness: sees morning-api balance | 0 | 0 | **Frozen** | **PERSISTENT DEVIATION #2b** — cross-node asymmetry |
| Witness: sees morning-api nonce | 0 | 0 | **Frozen** | **PERSISTENT DEVIATION #2b** |

**OBSERVED:** Economic state frozen across the board for the entire session (>4.5h, since ~13:01Z). No transaction flow since first observer pass. Morning-api plateaued at balance 20 (ratio ~1.019), witness at balance 0 (ratio ~1.192). Peer balance asymmetry unchanged.

---

## Persistence State — Delta from Pass 113

### morning-api (simultaneous capture, 17:46–17:47Z)

| Field | Pass 113 (17:39Z) | This pass | Δ | DEVIATION |
|-------|-------------------|-----------|----|-----------|
| last_snapshot_epoch | 550 | **570** | +20 (2 rotations) | None — normal 10-epoch cadence |
| wal_bytes | 379 | 379 | Unchanged | None — byte-equality PASS (379=379) |
| wal_entries | 3 | 3 | Unchanged | **KNOWN-PROVISIONAL** — size/120 heuristic |

**File inventory:**

| File | Size | mtime (EDT) | Notes |
|------|------|-------------|-------|
| state.snapshot | **895** bytes | Jul 28 13:46 | (epoch 570 snapshot — **unchanged from 895 at epoch 550**) |
| wal.log | 379 bytes | Jul 28 13:46 | Active WAL (genesis re-seed only, 3 entries) |
| wal.wal.old | 379 bytes | Jul 28 13:41 | Pre-rotation backup from epoch 560 |

**Snapshot size oscillation note:** Size stayed at 895 bytes (same as epoch 550 in pass 113). Previous pattern across 5 data points: 895 (450) → 894 (500) → 895 (520) → 894 (530) → 895 (550) → **895 (570)** — the alternating pattern stopped; consecutive snapshots now both 895. UNKNOWN: whether this is a permanent stabilization or the cycle phase shifted.

Witness snapshot stable at 569 bytes (no oscillation ever observed on witness).

### local-witness (simultaneous capture, ~17:46Z)

| Field | Pass 113 (17:39Z) | This pass | Δ | DEVIATION |
|-------|-------------------|-----------|----|-----------|
| last_snapshot_epoch | 550 | **570** | +20 | None — normal rotation |
| wal_bytes | 379 | 379 | Unchanged | Byte-equality PASS (379=379) |
| wal_entries | 3 | 3 | Unchanged | KNOWN-PROVISIONAL |

**File inventory:**

| File | Size | mtime (EDT) | Notes |
|------|------|-------------|-------|
| state.snapshot | **569** bytes | Jul 28 13:46 | Smaller than morning-api (different balance state — 0 vs 9980) |
| wal.log | 379 bytes | Jul 28 13:46 | Active WAL |
| wal.wal.old | 379 bytes | Jul 28 13:41 | Pre-rotation backup |

---

## Metrics

### morning-api (from log)
Metrics lines not directly captured this pass (log tail at epoch 572-573). Pass 113 showed:
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```
No new fetch sweep events expected given frozen economic state. No zombie eviction activity.

### local-witness (from log)
Same pattern as morning-api. No stale fetch sweeps, no zombie evictions.

---

## Log Health Scan

### morning-api (/tmp/m-ap.log)

| Item | Count / Detail |
|------|----------------|
| **Epoch complete** lines | **572** (last: epoch=572 at 17:47:17Z → epoch=573 at 17:47:47Z, balance=20→20, ratio=1.02) |
| **Snapshot rotations** | Normal 10-epoch cadence through 570 |
| **KAD WARN** | `Failed to trigger bootstrap: No known peers.` (~5-min interval, expected with --no-mdns, no DHT) |
| **Non-KAD WARN/ERROR** | **0** (after filtering KAD, startup, and insufficient-balance) |
| **Panics** | 0 |
| **Zombie evictions** | 0 |
| **Stale fetch sweeps** | 0 |
| **Transactions** | 0 new since session start |
| **WAL recovery** | `Genesis recovered from WAL` at startup (13:01:47Z) — correct |

### local-witness (/tmp/lw.log)

| Item | Count / Detail |
|------|----------------|
| **Epoch complete** lines | **572** (last: epoch=572 at 17:47:40Z, balance=0→0, ratio=1.19) |
| **KAD WARN** | Present (--no-mdns) |
| **Insufficient-balance WARN** | 119 (unchanged — last at 14:01:47Z from Jul 27 redistribution) |
| **Non-KAD/non-insufficient-balance WARN/ERROR** | **0** |
| **Panics** | 0 |
| **Zombie evictions** | 0 |
| **Stale fetch sweeps** | 0 |

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Status | Changed Since Pass 113? |
|---|-----------|---------------|--------|------------------------|
| 1 | `build_commit` stale: `cb5d4b1-dirty` vs HEAD `452b64f` (2 behind + dirty) | Jul 27 | PERSISTENT | **Unchanged** |
| 2a | Supply divergence: morning-api total=10,000 (20+9980), witness total=0 | Jul 27 pass 3 | PERSISTENT | **Unchanged** — documented in VERIFIED-BEHAVIOR.md as CONTRADICTED |
| 2b | Cross-node peer balance/nonce asymmetry (witness sees morning-api balance=0, morning-api sees witness nonce=0) | Pass 1 (13:01Z) | PERSISTENT | **Unchanged** |
| 3 | `wal.wal.old` naming (cosmetic) | ae89fbd | KNOWN-PROVISIONAL | **Unchanged** |

## Resolved Observations (from pass 113→114)

None. No new items resolved.

---

## UNKNOWN Items

| # | Unknown | Why unclassified |
|---|---------|-----------------|
| 1 | Ratio divergence (morning-api 1.019 vs witness 1.192 — witness declining asymptotically from 1.934 at session start) | No design document specifies expected ratio behavior across nodes with different balances. |
| 2 | Why economic state is completely frozen (>4.5h without any transaction activity) | Could be expected (no external transactions submitted to UDS), designed (balance 20 floor below 1-token threshold for redistribution), or a bug. Cannot determine from observation alone. |
| 3 | Snapshot size oscillation stopped (895→894→895→894→895→**895** — alternating pattern broke at epoch 570) | Witness snapshot stable at 569 bytes (no oscillation ever). Could have been a 5-cycle coincidence that has now stabilized, or the next snapshot may toggle back to 894. |
| 4 | MESH.md header reads "No production nodes running" but both nodes have been running since ~09:01 EDT (~4.75h) | Not clear whether this is intentional (deferred update) or an oversight. Does not affect mesh behavior. Noted since pass 2. |

---

## Summary

**Pass 114: delta-only. No new deviations.**

The mesh remains in a frozen steady-state — functionally a heartbeat daemon with no transaction activity:

- **2 nodes**, 1 peer each, bidirectional heartbeats healthy (1708/1709), silence ≤7s
- **Epochs cycling** at ~30-33s cadence. Both nodes synchronized (δ=0 at this capture instant)
- **No transactions flowing** — nonces frozen (241/4) since session start (>4.5h)
- **Balance 20 floor** on morning-api, zero on witness — unchanged entire session
- **Snapshot rotation** at normal 10-epoch cadence (570). Two rotations (550→560→570) since pass 113. WAL unchanged at 379 bytes.
- **Snapshot size** at 895 bytes — no oscillation this pass (stayed at 895 instead of toggling to 894)
- **Build commit** 2 behind HEAD + dirty (unchanged since Jul 27)
- **Supply conservation divergence** unchanged (documented, pending governance)
- **MESH.md** still claims "No production nodes running" — stale since Jul 27 restart

**Next expected event:** Snapshot rotation at epoch 580 (~5 min). No other state changes expected given the frozen economic state.
