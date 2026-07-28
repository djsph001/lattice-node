# Observer Evidence Record — 2026-07-28, Pass 11

**Timestamp (all values):** 2026-07-28T13:42:27Z
**Machine:** dale-joseph-HP-Z4-G4-Workstation (192.168.10.200, 100.93.232.107 Tailscale)
**Observer:** Hermes cron job
**Socket:** `/tmp/m-ap/lattice.sock` (morning-api), `/tmp/local-witness/lattice.sock` (local-witness)
**Topology:** 2-node local mesh, both on same machine

| Node | Port | Identity | Role |
|------|------|----------|------|
| morning-api | 4005 | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` | Auto-genesis root, mint=5000 |
| local-witness | 4010 | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` | Bootstrap peer, mint=0 |

**Uptime:** morning-api=2429s (~40 min), witness=2408s (~40 min)
**Binary:** Both running `cb5d4b1-dirty` (suffix means uncommitted changes present at build time)

---

## 1. Node Info (GetNodeInfo)

### morning-api

- **OBSERVED:** peer_id=`12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ`, name=`morning-api`, genesis_root_id=`auto`, chain_tip=1, uptime_secs=2429, build_commit=`cb5d4b1-dirty`, thickness=982.02
- **EXPECTED:** peer_id matches MESH.md. chain_tip=1 (genesis only, no blocks beyond genesis). build_commit should match git HEAD.
- **DEVIATION:** build_commit `cb5d4b1-dirty` vs git HEAD `452b64f`. Running binary is 1 commit behind HEAD. `-dirty` suffix confirms the working tree had uncommitted changes at build time.
- **FIRST OBSERVED:** This observer pass. Not known whether the binary was already stale at previous passes.

### local-witness

- **OBSERVED:** peer_id=`12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch`, name=`local-witness`, genesis_root_id=`12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ`, chain_tip=1, uptime_secs=2408, build_commit=`cb5d4b1-dirty`
- **EXPECTED:** genesis_root_id matches morning-api's PeerId. chain_tip=1.
- **DEVIATION:** Same stale build_commit as morning-api.
- **NOTE:** witness does NOT report a `thickness` field (unlike morning-api). UNKNOWN whether the field was omitted intentionally or this node's binary doesn't include it.

---

## 2. Peers (GetPeers)

### morning-api sees

- **OBSERVED:** 1 peer: `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` (local-witness), heartbeats=240, silence_secs=0, is_dead=false, queue_depth=0
- **EXPECTED:** 1 peer (local-witness) actively connected, silence near zero.
- **DEVIATION:** None. Connection healthy, zero silence, zero queue depth.

### local-witness sees

- **OBSERVED:** 1 peer: `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` (morning-api), heartbeats=237, silence_secs=6, is_dead=false, queue_depth=0
- **EXPECTED:** 1 peer actively connected.
- **DEVIATION:** silence_secs=6 (small). Within normal range for a polling interval. No concern.
- **NOTE:** 3-heartbeat gap between nodes (240 vs 237) — expected for independent timer ticks.

---

## 3. Epoch State (GetEpochState)

### morning-api

- **OBSERVED:** epoch=82, ratio=1.016, tax_calculated=4, tax_collected=4, minted=0, redistributed_to=1
- **EXPECTED:** Epoch advances monotonically. Tax collected > 0 (morning-api has spendable balance).
- **DEVIATION:** None isolated. Ratio is stable near 1.016. Tax cycle functioning.

### local-witness

- **OBSERVED:** epoch=81, ratio=1.93, tax_calculated=0, tax_collected=0, minted=0, redistributed_to=1
- **EXPECTED:** Epoch should be within 1 of morning-api.
- **DEVIATION:** 1-epoch gap between nodes (82 vs 81). Within normal range for independent epoch timers. Witness tax=0 with balance=0 is consistent (nothing to tax).
- **NOTE:** Witness ratio=1.93 vs morning-api ratio=1.02 — massive divergence in ratio calculation. UNKNOWN whether this is expected (witness has zero balance, zero thickness, so the ratio formula may diverge).

### Three-way epoch equality comparison

- Morning-api: endpoint=82, `grep -c "Epoch complete"`=81, last log epoch=<not checked in same read>
- Witness: endpoint=81, `grep -c "Epoch complete"`=81, last log epoch=<not checked>

Morning-api shows a 1-off between endpoint and log count. Likely a race at epoch boundary (epoch 82 just ticked, 81 completed are in the log). No deviation implied.

---

## 4. Persistence State (GetPersistenceState)

### morning-api

- **OBSERVED:** last_snapshot_epoch=80, wal_bytes=1248, wal_entries=10
- **DISK:** wal.log=1248 bytes → **MATCHES** (simultaneous read verified)
- **EXPECTED:** wal_bytes matches file size on disk. Snapshot at epoch 80 (every 10 epochs from genesis: 0, 10, 20, ..., 80). wal_entries based on size/120 heuristic (=10.4 → 10).
- **DEVIATION:** None. Byte-equality verified at same timestamp.

### local-witness

- **OBSERVED:** last_snapshot_epoch=80, wal_bytes=379, wal_entries=3
- **DISK:** wal.log=379 bytes → **MATCHES** (simultaneous read verified)
- **EXPECTED:** Same snapshot epoch as morning-api. wal_bytes matches file size.
- **DEVIATION:** None. Byte-equality verified.

**Notable:** Both nodes show `wal.wal.old` naming (instead of `wal.log.old`). Known-provisional from VERIFIED-BEHAVIOR.md. Persistent across this deployment.

---

## 5. Economic State (GetEconomicState)

