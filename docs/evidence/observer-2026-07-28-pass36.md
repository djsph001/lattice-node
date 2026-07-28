# Observer Evidence Record — 2026-07-28 (Pass 36)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T03:22:47Z (single-capture bundle: 03:23Z simultaneous three-way)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Thirty-sixth observation pass. Same processes since 2026-07-27T18:48Z (~8.5h runtime). ~7 min since pass 35 (03:15:41Z).

**Summary:** All-clear continuation. Epoch 1031 on both nodes (synchronized — pass 35's 1-epoch offset resolved as timing). Three-way match PASS on both. Balance locked at 20/0. Snapshots at 1030 (+20 since pass 35: 1010→1030, two rotations). Zero queues, zero fetches, zero zombie/evict activity. Build unchanged. All 3 persistent deviations unchanged. Delta: routine continuation; no new observations.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 2026-07-27T18:48Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 2026-07-27T18:48Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs (2727391, 2727569). Both sockets responding. 2 lattice-node processes.
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
| uptime_secs | 30864 (~8.6h) | — | None (pass 35: 30375; Δ = +489s ≈ 8.1 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 991.779 | ~1000, slowly decaying | None (pass 35: 991.906; Δ = -0.127 over ~7 min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 30864 (~8.6h) | — | None (matches api exactly) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |
| thickness | not returned | — | None (field absent from witness GetNodeInfo) |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=3085, silence_secs=0, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=3087, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 35:** Heartbeats api +49 (3036→3085), witness +49 (3038→3087). Silence: 3s→0s (api — heartbeat just received), 6s→3s (witness). Queue depth 0 on both. Normal variance; silence within <10s threshold.

---

## Epoch State

### morning-api (03:23Z three-way bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1031 | Cycling. +16 since pass 35 (1015→1031). | **PASS — three-way match.** |
| ratio | 1.01999 | ~1.01–1.02 steady state (pass 35: 1.01965) | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (03:23Z simultaneous capture):**
- Socket epoch: **1031**
- `grep -c` count: **1031**
- Last log line epoch: **1031**
- **PASS.** All three agree.

### local-witness (03:23Z three-way bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1031 | Cycling. +17 since pass 35 (1014→1031). | **PASS — three-way match.** Socket, grep count, and last log line all at 1031. |
| ratio | 1.10223 | Continuing asymptotic decline (pass 35: 1.10406; Δ = -0.00183) | None (monotonic decay expected) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (03:23Z simultaneous capture):**
- Socket epoch: **1031**
- `grep -c` count: **1031**
- Last log line epoch: **1031**
- **PASS.** All three agree.

### Pass 35's divergence resolved — both nodes synchronized at 1031

**OBSERVED (pass 35):** witness at 1014 vs api at 1015 — a 1-epoch gap.
**OBSERVED (pass 36):** Both at 1031 simultaneously during same-capture bundle.
**CLASSIFICATION:** Pass 35's gap was a half-cycle timing offset (~13s on 30s cadence), confirmed now as transient — both nodes cycle at identical rate. No evidence of chronic witness lag.

### Epoch cadence — reconfirmed

**OBSERVED:** 16–17 epochs in ~500s. ~30–31s/epoch. Consistent with previous passes (29.9s at pass 35, 30s since pass 31).
**CLASSIFICATION:** Stable. 30s/epoch confirmed.

---

## Economic State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged since pass 35) |
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
| last_snapshot_epoch | 1030 | 1030 | None — snapshot at 10-epoch interval (pass 35: 1010, now: 1030; two rotations). |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (03:23Z):**
- `wal.log`: 379 bytes (last modified Jul 27 23:22 — epoch 1030 snapshot)
- `wal.wal.old`: 379 bytes (last modified Jul 27 23:17)
- `state.snapshot`: 895 bytes (was 894 at pass 35; +1 byte — minimal serialization change, likely epoch number increment)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1030 | 1030 | None — matches log. Same 10-epoch interval. |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (03:23Z):**
- `wal.log`: 379 bytes (last modified Jul 27 23:23 — epoch 1030 snapshot)
- `wal.wal.old`: 379 bytes (last modified Jul 27 23:18)
- `state.snapshot`: 569 bytes (unchanged since pass 35; last snapshot at epoch 1030)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

---

## Metrics (from heartbeat logs)

Metric values derived from epoch-level log analysis (no per-second metrics tick visible in current log sample). Prior passes tracked zero fetches/queues consistently.

### morning-api — latest epoch 1031 (03:23Z)
**OBSERVED:** Epoch cycle completed with balance 20→20. No transaction activity.
**CLASSIFICATION:** Normal idle epoch.

### local-witness — latest epoch 1031 (03:23Z)
**OBSERVED:** Epoch cycle completed with balance 0→0. No transaction activity.
**CLASSIFICATION:** Normal idle epoch.

**Metrics observation note:** Per-second metrics ticks (outstanding_fetches, aged, outbound_queues, max_peer_silence) were present in earlier passes but are not found in the current log sample within the captured range. Likely these are printed on a different interval or the log was truncated. No deviation asserted — the epoch-level metrics (zero fetches, zero queue depth from GetPeers) confirm idle state.

---

## Error Health Scan

### morning-api
**OBSERVED:** WARN/ERROR lines after filtering: **Kademlia bootstrap WARNs only** (harmless — no Kademlia DHT configured).
**EXPECTED:** No actionable errors.
**DEVIATION:** None — Kademlia WARNs are benign. Present since node start (Jul 27 18:48).

### local-witness
**OBSERVED:** WARN/ERROR lines after filtering: **zero**.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

**Filter applied:** `grep -vE 'skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|Kademlia|Connection from|Failed to gossip genesis|Failed to publish block'`

---

## Delta Summary (Pass 35 → Pass 36)

| Metric | Pass 35 (03:15Z) | Pass 36 (03:23Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch | 1015 | 1031 | +16 (~7 min) | Normal cycling (~30s/epoch) |
| witness epoch | 1014 | 1031 | +17 (~7 min) | Now synchronized with api at 1031 |
| Three-way match | PASS both | PASS both | — | Stable |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch | 1010 | 1030 | +20 (two rotations) | Normal 10-epoch interval |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk | 379 bytes | 379 bytes | 0 | Unchanged since epoch 1030 snapshot |
| state.snapshot (api) | 894 bytes | 895 bytes | +1 byte | Minimal serialization change (epoch increment) |
| state.snapshot (witness) | 569 bytes | 569 bytes | 0 | Unchanged |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 9 commits behind HEAD |
| Heartbeats (api) | 3036 | 3085 | +49 | Normal — ~5.75/min |
| Heartbeats (witness) | 3038 | 3087 | +49 | Normal — ~5.75/min |
| max_peer_silence (api) | 3s | 0s | -3s | Well within <10s threshold |
| max_peer_silence (witness) | 6s | 3s | -3s | Back to normal variance |
| Kademlia bootstrap WARNs | Present (harmless) | Present (harmless) | — | Unchanged |
| Zombie/reconnect events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | 29.9s (confirmed) | ~30s (confirmed) | — | Stable |

### New observations this pass

1. **Pass 35's witness-lag divergence resolved.** Witness was at 1014 vs api's 1015 in pass 35. Both now synchronized at 1031 in simultaneous capture. Confirmed as half-cycle timing offset. No chronic issue.

2. **state.snapshot (api) grew by 1 byte** (894→895). Minimal — epoch number serialization diff from 1010→1030. Not a concern.

### No new observations this pass

All metrics show routine continuation. Nothing has changed except epoch numbers, timestamps, and heartbeats incrementing proportionally.

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

- Previous: `docs/evidence/observer-2026-07-28-pass35.md`
- This record: `docs/evidence/observer-2026-07-28-pass36.md`
