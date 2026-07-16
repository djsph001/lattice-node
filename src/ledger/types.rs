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

// ── Phase 6: resource claims ──────────────────────────────

/// A resource claim tracked in the local ledger.
///
/// When a peer asserts it stores a resource, the ledger creates a
/// claim.  Periodic storage challenges verify the claim.  Successful
/// verifications increase `tenure_health` toward 1.0; failures
/// degrade it.  If `tenure_health` drops below `EVICTION_HEALTH`,
/// the claim is dissolved.
#[derive(Debug, Clone)]
pub struct ResourceClaim {
    /// Blake3 hash of the full resource (the Merkle root).
    pub resource_id: [u8; 32],
    /// The peer claiming to store this resource.
    pub owner: String,
    /// Total size of the resource in bytes.
    pub size_bytes: u64,
    /// Number of 1 MiB chunks the resource spans.
    pub total_chunks: u64,
    /// The last epoch in which a storage challenge succeeded.
    pub last_successful_challenge: Option<u64>,
    /// How many consecutive challenges this claim has failed.
    /// Resets to 0 on any success.
    pub consecutive_failures: u32,
    /// Tenure health multiplier, bounded [0.0, 1.0].
    /// 1.0 = perfectly validated, 0.0 = dissolved.
    pub tenure_health: f64,
}

impl ResourceClaim {
    /// The floor below which a claim is automatically evicted.
    pub const EVICTION_HEALTH: f64 = 0.30;

    /// Create a new claim with perfect health.
    pub fn new(
        resource_id: [u8; 32],
        owner: String,
        size_bytes: u64,
        total_chunks: u64,
    ) -> Self {
        Self {
            resource_id,
            owner,
            size_bytes,
            total_chunks,
            last_successful_challenge: None,
            consecutive_failures: 0,
            tenure_health: 1.0,
        }
    }

    /// Whether this claim should be evicted.
    pub fn should_evict(&self) -> bool {
        self.tenure_health < Self::EVICTION_HEALTH
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
    /// A vouch stakes the voucher's thickness on a vouchee, granting them
    /// derived thickness. Part of the Sybil-resistance mechanism: stake
    /// scales with the voucher's capacity, and per-vouchee influence is
    /// inversely proportional to swarm size.
    ///
    /// stake_bps is integer basis points (0–10_000) — exact, order-independent.
    /// No f64 touches the vouch path.
    Vouch {
        /// The node staking their thickness (PeerId string form).
        voucher: String,
        /// The node receiving derived thickness (PeerId string form).
        vouchee: String,
        /// Basis points of voucher's thickness to stake [0, 10_000]. Integer, exact.
        stake_bps: u32,
        /// Epoch after which this vouch expires. None = permanent (until clawback).
        expiration_epoch: Option<u64>,
        /// Monotonic nonce from the voucher, preventing replay.
        nonce: u64,
        /// When the vouch was created.
        timestamp: DateTime<Utc>,
    },
    /// BootstrapGenesis seeds the mesh with root thickness and declares
    /// which keys belong to the root operator. Accepted exactly once, at
    /// chain height 0, root-signed. Era one begins with this block.
    Genesis {
        /// The root operator's PeerId.
        root: String,
        /// Initial thickness grant in gauge-scaled units.
        thickness_grant: f64,
        /// Keys declared as belonging to the same operator as root.
        /// Certificates from panels composed entirely of declared keys
        /// are structurally self-certifications — auditable, not forbidden.
        declared_operator_keys: Vec<String>,
        nonce: u64,
        timestamp: DateTime<Utc>,
    },
    /// BootstrapEnded is the one-way transition from era one (root-authorized)
    /// to era two (quorum-certified). Signed by root, accepted exactly once.
    /// Its presence in the chain is the era marker — derived, not stored.
    BootstrapEnded {
        /// Who declared the end of bootstrap.
        declared_by: String,
        /// Human-readable reason for the transition.
        reason: String,
        nonce: u64,
        timestamp: DateTime<Utc>,
    },
}

impl Transaction {
    /// The PeerId of the actor initiating this transaction.
    pub fn signer(&self) -> &str {
        match self {
            Transaction::Transfer { from, .. } => from,
            Transaction::Mint { authority, .. } => authority,
            Transaction::Vouch { voucher, .. } => voucher,
            Transaction::Genesis { root, .. } => root,
            Transaction::BootstrapEnded { declared_by, .. } => declared_by,
        }
    }

    pub fn nonce(&self) -> u64 {
        match self {
            Transaction::Transfer { nonce, .. } => *nonce,
            Transaction::Mint { nonce, .. } => *nonce,
            Transaction::Vouch { nonce, .. } => *nonce,
            Transaction::Genesis { nonce, .. } => *nonce,
            Transaction::BootstrapEnded { nonce, .. } => *nonce,
        }
    }

    pub fn is_mint(&self) -> bool {
        matches!(self, Transaction::Mint { .. })
    }

    /// Whether this transaction alters thickness in any way.
    pub fn affects_thickness(&self) -> bool {
        matches!(
            self,
            Transaction::Genesis { .. } | Transaction::Vouch { .. }
        )
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
