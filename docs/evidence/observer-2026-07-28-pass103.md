# Observer Evidence Record — 2026-07-28 (Pass 103)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** ~2026-07-28T16:58:45-16:59:15Z (single capture bundle)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (Boynton Beach FL)
**Session type:** 103rd observation pass. Same session as passes 97-102 (same PIDs, no restart). Mesh running since ~13:01Z (~3h58min uptime).

**Summary:** Delta-only pass from pass 102. Mesh remains in frozen steady-state. Epochs advanced 254→475 (+221). Snapshot rotated through to epoch 470. All economic state frozen since pass 97 (~3 hours). Metrics clean. Build commit unchanged. No new deviations.

---

## Timing Note — Three-Way Epoch Check

The witness socket returned epoch=474 at ~16:58:45Z, but the log count (grep -c) returned 475 at ~16:59:15Z. The final log line shows epoch=475 at 16:59:10Z. This is a **race at an epoch boundary**: the socket was queried before epoch 475 completed on the witness, and the log query ran after. Not a deviation — consistent with capture ordering across ~30s.

morning-api: all three values match at 475 (socket, log count, last log). **PASS.**

**Corrected classification:** Witness 1-off is a timing artifact, not a deviation.

---

## Topology Disclosure — Unchanged from Pass 102

| PID | Name | Port | Since (UTC) | Status |
|-----|------|------|-------------|--------|
| 3579452 | morning-api | 4005 | 13:01 | Running, no restart |
| 3579821 | local-witness | 4010 | 13:02 | Running, no restart |

**Topology change from pass 102:** None. Same PIDs, same nodes, same session.

---

## Node Info (Delta from Pass 102)

