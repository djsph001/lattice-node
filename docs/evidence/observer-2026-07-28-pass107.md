# Observer Evidence Record — 2026-07-28 (Pass 107)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** 2026-07-28T16:26:46Z (simultaneous single-capture)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** 107th observation pass of Jul 28. ~8 min since pass 106 (16:18:38Z). Same PIDs since 13:01Z (~3h26m runtime).

**Summary:** Delta-only from pass 106. All evidence guards PASS. Snapshot advanced from 390 to 410 (two rotations: 400, 410) — resolving the "frozen at 390" observation from pass 106. Three-way epoch match clean on both nodes. Economic state completely frozen. Metrics clean. No new deviations.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 13:01Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 13:02Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs since pass 1 (13:01Z). Both sockets responding. 2 lattice-node processes + 2 bash wrappers.

---

## Evidence Integrity Guards — Simultaneous Single-Capture (16:26:46Z)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch (morning-api) | Socket=411, Log count=411, Last log epoch=411 | All three equal | **PASS** — 411=411=411 |
| Three-way epoch (witness) | Socket=410, Log count=410, Last log epoch=410 | All three equal | **PASS** — 410=410=410 |
| Byte-equality (morning-api) | wal_bytes=379, `ls wal.log`=379 | Must match | **PASS** |
| Byte-equality (witness) | wal_bytes=379, `ls wal.log`=379 | Must match | **PASS** |
| Cross-node epoch sync | morning-api=411, witness=410 (δ=1) | Should be ≤1-2 | **OK** — normal drift |
| PID consistency | 3579452/3579821 unchanged | Same since 13:01Z | **PASS** |
| Log health | 0 non-KAD WARN/ERROR (m-ap), 0 non-KAD/non-insufficient-balance (witness) | Clean | **PASS** |
| Metrics health | aged=0, queues=[], max_peer_silence 6s/3s | aged≈0, silence<30s | **PASS** |

---

## Node Info

### morning-api (16:26:46Z)

| Field | OBSERVED (this pass) | DEVIATION |
|-------|---------------------|-----------|
| peer_id | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | None — matches genesis |
| name | morning-api | — |
| genesis_root_id | auto | — |
| chain_tip | 1 | None — genesis-only mesh |
| uptime_secs | 12298 | None — 3h25min consistent with 13:01Z start |
| build_commit | **cb5d4b1-dirty** | **PERSISTENT** — 2 commits behind HEAD 452b64f. Dirty from markdown evidence files only. Unchanged since first observation (Jul 27). |
| thickness | 979.44 | No expected value documented |

### local-witness (16:26:46Z)

| Field | OBSERVED (this pass) | DEVIATION |
|-------|---------------------|-----------|
| peer_id | 12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch | None |
| name | local-witness | — |
| genesis_root_id | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | Correct — matches morning-api |
| chain_tip | 1 | None |
| uptime_secs | 12302 | — |
| build_commit | **cb5d4b1-dirty** | Same PERSISTENT deviation |

---

## Epoch State — Delta from Pass 106

### morning-api (simultaneous capture, 16:26:46Z)

| Check | Pass 106 (16:18:38Z) | This pass (16:26:46Z) | Δ | DEVIATION |
|-------|----------------------|-----------------------|---|-----------|
| Socket epoch | 394 | **411** | +17 | None — ~28s/epoch normal cadence |
| Log count | 394 | **411** | +17 | None |
| Last log epoch | 394 | **411** (16:26:47Z) | +17 | None |
| Three-way equality | PASS (394=394=394) | **PASS** (411=411=411) | — | None |

### local-witness (simultaneous capture, ~16:26:40Z)

| Check | Pass 106 (16:18:38Z) | This pass (16:26:40Z) | Δ | DEVIATION |
|-------|----------------------|-----------------------|---|-----------|
| Socket epoch | 393 | **410** | +17 | None |
| Log count | 394 | **410** | +16 | None — ±1 boundary race resolved |
| Last log epoch | 394 | **410** (16:26:40Z) | +16 | None |
| Three-way equality | ±1 boundary race | **PASS** (410=410=410) | — | **RESOLVED** — no race this pass |

### Cross-node comparison

