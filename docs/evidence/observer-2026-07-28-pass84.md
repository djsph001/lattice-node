# Observer Evidence Record — 2026-07-28 (Pass 84)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T11:14–11:16Z bundle (socket queries + log/metrics)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (Boynton Beach FL)
**Session type:** Eighty-fourth observation pass. Same processes since 2026-07-27T18:48Z (~16.5h runtime). ~8 min since pass 83 (11:04–07Z).

**Summary:** Routine continuation. Both nodes cycling normally. Three persistent deviations unchanged. **Cross-node epoch δ=1 resolved to δ=0** at simultaneous socket capture (both at epoch 1973 at 11:14:41Z; both at 1976 at 11:15:56Z). Log counts show m-ap=1977, witness=1976 (δ=1) due to ratio-driven drift — consistent with pass 83 pattern. One **new NTP event** on m-ap at 11:07:29Z (pool.ntp.org) — first recurrence since 09:50:32Z (~1h17m gap). Zero new WARN/ERROR beyond the NTP event. Snapshot rotation: last_snapshot_epoch advanced from 1950→1970 (1 rotation since pass 83). Single-capture three-way epoch check PASSED on both nodes (socket epoch matches last log line epoch).

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

---

## Node Info

### morning-api (~11:14Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 59174 | — | None (pass 83: 58668; Δ = +506s ≈ 8.4 min — consistent with capture interval) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree. Unchanged since pass 83. |
| thickness | 984.30 | ~985, slowly decaying | None (pass 83: 984.43; Δ = −0.13 over ~8 min — consistent decay rate) |

### local-witness (~11:14Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 59181 | — | None (pass 83: 58663; Δ = +518s ≈ 8.6 min — sequential capture offset) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api (~11:14Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=5916, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~11:14Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=5919, silence_secs=4, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 83 (~11:04Z):** Heartbeats: m-ap +51 (5865→5916 ~6.4/min), witness +52 (5867→5919 ~6.5/min). Both at expected rate. Silence: m-ap 3s (pass 83: 4s), witness 4s (pass 83: 3s). Queue depth 0 on both. **No zombie eviction events. No sweep events.**

---

## Epoch State

