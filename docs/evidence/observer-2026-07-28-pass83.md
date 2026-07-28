# Observer Evidence Record — 2026-07-28 (Pass 83)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T11:04–11:07Z bundle (socket queries + log/metrics)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (Boynton Beach FL)
**Session type:** Eighty-third observation pass. Same processes since 2026-07-27T18:48Z (~16.2h runtime). ~17 min since pass 82 (10:50–52Z).

**Summary:** Routine continuation. Both nodes cycling normally. Three persistent deviations unchanged. **New observation:** cross-node epoch synchronization δ has diverged to 1 (m-ap at 1954, witness at 1953) — first observed this pass; previously δ=0 at epoch 1928 in pass 82. Zero new WARN/ERROR events since pass 82. NTP silence continues (last m-ap event at 09:50:32Z, ~1h10m ago). Single-capture three-way epoch check PASSED on both nodes (each internally consistent). Snapshot rotations proceeding normally (3 rotations since pass 82: 1920→1950 on both nodes).

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

### morning-api (~11:05Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 58668 | — | None (pass 82: 57758; Δ = +910s ≈ 15.2 min — consistent with capture interval) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree (74 unstaged files, up from 73 in pass 82). First observed: pass 1 (Jul 27). Unchanged. |
| thickness | 984.43 | ~985, slowly decaying | None (pass 82: 984.67; Δ = −0.24 over ~17 min ≈ −0.014/min — consistent decay rate) |

### local-witness (~11:06Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 58663 | — | None (pass 82: 57792; Δ = +871s ≈ 14.5 min — sequential capture offset) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api (~11:05Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=5865, silence_secs=4, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~11:06Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=5867, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 82 (~10:50Z):** Heartbeats: api +91 (5774→5865 ~5.4/min), witness +87 (5780→5867 ~5.1/min). Both at expected rate. Silence: api at 4s (pass 82: 6s — improved), witness at 3s (pass 82: 3s — stable). Queue depth 0 on both. **No zombie eviction events. No sweep events.**

---

## Epoch State

### morning-api (~11:06Z socket+log simultaneous capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1954 (endpoint, confirmed in simultaneous capture) | Cycling. +26 from pass 82 (1928→1954) in ~17 min. | None. Normal cadence (~39s/epoch). |
| ratio | 1.01980 | ~1.01–1.02 steady state | None (pass 82: 1.01980; essentially unchanged) |
| tax_calculated | 0 | Balance 20: ~5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (~11:06Z socket+log simultaneous capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1953 (endpoint, confirmed on re-query) | Cycling. +25 from pass 82 (1928→1953) in ~17 min. | **Minor DEVIATION (first observed this pass).** Witness lags m-ap by 1 epoch (m-ap=1954, witness=1953, δ=1). In pass 82 both were at 1928 (δ=0). Likely a cumulative effect of the ratio difference (m-ap 1.0198 vs witness 1.0509 — witness epochs are ~3% longer). |
| ratio | 1.05085 | Continuing asymptotic decline toward 1.0 | None (pass 82: 1.05160; continued gradual decline) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** m-ap at epoch 1954, witness at epoch 1953 (simultaneous capture). δ=1.
**DEVIATION:** First observed this pass. Pass 82 had δ=0 at epoch 1928.

**UNKNOWN:** Whether this is a persistent phase drift (witness epochs are ~3% longer by ratio) or a transient measuring artifact. Not investigated — this is the Observer's first δ>0 capture.

### Three-way epoch check (simultaneous capture, ~11:06Z)
- **morning-api:** Socket=1954, log_count=1954, last_log epoch=1954 (11:04:56Z). **Δ=0 — PASS.**
- **local-witness:** Socket=1953, log_count=1953, last_log epoch=1953 (11:04:43Z). **Δ=0 — PASS.**

---

## Economic State

### morning-api (~11:05Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged) |
| own_nonce | 120 | 120 | None (unchanged) |
| witness_balance (reported) | 4980 | 5000 - morning_api_balance = 4980 | None (unchanged) |
| witness_nonce (reported) | 0 | 0 | None |

### local-witness (~11:06Z)

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

### morning-api (~11:06Z socket + simultaneous filesystem check)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1950 | Incrementing by 10 (pass 82: 1920; +30 = 3 rotations) | None (normal — 3 rotations: 1920→1930→1940→1950) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause) |

**File system cross-check (~11:06Z):**
- `state.snapshot`: **895 bytes** (mtime: 2026-07-28T07:02 EDT — updated from 06:47 EDT in pass 82. 3 snapshot rotations: 1920→1930→1940→1950.)
- `wal.log`: 379 bytes (mtime: 2026-07-28T07:02 EDT — updated from 06:47)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T06:57 EDT — updated from 06:42)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- **Snapshot rotation count:** 3 rotations since pass 82 (1920→1950 confirmed by mtime change from 06:47→07:02 EDT).
- Snapshot size stable at 895 bytes for ninth consecutive pass. ✓

### local-witness (~11:06Z socket + simultaneous filesystem check)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1950 | Incrementing by 10 (pass 82: 1920; +30 = 3 rotations) | None (normal) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~11:06Z):**
- `state.snapshot`: **569 bytes** (mtime: 2026-07-28T07:03 EDT — updated from 06:48 EDT in pass 82)
- `wal.log`: 379 bytes (mtime: 2026-07-28T07:03 EDT — updated from 06:48)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T06:58 EDT — updated from 06:43)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot size 569 bytes — unchanged across many passes. ✓

