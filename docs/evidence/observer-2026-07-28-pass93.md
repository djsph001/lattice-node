# Observer Evidence Record — 2026-07-28 (Pass 93)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T12:33Z bundle (single-capture discipline)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (Boynton Beach FL)
**Session type:** 93rd observation pass. Same processes since 2026-07-27T18:48Z (~17.7h runtime). ~7 min since pass 92 (12:26Z).

**Summary:** Routine continuation. All three persistent deviations unchanged. Epoch advanced from 2115→2131 on both nodes (+16 each). Three-way epoch match is CLEAN this pass — no race (socket=2131, count=2131, last_log=2131 on both). Snapshot rotated once (2110→2130). Metrics all healthy (aged=0, queues empty, silence<10s). No new WARN/ERROR events. No panics. No zombie evictions. Mesh is quiescent and healthy.

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

### morning-api (~12:33Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZ...zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 63891 | — | None (pass 92: 63440; Δ = +451s ≈ 7.5 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION (#1).** 9 commits behind HEAD + dirty tree. Unchanged from pass 92. |
| thickness | 983.06 | ~983, slowly decaying | None (pass 92: 983.18; Δ = −0.12 over ~7 min — consistent decay rate) |

### local-witness (~12:33Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZ...9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZ...zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 63911 | — | None (pass 92: 63440; Δ = +471s ≈ 7.8 min — consistent) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

### Build gap (unchanged from pass 92)

Running binary at `71aa16b` (`wip: update Cargo.lock`, Jul 27 13:46). HEAD is `cb5d4b1` (docs). All missing commits are docs + test/fix patches. No production code changes missing. `-dirty` composition unknown.

---

## Epoch State

### Single-capture bundle — morning-api (~12:33Z)

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 2131 (at 12:33:40Z) | — |
| Log count (grep -c) | 2131 | — |
| Last log epoch | 2131 (12:33:26Z) | — |
| Three-way equality | **CLEAN** — socket=2131, count=2131, last_log=2131. All match. |

### Single-capture bundle — local-witness (~12:33Z)

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 2131 (at 12:33:45Z) | — |
| Log count (grep -c) | 2131 | — |
| Last log epoch | 2131 (12:33:43Z) | — |
| Three-way equality | **CLEAN** — socket=2131, count=2131, last_log=2131. All match. |

**OBSERVED:** Both nodes advancing normally. +16 epochs in ~7 min from pass 92 (2115→2131). Consistent with previous rate (~2/min).

**Cross-node epoch δ:** Identical (2131 on simultaneous read). Both nodes converged.

**DEVIATION:** None. Three-way match is CLEAN this pass — the epoch boundary race documented in pass 92 resolved normally.

**Delta from pass 92 (~12:26Z):** Pass 92 epoch=2115. Pass 93 epoch=2131. Advance of +16 epochs in ~7 min.

---

## Peer Connections

### morning-api (~12:33Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=6387, silence_secs=8, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~12:33Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=6392, silence_secs=1, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 92 (~12:26Z):** Heartbeats: m-ap +44 (6343→6387 ~6.3/min), witness +47 (6345→6392 ~6.7/min). Both at expected rate (~6/min = every 10s). Silence: m-ap 8s (pass 92: 3s — slight increase but well under 30s threshold), witness 1s (pass 92: 3s — stable). Queue depth 0 on both.
**No zombie eviction events. No sweep events.**

---

## Economic State

### morning-api (~12:33Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged) |
| own_nonce | 120 | 120 | None (unchanged) |
| witness balance (reported) | 4980 | 5000 - morning_api_balance = 4980 | None (unchanged) |
| witness nonce (reported) | 0 | 0 | None |

### local-witness (~12:33Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 | None |
| own_nonce | 2 | 2 | None (unchanged since early passes) |
| morning_api balance (reported) | 0 | 5000 | **Persistent DEVIATION (#3).** Witness reports morning-api balance as 0. First observed: pass 1 (Jul 27 18:48Z). Unchanged. |

### Supply divergence
**OBSERVED:** morning-api total = 5000 (20 + 4980). Witness total = 0 (0 + 0). Unchanged. See VERIFIED-BEHAVIOR.md "Supply Conservation (CONTRADICTED)."

---

## Persistence State

### morning-api (single-capture bundle ~12:33Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 2130 | Incrementing by 10 (pass 92: 2110; +20 = 2 rotations since pass 92) | None (normal — 2 rotations in ~7 min) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION (#2).** Endpoint reads legacy `transactions.wal`. Unchanged. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause) |

**File system cross-check (~12:33Z):**
- `state.snapshot`: **895 bytes** (mtime 12:32Z). Pass 92: 895 bytes (08:22 EDT). Size unchanged.
- `wal.log`: 379 bytes (mtime 12:32Z). Unchanged size from pass 92.
- `wal.wal.old`: 379 bytes (mtime 12:27Z). Rotated as expected.
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known deviation #2).

### local-witness (single-capture bundle ~12:33Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 2130 | Incrementing by 10 (pass 92: 2110; +20 = 2 rotations) | None (normal) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~12:33Z):**
- `state.snapshot`: **569 bytes** (mtime 12:31Z). Pass 92: 569 bytes (08:23 EDT). Size unchanged.
- `wal.log`: 379 bytes (mtime 12:31Z). Unchanged.
- `wal.wal.old`: 379 bytes (mtime 12:28Z). Rotated as expected.
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known bug).

---

## Metrics Instrumentation (from heartbeat timer lines)

**morning-api (12:33:36Z):** outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=3s
**local-witness (12:33:43Z):** outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=6s

**DEVIATION:** None. Mesh is quiescent. All gauges at zero or well under thresholds. Silence stable (m-ap 3s; witness 6s, both well under 30s zombie threshold).

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **No new WARN/ERROR events** beyond historic startup messages and:
  - NTP failures (10 total, all historic — most recent at 12:13Z. Network-limited machine, not affecting mesh operations.)
  - `Failed to trigger bootstrap: No known peers.` (Kademlia bootstrap tick on a --no-mdns 2-node mesh — expected, cosmetic)
- Zombie eviction events: **None**.
- Sweep/eviction events: **None**.
- Panics: **0**.
- Insufficient balance: **0**.

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

| # | Deviation | First Observed | Pass 92 | Pass 93 | Changed? |
|---|-----------|----------------|---------|---------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent (9 behind) | Persistent (9 behind) | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (socket vs last log vs count) | **CLEAN** — m-ap: socket=2131, count=2131, last_log=2131. Witness: socket=2131, count=2131, last_log=2131. All match on both nodes. No race this pass. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2). Both nodes. |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 unchanged. |
| Log health (WARN/ERROR filtered) | **PASS** — historic insufficient-balance entries only. No panics, no zombies, no new errors. NTP failures and Kademlia bootstrap warnings are cosmetic on this network. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** — m-ap 3s, witness 6s. Both well under threshold. |
| Cross-node epoch sync (single-capture bundle) | **PASS** — both at 2131 (simultaneous). Fully converged. |
| Snapshot rotation | **PASS** — 2 rotations since pass 92 (2110→2120→2130). Filesystem mtimes confirm disk writes. |
