# Observer Evidence Record — 2026-07-28 (Pass 81)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T10:44–10:45Z bundle (socket queries + log/metrics)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Eighty-first observation pass. Same processes since 2026-07-27T18:48Z (~15.9h runtime). ~9 min since pass 80 (10:35–36Z).

**Summary:** Routine continuation. Both nodes cycling normally. m-ap at epoch 1912 (+18 from pass 80's 1894 in ~9 min; ~30s/epoch cadence — slightly faster than pass 80's ~32s/epoch). Three persistent deviations unchanged. Zero new WARN/ERROR events since pass 80 capture at 10:35Z. NTP events confirmed as historic (last at 09:50:32Z on m-ap, ~54 min ago; last at 08:00:06Z on witness, ~2h44m ago). Single-capture three-way epoch check PASSED on both nodes (all three = 1912). Cross-node epoch sync: δ=0 after re-query (initial sequential query showed δ=1 — race at epoch boundary, resolved on re-query).

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 2026-07-27T18:48Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 2026-07-27T18:48Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**Log file note:** witness log at `/tmp/lw.log` (shell redirect in process command), NOT `/tmp/local-witness/lw.log`. Socket and storage dir at `/tmp/local-witness/`.

**No topology changes.** Same PIDs (2727391, 2727569). Both sockets responding.

**NTP status (system):** System clock synchronized: yes. NTP service active.

---

## Node Info

### morning-api (~10:44Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 57304 (~15.9h) | — | None (pass 80: 56745; Δ = +559s ≈ 9.3 min — consistent with capture interval) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree (72 unstaged files, up from 71 in pass 80). First observed: pass 1 (Jul 27). Unchanged. |
| thickness | 984.79 | ~985, slowly decaying | None (pass 80: 984.94; Δ = −0.15 over ~9 min ≈ −0.017/min — consistent decay rate) |

### local-witness (~10:44Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 57300 (~15.9h) | — | None (pass 80: 56760; Δ = +540s ≈ 9 min) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api (~10:44Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=5729, silence_secs=1, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~10:44Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=5731, silence_secs=1, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 80 (~10:35Z):** Heartbeats: api +56 (5673→5729 ~6.2/min), witness +54 (5677→5731 ~6.0/min). Both at expected rate. Silence: both at 1s (improvement from pass 80: api 3s, witness 2s). Queue depth 0 on both. **No zombie eviction events. No sweep events.**

---

## Epoch State

### morning-api (~10:44Z socket capture → 10:44Z log check)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1912 (endpoint, confirmed on re-query) | Cycling. +18 from pass 80 (1894→1912) in ~9 min. | None. Normal cadence (~30s/epoch). |
| ratio | 1.01980 | ~1.01–1.02 steady state | None (pass 80: 1.01998; essentially unchanged) |
| tax_calculated | 0 | Balance 20: ~5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (~10:44Z socket capture → 10:44Z log check)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1912 (endpoint, confirmed on re-query) | Cycling. +18 from pass 80 (1894→1912) in ~9 min. | None. Witness in sync (δ=0). |
| ratio | 1.05208 | Continuing asymptotic decline toward 1.0 | None (pass 80: 1.05264; continued gradual decline) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** Both nodes at epoch 1912 (re-queried simultaneously). δ=0. Fully synchronized.
**Note:** Initial sequential query showed δ=1 (api=1912, witness=1911) — this was a race at the epoch boundary. Re-query 10 seconds later confirmed both at 1912.

### Three-way epoch check (sequential capture bundle, ~10:44Z)
- **morning-api:** Socket=1912, log_count=1912, last_log epoch=1912 (10:43:56Z). **Δ=0 — PASS.**
- **local-witness:** Socket=1912, log_count=1912, last_log epoch=1912 (10:44:13Z). **Δ=0 — PASS.**

---

## Economic State

### morning-api (~10:44Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged) |
| own_nonce | 120 | 120 | None (unchanged) |
| witness_balance (reported) | 4980 | 5000 - morning_api_balance = 4980 | None (mesh consensus on peer balance — unchanged) |
| witness_nonce (reported) | 0 | 0 | None |

### local-witness (~10:44Z)

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

### morning-api (~10:44Z socket + filesystem, sequential capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1910 | Incrementing by 10 (pass 80: 1890; +20 = 2 rotations) | None (normal — 2 rotations: 1890→1900→1910) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause) |

**File system cross-check (~10:44Z):**
- `state.snapshot`: **895 bytes** (mtime: 2026-07-28T06:42 EDT — updated from 06:32 in pass 80. Corresponds to epoch 1910 snapshot rotation.)
- `wal.log`: 379 bytes (mtime: 2026-07-28T06:42 EDT — updated from 06:32)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T06:37 EDT — updated from 06:27)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- **Snapshot rotation count:** 2 rotations since pass 80 (1890→1900→1910 confirmed by mtime change from 06:32→06:42 EDT).
- Snapshot size stable at 895 bytes for seventh consecutive pass. ✓

