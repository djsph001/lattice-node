# Changes from Sandbox to Production — Status Report

This document tracks every design decision and protocol change that was discussed or designed in the sandbox, and whether it has been implemented in the running codebase or remains unbuilt.

## Legend

- **✅ Landed** — code is committed, tests pass, live on test mesh
- **⚠️ Designed, Not Built** — specification exists, no implementation
- **❌ Not Started** — identified as needed, no design yet

---

## 1. Fork Resolution Protocol

### 1.1 Fork Detection (`detect_fork`)
**Status: ✅ Landed (49a5835)**

Sequential block-hash comparison between local and remote ledger files. Handles equal-height, longer-local, longer-remote, and no-divergence cases. Returns `CertificateFork` struct with `fork_point`, `local_fork`, and `remote_fork` segments.

### 1.2 Thickness-Weighted Scoring (`calculate_fork_score`)
**Status: ✅ Landed (49a5835)**

f64 sum of thickness for unique witness PeerIds in a fork segment. Accepts a closure for thickness lookup, keeping the economic layer decoupled from the ledger.

### 1.3 Fork Resolution (`resolve_fork`)
**Status: ✅ Landed (49a5835)**

Thickness-weighted with two tiebreaker levels (longer chain, then deterministic fork hash). Empty-fork edge cases handled (both empty → NoFork; one empty → other wins).

### 1.4 Ledger Adoption (`adopt_winning_fork`)
**Status: ✅ Landed (49a5835)**

Copies common prefix + winning blocks to temp file, atomic rename, updates in-memory `block_height` and `last_block_hash`.

### 1.5 Committed Set Cleanup (`remove_committed_proposals`)
**Status: ✅ Landed (c6c073c)**

Clears losing-fork proposal IDs from the in-memory `committed` HashSet after adoption. Without this, re-submitted proposals hit the dedup guard and are rejected.

### 1.6 On-Disk Format Change (PeerId in Signature Frames)
**Status: ✅ Landed (49a5835)**

Every signature entry in the ledger now stores `[peer_id_len: u16] [peer_id_bytes] [sig_len: u16] [sig_bytes]` instead of just `[sig_len] [sig_bytes]`. All readers (`scan_to_tip`, `get_block_bytes`, `is_bootstrap_ended`) updated to account for the PeerId field.

**Breaking change.** Old-format ledgers cannot be read by new code. Existing test mesh data was wiped.

**Rationale:** The alternative was maintaining an external signature→PeerId mapping for the scoring function. That mapping doesn't exist and would require a separate protocol to populate. By embedding PeerId in the signature frame, `calculate_fork_score` is self-contained — the ledger carries its own witness identities.

### 1.7 Gossip Sync Handler (`handle_remote_ledger`)
**Status: ⚠️ Designed, Not Built**

Full handler design exists in `src/ledger/gossip_sync_handler.md`. Flow:

```
Peer connects → exchange (height, tip_hash)
  → if tips differ, fetch remote ledger → file on disk
  → detect_fork(local, remote)
  → resolve_fork(fork, thickness_fn)
  → adopt_winning_fork(winner, fork_point)  // if local lost
  → remove_committed_proposals(losing fork)
  → extract_proposal_ids(losing fork)       // for re-submission
```

**Why not implemented:** Not needed until partition healing is tested. The current mesh runs without partitions, so the handler is never triggered.

**Scope of this handler:**
- **Sync-on-join** — a new or recovering peer catches up to the canonical chain by fetching the longer ledger from a connected peer.
- **Drift reconciliation** — two already-connected peers detect divergent tips (same height, different hashes), fetch the full ledger, and converge via fork resolution.
Both triggers map to the same handler: fetch full ledger, detect fork, resolve. One scope, two triggers. Full-file transfer is the alpha simplification; chunked sync is deferred but the handler interface abstracts the transport.

---

## 2. Persistence Layer

### 2.1 WAL + Snapshot Architecture (`WalStateStore`)
**Status: ✅ Landed (9efdfdd, bbd44fd) — Provisional Format**

Write-Ahead Log appends every validated transaction in serde_cbor format. Batched fsync (every 100 txs or 100ms, first tx always flushes). Periodic snapshots of `seen_nonces` (thickness graph snapshot is deferred).

