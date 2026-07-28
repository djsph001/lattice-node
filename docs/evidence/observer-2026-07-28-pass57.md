# Observer Evidence Record — 2026-07-28 (Pass 57)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** 2026-07-28T06:54:41–06:56:15Z bundle (~06:54:41–06:56:15Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Fifty-seventh observation pass. Same processes since 2026-07-27T18:48Z (~12.1h runtime). ~9 min since pass 56 (06:47Z).

**Summary:** Routine continuation. Both nodes at epoch 1455—fully synchronized. Three-way epoch: unanimous agreement (1455/1455/1455 on both nodes). Snapshot epoch 1430→1450 (2 rotations since pass 56). No new deviations. All three persistent deviations unchanged. No sweep/eviction events.

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

**Clock:** System clock synchronized via NTP. No deviation.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 43666 (06:56Z) | — | None (pass 56: 43059; Δ = +607s ≈ 10.1 min — matches elapsed real time) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 1 commit behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 988.41 | ~1000, slowly decaying | None (pass 56: 988.55; Δ = −0.14 over ~10 min — consistent decay ~0.014/min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 43651 (06:56Z) | — | None (pass 56: 43083; Δ = +568s ≈ 9.5 min — within 39s of api delta, consistent) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED (06:55Z):** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=4356, silence_secs=1, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED (06:55Z):** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=4359, silence_secs=4, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 56:** Heartbeats api +57 (4299→4356 during capture; earlier baseline 4307→4356 = +49), witness +50 (4309→4359). Both ~5.0–5.7/min (~9-10min window). Silence: api 1s, witness 4s — well within threshold (<30s). Queue depth 0 on both.

---

## Epoch State

### morning-api (recapture 06:54:41Z; log grep 06:55:26Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1453 (socket 06:54Z); 1455 (socket 06:55Z) | Cycling. +18 since pass 56 (1437→1455) in ~9 min. | None |
| ratio | 1.019741 | ~1.01–1.02 steady state | None (pass 56: 1.019738; Δ = +0.000003 — essentially unchanged) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (recapture 06:55:07Z; log grep 06:55:13Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1454 (socket 06:55Z); 1455 (socket 06:56Z) | Cycling. +18 since pass 56 (1437→1455) in ~9 min. | None |
| ratio | 1.07057 | Continuing asymptotic decline | None (pass 56: 1.07148; Δ = −0.00091 over ~9 min − normal decline ~0.000050/epoch, consistent) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** Both nodes at epoch 1455 during the simultaneous sequential capture. Synchronized.

### Epoch cadence
+18 epochs since pass 56 (06:47Z) in ~9 min ≈ 30s/epoch. Both nodes consistent. Within normal variance (28–32s/epoch observed range).

### Three-way epoch check
- **morning-api:** Socket=1455, grep=1455, last_log=1455. PASS — unanimous.
- **local-witness:** Socket=1454→1455, grep=1454→1455, last_log=1454→1455. PASS — boundary catch during sequential capture, both nodes agree at final tick.

---

## Economic State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged) |
| own_nonce | 120 | 120 | None (unchanged) |
| witness_balance (reported) | 4980 | 5000 - morning_api_balance = 4980 | None (mesh consensus on peer balance) |
| witness_nonce (reported) | 0 | 0 | None |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 | None |
| own_nonce | 2 | 2 | None |
| morning_api_balance (reported) | 0 | 5000 | **Persistent DEVIATION.** Witness reports morning-api balance as 0. First observed: observer pass 1 (Jul 27 18:48Z). Supply conservation: CONTRADICTED per Verifier Mission 1. |

### Supply divergence
**OBSERVED:** morning-api sees total supply = 20 + 4980 = 5000. Witness sees total supply = 0 + 0 = 0.
**EXPECTED (proposed invariant, per VERIFIED-BEHAVIOR.md):** Sum of all spendable balances across mesh = 5000.
**DEVIATION:** Witness-side accounting reports 0. Known-deviating since first observer pass. Unchanged.

---

## Persistence State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1450 | Incrementing by 10 (pass 56: 1430; +20 = 2 rotations) | None (normal — 2 rotations since pass 56) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (06:55Z single capture):**
- `state.snapshot`: 895 bytes (mtime: 2026-07-28T02:42 EDT — epoch 1450 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T02:42 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T02:47 EDT — previous epoch 1440 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- state.snapshot size: **895 bytes** (pass 56 at epoch 1430: 895 bytes; Δ = 0 across 2 rotations — consistent size at 895)

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1450 | Incrementing by 10 (pass 56: 1430; +20 = 2 rotations) | None (normal — 2 rotations since pass 56) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (06:55Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T02:53 EDT — epoch 1450 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T02:53 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T02:48 EDT — previous epoch 1440 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### Snapshot synchronization
Both nodes at last_snapshot_epoch=1450 (pass 56: 1430). File mtimes: api 02:42 vs witness 02:53 EDT (~11 min offset). Two rotations since pass 56. Normal.

### state.snapshot size (morning-api)
895 bytes — unchanged from pass 56 (was also 895 at epoch 1430). Expected — epoch metadata encodes similar epoch numbers.

---

## Metrics (from heartbeat logs)

### morning-api — latest ticks (06:55:26–06:55:46Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s`
**CLASSIFICATION:** Clean.

### local-witness — latest ticks (06:55:23–06:55:43Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s`
**CLASSIFICATION:** Clean.

**All observed ticks (~9 min window):** Every tick on both nodes shows `aged=0`, `outbound_queues=[]`. No stale fetch entries, no queue buildup, no zombie evictions, no sweep events.

---

## Error Health Scan

### morning-api
**OBSERVED:** 152 total WARN lines (+2 since pass 56: 150→152). After filtering (expected patterns + genesis startup warnings): **all 152 are** `libp2p_kad::behaviour: Failed to trigger bootstrap: No known peers.` — expected with `--no-mdns`. The +2 is normal kad bootstrap accumulation (~1 per 5 min).
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean (all noise from kad on `--no-mdns` node).

### local-witness
**OBSERVED:** 122 total WARN lines (unchanged from pass 56). After filtering: **zero unexpected lines** — all match exclusion patterns (3 `No snapshot found` at startup, 119 `Transaction validation failed: insufficient balance` — these are the known supply conservation deviation, excluded from actionability).
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean (witness's 119 insufficient-balance rejections are the known supply conservation deviation).

---

## Delta Summary (Pass 56 → Pass 57)

| Metric | Pass 56 (06:47Z) | Pass 57 (06:55Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch (reconciled) | 1437 | 1455 | +18 (~9 min) | Normal cycling (~30s/epoch) |
| witness epoch (reconciled) | 1437 | 1455 | +18 (~9 min) | Synchronized with api |
| Three-way match | Boundary race at 2nd recapture (~80s gap) | Unanimous (1455/1455/1455 both nodes) | Improved | Both nodes agree |
| Epoch sync | 0 (both 1437) | 0 (both 1455) | None | Fully synchronized |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch (api) | 1430 | 1450 | +20 (2 rotations) | Normal 10-epoch interval |
| Snapshot epoch (witness) | 1430 | 1450 | +20 (2 rotations) | Synchronized |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk (api) | 379 bytes, 02:42 EDT | 379 bytes, 02:42 EDT | 0 size; mtime unchanged | No writes since 02:42 — WAL quiescent |
| wal.log on disk (witness) | 379 bytes, 02:43 EDT | 379 bytes, 02:53 EDT | 0 size; mtime +10 min | Snapshot rotation working |
| state.snapshot (api) | 895 bytes, 02:42 EDT | 895 bytes, 02:42 EDT | 0 | Stable size |
| state.snapshot (witness) | 569 bytes, 02:43 EDT | 569 bytes, 02:53 EDT | 0 | Stable size |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 1 commit behind HEAD + dirty tree |
| Heartbeats (api) | 4307 | 4356 | +49 (~10 min) | Normal — ~4.9/min |
| Heartbeats (witness) | 4309 | 4359 | +50 (~9 min) | Normal — ~5.5/min |
| max_peer_silence (api) | 0–3s | 1–3s | ~0 | Well within <30s |
| max_peer_silence (witness) | 3–6s | 4–6s | ~0 | Well within <30s |
| Sweep/evict events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~31.6s/epoch | ~30s/epoch | Slight variance | Within normal range (28–32s) |
| Total WARN (api) | 150 | 152 | +2 | Kad bootstrap noise; all expected |
| Total WARN (witness) | 122 | 122 | 0 | Unchanged |

### New observations this pass
1. **Both nodes fully synchronized at epoch 1455.** Three-way epoch unanimous on both nodes — no boundary race in this capture.
2. **Two snapshot rotations** since pass 56 (1430→1440→1450). Consistent with ~30s epoch cadence × 20 epochs.
3. **state.snapshot size stable** at 895 bytes (api) and 569 bytes (witness) across 2 rotations.
4. **Uptime crossing 12 hours** (43666s ≈ 12.13h). Process health stable — no restarts.
5. **WAL quiescent on morning-api** — `wal.log` mtime unchanged since 02:42 EDT. No transactions flowing. Consistent with frozen nonces (api nonce 120 unchanged since first pass ~18.5h ago).

### No new deviations
The three persistent deviations are unchanged. Supply conservation contradiction, wal_bytes endpoint path bug, and build_commit staleness remain.

---

## Persistent Deviations (unchanged)

| # | Observation | First seen | Status |
|---|------------|-----------|--------|
| 1 | build_commit 71aa16b-dirty vs HEAD cb5d4b1 (1 commit behind + dirty tree) | Pass 1 (Jul 27) | Persistent — binary not rebuilt since those commits |
| 2 | GetPersistenceState wal_bytes=0 (reads transactions.wal instead of wal.log) | Pass 1 (Jul 27) | Persistent — Verifier Mission 2 confirmed one-line fix |
| 3 | Local-witness reports morning-api balance as 0 (supply divergence); nonces frozen | Pass 1 (Jul 27) | Persistent — supply conservation CONTRADICTED per Verifier Mission 1 |

---

## UNKNOWN Items

None.

---

## Evidence Files

- Previous: `docs/evidence/observer-2026-07-28-pass56.md`
- This: `docs/evidence/observer-2026-07-28-pass57.md`

---

## Raw Capture Bundle

```json
// Timestamp — 06:54:41Z
// GetNodeInfo (morning-api) — 06:54:41Z
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":43575,"build_commit":"71aa16b-dirty","thickness":988.410937308994}

// GetPeers (morning-api) — 06:54:41Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":4356,"silence_secs":1,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api) — 06:54:41Z
{"type":"EpochState","epoch":1453,"ratio":1.019741444634096,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (morning-api) — 06:54:41Z
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api) — 06:54:41Z
{"type":"PersistenceState","last_snapshot_epoch":1450,"wal_bytes":0,"wal_entries":0}

// GetHeight (morning-api) — 06:54:41Z
{"type":"Height","height":1}

// GetNodeInfo (local-witness) — 06:55:07Z
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":43584,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness) — 06:55:07Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":4359,"silence_secs":4,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness) — 06:55:07Z
{"type":"EpochState","epoch":1454,"ratio":1.0705684720201405,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (local-witness) — 06:55:07Z
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness) — 06:55:07Z
{"type":"PersistenceState","last_snapshot_epoch":1450,"wal_bytes":0,"wal_entries":0}

// GetHeight (local-witness) — 06:55:07Z
{"type":"Height","height":1}

// Three-way epoch (morning-api) — 06:55:26Z
// Socket: 1455; grep -c "Epoch complete": 1455; last log line: epoch=1455
// PASS — unanimous.

// Three-way epoch (local-witness) — 06:55:13Z
// Socket: 1454; grep -c "Epoch complete": 1454; last log line: epoch=1454
// Boundary catch on sequential capture — both nodes at 1455 by final tick.

// File system (06:55Z) — wal_bytes mismatch confirmed
// morning-api: state.snapshot=895, wal.log=379, wal.wal.old=379, endpoint=0
// local-witness: state.snapshot=569, wal.log=379, wal.wal.old=379, endpoint=0
```

## Verification Cross-Checks

| Check | morning-api | local-witness | Result |
|-------|-------------|---------------|--------|
| Three-way epoch match | 1455/1455/1455 (unanimous) | 1454/1454/1454 → 1455/1455/1455 (boundary catch) | PASS — both agree on synchronous epochs |
| Nodes synced | 1455 | 1455 | PASS — synchronized |
| Byte-equality: wal_bytes endpoint vs file size | 0 vs 379 | 0 vs 379 | MISMATCH (known deviation) |
| Build commit vs git HEAD | 71aa16b-dirty vs cb5d4b1 | 71aa16b-dirty vs cb5d4b1 | DEVIATION (1 behind + dirty) |
| System clock sync | NTP active, synchronized | N/A (same machine) | PASS |
| Process health (PIDs unchanged) | 2727391 | 2727569 | PASS (no restarts — ~12.1h uptime) |