### morning-api (~11:16Z socket+log simultaneous capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1976 (endpoint, confirmed in simultaneous capture) | Cycling. +22 from pass 83 (1954→1976) in ~8 min. | None. Normal cadence (~30s/epoch). |
| ratio | 1.01981 | ~1.01–1.02 steady state | None (pass 83: 1.01980; essentially unchanged) |
| tax_calculated | 0 | Balance 20: ~5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (~11:16Z socket+log simultaneous capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1976 (endpoint, confirmed on simultaneous capture) | Cycling. +23 from pass 83 (1953→1976) in ~8 min. | None |
| ratio | 1.05018 | Continuing asymptotic decline toward 1.0 | None (pass 83: 1.05085; continued gradual decline from 1.05085→1.05018) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** Simultaneous socket capture at 11:14:41Z: both at epoch 1973 (δ=0). Simultaneous socket capture at 11:15:56Z: both at epoch 1976 (δ=0). **δ=1 at pass 83 has resolved to δ=0 on this pass's socket captures.**

Log counts (11:16:36Z): m-ap=1977, witness=1976 (δ=1 in log count). This aligns with the ratio-driven drift: m-ap at ~30s/epoch, witness at ~31.5s/epoch (~5% longer). When captured at the same socket-query instant, both read the same epoch; over time the witness accumulates fewer total epochs.

**UNKNOWN:** Whether δ will return to 1 and then back to 0 in a cycle (expected if cadence varies by ~5%) or if δ will grow monotonically. This is the second pass documenting the pattern (first observed δ=1 in pass 83). An Observer cannot distinguish periodic phase drift from monotonic drift without more data.

### Three-way epoch check (simultaneous capture, ~11:16Z)
- **morning-api:** Socket=1976, last_log epoch=1976 (11:15:56Z). **Socket = last_log — PASS.**
- **local-witness:** Socket=1976, last_log epoch=1976 (11:16:13Z). **Socket = last_log — PASS.**

---

## Economic State

### morning-api (~11:14Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged) |
| own_nonce | 120 | 120 | None (unchanged) |
| witness_balance (reported) | 4980 | 5000 - morning_api_balance = 4980 | None (unchanged) |
| witness_nonce (reported) | 0 | 0 | None |

### local-witness (~11:14Z)

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

### morning-api (~11:14Z socket + simultaneous filesystem check)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1970 | Incrementing by 10 (pass 83: 1950; +20 = 2 rotations) | None (normal — 2 rotations: 1950→1960→1970) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause) |

**File system cross-check (~11:14Z):**
- `state.snapshot`: **894 bytes** (mtime: 2026-07-28T07:12 EDT — updated from 07:02 EDT in pass 83. 2 snapshot rotations: 1950→1970.)
- `wal.log`: 379 bytes (mtime: 2026-07-28T07:12 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T07:07 EDT — updated from 06:57 EDT in pass 83)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- **Snapshot rotation count:** 2 rotations since pass 83 (1950→1970 confirmed by mtime change from 07:02→07:12 EDT).
- Snapshot size: 894 bytes (pass 83: 895 bytes — Δ=-1 byte. Minor; probably a rounding change or serialization artifact.)

### local-witness (~11:14Z socket + simultaneous filesystem check)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1970 | Incrementing by 10 (pass 83: 1950; +20 = 2 rotations) | None (normal) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~11:14Z):**
- `state.snapshot`: **569 bytes** (mtime: 2026-07-28T07:13 EDT — updated from 07:03 EDT in pass 83)
- `wal.log`: 379 bytes (mtime: 2026-07-28T07:13 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T07:08 EDT — updated from 06:58 EDT in pass 83)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot size 569 bytes — unchanged. ✓

---

## Metrics Instrumentation

**OBSERVED (from m-ap metrics lines at ~11:15Z):**
- `outstanding_fetches=0` — no pending fetches
- `aged=0` — no stale fetches
- `outbound_queues=[]` — all peer queues empty
- `max_peer_silence=3s` — well under 30s threshold

**OBSERVED (from witness metrics lines at ~11:15Z):**
- `outstanding_fetches=0`
- `aged=0`
- `outbound_queues=[]`
- `max_peer_silence=6s` — slightly higher than m-ap but well under 30s threshold

**EXPECTED:** All gauges near zero on a settled 2-node mesh with no new transactions.
**DEVIATION:** None. Mesh is quiescent.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **New NTP event at 11:07:29Z** since pass 83 capture (~11:04Z). First NTP recurrence since 09:50:32Z (~1h17m gap). pool.ntp.org: "Input/output error: Resource temporarily unavailable (os error 11) (fallback)".
- **Total NTP WARN events:** 4 (`grep -c "NTP query"`). Total NTP-related lines (all levels): 11 (`grep -ci ntp`).
- **No other new WARN/ERROR events.** KAD bootstrap warnings continue every 5 min (benign).
- **Zombie eviction events:** None.
- **Sweep/eviction events:** None.
- **Panics:** None.

### local-witness (/tmp/lw.log)
- **No new WARN/ERROR events since pass 83** (last: 08:00:06Z NTP query failure, ~3h14m ago).
- **Total NTP WARN events:** 1 (unchanged).
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
| `NTP.*fail\|Failed to create NTP\|RUNTIME NTP\|NTP query` | **4** (including new event at 11:07:29Z) | **1** (08:00Z, historic) | **New event on m-ap at 11:07:29Z** — first recurrence since 09:50:32Z. Witness unchanged. |

---

## Build Commit Verification

| Check | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| git HEAD | `cb5d4b1` | — | — |
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree. Unchanged since pass 83. |

**Unstaged files:** 75 (pass 83: 74 — Δ=+1). Minor drift; likely a cron/temp artifact.

**Note:** All 9 commits between binary and HEAD are docs/tests/fixes — no wire-format changes. The running binary's stale build commit is not a functional safety risk.

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 83 Status | Pass 84 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No. Unstaged files: 75 (pass 83: 74 — +1 minor drift). |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

### Previous observation carried forward: cross-node epoch δ

| Observation | Pass 83 Status | Pass 84 Status |
|-------------|----------------|----------------|
| Cross-node epoch synchronization | δ=1 (m-ap=1954, witness=1953) socket capture at 11:04Z | **δ=0 on both simultaneous socket captures** (11:14:41Z: both=1973; 11:15:56Z: both=1976). δ=1 in log counts (m-ap=1977, witness=1976) at 11:16:36Z. |

**OBSERVED:** When captured at the exact same socket-query instant, both nodes report the same epoch. Over time, the witness accumulates fewer epochs due to its longer epoch cadence (~5% by ratio: 1.050 vs 1.020). The δ=1 from pass 83 was resolved by this pass's simultaneous socket captures showing δ=0. The mismatch between socket (δ=0) and log counts (δ=1) is consistent with ratio-driven drift — the log counts reflect cumulative epochs completed (which differs), while the socket reflects the current in-progress epoch (which may or may not be the same at the instant of query).

**UNKNOWN:** Whether this pattern will continue indefinitely (periodic δ=0/δ=1 alternation) or if δ will diverge further. Not investigated — this is observation, not diagnosis.

---

## Minor Observations (Not Deviations)

| Observation | First Noted | Status |
|-------------|------------|--------|
| morning-api snapshot size: 894 bytes (pass 83: 895 — Δ=-1) | This pass | Minor byte change. Possibly a rounding or serialization artifact. Not a deviation. |
| morning-api snapshot stable at 894–895 bytes across many passes | Pass 71 | Stable. |
| NTP runtime check failure recurred at 11:07:29Z on m-ap (pool.ntp.org) | This pass | New event since pass 83. First recurrence after ~1h17m gap (previous: 09:50:32Z). Witness last NTP event at 08:00:06Z (~3h16m ago). |
| KAD bootstrap warnings on m-ap every 5 min (continuous) | Pass 1 | Benign with `--no-mdns`. Noted for completeness. |
| Unstaged files count: 75 (pass 83: 74) — Δ=+1 | This pass | Minor drift. Not a concern — likely a cron/temp artifact. |
| Cross-node epoch δ=0 on simultaneous socket capture (both at 1973, then both at 1976) | This pass | Resolved from δ=1 at pass 83. Consistent with ratio-driven drift pattern — δ varies between 0 and 1 depending on capture timing relative to each node's epoch boundary. |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (socket vs last log line) | **m-ap: PASS** (socket=1976, last_log=1976). **Witness: PASS** (socket=1976, last_log=1976). |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2). Both nodes. |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 unchanged. |
| Log health (WARN/ERROR filtered) | **PASS** — one new NTP event on m-ap (11:07:29Z) but no other anomalies. 118 historic insufficient-balance entries unchanged. No panics, no zombies, no errors. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** — m-ap 3s, witness 6s. Both well under threshold. |
| Cross-node epoch sync (simultaneous socket capture) | **PASS** (δ=0 at 11:14:41Z and 11:15:56Z captures). |
| Snapshot rotation | **PASS** — 2 rotations since pass 83 (1950→1970). New snapshot filesystem mtimes confirm disk writes. |
