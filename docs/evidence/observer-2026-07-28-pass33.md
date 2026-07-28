# Observer Evidence Record — 2026-07-28 (Pass 33)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T02:54:13Z (single-capture bundle: 02:53:52–02:54:13Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Thirty-third observation pass. Same processes since 2026-07-27T18:48Z (~8.1h runtime). ~7.5 min since pass 32 (02:46:11Z).

**Summary:** All-clear continuation. Epochs 972/972 (+16/+17 since pass 32 — witness caught up). **First time both nodes at identical epoch in this evidence series.** Three-way epoch match PASS on both. Balance locked at 20 (morning-api) / 0 (witness). Snapshots at 970 (+20 since pass 32: 950→970, two rotations). Zero queues, zero fetches, zero sweep/evict/zombie activity. Build unchanged. All 3 persistent deviations unchanged. One notable new observation: witness epoch converged to match api (both at 972).

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
- Socket `/tmp/witness-b/lattice.sock` — **Connection refused** (historical/stale)

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
| uptime_secs | 29137 (~8.1h) | — | None (pass 32: 28606; Δ = +531s ≈ 8.9 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind. Docs-only + test fixes since binary build. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 992.23 | ~1000, slowly decaying | None (pass 32: 992.38; Δ = -0.15 over ~8 min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 29137 (~8.1h) | — | None (matches api — both captured same instant) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=2906, silence_secs=0, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=2909, silence_secs=9, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 32:** Heartbeats api +47 (2859→2906), witness +46 (2863→2909). Silence: 2s→0s (api), 1s→9s (witness) — normal variance. Queue depth 0 on both.

---

## Epoch State

### morning-api (~02:54Z re-query, after boundary race resolved)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 972 | Cycling. +16 since pass 32 (956→972). | **PASS — three-way match.** Socket, grep count, and last log line all at 972. |
| ratio | 1.01963 | ~1.01–1.02 steady state (pass 32: 1.01962) | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (~02:54Z):**
- Socket epoch: **972**
- `grep -c` count: **972**
- Last log line epoch: **972**
- **PASS.** All three agree.

**Note on first read:** Initial socket read returned 970 while grep count showed 971 (race at epoch boundary between 970→971). Re-query 30s later at 972 resolved the race. Standard boundary condition observed in prior passes.

### local-witness (~02:54Z bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 972 | Cycling. +17 since pass 32 (955→972). | **PASS — three-way match.** Socket, grep count, and last log line all at 972. |
| ratio | 1.10923 | Continuing asymptotic decline (pass 32: 1.11091; Δ = -0.00168) | None (monotonic decay expected) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (~02:54Z):**
- Socket epoch: **972**
- `grep -c` count: **972**
- Last log line epoch: **972**
- **PASS.** All three agree.

**Notable: First time both nodes at identical epoch (972).** Witness caught up from its chronic 1-epoch lag. Pass 32 had api=956, witness=955. Now both at 972. This is a new observation — not a deviation, just a timing convergence.

### Epoch cadence — reconfirmed

**OBSERVED:** 16 epochs elapsed on api in ~480s (02:46:11Z→02:54:11Z). 480s / 16 epochs = **30s/epoch**. Reconfirmed.
**OBSERVED:** 17 epochs on witness in same window (caught up from lag). Consistent with 30s/epoch.

**Classification:** Stable. 30s/epoch confirmed since pass 31.

---

## Economic State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged since pass 32) |
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
**DEVIATION:** Witness-side accounting reports 0. Known-deviating since first observer pass. Causal claim: not an observation — belongs to Verifier.

**Witness log confirms ongoing redistribution rejection:** 118 `insufficient balance` WARNs in witness log, morning-api (12D3KooWPfrZ…) sending redistribution transactions at amounts from 1 to 95 DUU, all rejected because witness sees api with 0 balance. Changed from pass 32? No — still 118 (unchanged; nonce on api frozen at 120, no new transactions flowing).

---

## Persistence State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 970 | 970 | None — snapshot at 10-epoch interval (pass 32: 950, now: 970; two rotations). |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (~02:54Z):**
- `wal.log`: 379 bytes (last modified Jul 27 22:52 — epoch 970 snapshot)
- `wal.wal.old`: 379 bytes (previous WAL, renamed at epoch 960 snapshot)
- `state.snapshot`: 894 bytes (last snapshot at epoch 970)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 970 | 970 | None — matches log. Same 10-epoch interval. |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~02:54Z):**
- `wal.log`: 379 bytes (last modified Jul 27 22:53)
- `wal.wal.old`: 379 bytes (previous WAL)
- `state.snapshot`: 569 bytes (last snapshot at epoch 970)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

---

## Metrics (from heartbeat logs)

### morning-api — latest metrics tick (02:54:06Z)

| Metric | Value | Expected | Deviation |
|--------|-------|----------|-----------|
| outstanding_fetches | 0 | 0 | None |
| aged (fetches >10×timeout) | 0 | 0 | None |
| outbound_queues | [] | (empty) | None |
| max_peer_silence | 3s | <10s | None |

### local-witness
Metrics not available from UDS. Log-based: heartbeats flowing normally. 118 insufficient-balance WARNs (known persistent deviation, not a runtime error). Zero zombie/evict/reconnect activity.

---

## Error Health Scan

### morning-api
**OBSERVED:** WARN/ERROR lines after filtering: **zero**.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

### local-witness
**OBSERVED:** WARN/ERROR lines after filtering: **zero**.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — clean.

**Filter applied:** `grep -vE 'skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|Kademlia|kad|Failed to gossip genesis|Failed to publish block|Connection from'` returns **zero lines on both nodes**. Clean.

---

## Delta Summary (Pass 32 → Pass 33)

| Metric | Pass 32 (02:46Z) | Pass 33 (02:54Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch | 956 | 972 | +16 (~8 min) | Normal cycling (30s/epoch) |
| witness epoch | 955 | 972 | +17 | Caught up from 1-epoch lag |
| Three-way match | PASS both (perfect) | PASS both (perfect) | — | Stable |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch | 950 | 970 | +20 (two rotations) | Normal 10-epoch interval |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 9 commits behind HEAD |
| Kademlia bootstrap WARNs | Present (harmless) | Present (harmless) | — | Unchanged |
| Zombie/reconnect events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | 30s (confirmed) | 30s (re-confirmed) | — | Stable |
| max_peer_silence | 3s | 3s | 0 | Stable |

### New observations this pass

1. **Witness epoch converged to match api.** First time in the evidence record both nodes report the same epoch (972). The chronic 1-epoch gap that persisted through all 32 prior passes closed. Not a deviation — just convergence in the capture timing.

2. **Both nodes at last_snapshot_epoch=970.** Two snapshot rotations since pass 32 (950→960→970). WAL files at 379 bytes (genesis re-seed only). No WAL growth between snapshots.

3. **Perfect three-way match on both nodes.** Socket, grep count, and last log line all agree at 972. No race at capture boundary on final read (first read had a 970/971 race that resolved itself).

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

- Previous: `docs/evidence/observer-2026-07-28-pass32.md`
- This record: `docs/evidence/observer-2026-07-28-pass33.md`
