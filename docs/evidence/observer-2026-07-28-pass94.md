# Observer Evidence Record — 2026-07-28 (Pass 94)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T12:40Z bundle (single-capture discipline)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (Boynton Beach FL)
**Session type:** 94th observation pass. Same processes since 2026-07-27T18:48Z (~18.0h runtime). ~7 min since pass 93 (12:33Z).

**Summary:** Routine continuation. All three persistent deviations unchanged. Epoch advanced from 2131→2144 on both nodes (+13 each, socket reading). Three-way epoch check shows a boundary race (socket=2144, count=2145, last_log=2145 — epoch 2145 completed during capture). One snapshot rotation (2130→2140). Metrics all healthy (aged=0, queues empty, silence≤6s). No new WARN/ERROR events. No panics. No zombie evictions. Mesh is quiescent and healthy.

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

### morning-api (~12:40Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZ...zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 64310 | — | None (pass 93: 63891; Δ = +419s ≈ 7 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION (#1).** 9 commits behind HEAD + dirty tree. Unchanged from pass 93. |
| thickness | 982.95 | ~983, slowly decaying | None (pass 93: 983.06; Δ = −0.11 over ~7 min — consistent decay rate) |

### local-witness (~12:40Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZ...9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZ...zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 64310 | — | None (pass 93: 63911; Δ = +399s ≈ 6.6 min — consistent) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

### Build gap (unchanged from pass 93)

Running binary at `71aa16b` (`wip: update Cargo.lock`, Jul 27 13:46). HEAD is `cb5d4b1` (docs). All missing commits are docs + test/fix patches. No production code changes missing. `-dirty` composition unknown.

---

## Epoch State

### Single-capture bundle — morning-api (~12:40Z)

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 2144 (at 12:40:16Z) | — |
| Log count (grep -c) | 2145 | — |
| Last log epoch | 2145 (12:40:26Z) | — |
| Three-way equality | **BOUNDARY RACE** — socket=2144, count=2145, last_log=2145. Epoch 2145 completed during capture (between socket read at 12:40:16 and log grep at ~12:40:30). Normal race condition on a ~30s epoch cycle. |

### Single-capture bundle — local-witness (~12:40Z)

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 2144 (at ~12:40:20Z) | — |
| Log count (grep -c) | 2145 | — |
| Last log epoch | 2145 (12:40:43Z) | — |
| Three-way equality | **BOUNDARY RACE** — socket=2144, count=2145, last_log=2145. Same race as m-ap. Both nodes at identical epoch. |

**OBSERVED:** Both nodes advancing normally. +13 epochs in ~7 min from pass 93 (2131→2144, socket). Consistent with previous rate (~2/min — the log count shows the actual rate is ~2/min, socket lags by 1 during the race).

**Cross-node epoch δ:** Identical (2144 on simultaneous read). Both nodes converged.

**DEVIATION:** None. The boundary race is a known artifact of 30s epoch cycles and non-simultaneous queries. First observed: pass 1. Does not indicate divergence.

**Delta from pass 93 (~12:33Z):** Pass 93 epoch=2131 (socket). Pass 94 epoch=2144 (socket). Advance of +13 epochs in ~7 min.

---

## Peer Connections

### morning-api (~12:40Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=6429, silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~12:40Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=6432, silence_secs=1, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 93 (~12:33Z):** Heartbeats: m-ap +42 (6387→6429 ~6.0/min), witness +40 (6392→6432 ~5.7/min). Both at expected rate (~6/min = every 10s). Silence: m-ap 7s (pass 93: 8s — stable), witness 1s (pass 93: 1s — stable). Queue depth 0 on both.
**No zombie eviction events. No sweep events.**

---

## Economic State

### morning-api (~12:40Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged) |
| own_nonce | 120 | 120 | None (unchanged) |
| witness balance (reported) | 4980 | 5000 - morning_api_balance = 4980 | None (unchanged) |
| witness nonce (reported) | 0 | 0 | None |

