# Observer Evidence Record — 2026-07-28 (Pass 88)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T11:48Z bundle (single-capture discipline)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (Boynton Beach FL)
**Session type:** Eighty-eighth observation pass. Same processes since 2026-07-27T18:48Z (~17.0h runtime). ~8 min since pass 87 (11:40Z).

**Summary:** Routine continuation. All three persistent deviations unchanged. Epoch advanced from 2025→2040 on morning-api (+15), 2024→2040 on witness (+16). Three-way epoch equality holds on both nodes. Snapshot rotated twice (2020→2040). Metrics all healthy (aged=0, queues empty, silence<10s). No new WARN/ERROR events beyond benign KAD. No panics. No zombie evictions. Mesh is quiescent and healthy.

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

### morning-api (~11:48Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZ...zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 61180 | — | None (pass 87: 60652; Δ = +528s ≈ 8.8 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION (#1).** 10 commits behind HEAD + dirty tree. Unchanged from pass 87. |
| thickness | 983.77 | ~984, slowly decaying | None (pass 87: 983.91; Δ = −0.14 over ~8 min) |

### local-witness (~11:48Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZ...9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZ...zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 61175 | — | None (pass 87: 60673; Δ = +502s ≈ 8.4 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api (~11:48Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=6116, silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~11:48Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=6118, silence_secs=6, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 87 (~11:40Z):** Heartbeats: m-ap +52 (6064→6116 ~6.5/min), witness +50 (6068→6118 ~6.3/min). Both at expected rate. Silence: m-ap 7s (pass 87: 3s — slight increase, well under 30s), witness 6s (pass 87: 8s — slight improvement). Queue depth 0 on both. **No zombie eviction events. No sweep events.**

---

## Epoch State

### Single-capture bundle (~11:48Z)

| Check | morning-api | local-witness | DEVIATION |
|-------|-------------|---------------|-----------|
| Socket epoch | 2040 | 2040 | **δ=0** (both matched at this capture boundary) |
| Log count (grep -c) | 2040 | 2040 | **δ=0** (counts match socket epochs) |
| Last log epoch | 2040 (11:47:56Z) | 2040 (11:48:13Z) | — |
| Three-way equality | **MATCH** — socket=2040, count=2040, last_log=2040 | **MATCH** — socket=2040, count=2040, last_log=2040 | None |

**OBSERVED:** Both nodes advancing normally. +15–16 epochs in ~8 min from pass 87. Ratio: m-ap ~1.02, witness ~1.05.

**DEVIATION:** None. Three-way equality holds on both nodes. Socket epoch δ = 0 (both at 2040 simultaneously — witness caught up from δ=-1 in pass 87 to δ=0 here).

### Cross-node epoch δ

| Pass 87 Status | Pass 88 Status |
|----------------|----------------|
| δ=+1 on socket captures (m-ap=2025, witness=2024). δ=0 on log counts (m-ap=2025, witness=2024). | δ=0 on socket captures (both=2040). δ=0 on log counts (both=2040). |

Pattern: δ varies -1 to +1 to 0 depending on boundary proximity. Log counts always align exactly. Normal behavior.

---

## Economic State

### morning-api (~11:48Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged) |
| own_nonce | 120 | 120 | None (unchanged) |
| witness balance (reported) | 4980 | 5000 - morning_api_balance = 4980 | None (unchanged) |
| witness nonce (reported) | 0 | 0 | None |

