# Observer Evidence Record — 2026-07-28 (Pass 30)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T02:29:00Z (single-capture bundle)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Thirtieth observation pass. Same processes since 2026-07-27T18:48Z (~11.7h runtime). ~2h36m since pass 29 (23:53:05Z).

**Summary:** All-clear continuation. Epochs 608→922 (+314 morning-api, +313+1 witness). Three-way epoch match PASS on both nodes at capture time. Balance locked at 20 (morning-api) / 0 (witness). First snapshot of this session at epoch 920 on both nodes — snapshot epoch synchronized. Zero queues, zero fetches, zero sweep/evict/zombie activity. Git HEAD unchanged. All three persistent deviations unchanged. New observation: epoch cadence appears slower (~30s/epoch) than earlier passes claimed (~20s/epoch).

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 2026-07-27T18:48Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 2026-07-27T18:48Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs as pass 29 (2727391, 2727569). Both sockets responding. 2 lattice-node processes.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 27586 (~7.7h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind. Docs-only + test fixes since binary build. First observed: observer pass 1 (Jul 27). Unchanged since. |
| thickness | 992.65 | ~1000, slowly decaying | None (pass 29: 995.13; Δ = -2.48 over ~2.6h) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 27597 (~7.7h) | — | None (slightly higher than api due to capture ordering) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=2757, silence_secs=5, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=2760, silence_secs=9, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 29:** Heartbeats morning-api +937 (1820→2757), witness +936 (1824→2760). Silence: morning-api 3s→5s (normal), witness 7s→9s (normal). Queue depth 0 on both.

---

## Epoch State

### morning-api (~02:29:00Z single capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 922 (socket), 922 (grep count), 922 (last log line) | Cycling. +314 since pass 29 (608→922). | **PASS — three-way match.** |
| ratio | 1.01961 | ~1.01–1.02 steady state (pass 29: 1.01939) | None (stable) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (~02:29:00Z):**
- Socket epoch: 922
- `grep -c` count: 922
- Last log line epoch: 922

**PASS.** All three agree.

### local-witness (~02:29:00Z bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 921 (socket), 921 (grep count), 921 (last log line at capture) | Same cadence. +313 since pass 29 (608→921). | **PASS — three-way match at capture.** Witness caught up to 922 by 02:29:13Z (normal race at epoch boundary, <10s offset). |
| ratio | 1.11525 | Continuing asymptotic decline (pass 29: 1.17808; Δ = -0.0628) | None (monotonic decay expected) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match at capture (~02:29:00Z):**
- Socket epoch: 921
- `grep -c` count: 921
- Last log line epoch: 921

**PASS.** All three agree. Witness 1 epoch behind morning-api at capture time, consistent with ~3s timing offset. Witness reached epoch 922 by 02:29:13Z.

### Epoch cadence observation
**OBSERVED:** ~29.8s per epoch (314 epochs in 9372s uptime delta since pass 29).
**Pass 29 claimed:** ~19-20s cadence.
**UNKNOWN:** Whether cadence was always ~30s and earlier passes miscalculated, or whether the epoch timer slowed. The ratio (1.01961) has been stable across all passes — it does not directly encode wall-clock cadence. Cadence can be measured precisely only by comparing two passes with known wall-clock and epoch deltas.

---

## Economic State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged since pass 29) |
| own_nonce | 120 | 120 | None (unchanged) |
| witness_balance (reported) | 4980 | 5000 - morning_api_balance = 4980 | None (mesh consensus on peer balance) |
| witness_nonce (reported) | 0 | 0 | None |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 0 | 0 | None |
| own_nonce | 2 | 2 | None |
| morning_api_balance (reported) | 0 | 5000 - witness_balance = 5000 | **Persistent DEVIATION.** Witness reports morning-api balance as 0. First observed: observer pass 1 (Jul 27 18:48Z). Supply conservation: CONTRADICTED per Verifier Mission 1. |

### Supply divergence

**OBSERVED:** morning-api sees total supply = 20 + 4980 = 5000. Witness sees total supply = 0 + 0 = 0.
**EXPECTED (proposed invariant, per VERIFIED-BEHAVIOR.md):** Sum of all spendable balances across mesh = 5000.
**DEVIATION:** Witness-side accounting reports 0. Known-deviating since first observer pass. Causal claim: not an observation — belongs to Verifier.

---

## Persistence State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 920 | Cycling (previous snapshot at epoch 10) | None — snapshot at epoch 920 is consistent with periodic snapshots |
| wal_bytes | 0 | File size of current WAL | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path, does not exist). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (~02:29Z):**
- `wal.log`: 379 bytes (written at epoch 920 snapshot, contains genesis re-seed)
- `wal.wal.old`: 379 bytes (previous WAL, renamed at epoch 920 snapshot)
- `state.snapshot`: 894 bytes (last snapshot at epoch 920)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 920 | Matching morning-api | None (both snapshot at same epoch, ~1 min apart) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~02:29Z):**
- `wal.log`: 379 bytes
- `wal.wal.old`: 379 bytes
- `state.snapshot`: 569 bytes
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

