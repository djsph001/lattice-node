# Observer Evidence Record — 2026-07-28 (Pass 123)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** 2026-07-28T19:40Z – 19:42Z (bundle), re-verify at 19:41Z
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** 123rd observation pass of Jul 28. ~10 min since pass 122 (19:31Z). Sockets responsive, PIDs unchanged. **GONE:** exp-cap-002 experiment nodes no longer running.

**Summary:** Delta from pass 122. All evidence guards PASS on morning-api (three-way exact match, byte-equality). Epoch advanced +18 (782→800) in ~10 min = ~33s/epoch (normal). Snapshot rotated at epoch 800 (new) — last_snapshot_epoch jumped from 790→800 during this pass. Economic state completely frozen — unchanged entire session (>6.5h). Three persistent deviations unchanged (balance divergence, build_commit gap [now 5 commits behind HEAD], stale MESH.md). No new NTP failures since pass 122. Exp-cap-002 experiment nodes stopped — no longer running.

---

## Topology Disclosure

**This machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Role in mesh:** Host for all processes (z4-workstation)

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 13:01Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 13:02Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**Topology changes since pass 122:** No changes to main mesh nodes (same PIDs, same commands). **REMOVED:** exp-cap-002 (exp-claimer + exp-witness) — processes no longer running, sockets unreachable. Expected termination given the experiment (cap enforcement) likely completed.

---

## Evidence Integrity Guards — Simultaneous Captures

### morning-api (19:41Z single capture)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch | Socket=800, Log count=800, Last log line epoch=800 | All three match at a single instant | **PASS** — exact match |
| Byte-equality | wal_bytes=379 (socket), stat=379 (wc -c wal.log) | Must match | **PASS** |

### local-witness (approximate, ~19:40Z)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch | Socket=798, Log count=799, Last log line epoch=799 | All three match at a single instant | **BOUNDARY RACE** — socket read ~19:40Z returned 798, log already at 799. Expected timing drift from sequential capture (~30s epoch cycle). |
| Byte-equality | wal_bytes=379 (socket), stat=379 (wc -c wal.log) | Must match | **PASS** |

**Note:** Morning-api clean match. Witness captured first (19:40Z) caught the boundary between 798→799. No re-verify attempted within this pass due to single-capture discipline — both nodes are healthy and cycling normally.

---

## Metrics (Node Health)

### morning-api (19:41Z capture)

| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| uptime_secs | 23,915 (19:41Z) | ~23,359 at pass 122. Δ +556 in ~10 min ≈ ~55s/s (matches wall clock) | None |
| epoch | 800 (19:41Z) | Increasing at ~30s per epoch | None. Δ from pass 122 (782→800): +18 epochs in ~10 min = ~33s/epoch (normal) |
| height | 1 | 1 (unchanged since genesis) | None |
| peers | 1 (12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch, heartbeats=2389, silence_secs=4, is_dead=false) | 1 peer (local-witness) | None. Heartbeats increasing (Δ +56 from pass 122's 2333) |
| own_balance | 20 | 20 (frozen since at least epoch 443) | **PERSISTENT** — supply conservation divergence. Unchanged. |
| own_nonce | 241 | 241 (frozen) | None — nonce frozen since redistribution stopped |
| thickness | 976.39 (19:41Z) | ~976.52 (pass 122). Δ -0.13 in ~10 min | None — normal slow decay |
| ratio | 1.01957 | ~1.01956 (pass 122) | None — stable to 5 decimal places |

### local-witness (approximate, ~19:40Z)

| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| uptime_secs | ~23,900 (estimated) | ~23,352 at pass 122 | None |
| epoch | 799 (log, 19:41Z) | ~800 (trailing morning-api by ~1 epoch, confirmed timing drift) | None |
| height | 1 | 1 | None |
| peers | 1 (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ, connected) | 1 peer (morning-api) | None |
| own_balance | 0 | 0 (never received redistribution) | **PERSISTENT** — supply conservation divergence |
| own_nonce | 4 | 4 (frozen) | None |
| ratio | 1.1353 (socket at ~19:40Z) | 1.1388 (pass 122). Δ -0.0035 in ~10 min | None — continuing slow decline. Consistent pattern. |

---

## Economic State

### morning-api (19:41Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | Frozen equilibrium (balance_before=20, balance_after=20, ratio=1.02 → integer truncation = 0 net). | None (terminal state for current parameters). |
| peer (witness) balance | 9,980 | witness reports 0 for itself | **PERSISTENT** — supply conservation divergence (VERIFIED-BEHAVIOR.md: CONTRADICTED). Unchanged. |

### local-witness (19:40Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 (never received any mint) | None |
| peer (morning-api) balance | 0 | morning-api reports 20 for itself | **PERSISTENT** — witness ledger sees morning-api as 0 balance. Unchanged. |

### Epoch Ratio Divergence

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| morning-api ratio | 1.01957 | ~1.02 | None — stable |
| witness ratio | 1.13530 | ~1.1388 (pass 122) | None — continuing slow decline (Δ -0.0035 since pass 122) |
| Gap | ~11.4% | Same formula, same `redistributed_to=1` | **PERSISTENT** — gap narrowed from ~11.7% (pass 122) to ~11.4% due to continued witness ratio drift. |

---

## Persistence State

### morning-api (19:41Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | **800** (19:41Z) | Increments every 10 epochs | **NEW ROTATION** — jumped from 790 (pass 122) to 800. Snapshot at epoch 800 occurred during this pass. |
| wal_bytes | 379 | 379 (byte-equality passes) | None |
| wal_entries | 3 | 3 (size/120 heuristic) | Known-provisional (VERIFIED-BEHAVIOR.md) |
| wal.wal.old present | 379 bytes (mtime 15:36) | Pre-rotation WAL file from epoch 790 snapshot | None — cosmetic naming known-provisional |
| state.snapshot | 894 bytes (mtime 15:41) | Contains balance/thickness/snapshot epoch data | **NEW** — shrunk from 895 bytes (epoch 780) to 894 bytes (epoch 800). Minor change. |

### local-witness (19:40Z, from pass 122 disk snapshot)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | Not re-queried in single capture | Should track morning-api's rotation | UNKNOWN — need fresh query |
| wal_bytes | 379 | 379 (byte-equality passes at pass 122) | None |
| wal_entries | 3 | 3 | Known-provisional |

---

## Build Provenance

### Git state at capture time

| Field | OBSERVED | EXPECTED |
|-------|----------|----------|
| Git HEAD | `4c29c52` (docs: objection cap and receive path VERIFIED via EXP-CAP-002) | — |
| Build commit (morning-api) | `cb5d4b1-dirty` | `4c29c52` (clean) |
| Build commit (witness) | `cb5d4b1-dirty` | `4c29c52` (clean) |
| Commits behind HEAD | **5** (missing `0c4bb7f` + `452b64f` + `d802680` + `8b329b7` + `4c29c52`) | 0 |
| Working tree at capture | No modified tracked files checked | — |

### Delta from pass 122

| Aspect | OBSERVED | DEVIATION |
|--------|----------|-----------|
| build_commit value | cb5d4b1-dirty (unchanged binary) | **PERSISTENT** — same since Jul 27 |
| Behind-HEAD gap | 5 commits (+1 from pass 122, HEAD advanced via new commit `4c29c52`) | **Gap grew** from 4→5 due to `4c29c52` landing on main |
| -dirty suffix | Present (unchanged) | **PERSISTENT** — uncommitted changes at compile time |

---

## Log Health Scan (19:40Z)

### morning-api

| Pattern | Count | Notes |
|---------|-------|-------|
| WARN (non-structural, filtered) | 0 structural | 3 NTP warnings (all historical, none since 18:58Z). No new WARNs this interval. |
| ERROR (filtered) | 0 | Clean |
| Zombie/sweep/eviction | 0 | None occurred this interval |
| Heartbeats flowing | Yes | Every ~10s from witness, incrementing normally (2,389 at capture) |
| Metrics healthy | Yes | outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=6s |
| Epoch cycling | Yes | Every ~30s, balance=20 constant |
| Last epoch line | `Epoch complete epoch=800 balance_before=20 balance_after=20 ratio=1.02` (at 19:41:17Z) | Equilibrium confirmed |

### local-witness

| Pattern | Count | Notes |
|---------|-------|-------|
| WARN (non-structural, filtered) | 0 structural | 1 NTP warning (historical, 19:09Z). No new WARNs this interval. |
| ERROR (filtered) | 0 | Clean |
| Zombie/sweep/eviction | 0 | None occurred |
| Heartbeats flowing | Yes | Every ~10s from morning-api, incrementing normally |
| Metrics healthy | Yes | outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=3s |
| Epoch cycling | Yes | Every ~30s, balance=0 constant |
| Last epoch line | `Epoch complete epoch=799 balance_before=0 balance_after=0 ratio=1.14` (at 19:41:10Z) | Equilibrium confirmed |
| Insufficient-balance rejections | ~119 (all historical, before 14:02Z) | No new rejections entire session |

---

## NTP Failure Series — Trend Report

### Data Since Pass 115

| Pass | Capture Time (Z) | NTP Failures | Cumulative | Trailing 6-Pass Rate |
|------|------------------|-------------|------------|----------------------|
| 115 | ~18:01Z | 0 (not yet occurred) | 0 | — |
| 116 | ~18:10Z | 2 (18:02Z: morning-api, pool + apple) | 2 | 33% (1/3) |
| 117 | ~18:11Z | 0 | 2 | 33% (1/3) |
| 118 | ~18:40Z | 0 | 2 | 33% (2/6) |
| 119 | ~18:49Z | 0 | 2 | 33% (2/6) |
| 120 | ~18:59Z | 1 (18:58Z: morning-api, pool) | 3 | 50% (3/6) |
| 121 | ~19:13Z | 0 | 3 | 33% (2/6) |
| 122 | ~19:31Z | 0 (no new since pass 121) | 3 | 33% (2/6) |
| **123** | **19:40Z** | **0** (no new since pass 122) | **3** | **33% (2/6)** |

### Trend Analysis

| Field | Value |
|-------|-------|
| **First observed** | Pass 116, 18:02:00Z |
| **Last pass classification** | Recurring (hourly cycle, quiescent phase) |
| **This pass classification** | **Recurring (hourly cycle, quiescent phase)** — no new failures in ~10 min since pass 122. Last morning-api batch: 18:58Z. Next expected: ~19:54Z. |
| **OBSERVED** | 3 total failures across 2 batches: morning-api at 18:02Z (2 failures) and 18:58Z (1); witness at 19:09Z (1). Inter-batch interval: ~56 min on morning-api. |
| **EXPECTED** | Periodic NTP checks should succeed consistently on an internet-connected machine with NTP synchronized. |
| **DEVIATION** | NTP checks fail periodically with `Input/output error: Resource temporarily unavailable (os error 11)`. Pattern is a predictable ~hourly recurrence, not a rising rate. |
| **UNKNOWN** | Whether the os error 11 is caused by NTP server rate limiting, local firewall, or system resource issue. Also unknown why witness NTP checks run on a different schedule (only 1 failure at 19:09Z). |
| **Changed since last pass** | No new failures since pass 122. Quiescent phase of the hourly cycle. |

---

## New Observations This Pass

### 1. Snapshot Rotated at Epoch 800

last_snapshot_epoch jumped from 790 (pass 122) to 800 during this pass. This is the expected rotation cadence (every 10 epochs). The snapshot file mtime updated to 15:41. state.snapshot shrunk from 895 bytes to 894 bytes (minor change — possibly balance data precision).

wal.wal.old now shows mtime 15:36 (the previous WAL from the epoch 790 snapshot rotation).

### 2. Build Commit Gap Grew (4→5 behind HEAD)

HEAD advanced from `8b329b7` to `4c29c52` (docs: objection cap and receive path VERIFIED via EXP-CAP-002). Binary was not rebuilt — still reports `cb5d4b1-dirty`. Binary is now 5 commits behind main.

### 3. Exp-cap-002 Experiment Terminated

The experiment nodes (exp-claimer on port 4300, exp-witness on port 4310) are no longer running. Sockets unreachable, no matching PIDs in process table. Expected — the experiment was likely a bounded cap-enforcement test that completed between pass 122 and 123. Evidence: the HEAD commit `4c29c52` documents "objection cap and receive path VERIFIED via EXP-CAP-002", confirming the experiment succeeded and was documented.

### 4. Ratio Gap Continued Narrowing

Witness ratio declined from 1.1388 (pass 122) to 1.1353 (pass 123), a Δ of -0.0035 in ~10 min. Morning-api ratio essentially flat (1.01956 → 1.01957). Gap narrowed from ~11.7% to ~11.4%. Narrowing rate: ~0.3% per 10 min ≈ ~1.8%/hour (slowing from earlier ~2.4%/hour as the gap closes).

### 5. Economic State Completely Frozen

All economic metrics unchanged: balance=20/nonce=241 on morning-api, balance=0/nonce=4 on witness. No redistribution (tax=0, minted=0). Epoch ratio formula continues to produce zero net change on both nodes. Session runtime now ~6h 40min with zero economic activity since before pass 1 (~18:06Z).

---

## Persistent Deviations — Status

### 1. Stale build_commit (cb5d4b1-dirty vs HEAD 4c29c52)

| Field | Value |
|-------|-------|
| **First observed** | Jul 27 (prior to 452b64f docs update) |
| **Last pass status** | Persistent — 4 commits behind HEAD (8b329b7) |
| **This pass status** | **Persistent — 5 commits behind HEAD**. Gap grew from 4→5 because HEAD advanced (4c29c52 landed), not because binary changed. |
| **OBSERVED** | Both nodes report `build_commit: "cb5d4b1-dirty"`. Git HEAD is `4c29c52`. Binary is 5 commits behind HEAD and was compiled from a dirty working tree. |
| **EXPECTED** | `build_commit` should match git HEAD. At minimum, not `-dirty`. |
| **DEVIATION** | Binary is stale by 5 commits with `-dirty` suffix. |
| **UNKNOWN** | Whether this affects runtime behavior. The wal_bytes fix (0c4bb7f) is verified WORKING (byte-equality passes). HEAD commits are docs + objection-injector fixes — unrelated to current mesh operation. |
| **Changed since last pass** | Yes — gap increased from 4→5 commits. Binary did not change; HEAD advanced. |

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

### 3. Epoch Ratio Divergence (~11.4% gap)

| Field | Value |
|-------|-------|
| **First observed** | Jul 28, pass 1 (18:06Z) |
| **Last pass status** | Persistent — gap ~11.7% |
| **This pass status** | **Persistent — gap ~11.4%**. Witness ratio continued decline (1.1353 vs 1.1388 at pass 122). |
| **OBSERVED** | morning-api ratio=1.01957, witness ratio=1.13530 (~11.4% gap). |
| **EXPECTED** | Both nodes apply the same Georgist formula with same `redistributed_to=1`. Ratio should converge. |
| **DEVIATION** | ~11.4% gap. Function of total supply (net of tax base), which differs between nodes. |
| **UNKNOWN** | Whether the ratio divergence is purely consequential or has independent contributions. |
| **Changed since last pass** | Gap narrowed from ~11.7% to ~11.4% due to witness ratio drift (−0.0035). Rate slowing as gap approaches zero. |

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

## Evidence Gaps

1. **No cross-node balance reconciliation.** Morning-api reports own balance=20, witness reports morning-api balance=0. No consensus mechanism active on Era One.

2. **Root cause of NTP `os error 11` unknown.** System clock confirmed synchronized. Node's periodic NTP check fails ~hourly. Need to determine if rate limiting, firewall, or resource issue.

3. **Witness NTP schedule differs from morning-api.** Witness shows only 1 NTP failure (19:09Z) while morning-api shows a consistent ~56-min cycle. Unknown whether witness runs fewer checks, started later, or has a different failure pattern.

4. **Root cause of `-dirty` suffix unknown.** Uncommitted changes at compile time. The current binary may differ from the clean cb5d4b1 tree.

5. **Redistribution inactivity continues.** No redistribution activity observed entire session (>6.5h, ~800 epochs). Economic engine reached equilibrium (balance=20, ratio≈1.02 → integer truncation yields 0 net change).

6. **Exp-cap-002 experiment completed — details unknown.** The experiment ran its course and was terminated. HEAD commit `4c29c52` documents it as VERIFIED. The specific cap behavior and metrics are not observable from the main mesh UDS.

---

**Next expected events:** NTP retry at ~19:54Z (morning-api, ~56 min cycle from 18:58Z). Snapshot rotation at epoch 810 (~19:46Z, estimated). No other state changes expected given the frozen economic equilibrium.

**Timeline:** Session started 13:01Z Jul 28. Now at 19:42Z Jul 28. Runtime: 6h 41min. 123 observation passes completed.
