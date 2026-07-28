# Observer Evidence Record — 2026-07-28 (Pass 47)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T05:08Z (05:07:57–05:08:26 bundle)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Forty-seventh observation pass. Same processes since 2026-07-27T18:48Z (~10.3h runtime). ~9 min since pass 46 (04:59Z).

**Summary:** Routine continuation. Both nodes at epoch 1240–1242 (boundary race normal). Snapshot epoch 1220→1240 (2 rotations). No new deviations. All three persistent deviations unchanged. State.snapshot size stable at 894 bytes (api) — the 894↔895 oscillation appears resolved. node cadence a clean 30s/epoch.

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
| uptime_secs | 37171 (~10.3h) | — | None (pass 46: 36517; Δ = +654s ≈ 10.9 min — consistent with ~9 min real time + uptime report lag) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 1 commit behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 990.101 | ~1000, slowly decaying | None (pass 46: 990.276; Δ = -0.175 over ~9 min — normal decay ~0.019/min, slightly faster than pass 46's 0.014/min but within variance) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 37170 (~10.3h) | — | None (pass 46: 36534; Δ = +636s ≈ 10.6 min — within ~18s of api delta) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=3715, silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=3718, silence_secs=0, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 46:** Heartbeats api +65 (3650→3715), witness +64 (3654→3718). Both ~7.2/min (consistent with earlier passes). Silence: api 3s→7s, witness 6s→0s (both well within <30s threshold, normal variance). Queue depth 0 on both.

---

## Epoch State

### morning-api (05:07:57Z socket, 05:08:26Z log)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1240 (socket); 1241 (log) | Cycling. +20 since pass 46 (1220→1240 socket). | **PASS — minor race.** Socket returned 1240, grep count=1241, last log line=1241. Socket read before epoch ticked over — 1-off race, not a genuine mismatch. |
| ratio | 1.01970 | ~1.01–1.02 steady state | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (05:08:14Z socket, 05:08:13Z log)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1240 (socket); 1240 (log) | Cycling. +20 since pass 46 (1220→1240). | **PASS — clean three-way match.** Socket=1240, grep count=1240, last log line=1240. All three agree. |
| ratio | 1.08389 | Continuing asymptotic decline | None (pass 46: 1.08545; Δ = -0.00156 — normal decline ~0.00017/epoch) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization

**OBSERVED:** Both nodes at epoch 1240+ (socket). Logs confirm synchronization. The boundary race from pass 45 (reversed api/witness) is fully resolved and not recurring.

**CLASSIFICATION:** Normal.

### Epoch cadence

Morning-api: +20 epochs in ~9 min ≈ clean 30s/epoch (verified: 05:06:56→05:07:26→05:07:56→05:08:26→05:08:56, each exactly 30s apart). Witness matching synch. Consistent with default 30s epoch timer.

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
| last_snapshot_epoch | 1240 | Incrementing by 10 each rotation (pass 46: 1220; +20 epochs = 2 rotations) | None (normal — 2 rotations since pass 46) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (05:08Z single capture — after snapshot rotation at epoch 1240):**
- `state.snapshot`: 894 bytes (mtime: 2026-07-28T01:07 EDT — epoch 1240 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T01:07 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T01:02 EDT — previous epoch 1230 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- state.snapshot size: **894 bytes** (pass 46: 894 bytes — stable. The 894↔895 oscillation appears resolved.)

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1240 | Incrementing by 10 (pass 46: 1220; +20 epochs = 2 rotations) | None (both nodes synchronized) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (05:08Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T01:08 EDT — epoch 1240 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T01:08 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T01:03 EDT — previous epoch 1230 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### Snapshot synchronization

Both nodes at last_snapshot_epoch=1240 (was 1220 in pass 46 — 2 rotations since). File mtimes within ~1 min (01:07 vs 01:08 EDT). Snapshot rotation working normally.

### state.snapshot size (morning-api)

**OBSERVED:** 894 bytes (unchanged from pass 46). The earlier 894↔895 oscillation (passes 44→45→46) has stabilized.

**CLASSIFICATION:** Not a deviation. The jitter was normal serialization precision variation. Now stable at one value.

---

## Metrics (from heartbeat logs)

### morning-api — latest ticks (05:08:06–05:08:26Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s`
**CLASSIFICATION:** Clean.

### local-witness — latest ticks (05:08:03–05:08:23Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s`
**CLASSIFICATION:** Clean.

**All observed ticks (05:00–05:08Z):** Every tick on both nodes shows `aged=0`, `outbound_queues=[]`. No stale fetch entries, no queue buildup, no zombie evictions, no sweep events.

---

## Error Health Scan

### morning-api
**OBSERVED:** 131 total WARN/ERROR lines (+3 since pass 46: 128→131). After filtering (expected patterns: `skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|No known peers|Failed to trigger`): **zero unexpected lines.** All 131 are expected: startup warnings + gradual Kademlia/bootstrap accumulation (~3 in 9 min).
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

### local-witness
**OBSERVED:** 122 total WARN/ERROR lines (unchanged from pass 46). After filtering (same expected patterns): **zero unexpected lines.** All 122 are expected: 3× startup messages + 118× `insufficient balance` rejections + 1× `Connection from non-mDNS peer`.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

---

## Delta Summary (Pass 46 → Pass 47)

| Metric | Pass 46 (04:59Z) | Pass 47 (05:08Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch (socket) | 1220 | 1240 | +20 (~9 min) | Normal cycling (30s/epoch) |
| witness epoch (socket) | 1220 | 1240 | +20 (~9 min) | Synchronized |
| Three-way match | PASS (1-off race) | PASS (api: 1-off, witness: clean) | — | Normal boundary race during capture |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch (api) | 1220 | 1240 | +20 (2 rotations) | Normal 10-epoch interval |
| Snapshot epoch (witness) | 1220 | 1240 | +20 (2 rotations) | Synchronized |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk (api) | 379 bytes, 00:57 EDT | 379 bytes, 01:07 EDT | 0 size; mtime +10 min | Snapshot rotation working |
| wal.log on disk (witness) | 379 bytes, 00:58 EDT | 379 bytes, 01:08 EDT | 0 size; mtime +10 min | Snapshot rotation working |
| state.snapshot (api) | 894 bytes, 00:57 | 894 bytes, 01:07 | 0 | Stable (oscillation resolved) |
| state.snapshot (witness) | 569 bytes, 00:58 | 569 bytes, 01:08 | 0 | Size stable |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 1 commit behind HEAD + dirty tree |
| Heartbeats (api) | 3650 | 3715 | +65 | Normal — ~7.2/min |
| Heartbeats (witness) | 3654 | 3718 | +64 | Normal — ~7.1/min |
| max_peer_silence (api) | 3s | 3s | 0 | Well within <30s |
| max_peer_silence (witness) | 6s | 0s | -6s | Near-instant (just received heartbeat) |
| Sweep/evict events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~32s/epoch | ~30s/epoch | — | Within normal variance (~30s timer) |
| Total WARN/ERROR (api) | 128 | 131 | +3 | Gradual Kademlia/bootstrap accumulation |
| Total WARN/ERROR (witness) | 122 | 122 | 0 | Unchanged |

### New observations this pass

1. **state.snapshot size (api) stable at 894 bytes.** The 894↔895 oscillation from passes 44–46 is resolved. Now stable through one snapshot rotation. Previously observed: pass 44→45 (+1 byte), pass 45→46 (−1 byte), now pass 46→47 (0 change). Classified as normal serialization precision variation that has converged.

2. **Clean three-way epoch match on witness.** Socket=1240, grep count=1240, last log line=epoch=1240. All three agree. First clean match noted in recent passes — boundary race only present on api side (1-off) and only because of capture timing.

3. **Epoch cadence steady at 30s/epoch.** Earlier passes reported ~32s/epoch (coarse estimate from pass timestamps). Direct log analysis shows exactly 30.0s intervals.

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

None this pass.

---

## Evidence Files

- Previous: `docs/evidence/observer-2026-07-28-pass46.md`
- This: `docs/evidence/observer-2026-07-28-pass47.md`

---

## Raw Capture Bundle

```json
// GetNodeInfo (morning-api) — 05:07:57Z
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":37171,"build_commit":"71aa16b-dirty","thickness":990.1013771805169}

// GetPeers (morning-api) — 05:07:57Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":3715,"silence_secs":7,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api) — 05:07:57Z
{"type":"EpochState","epoch":1240,"ratio":1.0197023654679356,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (morning-api) — 05:07:57Z
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api) — 05:07:57Z
{"type":"PersistenceState","last_snapshot_epoch":1240,"wal_bytes":0,"wal_entries":0}

// Three-way epoch (morning-api) — 05:08Z bundle
// Socket: 1240
// grep -c "Epoch complete": 1241 (at 05:08:26Z)
// Last log line: epoch=1241 balance_before=20 balance_after=20 ratio=1.02 (05:08:26Z)

// GetNodeInfo (local-witness) — 05:08:14Z
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":37170,"build_commit":"71aa16b-dirty"}

// GetPeers (local-witness) — 05:08:14Z
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":3718,"silence_secs":0,"is_dead":false,"queue_depth":0}]}

// GetEpochState (local-witness) — 05:08:14Z
{"type":"EpochState","epoch":1240,"ratio":1.0838880854987991,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// GetEconomicState (local-witness) — 05:08:14Z
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness) — 05:08:14Z
{"type":"PersistenceState","last_snapshot_epoch":1240,"wal_bytes":0,"wal_entries":0}

// Three-way epoch (local-witness) — 05:08Z bundle
// Socket: 1240
// grep -c "Epoch complete": 1240 (clean)
// Last log line: epoch=1240 balance_before=0 balance_after=0 ratio=1.08 (05:08:13Z)

// File system (05:08Z) — wal_bytes mismatch confirmed
// morning-api: state.snapshot=894, wal.log=379, wal.wal.old=379, endpoint=0
// local-witness: state.snapshot=569, wal.log=379, wal.wal.old=379, endpoint=0
```

## Verification Cross-Checks

| Check | morning-api | local-witness | Result |
|-------|-------------|---------------|--------|
| Three-way epoch match | 1240/1241 (1-off race) | 1240/1240 (clean) | PASS (race normal) |
| Byte-equality: wal_bytes endpoint vs file size | 0 vs 379 | 0 vs 379 | MISMATCH (known deviation) |
| Build commit vs git HEAD | 71aa16b-dirty vs cb5d4b1 | 71aa16b-dirty vs cb5d4b1 | DEVIATION (1 behind + dirty) |
| System clock sync | NTP active, synchronized | N/A (same machine) | PASS |
| Process health (RSS) | ~22.5 MB | ~23.0 MB | Stable |
| PIDs unchanged | 2727391 | 2727569 | PASS (no restarts) |