**Note on format stability:** The on-disk ledger format (PeerId-in-sig frames) is provisional. The Era Two decision (economics-on-chain) may change how state is persisted. The `StateStore` trait is designed to absorb this — a future `ChainStateStore` implementation replaces WAL replay with chain replay, and the current format is swapped out without touching any callers.

### 2.2 `StateStore` Trait
**Status: ✅ Landed (9efdfdd)**

`StateStore` trait with `persist`, `recover`, `take_snapshot`. Current implementation is `WalStateStore`. Future implementation will be `ChainStateStore` that replays from the certificate chain (no WAL needed once economics-on-chain lands).

### 2.3 CLI `--persistence` Flag
**Status: ✅ Landed (bbd44fd)**

Enables persistence from the command line. Stores state in `<storage-dir>/persistence/`. Calls `enable_persistence()` on startup which recovers `seen_nonces` from snapshot + WAL replay.

### 2.4 Local Apply Paths Persist to WAL
**Status: ✅ Landed (bbd44fd)**

`submit_genesis` and `submit_bootstrap_ended` now wrap the raw `Transaction` in a `SignedTransaction` and call `on_transaction_applied()` so the WAL records era-one transactions for crash recovery.

### 2.5 Live-Node Recovery Test
**Status: ✅ Verified on Live Mesh**

```
Recovered ledger tip ... height=1
Replayed transactions from WAL replayed=1
Recovered 1 peer nonces from persistence count=1
```

Node survived kill-and-restart with full state recovery.

### 2.6 Periodic Snapshot Timer
**Status: ⚠️ Designed, Not Built**

The `take_snapshot` method exists on `WalStateStore` but nothing calls it periodically. The event loop needs a timer (every ~1000 transactions or 5 minutes) that calls `take_snapshot`.

**Workaround:** Manual snapshot can be triggered via API. Without the timer, the WAL grows unbounded.

### 2.7 Graceful Shutdown Flush
**Status: ⚠️ Designed, Not Built**

On `SIGTERM`, the node should flush the WAL buffer and take a final snapshot. Currently, a kill -9 loses any transactions in the WAL buffer (though the last fsync-point is safe).

### 2.8 ThicknessGraph Snapshot Serialization
**Status: ⚠️ Designed, Not Built**

`export_edges()` and `import_edges()` methods exist on `ThicknessGraph` but the import path is a stub (logs a warning and skips). The snapshot currently only saves `seen_nonces`. Full thickness graph recovery requires WAL replay — the snapshot is a performance optimization for faster startup.

### 2.9 Epoch Mint/Redistribution Persistence
**Status: ⚠️ Designed, Not Built**

Epoch mint and redistribution transactions are created, signed, and gossiped via the outbound queue, but they are NOT persisted to the WAL. Only genesis, bootstrap_ended, and gossip-arrived transactions are recorded. If the node restarts during an epoch cycle, the current epoch's mint/redistribution may be lost (though the receipt chain persists the blocks).

---

## 5. Operational Safeguards

### 5.1 Startup NTP Clock Drift Check
**Status: ✅ Landed (e4aec30)**

On startup, the node queries NTP servers in order (`time.apple.com`, `time.google.com`, `pool.ntp.org`), compares the response against the local wall clock, and hard-fails with an actionable error if drift > 300s. Prevents operators from broadcasting bad-timestamped transactions (the 15-hour Mac drift failure revealed by the soak).

- `--skip-ntp-check` flag bypasses the check (air-gapped/lab networks, logged at WARN).
- `--ntp-server` flag for custom NTP servers.
- Uses `rsntp` crate with `SntpClient` in a `spawn_blocking` call (sync protocol is fine for a one-time startup check).

Validated live: Florida booted with `drift 0s against pool.ntp.org`. Mac pending pull.

---

## 3. Fetch Protocol (Receiver-Side Gap Recovery)

### 3.1 `ValidationError::GappedNonce`
**Status: ✅ Landed (2864e54)**

Structured error returned by `check_nonce` when `incoming_nonce != last_nonce + 1`. Carries `signer`, `expected`, and `got` fields. Replaced the old `bail!()` stringly-typed error.

### 3.2 Transaction Request/Response Types
**Status: ✅ Landed (ebb8961)**

`TransactionRequest { signer, from_nonce, to_nonce }` and `TransactionResponse { transactions }` defined alongside existing `BalanceCodec`, `VerifyCodec`, etc.

### 3.3 tx_rpc Behaviour + Handler
**Status: ✅ Landed (ebb8961)**

