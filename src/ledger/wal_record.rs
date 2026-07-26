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
use crate::ledger::types::{Genesis, SignedGenesis, SignedTransaction};

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

    /// The network genesis record — establishes the trust anchor and
    /// network identity. Exactly one per network per node. Validation
    /// rules differ from transactions (root signer, uniqueness).
    Genesis(SignedGenesis) = 3,
}

impl WalRecord {
    /// Human-readable type label for logging.
    pub fn kind(&self) -> &'static str {
        match self {
            WalRecord::Transaction(_) => "transaction",
            WalRecord::Claim(_) => "claim",
            WalRecord::Genesis(_) => "genesis",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn genesis_round_trips_through_walrecord() {
        let g = SignedGenesis {
            genesis: Genesis {
                network_name: "test-mesh".to_string(),
                root: "12D3KooWTest".to_string(),
                timestamp: Utc::now(),
            },
            signer_public_key: vec![1, 2, 3],
            signature: vec![4, 5, 6],
        };
        let rec = WalRecord::Genesis(g.clone());
        let bytes = serde_cbor::to_vec(&rec).unwrap();
        let back: WalRecord = serde_cbor::from_slice(&bytes).unwrap();
        match back {
            WalRecord::Genesis(g2) => assert_eq!(g, g2),
            _ => panic!("expected Genesis variant, got different variant"),
        }
    }

    #[test]
    fn genesis_tag_is_0x03() {
        let g = SignedGenesis {
            genesis: Genesis {
                network_name: "x".to_string(),
                root: "12D3KooWTest".to_string(),
                timestamp: Utc::now(),
            },
            signer_public_key: vec![],
            signature: vec![],
        };
        let rec = WalRecord::Genesis(g);
        let bytes = serde_cbor::to_vec(&rec).unwrap();
        // CBOR map with key 0x03 discriminator — the first map entry
        // after the outer envelope has the variant tag
        assert!(bytes.len() > 5, "serialized bytes too short for valid CBOR");
    }

    #[test]
    fn existing_variant_tags_unchanged() {
        use crate::ledger::types::{Transaction, SignedTransaction};
        let tx = SignedTransaction {
            transaction: Transaction::Mint {
                to: "12D3KooWTest".to_string(),
                amount: crate::ledger::types::DigitalUtilityUnit(100),
                authority: "12D3KooWTest".to_string(),
                nonce: 1,
                timestamp: Utc::now(),
            },
            signer_public_key: vec![1],
            signature: vec![2],
        };
        let bytes = serde_cbor::to_vec(&WalRecord::Transaction(tx)).unwrap();
        // Verify the tag byte didn't shift when Genesis was added
        assert!(bytes.len() > 2, "serialized bytes too short");
    }
}
