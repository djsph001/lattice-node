# Observer Evidence Record — 2026-07-27 (Pass 26)

**Observer:** lattice-observer (Engineering Cell, autonomous agent)
**Capture time:** 2026-07-27T23:17:28Z (composite bundle, spread ~+30s)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, Boynton Beach FL)
**Session type:** Twenty-sixth observation pass. Same processes since 14:48 EDT (~8.5h runtime).

**Summary:** All-clear continuation with one resolved UNKNOWN and one new observation. ~7 min since pass 25 (23:10Z). Epochs 525→540 (+15). Three-way epoch match PASS on both nodes individually (540/540/540 morning-api, 539/539/539 witness — 1-epoch phase difference). Balance locked at 20. Thickness 995.69 (continuing asymptotic decay). Zero queues, zero fetches, zero sweep/evict/zombie activity.

**Resolved UNKNOWN #2 (snapshot mtime):** Snapshot file mtime DID advance (19:07→19:17). The observation window between passes 24→25 (11 min) was too narrow to capture a rotation. Confirmed: morning-api state.snapshot mtime advanced from 19:07 to 19:17:56; witness from 19:08 to 19:18:13. Both correspond to real snapshot writes. UNKNOWN marked resolved.

**New observation — snapshot epoch divergence:** At pass 25, both nodes reported last_snapshot_epoch=520. Now morning-api=540, witness=530. A 10-epoch gap between nodes. First time observed. Not yet classified as deviation.

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since | Command |
|-----|------|------|--------------|-------|---------|
| 2727391 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 14:48 EDT | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 2727569 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 14:48 EDT | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs as all prior passes. Both sockets responding. 2 lattice-node processes, 3 bash wrapper survivors (expected).

---

## Node Info

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Mesh identity | None |
| name | `morning-api` | `morning-api` | None |
| genesis_root_id | `auto` | `auto` (`--auto-genesis`) | None |
| chain_tip | 1 | 1 (genesis only) | None |
| uptime_secs | 16153 (~4.5h) | — | None |
| build_commit | `71aa16b-dirty` | git HEAD `aa62d12` | **Persistent DEVIATION.** 8 commits behind. Docs-only changes since binary build. |
| thickness | 995.69 | ~1000, slowly decaying | None (pass 25: 995.80; Δ = -0.11) |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Mesh identity | None |
| name | `local-witness` | `local-witness` | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api PeerId | None |
| chain_tip | 1 | 1 | None |
| uptime_secs | 16149 (~4.5h) | — | None |
| build_commit | `71aa16b-dirty` | Same binary | Persistent DEVIATION (same as morning-api). |

---

## Peer Connections

### morning-api
**OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness). heartbeats=1613, silence_secs=9, is_dead=false, queue_depth=0.
**EXPECTED:** 2-node mesh, 1 peer.
**DEVIATION:** None.

### local-witness
**OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api). heartbeats=1615, silence_secs=7, is_dead=false, queue_depth=0.
**EXPECTED:** 1 peer.
**DEVIATION:** None.

**Delta from pass 25:** Heartbeats morning-api +42 (1571→1613), witness +42 (1573→1615). Silence: morning-api 2s→9s (normal variation), witness 1s→7s (normal variation). Queue depth 0 on both.

---

## Epoch State

### morning-api (single capture, ~23:17:42Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 540 (socket), 540 (grep), 540 (last log line) | Cycling ~30s cadence. +15 since pass 25 (525→540). | **PASS — three-way match.** |
| ratio | 1.01931 | ~1.01–1.02 steady state (pass 25: 1.0199) | None (near-gaussian variation) |
| tax_calculated | 0 | Balance at 20: 5% rounds to 0. | None |
| tax_collected | 0 | Matches calculated. | None |
| minted | 0 | No minting after initial grant. | None |
| redistributed_to | 1 | 1 peer receiving redistribution | None |

**Three-way epoch match (~23:17:42Z):**
- Socket epoch: 540
- `grep -c` count: 540
- Last log line epoch: 540

**PASS.** All three agree.

### local-witness (single capture, ~23:17:43Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| epoch | 539 (socket), 539 (grep), 539 (last log line) | Same cadence. +14 since pass 25 (525→539). 1-epoch phase behind morning-api (540). | **PASS — three-way match on witness.** |
| ratio | 1.2025 | Continuing asymptotic decline (pass 25: 1.2073; Δ = -0.0048) | None |
| tax_calculated | 0 | 0 balance = no tax. | None |
| tax_collected | 0 | None | None |
| minted | 0 | None | None |
| redistributed_to | 1 | 1 peer | None |

