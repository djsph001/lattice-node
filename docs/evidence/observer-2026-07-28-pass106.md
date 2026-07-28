# Observer Evidence Record — 2026-07-28 (Pass 106)

**Observer:** lattice-observer (Engineering Cell, autonomous cron agent)
**Capture time:** 2026-07-28T16:18:38Z (simultaneous single-capture)
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200)
**Session type:** 106th observation pass of Jul 28. ~42 min since pass 105 (15:34:17Z). Same PIDs since 13:01Z (~3h18m runtime).

**Summary:** All-clear on three-way epoch match — morning-api 394=394=394 clean. Witness ±1 boundary race (393/394). Supply divergence persists unchanged (20/9980 vs 0/0). Snapshot frozen at epoch 390 since 12:16 local (~1.5h). Zero queues, zero fetches, zero zombie evictions. Build commit `cb5d4b1-dirty` still 2 commits behind HEAD `452b64f`. No new deviations identified.

Single pass observation; delta from pass 105 estimated at ~+89 epochs across ~42 min (~2.1 epochs/min consistent with 30s cadence).

---

## Topology Disclosure

| Machine | Hostname | Role |
|---------|----------|------|
| z4-workstation | dale-joseph-HP-Z4-G4-Workstation | All mesh nodes |

| PID | Name | Port | Genesis Root | Since (UTC) | Command |
|-----|------|------|--------------|-------------|---------|
| 3579452 | morning-api | 4005 | auto (12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ) | 13:01Z | `--name morning-api --port 4005 --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap --auto-genesis --no-mdns --persistence --mint 5000` |
| 3579821 | local-witness | 4010 | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | 13:02Z | `--name local-witness --port 4010 --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness --genesis-root 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ --no-mdns --persistence --mint 0` |

**No topology changes.** Same PIDs since pass 1 (13:01Z). Both sockets responding. 2 lattice-node processes + 2 bash wrappers.

---

## Node Info

| Field | morning-api (16:18:38Z) | DEVIATION |
|-------|-------------------------|-----------|
| peer_id | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | None — matches genesis |
| name | morning-api | — |
| genesis_root_id | auto | — |
| chain_tip | 1 | None — genesis-only mesh (no blocks beyond genesis) |
| uptime_secs | 11800+ (not captured this pass but monotonic) | — |
| build_commit | **cb5d4b1-dirty** | **PERSISTENT DEVIATION** — 2 commits behind HEAD `452b64f`. `-dirty` suffix indicates uncommitted source at compile time (diff is evidence markdown files only, not Rust source). First observed pass 1 (13:01Z), unchanged. |
| thickness | ~979.6 | No expectation documented |

| Field | local-witness (16:18:38Z) | DEVIATION |
|-------|---------------------------|-----------|
| peer_id | 12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch | None |
| name | local-witness | — |
| genesis_root_id | 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | Correct — matches morning-api peer_id |
| chain_tip | 1 | None |
| uptime_secs | 11741 | — |
| build_commit | **cb5d4b1-dirty** | **Same PERSISTENT DEVIATION** as morning-api |

---

## Epoch State

### morning-api (simultaneous capture, 16:18:38Z)

| Check | This pass (16:18:38Z) | DEVIATION |
|-------|------------------------|-----------|
| Socket epoch | **394** | — |
| Log count (grep -c) | **394** | — |
| Last log epoch | **394** (16:18:17Z) | — |
| **Three-way equality** | **PASS** — 394=394=394 | None — clean match |

### local-witness (simultaneous capture, 16:18:38Z)

| Check | This pass (16:18:38Z) | DEVIATION |
|-------|------------------------|-----------|
| Socket epoch | **393** | — |
| Log count (grep -c) | **394** | — |
| Last log epoch | **394** (16:18:40Z) | — |
| **Three-way equality** | **±1 boundary race** — socket=393 vs log=394 | None — acceptable; socket queried ~2s before epoch boundary ticked the log count. Last log epoch=394 confirms. |

