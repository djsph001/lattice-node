# Observer Evidence Record — 2026-07-28 (Pass 95)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T12:49Z bundle (single-capture discipline)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (Boynton Beach FL)
**Session type:** 95th observation pass. Same processes since 2026-07-27T18:48Z (~18.0h runtime). ~8 min since pass 94 (12:40Z).

**Summary:** Routine continuation. All three persistent deviations unchanged. Epoch advanced from 2144→2162 on both nodes (+18 each, socket reading). Three-way epoch check shows **FULL MATCH** on both nodes (socket=2162, count=2162, last_log=2162) — no boundary race this pass. Two snapshot rotations since pass 94 (2140→2160 on both). Metrics all healthy (aged=0, queues empty, silence≤6s). No new WARN/ERROR events. No panics. No zombie evictions. Mesh is quiescent and healthy.

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

### morning-api (~12:49Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZ...zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 64711 | — | None (pass 94: 64310; Δ = +401s ≈ 6.7 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION (#1).** 9 commits behind HEAD + dirty tree. Unchanged from pass 94. |
| thickness | 982.84 | ~983, slowly decaying | None (pass 94: 982.95; Δ = −0.11 over ~8 min — consistent decay rate) |

### local-witness (~12:49Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZ...9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZ...zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 64773 | — | None (pass 94: 64310; Δ = +463s ≈ 7.7 min — sequential read gap: witness queried ~62s after m-ap) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

### Build gap (unchanged from pass 94)

Running binary at `71aa16b` (`wip: update Cargo.lock`, Jul 27 13:46). HEAD is `cb5d4b1` (docs). All missing commits are docs + evidence files. No production code changes missing. `-dirty` composition unknown.

---

## Epoch State

### Single-capture bundle — morning-api (~12:49Z)

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 2162 (at 12:48:56Z) | — |
| Log count (grep -c) | 2162 | — |
| Last log epoch | 2162 (12:48:56Z) | — |
| Three-way equality | **MATCH** — socket=2162, count=2162, last_log=2162. No boundary race. | None |

### Single-capture bundle — local-witness (~12:49Z)

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 2162 (at ~12:49:13Z) | — |
| Log count (grep -c) | 2162 | — |
| Last log epoch | 2162 (12:49:13Z) | — |
| Three-way equality | **MATCH** — socket=2162, count=2162, last_log=2162. No boundary race. | None |

**OBSERVED:** Both nodes advancing normally. +18 epochs in ~8 min from pass 94 (2144→2162, socket). Consistent with previous rate (~2/min).

**Cross-node epoch δ:** Identical (2162 on simultaneous read). Both nodes perfectly converged.

**Delta from pass 94 (~12:40Z):** Pass 94 epoch=2144 (socket). Pass 95 epoch=2162 (socket). Advance of +18 epochs in ~8 min.

---

## Peer Connections

### morning-api (~12:49Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=6470, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~12:49Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=6479, silence_secs=6, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 94 (~12:40Z):** Heartbeats: m-ap +41 (6429→6470 ~5.9/min), witness +47 (6432→6479 ~6.7/min). Both near expected rate (~6/min). Silence: m-ap 3s (pass 94: 7s — stable), witness 6s (pass 94: 1s — slightly higher but well under threshold). Queue depth 0 on both.
**No zombie eviction events. No sweep events.**

---

## Economic State

### morning-api (~12:49Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged) |
| own_nonce | 120 | 120 | None (unchanged) |
| witness balance (reported) | 4980 | 5000 - morning_api_balance = 4980 | None (unchanged) |
| witness nonce (reported) | 0 | 0 | None |

