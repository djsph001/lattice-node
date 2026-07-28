# Observer Evidence Record — 2026-07-28 (Pass 96)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T12:58Z bundle (single-capture discipline)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (Boynton Beach FL)
**Session type:** 96th observation pass. Same processes since 2026-07-27T18:48Z (~18.1h runtime). ~9 min since pass 95 (12:49Z).

**Summary:** Routine continuation. All three persistent deviations unchanged. Epoch advanced from 2162→2180 on both nodes (+18 each, socket reading). Three-way epoch check shows **FULL MATCH** on both nodes (socket=2180, count=2180, last_log=2180) — no boundary race this pass. Two snapshot rotations since pass 95 (2160→2180 on both). Metrics all healthy (aged=0, queues empty, silence≤8s). No new WARN/ERROR events. No panics. No zombie evictions. Mesh is quiescent and healthy.

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

### morning-api (~12:58Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZ...zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 65369 | — | None (pass 95: 64711; Δ = +658s ≈ 11 min — includes sequential-read gap) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION (#1).** 9 commits behind HEAD + dirty tree. Unchanged from pass 95. |
| thickness | 982.67 | ~983, slowly decaying | None (pass 95: 982.84; Δ = −0.17 over ~9 min — consistent decay rate) |

### local-witness (~12:58Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZ...9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZ...zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 65368 | — | None (pass 95: 64773; Δ = +595s ≈ 9.9 min — simultaneous read with morning-api) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

### Build gap (unchanged from pass 95)

Running binary at `71aa16b` (`wip: update Cargo.lock`, Jul 27 13:46). HEAD is `cb5d4b1` (docs). All missing commits are docs + evidence files. No production code changes missing. `-dirty` composition unknown.

---

## Epoch State

### Single-capture bundle — morning-api (~12:58Z)

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 2180 (at 12:57:56Z) | — |
| Log count (grep -c) | 2180 | — |
| Last log epoch | 2180 (12:57:56Z) | — |
| Three-way equality | **MATCH** — socket=2180, count=2180, last_log=2180. No boundary race. | None |

### Single-capture bundle — local-witness (~12:58Z)

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 2180 (at 12:58:13Z) | — |
| Log count (grep -c) | 2180 | — |
| Last log epoch | 2180 (12:58:13Z) | — |
| Three-way equality | **MATCH** — socket=2180, count=2180, last_log=2180. No boundary race. | None |

**OBSERVED:** Both nodes advancing normally. +18 epochs in ~9 min from pass 95 (2162→2180, socket). Consistent with previous rate (~2/min).

**Cross-node epoch δ:** Identical (2180 on simultaneous read). Both nodes perfectly converged.

**Delta from pass 95 (~12:49Z):** Pass 95 epoch=2162 (socket). Pass 96 epoch=2180 (socket). Advance of +18 epochs in ~9 min.

---

## Peer Connections

### morning-api (~12:58Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=6535, silence_secs=8, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~12:58Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=6538, silence_secs=1, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 95 (~12:49Z):** Heartbeats: m-ap +65 (6470→6535 ~7.2/min), witness +59 (6479→6538 ~6.6/min). Both at expected rate. Silence: m-ap 8s (pass 95: 3s — slightly higher but well under zombie threshold), witness 1s (pass 95: 6s — stable). Queue depth 0 on both.
**No zombie eviction events. No sweep events.**

---

## Economic State

### morning-api (~12:58Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged) |
| own_nonce | 120 | 120 | None (unchanged) |
| witness balance (reported) | 4980 | 5000 - morning_api_balance = 4980 | None (unchanged) |
| witness nonce (reported) | 0 | 0 | None |

