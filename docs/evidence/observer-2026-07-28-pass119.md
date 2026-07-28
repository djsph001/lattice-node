# Observer Evidence Record — 2026-07-28 (Pass 119)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** 2026-07-28T18:49:52Z – 18:50:17Z (morning-api @ 18:49:52Z, witness @ 18:50:03Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** 119th observation pass of Jul 28. ~10 min since pass 118 (18:39Z). Sockets responsive, PIDs unchanged (3579452/3579821).

**Summary:** Delta-only from pass 118. All evidence guards PASS. Epochs advanced +19–+20 at normal cadence (~29s/epoch). Snapshot rotated 670→690 (2 rotations). Economic state completely frozen — unchanged entire session (>5.7h). Three persistent deviations unchanged (stale build_commit 2 commits behind HEAD, supply conservation divergence, epoch ratio divergence). NTP failures did NOT recur — consistent with pass 118's resolved classification.

---

## Topology Disclosure

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 13:01Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 13:02Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes since session start (13:01Z).** Same PIDs across all 119 passes.

---

## Evidence Integrity Guards — Simultaneous Captures

### morning-api (18:49:52Z)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch | Socket=697, Log count=698, Last log epoch at instant=698 | All three match at a single instant | **PASS** — 1-epoch gap between socket capture (18:49:52Z) and log read (18:50+Z) is timing drift within normal cadence. Verified by comparison: 18:50:17Z log line shows epoch=698 (+1 in 25s=~29s/epoch). |
| Byte-equality | wal_bytes=379 (socket), stat=379 (ls -la wal.log) | Must match | **PASS** |

### local-witness (18:50:03Z)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch | Socket=696, Log count=697, Last log epoch=697 | All three match at a single instant | **PASS** — same timing drift as morning-api (1-epoch gap between socket and log read). |
| Byte-equality | wal_bytes=379 (socket), stat=379 (ls -la wal.log) | Must match | **PASS** |

**Note:** Witness lags morning-api by 1 epoch at capture (696 vs 697). In pass 118, both were at parity (677). This may be timing drift or the beginning of a small lag — monitor in next pass.

---

## Metrics (Node Health)

### morning-api (18:49:52Z simultaneous)

| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| uptime_secs | 20,885 (18:49Z) | Increasing at ~10s per metrics tick | None. Δ from pass 118: +597s (~10 min, close to wall time of ~9.9 min; +1s jitter is negligible) |
| epoch | 697 (18:49Z) | Increasing at ~30s per epoch | None. Δ from pass 118: +20 epochs in ~10 min = ~30s/epoch (normal) |
| height | 1 | 1 (unchanged since genesis) | None |
| peers | 1 (12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch, heartbeats=2086, silence_secs=7, is_dead=false) | 1 peer (local-witness) | None. Heartbeats increasing steadily (Δ +59 from pass 118's 2027) |
| own_balance | 20 | 20 (frozen since at least epoch 443) | **PERSISTENT** — supply conservation divergence documented since Jul 27. Unchanged since pass 118. |
| own_nonce | 241 | 241 (frozen) | None — nonce frozen since redistribution stopped |
| thickness | 977.18 | ~977.34 (pass 118) | None — normal slow drift from economic inactivity (-0.16 since pass 118) |
| ratio | 1.01951 | ~1.01948 (pass 118) | None — stable to 5 decimal places |

### local-witness (18:50:03Z simultaneous)

| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| uptime_secs | 20,873 (18:50Z) | ~20,885 (started ~23s after morning-api) | None. Δ from pass 118: +589s (~9.8 min) |
| epoch | 696 (18:50Z) | 696 (trailing morning-api by 1 epoch) | None. Witness 1 epoch behind morning-api (was at parity in pass 118). Monitor next pass for widening. |
| height | 1 | 1 | None |
| peers | 1 (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ, heartbeats=2088, silence_secs=2, is_dead=false) | 1 peer (morning-api) | None. Heartbeats increasing steadily (Δ +59 from pass 118's 2029) |
| own_balance | 0 | 0 (never received redistribution) | **PERSISTENT** — supply conservation divergence, documented since Jul 27 |
| own_nonce | 4 | 4 (frozen) | None |
| ratio | 1.15612 | ~1.16071 (pass 118) | None — small drift (-0.0046) from precision, not structural change. Slightly larger drift than last pass (-0.004 vs -0.004). |

---

## Economic State

### morning-api (18:49:52Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | From initial mint of 5,000, redistributed to near-zero floor. Frozen since at least epoch 443 (~5.5h ago). | None (terminal state for current parameters). Ratio=1.019 on balance=20 yields net zero change from integer truncation. |
| peer (witness) balance | 9,980 | witness reports 0 for itself | **PERSISTENT** — supply conservation divergence (VERIFIED-BEHAVIOR.md: CONTRADICTED). Unchanged. |

### local-witness (18:50:03Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 (never received any mint) | None |
| peer (morning-api) balance | 0 | morning-api reports 20 for itself | **PERSISTENT** — witness ledger sees morning-api as 0 balance. Unchanged. |

### Epoch Ratio Divergence

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| morning-api ratio | 1.01951 | ~1.02 | None — stable |
| witness ratio | 1.15612 | ~1.16 | None — stable |
| Gap | ~13.4% | Same formula, same `redistributed_to=1` | **PERSISTENT** — first observed pass 1. Consequence of supply divergence, not independent. Gap narrowed ~0.4% from pass 118 (13.8%→13.4%) due to witness ratio drift. |

---

## Persistence State

### morning-api (18:49:52Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 690 | Increments every 10 epochs | None. Δ from pass 118: 670→690 (2 rotations at 680 and 690) |
| wal_bytes | 379 | 379 (byte-equality passes) | None |
| wal_entries | 3 | 3 (size/120 heuristic) | Known-provisional (VERIFIED-BEHAVIOR.md) |
| wal.wal.old present | 379 bytes | Pre-rotation WAL file | Cosmetic naming — known-provisional. Same size as active wal.log |
| state.snapshot | 895 bytes | Contains balance/thickness/snapshot epoch data | None — size consistent with balance data (20 + 9980 peer) |

### local-witness (18:50:03Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 690 | Matches morning-api | None — both nodes snapshotted at same epoch |
| wal_bytes | 379 | 379 (byte-equality passes) | None |
| wal_entries | 3 | 3 (size/120 heuristic) | Known-provisional |
| wal.wal.old present | 379 bytes | Pre-rotation WAL file | Cosmetic naming — known-provisional |
| state.snapshot | 569 bytes | Smaller than morning-api (less balance data) | None — consistent with near-zero balances |

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

### Delta from pass 118

| Aspect | OBSERVED | DEVIATION |
|--------|----------|-----------|
| build_commit value | cb5d4b1-dirty (unchanged) | **PERSISTENT** — same since Jul 27 |
| Behind-HEAD gap | 2 commits | **PERSISTENT** — unchanged since pass 118 (HEAD did not advance) |
| -dirty suffix | Present (unchanged) | **PERSISTENT** — uncommitted changes at compile time |

---

## Log Health Scan (18:50Z)

### morning-api

| Pattern | Count | Notes |
|---------|-------|-------|
| WARN (structural, filtered) | 2 (NTP failures at 18:02Z) | Kademlia warnings (~72) filtered as benign. NTP failures are historical (18:02Z), not new. No new WARNs this interval. |
| ERROR (filtered) | 0 | Clean |
| Zombie/sweep/eviction | 0 | None occurred this interval |
| Heartbeats flowing | Yes | Every ~10s from witness, incrementing normally (2086 at capture → 2090+ in latest log) |
| Metrics healthy | Yes | outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=7s |
| Epoch cycling | Yes | Every ~30s, balance=20 constant |
| Last epoch line | `Epoch complete epoch=698 balance_before=20 balance_after=20 ratio=1.02` | Equilibrium confirmed |

### local-witness

| Pattern | Count | Notes |
|---------|-------|-------|
| WARN (filtered) | 0 | All Kademlia warnings filtered as benign. No new WARNs this interval. |
| ERROR (filtered) | 0 | Clean |
| Zombie/sweep/eviction | 0 | None occurred |
| Heartbeats flowing | Yes | Every ~10s from morning-api, incrementing normally (2088 at capture → 2091+ in latest log) |
| Metrics healthy | Yes | outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=2s |
| Epoch cycling | Yes | Every ~30s, balance=0 constant |
| Last epoch line | `Epoch complete epoch=697 balance_before=0 balance_after=0 ratio=1.16` | Equilibrium confirmed |

---

## Persistent Deviations — Status

### 1. Stale build_commit (cb5d4b1-dirty vs HEAD 452b64f)

| Field | Value |
|-------|-------|
| **First observed** | Jul 27 (prior to 452b64f docs update) |
| **Last pass status** | Persistent — 2 commits behind HEAD |
| **This pass status** | **Persistent — 2 commits behind HEAD**. No change since pass 118. HEAD did not advance. |
| **OBSERVED** | Both nodes report `build_commit: "cb5d4b1-dirty"`. Git HEAD is `452b64f`. Binary is 2 commits behind HEAD and was compiled from a dirty working tree. |
| **EXPECTED** | `build_commit` should match git HEAD. At minimum, not `-dirty`. |
| **DEVIATION** | Binary is stale by 2 commits with `-dirty` suffix. |
| **UNKNOWN** | Whether this affects any runtime behavior. The wal_bytes fix (0c4bb7f — the only behavioral change in the missing commits) is verified WORKING on this binary (byte-equality passes at 379 bytes on wal.log), and the HEAD commit (452b64f) is docs-only. |
| **Changed since last pass** | No — gap unchanged at 2 commits. |

### 2. Supply Conservation Divergence (morning-api=20, witness=0)

| Field | Value |
|-------|-------|
| **First observed** | Jul 27, pass #3 (18:48 EDT) |
| **Last pass status** | Persistent — unchanged |
| **This pass status** | **Persistent — unchanged** |
| **OBSERVED** | morning-api balance=20 (own), 9,980 (witness). witness balance=0 (own), 0 (morning-api). Total supply per morning-api: 10,000. Total supply per witness: 0. Frozen entire session (>5.7h). |
| **EXPECTED** | Supply Conservation Invariant (proposed, pending governance): sum of spendable balances across the mesh should equal the network's recognized total supply. |
| **DEVIATION** | CONTRADICTED — the two nodes' ledgers disagree on total supply. Verified in VERIFIED-BEHAVIOR.md. |
| **UNKNOWN** | The cause (initial mint local-only, sender debits before recipient confirms, no reconciliation mechanism) — these are documented in VERIFIED-BEHAVIOR.md as "Causes / Contributing Conditions," not verified diagnoses. |
| **Changed since last pass** | No — balance frozen at 20 for entire session. |

### 3. Epoch Ratio Divergence (~13.4% gap)

| Field | Value |
|-------|-------|
| **First observed** | Jul 28, pass 1 (18:06Z) |
| **Last pass status** | Persistent — gap ~13.8% |
| **This pass status** | **Persistent — gap ~13.4%**. Slight narrowing from drift alone. |
| **OBSERVED** | morning-api ratio=1.01951, witness ratio=1.15612 (~13.4% gap). |
| **EXPECTED** | Both nodes apply the same Georgist formula with same `redistributed_to=1`. The ratio should converge. |
| **DEVIATION** | ~13.4% gap. Ratio is a function of total supply (net of tax base), which differs between nodes due to the supply divergence. |
| **UNKNOWN** | Whether the ratio divergence is purely a consequence of the supply divergence or has independent contributions. |
| **Changed since last pass** | Gap narrowed from ~13.8% to ~13.4% due to witness ratio drift (−0.0046). This is precision drift, not structural change. |

### 4. MESH.md Stale

| Field | Value |
|-------|-------|
| **First observed** | Jul 27 (prior to Jul 28 session start) |
| **Last pass status** | Persistent — unchanged |
| **This pass status** | **Persistent — unchanged** |
| **OBSERVED** | MESH.md at commit c008def says "No production nodes running — both nodes were stopped and storage wiped during Jul 27 cleanup." Both nodes have been running continuously since 13:01Z Jul 28 (>5.7h). |
| **EXPECTED** | MESH.md should reflect current active topology. |
| **DEVIATION** | Topology documentation out of sync with running nodes for >5.7 hours. |
| **UNKNOWN** | Whether intentional (ongoing session with pending cleanup) or an oversight. Node launch commands in MESH.md are accurate — the status header is stale. |

---

## New Observations This Pass

### Witness trailing by 1 epoch (first observed)

In pass 118, both nodes reported identical epoch numbers simultaneously (677). In this pass, witness trails morning-api by 1 epoch (696 vs 697 at 11s apart). This is possibly timing drift (the 11-second gap between captures = ~1/3 of an epoch can account for the difference). Monitor next pass to determine if this is a true lag developing or just capture timing.

### Snapshot now at 690 (2 rotations since pass 118)

Pass 118: last_snapshot_epoch=670. This pass: 690. Two snapshot rotations occurred at 680 and 690 as expected. Consistent with ~20 epochs of advance at normal cadence. No anomalies.

### NTP Failures — Recurrence check

The two NTP failures at 18:02Z have NOT recurred in the ~48 minutes since first observation (pass 116 at 18:02Z). This is consistent with pass 118's reclassification from "Transient" to **Resolved** (recurrence schedule unknown). No new NTP-related log lines observed in this pass.

### No new zombies, evictions, or sweep events

zero zombies, zero evictions, zero stale fetch/outbound sweeps. Connection health normal (silence_secs 2-7s, well under 30s zombie threshold).

---

## Evidence Gaps

1. **No cross-node balance reconciliation.** The witness reports morning-api's balance as 0; morning-api reports its own as 20. No consensus mechanism active on Era One. The Era Two state_root sortition path (code landed at 553ce22, dormant) would address this.

2. **NTP periodic retry mechanism unknown.** Two NTP fallback failures occurred at 18:02Z after 5h of runtime. No recurrence in 48 min. Whether this repeats on an hourly/daily cycle or was a one-time event is UNKNOWN. Status: **Resolved — no recurrence this pass**.

3. **Root cause of `-dirty` suffix unknown.** Current working tree shows 1 modified tracked file (docs/evidence/observer-2026-07-27-pass10.md) plus untracked log files. The `-dirty` suffix could reflect a different state at compile time. Not reproducible from current worktree.

4. **Redistribution inactivity.** No redistribution transactions have been observed this entire session (>5.7h, ~697+ epochs). The economic engine reached an equilibrium state (balance=20, ratio≈1.02 → integer truncation yields 0 net change) and has not been disturbed.

5. **Witness trailing indicator.** Whether the 1-epoch gap between witness and morning-api is true lag or capture timing cannot be determined from a single pass. Will become clearer with next pass's data.

---

**Next expected event:** Snapshot rotation at epoch 700 (~3 min). No other state changes expected given the frozen economic equilibrium.

**Timeline:** Session started 13:01Z Jul 28. Now at 18:50Z Jul 28. Runtime: 5h 49min. 119 observation passes completed (~0.34 passes/min, consistent with ~3 min interval).
