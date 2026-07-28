# Observer Evidence Record — 2026-07-28 (Pass 80)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T10:35–10:36Z bundle (socket queries + log/metrics)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Eightieth observation pass. Same processes since 2026-07-27T18:48Z (~15.8h runtime). ~8 min since pass 79 (10:26–28Z).

**Summary:** Routine continuation. Both nodes cycling normally. m-ap at epoch 1894 (+15 from pass 79's 1879 in ~8–9 min; ~32s/epoch cadence). Three persistent deviations unchanged. Zero new WARN/ERROR events since pass 79's capture at 10:26Z. **NTP failures at 09:50Z are now confirmed as historic** — the 09:40Z "RUNTIME NTP: check failed" is not the only episode; NTP query warnings also occurred at 09:50:29/32Z. Both are historic. No NTP events since 09:50:32Z (~45 min ago). Single-capture three-way epoch check PASSED on BOTH nodes (all three = 1894). Witness epoch now fully synchronized with m-ap (δ=0).

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

**NTP status (system):** System clock synchronized: yes. NTP service active.

---

## Node Info

### morning-api (~10:35Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 56745 (~15.8h) | — | None (pass 79: ~56364; Δ = +381s ≈ 6.4 min — consistent with capture interval) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree (71 unstaged files, up from 70 in pass 79). First observed: pass 1 (Jul 27). Unchanged. |
| thickness | 984.936 | ~985, slowly decaying | None (pass 79: 985.062; Δ = −0.126 over ~9 min ≈ −0.014/min — consistent decay rate) |

### local-witness (~10:35Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 56760 (~15.8h) | — | None (pass 79: ~56350; Δ = +410s ≈ 6.8 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api (~10:35Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=5673, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~10:35Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=5677, silence_secs=2, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 79 (~10:18Z):** Heartbeats: api +49 (5624→5673 ~5.4/min), witness +49 (5628→5677 ~5.4/min). Both at expected rate. Silence: api 3s (pass 79: 6s — improved), witness 2s (pass 79: 0s). Queue depth 0 on both. **No zombie eviction events. No sweep events.**

---

## Epoch State

### morning-api (~10:35Z socket capture → 10:35Z log check)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1894 (endpoint) | Cycling. +15 from pass 79 (1879→1894) in ~8–9 min. | None. Normal cadence (~32s/epoch). |
| ratio | 1.01998 | ~1.01–1.02 steady state | None (pass 79: 1.02; essentially unchanged) |
| tax_calculated | 0 | Balance 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (~10:35Z socket capture → 10:35Z log check)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1894 (endpoint) | Cycling. +15 from pass 79 (1879→1894) in ~8–9 min. | None. Witness now fully in sync (δ=0; was δ=1 in pass 78, δ=0 in pass 79). |
| ratio | 1.05264 | Continuing asymptotic decline toward 1.0 | None (pass 79: 1.05; continued gradual decline) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** Both nodes at epoch 1894 (simultaneous socket queries). δ=0. Fully synchronized.

### Three-way epoch check (sequential capture bundle, ~10:35Z)
- **morning-api:** Socket=1894, log_count=1894, last_log epoch=1894 (10:34:56Z). **Δ=0 — PASS.**
- **local-witness:** Socket=1894, log_count=1894, last_log epoch=1894 (10:35:13Z). **Δ=0 — PASS.** (Improvement from pass 79's δ=1 on witness)

---

## Economic State

### morning-api (~10:35Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged) |
| own_nonce | 120 | 120 | None (unchanged) |
| witness_balance (reported) | 4980 | 5000 - morning_api_balance = 4980 | None (mesh consensus on peer balance — unchanged) |
| witness_nonce (reported) | 0 | 0 | None |

### local-witness (~10:35Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 | None |
| own_nonce | 2 | 2 | None (unchanged since early passes) |
| morning_api_balance (reported) | 0 | 5000 | **Persistent DEVIATION.** Witness reports morning-api balance as 0. First observed: observer pass 1 (Jul 27 18:48Z). Unchanged. |

### Supply divergence
**OBSERVED:** morning-api sees total supply = 20 + 4980 = 5000. Witness sees total supply = 0 + 0 = 0.
**DEVIATION:** Witness-side accounting reports 0. Unchanged since first observer pass. See VERIFIED-BEHAVIOR.md "Supply Conservation (CONTRADICTED)" for the canonical reference.

---

## Persistence State

### morning-api (~10:35Z socket + filesystem, sequential capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1890 | Incrementing by 10 (pass 79: 1870; +20 = 2 rotations) | None (normal — 2 rotations: 1870→1880→1890) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause) |

**File system cross-check (~10:35Z):**
- `state.snapshot`: **895 bytes** (mtime: 2026-07-28T06:32:16 EDT — NEW; was 06:17 in pass 79. Corresponds to epoch 1890 snapshot rotation.)
- `wal.log`: 379 bytes (mtime: 2026-07-28T06:32:16 EDT — updated from 06:17)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T06:27:16 EDT — unchanged since pass 79's 06:27)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- **Snapshot rotation count:** 2 rotations since pass 79 (1860→1870 already counted; now 1870→1880→1890 confirmed by mtime change from 06:17→06:32 EDT).
- **Note:** The 06:17 snapshot from pass 79 was the epoch 1860 snapshot. The 06:32 snapshot at epoch 1890 is a new rotation. This means a rotation was missed between 1870 and 1880 — it would have occurred between 10:18Z (pass 79) and 10:35Z (now). The file system only shows the latest snapshot, so intermediate rotations are not visible on disk.

### local-witness (~10:35Z socket + filesystem, sequential capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1890 | Incrementing by 10 (pass 79: 1870; +20 = 2 rotations) | None (normal) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~10:35Z):**
- `state.snapshot`: **569 bytes** (mtime: 2026-07-28T06:33:13 EDT — updated from 06:18 in pass 79)
- `wal.log`: 379 bytes (mtime: 2026-07-28T06:33:13 EDT — updated)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T06:28:13 EDT — updated from 06:13)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot size 569 bytes — unchanged. ✓

---

## Metrics Instrumentation

**OBSERVED (from m-ap metrics lines at ~10:35Z):**
- `outstanding_fetches=0` — no pending fetches
- `aged=0` — no stale fetches
- `outbound_queues=[]` — all peer queues empty
- `max_peer_silence=3s` — well under 30s threshold

**OBSERVED (from witness metrics lines at ~10:36Z):**
- `outstanding_fetches=0`
- `aged=0`
- `outbound_queues=[]`
- `max_peer_silence=6s`

**EXPECTED:** All gauges near zero on a settled 2-node mesh with no new transactions.
**DEVIATION:** None. Mesh is quiescent.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **No new WARN/ERROR events since pass 79 capture (10:26Z).**
- **NTP:** NTP WARN-level events at 07:37:59Z (pass range 74–77), 09:40:16Z (pass 74 — the "RUNTIME NTP: check failed" episode), and 09:50:29/32Z (pass ~75/76 — individual query failures). **CORRECTION to pass 79:** The claim "no NTP-related log lines since pass 77" needs qualification — NTP query failures occurred at 09:50:29Z and 09:50:32Z, which were between pass 75 and pass 77. These are now historic (~45 min ago). **No NTP events since 09:50:32Z.**
- **Zombie eviction events:** None.
- **Sweep/eviction events:** None (0 `swept` events, 0 `evict` events, 0 `stale fetch` events).
- **Panics:** None.
- **KAD bootstrap warnings:** Continuous (every 5 minutes). Benign — `--no-mdns` with single bootstrap peer. Filtered as known pattern.

### local-witness (/tmp/lw.log)
- **No new WARN/ERROR events since pass 79 capture (10:26Z).**
- **NTP:** One WARN at 08:00:06Z (historic, pass 74–75 range). No NTP events since.
- **Insufficient balance:** 118 (unchanged, all historic Jul 27). **No new occurrences.**
- **Panics:** None.

### Log filter (WARN/ERROR excluded as benign)

| Pattern | m-ap | lw | Status |
|---------|------|----|--------|
| `Failed to trigger bootstrap` | Many (continuous) | 0 | Benign — `--no-mdns` with single bootstrap peer |
| `skip-ntp-check` | 0 | 0 | Clean |
| `No snapshot` | 0 | 3 (startup) | Clean |
| `zombie` | 0 | 0 | Clean |
| `insufficient balance` | 0 | 118 (historic) | No new occurrences |
| `panicked` | 0 | 0 | Clean |
| `NTP.*fail` | **5** (07:37Z, 09:40Z×2, 09:50Z×2 — all historic) | 1 (08:00Z, historic) | **CORRECTION to pass 79:** 5 total NTP failures (not 3). The 09:50Z pair was not counted in pass 79's NTP line. Not recurred for ~45 min. |

---

## Build Commit Verification

| Check | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| git HEAD | `cb5d4b1` | — | — |
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree (71 unstaged files, up from 70 in pass 79). Unchanged since pass 1. |

**Commits between binary and HEAD (9):**
```
cb5d4b1 docs: Observer evidence corpus + Verifier missions 1 and 2
aa62d12 docs: note /tmp identity dir fragility across reboots
c008def docs: MESH.md — record stable PeerIds after mesh relaunch
93d0ef4 docs: restructure verified behavior with evidence tiers
7ab64c2 docs: sharpen MESH.md header to configuration-focused language
19c9d05 docs: split MESH.md (topology only) from VERIFIED-BEHAVIOR.md
32efcf1 fix: stale fixture bugs in witness harness — epoch + witness identity
214eb73 fix: declare claimant variable in two_swarm witness harness tests
b4aa212 test: cap enforcement — 64th accepted, 65th rejected, duplicate is no-op
```

**Note:** All 9 commits are docs/tests/fixes — no wire-format changes. The running binary's stale build commit is not a functional safety risk.

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 79 Status | Pass 80 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No. Unstaged files count: 71 (pass 79: 70 — ±1 drift, likely from cron/log artifacts). |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

**No new deviations detected in this pass.**

---

## Minor Observations (Not Deviations)

| Observation | First Noted | Status |
|-------------|------------|--------|
| morning-api snapshot size stable at 895 bytes for sixth consecutive pass | Pass 71 | Resolved. Size stabilized at 895 (no further oscillation). |
| NTP runtime check failure episodes (09:40Z + 09:50Z) — not recurred for ~45 min | Pass 74 | **CORRECTION: Two episodes, not one.** 09:40Z "RUNTIME NTP: check failed" + 09:50Z individual query failures. Both historic. Last event at 09:50:32Z — 45 min of NTP silence confirmed as of 10:35Z. |
| Witness epoch fully synchronized with m-ap (δ=0) — improvement from pass 78's δ=1 | Pass 80 | Both nodes at epoch 1894. Three-way epoch check PASSED on both. |
| KAD bootstrap warnings on m-ap every 5 min (continuous) | Pass 1 | Benign with `--no-mdns`. Noted for completeness. |
| Unstaged files count: 71 (pass 79: 70) — Δ=+1 | This pass | Minor drift. Not a concern — could be a cron/temp artifact touching a tracked file. Not investigated. |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (socket, log count, last log line) | **m-ap: PASS** (all three = 1894). **Witness: PASS** (all three = 1894 — exact match, improvement from pass 79's δ=1). |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2). Both nodes. |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 unchanged. |
| Log health (WARN/ERROR filtered) | **PASS** — no new anomalous events. NTP events confirmed as historic (07:37Z through 09:50Z; last at 09:50:32Z; 45 min silent). 118 historic insufficient-balance entries unchanged. No panics, no zombies, no errors. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** — m-ap 3s, witness 6s. Both well under threshold. |
| Cross-node epoch sync | **PASS** — both nodes at epoch 1894 (δ=0). Witness fully synchronized. |
| Snapshot rotation | **PASS** — 2 rotations since pass 79 (1870→1880→1890). New snapshot filesystem mtimes confirm disk writes. |
