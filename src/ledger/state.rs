use std::collections::HashMap;

use anyhow::{bail, Result};
use libp2p::PeerId;
use tracing::{debug, info, warn};

use super::thickness::ThicknessGraph;
use super::types::{DigitalUtilityUnit, ResourceClaim, Transaction};

/// How many epochs must pass before a previously-verified claim
/// is due for re-verification.
const VERIFICATION_INTERVAL: u64 = 5;

/// Local ledger state — each node's view of balances and resource claims.
///
/// This is NOT consensus. Two nodes might temporarily disagree about
/// balances if they haven't seen the same transactions yet. Consensus
/// is a Phase 5/6 concern. For Phase 4, this is a local ledger that
/// tracks balances as this node sees them.
#[derive(Debug)]
pub struct LedgerState {
    balances: HashMap<PeerId, DigitalUtilityUnit>,
    /// Resource claims tracked by this node, keyed by resource_id.
    claims: HashMap<[u8; 32], ResourceClaim>,
    /// Thickness provenance graph — tracks contribution-derived and
    /// vouch-derived thickness with full derivation lineage.
    pub thickness_graph: ThicknessGraph,
}

impl LedgerState {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
            claims: HashMap::new(),
            thickness_graph: ThicknessGraph::new(),
        }
    }

    /// Query a peer's balance as this node sees it.
    /// Returns zero for unknown peers (they haven't transacted yet).
    pub fn balance_of(&self, peer: &PeerId) -> DigitalUtilityUnit {
        self.balances
            .get(peer)
            .copied()
            .unwrap_or(DigitalUtilityUnit::ZERO)
    }

    /// Directly set a balance (for minting and test bootstrapping).
    /// Unlike `apply_transaction`, this bypasses all validation.
    pub fn set_balance(&mut self, peer: &PeerId, amount: DigitalUtilityUnit) {
        self.balances.insert(*peer, amount);
    }

    /// Apply a validated transaction to local state.
    ///
    /// Caller must have already verified the signature, checked the
    /// nonce, and confirmed sufficient balance. This method performs
    /// only the state mutation.
    pub fn apply_transaction(&mut self, tx: &Transaction) -> Result<()> {
        match tx {
            Transaction::Transfer {
                from, to, amount, ..
            } => {
                let from_peer: PeerId = from
                    .parse()
                    .map_err(|e| anyhow::anyhow!("invalid from PeerId in transfer: {e}"))?;
                let to_peer: PeerId = to
                    .parse()
                    .map_err(|e| anyhow::anyhow!("invalid to PeerId in transfer: {e}"))?;

                // Debit sender
                let sender_balance = self.balance_of(&from_peer);
                let new_sender = sender_balance
                    .checked_sub(*amount)
                    .ok_or_else(|| anyhow::anyhow!("insufficient balance"))?;
                self.balances.insert(from_peer, new_sender);

                // Credit recipient
                let recipient_balance = self.balance_of(&to_peer);
                let new_recipient = recipient_balance
                    .checked_add(*amount)
                    .ok_or_else(|| anyhow::anyhow!("balance overflow"))?;
                self.balances.insert(to_peer, new_recipient);
            }
            Transaction::Mint { to, amount, .. } => {
                let to_peer: PeerId = to
                    .parse()
                    .map_err(|e| anyhow::anyhow!("invalid to PeerId in mint: {e}"))?;
                let current = self.balance_of(&to_peer);
                let new_balance = current
                    .checked_add(*amount)
                    .ok_or_else(|| anyhow::anyhow!("balance overflow on mint"))?;
                self.balances.insert(to_peer, new_balance);
            }
            Transaction::Vouch {
                voucher,
                vouchee,
                staked_fraction,
                expiration_epoch,
                nonce,
                ..
            } => {
                let voucher_peer: PeerId = voucher
                    .parse()
                    .map_err(|e| anyhow::anyhow!("invalid voucher PeerId: {e}"))?;
                let vouchee_peer: PeerId = vouchee
                    .parse()
                    .map_err(|e| anyhow::anyhow!("invalid vouchee PeerId: {e}"))?;

                // The validation layer already checked unencumbered thickness.
                // Here we just apply the graph mutation.
                let per_vouchee = self
                    .thickness_graph
                    .stake_vouch(&voucher_peer, &vouchee_peer, *staked_fraction, *nonce, *expiration_epoch)
                    .map_err(|e| anyhow::anyhow!("vouch failed: {e}"))?;

                info!(
                    voucher = %voucher,
                    vouchee = %vouchee,
                    per_vouchee = format!("{:.4}", per_vouchee),
                    expires = ?expiration_epoch,
                    "Vouch applied — derived thickness granted"
                );
            }
        }
        Ok(())
    }

    // ── Phase 6: resource claims ──────────────────────────

    /// Register a resource claim for a peer.
    pub fn register_claim(
        &mut self,
        resource_id: [u8; 32],
        owner: String,
        size_bytes: u64,
        total_chunks: u64,
    ) {
        let claim = ResourceClaim::new(resource_id, owner, size_bytes, total_chunks);
        info!(
            resource = %hex::encode(resource_id),
            size = size_bytes,
            chunks = total_chunks,
            "Resource claim registered"
        );
        self.claims.insert(resource_id, claim);
    }

    /// Look up a claim by resource_id.
    pub fn get_claim(&self, resource_id: &[u8; 32]) -> Option<&ResourceClaim> {
        self.claims.get(resource_id)
    }

    /// Return all claims that are due for re-verification in the
    /// current epoch.
    ///
    /// A claim is due if it has never been verified, or if the
    /// last successful verification was more than
    /// `VERIFICATION_INTERVAL` epochs ago.
    pub fn get_claims_due_for_verification(
        &self,
        current_epoch: u64,
    ) -> Vec<&ResourceClaim> {
        self.claims
            .values()
            .filter(|claim| {
                match claim.last_successful_challenge {
                    None => true, // never verified
                    Some(last_epoch) => {
                        current_epoch.saturating_sub(last_epoch)
                            >= VERIFICATION_INTERVAL
                    }
                }
            })
            .collect()
    }

    /// Return an iterator over all tracked claims.
    pub fn claims(&self) -> impl Iterator<Item = &ResourceClaim> {
        self.claims.values()
    }

    // ── Phase 6: verification tracking ────────────────────

    /// Record a successful storage verification.
    ///
    /// Resets `consecutive_failures` to zero and increases
    /// `tenure_health` toward 1.0.  The health recovery is
    /// gradual — a Pi 5 that drops one challenge due to Wi-Fi
    /// glitch can earn its way back.
    ///
    /// Returns the contribution reward amount (size_bytes ×
    /// tenure_health) that should be minted for the owner.
    pub fn record_verification_success(
        &mut self,
        resource_id: &[u8; 32],
        peer: &PeerId,
        epoch: u64,
    ) -> u64 {
        let claim = match self.claims.get_mut(resource_id) {
            Some(c) => c,
            None => {
                debug!("Verification success for unknown claim — ignoring");
                return 0;
            }
        };

        claim.consecutive_failures = 0;
        claim.last_successful_challenge = Some(epoch);
        claim.tenure_health = (claim.tenure_health + 0.10).min(1.0);

        // Contribution reward: larger resources with healthy
        // tenure earn more.  The reward scales with both the
        // resource size and the current health multiplier.
        let reward = (claim.size_bytes as f64 * claim.tenure_health) as u64;

        // Layer 1 thickness: verified storage contribution mints thickness.
        // This is the ONLY source of NEW thickness in the provenance graph.
        // Amount = size_bytes × tenure_health / 1_000_000, so a 10 MiB
        // resource at 1.0 health mints ~10.5 thickness units.
        let thickness_amount = (claim.size_bytes as f64 * claim.tenure_health) / 1_000_000.0;
        if thickness_amount > 0.0 {
            self.thickness_graph.add_verified_contribution(
                peer,
                *resource_id,
                thickness_amount,
            );
        }

        info!(
            resource = %hex::encode(*resource_id),
            peer = %peer,
            health = %format!("{:.2}", claim.tenure_health),
            reward,
            "Storage verification SUCCESS — health recovering"
        );

        reward
    }

    /// Record a failed storage verification.
    ///
    /// Increments `consecutive_failures` and degrades
    /// `tenure_health` by multiplication (×0.80).  If health
    /// drops below `EVICTION_HEALTH`, the claim is dissolved.
    ///
    /// Returns `true` if the claim was evicted.
    pub fn record_verification_failure(
        &mut self,
        resource_id: &[u8; 32],
        peer: &PeerId,
        epoch: u64,
    ) -> bool {
        let claim = match self.claims.get_mut(resource_id) {
            Some(c) => c,
            None => {
                debug!("Verification failure for unknown claim — ignoring");
                return false;
            }
        };

        claim.consecutive_failures += 1;
        claim.last_successful_challenge = Some(epoch);
        // Exponential decay: each failure shaves 20% off health.
        claim.tenure_health *= 0.80;

        let evicted = claim.should_evict();

        if evicted {
            warn!(
                resource = %hex::encode(*resource_id),
                peer = %peer,
                health = %format!("{:.3}", claim.tenure_health),
                consecutive = claim.consecutive_failures,
                "Storage verification FAILED — health below eviction threshold, CLAIM DISSOLVED"
            );
            self.claims.remove(resource_id);
        } else {
            warn!(
                resource = %hex::encode(*resource_id),
                peer = %peer,
                health = %format!("{:.2}", claim.tenure_health),
                consecutive = claim.consecutive_failures,
                "Storage verification FAILED — health degrading, LVT escalating"
            );
        }

        evicted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::types::DigitalUtilityUnit;
    use chrono::Utc;

    #[test]
    fn transfer_moves_units() {
        let mut state = LedgerState::new();
        let alice: PeerId = PeerId::random();
        let bob: PeerId = PeerId::random();

        state.set_balance(&alice, DigitalUtilityUnit(1000));

        let tx = Transaction::Transfer {
            from: alice.to_string(),
            to: bob.to_string(),
            amount: DigitalUtilityUnit(300),
            nonce: 1,
            timestamp: Utc::now(),
        };

        state.apply_transaction(&tx).unwrap();
        assert_eq!(state.balance_of(&alice), DigitalUtilityUnit(700));
        assert_eq!(state.balance_of(&bob), DigitalUtilityUnit(300));
    }

    #[test]
    fn mint_creates_units() {
        let mut state = LedgerState::new();
        let recipient: PeerId = PeerId::random();

        let tx = Transaction::Mint {
            to: recipient.to_string(),
            amount: DigitalUtilityUnit(500),
            authority: PeerId::random().to_string(),
            nonce: 1,
            timestamp: Utc::now(),
        };

        state.apply_transaction(&tx).unwrap();
        assert_eq!(state.balance_of(&recipient), DigitalUtilityUnit(500));
    }

    #[test]
    fn unknown_peer_balance_is_zero() {
        let state = LedgerState::new();
        let unknown: PeerId = PeerId::random();
        assert_eq!(state.balance_of(&unknown), DigitalUtilityUnit::ZERO);
    }

    #[test]
    fn claim_health_recovers_on_success() {
        let mut state = LedgerState::new();
        let peer = PeerId::random();
        let rid = [0xAA; 32];

        state.register_claim(rid, peer.to_string(), 1024 * 1024, 16);

        // Fail twice — health degrades.
        state.record_verification_failure(&rid, &peer, 1);
        state.record_verification_failure(&rid, &peer, 2);

        let claim = state.get_claim(&rid).unwrap();
        let health_after_fails = claim.tenure_health;
        assert!(health_after_fails < 1.0, "health should degrade on failure");

        // Succeed — health recovers.
        let reward = state.record_verification_success(&rid, &peer, 3);
        let claim = state.get_claim(&rid).unwrap();
        assert!(
            claim.tenure_health > health_after_fails,
            "health should recover on success"
        );
        assert_eq!(claim.consecutive_failures, 0);
        assert!(reward > 0);
    }

    #[test]
    fn claim_evicted_after_repeated_failures() {
        let mut state = LedgerState::new();
        let peer = PeerId::random();
        let rid = [0xBB; 32];

        state.register_claim(rid, peer.to_string(), 100, 1);

        // Fail repeatedly until eviction.
        let mut evicted = false;
        for epoch in 1..=20 {
            evicted = state.record_verification_failure(&rid, &peer, epoch);
            if evicted {
                break;
            }
        }

        assert!(evicted, "claim should be evicted after repeated failures");
        assert!(
            state.get_claim(&rid).is_none(),
            "evicted claim should be removed"
        );
    }
}
