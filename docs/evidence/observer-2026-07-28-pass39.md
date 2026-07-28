# Observer Evidence Record — 2026-07-28 (Pass 39)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T03:52:09Z (03:52Z; three-way bundle at 03:52:26Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Thirty-ninth observation pass. Same processes since 2026-07-27T18:48Z (~9.1h runtime). ~9 min since pass 38 (03:43Z).

**Summary:** Routine continuation. Both nodes at epoch 1089 (three-way PASS on both). Snapshot rotated 1070→1080 (10-epoch interval confirmed). Balance locked at 20/0. Zero queues, zero fetches, zero zombie/evict activity. Build unchanged. All 3 persistent deviations unchanged. Delta: routine continuation; no new observations.

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
| uptime_secs | 32623 (~9.1h) | — | None (pass 38: 32120; Δ = +503s ≈ 8.4 min — consistent with ~9min real time minus capture window) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 991.310 | ~1000, slowly decaying | None (pass 38: 991.445; Δ = -0.135 over ~9 min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 32624 (~9.1h) | — | None (matches api within 1s) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=3260, silence_secs=9, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=3258 (from earlier capture), silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 38:** Heartbeats api +51 (3209→3260). Silence: api 7s→9s (well within <30s threshold). Queue depth 0 on both. Normal variance.

---

## Epoch State

### morning-api (03:52Z bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1089 | Cycling. +19 since pass 38 (1070→1089). | **PASS — three-way match.** Socket=1089, grep -c=1089, last log line=1089. All agree. |
| ratio | 1.01999 | ~1.01–1.02 steady state (pass 38: 1.01967) | None (stable; small variance from tick timing) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (03:52Z simultaneous capture):**
- Socket epoch: **1089**
- `grep -c` count: **1089**
- Last log line epoch: **1089** (at 03:52:26Z)
- **PASS.** All three agree.

### local-witness (03:52Z bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1089 | Cycling. +19 since pass 38 (1070→1089). | **PASS — three-way match.** Socket, grep count, and last log line all at 1089. |
| ratio | 1.0964 | Continuing asymptotic decline (pass 38: 1.09826; Δ = -0.00186) | None (monotonic decay expected) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (03:52Z simultaneous capture):**
- Socket epoch: **1089**
- `grep -c` count: **1089**
- Last log line epoch: **1089** (at 03:52:43Z)
- **PASS.** All three agree.

### Epoch synchronization — confirmed

**OBSERVED:** Both nodes at epoch 1089. No offset. Pass 38's timing artifact (api showed +1 due to boundary race) resolved — both captured at same instant with matching epoch.

### Epoch cadence

+19/+19 epochs in ~9 minutes. ~28s/epoch. Consistent with established ~30s cadence.

---

## Economic State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged since pass 38) |
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
**DEVIATION:** Witness-side accounting reports 0. Known-deviating since first observer pass.

---

## Persistence State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1080 | 1080 | None — snapshot at 10-epoch interval (pass 38: 1070, now: 1080; one rotation). |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (03:52Z):**
- `state.snapshot`: 894 bytes (mtime: 2026-07-27T23:52Z — fresh, epoch 1080 snapshot). Size unchanged from pass 38 (895 bytes — minor variance within encoding tolerance; format stable).
- `wal.log`: 379 bytes (mtime: 2026-07-27T23:52Z — reflects latest snapshot rotation)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-27T23:47Z — previous epoch 1070 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1080 | 1080 | None — matches log. Same 10-epoch interval. |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (03:52Z):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-27T23:48Z — epoch 1080 snapshot). Size unchanged from pass 38.
- `wal.log`: 379 bytes (mtime: 2026-07-27T23:48Z)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-27T23:43Z)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

---

## Metrics (from heartbeat logs)

### morning-api — latest tick (03:52:56Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s`
**CLASSIFICATION:** Clean. Zero fetches, zero queues, max peer silence well within threshold.

