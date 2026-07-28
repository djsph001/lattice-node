# Observer Evidence Record — 2026-07-28 (Pass 100)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** ~2026-07-28T20:32Z bundle (single-capture discipline)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (Boynton Beach FL)
**Session type:** 100th observation pass. **Session restart detected** — this is a NEW session, NOT a continuation of pass 99.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

### Running Processes (4 lattice-node, 2 distinct sessions)

| PID | Name | Started (UTC) | Build | Port | Genesis | Status |
|-----|------|---------------|-------|------|---------|--------|
| **3946739** | **morning-api** | **20:07** | **9f604ed-dirty** | **4005** | **auto (new)** | **ACTIVE — new session** |
| **3947000** | **local-witness** | **20:08** | **9f604ed-dirty** | **4010** | **12D3KooWPfrZ...zLVxJ** | **ACTIVE — new session** |
| 3579821 | local-witness (STALE) | 09:02 | cb5d4b1-dirty | 4010 | same | **STALE — zombie from pass 99** |

### Socket Map
| Socket Path | Served By | Notes |
|-------------|-----------|-------|
| `/tmp/m-ap/lattice.sock` | PID 3946739 (new morning-api) | Clean |
| `/tmp/local-witness/lattice.sock` | PID 3947000 (new local-witness) | Clean |
| (no socket) | PID 3579821 (stale witness) | Stale — no UDS socket visible, but bound to TCP 4010 |

### Topology Change from Pass 99

**RECONFIGURATION:** The mesh was rebuilt and restarted between pass 99 (14:42Z) and this pass (20:32Z). Evidence:
1. build_commit changed: `cb5d4b1-dirty` → `9f604ed-dirty`
2. Uptime dropped from 6015s to 1456s
3. PIDs changed: 3579452/3579821 → 3946739/3947000
4. morning-api port 4005 now served by new PID only. Stale morning-api at PID 3579452 is GONE.
5. Evidence directory shows no observer passes between ~14:42Z and ~20:32Z (~5h50m gap).

### STALE PROCESS CONTAMINATION

PID 3579821 (old local-witness, cb5d4b1-dirty) survived the restart. It remains bound to TCP port 4010 alongside the new witness (PID 3947000). Evidence:
- `ss -tlnp` shows BOTH PIDs on 0.0.0.0:4010 (SO_REUSEPORT or fallback port behavior)
- Old witness epoch log shows epoch=898, ratio=1.13, `redistributed_to=0`, `"n/a (no peers)"` — isolated from all peers
- Both processes write to `/tmp/lw.log`, making grep-based analysis of the NEW witness unreliable (log contamination)
- Uptime: 7h27m (09:02Z to 20:30Z)

**DEVIATION (NEW):** Stale local-witness process survived across sessions. Log contamination prevents three-way epoch verification on local-witness.

---

## Node Info (Fresh Session — No Delta from Prior Pass)

