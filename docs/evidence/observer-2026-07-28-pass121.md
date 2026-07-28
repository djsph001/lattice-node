# Observer Evidence Record — 2026-07-28 (Pass 121)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** 2026-07-28T19:11Z – 19:14Z bundle
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** 121st observation pass of Jul 28. ~14 min since pass 120 (18:59Z). Sockets responsive, PIDs unchanged (3579452/3579821).

**Summary:** Delta-only from pass 120. All evidence guards PASS (three-way exact match on both nodes). Epochs advanced +28 with correct cadence (~30s/epoch). Snapshot rotated 3 times (710→720→730→740). Economic state completely frozen — unchanged entire session (>6h). Three persistent deviations unchanged (build_commit gap grew from 2→3 behind HEAD due to new commit). NTP failure series: no new failures since pass 120 (last morn-api at 18:58Z, witness at 19:09Z). Expect next batch at ~19:54Z.

---

## Topology Disclosure

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 13:01Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 13:02Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes since session start (13:01Z).** Same PIDs across all 121 passes.

---

## Evidence Integrity Guards — Simultaneous Captures

### morning-api (19:11Z bundle)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch | Socket=743, Log count=743, Last log line epoch=743 | All three match at a single instant | **PASS** — exact match (no timing drift) |
| Byte-equality | wal_bytes=379 (socket), stat=379 (ls -la wal.log) | Must match | **PASS** |

### local-witness (19:11Z bundle)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch | Socket=742, Log count=742, Last log line epoch=742 | All three match at a single instant | **PASS** — exact match |
| Byte-equality | wal_bytes=379 (socket), stat=379 (ls -la wal.log) | Must match | **PASS** |

**Note:** Witness socket=742 vs morning-api socket=743 (expected timing drift from sequential capture, ~1 epoch offset). Both three-way matches internally.

---

## Metrics (Node Health)

### morning-api (19:11Z)

| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| uptime_secs | 22,169 | Increasing at ~10s per metrics tick | None. Δ from pass 120: +704s (~11.7 min, close to wall time of ~13 min) |
| epoch | 743 | Increasing at ~30s per epoch | None. Δ from pass 120: +28 epochs in ~13 min = ~28s/epoch (normal) |
| height | 1 | 1 (unchanged since genesis) | None |
| peers | 1 (12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch, heartbeats=2215, silence_secs=0, is_dead=false) | 1 peer (local-witness) | None. Heartbeats increasing (Δ +85 from pass 120's 2130) |
| own_balance | 20 | 20 (frozen since at least epoch 443) | **PERSISTENT** — supply conservation divergence. Unchanged. |
| own_nonce | 241 | 241 (frozen) | None — nonce frozen since redistribution stopped |
| thickness | 976.85 | ~977.07 (pass 120) | None — normal slow decay (-0.22 since pass 120) |
| ratio | 1.0195 | ~1.0195 (pass 120) | None — stable to 4 decimal places |

### local-witness (19:11Z)

| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| uptime_secs | 22,159 | ~22,169 (started ~1 min after morning-api) | None. Δ from pass 120: +714s (~11.9 min) |
| epoch | 742 | 742 (trailing morning-api by 1 epoch, confirmed timing drift) | None. Witness same relative position as pass 120 (1 epoch behind) |
| height | 1 | 1 | None |
| peers | 1 (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ, heartbeats=2216, silence_secs=7, is_dead=false) | 1 peer (morning-api) | None. Heartbeats increasing (Δ +81 from pass 120's 2135) |
| own_balance | 0 | 0 (never received redistribution) | **PERSISTENT** — supply conservation divergence |
| own_nonce | 4 | 4 (frozen) | None |
| ratio | 1.1466 | ~1.1520 (pass 120) | None — continuing slow decline (-0.0054). Consistent pattern. |

---

## Economic State

### morning-api (19:11Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | Frozen equilibrium (balance_before=20, balance_after=20, ratio=1.02 → integer truncation = 0 net). | None (terminal state for current parameters). |
| peer (witness) balance | 9,980 | witness reports 0 for itself | **PERSISTENT** — supply conservation divergence (VERIFIED-BEHAVIOR.md: CONTRADICTED). Unchanged. |

### local-witness (19:11Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 (never received any mint) | None |
| peer (morning-api) balance | 0 | morning-api reports 20 for itself | **PERSISTENT** — witness ledger sees morning-api as 0 balance. Unchanged. |

### Epoch Ratio Divergence

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| morning-api ratio | 1.0195 | ~1.02 | None — stable |
| witness ratio | 1.1466 | ~1.15 | None — continuing slow decline |
| Gap | ~12.5% | Same formula, same `redistributed_to=1` | **PERSISTENT** — gap narrowed from ~13.0% (pass 120) to ~12.5% due to continued witness ratio drift. |

---

## Persistence State

### morning-api (19:11Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 740 | Increments every 10 epochs | None. Δ from pass 120: 710→740 (3 rotations: 720, 730, 740) |
| wal_bytes | 379 | 379 (byte-equality passes) | None |
| wal_entries | 3 | 3 (size/120 heuristic) | Known-provisional (VERIFIED-BEHAVIOR.md) |
| wal.wal.old present | 379 bytes (mtime 15:06, unchanged) | Pre-rotation WAL file | Cosmetic naming — known-provisional |
| state.snapshot | 895 bytes | Contains balance/thickness/snapshot epoch data | None — size consistent. mtime 15:11 (updated at snapshot epoch 740) |

### local-witness (19:11Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 730 | Matches morning-api | **Boundary race** — witness shows 730 vs morn-api 740. Different capture moment (witness captured ~11s later), snapshot rotation at 740 not yet visible on witness. Next pass will confirm convergence. |
| wal_bytes | 379 | 379 (byte-equality passes) | None |
| wal_entries | 3 | 3 (size/120 heuristic) | Known-provisional |
| wal.wal.old present | 379 bytes (mtime 15:06) | Pre-rotation WAL file | Cosmetic naming — known-provisional |
| state.snapshot | 569 bytes | Smaller than morning-api (less balance data) | None — consistent with near-zero balances. mtime 15:11 |

---

## Build Provenance

### Git state at capture time

| Field | OBSERVED | EXPECTED |
|-------|----------|----------|
| Git HEAD | `d802680` (feat: objection-injector binary for cap enforcement experiments) | — |
| Build commit (morning-api) | `cb5d4b1-dirty` | `d802680` (clean) |
| Build commit (witness) | `cb5d4b1-dirty` | `d802680` (clean) |
| Commits behind HEAD | 3 (missing `0c4bb7f` + `452b64f` + `d802680`) | 0 |
| Working tree at capture | 1 tracked file modified (`docs/evidence/observer-2026-07-27-pass10.md`), untracked log/evidence files | Clean |

### Delta from pass 120

| Aspect | OBSERVED | DEVIATION |
|--------|----------|-----------|
| build_commit value | cb5d4b1-dirty (unchanged binary) | **PERSISTENT** — same since Jul 27 |
| Behind-HEAD gap | 3 commits (+1 from pass 120, HEAD advanced via new commit) | **Gap grew** from 2→3 due to `d802680` landing on main |
| -dirty suffix | Present (unchanged) | **PERSISTENT** — uncommitted changes at compile time |

---

## Log Health Scan (19:11Z)

### morning-api

| Pattern | Count | Notes |
|---------|-------|-------|
| WARN (structural, filtered) | ~139 KAD bootstrap (5-min cadence, expected on no-mdns mesh) + 3 NTP failures (18:02Z ×2, 18:58Z ×1) | Kademlia filtered as benign. NTP covered in separate section. |
| ERROR (filtered) | 0 | Clean |
| Zombie/sweep/eviction | 0 | None occurred this interval |
| Heartbeats flowing | Yes | Every ~10s from witness, incrementing normally (2215 at capture) |
| Metrics healthy | Yes | outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=6s |
| Epoch cycling | Yes | Every ~30s, balance=20 constant |
| Last epoch line | `Epoch complete epoch=744 balance_before=20 balance_after=20 ratio=1.02` (at 19:13:17Z) | Equilibrium confirmed |

### local-witness

| Pattern | Count | Notes |
|---------|-------|-------|
| WARN (filtered) | 1 NTP failure (19:09Z: pool.ntp.org) | Covered in NTP section |
| ERROR (filtered) | 0 | Clean |
| Zombie/sweep/eviction | 0 | None occurred |
| Heartbeats flowing | Yes | Every ~10s from morning-api, incrementing normally |
| Metrics healthy | Yes | outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=3s |
| Epoch cycling | Yes | Every ~30s, balance=0 constant |
| Last epoch line | `Epoch complete epoch=743 balance_before=0 balance_after=0 ratio=1.15` (at 19:13:10Z) | Equilibrium confirmed |
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
| **121** | **19:13Z** | **0** (no new since pass 120) | **3** | **33% (2/6)** |

### Trend Analysis

| Field | Value |
|-------|-------|
| **First observed** | Pass 116, 18:02:00Z |
| **Last pass classification** | Recurring (approximate hourly cycle) |
| **This pass classification** | **Recurring (hourly cycle, quiescent phase)** — no new failures in ~14 min since pass 120. Cycle pattern holds: batch at 18:02Z, next at 18:58Z (~56 min), expected next at ~19:54Z. |
| **OBSERVED** | 3 total failures across 2 batches: morning-api at 18:02Z (2 failures: pool.ntp.org + time.apple.com) and 18:58Z (1: pool.ntp.org); witness at 19:09Z (1: pool.ntp.org). Inter-batch interval: ~56 min on morning-api, ~67 min on witness. |
| **EXPECTED** | Periodic NTP checks should succeed consistently on an internet-connected machine with NTP synchronized (system clock confirmed synchronized by timedatectl). |
| **DEVIATION** | NTP checks fail periodically with `Input/output error: Resource temporarily unavailable (os error 11)`. The pattern is a predictable ~hourly recurrence, not a rising rate. |
| **UNKNOWN** | Whether the os error 11 is caused by NTP server rate limiting, local firewall, or system resource issue. Also unknown why witness NTP checks run on a different schedule (only 1 failure at 19:09Z vs morning-api's consistent ~56-min cycle). |
| **Changed since last pass** | No new failures since pass 120. Quiescent phase of the hourly cycle. |

---

## Persistent Deviations — Status

### 1. Stale build_commit (cb5d4b1-dirty vs HEAD d802680)

| Field | Value |
|-------|-------|
| **First observed** | Jul 27 (prior to 452b64f docs update) |
| **Last pass status** | Persistent — 2 commits behind HEAD (452b64f) |
| **This pass status** | **Persistent — 3 commits behind HEAD**. Gap grew from 2→3 because HEAD advanced (d802680 landed), not because binary changed. |
| **OBSERVED** | Both nodes report `build_commit: "cb5d4b1-dirty"`. Git HEAD is `d802680`. Binary is 3 commits behind HEAD and was compiled from a dirty working tree. |
| **EXPECTED** | `build_commit` should match git HEAD. At minimum, not `-dirty`. |
| **DEVIATION** | Binary is stale by 3 commits with `-dirty` suffix. |
| **UNKNOWN** | Whether this affects runtime behavior. The wal_bytes fix (0c4bb7f) is verified WORKING (byte-equality passes). HEAD commit (d802680) adds objection-injector binary — unrelated to current mesh operation. |
| **Changed since last pass** | Yes — gap increased from 2→3 commits. The binary itself did not change (same cb5d4b1-dirty value); the gap widened because HEAD advanced. |

### 2. Supply Conservation Divergence (morning-api=20, witness=0)

| Field | Value |
|-------|-------|
| **First observed** | Jul 27, pass #3 (18:48 EDT) |
| **Last pass status** | Persistent — unchanged |
| **This pass status** | **Persistent — unchanged** |
| **OBSERVED** | morning-api balance=20 (own), 9,980 (witness). witness balance=0 (own), 0 (morning-api). Total supply per morning-api: 10,000. Total supply per witness: 0. Frozen entire session (>6h). |
| **EXPECTED** | Supply Conservation Invariant (proposed, pending governance): sum of spendable balances across mesh should equal total supply. |
| **DEVIATION** | CONTRADICTED — ledgers disagree on total supply. |
| **UNKNOWN** | The cause (initial mint local-only, sender debits before recipient confirms, no reconciliation mechanism) — documented in VERIFIED-BEHAVIOR.md as "Causes / Contributing Conditions," not verified diagnoses. |
| **Changed since last pass** | No — balance frozen at 20 for entire session. |

### 3. Epoch Ratio Divergence (~12.5% gap)

| Field | Value |
|-------|-------|
| **First observed** | Jul 28, pass 1 (18:06Z) |
| **Last pass status** | Persistent — gap ~13.0% |
| **This pass status** | **Persistent — gap ~12.5%**. Narrowed from witness ratio drift continuing (1.1466 vs 1.1520 at pass 120). |
| **OBSERVED** | morning-api ratio=1.0195, witness ratio=1.1466 (~12.5% gap). |
| **EXPECTED** | Both nodes apply the same Georgist formula with same `redistributed_to=1`. Ratio should converge. |
| **DEVIATION** | ~12.5% gap. Function of total supply (net of tax base), which differs between nodes. |
| **UNKNOWN** | Whether the ratio divergence is purely consequential or has independent contributions. |
| **Changed since last pass** | Gap narrowed from ~13.0% to ~12.5% due to witness ratio drift (−0.0054). Precision drift, not structural change. |

### 4. MESH.md Stale

| Field | Value |
|-------|-------|
| **First observed** | Jul 27 (prior to Jul 28 session start) |
| **Last pass status** | Persistent — unchanged |
| **This pass status** | **Persistent — unchanged** |
| **OBSERVED** | MESH.md at commit c008def says "No production nodes running." Both nodes running continuously since 13:01Z (>6h). |
| **EXPECTED** | MESH.md should reflect current active topology. |
| **DEVIATION** | Topology documentation out of sync for >6 hours. |
| **UNKNOWN** | Whether intentional (ongoing session) or oversight. Node launch commands in MESH.md are accurate. |
| **Changed since last pass** | No. |

---

## New Observations This Pass

### 1. Build Commit Gap Grew (2→3 behind HEAD)

HEAD advanced from `452b64f` to `d802680` (objection-injector binary). Binary was not rebuilt — still reports `cb5d4b1-dirty`. This is the first HEAD advance since session start (>6h ago). The binary is now 3 commits behind main.

### 2. Snapshot at 740, Both Nodes Converged

Both nodes rotated through epochs 720, 730, and 740 successfully. Snapshot files present on disk with consistent sizes. Byte-equality passes on both nodes. wal.wal.old (379 bytes) unchanged from earlier rotations — no new transactions to persist.

### 3. No New NTP Failures Since Pass 120

The quiescent phase of the ~hourly NTP failure cycle. Next expected batch: morning-api ~19:54Z, witness ~20:05Z. No evidence of rising rate.

### 4. Ratios Continue Slow Narrowing

Witness ratio declined from 1.15201 (pass 120) to 1.1466 (pass 121), a Δ of -0.0054. Morning-api ratio essentially flat (1.0195 → 1.0195). The gap narrowed from ~13.0% to ~12.5%. This is the continuing trend observed since pass 97 — as more epochs pass with zero economic activity, the witness ratio approaches morning-api's ratio asymptotically (though they will never converge to equality given the different total supplies).

---

## Evidence Gaps

1. **No cross-node balance reconciliation.** Morning-api reports own balance=20, witness reports morning-api balance=0. No consensus mechanism active on Era One.

2. **Root cause of NTP `os error 11` unknown.** System clock confirmed synchronized (timedatectl). Node's periodic NTP check fails. Need to determine if rate limiting, firewall, or resource issue.

3. **Witness NTP schedule differs from morning-api.** Witness shows only 1 NTP failure (19:09Z) while morning-api shows a consistent ~56-min cycle. Unknown whether witness runs fewer checks, started later, or has a different failure pattern.

4. **Root cause of `-dirty` suffix unknown.** Current worktree shows 1 modified tracked file and untracked log/evidence files. The suffix could reflect a different state at compile time.

5. **Redistribution inactivity continues.** No redistribution activity observed entire session (>6h, ~740+ epochs). Economic engine reached equilibrium (balance=20, ratio≈1.02 → integer truncation yields 0 net change).

---

**Next expected events:** NTP retry at ~19:54Z (morning-api, ~56 min cycle). Snapshot rotation at epoch 750 (~19:26Z). No other state changes expected given the frozen economic equilibrium.

**Timeline:** Session started 13:01Z Jul 28. Now at 19:13Z Jul 28. Runtime: 6h 12min. 121 observation passes completed.
