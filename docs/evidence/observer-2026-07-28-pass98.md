# Observer Evidence Record — 2026-07-28 (Pass 98)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T14:28Z bundle (single-capture discipline)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (Boynton Beach FL)
**Session type:** 98th observation pass. Same session as pass 97 (no restart). Mesh restarted between pass 96 and 97; pass 97 was first in this session.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 2026-07-28T13:01Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 2026-07-28T13:02Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**Topology change since pass 97:** None. Same PIDs, same nodes, same session.

---

## Node Info

### morning-api (~14:28Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZ...zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 5140 (~86 min) | — | None |
| build_commit | `cb5d4b1-dirty` | git HEAD `452b64f` | **Persistent DEVIATION (#1).** Running binary stale by 2 commits (0c4bb7f fix + 452b64f docs). Unchanged since pass 97. |
| thickness | ~981.31 | Slowly decaying | None (slight decay: 981.8→981.3, expected) |

### local-witness (~14:28Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZ...9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZ...zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 5150 (~86 min) | — | None |
| build_commit | `cb5d4b1-dirty` | git HEAD `452b64f` | **Persistent DEVIATION (#1).** Same as morning-api. Unchanged. |

### Build gap — unchanged since pass 97

| Check | Pass 97 (14:08Z) | Pass 98 (14:28Z) | Δ |
|-------|-------------------|-------------------|---|
| git HEAD | `452b64f` | `452b64f` | Unchanged |
| running binary | `cb5d4b1-dirty` | `cb5d4b1-dirty` | Unchanged |
| gap | 2 commits behind | 2 commits behind | Unchanged |
| -dirty uncommitted | Present | Present | Unchanged |

**OBSERVED:** Binary unchanged since pass 97. No rebuild. `-dirty` composition unknown (same docs/evidence edit as before).

---

## Epoch State

### morning-api (~14:28Z bundle)

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 172 | — |
| Log count (grep -c) | 172 | — |
| Last log epoch | 172 (14:27:17Z) | — |
| Three-way equality | **MATCH** — socket=172, count=172, last_log=172. No boundary race. | None |

### local-witness (~14:28Z bundle)

| Check | OBSERVED | DEVIATION |
|-------|----------|-----------|
| Socket epoch | 172 | — |
| Log count (grep -c) | 172 | — |
| Last log epoch | 172 (14:27:40Z) | — |
| Three-way equality | **MATCH** — socket=172, count=172, last_log=172. No boundary race. | None |

**Delta from pass 97:**

| Metric | Pass 97 (14:08Z) | Pass 98 (14:28Z) | Δ |
|--------|-------------------|-------------------|---|
| morning-api epoch | 135 | 172 | +37 epochs |
| witness epoch | 135 | 172 | +37 epochs |
| Cross-node δ | 0 | 0 | Unchanged (fully converged) |
| Epoch rate | ~2.0/min | ~1.95/min (~30.8s/epoch) | Consistent |

**OBSERVED:** Both nodes at epoch 172, fully converged (δ=0). 37 epochs elapsed in ~19 minutes = ~1.95/min, consistent with 30s epoch cadence.

---

## Peer Connections

### morning-api (~14:28Z)

**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=512, silence_secs=0, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~14:28Z)

**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=515, silence_secs=6, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 97:**

| Metric | Pass 97 (14:08Z) | Pass 98 (14:28Z) | Δ |
|--------|-------------------|-------------------|---|
| m-ap: heartbeats | 345 | 512 | +167 in ~19 min (~8.8/min) |
| m-ap: silence_secs | 7 | **0** | Improved — fresh heartbeat just arrived |
| witness: heartbeats | 345 | 515 | +170 in ~19 min |
| witness: silence_secs | 7 | 6 | Unchanged (healthy drift) |
| queue_depth (both) | 0 | 0 | Unchanged |

**OBSERVED:** Mesh connectivity healthy. Heartbeats flowing at expected rate (~6.8s interval). Silence well under 30s threshold. No zombie evictions. No backpressure.

---

## Economic State

### morning-api (~14:28Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | Initially 5000 (--mint 5000), decreasing by epoch tax | None (stabilized at floor — no change since pass 97) |
| own_nonce | 241 | — | None (unchanged — no new transactions) |
| witness balance (reported) | 9980 | — | **Persistent DEVIATION (#3) — UNCHANGED.** Total supply = 10,000. Same as pass 97. |

### local-witness (~14:28Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 (--mint 0, no redistribution confirmed) | None |
| own_nonce | 4 | — | None (unchanged — no new transactions) |
| morning-api balance (reported) | 0 | 5000 | **Persistent DEVIATION (#4).** Witness sees morning-api balance as 0. First observed: Jul 27 pass 1. Unchanged. |

### Supply divergence — unchanged since pass 97

| Metric | Pass 97 (14:08Z) | Pass 98 (14:28Z) | Δ |
|--------|-------------------|-------------------|---|
| morning-api: own_balance | 20 | 20 | Unchanged |
| morning-api: reports witness balance | 9980 | 9980 | Unchanged |
| Total supply (m-api view) | 10,000 | 10,000 | Unchanged |
| Witness: own_balance | 0 | 0 | Unchanged |
| Witness: reports m-api balance | 0 | 0 | Unchanged |
| Total supply (witness view) | 0 | 0 | Unchanged |
| Insufficient-balance events (lifetime) | 119 | 119 | **Unchanged — no new rejections** |

**OBSERVED:** Economic state fully frozen since pass 97. No new transactions in either direction (nonces unchanged). No new insufficient-balance events. Balance 20 floor holds through epoch cycles (last epoch: balance_before=20, balance_after=20 with ratio=1.02).

**UNKNOWN (unchanged from pass 97):** Why total supply is 10,000 (vs 5,000 minted) on morning-api's books. This was first observed in pass 97 and is unchanged.

---

## Persistence State

### morning-api (single-capture bundle ~14:28Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 170 | Every 10 epochs | None — snapshot rotated at 170 (was 130 at pass 97) |
| wal_bytes | 379 | File size of current WAL | **None — MATCHES disk.** wal.log = 379 bytes. |
| wal_entries | 3 | Non-zero | Known-provisional (heuristic: size/120 ≈ 3.16) |

**File system cross-check (~14:28Z):**
- `state.snapshot`: **895 bytes** (mtime ~10:26 EDT / 14:26 UTC — epoch 170 snapshot)
- `wal.log`: 379 bytes (mtime ~10:26 EDT — genesis re-seed after rotation)
- `wal.wal.old`: 379 bytes (mtime ~10:21 EDT — pre-snapshot-170 WAL, contains only genesis since no transactions were persisted between epochs 130-170)
- `wal_bytes` endpoint: 379 → **MATCH.** Byte-equality ACHIEVED.

### local-witness (single-capture bundle ~14:28Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 170 | 170 | None — matches morning-api |
| wal_bytes | 379 | File size of current WAL | **None — MATCHES disk.** wal.log = 379 bytes. |
| wal_entries | 3 | Non-zero | Known-provisional heuristic |

**File system cross-check (~14:28Z):**
- `state.snapshot`: **569 bytes** (mtime ~10:26 EDT)
- `wal.log`: 379 bytes (mtime ~10:26 EDT)
- `wal.wal.old`: 379 bytes (mtime ~10:21 EDT)
- `wal_bytes` endpoint: 379 → **MATCH.** Byte-equality ACHIEVED.

**Delta from pass 97:**

| Metric | Pass 97 | Pass 98 | Δ |
|--------|---------|---------|---|
| m-ap: last_snapshot_epoch | 130 | **170** | Rotated at epoch 170 |
| m-ap: wal.wal.old size | 4742 bytes | 379 bytes | Replaced by epoch-170 rotation (smaller because no new transactions 130-170) |
| m-ap: state.snapshot | 896 bytes | 895 bytes | -1 byte (minor) |
| witness: last_snapshot_epoch | 130 | **170** | Rotated at epoch 170 |
| Both: wal_bytes | 379 | 379 | Unchanged |

**OBSERVED:** Snapshot rotation at epoch 170 confirmed on both nodes. Both nodes converged on the same snapshot epoch. wal.wal.old size decreased from 4742 to 379 bytes — consistent with no new transactions being persisted between rotations (nonce unchanged).

---

## Metrics Instrumentation

| Gauge | morning-api (pass 97→98) | local-witness (pass 97→98) | Threshold | Δ |
|-------|-------------------------|---------------------------|---|----|
| outstanding_fetches | 0 → **0** | 0 → **0** | Near zero | Unchanged |
| aged (>50s) | 0 → **0** | 0 → **0** | Near zero | Unchanged |
| outbound_queues (non-empty) | 0 → **0** | 0 → **0** | Empty | Unchanged |
| max_peer_silence | 7s → **0s** | 7s → **6s** | <30s | Healthy |
| Active peers | 1 → **1** | 1 → **1** | Stable | Unchanged |

**DEVIATION:** None. Mesh is quiescent. All gauges at zero or under thresholds.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **KAD bootstrap WARNs:** Expected on `--no-mdns` mesh. Not new.
- **Startup:** Already recovered at pass 97.
- **Metrics lines:** Clean. `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s`
- **Zombie eviction events:** **0** (lifetime)
- **Panics:** **0**
- **Non-KAD WARN/ERROR:** **None**

### local-witness (/tmp/lw.log)
- **KAD bootstrap WARNs:** None
- **Metrics lines:** Clean. `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s`
- **Insufficient balance:** **119** (unchanged — no new rejections since pass 97)
- **Zombie eviction events:** **0** (lifetime)
- **Panics:** **0**
- **Non-KAD non-zombie WARN/ERROR:** **None**

**OBSERVED:** No new log anomalies since pass 97. The 119 insufficient-balance events are historical (same set as pass 97, no new events).

---

## Build Commit Verification

| Check | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| git HEAD | `452b64f` | — | — |
| running binary (m-ap) | `cb5d4b1-dirty` | `452b64f` | **Persistent DEVIATION (#1).** 2 code commits behind HEAD + dirty tree. Unchanged since pass 97. |
| running binary (witness) | `cb5d4b1-dirty` | `452b64f` | Same. |

**Missing commits:**
- `0c4bb7f` — fix: get_stats() reads unified wal.log (code change)
- `452b64f` — docs: wal_bytes fix verified (docs only)

**NOTE:** wal_bytes fix was active in pass 97 (likely from `-dirty` uncommitted code at build time) and remains active in pass 98.

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 97 | Pass 98 | Changed? |
|---|-----------|----------------|---------|---------|----------|
| 1 | `build_commit` stale (`cb5d4b1-dirty`, HEAD `452b64f`) | Pass 1 (Jul 27) | `cb5d4b1-dirty` (2 behind) | `cb5d4b1-dirty` (2 behind) | **Unchanged** |
| 2 | `wal_bytes` returns 0 (legacy path) | Pass 1 (Jul 27) | **RESOLVED** (379 = wal.log) | **RESOLVED** (379 = wal.log) | **Unchanged** — fix working |
| 3 | Supply divergence (total=10,000 on m-api vs 5,000 minted) | Pass 97 (14:08Z) | Total=10,000 | Total=10,000 | **Unchanged** |
| 4 | Witness reports morning-api balance as 0 | Pass 1 (Jul 27 18:48Z) | Persistent (0) | Persistent (0) | **Unchanged** |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (socket vs last log vs count) | **PASS** — m-ap: socket=172, count=172, last_log=172. Witness: socket=172, count=172, last_log=172. Full match on both. No boundary race. |
| Byte-equality (wal_bytes vs file size) | **PASS** — m-ap: 379=379. Witness: 379=379. Byte-equality maintained. |
| PID consistency (same processes since session start) | **PASS** — 3579427, 3579821 unchanged since pass 97. |
| Log health (WARN/ERROR filtered) | **PASS** — KAD bootstrap WARNs expected. 119 insufficient-balance events (historical, not new). No panics, no zombies. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** — m-ap 0s, witness 6s. Both well under threshold. |
| Cross-node epoch sync | **PASS** — both at 172 (δ=0). Fully converged. |
| Snapshot rotation | **PASS** — rotation at epoch 170 confirmed on both nodes. wal.wal.old present on both. |

---

## Summary

**Session status:** 98th pass, same session since pass 97 (~80 min runtime). Mesh is stable and quiescent.

**What changed since pass 97:**
1. Epochs: 135 → 172 (+37, ~19 min, consistent cadence)
2. Heartbeats: 345 → ~512 (+167-170, healthy rate)
3. Snapshot rotated at epoch 170 on both nodes (was 130)
4. Nothing else changed — all balances, nonces, and counters frozen

**What did NOT change (frozen state):**
- morning-api balance: 20 (unchanged across 37 epochs)
- morning-api nonce: 241 (no new transactions)
- Witness balance: 0 (no new credits)
- Witness nonce: 4 (no new transactions)
- Insufficient-balance events: 119 (no new rejections)
- Total supply divergence: 10,000 vs 5,000 (unchanged)

**Persistent deviations unchanged:**
- Build commit stale (cb5d4b1-dirty, 2 behind HEAD)
- Supply divergence (total=10,000 on m-api books)
- Witness sees morning-api balance as 0
