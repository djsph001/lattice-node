# Observer Record — Pass 89

**Date:** 2026-07-28
**Time:** 11:55 UTC (first capture), 11:57 UTC (re-queries)
**Machine:** z4-workstation (Ubuntu 24.04, Boynton Beach, FL)
**Observer:** Engineering Cell — autonomous cron pass

---

## Topology

| Node | PeerId | UDS Socket | PID | Command |
|------|--------|-----------|-----|---------|
| morning-api | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | `/tmp/m-ap/lattice.sock` | 2727391 | `--name morning-api --port 4005 --auto-genesis --no-mdns --persistence --mint 5000` |
| local-witness | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | `/tmp/local-witness/lattice.sock` | 2727569 | `--name local-witness --port 4010 --genesis-root 12D3KooWPfr... --bootstrap-peer ... --no-mdns --persistence --mint 0` |

Both launched Jul 27 (uptime ~17.1h), both running same binary (`71aa16b-dirty`). Identity dirs at `/tmp/m-ap-id/` and `/tmp/lw-id/` (tmpfs — will not survive reboot).

Also present on host: 21 stale `.sock` files from earlier test runs (`/tmp/gr-an/`, `/tmp/lv-quick/`, `/tmp/as/`, etc.). These are NOT running processes — no matching PIDs. All are from previous isolated test sessions and are harmless debris.

---

## Metric Observations

### 1. GetNodeInfo — Build Identity

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| name | `morning-api` | `morning-api` | None |
| peer_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (per MESH.md) | None |
| genesis_root_id | `auto` | N/A (auto-genesis mode) | — |
| chain_tip | `1` | `1` (genesis only — no blocks) | None |
| build_commit | `71aa16b-dirty` | `cb5d4b1` (HEAD of main) | **DEVIATION: 9 commits behind HEAD.** Running binary compiled from `71aa16b` (`wip: update Cargo.lock`, Jul 27 13:46). HEAD is `cb5d4b1` (`docs: Observer evidence corpus + Verifier missions 1 and 2`). All 9 missing commits are docs + test-only changes. `-dirty` suffix means uncommitted source changes were present at build time. **First observed: this pass.** |
| uptime_secs | `61672` (T0) | N/A | — |
| thickness | `983.64` | N/A | — |

**Build gap details (commits in `71aa16b..cb5d4b1`):**
```
cb5d4b1 docs: Observer evidence corpus + Verifier missions 1 and 2
aa62d12 docs: note /tmp identity dir fragility across reboots
c008def docs: MESH.md — record stable PeerIds after mesh relaunch
93d0ef4 docs: restructure verified behavior with evidence tiers
7ab64c2 docs: sharpen MESH.md header to configuration-focused language
19c9d05 docs: split MESH.md (topology only) from VERIFIED-BEHAVIOR.md
32efcf1 fix: stale fixture bugs in witness harness — epoch + witness identity
214eb73 fix: declare claimant variable in two_swarm witness harness tests
b4aa212 test: cap enforcement — 64th accepted, 65th rejected, duplicate is no-op
```
**Classification:** All 9 missing commits are docs or test-only fixes. No production code changes missing. The `-dirty` composition is unknown.

### 2. GetNodeInfo — local-witness (cross-check)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (per MESH.md) | None |
| genesis_root_id | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | morning-api's peer ID | None |
| build_commit | `71aa16b-dirty` | Same binary as morning-api | None |
| uptime_secs | `61677` | ~same as morning-api (61672) | None (20s difference from query timing) |

### 3. GetEpochState — Epoch Progression

| Field | OBSERVED (morning-api T0) | OBSERVED (morning-api T+81s) | OBSERVED (witness T+120s) |
|-------|--------------------------|------------------------------|---------------------------|
| epoch | 2054 | 2056 | 2057 |
| ratio | 1.019814 | 1.019814 | 1.047945 |
| tax_calculated | 0 | 0 | 0 |
| tax_collected | 0 | 0 | 0 |
| minted | 0 | 0 | 0 |
| redistributed_to | 1 | 1 | 1 |

**Progression:** +2 epochs in ~81s morning-api side → +3 in ~120s with witness. Rate: ~40s/epoch. Consistent with default epoch duration.

