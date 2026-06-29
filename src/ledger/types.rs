use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A digital utility unit — the economic primitive of the Lattice.
///
/// For Phase 4 this is a simple newtype over u64. The Georgist resource
/// accounting logic (how units are created, taxed, and redistributed)
/// arrives in Phase 5. Right now this gives us something to transact with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DigitalUtilityUnit(pub u64);

impl DigitalUtilityUnit {
    pub const ZERO: Self = Self(0);

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self)
    }

    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }
}

impl std::fmt::Display for DigitalUtilityUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Add for DigitalUtilityUnit {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::Sub for DigitalUtilityUnit {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

/// An economic transaction on the Lattice.
///
/// Two initial variants:
/// - `Transfer`: moves units between peers
/// - `Mint`: creates new units (test bootstrapping only; Phase 5 replaces
///   with Georgist issuance)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transaction {
    Transfer {
        /// Sender's PeerId (string form for CBOR serialization).
        from: String,
        /// Recipient's PeerId.
        to: String,
        /// Amount being transferred.
        amount: DigitalUtilityUnit,
        /// Monotonic nonce from the sender, preventing replay.
        nonce: u64,
        /// When the transaction was created.
        timestamp: DateTime<Utc>,
    },
    Mint {
        /// Recipient of newly-created units.
        to: String,
        /// Amount to mint.
        amount: DigitalUtilityUnit,
        /// Authority approving the mint (for Phase 4, the minter's PeerId).
        authority: String,
        /// Monotonic nonce.
        nonce: u64,
        /// When the mint was created.
        timestamp: DateTime<Utc>,
    },
}

impl Transaction {
    /// The PeerId of the actor initiating this transaction.
    pub fn signer(&self) -> &str {
        match self {
            Transaction::Transfer { from, .. } => from,
            Transaction::Mint { authority, .. } => authority,
        }
    }

    /// The nonce for replay protection.
    pub fn nonce(&self) -> u64 {
        match self {
            Transaction::Transfer { nonce, .. } => *nonce,
            Transaction::Mint { nonce, .. } => *nonce,
        }
    }

    /// Whether this is a mint operation (no sender balance check needed).
    pub fn is_mint(&self) -> bool {
        matches!(self, Transaction::Mint { .. })
    }
}

/// A transaction signed by its initiator.
///
/// The signature covers the CBOR-encoded `Transaction` bytes. The public
/// key is included so verifiers can check the signature without needing
/// a separate key registry — the same keypair that authenticates the
/// node's network presence authorizes its economic actions. One identity,
/// one reputation, one economic actor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedTransaction {
    /// The underlying transaction.
    pub transaction: Transaction,
    /// Protobuf-encoded Ed25519 public key bytes from the signer.
    /// Stored as raw bytes for CBOR serialization compatibility.
    pub signer_public_key: Vec<u8>,
    /// Ed25519 signature over the CBOR-encoded transaction bytes.
    pub signature: Vec<u8>,
}
