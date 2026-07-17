// In node.rs

impl Node {
    /// Handle a remote ledger received from a peer during gossip sync.
    /// Returns Ok(true) if our chain was replaced (we were on the losing fork).
    pub fn handle_remote_ledger(
        &mut self,
        remote_ledger: &std::path::Path,
        peer_id: &PeerId,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // 1. Detect fork
        let fork = match self.commit_manager.detect_fork(remote_ledger) {
            Ok(Some(f)) => f,
            Ok(None) => {
                info!("[sync] Ledgers match peer {peer_id} — no fork");
                return Ok(false);
            }
            Err(e) => {
                warn!("[sync] Failed to read remote ledger from {peer_id}: {e}. Skipping sync.");
                return Ok(false);
            }
        };

        info!("[sync] Fork detected with {peer_id} at height {}", fork.fork_point);

        // 2. Resolve — closure captures the node's thickness view
        let resolution = self.commit_manager.resolve_fork(&fork, |peer_id: &PeerId| {
            self.ledger.thickness_graph.total_thickness(peer_id)
        });

        match resolution {
            ResolutionResult::Winner(winning_fork) => {
                // Check if we won — compare first divergent block hash
                let local_won = fork.local_fork.first()
                    .zip(winning_fork.first())
                    .map(|(a, b)| a.block_hash == b.block_hash)
                    .unwrap_or(false);

                if !local_won {
                    info!("[sync] Adopting winning fork from {peer_id}");

                    // Extract lost proposals BEFORE adopting (losing fork blocks still valid)
                    let lost_proposals = self.commit_manager.extract_proposal_ids(&fork.local_fork);

                    // Adopt winning fork — atomic disk replacement + in-memory state update
                    self.commit_manager.adopt_winning_fork(&winning_fork, fork.fork_point)?;

                    // Clear losing fork proposals from committed set so they can be re-submitted
                    self.commit_manager.remove_committed_proposals(&lost_proposals);

                    if !lost_proposals.is_empty() {
                        info!("[sync] {} proposals from losing fork available for re-submission", lost_proposals.len());
                        // Future: emit NodeEvent::LosingForkDetected(lost_proposals)
                    }

                    Ok(true)
                } else {
                    info!("[sync] Local fork won — no action needed");
                    Ok(false)
                }
            }
            ResolutionResult::NoFork => {
                // Unreachable if detect_fork returned Some
                warn!("[sync] resolve_fork returned NoFork after detect_fork succeeded");
                Ok(false)
            }
        }
    }
}
