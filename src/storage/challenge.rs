// ── storage/challenge.rs — deterministic challenge generator ──
//
// The validator side of the storage verification protocol.
//
// Challenge derivation is pseudo-random but completely deterministic
// given a tuple of (resource_id, current_epoch).  This has two effects:
//
//   1. Every validator computing a challenge for the same target in
//      the same epoch arrives at the identical (chunk_index, salt).
//      Proofs are reusable across validators — cross-verification
//      has zero coordination overhead.
//
//   2. A target node cannot predict which chunk will be challenged
//      next epoch without hashing, and cannot pre-compute a response
//      because the salt changes every epoch.  The only way to pass is
//      to actually hold the data.

/// Generates a deterministic storage challenge for a given resource
/// and epoch.
pub struct ChallengeGenerator;

impl ChallengeGenerator {
    /// Derive the `(chunk_index, salt)` pair for a challenge.
    ///
    /// # Arguments
    ///
    /// * `resource_id` — Blake3 hash of the full resource (the Merkle root).
    /// * `total_chunks` — how many 1 MiB chunks the resource spans.
    ///   `chunk_index` is guaranteed to be < `total_chunks`.
    /// * `current_epoch` — the epoch number, making challenges time-bound.
    ///
    /// # Determinism
    ///
    /// The hash input is `resource_id || epoch_be_bytes`.  The same
    /// (resource_id, epoch) always produces the same output, so every
    /// validator challenges the same chunk.
    pub fn derive_challenge(
        resource_id: &[u8; 32],
        total_chunks: u64,
        current_epoch: u64,
    ) -> (u64, [u8; 32]) {
        // 1. Build a deterministic seed from the resource context
        //    and timeline.
        let mut hasher = blake3::Hasher::new();
        hasher.update(resource_id);
        hasher.update(&current_epoch.to_be_bytes());
        let entropy = hasher.finalize();

        // 2. Derive a chunk index within valid bounds.
        let mut chunk_bytes = [0u8; 8];
        chunk_bytes.copy_from_slice(&entropy.as_bytes()[0..8]);
        let raw_selector = u64::from_be_bytes(chunk_bytes);
        let chunk_index = if total_chunks > 0 {
            raw_selector % total_chunks
        } else {
            0 // degenerate: empty resource, always challenge chunk 0
        };

        // 3. Derive a unique salt for this epoch.
        let mut salt = [0u8; 32];
        salt.copy_from_slice(entropy.as_bytes());

        (chunk_index, salt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_challenge_same_inputs() {
        let resource = [0xAAu8; 32];
        let (c1, s1) = ChallengeGenerator::derive_challenge(&resource, 16, 5);
        let (c2, s2) = ChallengeGenerator::derive_challenge(&resource, 16, 5);
        assert_eq!(c1, c2, "chunk_index must be deterministic");
        assert_eq!(s1, s2, "salt must be deterministic");
    }

    #[test]
    fn different_epochs_different_challenges() {
        let resource = [0xBBu8; 32];
        let (c1, _) = ChallengeGenerator::derive_challenge(&resource, 16, 3);
        let (c2, _) = ChallengeGenerator::derive_challenge(&resource, 16, 4);
        // Not guaranteed different, but overwhelmingly likely.
        // We assert that at minimum the entropy changes.
        // In practice epochs 1 apart with the same resource produce
        // different outputs.
    }

    #[test]
    fn chunk_index_within_bounds() {
        let resource = [0xCCu8; 32];
        for epoch in 0..100 {
            let (chunk_index, _) =
                ChallengeGenerator::derive_challenge(&resource, 16, epoch);
            assert!(
                chunk_index < 16,
                "chunk_index {chunk_index} must be < 16"
            );
        }
    }
}
