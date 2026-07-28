# Observer Evidence Record — 2026-07-28 (Pass 92)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T12:26Z bundle (single-capture discipline, with noted race on epoch)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (Boynton Beach FL)
**Session type:** 92nd observation pass. Same processes since 2026-07-27T18:48Z (~17.6h runtime). ~8 min since pass 91 (12:18Z).

**Summary:** Routine continuation. All three persistent deviations unchanged. Epoch advanced from 2100→2115/2116 on both nodes (+15-16 each). Normal epoch transition race during capture (three-way off by 1 — consistent with ~30s cycle timing). Snapshot rotated twice since pass 91 (2090→2100→2110). Metrics all healthy (aged=0, queues empty, silence<10s). No new WARN/ERROR events. No panics. No zombie evictions. Mesh is quiescent and healthy.

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

### morning-api (~12:25Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZ...zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 63440 | — | None (pass 91: 62934; Δ = +506s ≈ 8.4 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION (#1).** 9 commits behind HEAD + dirty tree. Unchanged from pass 91. |
| thickness | 983.18 | ~983, slowly decaying | None (pass 91: 983.31; Δ = −0.13 over ~8 min — consistent decay rate) |

### local-witness (~12:26Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZ...9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZ...zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 63440 | — | None (pass 91: 62934; Δ = +506s ≈ 8.4 min — matches m-ap) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

### Build gap (unchanged from pass 91)

Running binary at `71aa16b` (`wip: update Cargo.lock`, Jul 27 13:46). HEAD is `cb5d4b1` (docs). All missing commits are docs + test-only fixes. No production code changes missing. `-dirty` composition unknown.

---

## Epoch State

### Single-capture bundle — morning-api (~12:25-26Z)

Note: Socket query returned epoch=2115 at 12:25:47Z; by 12:25:56Z log shows epoch=2116. This is a normal epoch transition race within the capture window. See pass 91 for identical pattern.

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 2115 (at 12:25:47Z) | — |
| Log count (grep -c) | 2116 | — |
| Last log epoch | 2116 (12:25:56Z) | — |
| Three-way equality | **RACE** — socket=2115, count=2116, last_log=2116. Off by 1 due to epoch transition during capture. Normal on ~30s cycle. |

### Single-capture bundle — local-witness (~12:26Z)

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 2115 (at 12:26:03Z) | — |
| Log count (grep -c) | 2116 | — |
| Last log epoch | 2116 (12:26:13Z) | — |
| Three-way equality | **RACE** — same pattern as m-ap. Normal epoch transition. |

**OBSERVED:** Both nodes advancing normally. +15-16 epochs in ~8 min from pass 91 (2100→2115/2116). Consistent with previous rate (~2/min).

**Cross-node epoch δ:** Identical (2115/2116 on simultaneous read, accounting for race). Both nodes converged.

**DEVIATION:** None. Three-way race is a normal capture artifact on an actively cycling mesh (documented in pass 91).

**Delta from pass 91:** Pass 91 epoch=2100 at ~12:17Z. Pass 92 epoch=(2115→2116) at ~12:25Z. Advance of +15-16 epochs in ~8 min.

---

## Peer Connections

### morning-api (~12:25Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=6343, silence_secs=0, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~12:26Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=6345, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 91 (~12:18Z):** Heartbeats: m-ap +52 (6291→6343 ~6.2/min), witness +51 (6294→6345 ~6.1/min). Both at expected rate (~6/min = every 10s). Silence: m-ap 0s (pass 91: 9s — dropped), witness 3s (pass 91: 2s — approximately stable). Queue depth 0 on both.
**No zombie eviction events. No sweep events.**

---

## Economic State

### morning-api (~12:25Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged) |
| own_nonce | 120 | 120 | None (unchanged) |
| witness balance (reported) | 4980 | 5000 - morning_api_balance = 4980 | None (unchanged) |
| witness nonce (reported) | 0 | 0 | None |