**Three-way epoch match (~23:17:43Z):**
- Socket epoch: 539
- `grep -c` count: 539
- Last log line epoch: 539

**PASS** on witness individually. Nodes at 539 vs 540 — normal 1-epoch phase difference (~30s).

---

## Economic State

### morning-api
**OBSERVED:** own_balance=20, own_nonce=120. Peer (witness) balance=4980, nonce=0.
**EXPECTED:** Balance at asymptotic floor (20) since ~epoch 121. Nonce at 120 since no new transactions.
**DEVIATION:** None.

### local-witness
**OBSERVED:** own_balance=0, own_nonce=2. Peer (morning-api) balance=0, nonce=0.
**EXPECTED:** Zero-balance witness with no mint grant. Nonce 2 (max nonce applied).
**DEVIATION:** Persistent — witness sees morning-api balance as 0 (actual: 20). Same since pass 1.

---

## Persistence State

### morning-api
**OBSERVED:** last_snapshot_epoch=540, wal_bytes=0, wal_entries=0.
**EXPECTED:** Snapshot rotates every 10 epochs. WAL drained after rotation.
**DEVIATION:** None on the endpoint values.

**Byte-equality check (~23:17:28Z):** GetPersistenceState wal_bytes=0. `ls -la` shows wal.log at 379 bytes, wal.wal.old at 379 bytes.
**DEVIATION:** **Persistent UNKNOWN.** Same discrepancy as all prior passes.

**File inventory (single capture, ~23:17:28Z):**

| File | Size | mtime | Delta from pass 25 | Notes |
|------|------|-------|-------------------|-------|
| `persistence/state.snapshot` | 895 bytes | Jul 27 19:17:56 | mtime advanced 19:07→19:17:56; size unchanged (895) | **RESOLVED UNKNOWN** — mtime now confirmed to advance. Prior observation gap was too narrow. |
| `persistence/wal.log` | 379 bytes | Jul 27 19:17:56 | mtime advanced from 19:07 | Genesis re-seed post-rotation |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 19:12 | mtime advanced from 19:02 | Prior rotation's WAL backup |

**Snapshot epoch progression:** 500→520 (pass 24→25), 520→540 (pass 25→26). +20 per ~7h. Consistent with 2 rotations per pass.

### local-witness
**OBSERVED:** last_snapshot_epoch=530, wal_bytes=0, wal_entries=0.
**EXPECTED:** Same rotation schedule as morning-api.
**DEVIATION:** **New observation — snapshot epoch divergence.** morning-api at 540, witness at 530. 10-epoch gap. First time observed. Status: UNKNOWN — not yet classified as deviation.

**File inventory:**

| File | Size | mtime | Delta from pass 25 |
|------|------|-------|-------------------|
| `persistence/state.snapshot` | 569 bytes | Jul 27 19:18:13 | mtime advanced 19:08→19:18:13; size unchanged |
| `persistence/wal.log` | 379 bytes | Jul 27 19:18:13 | mtime advanced from 19:08 |
| `persistence/wal.wal.old` | 379 bytes | Jul 27 19:08 | mtime advanced from 19:03 |

**Snapshot epoch progression:** 500→520 (pass 24→25), 520→530 (pass 25→26). +10 per ~7h. Only 1 rotation v. morning-api's 2.

---

## Build Commit & Binary Freshness

**OBSERVED:** `71aa16b-dirty` on both nodes (unchanged since pass 1).
**EXPECTED (VERIFIED-BEHAVIOR.md):** Should match git HEAD (`aa62d12`).
**DEVIATION:** **Persistent.** 8 commits behind. Docs-only changes — no wire-format, codec, or protocol changes.

Git HEAD: `aa62d12` ("docs: note /tmp identity dir fragility across reboots")
Running binary: `71aa16b` ("wip: update Cargo.lock") + `-dirty`

Commits between (all docs/test, no wire-format changes):
1. `b4aa212` test: cap enforcement fixture
2. `214eb73` fix: witness harness fixture bugs
3. `32efcf1` fix: stale fixture bugs
4. `19c9d05` docs: split MESH.md / VERIFIED-BEHAVIOR.md
5. `7ab64c2` docs: sharpen MESH.md
6. `93d0ef4` docs: verified behavior tiers
7. `c008def` docs: MESH.md PeerIds after relaunch
8. `aa62d12` docs: /tmp identity dir fragility

