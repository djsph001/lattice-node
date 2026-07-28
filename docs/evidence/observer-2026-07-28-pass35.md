# Observer Evidence Record — 2026-07-28 (Pass 35)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T03:15:41Z (single-capture bundle: 03:15:26–03:15:41Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Thirty-fifth observation pass. Same processes since 2026-07-27T18:48Z (~8.4h runtime). ~8.5 min since pass 34 (03:07:13Z).

**Summary:** All-clear continuation. Epochs 1015/1014 (api/witness; +17/+16 since pass 34). Three-way match PASS on both — witness lags by 1 epoch (timing offset, normal). Balance locked at 20/0. Snapshot at 1010 (+20 since pass 34: 990→1010, two rotations). Zero queues, zero fetches, zero zombie/evict activity. Build unchanged. All 3 persistent deviations unchanged. Delta: routine continuation; no new observations.

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
| uptime_secs | 30375 (~8.4h) | — | None (pass 34: 29866; Δ = +509s ≈ 8.5 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 991.906 | ~1000, slowly decaying | None (pass 34: 992.04; Δ = -0.134 over ~8.5 min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 30376 (~8.4h) | — | None (matches api within 1s) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |
| thickness | not returned | — | None (field absent from witness GetNodeInfo) |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=3036, silence_secs=4, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=3038, silence_secs=6, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 34:** Heartbeats api +51 (2985→3036), witness +49 (2989→3038). Silence: 3s→4s (api), 3s→6s (witness) — normal variance. Queue depth 0 on both.

---

## Epoch State

### morning-api (03:15:26Z three-way bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1015 | Cycling. +17 since pass 34 (998→1015). | **PASS — three-way match.** |
| ratio | 1.01965 | ~1.01–1.02 steady state (pass 34: 1.01965) | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (03:15:26Z):**
- Socket epoch: **1015**
- `grep -c` count: **1015**
- Last log line epoch: **1015**
- **PASS.** All three agree.

### local-witness (03:15:13Z three-way bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1014 | Cycling. +16 since pass 34 (998→1014). | **PASS — three-way match.** Socket, grep count, and last log line all at 1014. |
| ratio | 1.10406 | Continuing asymptotic decline (pass 34: 1.10583; Δ = -0.00177) | None (monotonic decay expected) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (03:15:13Z):**
- Socket epoch: **1014**
- `grep -c` count: **1014**
- Last log line epoch: **1014**
- **PASS.** All three agree.

### Witness lags api by 1 epoch (timing offset, not chronic)

**OBSERVED:** During the simultaneous bundle capture (03:15:13Z–03:15:26Z), api at epoch 1015, witness at epoch 1014. Witness last log line at 03:15:13Z, api last log line at 03:15:26Z — ~13s cycle offset.
**EXPECTED:** Both at same epoch (pass 34 showed both at 998 simultaneously).
**DEVIATION:** None — this is a half-cycle timing offset (~13s on a 30s cadence). Both nodes cycle at identical rate. No evidence of chronic witness lag — it was synchronized at pass 34 (both at 998) and is now offset by 1 epoch, consistent with independent epoch timers starting at slightly different wall times.

### Epoch cadence — reconfirmed

**OBSERVED:** 17 epochs on api in ~509s (03:07:13Z→03:15:42Z). 509s / 17 epochs = **29.9s/epoch**. Reconfirmed.
**Classification:** Stable. 30s/epoch confirmed since pass 31.

---

## Economic State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged since pass 34) |
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
| last_snapshot_epoch | 1010 | 1010 | None — snapshot at 10-epoch interval (pass 34: 990, now: 1010; two rotations). |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (03:15Z):**
- `wal.log`: 379 bytes (last modified Jul 27 23:12 — epoch 1010 snapshot? Unchanged since pass 34)
- `wal.wal.old`: 379 bytes (unchanged)
- `state.snapshot`: 894 bytes (last snapshot at epoch 1010)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1010 | 1010 | None — matches log. Same 10-epoch interval. |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (03:15Z):**
- `wal.log`: 379 bytes (last modified Jul 27 23:13 — epoch 1010 snapshot)
- `wal.wal.old`: 379 bytes (unchanged)
- `state.snapshot`: 569 bytes (last snapshot at epoch 1010)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

---

## Metrics (from heartbeat logs)

### morning-api — latest metrics tick (~03:15Z)

| Metric | Value | Expected | Deviation |
|--------|-------|----------|-----------|
| outstanding_fetches | 0 | 0 | None |
| aged (fetches >10×timeout) | 0 | 0 | None |
| outbound_queues | [] | (empty) | None |
| max_peer_silence | 3s | <10s | None |

### local-witness — latest metrics tick (~03:15Z)

| Metric | Value | Expected | Deviation |
|--------|-------|----------|-----------|
| outstanding_fetches | 0 | 0 | None |
| aged (fetches >10×timeout) | 0 | 0 | None |
| outbound_queues | [] | (empty) | None |
| max_peer_silence | 6s | <10s | None |

---

## Error Health Scan

### morning-api
**OBSERVED:** WARN/ERROR lines after filtering: **Kademlia bootstrap WARNs** (continuing — harmless, no Kademlia DHT configured).
**EXPECTED:** No actionable errors.
**DEVIATION:** None — Kademlia WARNs are benign. Present since node start (Jul 27 18:48).

### local-witness
**OBSERVED:** WARN/ERROR lines after filtering: **zero**.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

**Filter applied:** `grep -vE 'skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|Kademlia|Connection from|Failed to gossip genesis|Failed to publish block'` — Kademlia WARNs on api only. Witness clean.

---

## Delta Summary (Pass 34 → Pass 35)

| Metric | Pass 34 (03:07Z) | Pass 35 (03:15Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch | 998 | 1015 | +17 (~8.5 min) | Normal cycling (30s/epoch) |
| witness epoch | 998 | 1014 | +16 (~8.5 min) | 1-epoch timing offset from api |
| Three-way match | PASS both | PASS both | — | Stable |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch | 990 | 1010 | +20 (two rotations) | Normal 10-epoch interval |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk | 379 bytes | 379 bytes | 0 | Unchanged since epoch 1010 snapshot |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 9 commits behind HEAD |
| Heartbeats (api) | 2985 | 3036 | +51 | Normal — ~6/min |
| Heartbeats (witness) | 2989 | 3038 | +49 | Normal — ~6/min |
| max_peer_silence (api) | 3s | 3s | 0 | Stable |
| max_peer_silence (witness) | 3s | 6s | +3s | Still within <10s threshold |
| Kademlia bootstrap WARNs | Present (harmless) | Present (harmless) | — | Unchanged |
| Zombie/reconnect events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | 30s (confirmed) | 29.9s (re-confirmed) | — | Stable |

### New observations this pass

1. **Witness lags api by 1 epoch during bundle capture** (1015 vs 1014). Confirmed as half-cycle timing offset (~13s of 30s epoch). Both passed three-way match independently. Not a chronic lag — pass 34 showed both synchronized at 998. Re-check next pass for persistent divergence.

2. **Witness max_peer_silence increased to 6s** (from 3s at pass 34). Still well within <10s threshold. Normal variance in heartbeat delivery timing.

---

## Persistent Deviations (unchanged)

| # | Observation | First seen | Status |
|---|------------|-----------|--------|
| 1 | build_commit 9 commits behind HEAD (71aa16b-dirty vs cb5d4b1) | Pass 1 (Jul 27) | Persistent — binary not rebuilt since those commits |
| 2 | GetPersistenceState wal_bytes=0 (reads transactions.wal instead of wal.log) | Pass 1 (Jul 27) | Persistent — Verifier Mission 2 confirmed one-line fix |
| 3 | Local-witness reports morning-api balance as 0 (supply divergence); nonce frozen | Pass 1 (Jul 27) | Persistent — supply conservation CONTRADICTED per Verifier Mission 1 |

---

## UNKNOWN Items

**(None.)** No new UNKNOWNS this pass.

---

## Evidence Files

- Previous: `docs/evidence/observer-2026-07-28-pass34.md`
- This record: `docs/evidence/observer-2026-07-28-pass35.md`