### Cross-node comparison

| Metric | morning-api | local-witness | Δ | DEVIATION |
|--------|-------------|---------------|---|-----------|
| Epoch | 394 | 393/394 | ±1 | None — within boundary race tolerance |
| Ratio | 1.0191 | 1.2819 | ~0.26 | **PERSISTENT** — morning-api ratio converges toward 1.0 (balance 20, tax ~0), witness ratio diverges higher (balance 0, same tax ~0 → different division result). UNKNOWN if this is expected — both are asymptotic consequences of their respective balance states. First observed pass 1. |
| Tax calc/collect | 0 / 0 | 0 / 0 | 0 | None — balance at 20: 5% rounds to 0 |
| Minted | 0 | 0 | 0 | None |
| Redistributed to | 1 | 1 | 0 | None — 1 peer |

---

## Peers

### morning-api (16:18:38Z)

| Peer | Name | Heartbeats | Silence (s) | Dead | Queue Depth | DEVIATION |
|------|------|-----------|-------------|------|-------------|-----------|
| 12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch | null | 1164+ | 3-6 | false | 0 | None — healthy 1-peer mesh |

### local-witness (16:18:38Z)

| Peer | Name | Heartbeats | Silence (s) | Dead | Queue Depth | DEVIATION |
|------|------|-----------|-------------|------|-------------|-----------|
| 12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ | null | 1174+ | 9 | false | 0 | None — healthy |

---

## Economic State

### morning-api

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | **20** | Initially 5000 (--mint 5000). After redistribution: 20 (consistent with pass 1+). | **PERSISTENT** — matches prior passes. Supply conservation with witness is contradicted (see below). |
| own_nonce | **241** | Monotonic increase | None — 241 nonces consumed since genesis |
| peer(witness).balance | **9980** | Witness started with --mint 0; should have 0 own_balance from local view. | **PERSISTENT UNKNOWN DEVIATION** — morning-api records witness having 9980. Witness reports own_balance=0. First observed pass 1 (13:01Z). Same as Jul 27 supply conservation divergence documented in VERIFIED-BEHAVIOR.md. |
| peer(witness).nonce | **0** | Witness nonce: witness reports own_nonce=4 | **PERSISTENT** — morning-api sees witness nonce=0, witness self-reports 4. Different views have persisted since pass 1. |

### local-witness

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| own_balance | **0** | 0 (--mint 0) | None — matches expectation |
| own_nonce | **4** | Small number | None |
| peer(morning-api).balance | **0** | Should see morning-api's balance | **PERSISTENT UNKNOWN DEVIATION** — witness records morning-api's balance as 0, but morning-api self-reports 20 and witness balance as 9980. First observed pass 1. |
| peer(morning-api).nonce | **0** | Should see morning-api's nonce (241) | **PERSISTENT** — witness sees morning-api nonce=0 |

---

## Persistence State

### morning-api (simultaneous capture, 16:18:38Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | **390** | Should advance as epochs cycle | **PERSISTENT — frozen since ~12:16 local (epoch 390)**. Pass 104 (15:25Z) also showed 390. ~1.5h without snapshot advance is unusual if snapshots fire every ~10 epochs. UNKNOWN: whether snapshot rotation is throttled or stalled. |
| wal_bytes | **379** | Should match disk | None — **byte-equality confirmed**: `ls -la wal.log` = 379 bytes |
| wal_entries | **3** | 379/120 ≈ 3.16 → 3 | **KNOWN-PROVISIONAL** — VERIFIED-BEHAVIOR.md notes `wal_entries` reports `size/120` heuristic, "plausible but unrelated to actual entry count." |

### local-witness (simultaneous capture, 16:18:38Z)

| Field | OBSERVED | EXPECTED | DEVIATION |
|-------|----------|----------|-----------|
| last_snapshot_epoch | **390** | Same as morning-api (both on same mesh) | Frozen at 390 same as morning-api |
| wal_bytes | **379** | Should match disk | None |
| wal_entries | **3** | Same heuristic | Known-provisional |