---

## Log Health

**morning-api (/tmp/m-ap.log):**
- 0 unexpected WARN/ERROR lines (healthy).
- Periodic `libp2p_kad WARN Failed to trigger bootstrap: No known peers` every 5 minutes — benign with `--no-mdns`.
- No ERROR lines.
- No sweep/evict/zombie activity detected (confirmed by grep for "sweep|evict|zombie|stale" — zero hits in entire log).

**local-witness (/tmp/lw.log):**
- 0 WARN, 0 ERROR lines.
- No sweep/evict/zombie activity.

---

## Metrics (Last 5 Tick Lines)

### morning-api
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=3s
(all 5 lines identical, timestamps 23:17:16–23:17:56Z)
```
**All clean:** zero fetches, zero queues, silence=3s. Unchanged from pass 25.

### local-witness
```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
(all 5 lines identical, timestamps 23:17:23–23:18:03Z)
```
**All clean:** zero fetches, zero queues, silence=6s. Unchanged from pass 25.

---

## Recent Epoch Activity (Last 5 Lines Each)

**morning-api (at ~23:17Z):**
```
Epoch complete epoch=536 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=537 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=538 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=539 balance_before=20 balance_after=20 ratio=1.02
Epoch complete epoch=540 balance_before=20 balance_after=20 ratio=1.02
```
Balance locked at 20. Ratio stable at ~1.02.

**local-witness (at ~23:17Z):**
```
Epoch complete epoch=536 balance_before=0 balance_after=0 ratio=1.20
Epoch complete epoch=537 balance_before=0 balance_after=0 ratio=1.20
Epoch complete epoch=538 balance_before=0 balance_after=0 ratio=1.20
Epoch complete epoch=539 balance_before=0 balance_after=0 ratio=1.20
Epoch complete epoch=540 balance_before=0 balance_after=0 ratio=1.20
```
Ratio: 1.2025 at socket query (declining from 1.2073 at pass 25). Continuing expected asymptotic decay.

---

## Summary of Persistent Deviations

| # | Node | Item | Severity | Status |
|---|------|------|----------|--------|
| 1 | both | `build_commit` = `71aa16b-dirty` (stale, 8 commits behind HEAD `aa62d12`) | Low — docs-only drift, no wire-format change | **Persistent** since pass 1 |
| 2 | both | `GetPersistenceState.wal_bytes` returns 0 but files exist (379B each) | Low — API endpoint not wired | **Persistent** since pass 1 |
| 3 | local-witness | witness sees morning-api balance as 0 (actual: 20) | Medium — functional impact, mesh stays healthy | **Persistent** since pass 1 |

**Deviations resolved since pass 25:**
- **UNKNOWN #2 (snapshot mtime not advancing):** RESOLVED. Snapshot mtime confirmed to advance (19:07→19:17). The observation gap between passes 24→25 (11 min) was too narrow to capture a 10-epoch rotation window.

**New observations since pass 25:**
- **Snapshot epoch divergence:** morning-api last_snapshot_epoch=540, witness=530. 10-epoch gap. First time observed. UNKNOWN — not yet classified as deviation.

---

## Delta from Pass 25 (23:10Z → 23:17Z)

| Metric | Pass 25 (~23:10Z) | Pass 26 (~23:17Z) | Delta |
|--------|--------------------|--------------------|-------|
| Uptime (morning-api) | 15725s | 16153s | +428s (~7.1 min) |
| Uptime (witness) | 15720s | 16149s | +429s |
| Epoch (morning-api socket) | 525 | 540 | +15 |
| Epoch (witness socket) | 525 | 539 | +14 (1-epoch phase diff) |
| Heartbeats (morning-api) | 1571 | 1613 | +42 |
| Heartbeats (witness) | 1573 | 1615 | +42 |
| Silence (morning-api) | 2s | 9s | +7s (normal variation) |
| Silence (witness) | 1s | 7s | +6s (normal variation) |
| Thickness | 995.80 | 995.69 | -0.11 (expected decay) |
| Balance | 20 | 20 | 0 |
| Nonce | 120 | 120 | 0 |
| Snapshot epoch (morning-api) | 520 | 540 | +20 (2 rotations) |
| Snapshot epoch (witness) | 520 | 530 | +10 (1 rotation) — **new divergence** |
| Snapshot size (morning-api) | 895 bytes | 895 bytes | 0 |
| Snapshot size (witness) | 569 bytes | 569 bytes | 0 |
| Snapshot mtime (morning-api) | 19:07 | 19:17:56 | Advanced (+10m) — **resolved UNKNOWN** |
| Snapshot mtime (witness) | 19:08 | 19:18:13 | Advanced (+10m) |
| Queue depth | 0 | 0 | 0 |
| Build commit | `71aa16b-dirty` | `71aa16b-dirty` | Unchanged |
| WARN count (both, filtered) | 0 unexpected | 0 unexpected | 0 |

---

## UNKNOWN Items

1. **wal_bytes vs disk size discrepancy** (unchanged from all prior passes). GetPersistenceState reports wal_bytes=0, but `ls -la` shows wal.log at 379 bytes on both nodes.

2. **witness sees morning-api balance as 0** (actual: 20). Persistent across all 26 passes. Mesh stays healthy; functional impact is limited to incorrect balance display on the witness.

3. **Snapshot epoch divergence (NEW).** morning-api last_snapshot_epoch=540, witness=530 at ~23:17Z. First time snapshot epochs are not equal between nodes. Not yet determined whether this is (a) a phase lag where witness's next rotation catches up, (b) a persistent skew, or (c) a symptom. Observer does not diagnose.

---

## Raw Capture Bundle

Single-capture queries from ~23:17:28Z (composite):

```
// Timestamp: 2026-07-27T23:17:28Z (bundle, spread ~+30s for witness)

