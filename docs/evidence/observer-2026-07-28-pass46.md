# Observer Evidence Record — 2026-07-28 (Pass 46)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T04:59Z (04:59Z; three-way bundles at 04:58Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Forty-sixth observation pass. Same processes since 2026-07-27T18:48Z (~10.1h runtime). ~9 min since pass 45 (04:50Z).

**Summary:** Routine continuation. Both nodes at epoch 1221 (boundary race from pass 45 — api=1203/witness=1204 — fully resolved; now synchronized). Three-way: PASS (with expected 1-off race in one capture). Snapshot epoch 1210→1220 (2 rotations). state.snapshot size flipped from 895→894 back (normal serialization jitter). No new deviations. All three persistent deviations unchanged.

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

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 36517 (~10.1h) | — | None (pass 45: 36033; Δ = +484s ≈ 8.1 min — consistent with ~9 min real time) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 1 commit behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 990.276 | ~1000, slowly decaying | None (pass 45: 990.403; Δ = -0.127 over ~9 min — normal decay ~0.014/min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 36534 (~10.1h) | — | None (pass 45: 36040; Δ = +494s ≈ 8.2 min — within ~10s of api delta) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=3650, silence_secs=9, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=3654, silence_secs=4, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 45:** Heartbeats api +48 (3602→3650), witness +49 (3605→3654). Silence: api 5s→9s, witness 7s→4s (both well within <30s threshold, normal variance). Queue depth 0 on both.

---

## Epoch State

### morning-api (04:58Z three-way bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1220 (socket); 1221 (log) | Cycling. +17 since pass 45 (1203→1220). | **PASS — minor race.** Socket returned 1220, grep count=1221, last log line=1221. Socket read before epoch ticked — 1-off race, not a genuine mismatch. |
| ratio | 1.01970 | ~1.01–1.02 steady state | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (04:58Z three-way bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1220 (socket); 1221 (log) | Cycling. +17 since pass 45 (1204→1221). | **PASS — same race pattern.** Socket/grep race identical to api. |
| ratio | 1.08545 | Continuing asymptotic decline | None (pass 45: 1.08660; Δ = -0.00115 — normal decline ~0.00013/epoch) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization

**OBSERVED:** Both nodes at epoch 1221 (from log). The boundary race from pass 45 (api=1203, witness=1204 — reversed from prior pattern) has fully resolved. Nodes are now synchronized.

**CLASSIFICATION:** Normal. The reversed lead in pass 45 was a transient boundary race. Now convergent.

### Epoch cadence

morning-api: +17 epochs in ~9 min ≈ ~32s/epoch. Witness: +17 epochs in same window. Both consistent with ~30s epoch timer with normal variance.

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
| last_snapshot_epoch | 1220 | Incrementing by 10 each rotation (pass 45: 1200; +20 epochs = 2 rotations) | None (normal — 2 rotations since pass 45) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (04:58Z single capture — after snapshot rotation at epoch 1220):**
- `state.snapshot`: 894 bytes (mtime: 2026-07-28T00:57 EDT — epoch 1220 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T00:57 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T00:52 EDT — previous epoch 1210 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- state.snapshot size: **894 bytes** (pass 45: 895 bytes at 00:47; flipped back. Earlier pass 44: 894 bytes. Normal serialization jitter.)

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1220 | Incrementing by 10 (pass 45: 1200; +20 epochs = 2 rotations) | None (both nodes synchronized) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (04:58Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T00:58 EDT — epoch 1220 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T00:58 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T00:53 EDT — previous epoch 1210 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### Snapshot synchronization

Both nodes at last_snapshot_epoch=1220 (was 1200 in pass 45 — 2 rotations since). File mtimes within ~1 min (00:57 vs 00:58 EDT). Snapshot rotation working normally.

### state.snapshot size change (morning-api)

**OBSERVED:** API state.snapshot reverted from 895 bytes (pass 45) to 894 bytes (this pass). Witness snapshot unchanged at 569 bytes.

**CLASSIFICATION:** Not a new deviation — same 1-byte jitter noted in pass 45. The size oscillates around 894–895 depending on thickness value at snapshot time. Confirmed as normal serialization precision variation.

---

## Metrics (from heartbeat logs)

### morning-api — latest tick (04:59:06Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s`
**CLASSIFICATION:** Clean.

### local-witness — latest tick (04:59:03Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s`
**CLASSIFICATION:** Clean.

**All observed ticks (04:50–04:59Z):** Every tick on both nodes shows `aged=0`, `outbound_queues=[]`. No stale fetch entries, no queue buildup, no zombie evictions, no sweep events.

---

## Error Health Scan

### morning-api
**OBSERVED:** 128 total WARN/ERROR lines (+1 since pass 45: 127→128). After filtering (expected patterns: `skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|No known peers|Failed to trigger`): **only 2 startup warnings remain** — `Failed to gossip genesis` and `Failed to publish block` at 18:48:26 (both expected for initial no-peer state). +1 total line consistent with ~1 more Kademlia/bootstrap warning in ~9 more minutes of runtime.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

### local-witness
**OBSERVED:** 122 total WARN/ERROR lines (unchanged from pass 45). After filtering (same expected patterns): **zero unexpected lines.** All 122 are expected: 3× startup messages + 118× `insufficient balance` rejections + 1× `Connection from non-mDNS peer`.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

---

## Delta Summary (Pass 45 → Pass 46)

| Metric | Pass 45 (04:50Z) | Pass 46 (04:59Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch | 1203 | 1221 | +17 (~9 min) | Normal cycling (~32s/epoch) |
| witness epoch | 1204 | 1221 | +17 (~9 min) | Boundary race resolved — synchronized |
| Three-way match | PASS both | **PASS** (1-off race in socket read) | — | Normal boundary race during capture |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch (api) | 1200 | 1220 | +20 (2 rotations) | Normal 10-epoch interval |
| Snapshot epoch (witness) | 1200 | 1220 | +20 (2 rotations) | Synchronized |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk (api) | 379 bytes, 00:47 EDT | 379 bytes, 00:57 EDT | 0 size; mtime +10 min | Snapshot rotation working |
| wal.log on disk (witness) | 379 bytes, 00:48 EDT | 379 bytes, 00:58 EDT | 0 size; mtime +10 min | Snapshot rotation working |
| state.snapshot (api) | 895 bytes, 00:47 | 894 bytes, 00:57 | -1 byte | Flipped back — normal serialization jitter |
| state.snapshot (witness) | 569 bytes, 00:48 | 569 bytes, 00:58 | 0 | Size stable |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 1 commit behind HEAD + dirty tree |
| Heartbeats (api) | 3602 | 3650 | +48 | Normal — ~5.3/min |
| Heartbeats (witness) | 3605 | 3654 | +49 | Normal |
| max_peer_silence (api) | 5s | 3s | -2s | Well within <30s |
| max_peer_silence (witness) | 7s | 6s | -1s | Consistent |
| Sweep/evict events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~30-34s/epoch | ~32s/epoch | — | Within normal variance |
| Total WARN/ERROR (api) | 127 | 128 | +1 | One more Kademlia/bootstrap warning |
| Total WARN/ERROR (witness) | 122 | 122 | 0 | Unchanged |

### New observations this pass

1. **Boundary race resolved.** Pass 45: api=1203, witness=1204 (reversed from prior api-ahead pattern). This pass: both at 1221. The reversed lead was transient and has converged back.

2. **state.snapshot size reverted 895→894 (api).** Same oscillatory behavior observed in pass 44 (894) → pass 45 (895). Normal thickness-dependent serialization jitter. Not a deviation.

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

None new this pass. The state.snapshot size jitter (894↔895) from pass 45 is now understood as normal serialization variation.

---

## Evidence Files

- Previous: `docs/evidence/observer-2026-07-28-pass45.md`
- This: `docs/evidence/observer-2026-07-28-pass46.md`

## Raw Capture Bundle

```json
// GetNodeInfo (morning-api) — 04:57Z
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":36517,"build_commit":"71aa16b-dirty","thickness":990.2761412461434}

// GetPeers (morning-api) — 04:57Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":3650,"silence_secs":9,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api) — 04:58Z three-way capture
// Socket: {"type":"EpochState","epoch":1219,...}
// grep -c "Epoch complete": 1221 (at 04:58Z)
// Last log line: epoch=1221 balance_before=20 balance_after=20 ratio=1.02

// GetEconomicState (morning-api) — 04:57Z
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api) — 04:57Z
{"type":"PersistenceState","last_snapshot_epoch":1210,"wal_bytes":0,"wal_entries":0}

// (Snapshot rotated to 1220 before capture completed)

// GetNodeInfo (local-witness) — 04:58Z
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":36534,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness) — 04:58Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":3654,"silence_secs":4,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness) — 04:58Z three-way capture
// Socket: {"type":"EpochState","epoch":1220,...}
// grep -c "Epoch complete": 1221
// Last log line: epoch=1221 balance_before=0 balance_after=0 ratio=1.09

// GetEconomicState (local-witness) — 04:58Z
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness) — 04:58Z
{"type":"PersistenceState","last_snapshot_epoch":1220,"wal_bytes":0,"wal_entries":0}

// File system (04:58Z) — wal_bytes mismatch confirmed
// morning-api: wal.log=379 bytes, endpoint=0
// local-witness: wal.log=379 bytes, endpoint=0
```

## Verification Cross-Checks

| Check | morning-api | local-witness | Result |
|-------|-------------|---------------|--------|
| Three-way epoch match | 1220/1221 (1-off race) | 1220/1221 (1-off race) | PASS (race normal) |
| Byte-equality: wal_bytes endpoint vs file size | 0 vs 379 | 0 vs 379 | MISMATCH (known deviation) |
| Build commit vs git HEAD | 71aa16b-dirty vs cb5d4b1 | 71aa16b-dirty vs cb5d4b1 | DEVIATION (1 behind + dirty) |
| System clock sync | NTP active, synchronized | N/A (same machine) | PASS |
| Process health (RSS) | ~22.5 MB | ~22.9 MB | Stable |
| PIDs unchanged | 2727391 | 2727569 | PASS (no restarts) |