### local-witness (~10:44Z socket + filesystem, sequential capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1910 | Incrementing by 10 (pass 80: 1890; +20 = 2 rotations) | None (normal) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~10:44Z):**
- `state.snapshot`: **569 bytes** (mtime: 2026-07-28T06:43 EDT — updated from 06:33 in pass 80)
- `wal.log`: 379 bytes (mtime: 2026-07-28T06:43 EDT — updated from 06:33)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T06:38 EDT — updated from 06:28)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot size 569 bytes — unchanged. ✓

---

## Metrics Instrumentation

**OBSERVED (from m-ap metrics lines at ~10:44Z):**
- `outstanding_fetches=0` — no pending fetches
- `aged=0` — no stale fetches
- `outbound_queues=[]` — all peer queues empty
- `max_peer_silence=3s` — well under 30s threshold

**OBSERVED (from witness metrics lines at ~10:44Z):**
- `outstanding_fetches=0`
- `aged=0`
- `outbound_queues=[]`
- `max_peer_silence=6s`

**EXPECTED:** All gauges near zero on a settled 2-node mesh with no new transactions.
**DEVIATION:** None. Mesh is quiescent.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **No new WARN/ERROR events since pass 80 capture (10:35Z).**
- **NTP:** Last NTP event: 09:50:32Z (time.apple.com query failed). ~54 min silence. All NTP events confirmed historic (7 events total: 07:37Z×1, 09:40Z×3, 09:50Z×2). No NTP events since 09:50:32Z.
- **Zombie eviction events:** None.
- **Sweep/eviction events:** None (0 `swept` events, 0 `evict` events, 0 `stale fetch` events).
- **Panics:** None.
- **KAD bootstrap warnings:** Continuous (every 5 minutes). Benign — `--no-mdns` with single bootstrap peer.

### local-witness (/tmp/lw.log)
- **No new WARN/ERROR events since pass 80 capture (10:35Z).**
- **NTP:** Last NTP event: 08:00:06Z (~2h44m ago). Historic.
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
| `NTP.*fail|NTP.*failed|Failed to create NTP|RUNTIME NTP` | **7** (all historic: 07:37Z, 09:40Z×3, 09:50Z×2 — last at 09:50:32Z, ~54 min ago) | **1** (08:00Z, historic, ~2h44m ago) | All historic. No new events. |

---

## Build Commit Verification

| Check | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| git HEAD | `cb5d4b1` | — | — |
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree (72 unstaged files, up from 71 in pass 80). Unchanged since pass 1. |

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

**Unstaged files:** 72 (pass 80: 71 — Δ=+1). Minor drift; likely a cron/temp artifact.

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 80 Status | Pass 81 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No. Unstaged files count: 72 (pass 80: 71 — +1 minor drift). |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

**No new deviations detected in this pass.**

---

## Minor Observations (Not Deviations)

| Observation | First Noted | Status |
|-------------|------------|--------|
| morning-api snapshot size stable at 895 bytes for seventh consecutive pass | Pass 71 | Stable. Size stabilized at 895 (no further oscillation). |
| NTP runtime check failure episodes (09:40Z + 09:50Z) — not recurred for ~54 min | Pass 74 | Historic. Last event on m-ap at 09:50:32Z. ~54 min of NTP silence confirmed. Witness last NTP event at 08:00:06Z (~2h44m ago). |
| Initial sequential query showed δ=1 epoch gap between nodes (api=1912, witness=1911) — resolved to δ=0 on re-query | This pass | Race at epoch boundary during sequential capture. Confirmed δ=0 on simultaneous re-query. Not a deviation. |
| KAD bootstrap warnings on m-ap every 5 min (continuous) | Pass 1 | Benign with `--no-mdns`. Noted for completeness. |
| Unstaged files count: 72 (pass 80: 71) — Δ=+1 | This pass | Minor drift. Not a concern — could be a cron/temp artifact. Not investigated. |
| Witness log at /tmp/lw.log (not /tmp/local-witness/lw.log) | This pass | Log path noted for future passes. The log redirect in the process command writes to `/tmp/lw.log` while the socket and storage are at `/tmp/local-witness/`. No functional impact. |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (socket, log count, last log line) | **m-ap: PASS** (all three = 1912). **Witness: PASS** (all three = 1912). |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2). Both nodes. |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 unchanged. |
| Log health (WARN/ERROR filtered) | **PASS** — no new anomalous events. NTP events confirmed as historic. 118 historic insufficient-balance entries unchanged. No panics, no zombies, no errors. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** — m-ap 3s, witness 6s. Both well under threshold. |
| Cross-node epoch sync | **PASS** — both nodes at epoch 1912 (δ=0). Fully synchronized. |
| Snapshot rotation | **PASS** — 2 rotations since pass 80 (1890→1900→1910). New snapshot filesystem mtimes confirm disk writes. |
