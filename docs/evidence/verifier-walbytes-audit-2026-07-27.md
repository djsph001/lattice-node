# Verifier WAL Bytes Audit — Mission 2
**Date:** 2026-07-27
**Provenance:** Direct source inspection (not delegate — mission interrupted, traced manually)

## BOUNDED QUESTION
What does GetPersistenceState.wal_bytes actually represent, and is its current value (0) consistent with the implemented semantics?

## SOURCE

`src/ledger/persistence.rs:696-706`:
```rust
fn get_stats(&self) -> (u64, u64, u64) {
    let size = std::fs::metadata(&self.wal_path)
        .map(|m| m.len())
        .unwrap_or(0);
    let est_entries = if size > 0 {
        (size / 120).max(1)
    } else {
        0
    };
    (self.last_snapshot_epoch, size, est_entries)
}
```

## WHERE wal_path POINTS

Line 207: `let wal_path = config.data_dir.join("transactions.wal");` — **legacy WAL path, retired in unified WAL migration.**

Line 209: `let unified_wal_path = config.data_dir.join("wal.log");` — **current WAL path, never referenced by get_stats.**

## CLASSIFICATIONS

### OBSERVED
- Both nodes: GetPersistenceState returns wal_bytes=0, wal_entries=0, last_snapshot_epoch=<current>
- Both nodes: wal.log exists on disk at 379 bytes (genesis re-seed after rotation)
- Both nodes: transactions.wal does NOT exist on disk (retired in unified WAL migration)

### VERIFIED
- `get_stats` reads `self.wal_path` → `transactions.wal`
- `transactions.wal` does not exist → `metadata()` fails → `unwrap_or(0)` returns 0
- `last_snapshot_epoch` is correctly reported (reads from `self.last_snapshot_epoch` field)
- The field IS wired — it calls `metadata().len()` on a real path
- The logic is correct — the path is wrong

### CONTRADICTED
- None. No contradiction between implementation and behavior. The implementation explains the behavior exactly.

### EVIDENCE GAP
- None. Root cause established.

## CONCLUSION

**The field reads the wrong WAL file.** `get_stats` uses `self.wal_path` (legacy `transactions.wal`, retired in unified migration) instead of `self.unified_wal_path` (`wal.log`, current). The function works correctly — `metadata().len()` returns the file size — but the file it reads no longer exists after the unified WAL migration.

Fix: change `self.wal_path` to `self.unified_wal_path` in get_stats(). One line.
