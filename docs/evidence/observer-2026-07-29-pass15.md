# Observer Evidence Record — Pass 15 (Jul 29)

**Date:** 2026-07-29
**Primary capture window:** 2026-07-29T04:57:33Z – 04:57:55Z
**Machine:** z4-workstation (localhost, Boynton Beach FL)
**Session type:** Delta-only pass. Continuation of passes 1–14 (same PIDs, same binary). Nodes running since ~20:40 UTC Jul 28 (~8.3 h).

## Topology Disclosure

| Node | PID | Port | Identity Dir | Storage Dir | Genesis | Since (UTC) |
|------|-----|------|-------------|-------------|---------|-------------|
| morning-api | 3987962 (unchanged) | 4005 | /tmp/m-ap-id | /tmp/m-ap | auto (12D3KooWPfr...) | ~20:40 Jul 28 |
| local-witness | 3988201 (unchanged) | 4010 | /tmp/lw-id | /tmp/local-witness | 12D3KooWPfrZ... | ~20:40 Jul 28 |

**Topology change from pass 14 (04:40Z):** None. Same PIDs, same nodes, same session.

**MESH.md stale as of Jul 27:** Still stating "No production nodes running." Not updated for this session. **Unchanged finding from all prior passes.**

**Socket paths:** morning-api: `/tmp/m-ap/lattice.sock`, witness: `/tmp/local-witness/lattice.sock`. Both reachable.

---

## Single Capture — morning-api (04:57:33Z)

All values from a single compound capture at 04:57:33Z unless noted.

### Node Info

| Field | Pass 14 (04:40Z) | This pass | Δ | DEVIATION |
|-------|-----------------|-----------|---|-----------|
| peer_id | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | same | Unchanged | None |
| name | morning-api | same | Unchanged | None |
| uptime_secs | 28,837 | **29,833** | +996s (~16.6 min) | None — matches ~16.8 min wall + capture offset |
| build_commit | `c31b333-dirty` | `c31b333-dirty` | Unchanged | **PERSISTENT (D1).** 1 commit behind HEAD `7fb1daa`. `-dirty` suffix. |
| thickness | 968.015 | **967.759** | -0.256 | None — expected gradual decay |
| chain_tip | 1 | 1 | Unchanged | None |

### Epoch State

| Source | Value |
|--------|-------|
| Socket epoch (04:57:33Z, GetEpochState) | **995** |
| Log count (grep -c "Epoch complete", 04:57:33Z) | **995** |
| Last log epoch line | **995** (04:57:20.095975Z — `balance_before=20 balance_after=20 ratio=1.02`) |

**Three-way epoch equality:** **MATCH** (995 = 995 = 995). Third consecutive perfect match.

**Cross-node epoch sync:** morning-api=995, witness=995 at ~04:57:33Z. **MATCH.** Third consecutive pass showing exact cross-node epoch alignment.

| Metric | Pass 14 (04:40Z) | This pass (04:57Z) | Δ |
|--------|-----------------|-------------------|---|
| Epoch (socket at capture) | 962 | **995** | +33 over ~16.7 min |
| Epoch rate | ~31s/epoch | **~30.3s/epoch** | Consistent within normal variance |
| ratio | 1.020 | **1.020** | Unchanged |
| tax_calculated | 0 | 0 | Unchanged |
| minted | 0 | 0 | Unchanged |
| redistributed_to | 1 | 1 | Unchanged |

### Peer Connections

| Metric | Pass 14 (04:40Z) | This pass | Δ | DEVIATION |
|--------|-----------------|-----------|---|-----------|
| Peer count | 1 (witness) | 1 (witness) | Unchanged | None |
| Heartbeats received | 2,882 (socket) | **2,982** (socket) | +100 in ~16.7 min (~6.0/min) | None — healthy, stable rate |
| silence_secs | 6s | **2s** | Decreased | None — well below 30s zombie threshold |
| queue_depth | 0 | 0 | Unchanged | None |
| is_dead | false | false | Unchanged | None |

