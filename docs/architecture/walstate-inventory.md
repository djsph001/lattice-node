# WalStateStore Inventory — Durability Semantics & Migration Prerequisites

**Date:** 2026-07-29
**Status:** Investigation — before replacing WalStateStore with FileWal
**Scope:** End-to-end trace of existing persistence authority

---

## 1. Write Paths

| **Record Type** | **Write Call Site** | **File** | **fsync?** | **Ordering** |
|-----------------|---------------------|----------|------------|--------------|
| SignedTransaction | `on_transaction_applied()` (node.rs:4054) | `transactions.wal` | Yes (batch_size=1, immediate) | **POST-apply** — written after graph/ledger application |
| WitnessedClaim | `finalize_claim()` (node.rs:2671) | `claims.wal` | Yes (sync_all after write) | **PRE-queue** — written before economic engine queues the claim |

**Critical finding:** Transaction persistence happens AFTER application. The design doc
(04709ee §2) specifies append-before-apply. The production code reverses this:
`graph.apply()` → `store.persist()`. A crash between apply and persist leaves the
transaction in graph state but absent from the WAL — unrecoverable.

Claims are correctly ordered: `persist_claim()` → `queue_claim()`.

Both WALs use `fsync` at the configured `fsync_batch_size` (production: 1, immediate).
The durability boundary EXISTS, but it's placed after the economic effect for
transactions.

---

## 2. Recovery Path

Recovery happens in `enable_persistence()` (node.rs:2843–2940), called during node
startup when `--persistence` is passed. The sequence:

| **Step** | **Source** | **State Reconstructed** |
|----------|------------|------------------------|
| Load snapshot | `state.snapshot` (CBOR) | Nonces, balances, thickness_edges, accepted_claims, self_tx_nonce |
| Replay transactions.wal | Length-prefixed CBOR entries | Advances nonces and balances per Transaction::Mint / Transfer |
| Replay claims.wal | Length-prefixed CBOR entries | Adds claims to accepted_claims (dedup against snapshot keys) |
| Rebuild last_claimed | From accepted_claims | Claim-window high-water mark per (peer, claim_type) |
| Consistency check | verify_consistency() | Snapshot+WAL replay vs WAL-only replay must match |

The old WALs ARE authoritative — the node depends on them for state reconstruction
after restart. Recovery without them would lose:

- All post-snapshot transaction history (nonces, balances)
- All post-snapshot claim history (accepted_claims)
- Thickness edges (snapshot-only; not WAL-logged)
- last_claimed (derived from accepted_claims)

**Snapshot dependency:** Thickness edges are NOT in either WAL — they exist only in
the snapshot. Recovery without the snapshot loses all thickness state. The WALs alone
are insufficient for full recovery. Claims.wal records are deduplicated against the
snapshot's accepted_claims set — the snapshot is the primary authority, the WALs are
supplementary.

---

## 3. Durability Semantics

| **Property** | **transactions.wal** | **claims.wal** |
|-------------|---------------------|----------------|
| Write order | Post-apply | Pre-queue |
| fsync guarantee | Yes (batch_size=1 in production) | Yes (sync_all per claim) |
| Crash-safe ordering | **No** — gap between apply and persist | Yes |
| Authoritative on recovery | Yes (advances snapshot state) | Yes (adds to snapshot state) |
| Denominated | SignedTransaction CBOR, length-prefixed | WitnessedClaim CBOR, length-prefixed |
| Rotation | Snapshot-anchored (fa50952) | Snapshot-anchored (fa50952) |

---

## 4. FileWal vs WalStateStore — Gap Analysis

| **Capability** | **WalStateStore (existing)** | **FileWal (new)** | **Delta** |
|----------------|------------------------------|-------------------|-----------|
| SignedTransaction persistence | Yes | Yes | — |
| WitnessedClaim persistence | Yes (claims.wal) | **No** | Must add |
| Correct durability ordering | No (transactions post-apply) | Yes (pre-apply) | Must fix in integration |
| Atomic append per entry | No (blob-based) | Yes (temp+rename) | Improvement |
| Tail integrity | Parsing stops at partial entry | Individual corrupt entries skipped | FileWal more robust |
| Per-peer directory layout | No (single blob) | Yes | Different model |
| Snapshot integration | Yes (PersistentEconomicState) | No | Current design block |
| Consistency verification | Yes (verify_consistency) | No | Must implement |
| last_nonce query | No (derived from blob) | Yes (directory scan) | Improvement |

---

## 5. Migration Recommendation

**Category A — authoritative, must preserve.** The old WALs are authoritative for
recovery. The node depends on them. Deleting without migration loses state.

**Recommended migration sequence:**

1. Extend FileWal to accept `WalRecord { Transaction(SignedTransaction), Claim(WitnessedClaim) }`
2. Implement consistency verification on FileWal (replay-only vs snapshot+replay)
3. Write a migration that reads old WALs and rewrites to new format
4. Prove old/new recovery equivalence
5. Switch write paths to FileWal
6. Switch recovery path to FileWal
7. Remove WalStateStore

**Migration scope:** The snapshot (`state.snapshot`) is the primary authority — it
contains thickness, accepted_claims, nonces, and balances at each checkpoint. The
WALs carry the delta between checkpoints. Migration must preserve both the snapshot
AND the WALs' ability to replay the inter-checkpoint window.

**Conversions required:**
- Old `transactions.wal` (length-prefixed SignedTransaction CBOR) → FileWal `<peer>/<nonce>.cbor`
- Old `claims.wal` (length-prefixed WitnessedClaim CBOR) → FileWal `WalRecord::Claim`
- Snapshot unchanged (format is independent of WAL implementation)

---

## 6. Open Questions

| **Question** | **Status** |
|--------------|------------|
| Do claims share per-signer nonce semantics with transactions? | Needs protocol confirmation |
| Should FileWal use single append-only file or per-peer directories for unified records? | Design decision needed |
| Does the old WAL truncation (snapshot-anchored rotation) need porting to FileWal? | Yes, after migration |
