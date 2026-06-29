use std::collections::HashMap;

use anyhow::{bail, Result};
use libp2p::PeerId;

use super::types::{DigitalUtilityUnit, Transaction};

/// Local ledger state — each node's view of balances.
///
/// This is NOT consensus. Two nodes might temporarily disagree about
/// balances if they haven't seen the same transactions yet. Consensus
/// is a Phase 5/6 concern. For Phase 4, this is a local ledger that
/// tracks balances as this node sees them.
#[derive(Debug)]
pub struct LedgerState {
    balances: HashMap<PeerId, DigitalUtilityUnit>,
}

impl LedgerState {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
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
            Transaction::Transfer { from, to, amount, .. } => {
                let from_peer: PeerId = from.parse().map_err(|e| {
                    anyhow::anyhow!("invalid from PeerId in transfer: {e}")
                })?;
                let to_peer: PeerId = to.parse().map_err(|e| {
                    anyhow::anyhow!("invalid to PeerId in transfer: {e}")
                })?;

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
                let to_peer: PeerId = to.parse().map_err(|e| {
                    anyhow::anyhow!("invalid to PeerId in mint: {e}")
                })?;
                let current = self.balance_of(&to_peer);
                let new_balance = current
                    .checked_add(*amount)
                    .ok_or_else(|| anyhow::anyhow!("balance overflow on mint"))?;
                self.balances.insert(to_peer, new_balance);
            }
        }
        Ok(())
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
}
