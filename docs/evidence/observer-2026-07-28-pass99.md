# Observer Evidence Record — 2026-07-28 (Pass 99)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** ~2026-07-28T14:42Z bundle (single-capture discipline)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (Boynton Beach FL)
**Session type:** 99th observation pass. Same session as pass 97-98 (same PIDs, no restart). Mesh running since ~13:01Z (~1h41min uptime).

**Summary:** Delta-only pass from pass 98. Mesh is stable and quiescent. Epochs advanced 172→201 (+29). No new deviations. Snapshot rotated at epoch 200 on both nodes. All economic state frozen (balances, nonces unchanged). Metrics clean. Build commit unchanged.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZ...zLVxJ) | 13:01 | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZ...zLVxJ | 13:02 | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZ...zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZ...zLVxJ --no-mdns --persistence --mint 0` |

**Topology change from pass 98:** None. Same PIDs, same nodes, same session.

---

## Node Info (Delta from Pass 98)

| Field | Pass 98 (14:28Z) | This pass (14:42Z) | Δ | DEVIATION |
|-------|-------------------|--------------------|---|-----------|
| uptime_secs (m-ap) | 5140 | 6015 | +875s (~14.6 min real) | None — matches clock |
| uptime_secs (witness) | 5150 | 6004 | +854s (~14.2 min) | None — witness query ~10s earlier |
| build_commit (both) | `cb5d4b1-dirty` | `cb5d4b1-dirty` | Unchanged | **Persistent DEVIATION (#1).** 2 commits behind HEAD `452b64f` + dirty. Unchanged since pass 1. |
| thickness (m-ap) | ~981.31 | 981.08 | -0.23 (expected decay) | None |

---

## Epoch State (Single Capture ~14:42Z)

### morning-api

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 201 | — |
| Log count (grep -c) | 201 | — |
| Last log epoch | 201 (14:41:47Z) | — |
| Three-way equality | **MATCH** — socket=201, count=201, last_log=201. No boundary race. | None |

### local-witness

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 201 | — |
| Log count (grep -c) | 201 | — |
| Last log epoch | 201 (14:42:10Z) | — |
| Three-way equality | **MATCH** — socket=201, count=201, last_log=201. No boundary race. | None |

**Delta from pass 98:**

| Metric | Pass 98 (14:28Z) | Pass 99 (14:42Z) | Δ |
|--------|-------------------|-------------------|---|
| morning-api epoch | 172 | 201 | +29 epochs |
| witness epoch | 172 | 201 | +29 epochs |
| Cross-node δ | 0 | 0 | **Unchanged** (fully converged) |
| Epoch rate | ~1.95/min | ~2.07/min (~29s/epoch) | Consistent |

---

## Peer Connections

| Metric | Pass 98 (14:28Z) | Pass 99 (14:42Z) | Δ |
|--------|-------------------|-------------------|---|
| m-ap: heartbeats | 512 | 599 | +87 (~6/min) |
| m-ap: silence_secs | 0 | 4 | Healthy drift |
| witness: heartbeats | 515 | 600 | +85 (~6/min) |
| witness: silence_secs | 6 | 9 | Healthy drift |
| queue_depth (both) | 0 | 0 | Unchanged |

**OBSERVED:** Both nodes still see exactly 1 peer. Heartbeats flowing at expected rate. Silence well under 30s threshold. No zombie evictions. No backpressure.

---

## Economic State

| Metric | Pass 98 (14:28Z) | Pass 99 (14:42Z) | Δ |
|--------|-------------------|-------------------|---|
| m-ap: own_balance | 20 | 20 | **Frozen** (no change) |
| m-ap: own_nonce | 241 | 241 | **Frozen** (no new transactions) |
| m-ap: witness balance | 9980 | 9980 | **Frozen** |
| Witness: own_balance | 0 | 0 | **Frozen** |
| Witness: own_nonce | 4 | 4 | **Frozen** |
| Witness: m-api balance | 0 | 0 | **Frozen** |
| m-ap total supply | 10,000 | 10,000 | **Frozen** |
| Insufficient-balance (lifetime) | 119 | 119 | **No new rejections** |

**OBSERVED:** Economic state fully frozen since pass 97. No activity on either node. Balance 20 floor at epoch 201 (balance_before=20, balance_after=20, ratio=1.02). No new insufficient-balance events.

**Persistent DEVIATIONS (#3/#4):** Unchanged. morning-api ledger shows 10,000 DUU total (vs 5,000 minted). Witness sees morning-api balance as 0.

---

## Persistence State

### morning-api (single-capture ~14:42Z)

| Field | Pass 98 (14:28Z) | Pass 99 (14:42Z) | Δ |
|-------|-------------------|-------------------|---|
| last_snapshot_epoch | 170 | **200** | Rotated at epoch 200 |
| wal_bytes | 379 | 379 | Unchanged |
| wal_entries | 3 | 3 | Unchanged |

**Byte-equality:** `GetPersistenceState.wal_bytes=379`. `ls -la persistence/wal.log=379 bytes`. **PASS.**

**File inventory (14:42Z):**
| File | Size | mtime (EDT) | Notes |
|------|------|-------------|-------|
| `state.snapshot` | 895 bytes | 10:41 (14:41Z) | Snapshot at epoch 200 |
| `wal.log` | 379 bytes | 10:41 (14:41Z) | Active WAL (genesis re-seed) |
| `wal.wal.old` | 379 bytes | 10:36 (14:36Z) | Pre-rotation backup |

### local-witness

| Field | Pass 98 (14:28Z) | Pass 99 (14:42Z) | Δ |
|-------|-------------------|-------------------|---|
| last_snapshot_epoch | 170 | **200** | Rotated at epoch 200 (matches m-ap) |
| wal_bytes | 379 | 379 | Unchanged |
| wal_entries | 3 | 3 | Unchanged |

**Byte-equality:** `GetPersistenceState.wal_bytes=379`. `ls -la persistence/wal.log=379 bytes`. **PASS.**

**File inventory (14:42Z):**
| File | Size | mtime (EDT) | Notes |
|------|------|-------------|-------|
| `state.snapshot` | 569 bytes | 10:41 (14:41Z) | Snapshot at epoch 200 |
| `wal.log` | 379 bytes | 10:41 (14:41Z) | Active WAL |
| `wal.wal.old` | 379 bytes | 10:36 (14:36Z) | Pre-rotation backup |

**Key observation:** Both nodes rotated snapshot at epoch 200 simultaneously. wal.wal.old went from 4742 bytes (pass 97) to 379 bytes (pass 98) to 379 bytes (pass 99) — no new transactions persisted in that window. state.snapshot size unchanged from pass 98 on both nodes.

---

## Metrics (Last Lines from Log)

### morning-api
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```
All clean. Same pattern since pass 97.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
```
All clean. Same pattern since pass 97.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **201 Epoch complete** lines (matches socket and count: PASS)
- **KAD bootstrap WARNs:** Last at 14:41:48Z. Expected (--no-mdns, no DHT peers). 5-min cadence.
- **Snapshot saved epoch=200** confirmed at 14:41:17Z.
- **Panics: 0. Zombie evictions: 0. Non-KAD WARN/ERROR: None.**

### local-witness (/tmp/lw.log)
- **201 Epoch complete** lines (matches socket and count: PASS)
- **119 insufficient-balance** events (unchanged — no new rejections)
- **Panics: 0. Zombie evictions: 0. Non-KAD WARN/ERROR: None.**

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch (m-ap) | **PASS** — socket=201, count=201, last_log=201 |
| Three-way epoch (witness) | **PASS** — socket=201, count=201, last_log=201 |
| Byte-equality (m-ap) | **PASS** — 379=379 |
| Byte-equality (witness) | **PASS** — 379=379 |
| PID consistency | **PASS** — 3579452/3579821 unchanged since pass 97 |
| Log health | **PASS** — KAD WARNs expected, 119 historical insufficient-balance, no new errors |
| Metrics health | **PASS** — aged=0, queues=[], silence<30s |
| Cross-node epoch sync | **PASS** — both at 201 (δ=0) |
| Snapshot rotation | **PASS** — both rotated at epoch 200, files present on disk |

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Status Since Pass 98 | Changed? |
|---|-----------|----------------|---------------------|----------|
| 1 | `build_commit` stale (`cb5d4b1-dirty`, HEAD `452b64f`) | Jul 27 pass 1 | Persistent (2 behind + dirty) | **Unchanged** |
| 2 | `wal_bytes` returns 0 (legacy path) | Jul 27 pass 1 | **RESOLVED** | **Unchanged** — fix working |
| 3 | Supply divergence (total=10,000 vs 5,000 minted) | Pass 97 (14:08Z) | Persistent (10,000 on m-ap books) | **Unchanged** |
| 4 | Witness reports morning-api balance as 0 | Jul 27 (18:48Z) | Persistent (0 vs ~20 actual) | **Unchanged** |

---

## Summary

**Pass 99: delta-only. No new deviations.**

The mesh remains in the same frozen steady state observed since pass 97:
- 2 nodes, 1 peer each, bidirectional heartbeats healthy
- Epochs cycling at ~30s cadence, both fully converged (δ=0)
- No transactions flowing (nonces frozen on both nodes)
- Balance 20 floor on morning-api, zero on witness
- Snapshot rotation at epoch 200 completed successfully on both nodes
- All metrics clean: zero fetches, zero queues, max_peer_silence < 10s
- Build commit 2 commits behind HEAD + dirty (unchanged since session start)
- Supply conservation divergence unchanged (documented, pending governance)

**Next expected event:** Snapshot rotation at epoch 210 (~5 min from now). No other state changes expected in the current quiescent state.
