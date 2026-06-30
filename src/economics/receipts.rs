// ── economics/receipts.rs — peer-verified contribution claims ───
//
// The Lattice does not trust self-reported metrics.  Every contribution
// claim must be backed by at least one peer's signature confirming it
// happened.  This module defines the cryptographic evidence layer:
//
//   RelayReceipt  — "I witnessed you relay this message for someone else"
//   SignedReceipt — the above, signed by the witness
//
// The exchange: when node A receives a gossipsub message delivered by
// node B (and the message originated from someone else), A issues a
// RelayReceipt to B: "B relayed this message."  B collects receipts
// from its peers as proof of contribution.
//
// At epoch boundary, the mint calculation uses ONLY verified receipts,
// not self-reported metrics.  Self-reported numbers become diagnostics.

use chrono::{DateTime, Utc};
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use tracing::debug;

/// A witness statement: "I saw you relay this message."
///
/// Created by the *receiver* of a gossipsub message and sent back to
/// the *deliverer* (the peer that handed the message to the receiver).
/// The beneficiary is the deliverer — the node whose contribution is
/// being attested.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RelayReceipt {
    /// The PeerId of the node that relayed/delivered the message
    /// (the beneficiary — the one getting credit).
    pub relayer: String,

    /// The PeerId of the node issuing this receipt (the witness).
    pub beneficiary: String,

    /// Size of the relayed message in bytes.
    pub bytes: u64,

    /// Blake3 hash of the gossipsub message payload.  Anchors the
    /// receipt to a specific message that actually transited the
    /// network — prevents fabricated receipts between colluding
    /// nodes for messages that never existed.
    pub message_hash: [u8; 32],

    /// When the receipt was issued.
    pub timestamp: DateTime<Utc>,
}

/// A RelayReceipt signed by the witness (beneficiary field).
///
/// Same pattern as SignedTransaction from Phase 4: the receipt body
/// plus an Ed25519 signature that the verifier can check against the
/// witness's known public key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignedReceipt {
    /// The receipt body.
    pub receipt: RelayReceipt,

    /// Protobuf-encoded public key of the signer (witness/beneficiary).
    pub signer_public_key: Vec<u8>,

    /// Ed25519 signature over the CBOR-encoded receipt body.
    pub signature: Vec<u8>,
}

impl RelayReceipt {
    /// Create a new relay receipt.
    pub fn new(
        relayer: PeerId,
        beneficiary: PeerId,
        bytes: u64,
        message_hash: [u8; 32],
    ) -> Self {
        Self {
            relayer: relayer.to_string(),
            beneficiary: beneficiary.to_string(),
            bytes,
            message_hash,
            timestamp: Utc::now(),
        }
    }
}

/// Validate a SignedReceipt:
///
/// 1. Signature verifies against the signer's public key.
/// 2. The message_hash corresponds to a message the verifying node
///    has seen transit the network (caller provides the recently-seen
///    hash set — prevents receipts for fabricated messages).
/// 3. Returns Ok(()) if valid.
pub fn validate_receipt(
    signed: &SignedReceipt,
    recently_seen_hashes: &std::collections::HashSet<[u8; 32]>,
) -> Result<(), ReceiptValidationError> {
    // ── 1. Message-hash anchoring ──────────────────────────
    // The receipt must reference a message hash this node has
    // actually observed transiting the network.  Without this,
    // two colluding nodes could forge receipts for imaginary
    // messages.  Full Byzantine fault tolerance would require
    // supermajority witness confirmation across the mesh, but
    // message-hash anchoring raises the bar significantly.
    //
    // TODO: full Byzantine fault tolerance requires supermajority
    // witness — multiple independent nodes confirming the same
    // message transited.  For Phase 6, single-witness anchoring
    // is sufficient.
    if !recently_seen_hashes.contains(&signed.receipt.message_hash) {
        debug!(
            hash = %hex::encode(signed.receipt.message_hash),
            "Receipt rejected: message hash not in recently-seen set"
        );
        return Err(ReceiptValidationError::UnknownMessageHash);
    }

    // ── 2. Signature verification ─────────────────────────
    // Decode the signer's public key and verify the signature
    // over the CBOR-encoded receipt body.
    let pk = libp2p::identity::PublicKey::try_decode_protobuf(
        &signed.signer_public_key,
    )
    .map_err(|_| ReceiptValidationError::InvalidPublicKey)?;

    let receipt_bytes = serde_cbor::to_vec(&signed.receipt)
        .map_err(|_| ReceiptValidationError::SerializationFailed)?;

    if !pk.verify(&receipt_bytes, &signed.signature) {
        return Err(ReceiptValidationError::InvalidSignature);
    }

    Ok(())
}

/// Errors that can occur during receipt validation.
#[derive(Debug)]
pub enum ReceiptValidationError {
    /// The message_hash in the receipt doesn't match any message
    /// this node has observed.
    UnknownMessageHash,
    /// The signer's public key is malformed.
    InvalidPublicKey,
    /// The signature doesn't verify against the receipt body.
    InvalidSignature,
    /// CBOR serialization failed (shouldn't happen for valid
    /// receipts but guards against invalid data).
    SerializationFailed,
}

impl std::fmt::Display for ReceiptValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownMessageHash => write!(f, "unknown message hash"),
            Self::InvalidPublicKey => write!(f, "invalid public key"),
            Self::InvalidSignature => write!(f, "invalid signature"),
            Self::SerializationFailed => write!(f, "serialization failed"),
        }
    }
}

impl std::error::Error for ReceiptValidationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::identity;

    #[test]
    fn valid_receipt_passes_validation() {
        let keypair = identity::Keypair::generate_ed25519();
        let relayer = PeerId::random();
        let beneficiary = PeerId::from(keypair.public());

        let receipt = RelayReceipt::new(
            relayer,
            beneficiary,
            1024,
            [0xAB; 32],
        );

        let receipt_bytes = serde_cbor::to_vec(&receipt).unwrap();
        let signature = keypair.sign(&receipt_bytes).unwrap();
        let signer_public_key = keypair.public().encode_protobuf();

        let signed = SignedReceipt {
            receipt,
            signer_public_key,
            signature,
        };

        let mut seen = std::collections::HashSet::new();
        seen.insert([0xAB; 32]);

        assert!(validate_receipt(&signed, &seen).is_ok());
    }

    #[test]
    fn unknown_message_hash_fails() {
        let keypair = identity::Keypair::generate_ed25519();
        let relayer = PeerId::random();
        let beneficiary = PeerId::from(keypair.public());

        let receipt = RelayReceipt::new(
            relayer,
            beneficiary,
            1024,
            [0xCD; 32],
        );

        let receipt_bytes = serde_cbor::to_vec(&receipt).unwrap();
        let signature = keypair.sign(&receipt_bytes).unwrap();

        let signed = SignedReceipt {
            receipt,
            signer_public_key: keypair.public().encode_protobuf(),
            signature,
        };

        let seen = std::collections::HashSet::new(); // empty — hash not seen

        assert!(validate_receipt(&signed, &seen).is_err());
    }
}