### local-witness (~12:58Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 | None |
| own_nonce | 2 | 2 | None (unchanged since early passes) |
| morning_api balance (reported) | 0 | 5000 | **Persistent DEVIATION (#3).** Witness reports morning-api balance as 0. First observed: pass 1 (Jul 27 18:48Z). Unchanged. |

### Supply divergence
**OBSERVED:** morning-api total = 5000 (20 + 4980). Witness total = 0 (0 + 0). Unchanged. See VERIFIED-BEHAVIOR.md "Supply Conservation (CONTRADICTED)."

---

## Persistence State

### morning-api (single-capture bundle ~12:58Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 2180 | Incrementing by 10 (pass 95: 2160; +20 = 2 rotations) | None (normal — 2 rotations in ~9 min) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION (#2).** Endpoint reads legacy `transactions.wal`. Unchanged. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause) |

**File system cross-check (~12:58Z):**
- `state.snapshot`: **894 bytes** (mtime 08:57). Pass 95: 894 bytes (mtime 08:47). Size unchanged.
- `wal.log`: 379 bytes (mtime 08:57). Pass 95: 379 bytes (mtime 08:47). Size unchanged.
- `wal.wal.old`: 379 bytes (mtime 08:52). Rotated as expected (one rotation ago).
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known deviation #2).

### local-witness (single-capture bundle ~12:58Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 2180 | Incrementing by 10 (pass 95: 2160; +20 = 2 rotations) | None (normal) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~12:58Z):**
- `state.snapshot`: **569 bytes** (mtime 08:58). Pass 95: 569 bytes (mtime 08:48). Size unchanged.
- `wal.log`: 379 bytes (mtime 08:58). Pass 95: 379 bytes (mtime ~12:48). Size unchanged.
- `wal.wal.old`: 379 bytes (mtime 08:53). Rotated as expected.
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known bug).

**Note:** Snapshot file sizes differ between nodes (894 vs 569 bytes) — expected given divergent economic state content (m-ap serializes balance=20 + peer_balance=4980, witness serializes balance=0 + peer_balance=0).

---

## Metrics Instrumentation (from heartbeat timer lines — inferred from gauges)

Metrics piggyback on heartbeat timer; these are derived from endpoint data:

| Gauge | morning-api | local-witness | Threshold |
|-------|------------|---------------|-----------|
| outstanding_fetches | 0 (endpoint reports no fetch state) | 0 | Should stay near zero |
| aged (>50s) | 0 | 0 | Should stay near zero |
| outbound_queues (non-empty peers) | 0 (queue_depth=0) | 0 (queue_depth=0) | Should be empty |
| max_peer_silence | 8s | 1s | <30s zombie threshold |
| Active peers | 1 | 1 | Stable |

**DEVIATION:** None. Mesh is quiescent. All gauges at zero or well under thresholds. Silence stable (m-ap 8s, witness 1s, both well under 30s zombie threshold).

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **KN5 bootstrap WARNs:** `Failed to trigger bootstrap: No known peers` — expected on `--no-mdns` mesh (no Kademlia DHT configured). Every 5 min. **Not new.**
- **NTP fallback WARNs:** Cosmetic — `/tmp` network-limited environment. **Not new.**
- Zombie eviction events: **0** (lifetime).
- Sweep/eviction events: **0** (lifetime).
- Panics: **0**.
- Insufficient balance: **0**.

### local-witness (/tmp/lw.log)
- **NTP fallback WARN:** One at 08:00 — cosmetic. No new entries since.
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

| # | Deviation | First Observed | Pass 95 | Pass 96 | Changed? |
|---|-----------|----------------|---------|---------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent (9 behind) | Persistent (9 behind) | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (socket vs last log vs count) | **PASS** — m-ap: socket=2180, count=2180, last_log=2180. Witness: socket=2180, count=2180, last_log=2180. Full match on both. No boundary race. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2). Both nodes. |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 unchanged. |
| Log health (WARN/ERROR filtered) | **PASS** — historic insufficient-balance entries only (witness: 118, m-ap: 0). KN5 bootstrap WARNs (expected). NTP failures (cosmetic). No panics, no zombies, no new errors. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** — m-ap 8s, witness 1s. Both well under threshold. |
| Cross-node epoch sync (single-capture bundle) | **PASS** — both at 2180 (simultaneous). Fully converged. |
| Snapshot rotation | **PASS** — 2 rotations since pass 95 (2160→2180). Filesystem mtimes confirm disk writes. |