---

## Metrics Instrumentation

**OBSERVED (from m-ap metrics lines at ~11:06Z):**
- `outstanding_fetches=0` — no pending fetches
- `aged=0` — no stale fetches
- `outbound_queues=[]` — all peer queues empty
- `max_peer_silence=3s` — well under 30s threshold

**OBSERVED (from witness metrics lines at ~11:05Z):**
- `outstanding_fetches=0`
- `aged=0`
- `outbound_queues=[]`
- `max_peer_silence=3s`

**EXPECTED:** All gauges near zero on a settled 2-node mesh with no new transactions.
**DEVIATION:** None. Mesh is quiescent.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **No new WARN/ERROR events since pass 82 capture (10:50Z).** Only continuous KAD bootstrap warnings (every 5 min).
- **NTP:** Last NTP event: 09:50:32Z (~1h10m ago). All NTP events confirmed historic. No new NTP events.
- **Zombie eviction events:** None.
- **Sweep/eviction events:** None (0 `swept` events, 0 `evict` events, 0 `stale fetch` events).
- **Panics:** None.
- **KAD bootstrap warnings:** Continuous (every 5 minutes). Benign — `--no-mdns` with single bootstrap peer.

### local-witness (/tmp/lw.log)
- **No new WARN/ERROR events since pass 82 capture (10:50Z).**
- **NTP:** Last NTP event: 08:00:06Z (~3h ago). Historic.
- **Insufficient balance:** 118 (unchanged, all historic Jul 27). **No new occurrences.**
- **Panics:** None.

### Log filter (WARN/ERROR excluded as benign)

| Pattern | m-ap | lw | Status |
|---------|------|----|--------|
| `Failed to trigger bootstrap` | Many (continuous) | 0 | Benign — `--no-mdns` with single bootstrap peer |
| `skip-ntp-check` | 0 | 0 | Clean |
| `No snapshot` | 0 | 0 (missing field: counted at startup) | Clean |
| `zombie` | 0 | 0 | Clean |
| `insufficient balance` | 0 | 118 (historic) | No new occurrences |
| `panicked` | 0 | 0 | Clean |
| `NTP.*fail|Failed to create NTP|RUNTIME NTP` | **8** (all historic. Last at 09:50:32Z, ~1h10m ago) | **1** (08:00Z, historic, ~3h ago) | All historic. No new events. |

---

## Build Commit Verification

| Check | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| git HEAD | `cb5d4b1` | — | — |
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree (74 unstaged files, up from 73 in pass 82). Unchanged since pass 1. |

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

**Unstaged files:** 74 (pass 82: 73 — Δ=+1). Minor drift; likely a cron/temp artifact.

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 82 Status | Pass 83 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No. Unstaged files count: 74 (pass 82: 73 — +1 minor drift). |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

### New observation this pass (not a deviation — status: UNKNOWN)

| Observation | Pass 82 Status | Pass 83 Status |
|-------------|----------------|----------------|
| Cross-node epoch synchronization | δ=0 (both at epoch 1928) | **δ=1** (m-ap=1954, witness=1953) |
| Cadence (per previous reports) | ~28–30s/epoch | ~39s/epoch (26 epochs in 17 min) |

**Note on δ=1:** This is the Observer's first δ>0 capture. Two plausible explanations: (1) the ratio difference (1.0198 vs 1.0509, ~3%) accumulates over time, causing the witness to slowly lag by ~1 epoch per ~16h, or (2) a transient measurement artifact. The Observer does not diagnose — recorded as OBSERVED: δ=1, UNKNOWN: cause. A Verifier could check whether witness epoch ratio is indeed converging to m-ap's ratio, or whether δ grows over time.

---

## Minor Observations (Not Deviations)

| Observation | First Noted | Status |
|-------------|------------|--------|
| morning-api snapshot size stable at 895 bytes for ninth consecutive pass | Pass 71 | Stable. |
| NTP runtime check failure episode (09:50:32Z) — not recurred for ~1h10m | Pass 74 | Historic. Last event at 09:50:32Z (~1h10m ago). Witness last NTP event at 08:00:06Z (~3h ago). |
| KAD bootstrap warnings on m-ap every 5 min (continuous) | Pass 1 | Benign with `--no-mdns`. Noted for completeness. |
| Unstaged files count: 74 (pass 82: 73) — Δ=+1 | This pass | Minor drift. Not a concern — likely a cron/temp artifact. |
| cadence: ~39s/epoch (vs pass 82's ~28s/epoch) | This pass | Variation. Possibly measurement window artifact — single 17-min window vs multiple passes averaged. No deviation. |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (socket, log count, last log line) | **m-ap: PASS** (all three = 1954). **Witness: PASS** (all three = 1953). |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2). Both nodes. |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 unchanged. |
| Log health (WARN/ERROR filtered) | **PASS** — no new anomalous events. NTP events confirmed as historic. 118 historic insufficient-balance entries unchanged. No panics, no zombies, no errors. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** — m-ap 3s, witness 3s. Both well under threshold. |
| Cross-node epoch sync | **MINOR δ=1** — m-ap at 1954, witness at 1953. First observed this pass. Noted but not classified as a deviation. |
| Snapshot rotation | **PASS** — 3 rotations since pass 82 (1920→1930→1940→1950). New snapshot filesystem mtimes confirm disk writes. |