// === GetNodeInfo (morning-api) ===
{"type":"NodeInfo","peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":"morning-api","genesis_root_id":"auto","chain_tip":1,"uptime_secs":16153,"build_commit":"71aa16b-dirty","thickness":995.6851908079951}

// === GetPeers (morning-api) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":null,"heartbeats":1613,"silence_secs":9,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (morning-api) — three-way: endpoint=540, grep=540, last line=540 (PASS) ===
{"type":"EpochState","epoch":540,"ratio":1.0193151018353885,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (morning-api) ===
{"type":"EconomicState","own_balance":20,"own_nonce":120,"peers":[{"peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","balance":4980,"nonce":0}]}

// === GetPersistenceState (morning-api) — wal_bytes=0, wal.log=379 bytes (UNKNOWN) ===
{"type":"PersistenceState","last_snapshot_epoch":540,"wal_bytes":0,"wal_entries":0}

// === GetHeight (morning-api) ===
{"type":"Height","height":1}

// === GetNodeInfo (local-witness) ===
{"type":"NodeInfo","peer_id":"12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch","name":"local-witness","genesis_root_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","chain_tip":1,"uptime_secs":16149,"build_commit":"71aa16b-dirty"}

// === GetPeers (local-witness) — 1 peer ===
{"type":"Peers","peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","name":null,"heartbeats":1615,"silence_secs":7,"is_dead":false,"queue_depth":0}]}

// === GetEpochState (local-witness) — three-way: endpoint=539, grep=539, last line=539 (PASS) ===
{"type":"EpochState","epoch":539,"ratio":1.2025125935594396,"tax_calculated":0,"tax_collected":0,"minted":0,"redistributed_to":1}

// === GetEconomicState (local-witness) ===
{"type":"EconomicState","own_balance":0,"own_nonce":2,"peers":[{"peer_id":"12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ","balance":0,"nonce":0}]}

// === GetPersistenceState (local-witness) — wal_bytes=0, snapshot_epoch=530 ===
{"type":"PersistenceState","last_snapshot_epoch":530,"wal_bytes":0,"wal_entries":0}

// === File inventory (morning-api) ===
state.snapshot  895 bytes  mtime: 19:17:56
wal.log         379 bytes  mtime: 19:17:56
wal.wal.old     379 bytes  mtime: 19:12

// === File inventory (witness) ===
state.snapshot  569 bytes  mtime: 19:18:13
wal.log         379 bytes  mtime: 19:18:13
wal.wal.old     379 bytes  mtime: 19:08

// === Git HEAD ===
aa62d12 docs: note /tmp identity dir fragility across reboots
```

---

## Bottom Line

**No new deviations. Two retained persistent anomalies (wal_bytes discrepancy, balance divergence). One UNKNOWN resolved (snapshot mtime advancing confirmed). One new observation (snapshot epoch divergence: morning-api=540, witness=530). Mesh has been running healthy for ~8.5h. Zero queues, zero fetches, zero sweep/evict/zombie activity. Three-way epoch PASS on both nodes individually.**

**Next observation pass:** Scheduled cron. Threshold warning if snapshot epoch gap persists or widens.
