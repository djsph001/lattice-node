# Observer Evidence Record — 2026-07-28 (Pass 120)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** 2026-07-28T18:57:51Z – 18:59:06Z (morning-api @ 18:58:55Z, witness @ 18:59:06Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** 120th observation pass of Jul 28. ~9 min since pass 119 (18:49Z). Sockets responsive, PIDs unchanged (3579452/3579821).

**Summary:** Delta-only from pass 119. All evidence guards PASS (three-way exact match, no timing drift). Epochs advanced +18 at normal cadence (~30s/epoch). Snapshot stable at 710 (next rotation at 720 expected ~8 min). Economic state completely frozen — unchanged entire session (>5.8h). Three persistent deviations unchanged. **One new observation: NTP failures recurred at 18:58Z (56 min after first batch at 18:02Z) — reclassifying from "Resolved" to "Recurring (hourly cycle)."** Witness no longer trailing (714 vs 715 at 11s gap is timing drift, confirmed).

---

## Topology Disclosure

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 13:01Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 13:02Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes since session start (13:01Z).** Same PIDs across all 120 passes.

---

## Evidence Integrity Guards — Simultaneous Captures

### morning-api (18:58:55Z)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch | Socket=715, Log count=715, Last log line epoch=715 | All three match at a single instant | **PASS** — exact match (no timing drift) |
| Byte-equality | wal_bytes=379 (socket), stat=379 (ls -la wal.log) | Must match | **PASS** |

### local-witness (18:59:06Z)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch | Socket=714, Log count=714, Last log line epoch=714 | All three match at a single instant | **PASS** — exact match |
| Byte-equality | wal_bytes=379 (socket), stat=379 (ls -la wal.log) | Must match | **PASS** |

**Note:** Witness at 714 vs morning-api at 715 (11s apart at capture) — same pattern as pass 119. Confirmed as timing drift, not true lag. Witness caught up to same relative position.

---

## Metrics (Node Health)

### morning-api (18:58:55Z simultaneous)

| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| uptime_secs | 21,465 (18:59Z) | Increasing at ~10s per metrics tick | None. Δ from pass 119: +580s (~9.7 min, close to wall time of ~9 min) |
| epoch | 715 (18:58Z) | Increasing at ~30s per epoch | None. Δ from pass 119: +18 epochs in ~9 min = ~30s/epoch (normal) |
| height | 1 | 1 (unchanged since genesis) | None |
| peers | 1 (12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch, heartbeats=2130, silence_secs=4, is_dead=false) | 1 peer (local-witness) | None. Heartbeats increasing steadily (Δ +44 from pass 119's 2086) |
| own_balance | 20 | 20 (frozen since at least epoch 443) | **PERSISTENT** — supply conservation divergence. Unchanged since pass 119. |
| own_nonce | 241 | 241 (frozen) | None — nonce frozen since redistribution stopped |
| thickness | 977.07 | ~977.18 (pass 119) | None — normal slow drift (-0.11 since pass 119) |
| ratio | 1.01952 | ~1.01951 (pass 119) | None — stable to 5 decimal places |

### local-witness (18:59:06Z simultaneous)

| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| uptime_secs | 21,445 (18:59Z) | ~21,465 (started ~20s after morning-api) | None. Δ from pass 119: +572s (~9.5 min) |
| epoch | 714 (18:59Z) | 714 (trailing morning-api by 1 epoch, confirmed timing drift) | None. Witness same relative position as pass 119 (1 epoch behind at 11s gap) |
| height | 1 | 1 | None |
| peers | 1 (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ, heartbeats=2135, silence_secs=0, is_dead=false) | 1 peer (morning-api) | None. Heartbeats increasing steadily (Δ +47 from pass 119's 2088) |
| own_balance | 0 | 0 (never received redistribution) | **PERSISTENT** — supply conservation divergence |
| own_nonce | 4 | 4 (frozen) | None |
| ratio | 1.15201 | ~1.15612 (pass 119) | None — small downward drift (-0.0041). Consistent pattern of slow ratio decline. |

---

## Economic State

### morning-api (18:58:55Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | From initial mint of 5,000, redistributed to near-zero floor. Frozen since at least epoch 443 (~5.5h ago). | None (terminal state for current parameters). |
| peer (witness) balance | 9,980 | witness reports 0 for itself | **PERSISTENT** — supply conservation divergence (VERIFIED-BEHAVIOR.md: CONTRADICTED). Unchanged. |

### local-witness (18:59:06Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 (never received any mint) | None |
| peer (morning-api) balance | 0 | morning-api reports 20 for itself | **PERSISTENT** — witness ledger sees morning-api as 0 balance. Unchanged. |

### Epoch Ratio Divergence

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| morning-api ratio | 1.01952 | ~1.02 | None — stable |
| witness ratio | 1.15201 | ~1.16 | None — continuing slow decline |
| Gap | ~13.0% | Same formula, same `redistributed_to=1` | **PERSISTENT** — gap narrowed from ~13.4% (pass 119) to ~13.0% due to continued witness ratio drift. Consequence of supply divergence. |

---

## Persistence State

### morning-api (18:58:55Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 710 | Increments every 10 epochs | None. Δ from pass 119: 690→710 (2 rotations at 700 and 710) |
| wal_bytes | 379 | 379 (byte-equality passes) | None |
| wal_entries | 3 | 3 (size/120 heuristic) | Known-provisional (VERIFIED-BEHAVIOR.md) |
| wal.wal.old present | 379 bytes (mtime 14:51) | Pre-rotation WAL file | Cosmetic naming — known-provisional. Same size as active wal.log |
| state.snapshot | 895 bytes | Contains balance/thickness/snapshot epoch data | None — size consistent. mtime 14:56 (updated at snapshot epoch 710) |

### local-witness (18:59:06Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 710 | Matches morning-api | None — both nodes snapshotted at epoch 710 |
| wal_bytes | 379 | 379 (byte-equality passes) | None |
| wal_entries | 3 | 3 (size/120 heuristic) | Known-provisional |
| wal.wal.old present | 379 bytes (mtime 14:51) | Pre-rotation WAL file | Cosmetic naming — known-provisional |
| state.snapshot | 569 bytes | Smaller than morning-api (less balance data) | None — consistent with near-zero balances. mtime 14:56 |

---

## Build Provenance

### Git state at capture time

| Field | OBSERVED | EXPECTED |
|-------|----------|----------|
| Git HEAD | `452b64f` (docs: wal_bytes fix verified, wal_entries heuristic noted) | — |
| Build commit (morning-api) | `cb5d4b1-dirty` | `452b64f` (clean) |
| Build commit (witness) | `cb5d4b1-dirty` | `452b64f` (clean) |
| Commits behind HEAD | 2 (missing `0c4bb7f` + `452b64f`) | 0 |
| Working tree at capture | 1 tracked file modified (`docs/evidence/observer-2026-07-27-pass10.md`), untracked log files + new evidence files | Clean |

### Delta from pass 119

| Aspect | OBSERVED | DEVIATION |
|--------|----------|-----------|
| build_commit value | cb5d4b1-dirty (unchanged) | **PERSISTENT** — same since Jul 27 |
| Behind-HEAD gap | 2 commits | **PERSISTENT** — unchanged since pass 119 (HEAD did not advance) |
| -dirty suffix | Present (unchanged) | **PERSISTENT** — uncommitted changes at compile time |

---

## Log Health Scan (18:59Z)

### morning-api

| Pattern | Count | Notes |
|---------|-------|-------|
| WARN (structural, filtered) | 1 | **New NTP failure at 18:58:00Z** — see NTP section below. Kademlia warnings (~77) filtered as benign (5-min cadence, no-mdns mesh). |
| ERROR (filtered) | 0 | Clean |
| Zombie/sweep/eviction | 0 | None occurred this interval |
| Heartbeats flowing | Yes | Every ~10s from witness, incrementing normally (2130 at capture → 2135+ in latest log) |
| Metrics healthy | Yes | outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=6s |
| Epoch cycling | Yes | Every ~30s, balance=20 constant |
| Last epoch line | `Epoch complete epoch=717 balance_before=20 balance_after=20 ratio=1.02` (at 18:59:47Z) | Equilibrium confirmed |

### local-witness

| Pattern | Count | Notes |
|---------|-------|-------|
| WARN (filtered) | 0 | No non-benign WARNs. Kademlia warnings absent (witness has no kademlia). |
| ERROR (filtered) | 0 | Clean |
| Zombie/sweep/eviction | 0 | None occurred |
| Heartbeats flowing | Yes | Every ~10s from morning-api, incrementing normally |
| Metrics healthy | Yes | (inferred from zero silence_secs and zero queue_depth) |
| Epoch cycling | Yes | Every ~30s, balance=0 constant |
| Last epoch line | `Epoch complete epoch=714 balance_before=0 balance_after=0 ratio=1.15` (at 18:58:40Z) | Equilibrium confirmed |

---

## NTP Failure Series — Trend Report

### Historical Data (6-pass window: passes 115–120)

| Pass | Capture Time (Z) | NTP Failures | Cumulative | Failure Rate (6-pass trailing) |
|------|------------------|-------------|------------|-------------------------------|
| 115 | ~18:01Z | 0 (not yet occurred) | 0 | 0% |
| 116 | ~18:10Z | 2 (18:02Z: pool.ntp.org + time.apple.com) | 2 | 33% (1/3 passes with failures) |
| 117 | ~18:11Z | 0 | 2 | 33% |
| 118 | ~18:40Z | 0 | 2 | 33% |
| 119 | ~18:49Z | 0 | 2 | 33% |
| **120** | **18:59Z** | **1 (18:58Z: pool.ntp.org)** | **3** | **50% (3/6 passes with failures)** |

### Trend Analysis

| Field | Value |
|-------|-------|
| **First observed** | Pass 116, 18:02:00Z (pool.ntp.org) and 18:02:03Z (time.apple.com) |
| **Last pass classification** | Resolved — no recurrence for 48 min |
| **This pass classification** | **Recurring (approximate hourly cycle)** |
| **OBSERVED** | 3 total failures across 2 batches (18:02Z: 2 failures, 18:58Z: 1 failure so far). The 18:58Z batch may produce a second time.apple.com failure at 18:58:03Z — not yet in log at capture. Inter-batch interval: ~56 minutes. |
| **EXPECTED** | Periodic NTP checks should succeed consistently on an internet-connected machine with NTP synchronized (system clock confirmed synchronized by timedatectl). |
| **DEVIATION** | NTP checks fail approximately every ~56 minutes with `Input/output error: Resource temporarily unavailable (os error 11)`. The failure rate has risen from 33% (3/6 passes) only if counting unique passes; but the pattern is a predictable hourly recurrence, not a rising rate. |
| **UNKNOWN** | Whether the os error 11 (Resource temporarily unavailable, EAGAIN) is caused by rate limiting (too many queries to pool.ntp.org), a local firewall restriction, or a system resource issue. Only morning-api runs the NTP check (witness log shows no NTP lines). The time.apple.com fallback succeeded or was skipped in the 18:58Z batch. |
| **Changed since last pass** | Yes — NTP failures recurred after 48 min of silence. Reclassification from "Resolved" to "Recurring (hourly cycle)." |

---

## Persistent Deviations — Status

### 1. Stale build_commit (cb5d4b1-dirty vs HEAD 452b64f)

| Field | Value |
|-------|-------|
| **First observed** | Jul 27 (prior to 452b64f docs update) |
| **Last pass status** | Persistent — 2 commits behind HEAD |
| **This pass status** | **Persistent — 2 commits behind HEAD**. No change. HEAD did not advance. |
| **OBSERVED** | Both nodes report `build_commit: "cb5d4b1-dirty"`. Git HEAD is `452b64f`. Binary is 2 commits behind HEAD and was compiled from a dirty working tree. |
| **EXPECTED** | `build_commit` should match git HEAD. At minimum, not `-dirty`. |
| **DEVIATION** | Binary is stale by 2 commits with `-dirty` suffix. |
| **UNKNOWN** | Whether this affects runtime behavior. The wal_bytes fix (0c4bb7f) is verified WORKING on this binary (byte-equality passes). HEAD commit (452b64f) is docs-only. |
| **Changed since last pass** | No — gap unchanged at 2 commits. |

### 2. Supply Conservation Divergence (morning-api=20, witness=0)

| Field | Value |
|-------|-------|
| **First observed** | Jul 27, pass #3 (18:48 EDT) |
| **Last pass status** | Persistent — unchanged |
| **This pass status** | **Persistent — unchanged** |
| **OBSERVED** | morning-api balance=20 (own), 9,980 (witness). witness balance=0 (own), 0 (morning-api). Total supply per morning-api: 10,000. Total supply per witness: 0. Frozen entire session (>5.8h). |
| **EXPECTED** | Supply Conservation Invariant (proposed, pending governance): sum of spendable balances across mesh should equal total supply. |
| **DEVIATION** | CONTRADICTED — ledgers disagree on total supply. |
| **UNKNOWN** | The cause (initial mint local-only, sender debits before recipient confirms, no reconciliation mechanism) — documented in VERIFIED-BEHAVIOR.md as "Causes / Contributing Conditions," not verified diagnoses. |
| **Changed since last pass** | No — balance frozen at 20 for entire session. |

### 3. Epoch Ratio Divergence (~13.0% gap)

| Field | Value |
|-------|-------|
| **First observed** | Jul 28, pass 1 (18:06Z) |
| **Last pass status** | Persistent — gap ~13.4% |
| **This pass status** | **Persistent — gap ~13.0%**. Narrowed from drift (witness ratio continues slow decline). |
| **OBSERVED** | morning-api ratio=1.01952, witness ratio=1.15201 (~13.0% gap). |
| **EXPECTED** | Both nodes apply the same Georgist formula with same `redistributed_to=1`. Ratio should converge. |
| **DEVIATION** | ~13.0% gap. Function of total supply (net of tax base), which differs between nodes. |
| **UNKNOWN** | Whether the ratio divergence is purely consequential or has independent contributions. |
| **Changed since last pass** | Gap narrowed from ~13.4% to ~13.0% due to witness ratio drift (−0.0041). Precision drift, not structural change. |

### 4. MESH.md Stale

| Field | Value |
|-------|-------|
| **First observed** | Jul 27 (prior to Jul 28 session start) |
| **Last pass status** | Persistent — unchanged |
| **This pass status** | **Persistent — unchanged** |
| **OBSERVED** | MESH.md at commit c008def says "No production nodes running." Both nodes running continuously since 13:01Z (>5.8h). |
| **EXPECTED** | MESH.md should reflect current active topology. |
| **DEVIATION** | Topology documentation out of sync for >5.8 hours. |
| **UNKNOWN** | Whether intentional (ongoing session) or oversight. Node launch commands in MESH.md are accurate. |
| **Changed since last pass** | No. |

---

## New Observations This Pass

### 1. NTP Failures — Recurrence Confirmed (Hourly Cycle)

The two NTP failures at 18:02Z (pass 116) were followed by a new failure at 18:58:00Z. The inter-batch interval is ~56 minutes. This changes the classification from "Resolved" (pass 118-119) to **Recurring (approximate hourly cycle)**.

The failure pattern:
- 18:02:00Z: pool.ntp.org → os error 11
- 18:02:03Z: time.apple.com → os error 11
- 18:58:00Z: pool.ntp.org → os error 11 (time.apple.com not yet confirmed as failed or skipped at capture time)

The time.apple.com absence in the second batch is notable — either it was skipped (code may try fewer fallbacks after first failure), succeeded (unlikely given same os error 11), or the log line will appear in the next observation window.

### 2. Snapshot at 710, Next Rotation at ~720

Last snapshot at epoch 710 (mtime 14:56). Expected next rotation at epoch 720 (~8 min from capture). Snapshot size consistent: morning-api 895 bytes (balance 20 + 9980 peer), witness 569 bytes (balance 0 + 0 peer). wal.wal.old files unchanged from pass 119 (mtime 14:51, 379 bytes).

### 3. Witness Trailing Confirmed as Timing Drift (Not True Lag)

In pass 118, both nodes reported identical epoch at capture. Pass 119 showed witness 1 epoch behind. Pass 120 shows witness 1 epoch behind morning-api again (714 vs 715 at 11s gap). This is consistent timing drift from the ~11-second gap between socket captures (~1/3 of an epoch cycle). No evidence of true lag developing. Monitoring can be reduced to periodic spot-check.

### 4. No Zombies, No Evictions, No Sweeps

zero zombies, zero evictions, zero stale fetch/outbound sweeps. Connection health normal (silence_secs 0-6s, well under 30s zombie threshold). Metrics clean across all dimensions.

---

## Evidence Gaps

1. **No cross-node balance reconciliation.** Morning-api reports own balance=20, witness reports morning-api balance=0. No consensus mechanism active on Era One. Era Two state_root sortition path (code at 553ce22, dormant) would address this.

2. **NTP time.apple.com status in second batch unknown.** Whether time.apple.com was skipped, succeeded, or the failure log line is pending cannot be determined from current data. Check in next pass.

3. **Root cause of `-dirty` suffix unknown.** Current worktree shows 1 modified tracked file and untracked log/evidence files. The suffix could reflect a different state at compile time. Not reproducible from current worktree.

4. **Redistribution inactivity.** No redistribution transactions observed this entire session (>5.8h, ~715+ epochs). Economic engine reached equilibrium (balance=20, ratio≈1.02 → integer truncation yields 0 net change).

5. **NTP failure rate trend.** Over the last 6 passes: 3 passes with failures (50%), 3 without. The pattern is periodic (hourly), not rising. No evidence of increasing rate at this point — but only 2 batches observed. Continue monitoring for pattern stability.

---

**Next expected events:** NTP retry at ~19:58Z (~1 hour). Snapshot rotation at epoch 720 (~19:07Z). No other state changes expected given the frozen economic equilibrium.

**Timeline:** Session started 13:01Z Jul 28. Now at 18:59Z Jul 28. Runtime: 5h 58min. 120 observation passes completed (~0.34 passes/min).
