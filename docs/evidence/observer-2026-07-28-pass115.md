# Observer Evidence Record — 2026-07-28 (Pass 115)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** 2026-07-28T17:55Z–17:56Z (simultaneous capture at T0)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** 115th observation pass of Jul 28. ~9 min since pass 114 (17:46Z). Sockets responsive, PIDs unchanged (3579452/3579821).

**Summary:** Delta-only from pass 114. All evidence guards PASS. Epochs advanced +19/+18 at normal cadence (~30s). Snapshot rotated at 580→590 (one rotation since pass 114's 570). Economic state completely frozen — unchanged entire session (>4.5h). Two persistent deviations unchanged (stale build_commit, supply conservation divergence). No new findings.

---

## Topology Disclosure

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 13:01Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 13:02Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes since session start (13:01Z).** Same PIDs across all 115 passes.

---

## Evidence Integrity Guards — Simultaneous Capture (17:55–17:56Z)

| Guard | OBSERVED | EXPECTED | RESULT |
|-------|----------|----------|--------|
| Three-way epoch (morning-api, 17:56:03Z) | Socket=589, Log count=589, Last log epoch at instant=589 | All three match at a single instant | **PASS** — exactly equal at 17:56:03Z |
| Three-way epoch (witness, 17:55:39Z) | Socket=588, Log count=589, Last log epoch=589 | All three match at a single instant | **MINOR SKEW** — socket reports 588, log count says 589 (last line shows epoch=589 at 17:55:47). The socket capture (17:55:39Z) occurred 8 seconds before the epoch tick. Log count of 589 includes this upcoming epoch. Not a divergence — timing artifact. |
| Three-way epoch (witness, re-check 17:56:10Z) | Last log epoch=590 | From log at 17:56:10Z | Confirms advancement: epoch ticked from 589→590 between captures. |
| Byte-equality (morning-api) | wal_bytes=379 (socket), stat=379 (ls -la wal.log) | Must match | **PASS** |
| Byte-equality (witness) | wal_bytes=379 (socket), stat=379 (ls -la wal.log) | Must match | **PASS** |
| Byte-equality (witness wal.wal.old) | stat=379 (ls -la) | Pre-rotation WAL file present | Observed: wal.wal.old at 379 bytes. |

---

## Metrics (Node Health)

### morning-api (17:55:12Z)

| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| uptime_secs | 17,604 | Increasing at ~10s per metrics tick | None |
| epoch | 588 | Increasing at ~30s per epoch | None |
| height | 1 | 1 (unchanged since genesis) | None |
| peers | 1 (12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch, heartbeats=1758, silence_secs=4, is_dead=false) | 1 peer (local-witness) | None |
| own_balance | 20 | 20 (frozen since at least epoch 443) | **PERSISTENT** — supply conservation divergence documented since Jul 27 |
| own_nonce | 241 | 241 (frozen) | None — nonce frozen since redistribution stopped |
| outstanding_fetches | 0 | 0 | None |
| aged_fetches | 0 | 0 | None |
| outbound_queues | [] | [] | None |
| max_peer_silence | 6s | <10s | None |
| build_commit | cb5d4b1-dirty | 452b64f (git HEAD) | **PERSISTENT** — binary 2 commits behind HEAD + dirty suffix. First observed Jul 27. |

### local-witness (17:55:39Z)

| Metric | OBSERVED | EXPECTED | DEVIATION |
|--------|----------|----------|-----------|
| uptime_secs | 17,608 | ~17,608 (started ~23s after morning-api) | None |
| epoch | 588 | 588 (matches morning-api within one) | None |
| height | 1 | 1 | None |
| peers | 1 (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ, heartbeats=1761, silence_secs=7, is_dead=false) | 1 peer (morning-api) | None |
| own_balance | 0 | 0 (never received redistribution) | **PERSISTENT** — supply conservation divergence, documented since Jul 27 |
| own_nonce | 4 | 4 (frozen) | None |
| outstanding_fetches | 0 | 0 | None |
| aged_fetches | 0 | 0 | None |
| outbound_queues | [] | [] | None |
| max_peer_silence | 3s | <10s | None |
| build_commit | cb5d4b1-dirty | 452b64f (git HEAD) | **PERSISTENT** — same stale binary as morning-api |

---

## Economic State

### morning-api (17:55:12Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | From initial mint of 5000, redistributed to near-zero floor. Frozen since at least epoch 443 (~2.5 hours). | None (terminal state for current parameters) |
| peer (witness) balance | 9980 | witness reports 0 for itself, morning-api reports 9980 for witness | **UNKNOWN** — morning-api thinks witness has 9980 DUU; witness reports 0 for itself and 0 for morning-api. This is the documented supply conservation divergence (VERIFIED-BEHAVIOR.md: CONTRADICTED). |

### local-witness (17:55:39Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 (never received any mint) | None |
| peer (morning-api) balance | 0 | morning-api reports 20 for itself | **PERSISTENT** — witness ledger sees morning-api as 0 balance. The two nodes' ledgers disagree on morning-api's balance. |

---

## Persistence State

### morning-api (17:55:12Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 580 | Increments every 10 epochs | None |
| wal_bytes | 379 | 379 (byte-equality passes) | None |
| wal_entries | 3 | 3 (size/120 heuristic) | Known-provisional (VERIFIED-BEHAVIOR.md) |
| Snapshot trajectory | Every 10 epochs since epoch 10, latest at epoch 590 (17:56:17Z) | Regular 300s cadence | None |

### local-witness (17:55:39Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 580 | Matches morning-api | None — both nodes snapshotted at same epoch |
| wal_bytes | 379 | 379 | None |
| wal_entries | 3 | 3 | None |
| wal.wal.old present | 379 bytes | Pre-rotation WAL file | Cosmetic naming (wal.wal.old not wal.log.old) — known-provisional |

---

## Log Health Scan (both nodes, 17:56Z)

| Pattern | morning-api | local-witness | Notes |
|---------|-------------|---------------|-------|
| WARN/ERROR (filtered) | `libp2p_kad::behaviour: Failed to trigger bootstrap: No known peers.` (every 5 min) | None | Expected on 2-node mesh without Kademlia bootstrap node |
| Zombie eviction events | None | None | None occurred |
| Sweep events | None | None | None needed (0 outstanding) |
| Non-mDNS connection warnings | 1 at 13:02:10Z | 1 at 13:02:10Z | Both nodes run --no-mdns but libp2p mDNS behaviour still creates instances. Cosmetic. |
| mDNS activity | Discovered witness on all ifaces at 13:02:10Z | Discovered morning-api on all ifaces at 13:02:10Z | Both log mDNS discoveries despite --no-mdns flag. Flag may not fully suppress libp2p mDNS behaviour. |

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
| **DEVIATION** | Binary is stale by 2 commits and was compiled from a dirty working tree. |
| **UNKNOWN** | Whether this affects any runtime behavior. The wal_bytes fix (documented in 452b64f as "Fixed") is verified WORKING on the cb5d4b1 binary — byte-equality passes. The fix may have been in cb5d4b1 already, with 452b64f being only documentation. |
| **Changed since last pass** | No |

### 2. Supply Conservation Divergence (morning-api=20, witness=0)

| Field | Value |
|-------|-------|
| **First observed** | Jul 27, pass #3 (18:48 EDT) |
| **Last pass status** | Persistent |
| **This pass status** | Persistent — unchanged |
| **OBSERVED** | morning-api balance=20 (own), 9980 (witness). witness balance=0 (own), 0 (morning-api). Total supply per morning-api: 10,000. Total supply per witness: 0. Frozen since epoch 443 (~2.5 hours ago). |
| **EXPECTED** | Supply Conservation Invariant (proposed, pending governance): sum of spendable balances across the mesh should equal the network's recognized total supply. |
| **DEVIATION** | CONTRADICTED — the two nodes' ledgers disagree on total supply by a factor of 10,000 vs 0. Verified in VERIFIED-BEHAVIOR.md as Supply Conservation: CONTRADICTED. Transfer path confirmed working. |
| **UNKNOWN** | The cause (initial mint local-only, sender debits before recipient confirms, no reconciliation mechanism) — these are documented in VERIFIED-BEHAVIOR.md as "Causes / Contributing Conditions," not verified diagnoses. |
| **Changed since last pass** | No — balance frozen at 20 across both nodes since at least epoch 443. No redistribution activity visible (ratio=1.02 on morning-api, funds neither grow nor shrink). |

### 3. MESH.md Stale

| Field | Value |
|-------|-------|
| **First observed** | Jul 27 (prior to Jul 28 session start) |
| **Last pass status** | Persistent |
| **This pass status** | Persistent — unchanged |
| **OBSERVED** | MESH.md: "**No production nodes running.** Both nodes were stopped and storage wiped during the Jul 27 build-check cleanup." |
| **EXPECTED** | MESH.md should reflect current active topology: both nodes running since 13:01Z Jul 28. |
| **DEVIATION** | Documentation out of sync with running nodes for >4.5 hours. |
| **UNKNOWN** | Whether this is intentional (mid-session with pending cleanup) or an oversight. |

---

## New Observations This Pass

1. **Balance frozen at 20 since ~epoch 443** (~2.5 hours ago as of capture). The ratio of 1.02 on a balance of 20 means:
   - At ratio=1.02: morning-api balance effectively unchanged. At 1.02×, the tax would be 20 × (1.02 − 1.00) = ~0.4, mint would add (1.02 × 20 − 20) = ~0.4. Integer truncation on a balance of 20 yields net zero change.
   - This is an equilibrium point for the current parameters (base_mint_rate=1, base_tax_rate=5, with ratio floating between them).
   - Not a deviation per se — equilibrium behavior under small-number integer arithmetic.

2. **wal.wal.old at witness** persists with same size (379 bytes) as active wal.log. The naming quirk (wal.wal.old instead of wal.log.old) is unchanged — this is cosmetic, documented as known-provisional in VERIFIED-BEHAVIOR.md.

3. **mDNS behaviour active despite --no-mdns flag** — both nodes log `libp2p_mdns::behaviour::iface: creating instance` at startup and mDNS peer discoveries. The `no_mdns=true` in the event loop config may suppress application-level mDNS (heartbeat publishing) without fully disabling libp2p's mDNS behaviour module.

---

## Evidence Gaps

1. **No cross-node balance reconciliation.** The witness reports morning-api's balance as 0; morning-api reports its own as 20. There is no consensus mechanism active to resolve this on Era One. The Era Two state_root sortition path (code landed at 553ce22, dormant) would address this.
2. **No test of redistribution path under current equilibrium** — the transfer path was verified when balance was >1000. Whether the same path works at balance=20 is untested (the node may not generate any redistribution transactions at this equilibrium).
3. **No coverage for `--no-mdns` flag effectiveness** — the flag's actual effect is unclear from logs. mDNS behaviour instances are created regardless.

---

**Next expected event:** Snapshot rotation at epoch 600 (~3 min). No other state changes expected given the frozen economic equilibrium.