| Metric | Pass 106 | This pass | Δ | DEVIATION |
|--------|----------|-----------|----|-----------|
| morning-ap epoch | 394 | 411 | +17 | — |
| witness epoch | 393 | 410 | +17 | — |
| Cross-node δ | 1 | 1 | Unchanged | None — normal |
| Epoch rate | ~29s/ep (est) | ~28s/ep | Consistent | None |

---

## Peers

### morning-api (16:26:46Z)

| Peer | Name | Heartbeats | Silence (s) | Dead | Queue Depth | DEVIATION |
|------|------|-----------|-------------|------|-------------|-----------|
| 12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch | null | 1227 | 8 | false | 0 | None — healthy 1-peer mesh |

### local-witness (16:26:40Z)

| Peer | Name | Heartbeats | Silence (s) | Dead | Queue Depth | DEVIATION |
|------|------|-----------|-------------|------|-------------|-----------|
| 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | null | 1230 | 8 | false | 0 | None — healthy |

---

## Economic State — Completely Frozen (Unchanged Since Pass 97, ~14:08Z)

| Metric | Pass 106 (16:18Z) | This pass (16:26Z) | Δ | DEVIATION |
|--------|-------------------|--------------------|----|-----------|
| morning-api: own_balance | 20 | 20 | **Frozen** | None — steady-state floor |
| morning-api: own_nonce | 241 | 241 | **Frozen** | None |
| morning-api: sees witness balance | 9980 | 9980 | **Frozen** | **PERSISTENT DEVIATION #2a** |
| morning-api: sees witness nonce | 0 | 0 | **Frozen** | **PERSISTENT DEVIATION #2b** |
| Witness: own_balance | 0 | 0 | **Frozen** | None — --mint 0 |
| Witness: own_nonce | 4 | 4 | **Frozen** | None |
| Witness: sees morning-api balance | 0 | 0 | **Frozen** | **PERSISTENT DEVIATION #2b** |
| Witness: sees morning-api nonce | 0 | 0 | **Frozen** | **PERSISTENT DEVIATION #2b** |

**OBSERVED:** Economic state frozen since pass 97 (~14:08Z), now >2.3 hours. No transactions flowing. Balance 20 floor cycles with ratio ~1.02. Witness balance 0 cycles with ratio ~1.27.

---

## Persistence State

### morning-api (16:26:46Z simultaneous capture)

| Field | Pass 106 (16:18Z) | This pass (16:26Z) | Δ | DEVIATION |
|-------|-------------------|--------------------|----|-----------|
| last_snapshot_epoch | **390** (frozen ~1.5h) | **410** | +20 (2 rotations: 400, 410) | **RESOLVED** — snapshot was not stalled, pass 106 hit mid-cycle |
| wal_bytes | 379 | 379 | Unchanged | None — byte-equality PASS (379=379) |
| wal_entries | 3 | 3 | Unchanged | **KNOWN-PROVISIONAL** — size/120 heuristic |

**File inventory:**

| File | Size | mtime (EDT) | Notes |
|------|------|-------------|-------|
| state.snapshot | 895 bytes | Jul 28 12:26 (epoch 410 snapshot) | Size unchanged across all rotation cycles |
| wal.log | 379 bytes | Jul 28 12:26 | Active WAL (genesis re-seed only, 3 entries) |
| wal.wal.old | 379 bytes | Jul 28 12:21 | Pre-rotation backup |

### local-witness (16:26:40Z simultaneous capture)

| Field | Pass 106 (16:18Z) | This pass (16:26Z) | Δ | DEVIATION |
|-------|-------------------|--------------------|----|-----------|
| last_snapshot_epoch | 390 | **410** | +20 (2 rotations) | Resolved — same as morning-api |
| wal_bytes | 379 | 379 | Unchanged | Byte-equality PASS (379=379) |
| wal_entries | 3 | 3 | Unchanged | Known-provisional |

**File inventory:**

| File | Size | mtime (EDT) | Notes |
|------|------|-------------|-------|
| state.snapshot | 569 bytes | Jul 28 12:26 | Smaller than morning-api snapshot (different balance state) |
| wal.log | 379 bytes | Jul 28 12:26 | Active WAL |
| wal.wal.old | 379 bytes | Jul 28 12:21 | Pre-rotation backup |