### Persistence State

| Field | Pass 14 (04:40Z) | This pass | Δ |
|-------|-----------------|-----------|---|
| last_snapshot_epoch | 960 | **990** | +30 (3 snapshot rotations) |
| wal_bytes | 379 | **379** | Unchanged |
| wal_entries | 3 | 3 | Unchanged |

**Byte-equality:**
- Disk `/tmp/m-ap/persistence/wal.log`: 379 bytes (mtime Jul 29 00:54 — rotated since pass 14)
- Endpoint `wal_bytes`: 379
- **PASS.** Eighth consecutive pass with byte equality.

WAL details:
- `wal.log`: 379 bytes (mtime Jul 29 00:54 — mtime updated, file content size unchanged)
- `wal.wal.old`: 379 bytes (mtime Jul 29 00:49 — unchanged)
- `state.snapshot`: **897 bytes** (mtime Jul 29 00:54 — stable across passes 7→15: **9 consecutive passes**)

**Snapshot rotation verified:** last_snapshot_epoch advanced from 960 to 990 (+30 epochs, 3 rotations). Snapshot mtime (00:54 EDT = 04:54 UTC) confirms rotation ~3 min before this capture. Snapshot size stable at 897B for 9 consecutive passes.

### Economic State

| Metric | Pass 14 (04:40Z) | This pass | Δ | EXPECTED | DEVIATION |
|--------|-----------------|-----------|---|----------|-----------|
| own_balance | 20 | **20** | Unchanged | Frozen since Jul 27 | **PERSISTENT (D2)** |
| own_nonce | 423 | **423** | Unchanged | Frozen since Jul 27 | None |
| witness balance (local view) | 19,980 | **19,980** | Unchanged | Frozen since Jul 27 | **PERSISTENT (D2)** |
| witness nonce (local view) | 0 | 0 | Unchanged | Frozen | None |
| Total (m-ap ledger) | 20,000 | **20,000** | Unchanged | Should be 5,000 (mint) | **PERSISTENT (D2)** |

Each epoch log: `balance_before=20 balance_after=20 ratio=1.02` — no balance movement. Pattern continues through epoch 995.

---

## Secondary Capture — local-witness (04:57:35Z)

### Node Info

| Field | Pass 14 (~04:41Z) | This pass | DEVIATION |
|-------|------------------|-----------|-----------|
| peer_id | 12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch | same | Matches MESH.md |
| name | local-witness | same | Matches MESH.md |
| uptime_secs | 28,832 | **29,783** | +951s (~15.9 min) |
| build_commit | `c31b333-dirty` | `c31b333-dirty` | Same D1 as morning-api |
| chain_tip | 1 | 1 | None |

### Epoch State

| Source | Value |
|--------|-------|
| Socket epoch (04:57:35Z, GetEpochState) | **995** |
| Log count (04:57:35Z — grep -c "Epoch complete") | **995** |
| Last log epoch line | **995** (04:57:35.340913Z — `balance_before=0 balance_after=0 ratio=1.11`) |

**Three-way epoch equality:** **MATCH** (995 = 995 = 995). Third consecutive perfect match on witness.

**Cross-node synchronization (04:57:33–04:57:35Z):** m-ap=995, witness=995. Exact match. Third consecutive pass showing cross-node epoch alignment.

**Cross-node ratio divergence:** morning-api=1.020, witness=1.107 — persistent, unchanged pattern.

### Peer Connections

