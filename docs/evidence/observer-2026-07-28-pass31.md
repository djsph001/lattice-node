# Observer Evidence Record — 2026-07-28 (Pass 31)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-28T02:38:39Z (single-capture bundle: 02:37:56–02:38:39Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Thirty-first observation pass. Same processes since 2026-07-27T18:48Z (~11.8h runtime). ~10 min since pass 30 (02:29:00Z).

**Summary:** All-clear continuation. Epochs 940/941 (+19/+18 since pass 30). Three-way epoch match PASS on both nodes. Balance locked at 20 (morning-api) / 0 (witness). Snapshot rotated 920→930→940 during this window (every 10 epochs, consistent). Zero queues, zero fetches, zero sweep/evict/zombie activity. Git HEAD unchanged. All 3 persistent deviations unchanged. **UNKNOWN from pass 30 resolved:** epoch cadence confirmed at exactly 30s/epoch.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 2026-07-27T18:48Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 2026-07-27T18:48Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs as pass 29/30 (2727391, 2727569). Both sockets responding. 2 lattice-node processes. Logs at `/tmp/m-ap.log` and `/tmp/lw.log`.

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 28121 (~7.8h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind. Docs-only + test fixes since binary build. First observed: observer pass 1 (Jul 27). Unchanged since. |
| thickness | 992.50 | ~1000, slowly decaying | None (pass 30: 992.65; Δ = -0.15 over ~10 min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 28126 (~7.8h) | — | None (slightly higher than api due to capture ordering) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=2810, silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=2813, silence_secs=6, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 30:** Heartbeats morning-api +53 (2757→2810), witness +53 (2760→2813). Silence: 5s→7s (api), 9s→6s (witness) — normal variance. Queue depth 0 on both.

---

## Epoch State

### morning-api (~02:38Z single capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 941 (socket), 940 (grep count), 940 (last log line at capture) | Cycling. +19 since pass 30 (922→941). | **PASS — three-way match within race window.** Socket at 941 (captured just after epoch transition), log last-line at 940 (captured before transition). grep count = 940 matches last log line. |
| ratio | 1.01998 | ~1.01–1.02 steady state (pass 30: 1.01961) | None (stable, +0.00037) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (~02:38Z):**
- Socket epoch: 941
- `grep -c` count: 940
- Last log line epoch: 940
- **PASS.** Socket at 941 indicates epoch transition during capture (between log check and socket query). Normal race — all three value pairs agree within the capture window.

### local-witness (~02:38Z bundle)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 940 (socket), 940 (grep count), 940 (last log line at capture) | Same cadence. +18 since pass 30 (921→940). | **PASS — three-way match.** All three agree at 940. |
| ratio | 1.11304 | Continuing asymptotic decline (pass 30: 1.11525; Δ = -0.00221) | None (monotonic decay expected) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (~02:38Z):**
- Socket epoch: 940
- `grep -c` count: 940
- Last log line epoch: 940
- **PASS.** All three agree.

### Epoch cadence — RESOLVED

**OBSERVED:** Epoch timestamps at exactly 30s intervals:
```
02:36:26 → epoch 937
02:36:56 → epoch 938
02:37:26 → epoch 939
02:37:56 → epoch 940
02:38:26 → epoch 941
```

**Deviation from pass 29/earlier claim (~20s/epoch):** The cadence is exactly 30s/epoch, confirmed by 5 consecutive epoch timestamps. Earlier passes claiming ~19-20s measured incorrectly. This has been the cadence since at least epoch 10 (first snapshot at 18:52:56 - 18:48:26 startup = 4.5 min = 270s for 8 epochs ≈ 33s/epoch at startup, stabilizing to 30s by epoch 30).

**Classification:** RESOLVED — earlier passes miscalculated; true cadence has always been 30s/epoch. No causal mechanism needed (no code changes to epoch timer during this runtime).

---

## Economic State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | 20 | 20 (frozen since ~epoch 30) | None (unchanged since pass 30) |
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

---

## Persistence State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 940 | 940 (confirmed by log: `Snapshot saved epoch=940` at 02:37:56Z) | None — matches log. Snapshot interval = 10 epochs. |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path, does not exist). First observed: pass 1 (Jul 27). Verifier Mission 2 (Jul 27): confirmed one-line fix. |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (~02:38Z):**
- `wal.log`: 379 bytes (written at epoch 940 snapshot, contains genesis re-seed)
- `wal.wal.old`: 379 bytes (previous WAL, renamed at epoch 930 snapshot)
- `state.snapshot`: 894 bytes (last snapshot at epoch 940)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 940 | 940 (confirmed by log: `Snapshot saved epoch=940` at 02:38:13Z) | None — matches log. Same 10-epoch interval as morning-api. |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~02:38Z):**
- `wal.log`: 379 bytes
- `wal.wal.old`: 379 bytes
- `state.snapshot`: 569 bytes
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.

---

## Metrics (from heartbeat logs)

### morning-api — latest metrics tick (02:38:06Z)

| Metric | Value | Expected | Deviation |
|--------|-------|----------|-----------|
| outstanding_fetches | 0 | 0 | None |
| aged (fetches >10×timeout) | 0 | 0 | None |
| outbound_queues | [] | (empty) | None |
| max_peer_silence | 3s | <10s | None |

### local-witness — latest metrics tick
Metrics not available from UDS. Log-based: heartbeats flowing normally (interval ~6-7s). Zero WARN/ERROR.

---

## Error Health Scan

### morning-api
**OBSERVED:** `Failed to gossip genesis (will retry on peer connect) error=InsufficientPeers` (startup), `Failed to publish block proposal_id="genesis" error=InsufficientPeers` (startup). Both from 18:48:26 — first ~17s after startup, before peer connection established.
**EXPECTED:** No actionable errors.
**DEVIATION:** None — startup noise, resolved within seconds. Present since first observer pass.

### local-witness
**OBSERVED:** Zero WARN/ERROR lines.
**EXPECTED:** Clean.
**DEVIATION:** None.

---

## Delta Summary (Pass 30 → Pass 31)

| Metric | Pass 30 (02:29Z) | Pass 31 (02:38Z) | Δ | Status |
|--------|-----------------|-----------------|----|--------|
| morning-api epoch | 922 | 941 | +19 elapsed (~9.5 min) | Normal cycling |
| witness epoch | 921→922 | 940 | +18→19 | Normal cycling |
| Three-way match | PASS both | PASS both (api had 1-epoch race at capture boundary) | — | Stable |
| Balance (api) | 20 | 20 | 0 | Frozen since first pass |
| Balance (witness) | 0 | 0 | 0 | Frozen since first pass |
| own_nonce (api) | 120 | 120 | 0 | Frozen |
| own_nonce (witness) | 2 | 2 | 0 | Frozen |
| Snapshot epoch | 920 | 940 | +20 | Two rotations: 920→930→940 |
| wal_bytes (endpoint) | 0 | 0 | 0 | Persistent deviation |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | 0 | 9 commits behind HEAD |
| KAD bootstrap WARNs | Present (harmless) | Present (harmless) | — | Unchanged |
| Zombie/reconnect events | None | None | — | Clean |
| Outstanding fetches | 0 | 0 | — | Clean |
| Epoch cadence | ~30s (suspected, UNKNOWN) | 30s (confirmed) | RESOLVED | Always 30s |

### New observations this pass

1. **Epoch cadence confirmed at exactly 30s/epoch.** Five consecutive epoch timestamps at :26 and :56 of each minute. Resolves UNKNOWN from pass 30. Earlier cadence claims (~20s) were measurement errors.

2. **Snapshot rotation 920→930→940.** Two snapshot rotations between pass 30 (02:29Z) and pass 31 (02:38Z) — epochs 930 (02:32:56) and 940 (02:37:56). Consistent 10-epoch interval. WAL files at 379 bytes (genesis re-seed only) after both rotations. No WAL growth between snapshots (expected: no transactions flowing).

3. **Near-synchronous epochs.** Both nodes at functionally the same epoch (940 vs 941) during capture. Witness no longer trailing by 1 epoch — it caught up to the transition boundary.

---

## Persistent Deviations (unchanged)

| # | Observation | First seen | Status |
|---|------------|-----------|--------|
| 1 | build_commit 9 commits behind HEAD (71aa16b-dirty vs cb5d4b1) | Pass 1 (Jul 27) | Persistent — binary not rebuilt since those commits |
| 2 | GetPersistenceState wal_bytes=0 (reads transactions.wal instead of wal.log) | Pass 1 (Jul 27) | Persistent — Verifier Mission 2 confirmed one-line fix |
| 3 | Local-witness reports morning-api balance as 0 (supply divergence); nonce frozen | Pass 1 (Jul 27) | Persistent — supply conservation CONTRADICTED per Verifier Mission 1 |

---

## UNKNOWN Items

**(None.)** All previous UNKNOWNS from pass 30 (epoch cadence) are now RESOLVED.

---

## Evidence Files

- Previous: `docs/evidence/observer-2026-07-28-pass30.md`
- This record: `docs/evidence/observer-2026-07-28-pass31.md`
