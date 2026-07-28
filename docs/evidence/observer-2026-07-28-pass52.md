# Observer Evidence Record — 2026-07-28 (Pass 52)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** 2026-07-28T05:51:26–05:52:26Z bundle (01:51:26–01:52:26 EDT)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Fifty-second observation pass. Same processes since 2026-07-27T18:48Z (~11h runtime). ~8.5 min since pass 51 (05:43Z).

**Summary:** Routine continuation. Both nodes at epoch 1328 — fully synchronized. Three-way epoch clean on both nodes. Snapshot epoch 1310→1320 (1 rotation since pass 51). No new deviations. All three persistent deviations unchanged. state.snapshot on morning-api steady at 895 bytes (confirmed stable across epoch 1320 boundary — was 895 at both 1310 and 1320).

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
- Socket: `/tmp/m-ap/lattice.sock` (morning-api) — responding
- Socket: `/tmp/local-witness/lattice.sock` (local-witness) — responding

Logs at `/tmp/m-ap.log` and `/tmp/lw.log`.

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
| uptime_secs | 39780 (~11.05h) | — | None (pass 51: 39312; Δ = +468s ≈ 7.8 min — within expected for ~8.5 min real time given timing of queries) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 1 commit behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 989.41 | ~1000, slowly decaying | None (pass 51: 989.54; Δ = -0.13 over ~8.5 min — normal decay ~0.015/min, consistent) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 39781 (~11.05h) | — | None (pass 51: 39307; Δ = +474s ≈ 7.9 min — within ~6s of api delta, normal) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=3976, silence_secs=9, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=3979, silence_secs=4, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 51:** Heartbeats api +47 (3929→3976), witness +48 (3931→3979). Both ~5.6/min. Silence: api 7s→9s (within threshold), witness 6s→4s (stable). Well within <30s threshold. Queue depth 0 on both.

---

## Epoch State