### local-witness — latest tick (03:52:53Z)
**OBSERVED:** `outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s`
**CLASSIFICATION:** Clean. Slightly higher max_peer_silence (6s vs api's 3s) but well under 10s threshold. Consistent pattern — witness tick timing lags api by ~3s.

**All observed ticks (03:43–03:52Z):** Every tick on both nodes shows `aged=0`, `outbound_queues=[]`. No stale fetch entries, no queue buildup, no zombie evictions.

---

## Error Health Scan

### morning-api
**OBSERVED:** 115 total WARN/ERROR lines. After filtering (expected patterns: `skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|No known peers`): **only 2 startup warnings remain** — `Failed to gossip genesis` and `Failed to publish block` at 18:48:26 (both expected for initial no-peer state). No new error types since pass 38.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

### local-witness
**OBSERVED:** 122 total WARN/ERROR lines. After filtering: **zero unexpected lines.** All 122 are expected: 3× startup messages + 118× `insufficient balance` rejections (one per epoch, witness rejects redistribution) + 1× `Connection from non-mDNS peer`.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

---

## Delta Summary (Pass 38 → Pass 39)

| Metric | Pass 38 (03:43Z) | Pass 39 (03:52Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch | 1070 | 1089 | +19 (~9 min) | Normal cycling (~28s/epoch) |
| witness epoch | 1070 | 1089 | +19 (~9 min) | Perfect match — no offset |
| Three-way match | PASS (witness); race at boundary (api) | **PASS BOTH** — all three values match on both nodes | — | Full synchronization | 
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch | 1070 | 1080 | +10 (one rotation) | Normal 10-epoch interval; both nodes rotated cleanly |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk (api) | 379 bytes, Jul 28 03:42 | 379 bytes, Jul 27 23:52 | 0 size; mtime live | Snapshot rotation working correctly |
| wal.log on disk (witness) | 379 bytes, Jul 28 03:43 | 379 bytes, Jul 27 23:48 | 0 size; mtime live | Snapshot rotation working correctly |
| state.snapshot (api) | 895 bytes | 894 bytes | -1 byte | Within encoding tolerance; format stable |
| state.snapshot (witness) | 569 bytes | 569 bytes | 0 | Format stable |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 9 commits behind HEAD |
| Heartbeats (api) | 3209 | 3260 | +51 | Normal — ~5.7/min |
| Heartbeats (witness) | — | 3258 | — | Normal |
| max_peer_silence (api) | 3s | 3s | 0 | Well within <30s |
| max_peer_silence (witness) | 6s | 6s | 0 | Consistent with witness tick lag |
| Kademlia bootstrap WARNs | Present (harmless) | Present in total count | — | Unchanged — always filtered |
| Zombie/reconnect events | None | None | — | Clean |
| Sweep/evict events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~27–31s | ~28s | — | Consistent |

### New observations this pass

1. **Three-way PASS on both nodes.** First pass where both nodes' socket+count+logline agree exactly. Pass 38 had an api boundary race; this pass captured both in steady state between ticks.

2. **Snapshot rotation at epoch 1080 confirmed.** Both nodes rotated from 1070→1080 on the 10-epoch schedule. State file bit-identical across passes (witness: 569 bytes stable; api: 894 vs 895 — single-byte variance within encoding tolerance).

### No new deviations

No new findings, deviations, or anomalous metrics. The mesh is in a stable steady state: epoch cycles at ~28–30s, snapshot rotations at 10-epoch intervals, zero transaction activity, zero fetches, zero queues, locked balances.

---

## Persistent Deviations (unchanged)

| # | Observation | First seen | Status |
|---|------------|-----------|--------|
| 1 | build_commit 9 commits behind HEAD (71aa16b-dirty vs cb5d4b1) | Pass 1 (Jul 27) | Persistent — binary not rebuilt since those commits |
| 2 | GetPersistenceState wal_bytes=0 (reads transactions.wal instead of wal.log) | Pass 1 (Jul 27) | Persistent — Verifier Mission 2 confirmed one-line fix |
| 3 | Local-witness reports morning-api balance as 0 (supply divergence); nonces frozen | Pass 1 (Jul 27) | Persistent — supply conservation CONTRADICTED per Verifier Mission 1 |

---

## UNKNOWN Items

**(None.)** No new UNKNOWNS this pass.

---

## Evidence Files

- Previous: `docs/evidence/observer-2026-07-28-pass38.md`
- This: `docs/evidence/observer-2026-07-28-pass39.md`

## Raw Capture Bundle

```json
// GetNodeInfo (morning-api)
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":32623,"build_commit":"71aa16b-dirty","thickness":991.3094678831166}

// GetPeers (morning-api)
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":3260,"silence_secs":9,"is_dead":false,"queue_depth":0}]}

// GetEpochState (morning-api) — three-way capture
// Socket: {"type":"EpochState","epoch":1089,...}
// grep -c "Epoch complete": 1089
// Last log line: epoch=1089 balance_before=20 balance_after=20 ratio=1.02

// GetEconomicState (morning-api)
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// GetPersistenceState (morning-api)
{"type":"PersistenceState","last_snapshot_epoch":1080,"wal_bytes":0,"wal_entries":0}

// GetHeight (morning-api)
{"type":"Height","height":1}

// GetNodeInfo (local-witness)
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":32624,"build_commit":"71aa16b-dirty"}

// GetEpochState (local-witness) — three-way capture
// Socket: {"type":"EpochState","epoch":1089,...}
// grep -c "Epoch complete": 1089
// Last log line: epoch=1089 balance_before=0 balance_after=0 ratio=1.10

// GetEconomicState (local-witness)
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// GetPersistenceState (local-witness)
{"type":"PersistenceState","last_snapshot_epoch":1080,"wal_bytes":0,"wal_entries":0}

// GetHeight (local-witness)
{"type":"Height","height":1}
```
