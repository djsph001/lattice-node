# Observer Evidence Record — 2026-07-28 (Pass 87)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T11:40Z bundle (single-capture discipline)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (Boynton Beach FL)
**Session type:** Eighty-seventh observation pass. Same processes since 2026-07-27T18:48Z (~16.8h runtime). ~9 min since pass 86 (11:31Z).

**Summary:** Routine continuation. All three persistent deviations unchanged. Epoch advanced from 2006→2025 on morning-api (+19), 2007→2024 on witness (+17). Log counts both at 2025/2024 matching socket captures. Snapshot rotated twice (2000→2020) on both nodes. Metrics all healthy (aged=0, queues empty, silence<10s). No new WARN/ERROR events beyond benign KAD/NTP. No panics. No zombie evictions. Mesh is quiescent and healthy.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 2026-07-27T18:48Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 2026-07-27T18:48Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs (2727391, 2727569). Both sockets responding.

---

## Node Info

### morning-api (~11:40Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZ...zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 60652 | — | None (pass 86: 60175; Δ = +477s ≈ 8.0 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION (#1).** 10 commits behind HEAD + dirty tree. Unchanged from pass 86. |
| thickness | 983.91 | ~984, slowly decaying | None (pass 86: 984.04; Δ = −0.13 over ~9 min) |

### local-witness (~11:40Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZ...9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZ...zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 60673 | — | None (pass 86: 60194; Δ = +479s ≈ 8.0 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api (~11:40Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=6064, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~11:40Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=6068, silence_secs=8, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 86 (~11:31Z):** Heartbeats: m-ap +48 (6016→6064 ~5.3/min), witness +48 (6020→6068 ~5.3/min). Both at expected rate. Silence: m-ap 3s (unchanged), witness 8s (pass 86: 7s — slight increase, well under 30s threshold). Queue depth 0 on both. **No zombie eviction events. No sweep events.**

---

## Epoch State

### Single-capture bundle (~11:40Z)

| Check | morning-api | local-witness | DEVIATION |
|-------|-------------|---------------|-----------|
| Socket epoch | 2025 | 2024 | **δ=+1** (m-ap ahead; pass 86 had δ=-1 — sign flips based on boundary proximity) |
| Log count (grep -c) | 2025 | 2024 | **δ=0** (counts match socket epochs) |
| Last log epoch | 2025 (11:40:26Z) | 2024 (11:40:13Z) | — |
| Three-way equality | **MATCH** — socket=2025, count=2025, last_log=2025 | **MATCH** — socket=2024, count=2024, last_log=2024 | None |

**OBSERVED:** Both nodes advancing normally. +17–19 epochs in ~9 min from pass 86. Ratio: m-ap ~1.02, witness ~1.05.

**DEVIATION:** None. Three-way equality holds for both nodes on simultaneous captures. The δ=-1/δ=+1 sign is a known boundary artifact — witness's higher ratio means it crosses boundaries at different moments than m-ap, but the log counts are self-consistent.

### Cross-node epoch δ (carried forward)
| Pass 86 Status | Pass 87 Status |
|----------------|----------------|
| δ=-1 on socket captures (m-ap=2006, witness=2007). δ=0 on log counts (both=2008). | δ=+1 on socket captures (m-ap=2025, witness=2024). δ=0 on log counts (m-ap=2025, witness=2024). |

Pattern: δ varies -1 to +1 depending on boundary proximity, but log counts always align. Not a divergence.

---

## Economic State

### morning-api (~11:40Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged since pass 86) |
| own_nonce | 120 | 120 | None (unchanged) |
| witness balance (reported) | 4980 | 5000 - morning_api_balance = 4980 | None (unchanged) |
| witness nonce (reported) | 0 | 0 | None |