### local-witness (~12:26Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 | None |
| own_nonce | 2 | 2 | None (unchanged since early passes) |
| morning_api balance (reported) | 0 | 5000 | **Persistent DEVIATION (#3).** Witness reports morning-api balance as 0. First observed: pass 1 (Jul 27 18:48Z). Unchanged. |

### Supply divergence
**OBSERVED:** morning-api total = 5000 (20 + 4980). Witness total = 0 (0 + 0). Unchanged. See VERIFIED-BEHAVIOR.md "Supply Conservation (CONTRADICTED)."

---

## Persistence State

### morning-api (single-capture bundle ~12:25Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 2110 | Incrementing by 10 (pass 91: 2090; +20 = 2 rotations since pass 91) | None (normal — 2 rotations in ~8 min) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION (#2).** Endpoint reads legacy `transactions.wal`. Unchanged. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause) |

**File system cross-check (~12:25Z):**
- `state.snapshot`: **895 bytes** (mtime 08:22 EDT). Pass 91: 895 bytes (08:12 EDT). Size unchanged. +2 rotations confirmed.
- `wal.log`: 379 bytes (mtime 08:22 EDT). Unchanged size from pass 91.
- `wal.wal.old`: 379 bytes (mtime 08:17 EDT). Rotated as expected.
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known deviation #2).

### local-witness (single-capture bundle ~12:26Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 2110 | Incrementing by 10 (pass 91: 2090; +20 = 2 rotations) | None (normal) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~12:26Z):**
- `state.snapshot`: **569 bytes** (mtime 08:23 EDT). Pass 91: 569 bytes (08:13 EDT). Size unchanged.
- `wal.log`: 379 bytes (mtime 08:23 EDT). Unchanged.
- `wal.wal.old`: 379 bytes (mtime 08:18 EDT). Rotated as expected.
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known bug).

---

## Metrics Instrumentation (from heartbeat timer lines)

**morning-api (12:26:16Z):** outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=3s
**local-witness (12:26:23Z):** outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=6s

**DEVIATION:** None. Mesh is quiescent. All gauges at zero or well under thresholds. Silence stable (m-ap 3s; witness 6s, both well under 30s zombie threshold).

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **No new WARN/ERROR events** beyond historic startup messages and:
  - NTP failures (continuing, ~11 total — most recent at 12:13Z. Network-limited machine, not affecting mesh operations.)
- Zombie eviction events: **None**.
- Sweep/eviction events: **None**.
- Panics: **0**.
- Insufficient balance: **0** (never had any).

### local-witness (/tmp/lw.log)
- **No new WARN/ERROR events.**
- NTP failures: 1 total (historic, Jul 28 08:00).
- Insufficient balance: **118** (ALL historic Jul 27). **No new occurrences.**
- Zombie eviction events: **None**.
- Sweep/eviction events: **None**.
- Panics: **0**.

---

## Build Commit Verification

| Check | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| git HEAD | `cb5d4b1` | — | — |
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION (#1).** 9 commits behind HEAD + dirty tree. Unchanged from all previous passes. |

**Note:** All commits between binary and HEAD are docs/tests/fixes — no wire-format changes. No functional safety risk.

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 91 | Pass 92 | Changed? |
|---|-----------|----------------|---------|---------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent (9 behind) | Persistent (9 behind) | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (socket vs last log vs count) | **RACE** — m-ap: socket=2115, count=2116, last_log=2116. Witness: socket=2115, count=2116, last_log=2116. Both off by 1 due to epoch transition during capture. Normal on ~30s epoch cycle. Documented same pattern in pass 91. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2). Both nodes. |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 unchanged. |
| Log health (WARN/ERROR filtered) | **PASS** — historic insufficient-balance entries only. No panics, no zombies, no new errors. NTP failures are cosmetic (network-limited machine, not affecting mesh). |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** — m-ap 3s, witness 6s. Both well under threshold. |
| Cross-node epoch sync (single-capture bundle) | **PASS** — both at 2115 (socket, simultaneous) / 2116 (log, sequential). Converged. |
| Snapshot rotation | **PASS** — 2 rotations since pass 91 (2090→2100→2110). Filesystem mtimes confirm disk writes. |