| Field | OBSERVED | DEVIATION |
|-------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | None — stable from identity key |
| name | `morning-api` | None |
| genesis_root_id | `auto` | Consistent with --auto-genesis |
| chain_tip | 1 | None (genesis-only, no blocks) |
| uptime_secs | 1456 (~24 min) | None — matches ~20:07Z start |
| build_commit | `9f604ed-dirty` | **Persistent DEVIATION (#1).** 2 commits behind HEAD (`66af9f7`) + dirty working tree. **Changed from pass 99** (was `cb5d4b1-dirty`), but still stale. |
| thickness | ~975.60 | None — slight decay from ~981 (pass 99) is expected on fresh start with different parameters |

### Build Commit Detail

Git HEAD: `66af9f7 docs: Verifier audit — peer-set exchange and request-response infrastructure`
Running commit: `9f604ed feat: GetStateRoot API endpoint — expose state_root hash with epoch` (2 behind)
Dirty suffix: working tree had uncommitted changes at build time

The new binary includes the GetStateRoot endpoint (confirmed: endpoint responds). Still 2 commits behind HEAD.

---

## Epoch State (Single Capture ~20:32Z)

### morning-api

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 49 | — |
| Log count (grep -c) | 50 | **Boundary race** — socket queried before epoch 50 completed |
| Last log epoch | 50 (20:32:13Z) | — |
| Three-way equality | Socket=49, count=50, last_log=50. **Boundary race at epoch boundary** — not a protocol deviation. | None — race at boundary |

Epochs completed so far this session: **50** in ~24 min ≈ 2.1/min (~29s/epoch). Consistent with 30s epoch_interval.

### local-witness

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 48 | — |
| Log count (grep -c) | **Cannot verify** — log contaminated by stale PID 3579821 | **DEVIATION (#5, NEW):** Log contamination. `grep -c "Epoch complete" /tmp/lw.log` returns 88 (both old and new process epochs combined). Witness log shows binary content. |
| Three-way equality | **FAIL — cannot verify** | See deviation #5 |

### Cross-Node Convergence

| Metric | morning-api | local-witness | Δ |
|--------|-------------|---------------|---|
| Epoch | 49 | 48 | **δ=1** (propagation lag or query timing) |
| Peers | 1 (12D3KooWD...9sch) | 1 (12D3KooWP...LVxJ) | Symmetric — healthy |
| Heartbeats | 144 | 141 | δ=3 — expected from different query times |

---

## Peer Connections

### morning-api

| Metric | OBSERVED | DEVIATION |
|--------|----------|-----------|
| Peer count | 1 (12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch) | None — 2-node mesh |
| Heartbeats received | 144 | Consistent with ~10s interval for ~24 min |
| Silence (peer→us) | 8s | Well under 30s zombie threshold |
| Queue depth | 0 | None — no backpressure |
| is_dead | false | Healthy |

### local-witness

| Metric | OBSERVED | DEVIATION |
|--------|----------|-----------|
| Peer count | 1 (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | None |
| Heartbeats received | 141 | Consistent |
| Silence (peer→us) | 3s | Healthy |
| Queue depth | 0 | None |
| is_dead | false | Healthy |

**Heartbeat log lines (morning-api):** 145 total. Pattern: "Heartbeat received from=local-witness peer=12D3KooWD... total_heartbeats=N" — incrementing every ~10s. First at 20:08:05Z, last at 20:31:55Z. **Flow healthy.**

**No zombie evictions.** No `max_peer_silence` exceeded 30s in any metrics line.

---

## Economic State

| Metric | morning-api OBSERVED | local-witness OBSERVED | DEVIATION |
|--------|---------------------|----------------------|-----------|
| own_balance | **431** | **0** | **Persistent DEVIATIONS (#3/#4)** |
| own_nonce | **291** | **6** | Nonce 6 on witness = 6 local operations |
| peer_balance | 14,569 | **0** | Supply divergence |
| peer_nonce | 0 | 0 | — |
| **Total supply (local books)** | **15,000** | **0** | Divergence |

**Total supply calculation:** morning-api own_balance (431) + morning-api's peer_balance for witness (14,569) = **15,000 DUU**. Started with --mint 5000.

**Deviation (#3) unchanged pattern:** morning-api books show 15,000 DUU total vs 5,000 minted genesis. Same mechanism as pass 97-99 — redistribution tax debits morning-api and credits witness on morning-api's local books, but the credit never reaches witness's ledger. Redistribution runs locally, never gossiped.

**Deviation (#4) unchanged:** witness sees morning-api balance as 0. Witness own_balance=0. No path exists for witness to learn morning-api's balance or receive redistribution credits.

**Non-coincident capture note:** morning-api and local-witness were queried ~30s apart (20:31:30Z vs 20:32:00Z). Epoch-state values may differ by 1-2 epochs between captures. Economic state reported independently per node.

### State Root (New Endpoint)

| Node | Epoch | State Root |
|------|-------|------------|
| morning-api | 49 | `faba16893f6ea9d323399c1dbcf8feda9aadde6ec609ccbcc30b41a6fd9b07d8` |
| local-witness | 48 | `4bd573e62bc2531b833c6faa1d5a91c64c64fa2b2d3ec219810160a176ad5654` |

Different epochs, different roots. Per Mission A finding, combined balance+nonce hash diverges even when balances match. The GetStateRoot endpoint works but cannot serve as convergence detection without modification (balance-only hash).

---

## Persistence State

### morning-api (Single Capture ~20:32Z)

| Field | OBSERVED | DEVIATION |
|-------|----------|-----------|
| last_snapshot_epoch | **50** | None — rotated from 40 to 50 during this pass |
| wal_bytes | 379 | None — WAL trimmed by snapshot |
| wal_entries | 3 | None — genesis re-seed only |

**Byte-equality:** `GetPersistenceState.wal_bytes=379`. `ls -la wal.log: 379 bytes`. `wc -c wal.log: 379 bytes`. **PASS.**

**Snapshot rotation observed:** During this pass, last_snapshot_epoch advanced from 40 → 50. The snapshot file was rewritten (mtime 16:32 EDT = 20:32Z, 899 bytes). WAL went from 4311 bytes (35 entries) at epoch 49 to 379 bytes (3 entries) at epoch 50 — snapshot consumed the transaction history.

**File inventory (20:32Z):**
| File | Size | mtime (EDT) | Notes |
|------|------|-------------|-------|
| `state.snapshot` | 899 bytes | 16:32 (20:32Z) | Fresh at epoch 50 |
| `wal.log` | 379 bytes | 16:32 (20:32Z) | Genesis re-seed |
| `wal.wal.old` | 4752 bytes | 16:32 (20:32Z) | Pre-rotation backup |

### local-witness (Single Capture ~20:32Z)

| Field | OBSERVED | DEVIATION |
|-------|----------|-----------|
| last_snapshot_epoch | **40** | None — next rotation at 50 (2 epochs away) |
| wal_bytes | 379 | None |
| wal_entries | 3 | None |

**Byte-equality:** `GetPersistenceState.wal_bytes=379`. `ls -la wal.log: 379 bytes`. **PASS.**

**File inventory (20:32Z):**
| File | Size | mtime (EDT) | Notes |
|------|------|-------------|-------|
| `state.snapshot` | 569 bytes | 16:31 (20:31Z) | Snapshot at epoch 40 |
| `wal.log` | 379 bytes | 16:31 (20:31Z) | Genesis re-seed |
| `wal.wal.old` | 379 bytes | 16:27 (20:27Z) | Pre-rotation backup |

---

## Metrics (Last Lines from Log)

### morning-api
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=7s
```
All clean. Same pattern as pass 97-99. No stale fetches, no queue buildup.

### local-witness
(Cannot extract from log — contaminated by stale PID 3579821.)

Socket query reports: queue_depth=0, silence_secs=3, is_dead=false. Metrics healthy from endpoint perspective.

---

## Log Health

### morning-api (/tmp/m-ap.log)

- **50 Epoch complete** lines (consistent with epoch count — boundary race excluded)
- **1 NTP failure at startup:** `WARN lattice_node::startup: NTP query to pool.ntp.org failed: Input/output error: Resource temporarily unavailable (os error 11) (fallback)` — at 20:07:43Z. Transient. Node continued without NTP.
- **5 KAD bootstrap WARNs:** `Failed to trigger bootstrap: No known peers.` — every 5 min. Expected with --no-mdns.
- **Panics: 0. Zombie evictions: 0. Non-benign WARN/ERROR: 0.**

**Filtered WARN/ERROR count: 6.** All 6 are the benign NTP failure (1) and KAD bootstrap (5) entries above.

### local-witness (/tmp/lw.log)

**CANNOT VERIFY.** Log contaminated by stale PID 3579821 (old binary, cb5d4b1-dirty). The old witness has been writing to the same log file for 11+ hours. `grep` reports binary content. Evidence from socket queries suggests the new witness is healthy (peers connected, heartbeats flowing, queue_depth=0), but log-level analysis is unreliable.

**DEVIATION (#5, NEW):** Witness log contamination from stale process. New witness log analysis impaired.

---

## NTP Series

| Pass | Outcome | Notes |
|------|---------|-------|
| Pass 99 | Last observed: no NTP entry (session was running since 13:01Z) | No fresh NTP check in that window |
| This pass (100) | **FAIL** — `pool.ntp.org` query failed at 20:07:43Z (EAGAIN os error 11) | Node started with `--skip-ntp-check` not in flags, fell back after failure |

**NTP failure rate (last 6 passes):** 1/1 observed (100%). Only 1 pass of data available for this session. No prior pass data from this session exists (fresh session). Trand cannot be established with a single sample.

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch (m-ap) | **BOUNDARY RACE** — socket=49, count=50, last_log=50. Race at epoch transition. Non-deviant. |
| Three-way epoch (witness) | **FAIL — cannot verify** — log contaminated by stale PID 3579821 |
| Byte-equality (m-ap) | **PASS** — 379=379 |
| Byte-equality (witness) | **PASS** — 379=379 |
| PID freshness | **PARTIAL FAIL** — new morning-api/witness PIDs correct, but stale witness PID 3579821 survives |
| Log health (m-ap) | **PASS** — 6 benign WARNs (1 NTP, 5 KAD), no errors |
| Log health (witness) | **FAIL — cannot verify** — log contaminated |
| Metrics health (m-ap) | **PASS** — aged=0, queues=[], silence<30s |
| Metrics health (witness) | **Partially verifiable** — queue_depth=0 from socket, silence<30s, but log metrics unreachable |
| Snapshot rotation (m-ap) | **PASS** — rotated at epoch 50 during this pass |
| Snapshot rotation (witness) | **PASS** — at epoch 40, next at 50 |
| MESH.md accuracy | **STALE** — says "No production nodes running" (Jul 27 update). Does not reflect current running session. |

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Status Since Pass 99 | Changed? |
|---|-----------|----------------|---------------------|----------|
| 1 | `build_commit` stale (`9f604ed-dirty`, HEAD `66af9f7`) | Jul 27 pass 1 | Persistent (2 behind + dirty) | **Changed** — was `cb5d4b1-dirty` in pass 99, now `9f604ed-dirty` (different binary rebuild). Still stale. |
| 2 | `wal_bytes` returns 0 (legacy path) | Jul 27 pass 1 | **RESOLVED** since cb5d4b1 | Unchanged — fix working |
| 3 | Supply divergence (total=15,000 vs 5,000 minted) | Pass 97 (14:08Z) | Persistent (15,000 on m-ap books, 0 on witness) | **Changed** — was 10,000 total in pass 99, now 15,000. Different session parameters. Pattern same. |
| 4 | Witness reports morning-api balance as 0 | Jul 27 (18:48Z) | Persistent (0 vs ~431 actual) | **Changed** — was 0 vs ~20 in pass 99, now 0 vs ~431. Pattern same. |
| **5** | **Witness log contaminated by stale PID 3579821** | **This pass (20:32Z)** | **NEW** | **First observation.** Stale local-witness (started 09:02Z, old binary) survived restart. Both old and new write to same log file. |

---

## Summary

**Pass 100: Session restart detected. 1 new deviation. Supply divergence persists with different numbers.**

The mesh was rebuilt between pass 99 and pass 100:
- New binary at `9f604ed-dirty` (includes GetStateRoot endpoint)
- morning-api restarted at ~20:07Z with `--mint 5000`
- local-witness restarted at ~20:08Z with `--mint 0`
- Both using the same storage dirs (/tmp/m-ap, /tmp/local-witness)

**What's healthy:**
- 2-node mesh fully connected, bidirectional heartbeats flowing at ~10s cadence
- Epochs cycling at ~29-30s, converged within 1 epoch across nodes
- Metrics clean: zero fetches, zero queues, max_peer_silence < 10s
- Persistence functional: snapshot rotation at epoch 50, byte-equality verified
- GetStateRoot endpoint responds correctly

**What's changed:**
- Supply divergence total = 15,000 DUU (was 10,000) — same pattern, different session
- morning-api balance decaying from 5000 through redistribution: currently 431 DUU
- build_commit advanced from cb5d4b1 to 9f604ed but still dirty + 2 behind HEAD

**What's new (Deviation #5):**
- Stale local-witness (PID 3579821, running since 09:02Z, old binary) survived the restart
- Log contamination makes witness log analysis unreliable
- Old witness is isolated (no peers) but still cycling epochs at ratio=1.13
- Root cause: `pkill -9 -f "lattice-node --name morning-api"` killed only morning-api, not the witness

**MESH.md is stale.** Last updated Jul 27. Says "No production nodes running." Does not reflect current active mesh.

**Next expected event:** Snapshot rotation on local-witness at epoch 50 (~2 epochs / ~1 min from now). No other state changes expected in the current quiescent state.
