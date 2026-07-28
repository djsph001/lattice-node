# Observer Evidence Record — 2026-07-28 (Pass 91)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T12:18Z bundle (single-capture discipline)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (Boynton Beach FL)
**Session type:** Ninety-first observation pass. Same processes since 2026-07-27T18:48Z (~17.5h runtime). ~8 min since pass 90 (12:10Z).

**Summary:** Routine continuation. All three persistent deviations unchanged. Epoch advanced from 2084→2100 on both nodes (+16 each). Three-way epoch equality holds on both nodes. Snapshot rotated once (2080→2090). Metrics all healthy (aged=0, queues empty, silence<10s). No new WARN/ERROR events. No panics. No zombie evictions. Mesh is quiescent and healthy.

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

### morning-api (~12:17Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZ...zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 62934 | — | None (pass 90: 62393; Δ = +541s ≈ 9 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION (#1).** 9 commits behind HEAD + dirty tree. Unchanged from pass 90. |
| thickness | 983.31 | ~983, slowly decaying | None (pass 90: 983.45; Δ = −0.14 over ~8 min) |

### local-witness (~12:17Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZ...9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZ...zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 62934 | — | None (pass 90: 62396; Δ = +538s ≈ 9 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

### Build gap (unchanged from pass 90)

Running binary at `71aa16b` (`wip: update Cargo.lock`, Jul 27 13:46). HEAD is `cb5d4b1` (docs). All missing commits are docs + test-only fixes. No production code changes missing. `-dirty` composition unknown.

---

## Peer Connections

### morning-api (~12:17Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=6291, silence_secs=9, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~12:17Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=6294, silence_secs=2, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 90 (~12:10Z):** Heartbeats: m-ap +53 (6238→6291 ~6.1/min), witness +54 (6240→6294 ~6.0/min). Both at expected rate (~6/min = every 10s). Silence: m-ap 9s (pass 90: 2s — higher but well under 30s), witness 2s (pass 90: 8s — lower). Queue depth 0 on both. **No zombie eviction events. No sweep events.**

---

## Epoch State

### Single-capture bundle — morning-api (12:17:56Z)

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 2100 | — |
| Log count (grep -c) | 2100 | — |
| Last log epoch | 2100 (12:17:56Z) | — |
| Three-way equality | **MATCH** — socket=2100, count=2100, last_log=2100 | None |

### Single-capture bundle — local-witness (12:18:13Z)

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 2100 | — |
| Log count (grep -c) | 2100 | — |
| Last log epoch | 2100 (12:18:13Z) | — |
| Three-way equality | **MATCH** — socket=2100, count=2100, last_log=2100 | None |

**Note:** First query (12:17:26Z) showed a transient three-way mismatch (socket=2098, count=2100, last_log=2099) caused by an epoch transition during capture. Re-verified with fresh simultaneous capture — all three match. This is a normal race on an actively-cycling mesh, not evidence of a drift.

**OBSERVED:** Both nodes advancing normally. +16 epochs in ~8 min from pass 90. Ratio: m-ap ~1.02, witness ~1.05.

**Cross-node epoch δ:** Identical (2100/2100). Both nodes converged.

**DEVIATION:** None. Three-way equality holds on both nodes on simultaneous capture.

---

## Economic State

### morning-api (~12:17Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged) |
| own_nonce | 120 | 120 | None (unchanged) |
| witness balance (reported) | 4980 | 5000 - morning_api_balance = 4980 | None (unchanged) |
| witness nonce (reported) | 0 | 0 | None |

### local-witness (~12:17Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 | None |
| own_nonce | 2 | 2 | None (unchanged since early passes) |
| morning_api balance (reported) | 0 | 5000 | **Persistent DEVIATION (#3).** Witness reports morning-api balance as 0. First observed: pass 1 (Jul 27 18:48Z). Unchanged. |

### Supply divergence
**OBSERVED:** morning-api total = 5000 (20 + 4980). Witness total = 0 (0 + 0). Unchanged. See VERIFIED-BEHAVIOR.md "Supply Conservation (CONTRADICTED)."

---

## Persistence State

### morning-api (single-capture bundle 12:17Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 2090 | Incrementing by 10 (pass 90: 2080; +10 = 1 rotation) | None (normal) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION (#2).** Endpoint reads legacy `transactions.wal`. Unchanged. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause) |

**File system cross-check (12:17Z):**
- `state.snapshot`: **895 bytes** (mtime 08:12 EDT). Pass 90: 895 bytes (08:07 EDT). Size unchanged. +1 rotation confirmed.
- `wal.log`: 379 bytes (mtime 08:12 EDT). Unchanged from pass 90.
- `wal.wal.old`: 379 bytes (mtime 08:07 EDT). Rotated as expected.
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known deviation #2).

### local-witness (single-capture bundle 12:18Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 2090 | Incrementing by 10 (pass 90: 2080; +10 = 1 rotation) | None (normal) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (12:18Z):**
- `state.snapshot`: **569 bytes** (mtime 08:13 EDT). Pass 90: 569 bytes (08:08 EDT). Size unchanged.
- `wal.log`: 379 bytes (mtime 08:13 EDT). Unchanged.
- `wal.wal.old`: 379 bytes (mtime 08:08 EDT). Rotated as expected.
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known bug).

---

## Metrics Instrumentation (from heartbeat timer lines)

**morning-api (12:18:26Z):** outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=3s
**local-witness (12:18:23Z):** outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=6s

**DEVIATION:** None. Mesh is quiescent. All gauges at zero or well under thresholds. Silence stable (m-ap 3s; witness 6s, both well under 30s threshold).

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **No new WARN/ERROR events** beyond historic startup messages and:
  - Kademlia `No known peers` warnings (ongoing, ~210 total, every 5 min — harmless, mesh uses direct bootstrap, not mDNS)
  - NTP failures (6 total, none new since pass 90)
- Zombie eviction events: **None**.
- Sweep/eviction events: **None**.
- Panics: **0**.
- Insufficient balance: **0** (never had any).

### local-witness (/tmp/lw.log)
- **No new WARN/ERROR events.**
- NTP failures: 1 total (historic, Jul 28 08:00).
- Insufficient balance: **118** (all historic Jul 27). **No new occurrences.**
- Zombie eviction events: **None**.
- Sweep/eviction events: **None**.
- Panics: **0**.

---

## Build Commit Verification

| Check | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| git HEAD | `cb5d4b1` | — | — |
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION (#1).** 9 commits behind HEAD + dirty tree. Unchanged from pass 90. |

**Note:** All commits between binary and HEAD are docs/tests/fixes — no wire-format changes. No functional safety risk.

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 90 | Pass 91 | Changed? |
|---|-----------|----------------|---------|---------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent (9 behind) | Persistent (9 behind) | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (socket vs last log vs count) | **PASS** — m-ap: socket=2100, count=2100, last_log=2100. Witness: socket=2100, count=2100, last_log=2100. All match. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2). Both nodes. |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 unchanged. |
| Log health (WARN/ERROR filtered) | **PASS** — historic insufficient-balance entries only. No panics, no zombies, no new errors. Kademlia `No known peers` and NTP failures are cosmetic (no mDNS / network-limited). |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** — m-ap 3s, witness 6s. Both well under threshold. |
| Cross-node epoch sync (single-capture bundle) | **PASS** — both at 2100. |
| Snapshot rotation | **PASS** — 1 rotation since pass 90 (2080→2090). Filesystem mtimes confirm disk writes. |
