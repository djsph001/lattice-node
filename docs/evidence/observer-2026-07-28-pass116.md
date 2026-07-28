# Observer Evidence Record — 2026-07-28 (Pass 116)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** 2026-07-28T18:21:43Z – 18:22:48Z (simultaneous capture at 18:22:48Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** 116th observation pass of Jul 28. ~27 min since pass 115 (17:55Z). Sockets responsive, PIDs unchanged (3579452/3579821).

**Summary:** Delta-only from pass 115. All evidence guards PASS. Epochs advanced +55/+54 at normal cadence (~30s). Snapshot rotated from 580→640 (6 rotations). Economic state completely frozen — unchanged entire session (>5h). Two persistent deviations unchanged (stale build_commit, supply conservation divergence). One new observation: NTP fallback warnings on morning-api at 18:02Z.

---

## Topology Disclosure

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 13:01Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 13:02Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes since session start (13:01Z).** Same PIDs across all 116 passes.

---

## Evidence Integrity Guards — Simultaneous Capture (18:22:48Z)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch (morning-api, 18:22:48Z) | Socket=643, Log count=643, Last log epoch at instant=643 | All three match at a single instant | **PASS** — exactly equal at 18:22:48Z |
| Three-way epoch (witness, 18:22:48Z) | Socket=642, Log count=642, Last log epoch=642 | All three match at a single instant | **PASS** — exactly equal at 18:22:48Z. Witness trails morning-api by 1 epoch (normal boundary timing: witness ticks ~30s offset from morning-api) |
| Byte-equality (morning-api) | wal_bytes=379 (socket), stat=379 (ls -la wal.log) | Must match | **PASS** |
| Byte-equality (witness) | wal_bytes=379 (socket), stat=379 (ls -la wal.log) | Must match | **PASS** |

---

## Metrics (Node Health)

### morning-api (18:21:43Z primary, 18:22:48Z simultaneous)

| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| uptime_secs | 19,195 (18:21Z) | Increasing at ~10s per metrics tick | None. Δ from pass 115: +1,591s (~26.5 min, matching wall time) |
| epoch | 640 (18:21Z); 643 (18:22Z) | Increasing at ~30s per epoch | None. Δ from pass 115: +52/+55 epochs in ~27 min = ~30s/epoch |
| height | 1 | 1 (unchanged since genesis) | None |
| peers | 1 (12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch, heartbeats=1917, silence_secs=8, is_dead=false) | 1 peer (local-witness) | None. Heartbeats increasing steadily (Δ ~160 from pass 115's 1758) |
| own_balance | 20 | 20 (frozen since at least epoch 443) | **PERSISTENT** — supply conservation divergence documented since Jul 27. Unchanged since pass 115. |
| own_nonce | 241 | 241 (frozen) | None — nonce frozen since redistribution stopped |
| outstanding_fetches | 0 | 0 | None |
| aged_fetches | 0 | 0 | None |
| outbound_queues | [] | [] | None |
| max_peer_silence | 6s | <10s | None |
| build_commit | cb5d4b1-dirty | 452b64f (git HEAD) | **PERSISTENT** — binary 1+ commits behind HEAD + dirty suffix. First observed Jul 27. Unchanged. |
| thickness | 977.63 | Drifts slowly (was 977.64 at earlier capture) | None — normal slow drift from economic inactivity |

### local-witness (18:21:59Z primary, 18:22:48Z simultaneous)

| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| uptime_secs | 19,188 (18:21Z) | ~19,188 (started ~23s after morning-api) | None. Δ from pass 115: +1,580s |
| epoch | 640 (18:21Z); 642 (18:22Z) | 640+ (matches morning-api within one) | None |
| height | 1 | 1 | None |
| peers | 1 (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ, heartbeats=1919, silence_secs=8, is_dead=false) | 1 peer (morning-api) | None. Heartbeats increasing steadily (Δ ~158 from pass 115's 1761) |
| own_balance | 0 | 0 (never received redistribution) | **PERSISTENT** — supply conservation divergence, documented since Jul 27 |
| own_nonce | 4 | 4 (frozen) | None |
| outstanding_fetches | 0 | 0 | None |
| aged_fetches | 0 | 0 | None |
| outbound_queues | [] | [] | None |
| max_peer_silence | 3s | <10s | None |
| build_commit | cb5d4b1-dirty | 452b64f (git HEAD) | **PERSISTENT** — same stale binary as morning-api |

---

## Economic State

### morning-api (18:21:43Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | From initial mint of 5,000, redistributed to near-zero floor. Frozen since at least epoch 443 (~5h ago). | None (terminal state for current parameters). Ratio=1.019 on balance=20 yields net zero change from integer truncation. |
| peer (witness) balance | 9,980 | witness reports 0 for itself | **PERSISTENT** — supply conservation divergence (VERIFIED-BEHAVIOR.md: CONTRADICTED). Unchanged. |

### local-witness (18:21:59Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 (never received any mint) | None |
| peer (morning-api) balance | 0 | morning-api reports 20 for itself | **PERSISTENT** — witness ledger sees morning-api as 0 balance. Unchanged. |

---

## Persistence State

### morning-api (18:22:48Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 640 | Increments every 10 epochs | None. Δ from pass 115: 580→640 (6 rotations at 590, 600, 610, 620, 630, 640) |
| wal_bytes | 379 | 379 (byte-equality passes) | None |
| wal_entries | 3 | 3 (size/120 heuristic) | Known-provisional (VERIFIED-BEHAVIOR.md) |

### local-witness (18:22:48Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 640 | Matches morning-api | None — both nodes snapshotted at same epoch |
| wal_bytes | 379 | 379 (byte-equality passes) | None |
| wal_entries | 3 | 3 (size/120 heuristic) | Known-provisional |
| wal.wal.old present | 379 bytes | Pre-rotation WAL file | Cosmetic naming — known-provisional (VERIFIED-BEHAVIOR.md). Same size as active wal.log | state.snapshot | 569 bytes (witness), 894 bytes (morning-api) | Different snapshot sizes due to different balance state | None — expected given economic state divergence |

---

## Log Health Scan (18:22Z)

### morning-api

| Pattern | Count | Notes |
|---------|-------|-------|
| WARN (filtered) | 2 | NTP fallback failures at 18:02:00Z and 18:02:03Z — see "New Observations" below |
| ERROR (filtered) | 0 | Clean |
| Zombie/sweep/eviction | 0 | None occurred |
| Heartbeats flowing | Yes | Every 3-5s from witness, incrementing normally (1925 at last check) |
| Metrics healthy | Yes | outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=6s |
| Epoch cycling | Yes | Every ~30s, balance=20 constant |

### local-witness

| Pattern | Count | Notes |
|---------|-------|-------|
| WARN (filtered) | 0 | Clean |
| ERROR (filtered) | 0 | Clean |
| Zombie/sweep/eviction | 0 | None occurred |
| Heartbeats flowing | Yes | Every 3-5s from morning-api, incrementing normally (1926 at last check) |
| Metrics healthy | Yes | outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=3s |
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
| **UNKNOWN** | Whether this affects any runtime behavior. The wal_bytes fix (documented in 452b64f as "Fixed") is verified WORKING — byte-equality passes. The fix may have been in cb5d4b1 already. |
| **Changed since last pass** | No |

### 2. Supply Conservation Divergence (morning-api=20, witness=0)

| Field | Value |
|-------|-------|
| **First observed** | Jul 27, pass #3 (18:48 EDT) |
| **Last pass status** | Persistent |
| **This pass status** | Persistent — unchanged |
| **OBSERVED** | morning-api balance=20 (own), 9,980 (witness). witness balance=0 (own), 0 (morning-api). Total supply per morning-api: 10,000. Total supply per witness: 0. Frozen entire session (>5h). |
| **EXPECTED** | Supply Conservation Invariant (proposed, pending governance): sum of spendable balances across the mesh should equal the network's recognized total supply. |
| **DEVIATION** | CONTRADICTED — the two nodes' ledgers disagree on total supply by a factor of 10,000 vs 0. Verified in VERIFIED-BEHAVIOR.md. |
| **UNKNOWN** | The cause (initial mint local-only, sender debits before recipient confirms, no reconciliation mechanism) — these are documented in VERIFIED-BEHAVIOR.md as "Causes / Contributing Conditions," not verified diagnoses. |
| **Changed since last pass** | No — balance frozen at 20 since at least epoch 443. No redistribution activity visible. |

### 3. MESH.md Stale

| Field | Value |
|-------|-------|
| **First observed** | Jul 27 (prior to Jul 28 session start) |
| **Last pass status** | Persistent |
| **This pass status** | Persistent — unchanged |
| **OBSERVED** | MESH.md: "**No production nodes running.** Both nodes were stopped and storage wiped during the Jul 27 build-check cleanup." |
| **EXPECTED** | MESH.md should reflect current active topology: both nodes running since 13:01Z Jul 28. |
| **DEVIATION** | Documentation out of sync with running nodes for >5 hours. |
| **UNKNOWN** | Whether intentional (mid-session with pending cleanup) or an oversight. |

---

## New Observations This Pass

### 1. NTP Fallback Warnings on morning-api (18:02Z)

Two WARN log lines, first occurrence in this session:

- `2026-07-28T18:02:00.777454Z  WARN lattice_node::startup: NTP query to pool.ntp.org failed: Input/output error: Resource temporarily unavailable (os error 11) (fallback)`
- `2026-07-28T18:02:03.977439Z  WARN lattice_node::startup: NTP query to time.apple.com failed: Input/output error: Resource temporarily unavailable (os error 11) (fallback)`

| Field | Value |
|-------|-------|
| **First observed** | 2026-07-28T18:02:00Z (this pass) |
| **Status** | New — not present in pass 115 (17:55Z) or any prior pass |
| **OBSERVED** | Two NTP fallback failures ~9 min ago on morning-api. No NTP failures on witness. |
| **EXPECTED** | Startup check at 13:01:47Z verified clock with `drift 0s (threshold ±300s)`. Periodic NTP re-checks (if implemented) should succeed or fall back gracefully. |
| **DEVIATION** | NTP servers pool.ntp.org and time.apple.com both returned `os error 11` (Resource temporarily unavailable). The startup clock check passed; these are fallback queries from a periodic retry, not the startup check. |
| **UNKNOWN** | (1) Whether this is a new periodic NTP check mechanism or a one-time retry after 5h. (2) Whether the same failure occurred at startup but was logged at a lower severity (no NTP WARN lines at 13:01Z). (3) Whether this will recur — only occurrence in the ~5h log so far. |

### 2. Equilibrium Confirmed

Pass 115 noted the balance of 20 was approaching an equilibrium point under integer truncation (ratio=1.02 × 20 ≈ 20.4 → 20). This pass confirms: after 55 additional epochs and 6 snapshot rotations, balance=20 is stable. The `balance_before=20 balance_after=20` in every Epoch complete line for the past ~27 min is exact evidence of a fixed point.

| Field | Value |
|-------|-------|
| **First observed** | Jul 28, epoch ~443 (~13:40Z) |
| **Status** | Persistent — confirmed stable across 55+ epochs |
| **OBSERVED** | balance=20 unchanged across 55 epochs (~27 min) since pass 115's epoch 588. Epoch complete lines verify: `balance_before=20 balance_after=20 ratio=1.02`. |
| **EXPECTED** | At small integer balances with ratio near 1.0, economic redistribution converges to a fixed point due to integer truncation. |
| **DEVIATION** | None — this is expected terminal behavior under the current parameters. |
| **UNKNOWN** | Whether the defined equilibrium is *intended* or *incidental* — the economic design docs would determine this. |

---

## Evidence Gaps

1. **No cross-node balance reconciliation.** The witness reports morning-api's balance as 0; morning-api reports its own as 20. There is no consensus mechanism active to resolve this on Era One. The Era Two state_root sortition path (code landed at 553ce22, dormant) would address this.

2. **NTP periodic retry mechanism unknown.** A single NTP retry event occurred at 18:02Z after 5h of runtime. Whether this repeats on an hourly/daily cycle or was a one-time event is unknown — only one observation.

3. **No coverage for `--no-mdns` flag effectiveness** — flagged in pass 115. Unchanged.

---

**Next expected event:** Snapshot rotation at epoch 650 (~5 min). No other state changes expected given the frozen economic equilibrium.