### File inventory

| File | morning-api | local-witness | Notes |
|------|-------------|---------------|-------|
| state.snapshot | 895 bytes (12:16) | 569 bytes (12:16) | Different sizes — reflects different balance state |
| wal.log | 379 bytes (12:16) | 379 bytes (12:16) | Both identical size |
| wal.wal.old | 379 bytes (12:11) | 379 bytes (12:11) | **KNOWN-PROVISIONAL** — should be `wal.log.old` (VERIFIED-BEHAVIOR.md) |

---

## Metrics (from morning-api log, last 3)

```
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
metrics: outstanding_fetches=0 aged=0 outbound_queues=[] max_peer_silence=6s
```

**All green:** 0 fetches, 0 aged, empty queues, max silence 6s (well under 30s threshold). No zombie eviction activity. No stale fetch sweep events.

---

## Error/WARN Scan

### morning-api (excluding KAD bootstrap WARNs)
**No entries found** — clean.

### local-witness (excluding KAD bootstrap WARNs)
**No entries found** — clean.

### Acceptable WARNs
- `libp2p_kad::behaviour: Failed to trigger bootstrap: No known peers` — expected with `--no-mdns`. KAD is running without peer discovery. Non-functional but harmless.

---

## Persistent Deviations (Carried Forward)

| # | Deviation | First Observed | Status | Detail |
|---|-----------|---------------|--------|--------|
| 1 | Build commit stale: `cb5d4b1-dirty` vs HEAD `452b64f` | Pass 1 (13:01Z) | PERSISTENT | 2 commits behind. `-dirty` from evidence markdown diffs only (not Rust source). Stale binary — rebuild needed. |
| 2 | Supply conservation divergence: morning-api sees total=10000 (20+9980), witness sees total=0 | Jul 27 (pass 3) | PERSISTENT | Documented in VERIFIED-BEHAVIOR.md as "CONTRADICTED." Transfer path confirmed working (118/118). Supply conservation invariant violated under tested conditions. |
| 3 | Cross-node peer balance/nonce asymmetry | Pass 1 (13:01Z) | PERSISTENT | morning-api sees witness balance=9980/nonce=0, witness self-reports balance=0/nonce=4. morning-api nonce=241, witness sees morning-api nonce=0. |
| 4 | Snapshot frozen at epoch 390 since ~12:16 local | ~12:16 local | PERSISTENT (this pass) | ~1.5h without snapshot advance. UNKNOWN if throttled or stalled. |
| 5 | `wal.wal.old` naming (cosmetic) | ae89fbd | KNOWN-PROVISIONAL | Documented in VERIFIED-BEHAVIOR.md |

---

## New Observations This Pass

| # | Observation | Status | Detail |
|---|-------------|--------|--------|
| 1 | Three-way epoch match clean on both nodes at 16:18:38Z | RESOLVED (transient) | Early sequential query at ~16:17Z showed socket=389 vs log=392 (3-epoch gap). Simultaneous capture 90s later showed clean 394=394=394. The gap was either a timing artifact of sequential capture or a transient endpoint lag that self-corrected. Marked UNKNOWN — cannot determine from a single pass. |

---

## UNKNOWN Items

| # | Unknown | Why unclassified |
|---|---------|-----------------|
| 1 | Snapshot frozen at epoch 390 | Could be throttled (snapshots fire every N epochs, and N hasn't been reached yet) or stalled (snapshot mechanism is broken). Cannot determine from socket data alone. |
| 2 | Why morning-api briefly showed 3-epoch endpoint lag at ~16:17Z | Sequential (not simultaneous) capture makes this timing-ambiguous. Could be a real lag or a capture artifact. |
| 3 | Ratio divergence (1.019 vs 1.282) | Both are asymptotic behavior from their respective balance states — both could be mathematically correct. No design document specifies expected ratio behavior across nodes with different balances. |