### morning-api (05:51:26Z capture; 05:51:56Z log confirm; 05:52:06Z recapture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1327 (socket 05:51Z) → 1328 (recapture 05:52Z) | Cycling. +16 since pass 51 (1312→1328) in ~8.5 min. | **PASS — clean three-way match at recapture.** Socket=1328, grep count=1328, last log line=epoch=1328. Clean. |
| ratio | 1.01972 | ~1.01–1.02 steady state | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (05:51:26Z; 05:52:13Z log confirm; 05:52:24Z recapture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1327 (socket 05:51Z) → 1328 (recapture 05:52Z) | Cycling. +16 since pass 51 (1312→1328) in ~8.5 min. | **PASS — clean three-way match at recapture.** Socket=1328, grep count=1328, last log line=epoch=1328. Clean. |
| ratio | 1.07789 | Continuing asymptotic decline | None (pass 51: 1.07899; Δ = -0.0011 — normal decline ~0.00013/epoch, consistent) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization

**OBSERVED:** At recapture (05:52Z): both nodes at epoch 1328. Fully synchronized.

### Epoch cadence

+16 epochs since pass 51 in ~8.5 min ≈ 31.9s/epoch. Both nodes consistent. Default timer (~30s) within normal variance.

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
| last_snapshot_epoch | 1320 | Incrementing by 10 (pass 51: 1310; +10 epochs = 1 rotation) | None (normal — 1 rotation since pass 51) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (05:52Z single capture):**
- `state.snapshot`: 895 bytes (mtime: 2026-07-28T01:47:56 EDT — epoch 1320 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T01:47:56 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T01:42:56 EDT — previous epoch 1310 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- state.snapshot size: **895 bytes** (pass 51 at epoch 1310: 895; intermediate at epoch 1300: 893; now at epoch 1320: 895. Confirmed stable at 895 across 2 of last 3 boundaries — oscillation 893–895 was content-dependent.)

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1320 | Incrementing by 10 (pass 51: 1310; +10 epochs = 1 rotation) | None (both nodes synchronized) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (05:52Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T01:48:13 EDT — epoch 1320 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T01:48:13 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T01:43:13 EDT — previous epoch 1310 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### Snapshot synchronization

Both nodes at last_snapshot_epoch=1320 (was 1310 in pass 51 — 1 rotation since). File mtimes within ~17s (01:47:56 vs 01:48:13 EDT). Snapshot rotation working normally.

### state.snapshot size (morning-api)

**OBSERVED:** 895 bytes at epoch 1320 (was 895 at epoch 1310, 893 at epoch 1300, 895 at epoch 1320). Steady at 895 across 2 of 3 recent boundaries.
**CLASSIFICATION:** Normal content-dependent variation. Not a deviation.

---

## Metrics (from heartbeat logs)

### morning-api — latest ticks (05:52:06–05:52:26Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s`
**CLASSIFICATION:** Clean.

### local-witness — latest ticks (05:52:03–05:52:23Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s`
**CLASSIFICATION:** Clean.

**All observed ticks (~9 min window):** Every tick on both nodes shows `aged=0`, `outbound_queues=[]`. No stale fetch entries, no queue buildup, no zombie evictions, no sweep events.

---

## Error Health Scan

### morning-api
**OBSERVED:** 139 total WARN/ERROR lines (+2 since pass 51: 137→139). After filtering (expected patterns + `Failed to trigger bootstrap`): **zero unexpected lines.** The +2 is normal gradual accumulation of 5-min-interval kad bootstrap warnings (expected with `--no-mdns`).
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

### local-witness
**OBSERVED:** 122 total WARN/ERROR lines (unchanged from pass 51). After filtering: **zero unexpected lines.**
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

---

## Delta Summary (Pass 51 → Pass 52)

| Metric | Pass 51 (05:43Z) | Pass 52 (05:52Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch (recapture) | 1312 | 1328 | +16 (~8.5 min) | Normal cycling (~32s/epoch) |
| witness epoch (recapture) | 1312 | 1328 | +16 (~8.5 min) | Synchronized with api |
| Three-way match | api: CLEAN, witness: CLEAN | api: CLEAN, witness: CLEAN | None | Both clean |
| Epoch sync | 0 (both 1312) | 0 (both 1328) | None | Fully synchronized |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch (api) | 1310 | 1320 | +10 (1 rotation) | Normal 10-epoch interval |
| Snapshot epoch (witness) | 1310 | 1320 | +10 (1 rotation) | Synchronized |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk (api) | 379 bytes, 01:42 EDT | 379 bytes, 01:47 EDT | 0 size; mtime +5 min | Snapshot rotation working |
| wal.log on disk (witness) | 379 bytes, 01:43 EDT | 379 bytes, 01:48 EDT | 0 size; mtime +5 min | Snapshot rotation working |
| state.snapshot (api) | 895 bytes, 01:42 | 895 bytes, 01:47 | 0 | Steady at 895 across 2 boundaries |
| state.snapshot (witness) | 569 bytes, 01:43 | 569 bytes, 01:48 | 0 | Stable |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 1 commit behind HEAD + dirty tree |
| Heartbeats (api) | 3929 | 3976 | +47 | Normal — ~5.6/min |
| Heartbeats (witness) | 3931 | 3979 | +48 | Normal — ~5.6/min |
| max_peer_silence (api) | 3s | 3s | 0 | Well within <30s |
| max_peer_silence (witness) | 6s | 6s | 0 | Well within <30s |
| Sweep/evict events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~30s/epoch | ~32s/epoch | Slight variance | Within normal range |
| Total WARN/ERROR (api) | 137 | 139 | +2 | Kad bootstrap noise; all expected |
| Total WARN/ERROR (witness) | 122 | 122 | 0 | Unchanged |

### New observations this pass

1. **Both nodes fully synchronized at epoch 1328.** Clean three-way match on both at recapture (1328/1328/1328).

2. **state.snapshot steady at 895 bytes** across epoch 1320 boundary (was 895 at 1310, now 895 at 1320). Confirms the 893→895 oscillation was a one-time mid-session content variation, not a persistent shift.

3. **One snapshot rotation** since pass 51 (1310→1320). Consistent with ~5 min interval.

### No new deviations

The three persistent deviations are unchanged.

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

- Previous: `docs/evidence/observer-2026-07-28-pass51.md`
- This: `docs/evidence/observer-2026-07-28-pass52.md`

---

## Raw Capture Bundle

```json
// Timestamp — 05:51:26Z
// GetNodeInfo (morning-api) — 05:51:26Z
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":39780,"build_commit":"71aa16b-dirty","thickness":989.4105668337133}

// GetPeers (morning-api) — 05:51:26Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":3976,"silence_secs":9,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api) — 05:51:26Z
{"type":"EpochState","epoch":1327,"ratio":1.019973339816732,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (morning-api) — 05:51:26Z
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api) — 05:51:26Z
{"type":"PersistenceState","last_snapshot_epoch":1320,"wal_bytes":0,"wal_entries":0}

// GetHeight (morning-api) — 05:51:26Z
{"type":"Height","height":1}

// GetNodeInfo (local-witness) — 05:51:26Z
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":39781,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness) — 05:51:26Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":3979,"silence_secs":4,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness) — ~05:52Z
{"type":"EpochState","epoch":1327,"ratio":1.0779576472456156,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (local-witness) — ~05:52Z
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness) — ~05:52Z
{"type":"PersistenceState","last_snapshot_epoch":1320,"wal_bytes":0,"wal_entries":0}

// GetHeight (local-witness) — ~05:52Z
{"type":"Height","height":1}

// Three-way epoch (morning-api recapture) — 05:52:06Z
// Socket: 1328; grep -c "Epoch complete": 1328; last log line: epoch=1328
// CLEAN.

// Three-way epoch (local-witness recapture) — 05:52:24Z
// Socket: 1328; grep -c "Epoch complete": 1328; last log line: epoch=1328
// CLEAN.

// File system (05:52Z) — wal_bytes mismatch confirmed
// morning-api: state.snapshot=895, wal.log=379, wal.wal.old=379, endpoint=0
// local-witness: state.snapshot=569, wal.log=379, wal.wal.old=379, endpoint=0
```

## Verification Cross-Checks

| Check | morning-api | local-witness | Result |
|-------|-------------|---------------|--------|
| Three-way epoch match | 1328/1328/1328 (clean) | 1328/1328/1328 (clean) | PASS — both clean |
| Nodes synced | 1328 | 1328 | PASS — synchronized |
| Byte-equality: wal_bytes endpoint vs file size | 0 vs 379 | 0 vs 379 | MISMATCH (known deviation) |
| Build commit vs git HEAD | 71aa16b-dirty vs cb5d4b1 | 71aa16b-dirty vs cb5d4b1 | DEVIATION (1 behind + dirty) |
| System clock sync | NTP active, synchronized | N/A (same machine) | PASS |
| Process health (PIDs unchanged) | 2727391 | 2727569 | PASS (no restarts — ~11h uptime) |
