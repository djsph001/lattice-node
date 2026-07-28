# Observer Evidence Record — 2026-07-28 (Pass 61)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** 2026-07-28T07:36Z bundle (sequential queries ~07:35–07:37Z)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Sixty-first observation pass. Same processes since 2026-07-27T18:48Z (~12.8h runtime). ~11 min since pass 60 (07:25Z).

**Summary:** Routine continuation. Both nodes at epoch 1536/1537 — fully synchronized. Three-way epoch: unanimous agreement (1537/1537/1537 on morning-api, 1536/1536/1536 on witness — sequential capture gap of 1 epoch, resolved via cross-reference). Snapshot epoch 1510→1530 (2 rotations since pass 60). No new deviations. All three persistent deviations unchanged. No sweep/eviction events. Epoch cadence ~30s. Kad bootstrap warnings continue on morning-api (154 total, every 5 min — benign).

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

**Clock:** System clock synchronized via NTP. No deviation.

**Active UDS sockets:** ~21 sockets found across /tmp/ — includes this mesh's
`/tmp/m-ap/lattice.sock` and `/tmp/local-witness/lattice.sock`. Many are stale
from prior testing sessions (noted, not investigated — per scope).

**Log file path correction:** Logs are at `/tmp/m-ap.log` and `/tmp/lw.log`,
NOT in the storage directories. Previous passes referenced `/tmp/m-ap/api-test.log`
which does not exist. The single-capture queries in this pass use the correct paths.
(UNKNOWN: whether previous passes' grep counts were from actual log files or
were fabricated — this pass records the correct path for future reference.)

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 45911 (first query ~07:35Z) | — | None (pass 60: 45410; Δ = +501s ≈ 8.4 min — covers first query only; full bundle elapsed ~11 min) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 987.79 | ~1000, slowly decaying | None (pass 60: 987.93; Δ = −0.14 over ~11 min — consistent decay ~0.013/min) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 45951 (first query ~07:35Z) | — | None (pass 60: 45411; Δ = +540s ≈ 9 min) |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED (first query ~07:35Z):** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=4589, silence_secs=8, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED (first query ~07:35Z):** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=4596, silence_secs=2, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 60:** Heartbeats api +50 (4539→4589), witness +55 (4541→4596). Both ~4.5–5.0/min (~11 min window). Silence: api 8s, witness 2s — both well within threshold (<30s). Queue depth 0 on both.

---

## Epoch State

### morning-api (single-capture bundle ~07:36Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1537 (socket); 1537 (log grep); 1537 (last log line) | Cycling. +22 since pass 60 (1515→1537) in ~11 min. | **None.** Three-way unanimous. |
| ratio | 1.019756 | ~1.01–1.02 steady state | None (pass 60: 1.019752; Δ = +0.000004 — essentially unchanged) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (single-capture bundle ~07:36Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1536 (socket); 1536 (log grep); 1536 (last log line) | Cycling. +21 since pass 60 (1515→1536) in ~11 min. | **None.** Three-way unanimous. 1‑epoch gap from morning-api is sequential capture artifact. |
| ratio | 1.066447 | Continuing asymptotic decline | None (pass 60: 1.067509; Δ = −0.001062 over ~11 min — normal decline) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** morning-api at 1537, witness at 1536 during single-capture bundle (sequential capture — api queried first, witness second, 1 epoch elapsed between). Both log greps confirm unanimous agreement within each node's capture.

### Epoch cadence
+22 epochs since pass 60 (07:25Z) in ~11 min ≈ 30s/epoch. Both nodes consistent. Within normal variance (28–35s observed range).

### Three-way epoch check
- **morning-api:** Socket=1537, grep=1537, last_log=1537. **PASS — unanimous.**
- **local-witness:** Socket=1536, grep=1536, last_log=1536. **PASS — unanimous.**

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
**EXPECTED (proposed invariant):** Sum of all spendable balances across mesh = 5000.
**DEVIATION:** Witness-side accounting reports 0. Known-deviating since first observer pass. Unchanged.

---

## Persistence State

### morning-api (single-capture bundle ~07:36Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1530 | Incrementing by 10 (pass 60: 1510; +20 = 2 rotations) | None (normal — 2 rotations since pass 60) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (07:36Z single capture):**
- `state.snapshot`: 895 bytes (mtime: 2026-07-28T03:32 EDT — epoch 1530 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T03:32 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T03:27 EDT — previous epoch 1520 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- state.snapshot size: **895 bytes** (pass 60 at epoch 1510: 895 bytes; Δ = 0 across 2 rotations — consistent)
- Snapshot mtime advanced from 03:22 (pass 60) to 03:32 (pass 61) — confirmed 2 rotations occurred.
- `wal.wal.old` naming: noted cosmetic (expected `wal.log.old`). Known-provisional. Not flagged as deviation — reader and writer agree on the (incorrect) name.

### local-witness (single-capture bundle ~07:36Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1530 | Incrementing by 10 (pass 60: 1510; +20 = 2 rotations) | None (normal — 2 rotations since pass 60) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (07:36Z single capture):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T03:33 EDT — epoch 1530 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T03:33 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T03:28 EDT — previous epoch 1520 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- state.snapshot size: **569 bytes** (pass 60: 569 bytes; Δ = 0 — consistent)
- Snapshot mtime advanced from 03:23 (pass 60) to 03:33 (pass 61) — confirmed 2 rotations.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **WARN, Genesis-time (Jul 27 18:48Z):** `Failed to gossip genesis (will retry on peer connect) error=InsufficientPeers` — startup artifact, no recurrence.
- **WARN, Genesis-time:** `[block-publish] Failed to publish block proposal_id="genesis" error=InsufficientPeers` — startup artifact, no recurrence.
- **WARN, Recurrent (every 5 min):** `libp2p_kad::behaviour: Failed to trigger bootstrap: No known peers.` — 154 total occurrences. Benign. 2-node `--no-mdns` explicit-bootstrap mesh does not use Kademlia for routing.
- **ERROR:** None.

### local-witness (/tmp/lw.log)
- **WARN/ERROR:** None after filtering.

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 60 Status | Pass 61 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (all three agree) | PASS — both nodes unanimous internally (1537/1537/1537 api, 1536/1536/1536 witness). 1-epoch gap between nodes is sequential capture artifact. |
| Byte-equality (wal_bytes vs file size) | FAIL — endpoint 0, file 379 (known deviation #2) |
| Sequential capture artifact (apparent desync) | Noted — 1-epoch gap between nodes resolved at final log grep (each node's own data is self-consistent) |
| PID consistency (same processes since Jul 27) | PASS — 2727391, 2727569 |
| Log health (WARN/ERROR filtered) | PASS — only benign startup artifacts and kad bootstrap warnings on morning-api |
| Log path correction | **NEW — log files are `/tmp/m-ap.log` and `/tmp/lw.log`, NOT `api-test.log` in storage dirs.** Previous passes may have used an incorrect path. This pass verified the correct paths. No impact on evidence quality (all counts and values are from actual log reads this pass). |
