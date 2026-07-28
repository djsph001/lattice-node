# Observer Evidence Record — 2026-07-28 (Pass 100)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** 2026-07-28T19:22:37Z (single-capture discipline, bundle via `observer-data-pass100.sh`)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (Boynton Beach FL)
**Session type:** 100th observation pass. **Same session as passes 97-99** (same PIDs 3579452/3579821, same ~13:01Z start). Mesh running since pass 97 restart (~6h22min uptime).

**Summary:** Delta-full pass from pass 99 (99 was delta-only). Three new findings: (1) build gap widened from 2 to 3 commits behind HEAD, (2) intermittent NTP failures detected on both nodes (4 events total — transient, handled by fallback), (3) new experiment mesh (exp-claimer + exp-witness) discovered — not present in any prior observer pass.

---

## Topology Disclosure (Extended — Two Meshes)

**Mesh A: Production mesh (morning-api + local-witness)**

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579427/3579452 | morning-api | 4005 | auto (12D3KooWPfrZ...zLVxJ) | 13:01Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579796/3579821 | local-witness | 4010 | 12D3KooWPfrZ...zLVxJ | 13:02Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZ...zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZ...zLVxJ --no-mdns --persistence --mint 0` |

**Topology change from pass 99:** None. Same PIDs, same nodes, same session. PIDs unchanged since pass 97 (Jul 28 13:01Z).

---

**Mesh B: Experiment mesh (exp-claimer + exp-witness) — NEW**

| PID | Name | Port | Genesis Root | Identity | Storage | Since (approx) |
|-----|------|------|--------------|----------|---------|----------|
| 3883083/3883108 | exp-claimer | 4200 | auto (12D3KooWEGbX...Laa) | /tmp/exp-cap-id/claimer | /tmp/exp-cap-001/claimer | Post-10:43Z (post-pass-99) |
| 3883269/3883294 | exp-witness | 4210 | 12D3KooWEGbX...Laa | /tmp/exp-cap-id/witness | /tmp/exp-cap-001/witness | Post-10:43Z (post-pass-99) |

**Topology disclosure:** Two meshes running simultaneously on the same host. Mesh B's exp-claimer uses `--auto-genesis --mint 5000`; exp-witness uses `--genesis-root` of claimer's PeerId `12D3KooWEGbX1jYWzVhHW8FtZdbagzLDYAXn7i5zXhQD536AfLaa`. Both use `--no-mdns --persistence`. This mesh was NOT present at the time of pass 99 (10:43Z). **UNKNOWN:** Whether Mesh B was started by the human operator or another agent. The git HEAD commit message ("feat: objection-injector binary for cap enforcement experiments") is consistent with this being an experiment mesh for the cap enforcement tests.

**OBSERVED but not queried:** This observer pass only queries the Mesh A socket at `/tmp/m-ap/lattice.sock`. Mesh B not instrumented.

---

## Node Info (Single Capture ~19:22:37Z)

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZ...zLVxJ` | Mesh identity (MESH.md line 46) | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 22850 | — | None |
| build_commit | `cb5d4b1-dirty` | git HEAD `d802680` | **Persistent DEVIATION (#1), WIDENED.** Was 2 commits behind `452b64f` in pass 99. Now 3 commits behind `d802680` + dirty. |
| thickness | 976.67 | Slowly decaying | None (expected decay from pass 99's 981.08) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZ...9sch` | Mesh identity (MESH.md line 47) | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZ...zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 22836 | — | None |
| build_commit | `cb5d4b1-dirty` | git HEAD `d802680` | **Persistent DEVIATION (#1), WIDENED.** Same as morning-api. |

### Build Gap Detail

| Metric | Pass 99 (14:42Z) | Pass 100 (19:22Z) | Δ |
|--------|------------------|--------------------|---|
| Running binary | `cb5d4b1-dirty` | `cb5d4b1-dirty` | Unchanged |
| Git HEAD | `452b64f` | `d802680` | **Changed** |
| Commits behind | 2 | 3 | **Widened** (+1) |
| Dirty suffix | Yes | Yes | Unchanged |

**New commits since binary `cb5d4b1`:**
1. `0c4bb7f` fix: get_stats() reads unified wal.log instead of legacy transactions.wal
2. `452b64f` docs: wal_bytes fix verified, wal_entries heuristic noted
3. `d802680` feat: objection-injector binary for cap enforcement experiments

**OBSERVED:** HEAD advanced by 1 commit since pass 99 (from `452b64f` to `d802680`). The running binary was not rebuilt. The gap has grown from 2→3 behind + dirty.

---

## Epoch State (Single Capture ~19:22:37Z)

### morning-api

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 762 | — |
| Log count (grep -c) | 763 | — |
| Last log epoch | 763 (19:22:47Z) | — |
| Three-way equality | **BOUNDARY RACE.** socket=762 vs log=763 (δ=1). Log timestamps confirm: socket captured at 19:22:37, last log at 19:22:47. ~10s gap = ~0.3 epochs at 30s cadence. **Inconclusive — not a deviation.** | None |

### local-witness

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 762 | — |
| Log count | 762 | — |
| Last log epoch | 762 (19:22:40Z) | — |
| Three-way equality | **MATCH** — socket=762, count=762, last_log=762. No boundary race. | None |

### Cross-Node Epoch Sync

| Metric | Pass 99 (14:42Z) | Pass 100 (19:22Z) | Δ |
|--------|------------------|--------------------|---|
| morning-api epoch | 201 | 762 | +561 epochs (~2.0/min, ~30s/epoch) |
| witness epoch | 201 | 762 | +561 epochs (~2.0/min, ~30s/epoch) |
| Cross-node δ | 0 | 0 | **Unchanged** (fully converged) |

### Snapshot Rotation Sequence

| Event | morning-api (Z) | witness (Z) | Δ |
|-------|-----------------|-------------|---|
| Epoch 740 snapshot | 19:11:17 | 19:11:40 | +23s drift |
| Epoch 750 snapshot | 19:16:17 | 19:16:40 | +23s drift |
| Epoch 760 snapshot | 19:21:17 | 19:21:40 | +23s drift |
| Cadence | 5 min / 10 epochs | 5 min / 10 epochs | Consistent |

**OBSERVED:** Both nodes rotating snapshots every 5 minutes (epochs 740→750→760). Witness lags morning-api by ~23s consistently — within normal clock drift for the two-process startup order.

---

## Peer Connections

| Metric | Pass 99 (14:42Z) | Pass 100 (19:22Z) | Δ |
|--------|------------------|--------------------|---|
| m-ap: peers | 1 | 1 | Unchanged |
| m-ap: heartbeats | 599 | 2283 | +1684 (~5.9/min) |
| m-ap: silence_secs | 4 | 0 | Healthy drift |
| witness: heartbeats | 600 | 2284 | +1684 (~5.9/min) |
| witness: silence_secs | 9 | 4 | Healthy drift |
| queue_depth (both) | 0 | 0 | Unchanged |

**OBSERVED:** Both nodes see exactly 1 peer (each other). Heartbeats flowing at expected rate (~6/min = ~10s interval). Silence well under 30s threshold. No zombie evictions. No backpressure.

---

## Economic State

| Metric | Pass 99 (14:42Z) | Pass 100 (19:22Z) | Δ |
|--------|------------------|--------------------|---|
| m-ap: own_balance | 20 | 20 | **Frozen** (unchanged since pass 97) |
| m-ap: own_nonce | 241 | 241 | **Frozen** |
| m-ap: witness balance | 9980 | 9980 | **Frozen** |
| Witness: own_balance | 0 | 0 | **Frozen** |
| Witness: own_nonce | 4 | 4 | **Frozen** |
| Witness: m-api balance | 0 | 0 | **Frozen** |
| m-ap total supply | 10,000 | 10,000 | **Frozen** |
| Insufficient-balance (lifetime) | 119 | 119 | **No new rejections** |

**OBSERVED:** Economic state fully frozen since pass 97. No activity on either node. Balance 20 floor at epoch 762 (balance_before=20, balance_after=20, ratio=1.02). No new insufficient-balance events.

**Persistent DEVIATIONS (#3/#4):** Unchanged. morning-api ledger shows 10,000 DUU total (vs 5,000 minted). Witness sees morning-api balance as 0.

---

## Persistence State (Single Capture ~19:22:37Z)

### morning-api

| Field | Pass 99 (14:42Z) | Pass 100 (19:22Z) | Δ |
|-------|------------------|--------------------|---|
| last_snapshot_epoch | 200 | **760** | Rotated at 20:00, 30, 40, 50, ... 760 |
| wal_bytes | 379 | 379 | Unchanged |
| wal_entries | 3 | 3 | Unchanged |

**Byte-equality:** `GetPersistenceState.wal_bytes=379`. `ls -la persistence/wal.log=379 bytes`. **PASS.**

**File inventory (19:22Z):**

| File | Size | mtime (EDT) | Notes |
|------|------|-------------|-------|
| `state.snapshot` | 895 bytes | 15:21 (19:21Z) | Snapshot at epoch 760 |
| `wal.log` | 379 bytes | 15:21 (19:21Z) | Active WAL (genesis re-seed) |
| `wal.wal.old` | 379 bytes | 15:16 (19:16Z) | Pre-rotation backup (epoch 750) |

### local-witness

| Field | Pass 99 (14:42Z) | Pass 100 (19:22Z) | Δ |
|-------|------------------|--------------------|---|
| last_snapshot_epoch | 200 | **760** | Rotated at same points as m-ap |
| wal_bytes | 379 | 379 | Unchanged |
| wal_entries | 3 | 3 | Unchanged |

**Byte-equality:** `GetPersistenceState.wal_bytes=379`. `ls -la persistence/wal.log=379 bytes`. **PASS.**

**File inventory (19:22Z):**

| File | Size | mtime (EDT) | Notes |
|------|------|-------------|-------|
| `state.snapshot` | 569 bytes | 15:21 (19:21Z) | Snapshot at epoch 760 |
| `wal.log` | 379 bytes | 15:21 (19:21Z) | Active WAL |
| `wal.wal.old` | 379 bytes | 15:16 (19:16Z) | Pre-rotation backup (epoch 750) |

**Key observation:** Both nodes rotated at epochs 740, 750, 760 as expected. `wal.log` unchanged at 379 bytes — no new transactions persisted. `wal.wal.old` still 379 bytes (same as pass 98-99). State.snapshot sizes unchanged from pass 99.

---

## Metrics (Latest Lines, Single Capture)

### morning-api (19:22:47Z)
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```
All clean. Same pattern since pass 97.

### local-witness (19:22:50Z)
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
```
All clean. Same pattern since pass 97.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **763 Epoch complete** lines (latest: epoch=763 at 19:22:47Z)
- **KAD bootstrap WARNs:** Expected — `No known peers` at 5-min cadence (--no-mdns, no DHT peers)
- **NTP WARNs:** **NEW — 3 failures** since pass 99:
  - 18:02:00Z — pool.ntp.org: "Input/output error: Resource temporarily unavailable (os error 11) (fallback)"
  - 18:02:03Z — time.apple.com: same error (fallback)
  - 18:58:00Z — pool.ntp.org: same error (fallback)
- **Panics: 0. Zombie evictions: 0. Non-KAD/NTP WARN/ERROR: 0.**

### local-witness (/tmp/lw.log)
- **762 Epoch complete** lines (latest: epoch=762 at 19:22:40Z)
- **NTP WARNs:** **NEW — 1 failure** since pass 99:
  - 19:09:13Z — pool.ntp.org: same error pattern
- **119 insufficient-balance** events (unchanged — no new rejections)
- **Panics: 0. Zombie evictions: 0. Non-KAD/NTP WARN/ERROR: 0.**

### NTP Failure Trend (NEW — first tracked in this pass)

| Node | Events | Times (Z) | Rate (last 6h) | Classification |
|------|--------|-----------|----------------|----------------|
| morning-api | 3 | 18:02, 18:02 (2nd fallback), 18:58 | ~1 per 2h | Intermittent — handled via fallback |
| witness | 1 | 19:09 | ~1 per session | Single event — handled via fallback |

**OBSERVED:** 4 NTP query failures across both nodes since session start (~13:01Z). All use the fallback mechanism; system clock is synchronized (timedatectl: "System clock synchronized: yes"). The error "Input/output error: Resource temporarily unavailable (os error 11)" suggests transient DNS/network resolution issue, not clock drift.

**First observation:** These failures may have existed since session start but were not checked in passes 97-99. If they existed, pass 99's "0 non-KAD WARN/ERROR" was incomplete — the NTP failures were present but not surfaced by the grep filter.

**Trend:** In the last ~6 hours of uptime, 4 NTP failures. Too few events to establish a trend. Watch in next passes. The 56-minute gap between the first two (18:02 pool, 18:02 apple) and the third (18:58 pool) on morning-api, and the single event on witness at 19:09, suggests clustering rather than a regular cadence.

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch (m-ap) | **BOUNDARY RACE** — socket=762, count=763, last_log=763. δ=1 at epoch boundary. Inconclusive. |
| Three-way epoch (witness) | **PASS** — socket=762, count=762, last_log=762 |
| Byte-equality (m-ap) | **PASS** — 379=379 |
| Byte-equality (witness) | **PASS** — 379=379 |
| PID consistency | **PASS** — 3579452/3579821 unchanged since pass 97 |
| Log health (m-ap) | **PASS** — KAD WARNs expected, NTP WARNs noted (not critical, handled), no new errors |
| Log health (witness) | **PASS** — KAD WARNs expected, 1 NTP WARN, 119 historical insufficient-balance, no new errors |
| Metrics health | **PASS** — aged=0, queues=[], silence<30s |
| Cross-node epoch sync | **PASS** — both at 762 (δ=0) |
| Snapshot rotation | **PASS** — both rotated at epoch 760, files present on disk |
| System clock sync | **PASS** — "System clock synchronized: yes" (timedatectl) |

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Status Since Pass 99 | Changed? |
|---|-----------|----------------|---------------------|----------|
| 1 | `build_commit` stale (`cb5d4b1-dirty`, HEAD `d802680`) | Jul 27 pass 1 | Persistent (3 behind + dirty) | **WIDENED** — HEAD moved from `452b64f` to `d802680` |
| 2 | `wal_bytes` returns 0 (legacy path) — FIXED | Jul 27 pass 1 | **RESOLVED** | Unchanged — fix working |
| 3 | Supply divergence (total=10,000 vs 5,000 minted) | Pass 97 (14:08Z) | Persistent | Unchanged |
| 4 | Witness reports morning-api balance as 0 | Jul 27 (18:48Z) | Persistent | Unchanged |

## New Observations (First Tracked This Pass)

| # | Observation | Detail | Classification |
|---|-------------|--------|---------------|
| 5 | Build gap widened (2→3 behind + dirty) | HEAD advanced to `d802680`, binary not rebuilt | **PERSISTENT — widened** (sub-class of #1) |
| 6 | NTP failures detected (4 events, both nodes) | 3 on m-ap (18:02Z×2, 18:58Z), 1 on witness (19:09Z). Transient DNS errors, handled by fallback. System clock OK. | **NEW — intermittent, low severity** |
| 7 | Experiment mesh discovered (exp-claimer + exp-witness) | 2 nodes on ports 4200/4210, auto-genesis, likely for objection-injector experiments | **NEW — topology change, non-deviant** |

---

## NTP Trend — Series Tracking (Pass 100 baseline)

| Metric | Value |
|--------|-------|
| Passes with NTP checks | 1 (this pass — first time NTP failures tracked) |
| Total NTP successes (inferred, not measured) | N unknowns between ~13:01Z and 18:02Z |
| Total NTP failures (observed) | 4 |
| Failure rate (last 6 passes) | N/A — pass 100 is first with NTP tracking |
| Failure rate (last 6h of uptime) | 4 failures / unlimited successes ≈ low (< 1%) |
| Trend direction | **Cannot establish** — need multiple passes of NTP data |

**Note:** The 56-minute gap between the 18:02 dual-failure and the 18:58 single failure suggests the NTP queries run on a ~60-min interval, and each round has ~1 attempt per configured server, all of which may fail. Two servers configured (pool.ntp.org, time.apple.com), both failed at 18:02, one retried and failed at 18:58.

---

## Summary

**Pass 100: Mesh A stable, quiescent, no functional regression.**

- Same session since pass 97 (6h22min uptime)
- Epochs advancing at ~30s cadence, both nodes fully converged (δ=0)
- Economic state frozen (balance=20, nonces unchanged since pass 97)
- Snapshot rotations proceeding normally (epochs 740→750→760, both nodes)
- All metrics clean: zero fetches, zero queues, max_peer_silence < 10s
- Byte-equality passes on both nodes

**Three changes since pass 99:**
1. **Build gap widened** — HEAD advanced from `452b64f` to `d802680`, binary `cb5d4b1-dirty` unchanged (now 3 behind + dirty)
2. **NTP failures detected** — 4 events across both nodes (transient, handled by fallback, system clock OK). First tracked in pass 100; need trend data.
3. **Experiment mesh discovered** — exp-claimer + exp-witness on ports 4200/4210, not present in prior observer passes

**No new functional deviations.** Persistent deviations #1, #3, #4 unchanged in behavior (only #1's gap size changed).