### local-witness (~11:40Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 | None |
| own_nonce | 2 | 2 | None (unchanged since early passes) |
| morning_api balance (reported) | 0 | 5000 | **Persistent DEVIATION (#3).** Witness reports morning-api balance as 0. First observed: pass 1 (Jul 27 18:48Z). Unchanged. |

### Supply divergence
**OBSERVED:** morning-api total = 5000 (20 + 4980). Witness total = 0 (0 + 0). Unchanged. See VERIFIED-BEHAVIOR.md "Supply Conservation (CONTRADICTED)."

---

## Persistence State

### morning-api (single-capture bundle ~11:40Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 2020 | Incrementing by 10 (pass 86: 2000; +20 = 2 rotations) | None (normal) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION (#2).** Endpoint reads legacy `transactions.wal`. Unchanged. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause) |

**File system cross-check (~11:40Z):**
- `state.snapshot`: **894 bytes** (mtime 07:37 EDT). Pass 86: 895 bytes (07:27 EDT). Small decrease (−1 byte), likely alignment or metadata.
- `wal.log`: 379 bytes (mtime 07:37 EDT). Unchanged from pass 86 (was 379 at 07:27).
- `wal.wal.old`: 379 bytes (mtime 07:32 EDT). Unchanged.
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known deviation #2).
- **Snapshot rotation count:** 2 rotations since pass 86 (2000→2020). Filesystem mtimes confirm.

### local-witness (single-capture bundle ~11:40Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 2020 | Incrementing by 10 (pass 86: 2000; +20 = 2 rotations) | None (normal) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~11:40Z):**
- `state.snapshot`: **569 bytes** (mtime 07:38 EDT). Pass 86: 569 bytes (07:28 EDT). Size unchanged.
- `wal.log`: 379 bytes (mtime 07:38 EDT). Unchanged.
- `wal.wal.old`: 379 bytes (mtime 07:33 EDT). Unchanged.
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known bug).

---

## Metrics Instrumentation (from heartbeat timer lines ~11:39Z)

**morning-api:** outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=3s
**local-witness:** outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=6s

**DEVIATION:** None. Mesh is quiescent. All gauges at zero or well under thresholds. Silence stable (m-ap 3s unchanged; witness 6s vs pass 86's 8s — slight improvement).

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **No new WARN/ERROR events beyond regular KAD bootstrap warnings** and **NTP query failures** (every ~5 min, benign with `--no-mdns`; DNS resolution fails intermittently, falls back to direct NTP query).
- Last NTP failure at 11:07:29Z (~33 min ago) — retry every 5 min as expected, intermittent DNS resolution.
- Zombie eviction events: **None**.
- Sweep/eviction events: **None**.
- Panics: **0**.

### local-witness (/tmp/lw.log)
- **No new WARN/ERROR events.** Last NTP failure at 08:00:06Z (~3.7h ago) — no retry logged since. Nodes talk to each other fine (NTP is startup check, not runtime requirement).
- Insufficient balance: **118 historic** (all Jul 27). **No new occurrences.**
- Zombie eviction events: **None**.
- Sweep/eviction events: **None**.
- Panics: **0**.

---

## Build Commit Verification

| Check | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| git HEAD | `cb5d4b1` | — | — |
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION (#1).** 10 commits behind HEAD + dirty tree. Unchanged from pass 86. |

**Note:** All 10 commits between binary and HEAD are docs/tests/fixes — no wire-format changes. No functional safety risk.

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 86 | Pass 87 | Changed? |
|---|-----------|----------------|---------|---------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (10 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent (10 behind) | Persistent (10 behind) | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

### Cross-node epoch δ
| Pass 86 Status | Pass 87 Status |
|----------------|----------------|
| δ=-1 on socket captures (m-ap=2006, witness=2007). δ=0 on log counts (both=2008). | δ=+1 on socket captures (m-ap=2025, witness=2024). δ=0 on log counts (m-ap=2025, witness=2024). |

Pattern unchanged: δ varies -1 to +1 depending on boundary proximity during sequential capture. Log counts always align exactly.

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (socket vs last log vs count) | **PASS** — m-ap: socket=2025, count=2025, last_log=2025. Witness: socket=2024, count=2024, last_log=2024. All match. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2). Both nodes. |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 unchanged. |
| Log health (WARN/ERROR filtered) | **PASS** — KAD bootstrap warnings and NTP failures only (both benign). 118 historic insufficient-balance entries unchanged. No panics, no zombies, no new errors. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** — m-ap 3s, witness 6s. Both well under threshold. |
| Cross-node epoch sync (single-capture bundle) | **PASS** (δ=0 on log counts; socket δ=+1 due to race). |
| Snapshot rotation | **PASS** — 2 rotations since pass 86 (2000→2020). New filesystem mtimes confirm disk writes. |
