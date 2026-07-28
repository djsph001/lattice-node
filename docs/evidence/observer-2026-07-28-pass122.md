# Observer Evidence Record — 2026-07-28 (Pass 122)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** 2026-07-28T19:31Z – 19:32Z (bundle), re-verify at 19:33Z
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** 122nd observation pass of Jul 28. ~20 min since pass 121 (19:13Z). Sockets responsive, PIDs unchanged for main mesh (3579452/3579821). **NEW:** exp-claimer/exp-witness experiment started (~15:30Z).

**Summary:** Delta from pass 121. All evidence guards PASS on both main mesh nodes (three-way exact match on witness, boundary race on first capture of morning-api resolved to clean match on re-verify). Epochs advanced +38 on morning-api, +39 on witness with correct cadence. Snapshot rotated 4 times (740→750→760→770→780). Economic state completely frozen — unchanged entire session (>6.5h). Three persistent deviations unchanged (balance divergence, build_commit gap, stale MESH.md). NTP failure series: no new failures since pass 121 (last morn-api at 18:58Z, last witness at 19:09Z). New experiment (exp-cap-002) is a separate isolated mesh on ports 4300/4310 with its own genesis — not connected to the main mesh.

---

## Topology Disclosure

**This machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Role in mesh:** Host for all processes (z4-workstation)

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 13:01Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 13:02Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |
| 3903468 | exp-claimer | 4300 | auto (12D3KooWFpPWrq1pMRamua8AkisnA5g6AQ8h3EjgENTk9WYYsnJn) | ~15:30Z | `--name exp-claimer --port 4300 --identity-dir /tmp/exp-cap-id/claimer --storage-dir /tmp/exp-cap-002/claimer --auto-genesis --no-mdns --persistence --mint 5000` |
| 3903752 | exp-witness | 4310 | 12D3KooWFpPWrq1pMRamua8AkisnA5g6AQ8h3EjgENTk9WYYsnJn | ~15:30Z | `--name exp-witness --port 4310 --identity-dir /tmp/exp-cap-id/witness --storage-dir /tmp/exp-cap-002/witness --genesis-root 12D3KooWFpPWrq1pMRamua8AkisnA5g6AQ8h3EjgENTk9WYYsnJn --bootstrap-peer /ip4/127.0.0.1/tcp/4300/p2p/12D3KooWFpPWrq1pMRamua8AkisnA5g6AQ8h3EjgENTk9WYYsnJn --no-mdns --persistence --mint 0` |

**Topology changes since last pass:** No changes to main mesh nodes (same PIDs, same commands). NEW: exp-claimer/exp-witness (exp-cap-002) launched between pass 121 and pass 122. This is a separate isolated mesh — different ports, different genesis PeerId, NOT connected to the morning-api/witness mesh.

---

## Evidence Integrity Guards — Simultaneous Captures (19:33Z re-verify)

### morning-api (19:33Z)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch | Socket=782, Log count=782, Last log line epoch=782 | All three match at a single instant | **PASS** — exact match |
| Byte-equality | wal_bytes=379 (socket), stat=379 (ls -la wal.log) | Must match | **PASS** |

### local-witness (19:33Z)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch | Socket=781, Log count=781, Last log line epoch=781 | All three match at a single instant | **PASS** — exact match |
| Byte-equality | wal_bytes=379 (socket), stat=379 (ls -la wal.log) | Must match | **PASS** |

**Note:** Witness socket=781 vs morning-api socket=782 (expected timing drift from sequential capture, ~1 epoch offset). Both three-way matches internally.

**First capture (19:31Z) — morning-api boundary race:** Initial socket read returned epoch=779 while log showed 780. This was a genuine epoch boundary race (~14s between socket query and last log line write). Re-verify at 19:33Z resolved to clean match. Witness first capture was clean (socket=779, log=779, last epoch=779 at the time, subsequently advanced to 781).

---

## Metrics (Node Health)

### morning-api (19:31Z bundle — approximate, re-verify used for three-way)

| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| uptime_secs | 23,359 (19:31Z) → 23,452 (19:33Z) | ~23,359 (pass 121: 22,169. Δ +1,190 in ~20 min ≈ ~59s/s, close to wall clock) | None |
| epoch | 782 (19:33Z) | Increasing at ~30s per epoch | None. Δ from pass 121 (743→782): +39 epochs in ~20 min = ~30.8s/epoch (normal) |
| height | 1 | 1 (unchanged since genesis) | None |
| peers | 1 (12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch, heartbeats=2333, silence_secs=9, is_dead=false) | 1 peer (local-witness) | None. Heartbeats increasing (Δ +118 from pass 121's 2215) |
| own_balance | 20 | 20 (frozen since at least epoch 443) | **PERSISTENT** — supply conservation divergence. Unchanged. |
| own_nonce | 241 | 241 (frozen) | None — nonce frozen since redistribution stopped |
| thickness | 976.52 (19:33Z) | ~976.85 (pass 121). Δ -0.33 in ~20 min | None — normal slow decay |
| ratio | 1.01956 | ~1.0195 (pass 121) | None — stable to 4 decimal places |

### local-witness (19:31Z bundle — approximate)

| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| uptime_secs | 23,352 (19:31Z) | ~23,359 (started ~1 min after morning-api) | None. Δ from pass 121: +1,193s in ~20 min |
| epoch | 781 (19:33Z) | ~782 (trailing morning-api by 1 epoch, confirmed timing drift) | None. Witness same relative position as pass 121 (1 epoch behind) |
| height | 1 | 1 | None |
| peers | 1 (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ, heartbeats=2335, silence_secs=9, is_dead=false) | 1 peer (morning-api) | None. Heartbeats increasing (Δ +119 from pass 121's 2216) |
| own_balance | 0 | 0 (never received redistribution) | **PERSISTENT** — supply conservation divergence |
| own_nonce | 4 | 4 (frozen) | None |
| ratio | 1.13876 | ~1.1466 (pass 121). Δ -0.0078 | None — continuing slow decline. Consistent pattern. |

---

## Economic State

### morning-api (19:31Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | Frozen equilibrium (balance_before=20, balance_after=20, ratio=1.02 → integer truncation = 0 net). | None (terminal state for current parameters). |
| peer (witness) balance | 9,980 | witness reports 0 for itself | **PERSISTENT** — supply conservation divergence (VERIFIED-BEHAVIOR.md: CONTRADICTED). Unchanged. |

### local-witness (19:31Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 (never received any mint) | None |
| peer (morning-api) balance | 0 | morning-api reports 20 for itself | **PERSISTENT** — witness ledger sees morning-api as 0 balance. Unchanged. |

### Epoch Ratio Divergence

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| morning-api ratio | 1.01956 | ~1.02 | None — stable |
| witness ratio | 1.13876 | ~1.15 | None — continuing slow decline (Δ -0.0078 since pass 121) |
| Gap | ~11.7% | Same formula, same `redistributed_to=1` | **PERSISTENT** — gap narrowed from ~12.5% (pass 121) to ~11.7% due to continued witness ratio drift. |

---

## Persistence State

### morning-api (19:33Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 780 | Increments every 10 epochs | None. Δ from pass 121: 740→780 (4 rotations: 750, 760, 770, 780) |
| wal_bytes | 379 | 379 (byte-equality passes) | None |
| wal_entries | 3 | 3 (size/120 heuristic) | Known-provisional (VERIFIED-BEHAVIOR.md) |
| wal.wal.old present | 379 bytes (mtime 15:21) | Pre-rotation WAL file | Cosmetic naming — known-provisional |
| state.snapshot | — | Contains balance/thickness/snapshot epoch data | mtime 15:31 (updated at snapshot epoch 780) |

### local-witness (19:33Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 780 | Matches morning-api | None — **converged**. Witness now shows same snapshot epoch 780 (was 730 at pass 121). |
| wal_bytes | 379 | 379 (byte-equality passes) | None |
| wal_entries | 3 | 3 (size/120 heuristic) | Known-provisional |
| wal.wal.old present | 379 bytes (mtime 15:26) | Pre-rotation WAL file | Cosmetic naming — known-provisional |
| state.snapshot | — | Smaller than morning-api (less balance data) | mtime 15:31 |

---

## Build Provenance

### Git state at capture time

| Field | OBSERVED | EXPECTED |
|-------|----------|----------|
| Git HEAD | `8b329b7` (fix: objection-injector waits for mesh peers and lingers after publish) | — |
| Build commit (morning-api) | `cb5d4b1-dirty` | `8b329b7` (clean) |
| Build commit (witness) | `cb5d4b1-dirty` | `8b329b7` (clean) |
| Commits behind HEAD | 4 (missing `0c4bb7f` + `452b64f` + `d802680` + `8b329b7`) | 0 |
| Working tree at capture | 1 tracked file modified (`docs/evidence/observer-2026-07-27-pass10.md`), untracked log/evidence files | Clean |

### Delta from pass 121

| Aspect | OBSERVED | DEVIATION |
|--------|----------|-----------|
| build_commit value | cb5d4b1-dirty (unchanged binary) | **PERSISTENT** — same since Jul 27 |
| Behind-HEAD gap | 4 commits (+1 from pass 121, HEAD advanced via new commit) | **Gap grew** from 3→4 due to `8b329b7` landing on main |
| -dirty suffix | Present (unchanged) | **PERSISTENT** — uncommitted changes at compile time |

---

## Log Health Scan (19:31Z)

### morning-api

| Pattern | Count | Notes |
|---------|-------|-------|
| WARN (structural, filtered) | ~157 KAD bootstrap (5-min cadence, expected on no-mdns mesh) + 3 NTP failures (18:02Z ×2, 18:58Z ×1) | Kademlia filtered as benign. NTP covered in separate section. |
| ERROR (filtered) | 0 | Clean |
| Zombie/sweep/eviction | 0 | None occurred this interval |
| Heartbeats flowing | Yes | Every ~10s from witness, incrementing normally (2,333 at capture) |
| Metrics healthy | Yes | outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=9s |
| Epoch cycling | Yes | Every ~30s, balance=20 constant |
| Last epoch line | `Epoch complete epoch=782 balance_before=20 balance_after=20 ratio=1.02` (at 19:32:17Z) | Equilibrium confirmed |

### local-witness

| Pattern | Count | Notes |
|---------|-------|-------|
| WARN (filtered) | 1 NTP failure (19:09Z: pool.ntp.org) | Covered in NTP section |
| ERROR (filtered) | 0 | Clean |
| Zombie/sweep/eviction | 0 | None occurred |
| Heartbeats flowing | Yes | Every ~10s from morning-api, incrementing normally |
| Metrics healthy | Yes | outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=9s |
| Epoch cycling | Yes | Every ~30s, balance=0 constant |
| Last epoch line | `Epoch complete epoch=781 balance_before=0 balance_after=0 ratio=1.14` (at 19:32:10Z) | Equilibrium confirmed |
| Insufficient-balance rejections | 119 (unchanged entire session) | Historical only — no new rejections |

---

## NTP Failure Series — Trend Report

### Data Since Pass 115

| Pass | Capture Time (Z) | NTP Failures | Cumulative | Trailing 6-Pass Rate |
|------|------------------|-------------|------------|----------------------|
| 115 | ~18:01Z | 0 (not yet occurred) | 0 | — |
| 116 | ~18:10Z | 2 (18:02Z: pool.ntp.org + time.apple.com) | 2 | 33% (1/3) |
| 117 | ~18:11Z | 0 | 2 | 33% (1/3) |
| 118 | ~18:40Z | 0 | 2 | 33% (2/6) |
| 119 | ~18:49Z | 0 | 2 | 33% (2/6) |
| 120 | ~18:59Z | 1 (18:58Z: pool.ntp.org) | 3 | 50% (3/6) |
| 121 | ~19:13Z | 0 | 3 | 33% (2/6) |
| **122** | **19:31Z** | **0** (no new since pass 121) | **3** | **33% (2/6)** |

### Trend Analysis

| Field | Value |
|-------|-------|
| **First observed** | Pass 116, 18:02:00Z |
| **Last pass classification** | Recurring (hourly cycle, quiescent phase) |
| **This pass classification** | **Recurring (hourly cycle, quiescent phase)** — no new failures in ~33 min since pass 121. Last morning-api batch: 18:58Z. Next expected: ~19:54Z. |
| **OBSERVED** | 3 total failures across 2 batches: morning-api at 18:02Z (2 failures: pool.ntp.org + time.apple.com) and 18:58Z (1: pool.ntp.org); witness at 19:09Z (1: pool.ntp.org). Inter-batch interval: ~56 min on morning-api. |
| **EXPECTED** | Periodic NTP checks should succeed consistently on an internet-connected machine with NTP synchronized. |
| **DEVIATION** | NTP checks fail periodically with `Input/output error: Resource temporarily unavailable (os error 11)`. The pattern is a predictable ~hourly recurrence, not a rising rate. |
| **UNKNOWN** | Whether the os error 11 is caused by NTP server rate limiting, local firewall, or system resource issue. Also unknown why witness NTP checks run on a different schedule (only 1 failure at 19:09Z vs morning-api's consistent ~56-min cycle). |
| **Changed since last pass** | No new failures since pass 121. Quiescent phase of the hourly cycle. |

---

## New Experiment: exp-cap-002 (exp-claimer + exp-witness)

| Field | exp-claimer | exp-witness |
|-------|------------|-------------|
| PeerId | `12D3KooWFpPWrq1pMRamua8AkisnA5g6AQ8h3EjgENTk9WYYsnJn` | `12D3KooWDgmmBPUwew3aeLgVjwnSGgXUHmQLqfJRe2DBDbJdDmFp` |
| Port | 4300 | 4310 |
| Genesis | auto | claimer's PeerId |
| Mint | 5,000 | 0 |
| Uptime | ~133s (19:33Z) | ~118s (19:33Z) |
| Epoch | 5 | 5 |
| Peers | 1 (witness) | 1 (claimer) |
| Balance | not queried (own) | not queried (own) |
| Ratio | 0.904 | 1.485 |
| Tax calculated/collected | 246/246 | 0/0 |
| Distinct witnesses | 1 | not reported |
| Build commit | cb5d4b1-dirty | cb5d4b1-dirty |

**Isolated mesh:** Different ports (4300/4310), different genesis PeerIds, not connected to the main morning-api/witness mesh. Fresh state with --mint 5000 on claimer. Tax already active at epoch 5 with 246 DUU calculated (redistributed_to=1). Witness ratio already diverging (1.485 vs 0.904 claimer). This is a new experiment — likely for cap enforcement testing given the `exp-cap` naming.

---

## Persistent Deviations — Status

### 1. Stale build_commit (cb5d4b1-dirty vs HEAD 8b329b7)

| Field | Value |
|-------|-------|
| **First observed** | Jul 27 (prior to 452b64f docs update) |
| **Last pass status** | Persistent — 3 commits behind HEAD (d802680) |
| **This pass status** | **Persistent — 4 commits behind HEAD**. Gap grew from 3→4 because HEAD advanced (8b329b7 landed), not because binary changed. |
| **OBSERVED** | Both nodes report `build_commit: "cb5d4b1-dirty"`. Git HEAD is `8b329b7`. Binary is 4 commits behind HEAD and was compiled from a dirty working tree. |
| **EXPECTED** | `build_commit` should match git HEAD. At minimum, not `-dirty`. |
| **DEVIATION** | Binary is stale by 4 commits with `-dirty` suffix. |
| **UNKNOWN** | Whether this affects runtime behavior. The wal_bytes fix (0c4bb7f) is verified WORKING (byte-equality passes). HEAD commit (8b329b7) fixes objection-injector wait logic — unrelated to current mesh operation. |
| **Changed since last pass** | Yes — gap increased from 3→4 commits. The binary itself did not change (same cb5d4b1-dirty value); the gap widened because HEAD advanced. |

### 2. Supply Conservation Divergence (morning-api=20, witness=0)

| Field | Value |
|-------|-------|
| **First observed** | Jul 27, pass #3 (18:48 EDT) |
| **Last pass status** | Persistent — unchanged |
| **This pass status** | **Persistent — unchanged** |
| **OBSERVED** | morning-api balance=20 (own), 9,980 (witness). witness balance=0 (own), 0 (morning-api). Total supply per morning-api: 10,000. Total supply per witness: 0. Frozen entire session (>6.5h). |
| **EXPECTED** | Supply Conservation Invariant (proposed, pending governance): sum of spendable balances across mesh should equal total supply. |
| **DEVIATION** | CONTRADICTED — ledgers disagree on total supply. |
| **UNKNOWN** | The cause (initial mint local-only, sender debits before recipient confirms, no reconciliation mechanism) — documented in VERIFIED-BEHAVIOR.md as "Causes / Contributing Conditions," not verified diagnoses. |
| **Changed since last pass** | No — balance frozen at 20 for entire session. |

### 3. Epoch Ratio Divergence (~11.7% gap)

| Field | Value |
|-------|-------|
| **First observed** | Jul 28, pass 1 (18:06Z) |
| **Last pass status** | Persistent — gap ~12.5% |
| **This pass status** | **Persistent — gap ~11.7%**. Narrowed from witness ratio drift continuing (1.1388 vs 1.1466 at pass 121). |
| **OBSERVED** | morning-api ratio=1.0196, witness ratio=1.1388 (~11.7% gap). |
| **EXPECTED** | Both nodes apply the same Georgist formula with same `redistributed_to=1`. Ratio should converge. |
| **DEVIATION** | ~11.7% gap. Function of total supply (net of tax base), which differs between nodes. |
| **UNKNOWN** | Whether the ratio divergence is purely consequential or has independent contributions. |
| **Changed since last pass** | Gap narrowed from ~12.5% to ~11.7% due to witness ratio drift (−0.0078). Precision drift, not structural change. |

### 4. MESH.md Stale

| Field | Value |
|-------|-------|
| **First observed** | Jul 27 (prior to Jul 28 session start) |
| **Last pass status** | Persistent — unchanged |
| **This pass status** | **Persistent — unchanged** |
| **OBSERVED** | MESH.md at commit c008def says "No production nodes running." Both nodes running continuously since 13:01Z (>6.5h). |
| **EXPECTED** | MESH.md should reflect current active topology. |
| **DEVIATION** | Topology documentation out of sync for >6.5 hours. |
| **UNKNOWN** | Whether intentional (ongoing session) or oversight. Node launch commands in MESH.md are accurate. |
| **Changed since last pass** | No. |

---

## New Observations This Pass

### 1. Build Commit Gap Grew (3→4 behind HEAD)

HEAD advanced from `d802680` to `8b329b7` (objection-injector wait fix). Binary was not rebuilt — still reports `cb5d4b1-dirty`. Binary is now 4 commits behind main.

### 2. Snapshot at 780, Both Nodes Converged and Rotating Consistently

Both nodes rotated through epochs 750, 760, 770, and 780 successfully. Snapshot files present with consistent mtimes (15:31). Byte-equality passes on both nodes (379 bytes). Witness snapshot epoch (780) now matches morning-api (780) — witness caught up since pass 121 when it was at 730 (boundary race).

### 3. No New NTP Failures Since Pass 121

Quiescent phase of the ~hourly NTP failure cycle. Next expected batch: morning-api ~19:54Z. No evidence of rising rate (33% trailing 6-pass rate, same as pass 121).

### 4. New Experiment: exp-cap-002 Started (~15:30Z)

Isolated two-node mesh (exp-claimer on port 4300, exp-witness on port 4310) with same binary (cb5d4b1-dirty). Claimer has 5000 mint, witness 0. Already 5 epochs old at capture, tax active (246 calculated/collected). Witness ratio already diverging (1.485 vs 0.904). Name suggests cap-enforcement experiment. Launched between pass 121 and 122 (~4 hours after main mesh).

### 5. Witness Snapshot Converged to Morning-api (both at 780)

At pass 121, witness had last_snapshot_epoch=730 vs morning-api's 740. At pass 122, both show 780. The witness caught up through 4 rotation cycles (740→750→760→770→780). The earlier gap was a boundary race, not a real divergence.

### 6. Ratio Gap Continues Narrowing

Witness ratio declined from 1.1466 (pass 121) to 1.1388 (pass 122), a Δ of -0.0078. Morning-api ratio essentially flat (1.0195 → 1.0196). The gap narrowed from ~12.5% to ~11.7%. This is the continuing trend — as more epochs pass with zero economic activity, the witness ratio asymptotically approaches morning-api's ratio. Gap narrowing rate: ~0.8% per 20 min ≈ ~2.4%/hour.

---

## Evidence Gaps

1. **No cross-node balance reconciliation.** Morning-api reports own balance=20, witness reports morning-api balance=0. No consensus mechanism active on Era One.

2. **Root cause of NTP `os error 11` unknown.** System clock confirmed synchronized (timedatectl). Node's periodic NTP check fails ~hourly. Need to determine if rate limiting, firewall, or resource issue.

3. **Witness NTP schedule differs from morning-api.** Witness shows only 1 NTP failure (19:09Z) while morning-api shows a consistent ~56-min cycle. Unknown whether witness runs fewer checks, started later, or has a different failure pattern.

4. **Root cause of `-dirty` suffix unknown.** Current worktree shows 1 modified tracked file and untracked log/evidence files. The suffix could reflect a different state at compile time.

5. **Redistribution inactivity continues.** No redistribution activity observed entire session (>6.5h, ~780+ epochs). Economic engine reached equilibrium (balance=20, ratio≈1.02 → integer truncation yields 0 net change).

6. **exp-cap-002 experiment purpose unknown.** The experiment launched between pass 121 and 122. Its purpose, expected duration, and relationship to the main mesh are undocumented from this observer's perspective. It uses the same stale binary. UNKNOWN whether the experiment will interact with the main mesh or is completely isolated.

---

**Next expected events:** NTP retry at ~19:54Z (morning-api, ~56 min cycle). Snapshot rotation at epoch 790 (~19:36Z, already likely complete). No other state changes expected given the frozen economic equilibrium.

**Timeline:** Session started 13:01Z Jul 28. Now at 19:31Z Jul 28. Runtime: 6h 30min. 122 observation passes completed.
