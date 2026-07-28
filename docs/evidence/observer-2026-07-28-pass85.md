# Observer Evidence Record — 2026-07-28 (Pass 85)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T11:23–11:25Z bundle (single-capture discipline)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (Boynton Beach FL)
**Session type:** Eighty-fifth observation pass. Same processes since 2026-07-27T18:48Z (~16.6h runtime). ~9 min since pass 84 (11:14–16Z).

**Summary:** Routine continuation. All three persistent deviations unchanged. Epoch advanced from 1976→1993 (+17) across both nodes. Snapshot rotated twice (1970→1990). Single-capture three-way epoch check PASSED on both nodes at socket=1993—last_log=1993—count=1993. Cross-node sync δ=0. No new WARN/ERROR events. Mesh is quiescent and healthy.

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

### morning-api (~11:23Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZ...zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 59686 | — | None (pass 84: 59174; Δ = +512s ≈ 8.5 min — consistent) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 10 commits behind HEAD + dirty tree. Unchanged since pass 84 (was 9 commits; 1 new doc commit added). |
| thickness | 984.16 | ~984, slowly decaying | None (pass 84: 984.30; Δ = −0.14 over ~9 min — consistent decay rate) |

### local-witness (~11:23Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZ...9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZ...zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 59708 | — | None (pass 84: 59181; Δ = +527s ≈ 8.8 min — sequential capture offset) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api (~11:23Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=5967, silence_secs=2, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~11:23Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=5971, silence_secs=8, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 84 (~11:14Z):** Heartbeats: m-ap +51 (5916→5967 ~6.4/min), witness +52 (5919→5971 ~6.5/min). Both at expected rate. Silence: m-ap 2s (pass 84: 3s), witness 8s (pass 84: 4s — slight increase, well under 30s threshold). Queue depth 0 on both. **No zombie eviction events. No sweep events.**

---

## Epoch State

### Single-capture bundle (~11:24:57Z)

| Check | morning-api | local-witness | DEVIATION |
|-------|-------------|---------------|-----------|
| Socket epoch | 1993 | 1993 | **δ=0** |
| Log count (grep -c) | 1993 | 1993 | **δ=0** |
| Last log epoch | 1993 (11:24:26Z) | 1993 (11:24:43Z) | — |
| Three-way equality | **PASS** (socket=1993, last_log=1993, count=1993) | **PASS** (socket=1993, last_log=1993, count=1993) | — |

**OBSERVED:** Both nodes at epoch 1993. +17 from pass 84's 1976 in ~9 min. Ratio unchanged: m-ap ~1.0198, witness ~1.050 (continued asymptotic decline from 1.05085→1.04970—consistent decay).

**DEVIATION:** None. Epoch cycling normal for both nodes.

---

## Economic State

### morning-api (~11:23Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged since pass 84) |
| own_nonce | 120 | 120 | None (unchanged since pass 84) |
| witness_balance (reported) | 4980 | 5000 - morning_api_balance = 4980 | None (unchanged) |
| witness_nonce (reported) | 0 | 0 | None |

### local-witness (~11:23Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 | None |
| own_nonce | 2 | 2 | None (unchanged since early passes) |
| morning_api_balance (reported) | 0 | 5000 | **Persistent DEVIATION (#3).** Witness reports morning-api balance as 0. First observed: pass 1 (Jul 27 18:48Z). Unchanged. |

### Supply divergence
**OBSERVED:** morning-api total = 5000 (20 + 4980). Witness total = 0 (0 + 0). Unchanged. See VERIFIED-BEHAVIOR.md "Supply Conservation (CONTRADICTED)."

---

## Persistence State

### morning-api (single-capture bundle ~11:24Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1990 | Incrementing by 10 (pass 84: 1970; +20 = 2 rotations) | None (normal) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION (#2).** Endpoint reads legacy `transactions.wal`. First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause) |

**File system cross-check (~11:23Z):**
- `state.snapshot`: **895 bytes** (mtime 07:22 EDT). Pass 84: 894 bytes (07:12 EDT). Δ=+1 byte over one rotation cycle.
- `wal.log`: 379 bytes (mtime 07:22 EDT)
- `wal.wal.old`: 379 bytes (mtime 07:17 EDT)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known bug).
- **Snapshot rotation count:** 2 rotations since pass 84 (1970→1980→1990). Filesystem mtimes confirm.

### local-witness (single-capture bundle ~11:24Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1990 | Incrementing by 10 (pass 84: 1970; +20 = 2 rotations) | None (normal) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~11:23Z):**
- `state.snapshot`: **569 bytes** (mtime 07:23 EDT). Pass 84: 569 bytes (07:13 EDT). Size unchanged.
- `wal.log`: 379 bytes (mtime 07:23 EDT)
- `wal.wal.old`: 379 bytes (mtime 07:18 EDT)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known bug).

---

## Metrics Instrumentation (from heartbeat timer lines ~11:23Z)

**morning-api:** outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=2s
**local-witness:** outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=8s

**DEVIATION:** None. Mesh is quiescent. All gauges at zero or well under thresholds.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **No new WARN/ERROR events since pass 84 capture** (last pass noted NTP event at 11:07:29Z; no additional since).
- KAD bootstrap warnings continue every 5 min (benign, `--no-mdns`).
- Zombie eviction events: None.
- Sweep/eviction events: None.
- Panics: None.

### local-witness (/tmp/lw.log)
- **No new WARN/ERROR events since pass 84.** Last: NTP query failure at 08:00:06Z (~3h24m ago).
- Insufficient balance: 118 (historic, all Jul 27). **No new occurrences.**
- Panics: None.

---

## Build Commit Verification

| Check | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| git HEAD | `cb5d4b1` | — | — |
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION (#1).** 10 commits behind HEAD + dirty tree (pass 84: 9 commits; 1 new doc commit). |

**Note:** All 10 commits between binary and HEAD are docs/tests/fixes — no wire-format changes. No functional safety risk.

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 84 | Pass 85 | Changed? |
|---|-----------|----------------|---------|---------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (10 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent (9 behind) | Persistent (10 behind) | Yes — +1 commit behind. Dirty tree unchanged. |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

### Previous observation carried forward: cross-node epoch δ
| Pass 84 Status | Pass 85 Status |
|----------------|----------------|
| δ=0 on simultaneous socket captures (both=1973, then both=1976). δ=1 in log counts (m-ap=1977, witness=1976). | **δ=0 on both socket AND log counts** (both=1993 at 11:24:57Z). Log counts equal (1993 each). |

**OBSERVED:** Epoch synchronization δ=0 on this pass's single-capture bundle — both socket and log agree at 1993. Consistent with the observed pattern: δ varies between 0 and 1 depending on capture timing relative to each node's epoch boundary.

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (socket vs last log vs count) | **m-ap: PASS** (socket=1993, last_log=1993, count=1993). **Witness: PASS** (socket=1993, last_log=1993, count=1993). |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2). Both nodes. |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 unchanged. |
| Log health (WARN/ERROR filtered) | **PASS** — no new events since pass 84. 118 historic insufficient-balance entries unchanged. No panics, no zombies, no new errors. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** — m-ap 2s, witness 8s. Both well under threshold. |
| Cross-node epoch sync (single-capture bundle) | **PASS** (δ=0 at 11:24:57Z). |
| Snapshot rotation | **PASS** — 2 rotations since pass 84 (1970→1990). New filesystem mtimes confirm disk writes. |