---

## Metrics

### morning-api (last 3 lines, 16:27:07–16:27:27Z)
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```

### local-witness (last 3 lines, 16:27:10–16:27:30Z)
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
```

**All green:** 0 fetches, 0 aged, empty queues, silence well under 30s threshold. No zombie eviction activity. No stale fetch sweep events. Same pattern since pass 97.

---

## Log Health Scan

### morning-api (/tmp/m-ap.log)
- **411 Epoch complete** lines (last: epoch=411, 16:26:47Z, balance=20→20)
- **Snapshot rotations:** At epochs 400, 410 (normal 10-epoch cadence)
- **Only non-KAD WARN:** `No snapshot found, starting fresh` (startup, expected)
- **Panics: 0. Zombie evictions: 0. Stale fetch sweeps: 0. Transactions: 0. Non-KAD WARN/ERROR: None.**

### local-witness (/tmp/lw.log)
- **410 Epoch complete** lines (last: epoch=410, 16:26:40Z, balance=0→0)
- **119 insufficient-balance** WARNs (unchanged — last at 14:01:47Z, no new rejections)
- **Only non-KAD WARN:** `No snapshot found, starting fresh` (startup, expected)
- **Panics: 0. Zombie evictions: 0. Stale fetch sweeps: 0. Non-KAD/non-insufficient-balance WARN/ERROR: None.**

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Status | Changed Since Pass 106? |
|---|-----------|---------------|--------|------------------------|
| 1 | `build_commit` stale: `cb5d4b1-dirty` vs HEAD `452b64f` (2 behind + dirty) | Jul 27 | PERSISTENT | **Unchanged** |
| 2a | Supply divergence: morning-api total=10,000 (20+9980), witness total=0 | Jul 27 pass 3 | PERSISTENT | **Unchanged** — documented in VERIFIED-BEHAVIOR.md as CONTRADICTED |
| 2b | Cross-node peer balance/nonce asymmetry | Pass 1 (13:01Z) | PERSISTENT | **Unchanged** |
| 3 | `wal.wal.old` naming (cosmetic) | ae89fbd | KNOWN-PROVISIONAL | **Unchanged** |

---

## Resolved Observations

| # | Observation (from pass 106) | Resolution |
|---|-----------------------------|-----------|
| 1 | Snapshot frozen at epoch 390 for ~1.5h | **RESOLVED** — snapshot now at 410 (two rotations since). Pass 106 hit mid-cycle between snapshot rotations. Not stalled. |
| 2 | Three-way epoch boundary race on witness (393/394) | **RESOLVED** — clean match at 410=410=410 this pass. |

---

## UNKNOWN Items

| # | Unknown | Why unclassified |
|---|---------|-----------------|
| 1 | Ratio divergence (morning-api 1.020 vs witness 1.270) | Both are asymptotic from their respective balance states. No design document specifies expected ratio behavior across nodes with different balances. |
| 2 | Why economic state is completely frozen (>2.3h without any transaction activity) | Could be expected (no external transactions submitted), designed (balance 20 floor below 1-token threshold for redistribution), or a bug. Cannot determine from observation alone. |

---

## Summary

**Pass 107: delta-only. No new deviations.**

The mesh remains in a frozen steady-state:

- **2 nodes**, 1 peer each, bidirectional heartbeats healthy (1227/1230), silence ≤8s
- **Epochs cycling** at ~28s cadence. morning-api at 411, witness at 410 (δ=1, normal drift)
- **No transactions flowing** — nonces frozen (241/4) since session start (~3.4h)
- **Balance 20 floor** on morning-api, zero on witness — unchanged entire session
- **Snapshot rotation** at normal 10-epoch cadence (400→410). WAL unchanged at 379 bytes.
- **All metrics clean:** zero fetches, zero queues, max_peer_silence ≤6s
- **Build commit** 2 behind HEAD + dirty (unchanged since Jul 27)
- **Supply conservation divergence** unchanged (documented, pending governance)

**Next expected event:** Snapshot rotation at epoch 420 (~5 min). No other state changes expected.
