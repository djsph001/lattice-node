// ── Persistence module ────────────────────────────────────────
//
// Design: docs/architecture/persistence-design.md at 04709ee.
//
// The WAL is the source of truth for transaction history.  The
// graph is derived state — always rebuildable from the WAL.
// A transaction is considered durable only after `fsync`.
//
// See `wal.rs` for the trait and file-backed implementation.

pub mod wal;