### local-witness (~11:48Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 | None |
| own_nonce | 2 | 2 | None (unchanged since early passes) |
| morning_api balance (reported) | 0 | 5000 | **Persistent DEVIATION (#3).** Witness reports morning-api balance as 0. First observed: pass 1 (Jul 27 18:48Z). Unchanged. |

### Supply divergence
**OBSERVED:** morning-api total = 5000 (20 + 4980). Witness total = 0 (0 + 0). Unchanged. See VERIFIED-BEHAVIOR.md "Supply Conservation (CONTRADICTED)."

---

## Persistence State

### morning-api (single-capture bundle ~11:48Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 2040 | Incrementing by 20 (pass 87: 2020; +20 = 2 rotations) | None (normal) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION (#2).** Endpoint reads legacy `transactions.wal`. Unchanged. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause) |

**File system cross-check (~11:48Z):**
- `state.snapshot`: **894 bytes** (mtime 07:47 EDT). Pass 87: 894 bytes (07:37 EDT). Size unchanged.
- `wal.log`: 379 bytes (mtime 07:47 EDT). Pass 87: 379 bytes (07:37 EDT). Unchanged.
- `wal.wal.old`: 379 bytes (mtime 07:42 EDT). Pass 87: 379 bytes (07:32 EDT). Unchanged (old version just got overwritten at 07:42).
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known deviation #2).
- **Snapshot rotation count:** 2 rotations since pass 87 (2020→2040). Filesystem mtimes confirm.

### local-witness (single-capture bundle ~11:48Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 2040 | Incrementing by 20 (pass 87: 2020; +20 = 2 rotations) | None (normal) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~11:48Z):**
- `state.snapshot`: **569 bytes** (mtime 07:48 EDT). Pass 87: 569 bytes (07:38 EDT). Size unchanged.
- `wal.log`: 379 bytes (mtime 07:48 EDT). Unchanged.
- `wal.wal.old`: 379 bytes (mtime 07:43 EDT). Unchanged.
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known bug).

---

## Metrics Instrumentation (from heartbeat timer lines ~11:48Z)

**morning-api:** outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=3s
**local-witness:** outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=6s

**DEVIATION:** None. Mesh is quiescent. All gauges at zero or well under thresholds. Silence stable (m-ap 3s; witness 6s, both well under 30s threshold).

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **No new WARN/ERROR events beyond regular KAD bootstrap warnings** (every 5 min given `--no-mdns`).
- Last NTP failure at 11:07:29Z (~41 min ago) — intermittent DNS resolution pattern as expected.
- Zombie eviction events: **None**.
- Sweep/eviction events: **None**.
- Panics: **0**.

### local-witness (/tmp/lw.log)
- **No new WARN/ERROR events.** Last NTP failure at 08:00:06Z (~3.8h ago) — no retry logged since.
- Insufficient balance: **118 historic** (all Jul 27). **No new occurrences.**
- Zombie eviction events: **None**.
- Sweep/eviction events: **None**.
- Panics: **0**.

---

## Build Commit Verification

| Check | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| git HEAD | `cb5d4b1` | — | — |
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION (#1).** 10 commits behind HEAD + dirty tree. Unchanged from pass 87. |

**Note:** All 10 commits between binary and HEAD are docs/tests/fixes — no wire-format changes. No functional safety risk.

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 87 | Pass 88 | Changed? |
|---|-----------|----------------|---------|---------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (10 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent (10 behind) | Persistent (10 behind) | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

### Cross-node epoch δ

| Pass 87 Status | Pass 88 Status |
|----------------|----------------|
| δ=+1 on socket captures (m-ap=2025, witness=2024). δ=0 on log counts (m-ap=2025, witness=2024). | δ=0 on socket captures (both=2040). δ=0 on log counts (both=2040). |

Pattern unchanged: δ varies -1 to +1 to 0 depending on boundary proximity during sequential capture. Log counts always align exactly.

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (socket vs last log vs count) | **PASS** — m-ap: socket=2040, count=2040, last_log=2040. Witness: socket=2040, count=2040, last_log=2040. All match. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2). Both nodes. |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 unchanged. |
| Log health (WARN/ERROR filtered) | **PASS** — KAD bootstrap warnings only (benign). 118 historic insufficient-balance entries unchanged. No panics, no zombies, no new errors. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** — m-ap 3s, witness 6s. Both well under threshold. |
| Cross-node epoch sync (single-capture bundle) | **PASS** (δ=0 on socket and log counts). |
| Snapshot rotation | **PASS** — 2 rotations since pass 87 (2020→2040). New filesystem mtimes confirm disk writes. |