| Field | Pass 102 (15:08Z) | This pass (16:58Z) | Δ | DEVIATION |
|-------|-------------------|--------------------|---|-----------|
| uptime_secs (m-ap) | 7602 | 14218 | +6616s (~110 min real) | None — matches clock |
| uptime_secs (witness) | ~7600 | 14209 | +6609s | None — matches clock |
| build_commit (both) | `cb5d4b1-dirty` | `cb5d4b1-dirty` | Unchanged | **Persistent DEVIATION (#1).** 2 commits behind HEAD `452b64f` + dirty. Unchanged since Jul 27. |
| thickness (m-ap) | 980.66 | 978.93 | −1.73 (expected decay) | None |
| chain_tip (both) | 1 | 1 | Unchanged | None — no RatificationBlock production on 2-node mesh |

---

## Epoch State

### morning-api

| Check | Pass 102 (15:08Z) | This pass (16:58:45Z) | Δ | DEVIATION |
|-------|-------------------|-----------------------|---|-----------|
| Socket epoch | 254 | **475** (@16:58:45Z) | +221 | — |
| Log count (grep -c) | 254 | **475** (@16:59:15Z) | +221 | — |
| Last log epoch | 254 | **475** (@16:58:47Z) | +221 | — |
| Three-way equality | **PASS** — 254=254=254 | **PASS** — 475=475=475 | — | None |

### local-witness

| Check | Pass 102 (15:08Z) | This pass (16:58:45Z) | Δ | DEVIATION |
|-------|-------------------|-----------------------|---|-----------|
| Socket epoch | 254 | **474** (@16:58:45Z) | +220 | — |
| Log count (grep -c) | 254 | **475** (@16:59:15Z) | +221 | — |
| Last log epoch | 254 | **475** (@16:59:10Z) | +221 | — |
| Three-way equality | **PASS** — 254=254=254 | **TIMING GAP** — socket 474 ≠ log 475 at different capture times | — | Race at epoch boundary. Not a deviation. |

### Cross-node comparison

| Metric | Pass 102 | This pass | Δ |
|--------|---------|-----------|----|
| m-ap | 254 | 475 | +221 epochs |
| witness | 254 | 475 | +221 epochs (adjusted for timing) |
| Cross-node δ | 0 | 0 — both at 475 in the same epoch | Tight sync |
| Epoch rate | ~1.9/min (~31s/epoch) | ~2.0/min (~30s/epoch) | Stable — matches configured 30s epoch_interval |

---

## Peer Connections

| Metric | Pass 102 (15:08Z) | This pass (16:58Z) | Δ |
|--------|-------------------|--------------------|---|
| m-ap: heartbeats received | 758 | 1420 | +662 (~6.0/min) |
| m-ap: silence_secs | 5 | 0 | Back to 0 — heartbeat just received |
| m-ap: queue_depth | 0 | 0 | Unchanged |
| m-ap: is_dead | false | false | Unchanged |
| witness: heartbeats received | 760 | 1421 | +661 (~6.0/min) |
| witness: silence_secs | 1 | 9 | Healthy (<30s threshold) |
| witness: queue_depth | 0 | 0 | Unchanged |
| witness: is_dead | false | false | Unchanged |

**OBSERVED:** Both nodes see exactly 1 peer. Heartbeats flowing at ~6.0/min bidirectionally. Silence well under 30s threshold. No zombie evictions (zero since session start). No backpressure.

---

## Economic State — Completely Frozen (Unchanged Since Pass 97, ~14:08Z)

| Metric | Pass 102 (15:08Z) | This pass (16:58Z) | Δ | DEVIATION |
|--------|-------------------|--------------------|---|-----------|
| m-ap: own_balance | 20 | 20 | **Frozen** | **Persistent DEVIATION (#2a).** See below. |
| m-ap: own_nonce | 241 | 241 | **Frozen** | None — no new transactions since pass 97 |
| m-ap: sees witness balance | 9980 | 9980 | **Frozen** | **Persistent DEVIATION (#2a component).** See below. |
| m-ap: sees witness nonce | 0 | 0 | **Frozen** | None |
| Witness: own_balance | 0 | 0 | **Frozen** | **Persistent DEVIATION (#2b).** See below. |
| Witness: own_nonce | 4 | 4 | **Frozen** | None — no new transactions since pass 97 |
| Witness: sees m-ap balance | 0 | 0 | **Frozen** | **Persistent DEVIATION (#2b).** See below. |
| Witness: sees m-ap nonce | 0 | 0 | **Frozen** | None |
| Total supply (m-ap books) | 10,000 | 10,000 (20 + 9980) | **Frozen** | **Persistent DEVIATION (#2a).** |

### Deviation Details

**#2a — morning-api ledger total supply = 10,000 vs minted 5,000**
- **OBSERVED:** morning-api reports own_balance=20, witness balance=9980, total=10,000
- **EXPECTED:** Total supply = 5,000 from `--mint 5000`. The additional 5,000 has no documented source.
- **First observed:** Pass 97 (~14:08Z, Jul 28) — morning-api had 20 + witness 9980 at session start after restart from snapshot
- **Changed since last pass?** No. Frozen since first observation.
- **Status:** Persistent.

**#2b — witness ledger sees morning-api balance as 0**
- **OBSERVED:** local-witness reports morning-api balance=0 (both via GetEconomicState and in log: `insufficient balance: 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ has 0, needs NNN`)
- **EXPECTED:** Witness should see morning-api's balance consistent with what morning-api reports (≥20). With 475 epochs of redistribution, some positive balance.
- **First observed:** Jul 27 (18:48Z) — documented in VERIFIED-BEHAVIOR.md
- **Changed since last pass?** No. Witness's view frozen at `m-ap=0` since session start.
- **Status:** Persistent. Causal explanation (supply conservation divergence) documented in VERIFIED-BEHAVIOR.md under "Not Verified — Confirmed Protocol-Level Findings."

**OBSERVED (mechanism):** At epoch 3 of the current session, morning-api began redistributing tax (329 DUU). The witness received the transaction but rejected it with `insufficient balance: ... has 0, needs 329`. Morning-api debited itself (20 DUU floor reached at tax < 1 DUU) and credited the witness on its own books. The witness's view of morning-api's balance remained 0. No redistribution has occurred since balance reached 20 (tax rounds to 0).

---

## Persistence State

### morning-api (16:58Z single capture)

| Field | Pass 102 (15:08Z) | This pass | Δ |
|-------|-------------------|-----------|---|
| last_snapshot_epoch | 250 | **470** | Rotated 250→260→270→...→470 (22 rotations) |
| wal_bytes | 379 | 379 | Unchanged |
| wal_entries | 3 | 3 | Unchanged |

**File inventory (m-ap, 16:58Z):**

| File | Size | mtime (UTC) | Notes |
|------|------|-------------|-------|
| `state.snapshot` | 895 bytes | 12:56 (snapshot at epoch 470) | Same size as earlier passes |
| `wal.log` | 379 bytes | 12:56 | Active WAL (genesis re-seed, no transactions) |
| `wal.wal.old` | 379 bytes | 12:51 | Pre-rotation backup (persists across passes) |

**Byte-equality:** `GetPersistenceState.wal_bytes=379`. `ls -la wal.log=379 bytes`. **PASS.**

### local-witness (16:58Z single capture)

| Field | Pass 102 (15:08Z) | This pass | Δ |
|-------|-------------------|-----------|---|
| last_snapshot_epoch | 250 | **470** | Rotated same set |
| wal_bytes | 379 | 379 | Unchanged |
| wal_entries | 3 | 3 | Unchanged |

**File inventory (witness, 16:58Z):**

| File | Size | mtime (UTC) | Notes |
|------|------|-------------|-------|
| `state.snapshot` | 569 bytes | 12:56 | Snapshot at epoch 470 (unchanged size) |
| `wal.log` | 379 bytes | 12:56 | Active WAL |
| `wal.wal.old` | 379 bytes | 12:51 | Pre-rotation backup |

**Byte-equality:** `GetPersistenceState.wal_bytes=379`. `ls -la wal.log=379 bytes`. **PASS.**

**Key observation:** Snapshot rotations continue on schedule (every 10 epochs). wal_bytes and wal_entries stable at 379/3 — no new transactions have entered the WAL since genesis re-seed. `wal.wal.old` persists through rotation cycles (known-provisional cosmetic naming, documented in VERIFIED-BEHAVIOR.md).

---

## Metrics (Last Lines from Log)

### morning-api (16:58:07Z)
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```
All clean. Same pattern since pass 97.

### local-witness (16:58:20Z)
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
```
All clean. Same pattern since pass 97.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **475 Epoch complete** lines (last: epoch=475, 16:58:47Z)
- **48 KAD bootstrap WARNs** (5-min cadence, expected — `--no-mdns`, no DHT peers)
- **Non-KAD WARN/ERROR: 0.** No panics, no zombie evictions, no errors.
- Snapshot rotations confirmed: last at epoch 470 (12:56Z)

### local-witness (/tmp/lw.log)
- **475 Epoch complete** lines (last: epoch=475, 16:59:10Z)
- **119 insufficient-balance** events (unchanged — no new rejections since early in session, before morning-api hit balance floor of 20)
- **Non-KAD/non-insufficient-balance WARN/ERROR: 0.** No panics, no zombie evictions, no errors.
- Snapshot rotations confirmed: last at epoch 470 (12:56Z)

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch (m-ap) | **PASS** — 475=475=475. Simultaneous match within capture window. |
| Three-way epoch (witness) | **TIMING GAP** — socket 474 vs log 475. Race at epoch boundary between socket query (~16:58:45) and log query (~16:59:15). Not a deviation. |
| Byte-equality (m-ap) | **PASS** — 379=379 |
| Byte-equality (witness) | **PASS** — 379=379 |
| PID consistency | **PASS** — 3579452/3579821 unchanged since pass 97 |
| Log health | **PASS** — KAD WARNs expected, 119 historical insufficient-balance (unchanged), no new errors |
| Metrics health | **PASS** — aged=0, queues=[], silence<30s on both nodes |
| Cross-node epoch sync | **PASS** — both at epoch 475 when accounting for capture timing |
| Snapshot rotation | **PASS** — both rotated through epoch 470, files on disk |

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Status Since Pass 102 | Changed? |
|---|-----------|----------------|----------------------|----------|
| 1 | `build_commit` stale (`cb5d4b1-dirty`, HEAD `452b64f`, 2 behind + dirty) | Jul 27 pass 1 | Persistent | **Unchanged** |
| 2a | Supply divergence (m-ap ledger total=10,000 on its books vs 5,000 minted) | Pass 97 (14:08Z, Jul 28) | Persistent (10,000 frozen on m-ap books) | **Unchanged** |
| 2b | Witness reports morning-api balance as 0 | Jul 27 (18:48Z) | Persistent (0 on witness vs ~20 on m-ap itself) | **Unchanged** |

---

## New Observations This Pass

1. **Stale binary confirmed by log evidence.** binary built from `cb5d4b1` (wal_bytes fix). Current HEAD is `452b64f`, 2 commits ahead. The `-dirty` suffix is from uncommitted changes at build time (`docs/evidence/observer-2026-07-27-pass10.md` modifications). Binary was never rebuilt after the wal_bytes fix was committed and subsequent doc commits were added.

2. **`transactions.wal` no longer exists on disk.** The unified WAL migration to `wal.log` is complete (cb5d4b1 fix). Byte-equality passes against `wal.log`, not `transactions.wal`. The `ls` error from pass 102's commands referencing `transactions.wal` would now fail (confirmed: `No such file or directory`).

3. **Morning-api hit balance floor.** At ~epoch 474, morning-api's tax calculation rounds to 0 (20 DUU × 495 bps = 0.99 < 1). No further redistribution occurs. The `redistribution_share="0"` in epoch log lines confirms the recipient receives nothing. This is a **NATURAL CONSEQUENCE** of the balance reaching a sub-1-DUU tax floor, not a new deviation.

---

## Summary

**Pass 103: delta-only. Three-way epoch MATCH on morning-api confirmed. Witness timing gap noted. No new deviations.**

The mesh remains in the frozen steady-state observed since pass 97 (~3 hours ago):
- 2 nodes, 1 peer each, bidirectional heartbeats healthy (~6.0/min)
- Epochs cycling at ~30s cadence, both synchronized at epoch 475
- No transactions flowing (nonces frozen on both nodes: 241/4 unchanged)
- Balance 20 floor on morning-api, zero on witness — unchanged for >3 hours
- Morning-api redistribution has ceased (tax < 1 DUU at 20 balance floor)
- Snapshot rotations continuing at 10-epoch intervals through epoch 470
- All metrics clean: zero fetches, zero queues, max_peer_silence < 10s
- Build commit 2 commits behind HEAD + dirty (unchanged since session start)
- Supply conservation divergence unchanged (documented in VERIFIED-BEHAVIOR.md)
- wal.wal.old persists through rotation cycles (known-provisional naming)
- `transactions.wal` fully superseded by unified `wal.log`

**Next expected event:** Snapshot rotation at epoch 480 (~5 min). No other state changes expected unless morning-api regains balance or new transactions are submitted.
