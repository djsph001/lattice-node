# Observer Evidence Record — 2026-07-28 (Pass 34)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T03:07:13Z (single-capture bundle: 03:04:30–03:07:13Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Thirty-fourth observation pass. Same processes since 2026-07-27T18:48Z (~8.3h runtime). ~13 min since pass 33 (02:54:13Z).

**Summary:** All-clear continuation. Epochs 998/998 (+26/+26 since pass 33). **Both nodes at same epoch, three-way match PASS on both.** Balance locked at 20 (morning-api) / 0 (witness). Snapshots at 990 (+20 since pass 33: 970→990, two rotations). Zero queues, zero fetches, zero sweep/evict/zombie activity. Build unchanged. All 3 persistent deviations unchanged. Witness epoch convergence confirmed (no chronic lag).

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
| uptime_secs | 29866 (~8.3h) | — | None (pass 33: 29137; Δ = +729s ≈ 12 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind. Docs-only + test fixes since binary build. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 992.04 | ~1000, slowly decaying | None (pass 33: 992.23; Δ = -0.19 over ~13 min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 29859 (~8.3h) | — | None (matches api within 7s — close enough for separate socket queries) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=2985, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=2989, silence_secs=3, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 33:** Heartbeats api +79 (2906→2985), witness +80 (2909→2989). Silence: 0s→3s (api), 9s→3s (witness) — normal variance. Queue depth 0 on both.

---

## Epoch State

### morning-api (03:07Z three-way bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 998 | Cycling. +26 since pass 33 (972→998). | **PASS — three-way match.** |
| ratio | 1.01964 | ~1.01–1.02 steady state (pass 33: 1.01963) | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (03:07Z):**
- Socket epoch: **998**
- `grep -c` count: **998**
- Last log line epoch: **998**
- **PASS.** All three agree.

### local-witness (03:07Z bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 998 | Cycling. +26 since pass 33 (972→998). | **PASS — three-way match.** Socket, grep count, and last log line all at 998. |
| ratio | 1.10583 | Continuing asymptotic decline (pass 33: 1.10923; Δ = -0.00340) | None (monotonic decay expected) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (03:07Z):**
- Socket epoch: **998**
- `grep -c` count: **998**
- Last log line epoch: **998**
- **PASS.** All three agree.

### Epoch cadence — reconfirmed

**OBSERVED:** 26 epochs elapsed on both nodes in ~780s (02:54:13Z→03:07:13Z). 780s / 26 epochs = **30s/epoch**. Reconfirmed.

**Classification:** Stable. 30s/epoch confirmed since pass 31.

---

## Economic State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged since pass 33) |
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
| last_snapshot_epoch | 990 | 990 | None — snapshot at 10-epoch interval (pass 33: 970, now: 990; two rotations). |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (03:07Z):**
- `wal.log`: 379 bytes (last modified Jul 27 23:02 — epoch 990 snapshot?)
- `wal.wal.old`: 379 bytes (previous WAL, renamed at epoch 980 snapshot)
- `state.snapshot`: 894 bytes (last snapshot at epoch 990)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 990 | 990 | None — matches log. Same 10-epoch interval. |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (03:07Z):**
- `wal.log`: 379 bytes (last modified Jul 27 23:03)
- `wal.wal.old`: 379 bytes (previous WAL)
- `state.snapshot`: 569 bytes (last snapshot at epoch 990)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

---

## Metrics (from heartbeat logs)

### morning-api — latest metrics tick (~03:06:56Z)

| Metric | Value | Expected | Deviation |
|--------|-------|----------|-----------|
| outstanding_fetches | 0 | 0 | None |
| aged (fetches >10×timeout) | 0 | 0 | None |
| outbound_queues | [] | (empty) | None |
| max_peer_silence | 3s | <10s | None |

### local-witness
Metrics not available from UDS. Log-based: heartbeats flowing normally. 118+ insufficient-balance WARNs (known persistent deviation, not a runtime error). Zero zombie/evict/reconnect activity.

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

**Filter applied:** `grep -vE 'skip-ntp-check|non-mDNS|No snapshot|zombie|insufficient balance|Kademlia|Connection from|Failed to gossip genesis|Failed to publish block'` returns only Kademlia bootstrap WARNs on morning-api, zero on witness. Clean.

---

## Delta Summary (Pass 33 → Pass 34)

| Metric | Pass 33 (02:54Z) | Pass 34 (03:07Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch | 972 | 998 | +26 (~13 min) | Normal cycling (30s/epoch) |
| witness epoch | 972 | 998 | +26 | Both caught up, matched |
| Three-way match | PASS both | PASS both | — | Stable |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch | 970 | 990 | +20 (two rotations) | Normal 10-epoch interval |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| wal.log on disk | 379 bytes | 379 bytes | 0 | Unchanged since epoch 970 snapshot |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 9 commits behind HEAD |
| Kademlia bootstrap WARNs | Present (harmless) | Present (harmless) | — | Unchanged |
| Zombie/reconnect events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | 30s (confirmed) | 30s (re-confirmed) | — | Stable |
| max_peer_silence | 3s | 3s | 0 | Stable |

### New observations this pass

1. **Both nodes at identical epoch 998, three-way match on both.** Witness fully converged — no chronic lag. Confirmed via simultaneous bundle: socket (998) = grep -c (998) = last log line (998) on both nodes.

2. **Witness grep -c now matches api.** Pass 33 showed witness at 997 while api at 998 during the log query (timestamp skew). This pass confirms both at 998 simultaneously.

3. **28 epochs in ~13 min = 30s/epoch reconfirmed** with a larger sample (26 instead of 16).

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

- Previous: `docs/evidence/observer-2026-07-28-pass33.md`
- This record: `docs/evidence/observer-2026-07-28-pass34.md`