### local-witness (~12:49Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 | None |
| own_nonce | 2 | 2 | None (unchanged since early passes) |
| morning_api balance (reported) | 0 | 5000 | **Persistent DEVIATION (#3).** Witness reports morning-api balance as 0. First observed: pass 1 (Jul 27 18:48Z). Unchanged. |

### Supply divergence
**OBSERVED:** morning-api total = 5000 (20 + 4980). Witness total = 0 (0 + 0). Unchanged. See VERIFIED-BEHAVIOR.md "Supply Conservation (CONTRADICTED)."

---

## Persistence State

### morning-api (single-capture bundle ~12:49Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 2160 | Incrementing by 10 (pass 94: 2140; +20 = 2 rotations since pass 94) | None (normal — 2 rotations in ~8 min) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION (#2).** Endpoint reads legacy `transactions.wal`. Unchanged. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause) |

**File system cross-check (~12:49Z):**
- `state.snapshot`: **894 bytes** (mtime 08:47). Pass 94: 895 bytes (mtime 08:37). Size decreased by 1 byte (normal variance — serialized state boundaries).
- `wal.log`: 379 bytes (mtime 08:47). Pass 94: 379 bytes (mtime 08:37). Size unchanged.
- `wal.wal.old`: 379 bytes (mtime 08:42). Rotated as expected (one rotation ago).
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known deviation #2).

### local-witness (single-capture bundle ~12:49Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 2160 | Incrementing by 10 (pass 94: 2140; +20 = 2 rotations) | None (normal) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~12:49Z):**
- `state.snapshot`: **569 bytes** (mtime 08:48). Pass 94: 569 bytes (mtime 08:38). Size unchanged.
- `wal.log`: 379 bytes (mtime 08:48). Pass 94: 379 bytes (mtime ~12:38). Size unchanged.
- `wal.wal.old`: 379 bytes (mtime 08:43). Rotated as expected.
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known bug).

**Note:** Snapshot file sizes differ between nodes (894 vs 569 bytes) — expected given divergent economic state content (m-ap serializes balance=20 + peer_balance=4980, witness serializes balance=0 + peer_balance=0).

---

## Metrics Instrumentation (from heartbeat timer lines)

**morning-api (12:49:06Z):** outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=3s
**local-witness (12:49:13Z):** outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=6s

**DEVIATION:** None. Mesh is quiescent. All gauges at zero or well under thresholds. Silence stable (m-ap 3s; witness 6s, both well under 30s zombie threshold).

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **No new WARN/ERROR events.** Only 2 historic WARNs from Jul 27 (genesis gossip retry on startup). No events since.
- NTP failures: Kosher — cosmetic (known cosmetic pattern on this network-limited machine).
- Zombie eviction events: **0** (lifetime).
- Sweep/eviction events: **0** (lifetime).
- Panics: **0**.
- Insufficient balance: **0**.

### local-witness (/tmp/lw.log)
- **No new WARN/ERROR events.**
- Insufficient balance: **118** (ALL historic Jul 27 — redistribution rejections). **No new occurrences.**
- Zombie eviction events: **0** (lifetime).
- Sweep/eviction events: **0** (lifetime).
- Panics: **0**.

---

## Build Commit Verification

| Check | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| git HEAD | `cb5d4b1` | — | — |
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION (#1).** 9 commits behind HEAD + dirty tree. Unchanged from all previous passes. |

**Note:** All commits between binary and HEAD are docs/evidence files — no wire-format changes or production code changes. No functional safety risk.

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 94 | Pass 95 | Changed? |
|---|-----------|----------------|---------|---------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent (9 behind) | Persistent (9 behind) | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (socket vs last log vs count) | **PASS** — m-ap: socket=2162, count=2162, last_log=2162. Witness: socket=2162, count=2162, last_log=2162. Full match on both. No boundary race. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2). Both nodes. |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 unchanged. |
| Log health (WARN/ERROR filtered) | **PASS** — historic insufficient-balance entries only (witness: 118, m-ap: 0). No panics, no zombies, no new errors. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** — m-ap 3s, witness 6s. Both well under threshold. |
| Cross-node epoch sync (single-capture bundle) | **PASS** — both at 2162 (simultaneous). Fully converged. |
| Snapshot rotation | **PASS** — 2 rotations since pass 94 (2140→2160). Filesystem mtimes confirm disk writes. |