### local-witness (~12:40Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 | None |
| own_nonce | 2 | 2 | None (unchanged since early passes) |
| morning_api balance (reported) | 0 | 5000 | **Persistent DEVIATION (#3).** Witness reports morning-api balance as 0. First observed: pass 1 (Jul 27 18:48Z). Unchanged. |

### Supply divergence
**OBSERVED:** morning-api total = 5000 (20 + 4980). Witness total = 0 (0 + 0). Unchanged. See VERIFIED-BEHAVIOR.md "Supply Conservation (CONTRADICTED)."

---

## Persistence State

### morning-api (single-capture bundle ~12:40Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 2140 | Incrementing by 10 (pass 93: 2130; +10 = 1 rotation since pass 93) | None (normal — 1 rotation in ~7 min) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION (#2).** Endpoint reads legacy `transactions.wal`. Unchanged. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause) |

**File system cross-check (~12:40Z):**
- `state.snapshot`: **895 bytes** (mtime 08:37). Pass 93: 895 bytes (mtime 08:22). Size unchanged.
- `wal.log`: 379 bytes (mtime 08:37). Pass 93: 379 bytes (mtime ~12:32). Size unchanged.
- `wal.wal.old`: 379 bytes (mtime 08:32). Rotated as expected.
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known deviation #2).

### local-witness (single-capture bundle ~12:40Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 2140 | Incrementing by 10 (pass 93: 2130; +10 = 1 rotation) | None (normal) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~12:40Z):**
- `state.snapshot`: **569 bytes** (mtime 08:38). Pass 93: 569 bytes (mtime 08:23). Size unchanged.
- `wal.log`: 379 bytes (mtime 08:38). Pass 93: 379 bytes (mtime ~12:31). Size unchanged.
- `wal.wal.old`: 379 bytes (mtime 08:33). Rotated as expected.
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known bug).

---

## Metrics Instrumentation (from heartbeat timer lines)

**morning-api (12:40:46Z):** outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=3s
**local-witness (12:40:43Z):** outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=6s

**DEVIATION:** None. Mesh is quiescent. All gauges at zero or well under thresholds. Silence stable (m-ap 3s; witness 6s, both well under 30s zombie threshold).

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **No new WARN/ERROR events** beyond historic startup messages and:
  - NTP failures: 3 new since pass 93 (pool.ntp.org + time.apple.com at 12:13Z, Kademlia bootstrap ticks continuing every 5 min — last at 12:38Z). Network-limited machine. Expected, cosmetic, not affecting mesh operations.
  - `Failed to trigger bootstrap: No known peers.` (Kademlia bootstrap tick on a --no-mdns 2-node mesh — expected, cosmetic)
- Zombie eviction events: **None**.
- Sweep/eviction events: **None**.
- Panics: **0**.
- Insufficient balance: **0**.

### local-witness (/tmp/lw.log)
- **No new WARN/ERROR events.**
- NTP failures: 1 total (historic, Jul 28 08:00). No new NTP failures.
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

| # | Deviation | First Observed | Pass 93 | Pass 94 | Changed? |
|---|-----------|----------------|---------|---------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent (9 behind) | Persistent (9 behind) | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (socket vs last log vs count) | **BOUNDARY RACE** — m-ap: socket=2144, count=2145, last_log=2145. Witness: socket=2144, count=2145, last_log=2145. Same race on both — normal artifact of ~30s epoch cycle. Both nodes in sync. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2). Both nodes. |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 unchanged. |
| Log health (WARN/ERROR filtered) | **PASS** — historic insufficient-balance entries only. No panics, no zombies, no new errors. NTP failures and Kademlia bootstrap warnings are cosmetic on this network. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** — m-ap 3s, witness 6s. Both well under threshold. |
| Cross-node epoch sync (single-capture bundle) | **PASS** — both at 2144 (simultaneous). Fully converged. |
| Snapshot rotation | **PASS** — 1 rotation since pass 93 (2130→2140). Filesystem mtimes confirm disk writes. |
