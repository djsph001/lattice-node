# Persistence — Durability Boundary & Recovery Invariant

**Date:** 2026-07-29
**Status:** Design — investigation and design only, no implementation
**Scope:** Establish the invariant before writing persistence code

---

## 1. Transaction Lifecycle (Current State)

The transaction pipeline as it exists today:

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                              TRANSACTION LIFECYCLE                                 │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│  ┌──────────┐    ┌───────────┐    ┌───────────┐    ┌──────────┐    ┌──────────┐   │
│  │ RECEIVED │───▶│ VALIDATED │───▶│  APPLIED  │───▶│  GRAPH   │───▶│  STORED  │   │
│  │  (wire)  │    │ (nonce,   │    │  (memory) │    │  UPDATE  │    │   (none) │   │
│  └──────────┘    │  sig,     │    └───────────┘    └──────────┘    └──────────┘   │
│                  │  bounds)  │                                                      │
│                  └───────────┘                                                      │
│                                                                                     │
│  ────────────────────────────────────────────────────────────────────────────────── │
│  VOLATILE ───────────────────────────────────────────────────────────────────────── │
│                                                                                     │
│  Recovery boundary:                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────┐   │
│  │  No durable storage exists. On restart: entire graph is reconstructed from  │   │
│  │  genesis function + in-memory state only. No transaction history persists.  │   │
│  └─────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

**Key observation:** The transaction pipeline has no durability boundary.
"Applied" means "in-memory graph updated." There is no `fsync`, no WAL, no
recovery path.

---

## 2. Durability Boundary (Proposed)

The WAL will introduce a crisp boundary:

```
received → validated → WAL append → fsync → durable → apply
                              ↑
                         The claim is made here
```

**Definition:**

| **State**  | **Meaning** |
|------------|-------------|
| Volatile   | Transaction exists only in process memory. A crash destroys it. |
| Durable    | Transaction has been written to the WAL and `fsync` has returned successfully. A crash does not destroy it; it is recoverable. |
| Applied    | Transaction has been applied to the in-memory graph. The graph is derived state, not primary storage. |

**Failure semantics:**

| **Phase** | **If crash occurs...** |
|-----------|------------------------|
| Before WAL append | Transaction is lost. Valid: it was never claimed as durable. |
| During WAL append (partial write) | Transaction may be partially written. On recovery, incomplete entry is detected and discarded. Valid: it was never `fsync`'d. |
| After write, before `fsync` | Transaction is in OS buffer, not on disk. Crash loses it. Valid: it was never claimed as durable. |
| After `fsync`, before apply | Transaction is durable but not applied. On recovery, it is replayed. Valid: durable entry exists. |
| During apply | Transaction is durable; graph may be partially applied. On recovery, replay completes the application. Valid: application is idempotent. |
| After apply | Transaction is durable and applied. Fully recovered. |

**The invariant:**

> A transaction is considered *accepted* for durability purposes only after
> `fsync` returns successfully. Before that point, it may be discarded on
> crash with no loss of correctness.

---

## 3. Recovery Invariant

> **After restart, the graph must equal the state derived from replaying all
> transactions that were durably committed to the WAL, in valid causal order,
> and no transaction that was not durably committed may affect recovered state.**

**Ordering constraints:**

- Transactions are ordered per signer by nonce (guaranteed by protocol)
- No global total order is guaranteed
- Replay must respect per-signer nonce order
- Cross-signer interleaving is preserved from the WAL sequence

**Implementation implications:**

- The WAL stores transactions in the order they were applied
- Replay applies them in the same order
- Per-signer nonce monotonicity is enforced during validation, not during replay

**Recovery procedure:**

```
On restart:
  1. Open WAL
  2. Read all transactions from the beginning
  3. For each transaction:
     a. Validate per-signer nonce monotonicity (skip any out-of-order)
     b. Apply to graph in WAL order
  4. Graph now matches the state at crash time (minus any volatile transactions)
```

