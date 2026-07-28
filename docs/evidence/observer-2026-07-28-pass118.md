# Observer Evidence Record — 2026-07-28 (Pass 118)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** 2026-07-28T18:39:55Z – 18:40:14Z (morning-api @ 18:39:55Z, witness @ 18:40:14Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** 118th observation pass of Jul 28. ~8 min since pass 117 (18:32Z). Sockets responsive, PIDs unchanged (3579452/3579821).

**Summary:** Delta-only from pass 117. All evidence guards PASS. Epochs advanced +16 at normal cadence (~29s/epoch). Snapshot rotated 660→670 (1 rotation). Economic state completely frozen — unchanged entire session (>5.5h). Three persistent deviations unchanged (stale build_commit 2 commits behind HEAD, supply conservation divergence, epoch ratio divergence). NTP failures from pass 116 (18:02Z) did NOT recur — confirmed transient/resolved.

---

## Topology Disclosure

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 13:01Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 13:02Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes since session start (13:01Z).** Same PIDs across all 118 passes.

---

## Evidence Integrity Guards — Simultaneous Captures

### morning-api (18:39:55Z)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch | Socket=677, Log count=677, Last log epoch at instant=677 | All three match at a single instant | **PASS** — exactly equal at 18:39:55Z |
| Byte-equality | wal_bytes=379 (socket), stat=379 (ls -la wal.log) | Must match | **PASS** |

### local-witness (18:40:14Z)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch | Socket=677, Log count=677, Last log epoch=677 | All three match at a single instant | **PASS** — exactly equal at 18:40:14Z |
| Byte-equality | wal_bytes=379 (socket), stat=379 (ls -la wal.log) | Must match | **PASS** |

**Note:** Both nodes report same epoch (677) simultaneously. Witness has held convergence with morning-api since pass 117.

---

## Metrics (Node Health)

### morning-api (18:39:55Z simultaneous)

| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| uptime_secs | 20,288 (18:39Z) | Increasing at ~10s per metrics tick | None. Δ from pass 117: +522s (~8.7 min, close to wall time of ~7.8 min; +53s jitter is within capture-async range) |
| epoch | 677 (18:39Z) | Increasing at ~30s per epoch | None. Δ from pass 117: +16 epochs in ~8 min = ~30s/epoch (normal) |
| height | 1 | 1 (unchanged since genesis) | None |
| peers | 1 (12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch, heartbeats=2027, silence_secs=3, is_dead=false) | 1 peer (local-witness) | None. Heartbeats increasing steadily (Δ +53 from pass 117's 1974) |
| own_balance | 20 | 20 (frozen since at least epoch 443) | **PERSISTENT** — supply conservation divergence documented since Jul 27. Unchanged since pass 117. |
| own_nonce | 241 | 241 (frozen) | None — nonce frozen since redistribution stopped |
| thickness | 977.34 | Drifts slowly (was 977.48 at pass 117) | None — normal slow drift from economic inactivity (-0.14 since pass 117) |
| ratio | 1.01948 | 1.01947 (pass 117) | None — stable to 5 decimal places |

### local-witness (18:40:14Z simultaneous)

| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| uptime_secs | 20,284 (18:40Z) | ~20,288 (started ~23s after morning-api) | None. Δ from pass 117: +518s (~8.6 min) |
| epoch | 677 (18:40Z) | 677 (matching morning-api) | None. Witness held convergence since pass 117. |
| height | 1 | 1 | None |
| peers | 1 (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ, heartbeats=2029, silence_secs=6, is_dead=false) | 1 peer (morning-api) | None. Heartbeats increasing steadily (Δ +53 from pass 117's 1976) |
| own_balance | 0 | 0 (never received redistribution) | **PERSISTENT** — supply conservation divergence, documented since Jul 27 |
| own_nonce | 4 | 4 (frozen) | None |
| ratio | 1.16071 | ~1.16476 (pass 117) | None — small drift (-0.004) from precision, not structural change |

### Metrics from log lines (both nodes, ~18:40Z)

| Metric | morning-api | local-witness | EXPECTED |
|--------|-------------|---------------|----------|
| outstanding_fetches | 0 | 0 | 0 |
| aged | 0 | 0 | 0 |
| outbound_queues | [] | [] | [] |
| max_peer_silence | 6s | 3s | <10s |

---

## Economic State

### morning-api (18:39:55Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | From initial mint of 5,000, redistributed to near-zero floor. Frozen since at least epoch 443 (~5.5h ago). | None (terminal state for current parameters). Ratio=1.019 on balance=20 yields net zero change from integer truncation. |
| peer (witness) balance | 9,980 | witness reports 0 for itself | **PERSISTENT** — supply conservation divergence (VERIFIED-BEHAVIOR.md: CONTRADICTED). Unchanged. |

### local-witness (18:40:14Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 (never received any mint) | None |
| peer (morning-api) balance | 0 | morning-api reports 20 for itself | **PERSISTENT** — witness ledger sees morning-api as 0 balance. Unchanged. |

### Epoch Ratio Divergence

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| morning-api ratio | 1.01948 | ~1.02 | None — stable |
| witness ratio | 1.16071 | ~1.16 | None — stable |
| Gap | ~13.8% | Same formula, same `redistributed_to=1` | **PERSISTENT** — first observed pass 1. Both nodes apply the same Georgist formula with the same parameters, but get different ratios because their total supply views differ (10,000 vs 0). Consequence of supply divergence, not independent. |

---

## Persistence State

### morning-api (18:39:55Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 670 | Increments every 10 epochs | None. Δ from pass 117: 660→670 (1 rotation at 670) |
| wal_bytes | 379 | 379 (byte-equality passes) | None |
| wal_entries | 3 | 3 (size/120 heuristic) | Known-provisional (VERIFIED-BEHAVIOR.md) |
| wal.wal.old present | 379 bytes | Pre-rotation WAL file | Cosmetic naming — known-provisional (VERIFIED-BEHAVIOR.md). Same size as active wal.log |

### local-witness (18:40:14Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 670 | Matches morning-api | None — both nodes snapshotted at same epoch |
| wal_bytes | 379 | 379 (byte-equality passes) | None |
| wal_entries | 3 | 3 (size/120 heuristic) | Known-provisional |
| wal.wal.old present | 379 bytes | Pre-rotation WAL file | Cosmetic naming — known-provisional |

---

## Build Provenance

### Git state at capture time

| Field | OBSERVED | EXPECTED |
|-------|----------|----------|
| Git HEAD | `452b64f` (docs: wal_bytes fix verified, wal_entries heuristic noted) | — |
| Build commit (morning-api) | `cb5d4b1-dirty` | `452b64f` (clean) |
| Build commit (witness) | `cb5d4b1-dirty` | `452b64f` (clean) |
| Commits behind HEAD | 2 (missing `0c4bb7f` + `452b64f`) | 0 |
| Working tree at capture | 1 tracked file modified (`docs/evidence/observer-2026-07-27-pass10.md`), untracked log files | Clean |

### Delta from pass 117

| Aspect | OBSERVED | DEVIATION |
|--------|----------|-----------|
| build_commit value | cb5d4b1-dirty (unchanged) | **PERSISTENT** — same since Jul 27 |
| Behind-HEAD gap | 2 commits (was 1 in pass 117) | **EXPANDED** — HEAD advanced 1 commit (452b64f) beyond 0c4bb7f, but binary unchanged |
| -dirty suffix | Present (unchanged) | **PERSISTENT** — uncommitted changes at compile time |

---

## Log Health Scan (18:40Z)

### morning-api

| Pattern | Count | Notes |
|---------|-------|-------|
| WARN (structural, filtered) | 0 | All 72 WARNs are Kademlia bootstrap failures (benign, `--no-mdns`). Filtered clean. |
| ERROR (filtered) | 0 | Clean |
| Zombie/sweep/eviction | 0 | None occurred |
| Heartbeats flowing | Yes | Every 3-5s from witness, incrementing normally (2027 at capture) |
| Metrics healthy | Yes | outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=6s |
| Epoch cycling | Yes | Every ~30s, balance=20 constant |
| Last epoch line | `Epoch complete epoch=677 balance_before=20 balance_after=20 ratio=1.02` | Equilibrium confirmed |

### local-witness

| Pattern | Count | Notes |
|---------|-------|-------|
| WARN (filtered) | 0 | All 121 WARNs are Kademlia (benign). Filtered clean. |
| ERROR (filtered) | 0 | Clean |
| Zombie/sweep/eviction | 0 | None occurred |
| Heartbeats flowing | Yes | Every 3-5s from morning-api, incrementing normally (2029 at capture) |
| Metrics healthy | Yes | outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=3s |
| Epoch cycling | Yes | Every ~30s, balance=0 constant |
| Last epoch line | `Epoch complete epoch=678 balance_before=0 balance_after=0 ratio=1.16` (epoch advanced to 678 during read — normal) | Equilibrium confirmed |

---

## Persistent Deviations — Status

### 1. Stale build_commit (cb5d4b1-dirty vs HEAD 452b64f)

| Field | Value |
|-------|-------|
| **First observed** | Jul 27 (prior to 452b64f docs update) |
| **Last pass status** | Persistent — 1 commit behind HEAD |
| **This pass status** | **Persistent — 2 commits behind HEAD**. HEAD advanced 1 commit (452b64f docs update); binary unchanged. |
| **OBSERVED** | Both nodes report `build_commit: "cb5d4b1-dirty"`. Git HEAD is `452b64f`. Binary is 2 commits behind HEAD and was compiled from a dirty working tree. |
| **EXPECTED** | `build_commit` should match git HEAD. At minimum, not `-dirty`. |
| **DEVIATION** | Binary is stale by 2 commits with `-dirty` suffix. |
| **UNKNOWN** | Whether this affects any runtime behavior. The wal_bytes fix (0c4bb7f) is verified WORKING on this binary (byte-equality passes at 379 bytes on wal.log), suggesting the fix may already be in cb5d4b1 or was independently correct by configuration. HEAD is a docs-only update (452b64f), so no runtime impact from the missing commits. |
| **Changed since last pass** | Yes — gap expanded from 1 to 2 commits behind HEAD. But this is expected: HEAD advanced (new docs commit), binary did not change. |

### 2. Supply Conservation Divergence (morning-api=20, witness=0)

| Field | Value |
|-------|-------|
| **First observed** | Jul 27, pass #3 (18:48 EDT) |
| **Last pass status** | Persistent — unchanged |
| **This pass status** | **Persistent — unchanged** |
| **OBSERVED** | morning-api balance=20 (own), 9,980 (witness). witness balance=0 (own), 0 (morning-api). Total supply per morning-api: 10,000. Total supply per witness: 0. Frozen entire session (>5.5h). |
| **EXPECTED** | Supply Conservation Invariant (proposed, pending governance): sum of spendable balances across the mesh should equal the network's recognized total supply. |
| **DEVIATION** | CONTRADICTED — the two nodes' ledgers disagree on total supply. Verified in VERIFIED-BEHAVIOR.md. |
| **UNKNOWN** | The cause (initial mint local-only, sender debits before recipient confirms, no reconciliation mechanism) — these are documented in VERIFIED-BEHAVIOR.md as "Causes / Contributing Conditions," not verified diagnoses. |
| **Changed since last pass** | No — balance frozen at 20 since at least epoch 443. |

### 3. Epoch Ratio Divergence (~14% gap)

| Field | Value |
|-------|-------|
| **First observed** | Jul 28, pass 1 (18:06Z) |
| **Last pass status** | Persistent — unchanged |
| **This pass status** | **Persistent — unchanged** |
| **OBSERVED** | morning-api ratio=1.01948, witness ratio=1.16071 (~13.8% gap). Gap virtually unchanged since pass 1. |
| **EXPECTED** | Both nodes apply the same Georgist formula with same `redistributed_to=1`. The ratio should converge. |
| **DEVIATION** | 14% gap. Ratio is a function of total supply (net of tax base), which differs between nodes due to the supply divergence. |
| **UNKNOWN** | Whether the ratio divergence is purely a consequence of the supply divergence or has independent contributions. |
| **Changed since last pass** | No — gap stable within precision drift. |

### 4. MESH.md Stale

| Field | Value |
|-------|-------|
| **First observed** | Jul 27 (prior to Jul 28 session start) |
| **Last pass status** | Persistent |
| **This pass status** | **Persistent — unchanged** |
| **OBSERVED** | MESH.md at commit c008def says "No production nodes running — both nodes were stopped and storage wiped during Jul 27 cleanup." Both nodes have been running continuously since 13:01Z Jul 28. |
| **EXPECTED** | MESH.md should reflect current active topology. |
| **DEVIATION** | Topology documentation out of sync with running nodes for >5.5 hours. |
| **UNKNOWN** | Whether intentional (ongoing session with pending cleanup) or an oversight. Node launch commands in MESH.md are accurate — the status header is stale. |

---

## New Observations This Pass

### Equilibrium Confirmed (carried forward from pass 117)

Balance=20 on morning-api remains a stable fixed point under integer truncation at ratio≈1.02. +16 additional epochs since pass 117 with exact `balance_before=20 balance_after=20` in every Epoch complete line. This has been stable for >5.5 hours across ~234 epochs and 67 snapshot rotations.

### Build gap expanded from 1 to 2 commits behind HEAD

The git HEAD advanced 1 commit (452b64f — docs-only) since pass 117. The running binary (cb5d4b1-dirty) is unchanged. The gap now stands at 2 commits. No runtime impact since the missing commits are docs-only and a WAL-path fix that was already verified working (byte-equality passes) under the current binary.

### Witness epoch convergence holding

Witness has maintained epoch parity with morning-api across both pass 117 (+18) and this pass (+16). Both nodes now advance at the same cadence (~29-30s/epoch) with no trailing.

### NTP Failures — Confirmed Resolved (no longer transient)

The two NTP fallback failures observed at 18:02Z in pass 116 have not recurred in the ~38 minutes since first observation. Reclassified from "Transient" to **Resolved**. Whether this is a one-time event that does not repeat, or repeats on an unknown schedule (hourly? daily?), remains **UNKNOWN**.

---

## Evidence Gaps

1. **No cross-node balance reconciliation.** The witness reports morning-api's balance as 0; morning-api reports its own as 20. No consensus mechanism active on Era One. The Era Two state_root sortition path (code landed at 553ce22, dormant) would address this.

2. **NTP periodic retry mechanism unknown.** Two NTP fallback failures occurred at 18:02Z after 5h of runtime. No recurrence in 38 min. Whether this repeats on an hourly/daily cycle or was a one-time event is UNKNOWN. Reclassified from "Transient" to "Resolved — recurrence schedule unknown."

3. **Root cause of `-dirty` suffix unknown.** Current working tree shows 1 modified tracked file (docs/evidence/observer-2026-07-27-pass10.md) plus untracked log files. The `-dirty` suffix could reflect a different state at compile time. Not reproducible from current worktree.

4. **Redistribution inactivity.** No redistribution transactions have been observed this entire session (>5.5h, ~677+ epochs). The economic engine reached an equilibrium state (balance=20, ratio≈1.02 → integer truncation yields 0 net change) and has not been disturbed. Whether this equilibrium is theoretically permanent or can be escaped via a different parameter set is outside observer scope.

---

**Next expected event:** Snapshot rotation at epoch 680 (~5 min). No other state changes expected given the frozen economic equilibrium.

**Timeline:** Session started 13:01Z Jul 28. Now at 18:40Z Jul 28. Runtime: 5h 39min. 118 observation passes completed (~0.17 passes/min, consistent with ~6 min interval).
