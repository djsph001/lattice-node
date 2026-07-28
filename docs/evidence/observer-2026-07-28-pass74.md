# Observer Evidence Record — 2026-07-28 (Pass 74)

**Observer:** lattice-observer (Engineering Cell, autonomous agent — cron job)
**Capture time:** ~2026-07-28T09:39Z bundle (socket queries)
**Log/metrics capture:** ~2026-07-28T09:40Z
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Seventy-fourth observation pass. Same processes since 2026-07-27T18:48Z (~14.8h runtime). ~9 min since pass 73 (09:30Z).

**Summary:** Routine continuation. Both nodes cycling normally. Epoch 1782/1783 (+14-16 since pass 73). Two snapshot rotations occurred (1760→1770→1780). All three persistent deviations unchanged. **One new observation:** first-ever RUNTIME NTP check failure at 09:40:16Z (DNS resolution failed for all 3 NTP servers). Still no zombie evictions, no errors, no panics.

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

---

## Node Info

### morning-api (~09:39Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 53429 (~14.8h) | — | None (pass 73: 53026; Δ = +403s ≈ 6.7 min — bundle timing variation) |
| build_commit | `71aa16b-dirty` | git HEAD `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind HEAD + dirty tree. First observed: observer pass 1 (Jul 27). Unchanged. |
| thickness | 985.813 | ~1000, slowly decaying | None (pass 73: 985.92; Δ = −0.107 over ~9 min — consistent decay ~0.012/min) |

### local-witness (~09:40Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 53463 (~14.8h) | — | None (pass 73: 52993; Δ = +470s ≈ 7.8 min — bundle timing variation) |
| build_commit | `71aa16b-dirty` | Same binary | **Persistent DEVIATION** (same as morning-api). |

---

## Peer Connections

### morning-api (~09:39Z)
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=5342, silence_secs=1, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness (~09:40Z)
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=5347, silence_secs=9, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 73 (~09:30Z):** Heartbeats api +41 (5301→5342 ~4.6/min), witness +47 (5300→5347 ~5.2/min). Both at expected rate. Silence: api 1s (pass 73: 5s), witness 9s (pass 73: 6s — normal variation). Queue depth 0 on both.

**No zombie eviction events** detected in either log.

---

## Epoch State

### morning-api (~09:39Z socket capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1782 (endpoint) | Cycling. +14 from pass 73 (1768→1782) in ~9 min. | None. Normal cadence (~38s/epoch). |
| ratio | 1.019789 | ~1.01–1.02 steady state | None (pass 73: 1.019788; unchanged within precision) |
| tax_calculated | 0 | Balance 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

### local-witness (~09:40Z socket capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 1783 (endpoint) | Cycling. +16 from pass 73 (1767→1783) in ~10 min. | None. Normal cadence. Bundle capture sequential. |
| ratio | 1.056325 | Continuing asymptotic decline | None (pass 73: 1.057094; Δ = −0.000769 over ~10 min — continued approach to 1.0) |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

### Epoch synchronization
**OBSERVED:** Bundle showed m-ap at 1782 (~09:39Z), witness at 1783 (~09:40Z) (Δ=1 — sequential capture with ~1 min gap, normal).

### Three-way epoch check (single-capture caveat — log queries ran ~1 min after socket queries)
- **morning-api:** Socket=1782 (~09:39Z), grep count=1784, last_log epoch=1784 (09:39:56Z). Δ=2 between endpoint and log — 2 epochs elapsed in ~60s capture window. Normal race. NOT a deviation.
- **local-witness:** Socket=1783 (~09:40Z), grep count=1784, last_log epoch=1784 (09:40:13Z). Δ=1 — 1 epoch elapsed in capture window. Normal race.

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
| own_nonce | 2 | 2 | None (unchanged since pass 64) |
| morning_api_balance (reported) | 0 | 5000 | **Persistent DEVIATION.** Witness reports morning-api balance as 0. First observed: observer pass 1 (Jul 27 18:48Z). Supply conservation: CONTRADICTED per Verifier Mission 1. |

### Supply divergence
**OBSERVED:** morning-api sees total supply = 20 + 4980 = 5000. Witness sees total supply = 0 + 0 = 0.
**DEVIATION:** Witness-side accounting reports 0. Known since first observer pass. Unchanged.

---

## Persistence State

### morning-api (~09:39Z socket + filesystem at ~09:40Z, sequential capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1780 | Incrementing by 10 (pass 73: 1760; +20 = 2 rotations) | None (normal — 2 rotations since pass 73) |
| wal_bytes | 0 | File size of current WAL (379 bytes) | **Persistent DEVIATION.** `wal.log` on disk is 379 bytes. Endpoint reads `transactions.wal` (legacy path). First observed: pass 1 (Jul 27). |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** (same root cause as wal_bytes) |

**File system cross-check (~09:40Z):**
- `state.snapshot`: **894 bytes** (mtime: 2026-07-28T05:37 EDT — epoch 1780 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T05:37 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T05:32 EDT — previous epoch 1770 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 05:32 (pass 73) to 05:37 — confirmed 2 rotations occurred (1760→1770→1780). ✓
- Snapshot size **894 bytes** (pass 73: 895; Δ = −1 byte). **Minor observation:** size regressed from 895 to 894. Pass 72 report noted stabilization at 895 after 894→895 in pass 71/pass 72. Now back to 894. UNKNOWN: cause of ±1 byte fluctuation. Not classified as a deviation — likely content-dependent (e.g., serialization of a threshold value at boundary).

### local-witness (~09:40Z socket + filesystem, sequential capture)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | 1780 | Incrementing by 10 (pass 73: 1760; +20 = 2 rotations) | None (normal — 2 rotations since pass 73) |
| wal_bytes | 0 | 379 | **Persistent DEVIATION** (same bug) |
| wal_entries | 0 | Non-zero | **Persistent DEVIATION** |

**File system cross-check (~09:40Z):**
- `state.snapshot`: 569 bytes (mtime: 2026-07-28T05:38 EDT — epoch 1780 snapshot)
- `wal.log`: 379 bytes (mtime: 2026-07-28T05:38 EDT)
- `wal.wal.old`: 379 bytes (mtime: 2026-07-28T05:33 EDT — previous epoch 1770 snapshot)
- `wal_bytes` endpoint: 0 → **MISMATCH.** Expected 379.
- Snapshot mtime advanced from 05:28 (pass 73) to 05:38 — confirmed 2 rotations. ✓
- Snapshot size 569 bytes (pass 73: 569; unchanged) ✓

---

## Metrics Instrumentation

**OBSERVED (from m-ap metrics line at ~09:40Z):**
- `outstanding_fetches=0` — no pending fetches
- `aged=0` — no stale fetches
- `outbound_queues=[]` — all peer queues empty
- `max_peer_silence=3s` — well under 30s threshold

**OBSERVED (from witness metrics line at ~09:40Z):**
- `outstanding_fetches=0`
- `aged=0`
- `outbound_queues=[]`
- `max_peer_silence=6s`

**EXPECTED:** All gauges near zero on a settled 2-node mesh with no new transactions.
**DEVIATION:** None. Mesh is quiescent.

---

## Log Health

### morning-api (/tmp/m-ap.log)
- **WARN (190 total, +13 since pass 73):**
  - KAD bootstrap warnings: continuous every 5 min. +2 since pass 73 (09:33, 09:38). Benign.
  - **NEW at 09:40:16Z: `RUNTIME NTP: check failed — skipping this cycle error=Could not reach any NTP server (tried: pool.ntp.org, time.apple.com, time.google.com).`** Preceded by DNS resolution failures for all 3 servers at 09:40:16Z. This is a NEW event type — previously NTP failures were only at startup (`NTP query to pool.ntp.org failed`). The runtime NTP check is a different codepath. **OBSERVED: first occurrence.** UNKNOWN: transient DNS issue or persistent network problem. This is a new observation but NOT classified as a deviation (no expected behavior documented for runtime NTP check cadence).
  - +3 DNS lookup failures (one per NTP server) at 09:40:16Z
  - +2 existing WARN patterns unchanged (historic startup artifacts)
- **ERROR:** None.
- **Zombie eviction events:** None.
- **Sweep/eviction events:** None.
- **Panics:** None.

### local-witness (/tmp/lw.log)
- **WARN (123 total, +5 since pass 73):** All 5 new WARNs are historic startup artifacts (3× "No snapshot", 1× "non-mDNS peer", 1× NTP startup failure). No new WARNs since pass 73.
- **ERROR:** None.
- **Insufficient balance:** 118 (unchanged, all historic Jul 27). No new occurrences.

### Log filter (WARN/ERROR excluded as benign)
| Pattern | m-ap | lw | Status |
|---------|------|----|--------|
| `skip-ntp-check` | 0 | 0 | Clean |
| `No snapshot` | 0 | 3 (startup) | Clean |
| `zombie` | 0 | 0 | Clean |
| `insufficient balance` | 0 | 118 (historic) | No new occurrences |
| `panicked` | 0 | 0 | Clean |

---

## Build Commit Verification

| Check | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| git HEAD | `cb5d4b1` | — | — |
| running binary | `71aa16b-dirty` | `cb5d4b1` | **Persistent DEVIATION.** 9 commits behind + dirty (64 unstaged files). Unchanged since pass 1. |

---

## New Observation: Runtime NTP Check Failure

| Field | Detail |
|-------|--------|
| **OBSERVED** | At 2026-07-28T09:40:16Z, morning-api logged: `RUNTIME NTP: check failed — skipping this cycle error=Could not reach any NTP server` |
| **Context** | 3 preceding DNS failures at same timestamp (pool.ntp.org, time.apple.com, time.google.com — all `Temporary failure in name resolution`) |
| **EXPECTED** | Not documented. No runtime NTP check cadence is specified in VERIFIED-BEHAVIOR.md or MESH.md. Startup NTP failures have been observed before (pass 73). |
| **FIRST OBSERVED** | This exact pass (74), 2026-07-28T09:40:16Z |
| **CLASSIFICATION** | Observation only. Not a deviation — no expected behavior defined. UNKNOWN: whether this is transient DNS or persistent network degradation. |
| **NOTE** | The `--skip-ntp-check` flag was used to start both nodes. Runtime NTP checks are a separate mechanism from startup checks. |

---

## Persistent Deviations Summary

| # | Deviation | First Observed | Pass 73 Status | Pass 74 Status | Changed? |
|---|-----------|----------------|----------------|----------------|----------|
| 1 | `build_commit` is `71aa16b-dirty` (9 commits behind HEAD `cb5d4b1`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 2 | `wal_bytes` returns 0 (reads legacy `transactions.wal` instead of `wal.log`) | Pass 1 (Jul 27) | Persistent | Persistent | No |
| 3 | Witness reports morning-api balance as 0 (supply divergence) | Pass 1 (Jul 27 18:48Z) | Persistent | Persistent | No |

**No new deviations detected in this pass.** One new observation (runtime NTP check failure) recorded.

---

## Minor Observations (Not Deviations)

| Observation | First Noted | Status |
|-------------|------------|--------|
| morning-api snapshot size regressed from 895→894 bytes (was 894 in pass 71, 895 in passes 72-73) | Pass 72 | Fluctuating. UNKNOWN: cause of ±1 byte oscillation. Possibly serialization of a threshold/boundary value. |
| First-ever runtime NTP check failure at 09:40:16Z | This pass | Single occurrence. UNKNOWN: transient or persistent. |

---

## Evidence Integrity

| Guard | Status |
|-------|--------|
| Three-way epoch check (all three agree) | **PASS WITH CAVEAT** — socket queries and log queries ran ~60s apart. Socket 1782/1783 vs grep 1784. Δ=1-2, consistent with 1-2 epochs elapsed during capture window. Normal race, not a deviation. |
| Byte-equality (wal_bytes vs file size) | **FAIL** — endpoint 0, file 379 (known deviation #2) |
| PID consistency (same processes since Jul 27) | **PASS** — 2727391, 2727569 |
| Log health (WARN/ERROR filtered) | **PASS WITH NOTE** — no new anomalous events beyond continuous KAD bootstrap warnings (190 count, +13), 118 historic insufficient-balance, and one new runtime NTP check failure (observation, not deviation). No errors, no panics, no zombies. |
| Metrics health (aged=0, queues=[], silence<30s) | **PASS** |
| Cross-node epoch sync | **PASS WITH CAVEAT** — m-ap at 1782, witness at 1783 (Δ=1, sequential capture timing) |