| Metric | Pass 14 | This pass | DEVIATION |
|--------|--------|-----------|-----------|
| Peer count | 1 (morning-api) | 1 (morning-api) | None |
| Heartbeats received | 2,884 (socket) | **2,983** (socket) | +99 (~6.0/min — matches m-ap's rate) |
| silence_secs | 5s | **5s** | Unchanged — healthy (<30s threshold) |
| queue_depth | 0 | 0 | None |
| is_dead | false | false | None |

### Persistence State

| Field | Pass 14 | This pass | DEVIATION |
|-------|--------|-----------|-----------|
| last_snapshot_epoch | 960 | **990** | +30 (3 rotations — matches morning-api) |
| wal_bytes | 379 | **379** | None |
| wal_entries | 3 | 3 | None |

**Byte-equality:** Disk `wal.log` = 379 bytes (mtime Jul 29 00:55 — rotated), Endpoint `wal_bytes` = 379. **PASS.**

WAL details:
- `wal.log`: 379 bytes (mtime Jul 29 00:55 — rotated since pass 14)
- `wal.wal.old`: 379 bytes (mtime Jul 29 00:50 — unchanged)
- `state.snapshot`: **569 bytes** (mtime Jul 29 00:55 — unchanged, stable for 9 consecutive passes)

### Economic State

| Metric | Pass 14 | This pass | EXPECTED | DEVIATION |
|--------|--------|-----------|----------|-----------|
| own_balance | 0 | **0** | 0 (--mint 0) | None |
| own_nonce | 8 | **8** | Frozen | None |
| morning-api balance (witness's view) | 0 | **0** | Should match morning-api's 20 | **PERSISTENT (D2)** — asymmetry |
| morning-api nonce (witness's view) | 0 | **0** | Should match morning-api's 423 | None — nonce asymmetry known |

Each epoch on witness: `balance_before=0 balance_after=0 ratio=1.11` — zero balance, no redistribution.

---

## NTP Series

| Metric | Pass 14 (~04:41Z) | This pass (~04:57Z) |
|--------|-------------------|---------------------|
| System NTP | Active, NTPSynchronized=yes | Active, NTPSynchronized=yes |
| morning-api NTP failures (total) | 4 | **4** (unchanged) |
| witness NTP failures (total) | 3 | **3** (unchanged) |
| New m-ap failures since pass 14 | 0 | **0 new** |
| New witness failures since pass 14 | 0 | **0 new** |
| Failures in last 6 passes (~03:40–04:57Z) | — | **0 new failures** |
| Time since last m-ap failure | ~28 min at pass 14 capture | **~44 min** (last at 04:13:26Z) |
| Time since last witness failure | ~37 min at pass 14 capture | **~53 min** (last at 04:04:28Z) |
| Trend | **STABLE** (2 consecutive quiet passes) | **STABLE** (3 consecutive quiet passes) |

**Assessment:** D6 (morning-api NTP failure rate) remains OPEN but is now trending strongly toward closure. **Three consecutive quiet passes** with zero new failures on either node. The pass 12 cluster (3 failures in 9 min at 04:04–04:13Z) is now 44+ minutes in the past with no recurrence. All failures identified as `os error 11` (Resource temporarily unavailable) from pool.ntp.org or time.apple.com. System NTP reports synchronized=yes. Fallback mechanism absorbing all failures (hard-fail count=0 on both nodes).

D6 closure criteria: needs 2 more quiet passes (10+ min total) after this pass. If pass 16 and 17 both remain quiet, D6 can close.

---

## Log Health

**morning-api log (/tmp/m-ap.log):**
- 4 total NTP failures (all old, none new this pass — last at 04:13:26Z, 44 min ago)
- Kademlia "Failed to trigger bootstrap: No known peers" WARNs every ~5 min — expected, no DHT
- No ERROR entries
- Metrics: `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=4s` — healthy, all pinned at zero
- Filtered WARN/ERROR (excluding NTP/Kademlia): **empty**

**local-witness log (/tmp/lw.log):**
- 3 total NTP failures (all old, none new this pass — last at 04:04:28Z, 53 min ago)
- No ERROR entries
- Metrics: `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=5s` — healthy
- Filtered WARN/ERROR (excluding NTP/Kademlia): **empty**

---

## Deviation Register

### D1 — Build commit behind HEAD (PERSISTENT)

**First observed:** Prior to pass 99 (14:42Z Jul 28), possibly earlier.
**Status:** Unchanged. `c31b333-dirty` on both nodes. Git HEAD is `7fb1daa` (1 commit ahead, docs-only diff: observer pass100 update). `-dirty` from 1 modified tracked file + 44+ untracked files (evidence docs, cert files, log files).
**Evidence:** Socket returns `c31b333-dirty`. `git log --oneline -1` returns `7fb1daa`.

### D2 — Economic state frozen, supply asymmetry (PERSISTENT)

**First observed:** 2026-07-27T18:48:56Z (Observer pass #3)
**Status:** Unchanged. Both ledgers frozen since Jul 27. No redistribution occurring.
- morning-api: own=20, witness=19,980, own_nonce=423, total=20,000
- witness: own=0, morning-api=0, own_nonce=8, total=0
- Cross-node balance asymmetry unchanged
- Each epoch on morning-api: `balance_before=20 balance_after=20 ratio=1.02`
- Each epoch on witness: `balance_before=0 balance_after=0 ratio=1.11`
- See VERIFIED-BEHAVIOR.md for full decomposition

### D3 — State root divergence (PERSISTENT, structurally expected)

**First observed:** Pass 5 (02:54Z Jul 29)
**Status:** Not captured this pass. Structurally expected per Mission A audit (classification C).

### D4 — Snapshot size asymmetry (PERSISTENT, stable)

**First observed:** Pass 4 (02:44:58Z Jul 29)
**Status:** Snapshot sizes unchanged: m-ap=**897B** (stable across passes 7→15 — **9 consecutive passes**), witness=**569B** (stable). Fluctuation confirmed resolved.

### D5 — Witness nonce (8) vs morning-api nonce (423) (PERSISTENT, UNKNOWN)

**First observed:** Pass 4 (02:44:58Z Jul 29)
**Status:** Unchanged. Witness has 8 local transactions; morning-api has 423.

### D6 — Morning-api NTP failure rate (OPEN, improving)

**First observed:** Pass 7 (03:10Z Jul 29 — first NTP WARN on morning-api)
**Status:** **Quiet this pass (third consecutive).** 0 new NTP failures on either node. Total counts unchanged at 4 (m-ap) and 3 (witness). Time since last failure: m-ap ~44 min, witness ~53 min. The pass 12 cluster (3 failures in 9 min) is now 44+ minutes in the past with no recurrence. Trend: STRONGLY STABLE. Needs 2 more quiet passes to close.

---

## UNKNOWN Items

1. **Morning-api snapshot size fluctuation** — **RESOLVED.** Stable at 897B for 9 consecutive passes. **CLOSED.**

2. **Witness nonce (8) vs morning-api nonce (423).** Unchanged. Witness's 8 local transactions not explained.

3. **Total supply discrepancy (20,000 vs expected 5,000).** Unchanged. morning-api ledger shows 20,000 DUU (20 self + 19,980 witness). Original --mint was 5,000. The additional 15,000 DUU provenance untraced.

4. **NTP "fallback" design.** Both nodes report `(fallback)` suffix on NTP failures. Unknown whether a second-tier NTP server was tried successfully, or whether startup proceeds without a successful query. Hard-failure count remains 0 on both nodes.

5. **Pass 12 NTP cluster — nature remains UNKNOWN.** Three failures within 9 minutes at 04:04–04:13Z. Now 44+ minutes without recurrence. Transient hypothesis strongly supported by three quiet passes but not confirmed. `os error 11` (EAGAIN) consistent with system-wide transient resource condition.

---

## Delta Summary from Pass 14

| Metric | Pass 14 (04:40Z) | This pass (~04:57Z) | Change |
|--------|-----------------|--------------------|--------|
| Uptime (m-ap) | 28,837s | 29,833s | +996s ✓ |
| Uptime (witness) | 28,832s | 29,783s | +951s ✓ |
| Epoch (m-ap, socket at capture) | 962 | 995 | +33 ✓ |
| Epoch (witness, socket at capture) | 962 | 995 | +33 ✓ |
| Cross-node epoch sync | MATCH (962=962) | **MATCH (995=995)** | ✓ (3rd consecutive) |
| Three-way epoch equality (m-ap) | MATCH (962=962=962) | **MATCH (995=995=995)** | ✓ (3rd consecutive) |
| Three-way epoch equality (witness) | MATCH (962=962=962) | **MATCH (995=995=995)** | ✓ (3rd consecutive) |
| Heartbeats (m-ap) | 2,882 | 2,982 | +100 ✓ |
| Heartbeats (witness) | 2,884 | 2,983 | +99 ✓ |
| silence_secs (m-ap) | 6s | 2s | Healthy (<30s) ✓ |
| silence_secs (witness) | 5s | 5s | Healthy (<30s) ✓ |
| thickness (m-ap) | 968.015 | 967.759 | -0.256 ✓ |
| wal_bytes | 379 | 379 | Unchanged ✓ |
| last_snapshot_epoch | 960 | 990 | +30 (3 rotations) ✓ |
| own_balance (m-ap) | 20 | 20 | Unchanged (D2) |
| Build commit | c31b333-dirty | c31b333-dirty | Unchanged (D1) |
| Snapshot sizes (m-ap/witness) | 897B / 569B | **897B** / **569B** | Stable (9 passes) ✓ |
| NTP failures (m-ap) | 4 | **4** | **No new failures (3rd consecutive)** |
| NTP failures (witness) | 3 | **3** | **No new failures (3rd consecutive)** |
| Byte equality (m-ap) | PASS (379=379) | **PASS** (379=379) | ✓ (8th consecutive) |
| Byte equality (witness) | PASS (379=379) | **PASS** (379=379) | ✓ |
| Three-way epoch (m-ap) | MATCH | **MATCH** | ✓ (3rd consecutive) |
| Three-way epoch (witness) | MATCH | **MATCH** | ✓ (3rd consecutive) |
| Cross-node epoch sync | MATCH | **MATCH** | ✓ (3rd consecutive) |

## Classification

| Item | Status | Severity |
|------|--------|----------|
| D1: Build commit 1 behind HEAD + dirty tree | Persistent — unchanged | LOW (docs-only diff) |
| D2: Economic state frozen / supply asymmetry | Persistent — unchanged | MEDIUM (known, documented in VERIFIED-BEHAVIOR.md) |
| D3: State root divergence | Not checked this pass | INFO (classification C, structurally expected) |
| D4: Snapshot size asymmetry (569B vs 897B) | Persistent — **stable** (897B for 9 passes) | INFO (fluctuation confirmed resolved) |
| D5: Witness nonce 8 vs morning-api nonce 423 | Persistent — unchanged | LOW (unexplained but not growing) |
| D6: Morning-api NTP failure rate | **OPEN — quiet (3rd consecutive pass)** | LOW (fallback absorbing, no hard-fail) |
| Mesh connectivity (1 peer, heartbeats flowing) | Healthy | — |
| WAL state (snapshot rotation, byte equality) | Healthy | — |
| NTP sync (system-level) | Active, synchronized | — |
| Epoch progression | Normal (~30.3s/epoch), cross-node match | — |
| Byte equality | PASS (both nodes, 8th consecutive) | — |
| Three-way epoch equality | **MATCH on both nodes (3rd consecutive)** | — |
| Metrics (fetches, queues, max_peer_silence) | All zero/healthy | — |

## New Findings This Pass

1. **Third consecutive three-way epoch equality on both nodes.** Both m-ap and witness returned MATCH (995=995=995). This is now a stable measurement pattern, not a coincidence.

2. **Third consecutive cross-node epoch MATCH.** Both nodes at epoch 995 at the same capture moment. The off-by-one timing artifact from earlier passes is fully resolved.

3. **NTP quiet — third consecutive pass.** Zero new NTP failures since pass 12's cluster. Time since last failure: m-ap ~44 min, witness ~53 min. D6 is approaching closure criteria (needs 2 more quiet passes).

4. **No other new findings.** All metrics within normal bounds. The mesh continues stable operation with the same frozen economic state, healthy heartbeat exchange, and regular snapshot rotation.