---

## 4. Crash Matrix

| **Crash Point** | **WAL State** | **Graph State** | **Recovery Outcome** | **Validity** |
|-----------------|---------------|-----------------|----------------------|--------------|
| Before WAL write | Absent | Pre-apply | Graph unchanged | ✅ Correct |
| During WAL write | Partial/incomplete | Pre-apply | Incomplete entry discarded; graph unchanged | ✅ Correct |
| After write, before `fsync` | Buffered, not durable | Pre-apply | Buffer lost; graph unchanged | ✅ Correct |
| After `fsync`, before apply | Present, durable | Pre-apply | Replay applies tx | ✅ Correct |
| During graph apply | Present, durable | Partially applied | Replay completes apply (idempotent) | ✅ Correct |
| After graph apply | Present, durable | Post-apply | Replay re-applies (idempotent) | ✅ Correct |

**No dangerous middle state:** no transaction appears committed but cannot be
reconstructed.

---

## 5. Two Guarantees the Implementation Must Prove

The crash matrix is a design hypothesis until the implementation and filesystem
semantics prove it. Two specific guarantees must be demonstrated:

### 5.1 WAL Tail Integrity

"During WAL write → incomplete entry discarded" is not automatic. Depending on
the record format and append strategy, recovery may encounter a partial final
record. The WAL must use a self-delimiting record format with integrity
checking — typically a length-prefixed header followed by the record payload,
with a checksum. An incomplete or invalid tail record is detected and safely
discarded.

**Recovery rule:** An incomplete final entry (truncated header, partial
payload, checksum mismatch) is discarded. Recovery proceeds with all complete
entries up to that point. The incomplete entry's transaction never reached
`fsync`, so discarding it is correct.

### 5.2 Replay Idempotence

"During graph apply → recovered on replay" depends on graph application being
idempotent or nonce-guarded. Replaying a durably committed transaction that had
already been partially or fully applied must not double-apply its effect.

**Recovery rule:** Graph application guards against double-application by
per-node nonce tracking. If the graph already reflects nonce N for a given
signer, replaying a transaction with that same nonce is a no-op.

### The Complete Chain

```
Durable history + recoverable records + deterministic replay + idempotent application
= Recoverable state
```

---

## 6. Design Decisions (Held for Implementation)

| **Decision** | **Rationale** |
|--------------|---------------|
| WAL is the source of truth | Graph is derived state; history is primary |
| `fsync` before "durable" | OS buffering is not durability |
| Append-only WAL | No in-place updates; history is immutable |
| Per-node WAL files | Simpler ordering; natural sharding |
| CBOR encoding | Matches existing transaction format |
| No snapshots until replay is proven | Don't optimize what isn't working |
| Self-delimiting records with integrity check | WAL tail integrity |
| Nonce-guarded idempotent replay | Double-application prevention |

---

## 7. Open Questions (Deferred to Implementation)

| **Question** | **Status** |
|--------------|------------|
| WAL file rotation? (single file vs. segmented) | Deferred |
| WAL entry format (header + transaction + checksum) | Deferred |
| Recovery performance for large WALs | Deferred (snapshots later) |
| WAL truncation policy | Deferred (after snapshots) |

---

## Summary

| **Section** | **Content** |
|-------------|-------------|
| Lifecycle | Mapped existing pipeline; identified no durability boundary |
| Durability boundary | `fsync` is the claim point; before = volatile, after = durable |
| Recovery invariant | Graph = replay of all durably committed transactions in valid order |
| Crash matrix | Every boundary resolves to a valid state; no dangerous middle state |
| Tail integrity | Self-delimiting records with integrity checks |
| Replay idempotence | Nonce-guarded; double-application prevented |
| Design decisions | WAL-first, append-only, per-node files, CBOR, no snapshots yet |
| Open questions | Deferred to implementation |

---

**This design commit establishes the invariant before any code is written.**
**The implementation will prove it.**