---

## Metrics (from heartbeat logs)

### morning-api — latest metrics tick (02:29:16Z)
| Metric | Value | Expected | Deviation |
|--------|-------|----------|-----------|
| outstanding_fetches | 0 | 0 | None |
| aged (fetches >10×timeout) | 0 | 0 | None |
| outbound_queues | [] | (empty) | None |
| max_peer_silence | 3s | <10s | None |

### local-witness — latest metrics tick (02:29:16Z)
Metrics not available from UDS. Log-based: heartbeats flowing normally (interval ~3-9s). No WARN/ERROR.

---

## Error Health Scan

### morning-api
**OBSERVED:** `Failed to trigger bootstrap: No known peers` WARN at ~5min intervals (libp2p KAD, harmless on --no-mdns 2-node mesh).
**EXPECTED:** No actionable errors.
**DEVIATION:** None — harmless KAD bootstrap noise. Present since first observer pass.

### local-witness
**OBSERVED:** Zero WARN/ERROR lines.
**EXPECTED:** Clean.
**DEVIATION:** None.

---

## Delta Summary (Pass 29 → Pass 30)

| Metric | Pass 29 (23:53Z) | Pass 30 (02:29Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch | 608 | 922 | +314 elapsed | Normal cycling |
| witness epoch | 608 | 921→922 | +313→314 | Normal cycling (1 epoch race at capture) |
| Three-way match | PASS both | PASS both | — | Stable |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch | 10 | 920 | +910 | Snapshot rotated during this window |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 9 commits behind HEAD |
| KAD bootstrap WARNs | Present | Present | — | Harmless, unchanged |
| Zombie/reconnect events | None | None | — | Clean |

### New observations this pass

1. **First snapshot rotation of this session.** Previous snapshot was epoch 10 (captured right at genesis stabilization). New snapshot at epoch 920 on both nodes. WAL files at 379 bytes (genesis re-seed only). Verified that genesis was not lost — `wal.log` is non-empty post-rotation.

2. **Epoch cadence appears ~30s/epoch.** This may have been the case since early passes but was misreported as ~20s. No code changes were made, so the cadence has either been ~30s all along or is unrelated to any known change.

---

## Persistent Deviations (unchanged)

| # | Observation | First seen | Status |
|---|------------|-----------|--------|
| 1 | build_commit 9 commits behind HEAD (71aa16b-dirty vs cb5d4b1) | Pass 1 (Jul 27) | Persistent — binary not rebuilt since those commits |
| 2 | GetPersistenceState wal_bytes=0 (reads transactions.wal instead of wal.log) | Pass 1 (Jul 27) | Persistent — Verifier Mission 2 confirmed one-line fix |
| 3 | Local-witness reports morning-api balance as 0 (supply divergence); nonce frozen | Pass 1 (Jul 27) | Persistent — supply conservation CONTRADICTED per Verifier Mission 1 |

---

## UNKNOWN Items

- **Epoch cadence.** Pass 29 claimed ~19-20s/epoch; current measurement yields ~30s/epoch. Either a measurement error in earlier passes or a real slowing. Cannot determine from observer data alone — would need exact epoch-timestamp pairs from the log.

---

## Evidence Files

- Previous: `docs/evidence/observer-2026-07-27-pass29.md`
- This record: `docs/evidence/observer-2026-07-28-pass30.md`
