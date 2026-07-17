use std::collections::HashMap;

use anyhow::{bail, Result};
use libp2p::{identity, PeerId};
use tracing::warn;

use super::state::LedgerState;
use super::types::{DigitalUtilityUnit, SignedTransaction, Transaction};

/// Structured error from transaction validation.
/// Callers can match on `GappedNonce` to trigger a fetch protocol.
/// All other validation failures are opaque `Other` variants.
#[derive(Debug)]
pub enum ValidationError {
    /// A transaction arrived with a nonce that skips one or more
    /// predecessors.  `expected` is the lowest nonce that would
    /// close the gap; `got` is the nonce that was rejected.
    GappedNonce { signer: PeerId, expected: u64, got: u64 },
    /// Any other validation failure (balance, signature, timestamp, …).
    Other(anyhow::Error),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GappedNonce { signer, expected, got } => {
                write!(f, "gapped nonce {got} from {signer} (expected {expected})")
            }
            Self::Other(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ValidationError {}

impl From<anyhow::Error> for ValidationError {
    fn from(e: anyhow::Error) -> Self {
        Self::Other(e)
    }
}

/// Maximum age for a transaction before it's considered stale.
/// Prevents replay of old transactions from before a node's memory.
const MAX_TX_AGE_SECS: i64 = 300; // 5 minutes

/// Validate and apply a signed transaction to the local ledger state.
///
/// Returns `Ok(())` if the transaction is valid and was applied,
/// or a `ValidationError` describing why it was rejected.
pub fn validate_and_apply(
    tx: &SignedTransaction,
    state: &mut LedgerState,
    seen_nonces: &mut HashMap<PeerId, u64>,
) -> Result<(), ValidationError> {
    validate_and_apply_with_genesis_root(tx, state, seen_nonces, None)
}

/// Full validation with genesis-root gate.
pub fn validate_and_apply_with_genesis_root(
    tx: &SignedTransaction,
    state: &mut LedgerState,
    seen_nonces: &mut HashMap<PeerId, u64>,
    genesis_root: Option<&PeerId>,
) -> Result<(), ValidationError> {
    // 1. Verify the signature
    verify_signature(tx)?;

    // 2. Genesis gate: only the configured root may submit Genesis.
    // Genesis mints thickness from nothing — the strictest gate.
    if matches!(tx.transaction, Transaction::Genesis { .. }) {
        let signer: PeerId = tx.transaction.signer().parse()
            .map_err(|e| anyhow::anyhow!("invalid genesis signer PeerId: {e}"))?;
        match genesis_root {
            Some(root) if signer == *root => { /* ok */ }
            Some(root) => return Err(ValidationError::Other(anyhow::anyhow!(
                "genesis rejected: signer {} is not the configured root {}",
                signer, root
            ))),
            None => return Err(ValidationError::Other(anyhow::anyhow!(
                "genesis rejected: --genesis-root not configured — \
                 this node cannot validate the trust anchor"
            ))),
        }
    }

    // 3. Extract signer and nonce
    check_timestamp(tx)?;

    let signer: PeerId = tx
        .transaction
        .signer()
        .parse()
        .map_err(|e| anyhow::anyhow!("invalid signer PeerId: {e}"))?;

    // 3. Replay protection: nonce must be strictly greater than last seen
    check_nonce(&signer, tx.transaction.nonce(), seen_nonces)?;

    // 4. For transfers, check sufficient balance
    if let Transaction::Transfer { from, amount, .. } = &tx.transaction {
        let from_peer: PeerId = from
            .parse()
            .map_err(|e| anyhow::anyhow!("invalid from PeerId: {e}"))?;
        let balance = state.balance_of(&from_peer);
        if balance < *amount {
            return Err(ValidationError::Other(anyhow::anyhow!(
                "insufficient balance: {} has {}, needs {}",
                from,
                balance,
                amount
            )));
        }
    }

    // 4b. For vouches, check sufficient unencumbered thickness (exact integer bps)
    if let Transaction::Vouch {
        voucher,
        stake_bps,
        ..
    } = &tx.transaction
    {
        let voucher_peer: PeerId = voucher
            .parse()
            .map_err(|e| anyhow::anyhow!("invalid voucher PeerId: {e}"))?;
        let voucher_total = state.thickness_graph.total_thickness(&voucher_peer);
        if voucher_total <= 0.0 {
            return Err(ValidationError::Other(anyhow::anyhow!(
                "insufficient thickness: {} has no thickness to stake",
                voucher
            )));
        }
        let current_bps = state.thickness_graph.active_stake_bps(&voucher_peer);
        if current_bps + stake_bps > 10_000 {
            return Err(ValidationError::Other(anyhow::anyhow!(
                "insufficient unencumbered thickness: {} has {current_bps}bps staked, cannot add {stake_bps}bps (max 10_000)",
                voucher,
            )));
        }
    }

    // 5. Apply to local state
    state.apply_transaction(&tx.transaction)?;

    // 6. Record the nonce so we reject replays
    seen_nonces.insert(signer, tx.transaction.nonce());

    Ok(())
}

/// Verify the Ed25519 signature on a signed transaction.
fn verify_signature(tx: &SignedTransaction) -> Result<()> {
    // Reconstruct the public key from the protobuf-encoded bytes.
    let public_key = identity::PublicKey::try_decode_protobuf(&tx.signer_public_key)
        .map_err(|e| anyhow::anyhow!("invalid public key: {e}"))?;

    // The signature covers the CBOR-encoded transaction body.
    let tx_bytes = serde_cbor::to_vec(&tx.transaction)
        .map_err(|e| anyhow::anyhow!("failed to encode transaction for verification: {e}"))?;

    if !public_key.verify(&tx_bytes, &tx.signature) {
        bail!("invalid signature");
    }

    // Verify that the signer's public key matches the transaction's claimed signer.
    let key_peer_id = PeerId::from(public_key);
    let claimed_signer: PeerId = tx
        .transaction
        .signer()
        .parse()
        .map_err(|e| anyhow::anyhow!("invalid signer PeerId: {e}"))?;
    if key_peer_id != claimed_signer {
        bail!(
            "signer key mismatch: key resolves to {} but transaction claims {}",
            key_peer_id,
            claimed_signer
        );
    }

    Ok(())
}

/// Reject transactions older than MAX_TX_AGE_SECS.
fn check_timestamp(tx: &SignedTransaction) -> Result<()> {
    let tx_time = match &tx.transaction {
        Transaction::Transfer { timestamp, .. } => timestamp,
        Transaction::Mint { timestamp, .. } => timestamp,
        Transaction::Vouch { timestamp, .. } => timestamp,
        Transaction::Genesis { timestamp, .. } => timestamp,
        Transaction::BootstrapEnded { timestamp, .. } => timestamp,
    };

    let now = chrono::Utc::now();
    let age = (now - *tx_time).num_seconds();
    if age > MAX_TX_AGE_SECS || age < -MAX_TX_AGE_SECS {
        bail!(
            "transaction timestamp is {}s from now (max ±{}s)",
            age,
            MAX_TX_AGE_SECS
        );
    }

    Ok(())
}

/// Gap-free nonce enforcement: the nonce must be exactly predecessor+1.
///
/// `>` (monotonic) permits gaps, and gaps cause divergence — two nodes
/// that accept nonces 4 → 6 and 4 → 5 → 6 derive different state for the
/// same signer.  Requiring `== last + 1` ensures every node's state is a
/// valid prefix of that signer's sequence.  Nodes may be at different
/// prefixes (behind) but nobody is wrong.
fn check_nonce(
    signer: &PeerId,
    nonce: u64,
    seen_nonces: &HashMap<PeerId, u64>,
) -> Result<(), ValidationError> {
    if let Some(&last_nonce) = seen_nonces.get(signer) {
        if nonce != last_nonce + 1 {
            return Err(ValidationError::GappedNonce {
                signer: *signer,
                expected: last_nonce + 1,
                got: nonce,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::types::DigitalUtilityUnit;
    use chrono::Utc;
    use libp2p::identity;

    fn make_keypair() -> identity::Keypair {
        identity::Keypair::generate_ed25519()
    }

    fn sign_transaction(tx: &Transaction, keypair: &identity::Keypair) -> SignedTransaction {
        let tx_bytes = serde_cbor::to_vec(tx).unwrap();
        let signature = keypair.sign(&tx_bytes).unwrap();
        let signer_public_key = keypair.public().encode_protobuf();

        SignedTransaction {
            transaction: tx.clone(),
            signer_public_key,
            signature,
        }
    }

    #[test]
    fn valid_transfer_succeeds() {
        let alice = make_keypair();
        let bob = make_keypair();
        let alice_id = PeerId::from(alice.public());
        let bob_id = PeerId::from(bob.public());

        let mut state = LedgerState::new();
        let mut nonces = HashMap::new();

        // Give alice some starting balance
        state.set_balance(&alice_id, DigitalUtilityUnit(1000));

        let tx = Transaction::Transfer {
            from: alice_id.to_string(),
            to: bob_id.to_string(),
            amount: DigitalUtilityUnit(100),
            nonce: 1,
            timestamp: Utc::now(),
        };
        let signed = sign_transaction(&tx, &alice);

        assert!(validate_and_apply(&signed, &mut state, &mut nonces).is_ok());
        assert_eq!(state.balance_of(&alice_id), DigitalUtilityUnit(900));
        assert_eq!(state.balance_of(&bob_id), DigitalUtilityUnit(100));
    }

    #[test]
    fn insufficient_balance_rejected() {
        let alice = make_keypair();
        let bob = make_keypair();
        let alice_id = PeerId::from(alice.public());
        let bob_id = PeerId::from(bob.public());

        let mut state = LedgerState::new();
        let mut nonces = HashMap::new();

        state.set_balance(&alice_id, DigitalUtilityUnit(50));

        let tx = Transaction::Transfer {
            from: alice_id.to_string(),
            to: bob_id.to_string(),
            amount: DigitalUtilityUnit(100),
            nonce: 1,
            timestamp: Utc::now(),
        };
        let signed = sign_transaction(&tx, &alice);

        let result = validate_and_apply(&signed, &mut state, &mut nonces);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("insufficient balance"));
    }

    #[test]
    fn replayed_nonce_rejected() {
        let alice = make_keypair();
        let bob = make_keypair();
        let alice_id = PeerId::from(alice.public());
        let bob_id = PeerId::from(bob.public());

        let mut state = LedgerState::new();
        let mut nonces = HashMap::new();

        state.set_balance(&alice_id, DigitalUtilityUnit(1000));

        let tx = Transaction::Transfer {
            from: alice_id.to_string(),
            to: bob_id.to_string(),
            amount: DigitalUtilityUnit(100),
            nonce: 1,
            timestamp: Utc::now(),
        };
        let signed = sign_transaction(&tx, &alice);

        // First time: OK
        assert!(validate_and_apply(&signed, &mut state, &mut nonces).is_ok());

        // Second time: replay rejected (now a gap since nonce 1 == last_nonce + 0)
        let signed2 = sign_transaction(&tx, &alice);
        let result = validate_and_apply(&signed2, &mut state, &mut nonces);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("gapped nonce"));
    }

    #[test]
    fn gapped_nonce_rejected_exact_next_accepted() {
        // Discriminator: the rule must accept exact-next nonces and
        // reject gaps — otherwise "no gapped nonces" could be
        // implemented as "reject everything."
        let alice = make_keypair();
        let bob = make_keypair();
        let alice_id = PeerId::from(alice.public());
        let bob_id = PeerId::from(bob.public());

        let mut state = LedgerState::new();
        let mut nonces = HashMap::new();

        state.set_balance(&alice_id, DigitalUtilityUnit(1000));

        let mk_tx = |nonce: u64| -> SignedTransaction {
            let tx = Transaction::Transfer {
                from: alice_id.to_string(),
                to: bob_id.to_string(),
                amount: DigitalUtilityUnit(100),
                nonce,
                timestamp: Utc::now(),
            };
            sign_transaction(&tx, &alice)
        };

        // nonce 1: first tx, no predecessor needed
        assert!(validate_and_apply(&mk_tx(1), &mut state, &mut nonces).is_ok());

        // nonce 3: gap (missing 2) → rejected
        let result = validate_and_apply(&mk_tx(3), &mut state, &mut nonces);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected 2"));

        // nonce 2: exact next → accepted (proves the rule isn't too broad)
        assert!(validate_and_apply(&mk_tx(2), &mut state, &mut nonces).is_ok());
    }

    #[test]
    fn cap_exceeded_on_fetched_vouch_is_rejected() {
        // Discriminator: a trusting fetch path that checks only
        // "signature valid, nonce matches request" would accept this
        // transaction.  The full validate_and_apply path must reject it.
        //
        // Alice has thickness. She vouches 6000 bps (60%) to Bob.
        // A second vouch of 5000 bps (60+50=110% > 100%) exceeds the cap.
        // A third vouch of 3000 bps (60+30=90% ≤ 100%) is legitimate.
        // The trust test: 5000 bps is rejected, 3000 bps is accepted.
        let alice = make_keypair();
        let bob = make_keypair();
        let charlie = make_keypair();
        let dave = make_keypair();
        let alice_id = PeerId::from(alice.public());
        let bob_id = PeerId::from(bob.public());
        let charlie_id = PeerId::from(charlie.public());
        let dave_id = PeerId::from(dave.public());

        let mut state = LedgerState::new();
        let mut nonces = HashMap::new();

        // Give Alice thickness via a direct contribution.
        state.thickness_graph.add_verified_contribution(&alice_id, [0xAA; 32], 1000.0);

        // Vouch 6000 bps to Bob — should succeed.
        let tx1 = Transaction::Vouch {
            voucher: alice_id.to_string(),
            vouchee: bob_id.to_string(),
            stake_bps: 6000,
            expiration_epoch: None,
            nonce: 1,
            timestamp: Utc::now(),
        };
        let signed1 = sign_transaction(&tx1, &alice);
        assert!(
            validate_and_apply(&signed1, &mut state, &mut nonces).is_ok(),
            "Alice's first vouch (6000 bps) should succeed"
        );

        // Vouch 5000 bps to Charlie — exceeds cap (6000 + 5000 = 11000 > 10000).
        // Correctly signed by Alice, correct nonce 2 — but fails the cap check.
        // A trusting fetch path that only checks envelope fields would apply it.
        let tx2 = Transaction::Vouch {
            voucher: alice_id.to_string(),
            vouchee: charlie_id.to_string(),
            stake_bps: 5000,
            expiration_epoch: None,
            nonce: 2,
            timestamp: Utc::now(),
        };
        let signed2 = sign_transaction(&tx2, &alice);
        let result = validate_and_apply(&signed2, &mut state, &mut nonces);
        assert!(result.is_err(), "Cap-violating vouch must be rejected");
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("insufficient unencumbered thickness"),
            "Error should name the cap, not the nonce: got {msg}"
        );

        // Vouch 3000 bps to Dave — fits (6000 + 3000 = 9000 ≤ 10000).
        // Nonce 2: the failed 5000-bps vouch at nonce 2 never advanced
        // seen_nonces, so nonce 2 is retryable.
        let tx3 = Transaction::Vouch {
            voucher: alice_id.to_string(),
            vouchee: dave_id.to_string(),
            stake_bps: 3000,
            expiration_epoch: None,
            nonce: 2,
            timestamp: Utc::now(),
        };
        let signed3 = sign_transaction(&tx3, &alice);
        assert!(
            validate_and_apply(&signed3, &mut state, &mut nonces).is_ok(),
            "Alice's third vouch (3000 bps, within cap) should succeed"
        );
    }

    #[test]
    fn bad_signature_rejected() {
        let alice = make_keypair();
        let bob = make_keypair();
        let mallory = make_keypair(); // not the signer
        let alice_id = PeerId::from(alice.public());
        let bob_id = PeerId::from(bob.public());

        let mut state = LedgerState::new();
        let mut nonces = HashMap::new();

        state.set_balance(&alice_id, DigitalUtilityUnit(1000));

        let tx = Transaction::Transfer {
            from: alice_id.to_string(),
            to: bob_id.to_string(),
            amount: DigitalUtilityUnit(100),
            nonce: 1,
            timestamp: Utc::now(),
        };
        // Sign with mallory's key but claim to be from alice
        let signed = sign_transaction(&tx, &mallory);

        let result = validate_and_apply(&signed, &mut state, &mut nonces);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("signer key mismatch"));
    }

    #[test]
    fn valid_mint_succeeds() {
        let minter = make_keypair();
        let recipient = make_keypair();
        let minter_id = PeerId::from(minter.public());
        let recipient_id = PeerId::from(recipient.public());

        let mut state = LedgerState::new();
        let mut nonces = HashMap::new();

        let tx = Transaction::Mint {
            to: recipient_id.to_string(),
            amount: DigitalUtilityUnit(500),
            authority: minter_id.to_string(),
            nonce: 1,
            timestamp: Utc::now(),
        };
        let signed = sign_transaction(&tx, &minter);

        assert!(validate_and_apply(&signed, &mut state, &mut nonces).is_ok());
        assert_eq!(state.balance_of(&recipient_id), DigitalUtilityUnit(500));
    }

    // ── Genesis validation tests ────────────────────────────

    #[test]
    fn genesis_received_from_non_root_is_rejected() {
        let root = PeerId::random();
        let imposter_key = identity::Keypair::generate_ed25519();
        let imposter = PeerId::from(imposter_key.public());
        let mut state = LedgerState::new();
        let mut nonces = HashMap::new();

        let tx = Transaction::Genesis {
            root: imposter.to_string(),
            thickness_grant: 1000.0,
            declared_operator_keys: vec![],
            nonce: 0,
            timestamp: chrono::Utc::now(),
        };
        let signed = sign_transaction(&tx, &imposter_key);

        let result = validate_and_apply_with_genesis_root(
            &signed, &mut state, &mut nonces, Some(&root),
        );
        assert!(result.is_err(), "non-root genesis must be rejected");
        assert!(
            result.unwrap_err().to_string().contains("is not the configured root"),
            "error should name the mismatch"
        );
    }

    #[test]
    fn genesis_received_without_config_is_rejected() {
        let key = identity::Keypair::generate_ed25519();
        let signer = PeerId::from(key.public());
        let mut state = LedgerState::new();
        let mut nonces = HashMap::new();

        let tx = Transaction::Genesis {
            root: signer.to_string(),
            thickness_grant: 1000.0,
            declared_operator_keys: vec![],
            nonce: 0,
            timestamp: chrono::Utc::now(),
        };
        let signed = sign_transaction(&tx, &key);

        let result = validate_and_apply_with_genesis_root(
            &signed, &mut state, &mut nonces, None,
        );
        assert!(result.is_err(), "genesis without trust anchor must be rejected");
        assert!(
            result.unwrap_err().to_string().contains("genesis-root not configured"),
            "error should name the missing config"
        );
    }

    #[test]
    fn valid_root_genesis_is_accepted() {
        let root_key = identity::Keypair::generate_ed25519();
        let root = PeerId::from(root_key.public());
        let mut state = LedgerState::new();
        let mut nonces = HashMap::new();

        let tx = Transaction::Genesis {
            root: root.to_string(),
            thickness_grant: 1000.0,
            declared_operator_keys: vec![root.to_string()],
            nonce: 0,
            timestamp: chrono::Utc::now(),
        };
        let signed = sign_transaction(&tx, &root_key);

        let result = validate_and_apply_with_genesis_root(
            &signed, &mut state, &mut nonces, Some(&root),
        );
        assert!(result.is_ok(), "valid root genesis must be accepted");
    }
}
