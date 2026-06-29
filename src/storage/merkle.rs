// ── storage/merkle.rs — Merkle tree for chunk verification ──
//
// A binary Merkle tree over fixed-size chunks of a resource.
// Each leaf is blake3(chunk_data).  Internal nodes are
// blake3(left_child || right_child).
//
// For odd-sized trees, the last leaf is duplicated to form a pair
// (standard Merkle construction — a 3-leaf tree becomes 4 leaves
// with leaf[3] == leaf[2]).

/// A full binary Merkle tree.
///
/// Stored in level-order: level 0 = leaves, level 1 = first internal
/// layer, ..., level N = root.  Each level is a Vec of [u8; 32] hashes.
#[derive(Debug, Clone)]
pub struct MerkleTree {
    /// Level 0 = leaf hashes, level N = root (single element).
    levels: Vec<Vec<[u8; 32]>>,
    /// The root hash (resource_id).
    root: [u8; 32],
}

impl MerkleTree {
    /// Build a Merkle tree from chunk data.
    ///
    /// Each chunk produces one leaf hash.  The root is the `resource_id`
    /// that other nodes can verify against.
    pub fn build(chunks: &[Vec<u8>]) -> Self {
        if chunks.is_empty() {
            // Degenerate: empty resource.  Root is blake3 of nothing.
            let root = *blake3::hash(&[]).as_bytes();
            return Self {
                levels: vec![vec![root]],
                root,
            };
        }

        // Level 0: hash each chunk.
        let leaves: Vec<[u8; 32]> = chunks
            .iter()
            .map(|c| *blake3::hash(c).as_bytes())
            .collect();

        let mut levels = vec![leaves.clone()];

        // Build internal levels until we reach a single root hash.
        let mut current = leaves;
        while current.len() > 1 {
            let mut next = Vec::with_capacity((current.len() + 1) / 2);

            for pair in current.chunks(2) {
                let left = pair[0];
                // If odd, duplicate the last element (standard Merkle).
                let right = if pair.len() == 1 { pair[0] } else { pair[1] };

                let mut hasher = blake3::Hasher::new();
                hasher.update(&left);
                hasher.update(&right);
                next.push(*hasher.finalize().as_bytes());
            }

            levels.push(next.clone());
            current = next;
        }

        let root = current[0];

        Self { levels, root }
    }

    /// The root hash (the `resource_id`).
    pub fn root(&self) -> [u8; 32] {
        self.root
    }

    /// Verify a Merkle inclusion proof without needing the full tree.
    ///
    /// Given a claimed leaf hash, a leaf index, a sibling path, and
    /// an expected root, this walks the path upward — applying the
    /// correct left/right ordering at each level — and checks that
    /// the result matches `expected_root`.
    ///
    /// This is a static method — the verifier doesn't need the full
    /// tree, just the root and the proof.
    pub fn verify_inclusion_proof(
        expected_root: &[u8; 32],
        claimed_leaf: &[u8; 32],
        leaf_index: u64,
        merkle_proof: &[Vec<u8>],
    ) -> bool {
        let mut current = *claimed_leaf;
        let mut idx = leaf_index as usize;

        for sibling in merkle_proof {
            let (left, right): (&[u8], &[u8]) = if idx % 2 == 0 {
                // Current is left child, sibling is right child.
                (current.as_slice(), sibling.as_slice())
            } else {
                // Current is right child, sibling is left child.
                (sibling.as_slice(), current.as_slice())
            };

            let mut hasher = blake3::Hasher::new();
            hasher.update(left);
            hasher.update(right);
            current = *hasher.finalize().as_bytes();

            idx /= 2;
        }

        // After processing all siblings, `current` should be the root.
        &current == expected_root
    }