Fifth instance of the request-response pattern. Behaviour wired into `NetworkBehaviour`, event variant in `LatticeBehaviourEvent`, handler in the event loop.

### 3.4 tx_store + Pending Queue
**Status: ✅ Landed (f88f662)**

`tx_store: HashMap<(PeerId, u64), SignedTransaction>` populated on every successful apply. `pending: HashMap<PeerId, BTreeMap<u64, SignedTransaction>>` holds gapped transactions until the missing predecessor arrives.

### 3.5 Fetch Trigger on GappedNonce
**Status: ✅ Landed (f88f662)**

In the gossip handler's `Err(ValidationError::GappedNonce { .. })` arm: parks the transaction in `pending`, checks `outstanding_fetches` for dedup, sends `TransactionRequest` to the propagation source.

### 3.6 `outstanding_fetches` Dedup
**Status: ✅ Landed (e0b4fa3)**

`HashMap<(PeerId, u64), Instant>` prevents request storms from repeated gapped arrivals for the same hole. Lazy timeout eviction (sweep expired entries for that signer on next insert attempt).

### 3.7 Stale/Replay Guard
**Status: ✅ Landed (94ecc04)**

If `got < expected` (transaction is behind current prefix), do not fetch, do not park in pending. Stale transactions arrive via normal gossip replay, not fetch.

### 3.8 Fire Test + Trust Test
**Status: ✅ Landed (6769f21, 74fc90a)**

Five tests covering the state machine: contiguous, ahead (gap), behind (stale), replay (same nonce), and trust (cap-violating vouch rejected on full validation path).

---

## 4. Sender Retry Queue

### 4.1 Outbound Queue
**Status: ✅ Landed (ec28002)**

`outbound: HashMap<PeerId, BTreeMap<u64, SignedTransaction>>`. Transactions inserted on sign, never removed on broadcast result. `flush_outbound()` broadcasts the lowest nonce.

### 4.2 Gossip Echo Removal
**Status: ✅ Landed (ec28002)**

When `signer == self.local_peer_id && propagation_source != self.local_peer_id`, the transaction is removed from the outbound queue (a peer other than us forwarded it → it's in the mesh).

### 4.3 Self-Echo Guard
**Status: ✅ Landed (ec28002)**

The `propagation_source != self` check is the discriminator. A self-echo (published and immediately received back via gossipsub) does NOT remove the queue entry.

### 4.4 Drain-on-Broadcast
**Status: ✅ Landed (ec28002)**

Every broadcast flushes the queue's lowest nonce before appending a new transaction. Maintains correct nonce ordering.

---

## Summary

| Area | Built | Designed, Not Built | Not Started |
|:---|:---:|:---:|:---:|
| Fork Resolution Core | 7/7 | — | — |
| Gossip Sync Integration | — | 1 | — |
| Persistence Core | 5/5 | — | — |
| Snapshot Timer | — | 1 | — |
| Graceful Shutdown | — | 1 | — |
| Snapshot Import | — | 1 | — |
| Epoch Persistence | — | 1 | — |
| Fetch Protocol | 8/8 | — | — |
| Sender Retry | 4/4 | — | — |

All protocol-critical items are built and tested. The remaining unbuilt items are operational refinements (snapshot timer, graceful shutdown, epoch persistence) and the gossip sync handler (which only matters when partitions are tested).

### Live Test Mesh

Three nodes running:

| Node | Location | Role | Status |
|:---|:---|:---|:---:|
| z4-node | Boynton Beach, FL (HP Z4) | Full node, persistence enabled | ✅ Live |
| mac-node | Germany (Mac Mini) | Full node, persistence status unconfirmed | ✅ Live |
| relay-hub | Hetzner (VPS) | Relay + bootstrap | ✅ Live |

All three are peered and exchanging heartbeats. The soak test is running at "hasn't crashed" fidelity — no instrumentation for slow-leak metrics (outstanding_fetches growth, outbound queue depth, memory trend). Adding those metrics is the next step before the soak result is meaningful beyond uptime.

**Asymmetry note:** Florida was started with `--persistence` and recovery was verified (replayed=1). Germany's persistence status is unconfirmed — the Mac Mini was set up during the session but the `--persistence` flag may not have been included. If Germany lacks persistence, it will lose its accumulated nonces on restart (though it will recover from gossip). This is harmless for the soak but should be resolved before the soak is considered production-grade.
