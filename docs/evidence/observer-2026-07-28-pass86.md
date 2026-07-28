# Observer Evidence Record — 2026-07-28 (Pass 86)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T11:31Z bundle (single-capture discipline)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (Boynton Beach FL)
**Session type:** Eighty-sixth observation pass. Same processes since 2026-07-27T18:48Z (~16.7h runtime). ~8 min since pass 85 (11:23–25Z).

**Summary:** Routine continuation. All three persistent deviations unchanged. Epoch advanced from 1993→2006 on morning-api (+13), 1993→2007 on witness (+14) across both nodes. Snapshot rotated once (1990→2000) on both. Metrics all healthy (aged=0, queues empty, silence<10s). No new WARN/ERROR events. No panics. Mesh is quiescent and healthy.

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

### morning-api (~11:31Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZ...zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 60175 | — | None (pass 85: 59686; Δ = +489s ≈ 8.2 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 10 commits behind HEAD + dirty tree. Unchanged from pass 85. |
| thickness | 984.04 | ~984, slowly decaying | None (pass 85: 984.16; Δ = −0.12 over ~8 min) |

### local-witness (~11:31Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZ...9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZ...zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 60194 | — | None (pass 85: 59708; Δ = +486s ≈ 8.1 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api (~11:31Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=6016, silence_secs=0, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~11:31Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=6020, silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 85 (~11:23Z):** Heartbeats: m-ap +49 (5967→6016 ~6.1/min), witness +49 (5971→6020 ~6.1/min). Both at expected rate. Silence: m-ap 0s (pass 85: 2s), witness 7s (pass 85: 8s — slight improvement). Queue depth 0 on both. **No zombie eviction events. No sweep events.**

---

## Epoch State

### Single-capture bundle (~11:31Z)

| Check | morning-api | local-witness | DEVIATION |
|-------|-------------|---------------|-----------|
| Socket epoch | 2006 | 2007 | **δ=-1** (epoch boundary crossing during capture window) |
| Log count (grep -c) | 2008 | 2008 | **δ=0** |
| Last log epoch | 2008 (11:31:56Z) | 2008 (11:32:13Z) | — |
| Three-way equality | **RACE** (socket=2006, last_log=2008, count=2008 — log advanced 2 epochs during ~35s compound command) | **RACE** (socket=2007, last_log=2008, count=2008 — same) | — |

**OBSERVED:** Both nodes advancing normally. +13–14 epochs in ~8 min from pass 85 (1993). Ratio unchanged: m-ap ~1.02, witness ~1.05.

**DEVIATION:** None. The 3-way race is timing-dependent at epoch boundaries — compound command took ~35s during which 2 epochs completed on both nodes. This is a known capture artifact, not a divergence.

---

## Economic State

### morning-api (~11:31Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged since pass 85) |
| own_nonce | 120 | 120 | None (unchanged) |
| witness_balance (reported) | 4980 | 5000 - morning_api_balance = 4980 | None (unchanged) |
| witness_nonce (reported) | 0 | 0 | None |

### local-witness (~11:31Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 | None |
| own_nonce | 2 | 2 | None (unchanged since early passes) |
| morning_api_balance (reported) | 0 | 5000 | **Persistent DEVIATION (#3).** Witness reports morning-api balance as 0. First observed: pass 1 (Jul 27 18:48Z). Unchanged. |

### Supply divergence
**OBSERVED:** morning-api total = 5000 (20 + 4980). Witness total = 0 (0 + 0). Unchanged. See VERIFIED-BEHAVIOR.md "Supply Conservation (CONTRADICTED)."

---

## Persistence State

### morning-api (single-capture bundle ~11:31Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 2000 | Incrementing by 10 (pass 85: 1990; +10 = 1 rotation) | None (normal) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION (#2).** Endpoint reads legacy `transactions.wal`. Unchanged. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause) |

**File system cross-check (~11:31Z):**
- `state.snapshot`: **895 bytes** (mtime 07:27 EDT). Pass 85: 895 bytes (07:22 EDT). Size unchanged across rotation.
- `wal.log`: 379 bytes (mtime 07:27 EDT)
- `wal.wal.old`: 379 bytes (mtime 07:22 EDT)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known bug).
- **Snapshot rotation count:** 1 rotation since pass 85 (1990→2000). Filesystem mtimes confirm.

### local-witness (single-capture bundle ~11:31Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 2000 | Incrementing by 10 (pass 85: 1990; +10 = 1 rotation) | None (normal) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~11:31Z):**
- `state.snapshot`: **569 bytes** (mtime 07:28 EDT). Pass 85: 569 bytes (07:23 EDT). Size unchanged.
- `wal.log`: 379 bytes (mtime 07:28 EDT)
- `wal.wal.old`: 379 bytes (mtime 07:23 EDT)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379 (known bug).

---

## Metrics Instrumentation (from heartbeat timer lines ~11:32Z)

**morning-api:** outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=3s
**local-witness:** outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=6s

**DEVIATION:** None. Mesh is quiescent. All gauges at zero or well under thresholds. Silence improved slightly (m-ap 3s vs pass 85's 2s; witness 6s vs 8s — both healthy).

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **No new WARN/ERROR events beyond regular KAD bootstrap warnings** (every 5 min, benign with `--no-mdns`).
- NTP query failure at 11:07:29Z (one new since pass 85's last noted NTP event). Retry every 5 min as expected.
- Zombie eviction events: None.
- Sweep/eviction events: None.
- Panics: 0.

### local-witness (/tmp/lw.log)
- **No new WARN/ERROR events.** Last NTP failure at 08:00:06Z (~3.5h ago) — no retry logged since. Nodes talk to each other fine (NTP is for startup check, not runtime).
- Insufficient balance: 118 historic (all Jul 27). **No new occurrences.**
- Panics: 0.

---

## Build Commit Verification

| Check | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| git HEAD | `cb5d4b1` | — | — |
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION (#1).** 10 commits behind HEAD + dirty tree. Unchanged from pass 85. |

**Note:** All 10 commits between binary and HEAD are docs/tests/fixes — no wire-format changes. No functional safety risk.

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 85 | Pass 86 | Changed? |
|---|-----------|----------------|---------|---------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (10 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent (10 behind) | Persistent (10 behind) | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

### Previous observation carried forward: cross-node epoch δ
| Pass 85 Status | Pass 86 Status |
|----------------|----------------|
| δ=0 on simultaneous socket captures (both=1993). δ=1 in log tail due to capture timing. | δ=-1 on socket captures (m-ap=2006, witness=2007 at ~11:31Z). δ=0 on log counts (both=2008). Consistent with prior pattern — δ varies 0-1 depending on boundary proximity. |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (socket vs last log vs count) | **RACE** — socket=2006/2007, count=2008 both nodes. Epoch boundaries passed during ~35s compound capture. Normal artifact; both nodes at matched counts. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2). Both nodes. |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 unchanged. |
| Log health (WARN/ERROR filtered) | **PASS** — KAD bootstrap warnings only (benign). 118 historic insufficient-balance entries unchanged. No panics, no zombies, no new errors. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** — m-ap 3s, witness 6s. Both well under threshold. |
| Cross-node epoch sync (single-capture bundle) | **PASS** (δ=0 on log counts at 2008; socket δ=-1 due to race). |
| Snapshot rotation | **PASS** — 1 rotation since pass 85 (1990→2000). New filesystem mtimes confirm disk writes. |