**Tax/collect/mint all zero (PERSISTENT):** All `0` since first observation (Jul 27 pass 1, ~20:18 EDT). `redistributed_to=1` fires every epoch but produces no mint or tax collection. Matches the VERIFIED-BEHAVIOR.md documentation: `--mint 5000` on morning-api applies locally; redistribution debits morning-api but witness rejects. **First observed: Jul 27, pass 3 (18:48 EDT) — documented in VERIFIED-BEHAVIOR.md.**

### 4. GetPeers — Connectivity

| Field | OBSERVED (morning-api T0) | OBSERVED (T+81s) |
|-------|--------------------------|-------------------|
| peer_count | 1 | 1 |
| peer_id | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness) | same |
| heartbeats | 6158 | 6165 (+7) |
| silence_secs | 3 | 8 |
| is_dead | false | false |
| queue_depth | 0 | 0 |

**Status:** HEALTHY. Single peer connected, heartbeats incrementing, silence < 10s, queue empty. No deviation.

### 5. GetPersistenceState — WAL & Snapshot

**Both nodes identical:**

| Field | OBSERVED (morning-api) | OBSERVED (witness) | EXPECTED |
|-------|----------------------|-------------------|----------|
| last_snapshot_epoch | 2050 | 2050 | N/A |
| wal_bytes | 0 | 0 | Should equal `ls -la persistence/wal.log` file size |
| wal_entries | 0 | 0 | Should reflect actual entry count |

**DEVIATION — `wal_bytes=0`, **wal_entries=0**, but `wal.log` exists with content:**

- `/tmp/m-ap/persistence/wal.log`: **379 bytes** — but endpoint reports 0
- `/tmp/local-witness/persistence/wal.log`: **379 bytes** — but endpoint reports 0
- `state.snapshot` present on both (895 bytes morning-api, 569 witness)
- `wal.wal.old` present on both (379 bytes each)

**Root cause (known, NOT a new finding):** `get_stats()` at `src/ledger/persistence.rs:696-697` reads `self.wal_path` (legacy `transactions.wal`, retired in unified WAL migration). Since `transactions.wal` no longer exists, `metadata()` fails → `unwrap_or(0)` → 0. The correct path is `self.unified_wal_path` (`wal.log`).

**First observed:** Jul 27, Verifier Mission 2 (evidence at `docs/evidence/verifier-walbytes-audit-2026-07-27.md`). Not yet fixed in the running binary (`71aa16b-dirty`) or in HEAD (`cb5d4b1`) — both still read `self.wal_path`.

**`wal.log` content analysis:**
- Both nodes: 379 bytes, last modified ~07:52-07:53 (consistent with snapshot rotation at epoch 2050)
- `wal.wal.old`: 379 bytes, last modified ~07:47-07:48 (pre-rotation WAL)
- Since epoch 2050, the current WAL has been static. Either: (a) no transactions have been applied since the snapshot, or (b) transactions are being applied but not persisted to the unified WAL path. UNKNOWN which.

### 6. GetEconomicState — Balances & Nonces

#### morning-api perspective:

| Field | OBSERVED | EXPECTED (per MESH.md `--mint 5000`) |
|-------|----------|---------------------------------------|
| own_balance | 20 | 5,000 (initial mint) — **changed** |
| own_nonce | 120 | 0 (initial) — **changed** |
| peer (witness) balance | 4,980 | N/A |
| peer (witness) nonce | 0 | N/A |

#### local-witness perspective:

| Field | OBSERVED | EXPECTED |
|-------|----------|----------|
| own_balance | 0 | 0 (`--mint 0`) |
| own_nonce | 2 | 2 (genesis + bootstrap_ended) |
| peer (morning-api) balance | 0 | N/A |
| peer (morning-api) nonce | 0 | N/A |

#### Cross-node supply reconciliation:

| Source | Balance | Source | Balance | Sum |
|--------|---------|--------|---------|-----|
| morning-api (self) | 20 | morning-api (witness) | 4,980 | 5,000 |
| witness (self) | 0 | witness (morning-api) | 0 | 0 |

