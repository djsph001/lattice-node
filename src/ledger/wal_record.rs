// ── Unified WAL record type ─────────────────────────────────
//
// WalRecord unifies transactions and claims under one durable
// authority.  The two record types retain distinct protocol
// semantics (identity, ordering, validation) — only the
// persistence mechanism is shared.
//
// Tag values are explicit `u8` discriminants for CBOR.  Numeric
// tags survive variant renames (e.g. Transaction → Tx), which
// string-tagged serde defaults would not.  Same discipline as
// the Era Two 0x02 block marker: explicit beats implicit.
//
// Group-commit (batch writes, single fsync) is a future
// optimisation if throughput becomes a constraint.  Under
// Version B it is safe only if apply also batches: don't apply
// any of the N until all N are fsynced.  For now, independent
// fsync per write is the correct design.

use serde::{Deserialize, Serialize};

use crate::claims::WitnessedClaim;
use crate::ledger::types::SignedTransaction;

/// A durable protocol record — either a transaction or a claim.
///
/// Numeric CBOR discriminants: Transaction = 1, Claim = 2.
/// New variants MUST use unique tags that have never been
/// emitted to production WAL files.  Retired tags remain
/// reserved to prevent silent reinterpretation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(u8)]
pub enum WalRecord {
    /// A signed protocol transaction (Mint, Transfer, Vouch, …).
    /// Validated at write time; replay trusts without re-validation.
    Transaction(SignedTransaction) = 1,

    /// A witnessed claim about work performed.
    /// Claims have their own identity, ordering, and lifecycle.
    Claim(WitnessedClaim) = 2,
}

impl WalRecord {
    /// Human-readable type label for logging.
    pub fn kind(&self) -> &'static str {
        match self {
            WalRecord::Transaction(_) => "transaction",
            WalRecord::Claim(_) => "claim",
        }
    }
}