### morning-api reports

- **OBSERVED:** own_balance=92, own_nonce=202, sees witness balance=9908, nonce=0
- **OBSERVED total supply (morning-api's view):** 92 + 9908 = 10,000

### local-witness reports

- **OBSERVED:** own_balance=0, own_nonce=4, sees morning-api balance=0, nonce=0
- **OBSERVED total supply (witness's view):** 0 + 0 = 0

### Supply divergence

- **EXPECTED:** No documented invariant for supply conservation exists in VERIFIED-BEHAVIOR.md (the candidate invariant was proposed but not ratified). For a 2-node mesh with `--mint 5000` on morning-api and `--mint 0` on witness, the expected behavior is NOT formally defined. However:
  - **Morning-api** started at balance 5000 and has been taxed across 82 epochs (mint=0, redistribution occurs). Own balance drops over time. This is expected tax behavior.
  - **Witness** started at balance 0 and receives redistribution from morning-api's tax (that's what `redistributed_to=1` means — likely the witness as sole peer).
  - The witness reports own_balance=0, but morning-api credits witness with 9908. This is the **same supply conservation contradiction** from Jul 27's Verifier Mission 1.

- **DEVIATION:** CONFIRMED PERSISTENT from Jul 27 finding (Status: CONTRADICTED). Morning-api and witness have divergent views of total supply (10,000 vs 0). The two ledgers do not agree on who has what.
- **FIRST OBSERVED:** Jul 27, 18:48 EDT (Observer pass #3). Confirmed still present in this pass (Jul 28, 13:42 UTC).
- **CHANGE SINCE LAST OBSERVED:** This is the first Observer pass in this session. Not comparable to earlier Jul 28 passes without reading their evidence files.

### Nonce divergence

- **OBSERVED:** morning-api nonce=202, witness nonce=4. Morning-api credits witness nonce=0; witness credits morning-api nonce=0 (both see the other's nonce as 0).
- **EXPECTED:** Nonces should increase with transaction count. Cross-node nonce synchronization is not documented as an invariant.
- **DEVIATION:** UNKNOWN. The 198-nonce gap (202 vs 4) and the mutual zero peer-nonce views suggest the nodes' transaction histories diverged. Morning-api processed 202 transactions (including the initial mint nonces?); witness processed 4.

---

## 6. Metrics / Heartbeat Health

**Morning-api metrics tick (most recent):**
- outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=6s
- **OBSERVED:** 247 metrics ticks since startup. All healthy indicators.

**Witness metrics tick (most recent):**
- outstanding_fetches=0, aged=0, outbound_queues=[], max_peer_silence=3s
- **OBSERVED:** 245 metrics ticks. All healthy indicators.

**DEVIATION:** None. No stale fetches, no queue buildup, peer silence within normal range (<10s).

---

## 7. Kademlia Warnings

**OBSERVED:** Multiple `Failed to trigger bootstrap: No known peers.` WARN messages on morning-api, spaced every 5 minutes (13:01, 13:06, 13:11, ... 13:41).

**EXPECTED:** On a 2-node mesh with `--no-mdns` and no Kademlia DHT, bootstrap will always fail. These are known harmless warnings.

**DEVIATION:** None. Expected noise on this topology pattern.

---

## 8. Build Commit Staleness

**OBSERVED:**
- Running binary: `cb5d4b1-dirty` (both nodes)
- Git HEAD: `452b64f` ("docs: wal_bytes fix verified, wal_entries heuristic noted")
- Binary is 1 commit behind HEAD

**EXPECTED:** `build_commit` should match `git rev-parse HEAD` for confidence in binary identity.

**DEVIATION:** Running binary was built at cb5d4b1, HEAD is 452b64f. The diff is docs-only (no code changes), so behavior is likely identical. However, the `-dirty` suffix confirms uncommitted changes were present at build time — the exact working tree state at build is not reproducible from git history.

**UNKNOWN:** Whether the `-dirty` state included source code changes that affect behavior, or only untracked files (evidence docs, helper scripts).

---

## Summary of Findings

| # | Metric | Status | Details |
|---|--------|--------|---------|
| 1 | Socket reachable | OK | Both sockets responding |
| 2 | Peer connectivity | OK | 1 peer each, silence <10s |
| 3 | Metrics health | OK | aged=0, queues empty, no stale fetches |
| 4 | WAL byte-equality | OK | Both nodes: endpoint matches file size |
| 5 | Supply conservation | **CONTRADICTED** (persistent) | Same divergence pattern as Jul 27 |
| 6 | Build commit | **STALE** | 1 commit behind HEAD, -dirty suffix |
| 7 | Epoch progression | OK | Within 1 epoch of each other |
| 8 | Kademlia WARN | OK (expected noise) | Harmless on --no-mdns topology |
| 9 | ratio divergence | UNKNOWN | 1.016 vs 1.93 — witness ratio formula may diverge at zero balance |

**PERSISTENT DEVIATIONS (unchanged since Jul 27):**
- Supply conservation contradiction (findings #5 from VERIFIED-BEHAVIOR.md)
- `wal.wal.old` naming (known-provisional)
- Kademlia bootstrap warnings (expected)

**NEW IN THIS PASS:**
- Build commit `cb5d4b1-dirty` vs HEAD `452b64f` — stale binary
- `thickness` field present on morning-api but absent on witness response
- Ratio divergence: morning-api=1.016, witness=1.93

**STATUS:** Mesh is operational. Gossip flows, metrics are clean, epochs cycle, persistence is byte-accurate. The known supply contradiction persists without change. No new runtime anomalies beyond the stale binary.