**DEVIATION — Supply conservation contradicted (PERSISTENT since Jul 27, 18:48 EDT):**
- morning-api believes total supply = 20 + 4,980 = **5,000**
- witness believes total supply = 0 + 0 = **0**
- Neither matches the initial 5,000 mint (per morning-api)
- morning-api debited 4,980 across 118 redistribution transfers (nonce=120 minus genesis+bootstrap=2)
- Witness accepted 2 transactions (genesis + bootstrap_ended), rejected all 118 transfers

**First observed:** Jul 27, Observer pass 3 (18:48 EDT). Decomposed by Verifier Mission 1 (Jul 27, 23:23:12Z):
- CONFIRMED: transfer path integrity (118/118 delivered, exact amounts)
- CONTRADICTED: supply conservation (sender debited 4,980, recipient credited 0)

**Causes (per VERIFIED-BEHAVIOR.md, NOT observer diagnosis):**
1. Initial mint is local, not propagated (never reaches wire on 2-node mesh)
2. Sender debits before recipient confirmation (unconditional debit in validate_and_apply)
3. No reconciliation mechanism exists between nodes

### 7. GetHeight

| Field | OBSERVED |
|-------|----------|
| height | 1 (genesis only) |

Consistent with `chain_tip=1` from NodeInfo. No blocks beyond genesis.

---

## Delta Summary (vs Previous Pass — pass88 at 07:49)

| Metric | Pass 88 | Pass 89 | Change |
|--------|---------|---------|--------|
| Epoch | ~2050 | 2054-2057 | +4-7 epochs (consistent with ~3h gap) |
| Uptime | ~57600s | 61591s+ | +~5300s (~88min) — consistent with ~3h elapsed |
| morning-api balance | 20 | 20 | Unchanged |
| morning-api nonce | 120 | 120 | Unchanged |
| witness balance | 0 | 0 | Unchanged |
| witness nonce | 2 | 2 | Unchanged |
| Build commit | 71aa16b-dirty | 71aa16b-dirty | Unchanged |
| wal_bytes | 0 | 0 | Unchanged (bug not fixed) |
| Peers | 1 (witness) | 1 (witness) | Unchanged |
| Heartbeats | ~5700 | 6158-6165 | +~450 (consistent) |

**Nothing changed structurally since pass88.** The mesh is in a stable equilibrium: epoch cycling, heartbeats flowing, no state mutations (balances frozen, nonces frozen, build commit unchanged).

---

## UNKNOWN Items

1. **`wal.log` content since epoch 2050 snapshot:** Both nodes have 379-byte `wal.log` files that haven't changed since ~07:52 (3+ hours ago). If epoch redistribution is still firing (it appears to be — epoch counts increase), it may be producing transactions that are NOT persisted to WAL. UNKNOWN: whether the redistribution path calls `persist_record()` or bypasses it.

2. **thickness divergence (if any):** morning-api reports thickness=983.64 — not checked against witness. Not part of this pass's query scope. Categorized as UNKNOWN for this pass.

3. **`-dirty` composition in build:** Running binary at `71aa16b-dirty` — unknown what uncommitted changes were compiled in. `git status` shows only `docs/evidence/` (observer records) as modified, which postdate build time.

---

## Classification Summary

| Metric | Status | Novelty |
|--------|--------|---------|
| Build commit lag (71aa16b vs cb5d4b1) | DEVIATION — 9 commits stale | NEW this pass |
| `-dirty` suffix in build_commit | DEVIATION — unknown composition | NEW this pass |
| `wal_bytes=0` (wrong WAL path) | DEVIATION — known bug | PERSISTENT (Jul 27, Verifier Mission 2) |
| Supply conservation | CONTRADICTED — witness sees 0, morning-api sees 5000 | PERSISTENT (Jul 27, 18:48 EDT) |
| Epoch cycling | OBSERVED — healthy at ~40s/epoch | CONTINUED |
| Peer connectivity | OBSERVED — 1 peer, silence <10s, queue empty | CONTINUED |
| Height | OBSERVED — 1 (genesis only) | CONTINUED |
| Economic state frozen | OBSERVED — balances/nonces unchanged since ~07:49 | CONTINUED (since Jul 27) |