    /// Extract the Merkle inclusion proof for a given leaf index.
    ///
    /// Returns the sibling hashes from leaf to root, in order.
    /// To verify: start with the leaf hash, then for each sibling,
    /// compute `blake3(left || right)` in the correct order until
    /// you reach the root.
    pub fn extract_merkle_path(&self, leaf_index: u64) -> Vec<Vec<u8>> {
        let mut path = Vec::new();
        let mut idx = leaf_index as usize;

        if self.levels.is_empty() || self.levels[0].is_empty() {
            return path;
        }

        let leaf_count = self.levels[0].len();
        if idx >= leaf_count {
            // Out of bounds — return empty path (validator will reject).
            return path;
        }

        for level in &self.levels {
            let sibling_idx = if idx % 2 == 0 {
                // My sibling is to the right (or I'm the last odd leaf).
                idx + 1
            } else {
                // My sibling is to the left.
                idx - 1
            };

            if sibling_idx < level.len() {
                path.push(level[sibling_idx].to_vec());
            } else {
                // Last odd leaf — sibling is myself (duplicated).
                path.push(level[idx].to_vec());
            }

            idx /= 2; // Move up to parent position in next level.
        }

        // Drop the last element — it's the root, which the verifier
        // already has.
        path.pop();

        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_resource_has_root() {
        let tree = MerkleTree::build(&[]);
        assert!(!tree.levels.is_empty());
        let root = tree.root();
        assert_eq!(root, *blake3::hash(&[]).as_bytes());
    }

    #[test]
    fn single_chunk_tree() {
        let chunk = b"hello world".to_vec();
        let tree = MerkleTree::build(&[chunk.clone()]);
        let expected_root = *blake3::hash(&chunk).as_bytes();
        assert_eq!(tree.root(), expected_root);

        // Path for leaf 0 should be empty (no siblings).
        let path = tree.extract_merkle_path(0);
        assert!(path.is_empty());
    }

    #[test]
    fn four_chunks_produces_valid_path() {
        let chunks: Vec<Vec<u8>> = (0..4).map(|i| vec![i as u8; 64]).collect();
        let tree = MerkleTree::build(&chunks);

        // Path for leaf 2 should contain 2 siblings.
        let path = tree.extract_merkle_path(2);
        assert_eq!(path.len(), 2);

        // Verify the path manually.
        // Leaf 2 = blake3([2; 64])
        // Level 0 siblings: leaf 2's sibling is leaf 3 (idx 2→3)
        // Level 1: parent of (2,3) is at idx 1; sibling is parent of (0,1) at idx 0
        let leaf2 = *blake3::hash(&chunks[2]).as_bytes();
        let leaf3 = *blake3::hash(&chunks[3]).as_bytes();

        // First sibling should be leaf3
        assert_eq!(path[0], leaf3.to_vec());

        // Compute parent of (2,3)
        let mut h = blake3::Hasher::new();
        h.update(&leaf2);
        h.update(&leaf3);
        let parent23 = *h.finalize().as_bytes();

        let leaf0 = *blake3::hash(&chunks[0]).as_bytes();
        let leaf1 = *blake3::hash(&chunks[1]).as_bytes();
        let mut h = blake3::Hasher::new();
        h.update(&leaf0);
        h.update(&leaf1);
        let parent01 = *h.finalize().as_bytes();

        // Second sibling should be parent(0,1)
        assert_eq!(path[1], parent01.to_vec());

        // Now verify: leaf2 + sibling[0] → parent(2,3) + sibling[1] → root
        // At level 0: leaf index 2 is even — sibling (leaf3) goes on the RIGHT.
        let mut h = blake3::Hasher::new();
        h.update(&leaf2);
        h.update(&leaf3);
        let computed_parent23 = *h.finalize().as_bytes();

        // At level 1: parent index is 1 (odd) — sibling (parent01) goes on the LEFT.
        let mut h = blake3::Hasher::new();
        h.update(&parent01);
        h.update(&computed_parent23);
        let computed_root = *h.finalize().as_bytes();

        assert_eq!(computed_root, tree.root());
    }

    #[test]
    fn three_chunks_odd_leaf_duplicated() {
        let chunks: Vec<Vec<u8>> = (0..3).map(|i| vec![i as u8; 32]).collect();
        let tree = MerkleTree::build(&chunks);

        // Path for leaf 2 (the last, odd leaf).
        let path = tree.extract_merkle_path(2);
        assert_eq!(path.len(), 2);

        // First sibling should be leaf2 itself (duplicated).
        let leaf2 = *blake3::hash(&chunks[2]).as_bytes();
        assert_eq!(path[0], leaf2.to_vec());
    }

    #[test]
    fn out_of_bounds_leaf_returns_empty_path() {
        let chunks: Vec<Vec<u8>> = (0..2).map(|i| vec![i as u8; 32]).collect();
        let tree = MerkleTree::build(&chunks);
        let path = tree.extract_merkle_path(99);
        assert!(path.is_empty());
    }

    #[test]
    fn deterministic_trees() {
        let chunks: Vec<Vec<u8>> = vec![b"fixed".to_vec(), b"data".to_vec()];
        let tree1 = MerkleTree::build(&chunks);
        let tree2 = MerkleTree::build(&chunks);
        assert_eq!(tree1.root(), tree2.root());
    }

    #[test]
    fn verify_inclusion_proof_matches_extract() {
        let chunks: Vec<Vec<u8>> = (0..8).map(|i| vec![i as u8; 64]).collect();
        let tree = MerkleTree::build(&chunks);
        let root = tree.root();

        // Extract path and verify it for each leaf.
        for leaf_idx in 0..8u64 {
            let leaf_hash = *blake3::hash(&chunks[leaf_idx as usize]).as_bytes();
            let path = tree.extract_merkle_path(leaf_idx);

            assert!(
                MerkleTree::verify_inclusion_proof(
                    &root,
                    &leaf_hash,
                    leaf_idx,
                    &path,
                ),
                "leaf {leaf_idx} should verify"
            );
        }
    }

    #[test]
    fn verify_inclusion_proof_rejects_wrong_leaf() {
        let chunks: Vec<Vec<u8>> = (0..4).map(|i| vec![i as u8; 64]).collect();
        let tree = MerkleTree::build(&chunks);
        let root = tree.root();
        let path = tree.extract_merkle_path(0);

        // Use a wrong leaf hash.
        let wrong_leaf = [0xFFu8; 32];
        assert!(
            !MerkleTree::verify_inclusion_proof(&root, &wrong_leaf, 0, &path),
            "wrong leaf should not verify"
        );
    }

    #[test]
    fn verify_inclusion_proof_rejects_wrong_index() {
        let chunks: Vec<Vec<u8>> = (0..4).map(|i| vec![i as u8; 64]).collect();
        let tree = MerkleTree::build(&chunks);
        let root = tree.root();

        // Path for leaf 0, but claim it's leaf 2.
        let path = tree.extract_merkle_path(0);
        let leaf_hash = *blake3::hash(&chunks[0]).as_bytes();

        assert!(
            !MerkleTree::verify_inclusion_proof(&root, &leaf_hash, 2, &path),
            "wrong leaf index should not verify"
        );
    }
}
