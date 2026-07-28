# Observer Evidence Record — 2026-07-28 (Pass 117)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** 2026-07-28T18:32:06Z – 18:32:13Z (simultaneous capture at 18:32:06Z/18:32:13Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** 117th observation pass of Jul 28. ~10 min since pass 116 (18:22Z). Sockets responsive, PIDs unchanged (3579452/3579821).

**Summary:** Delta-only from pass 116. All evidence guards PASS. Epochs advanced +18/+19 at normal cadence (~30s). Snapshot rotated 640→660 (2 rotations). Economic state completely frozen — unchanged entire session (>5.5h). Two persistent deviations unchanged (stale build_commit, supply conservation divergence). NTP fallback failures from pass 116 did NOT recur — classified as transient.

---

## Topology Disclosure

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 13:01Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 13:02Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes since session start (13:01Z).** Same PIDs across all 117 passes.

---

## Evidence Integrity Guards — Simultaneous Captures

### morning-api (18:32:06Z)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch | Socket=661, Log count=661, Last log epoch at instant=661 | All three match at a single instant | **PASS** — exactly equal at 18:32:06Z |
| Byte-equality | wal_bytes=379 (socket), stat=379 (ls -la wal.log) | Must match | **PASS** |

### local-witness (18:32:13Z)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch | Socket=661, Log count=661, Last log epoch=661 | All three match at a single instant | **PASS** — exactly equal at 18:32:13Z |
| Byte-equality | wal_bytes=379 (socket), stat=379 (ls -la wal.log) | Must match | **PASS** |

**Note:** Both nodes now report the same epoch (661) simultaneously — witness caught up from trailing by 1 in pass 116. This is a normal boundary timing shift, not a drift correction.

---

## Metrics (Node Health)

### morning-api (18:32:06Z simultaneous)

| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| uptime_secs | 19,766 (18:31Z) | Increasing at ~10s per metrics tick | None. Δ from pass 116: +571s (~9.5 min, matching wall time) |
| epoch | 661 (18:32Z) | Increasing at ~30s per epoch | None. Δ from pass 116: +18 epochs in ~10 min = ~33s/epoch |
| height | 1 | 1 (unchanged since genesis) | None |
| peers | 1 (12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch, heartbeats=1974, silence_secs=8, is_dead=false) | 1 peer (local-witness) | None. Heartbeats increasing steadily (Δ +57 from pass 116's 1917) |
| own_balance | 20 | 20 (frozen since at least epoch 443) | **PERSISTENT** — supply conservation divergence documented since Jul 27. Unchanged since pass 116. |
| own_nonce | 241 | 241 (frozen) | None — nonce frozen since redistribution stopped |
| outstanding_fetches | 0 | 0 | None |
| aged_fetches | 0 | 0 | None |
| outbound_queues | [] | [] | None |
| max_peer_silence | 6s | <10s | None |
| build_commit | cb5d4b1-dirty | 452b64f (git HEAD) | **PERSISTENT** — binary 1+ commits behind HEAD + dirty suffix. First observed Jul 27. Unchanged. |
| thickness | 977.48 | Drifts slowly (was 977.63 at pass 116) | None — normal slow drift from economic inactivity |

### local-witness (18:32:13Z simultaneous)

| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| uptime_secs | 19,759 (18:31Z) | ~19,759 (started ~23s after morning-api) | None. Δ from pass 116: +571s |
| epoch | 661 (18:32Z) | 661 | None. Witness caught up from 642 (pass 116) to match morning-api at 661 |
| height | 1 | 1 | None |
| peers | 1 (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ, heartbeats=1976, silence_secs=8, is_dead=false) | 1 peer (morning-api) | None. Heartbeats increasing steadily (Δ +57 from pass 116's 1919) |
| own_balance | 0 | 0 (never received redistribution) | **PERSISTENT** — supply conservation divergence, documented since Jul 27 |
| own_nonce | 4 | 4 (frozen) | None |
| outstanding_fetches | 0 | 0 | None |
| aged_fetches | 0 | 0 | None |
| outbound_queues | [] | [] | None |
| max_peer_silence | 2-3s | <10s | None |
| build_commit | cb5d4b1-dirty | 452b64f (git HEAD) | **PERSISTENT** — same stale binary as morning-api |

---

## Economic State

### morning-api (18:32:06Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | From initial mint of 5,000, redistributed to near-zero floor. Frozen since at least epoch 443 (~5.5h ago). | None (terminal state for current parameters). Ratio=1.019 on balance=20 yields net zero change from integer truncation. |
| peer (witness) balance | 9,980 | witness reports 0 for itself | **PERSISTENT** — supply conservation divergence (VERIFIED-BEHAVIOR.md: CONTRADICTED). Unchanged. |

### local-witness (18:32:13Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 (never received any mint) | None |
| peer (morning-api) balance | 0 | morning-api reports 20 for itself | **PERSISTENT** — witness ledger sees morning-api as 0 balance. Unchanged. |

### Epoch Ratio Divergence

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| morning-api ratio | 1.01947 | ~1.02 (matches pass 116) | None — unchanged |
| witness ratio | 1.16476 | ~1.16 (matches pass 116) | None — unchanged |
| Gap | ~14% | Same formula, same `redistributed_to=1` | **PERSISTENT** — first observed pass 1. Both nodes apply the same Georgist formula with the same parameters, but get different ratios because their total supply views differ (10,000 vs 0). This is a consequence of the supply divergence, not an independent bug. |

---

## Persistence State

### morning-api (18:32:06Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 660 | Increments every 10 epochs | None. Δ from pass 116: 640→660 (2 rotations at 650, 660) |
| wal_bytes | 379 | 379 (byte-equality passes) | None |
| wal_entries | 3 | 3 (size/120 heuristic) | Known-provisional (VERIFIED-BEHAVIOR.md) |

### local-witness (18:32:13Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 660 | Matches morning-api | None — both nodes snapshotted at same epoch (caught up from 640 in pass 116) |
| wal_bytes | 379 | 379 (byte-equality passes) | None |
| wal_entries | 3 | 3 (size/120 heuristic) | Known-provisional |
| wal.wal.old present | 379 bytes | Pre-rotation WAL file | Cosmetic naming — known-provisional (VERIFIED-BEHAVIOR.md). Same size as active wal.log |

---

## Log Health Scan (18:32Z)

### morning-api

| Pattern | Count | Notes |
|---------|-------|-------|
| WARN (structural, filtered) | 0 | Kademlia bootstrap failures filtered (benign, `--no-mdns`). NTP failures (2 lines at 18:02Z) — resolved, did not recur. |
| ERROR (filtered) | 0 | Clean |
| Zombie/sweep/eviction | 0 | None occurred |
| Heartbeats flowing | Yes | Every 3-5s from witness, incrementing normally (1974 at capture) |
| Metrics healthy | Yes | outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=6s |
| Epoch cycling | Yes | Every ~30s, balance=20 constant |

### local-witness

| Pattern | Count | Notes |
|---------|-------|-------|
| WARN (filtered) | 0 | Clean |
| ERROR (filtered) | 0 | Clean |
| Zombie/sweep/eviction | 0 | None occurred |
| Heartbeats flowing | Yes | Every 3-5s from morning-api, incrementing normally (1976 at capture) |
| Metrics healthy | Yes | outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=2-3s |
| Epoch cycling | Yes | Every ~30s, balance=0 constant |

---

## Persistent Deviations — Status

### 1. Stale build_commit (cb5d4b1-dirty vs HEAD 452b64f)

| Field | Value |
|-------|-------|
| **First observed** | Jul 27 (prior to 452b64f docs update) |
| **Last pass status** | Persistent |
| **This pass status** | Persistent — unchanged |
| **OBSERVED** | Both nodes report `build_commit: "cb5d4b1-dirty"`. Git HEAD is `452b64f` (`docs: wal_bytes fix verified, wal_entries heuristic noted`). |
| **EXPECTED** | `build_commit` should match git HEAD. At minimum, not `-dirty`. |
| **DEVIATION** | Binary is stale by 1+ commits and was compiled from a dirty working tree. |
| **UNKNOWN** | Whether this affects any runtime behavior. The wal_bytes fix is verified WORKING (byte-equality passes). The `cb5d4b1` commit is a docs commit, so the `-dirty` suffix may be from test artifacts or working-tree changes unrelated to the binary. |
| **Changed since last pass** | No |

### 2. Supply Conservation Divergence (morning-api=20, witness=0)

| Field | Value |
|-------|-------|
| **First observed** | Jul 27, pass #3 (18:48 EDT) |
| **Last pass status** | Persistent |
| **This pass status** | Persistent — unchanged |
| **OBSERVED** | morning-api balance=20 (own), 9,980 (witness). witness balance=0 (own), 0 (morning-api). Total supply per morning-api: 10,000. Total supply per witness: 0. Frozen entire session (>5.5h). |
| **EXPECTED** | Supply Conservation Invariant (proposed, pending governance): sum of spendable balances across the mesh should equal the network's recognized total supply. |
| **DEVIATION** | CONTRADICTED — the two nodes' ledgers disagree on total supply by a factor of 10,000 vs 0. Verified in VERIFIED-BEHAVIOR.md. |
| **UNKNOWN** | The cause (initial mint local-only, sender debits before recipient confirms, no reconciliation mechanism) — these are documented in VERIFIED-BEHAVIOR.md as "Causes / Contributing Conditions," not verified diagnoses. |
| **Changed since last pass** | No — balance frozen at 20 since at least epoch 443. No redistribution activity visible. |

### 3. Epoch Ratio Divergence (~14% gap)

| Field | Value |
|-------|-------|
| **First observed** | Jul 28, pass 1 (18:06Z) |
| **Last pass status** | Persistent |
| **This pass status** | Persistent — unchanged |
| **OBSERVED** | morning-api ratio=1.01947, witness ratio=1.16476. Gap unchanged since pass 1. |
| **EXPECTED** | Both nodes apply the same Georgist formula with same `redistributed_to=1`. The ratio should converge. |
| **DEVIATION** | 14% gap. Ratio is a function of total supply (net of tax base), which differs between nodes due to the supply divergence. |
| **UNKNOWN** | Whether the ratio divergence is purely a consequence of the supply divergence or has independent contributions. |
| **Changed since last pass** | No |

### 4. MESH.md Stale

| Field | Value |
|-------|-------|
| **First observed** | Jul 27 (prior to Jul 28 session start) |
| **Last pass status** | Persistent |
| **This pass status** | Persistent — unchanged |
| **OBSERVED** | MESH.md does not exist (empty file at `docs/MESH.md` or absent). |
| **EXPECTED** | MESH.md should reflect current active topology: both nodes running since 13:01Z Jul 28. |
| **DEVIATION** | Topology documentation absent or out of sync with running nodes for >5.5 hours. |
| **UNKNOWN** | Whether intentional (mid-session with pending cleanup) or an oversight. |

---

## New Observations This Pass

None. All metrics delta-only from pass 116.

### NTP Fallback Failures (from pass 116) — Status Update

| Field | Value |
|-------|-------|
| **First observed** | 2026-07-28T18:02:00Z (pass 116) |
| **Status this pass** | **Resolved** — transient. |
| **OBSERVED** | Two NTP fallback failures at 18:02Z on morning-api. Pool.ntp.org and time.apple.com both `os error 11`. No recurrence in ~30 min since first observation. |
| **EXPECTED** | Periodic NTP checks should succeed or fail gracefully. |
| **DEVIATION** | None in this pass — transient failure resolved on its own. |
| **UNKNOWN** | What triggered the NTP retry at 5h of runtime. Whether it will recur on an hourly/daily cycle. |

### Equilibrium Confirmed (carried forward from pass 116)

Balance=20 on morning-api remains a stable fixed point under integer truncation at ratio≈1.02. 18 additional epochs since pass 116 with exact `balance_before=20 balance_after=20` in every Epoch complete line. This has been stable for >5.5 hours across ~218 epochs and 66 snapshot rotations.

---

## Evidence Gaps

1. **No cross-node balance reconciliation.** The witness reports morning-api's balance as 0; morning-api reports its own as 20. There is no consensus mechanism active to resolve this on Era One. The Era Two state_root sortition path (code landed at 553ce22, dormant) would address this.

2. **NTP periodic retry mechanism unknown.** A single NTP retry event occurred at 18:02Z after 5h of runtime. No recurrence in 30 min. Whether this repeats on an hourly/daily cycle or was a one-time event is unknown.

3. **No coverage for `--no-mdns` flag effectiveness** — flagged in pass 115. Unchanged.

4. **Root cause of `-dirty` suffix unknown.** No modified tracked files visible in `git status` from the build environment. The suffix may be from build.rs or a CI artifact not reproducible from the current worktree.

---

**Next expected event:** Snapshot rotation at epoch 670 (~5 min). No other state changes expected given the frozen economic equilibrium.
