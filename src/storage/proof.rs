// ── storage/proof.rs — target-side proof generation ─────────
//
// When a node receives a StorageChallenge, the ProofEngine:
//   1. Locates the chunk on disk (non-blocking via spawn_blocking)
//   2. Computes blake3(chunk_bytes || salt)
//   3. Builds the Merkle inclusion path from chunk to resource root
//
// Disk I/O is deliberately offloaded from the main async event loop
// because the Pi 5 runs on a lean quad-core — blocking the event
// loop on a file seek would drop gossipsub heartbeats.

use std::path::Path;

use super::merkle::MerkleTree;

/// Errors that can occur during proof generation.
#[derive(Debug, thiserror::Error)]
pub enum ProofError {
    #[error("resource not found on local disk: {0}")]
    ResourceNotFound(String),
    #[error("disk I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("chunk index {0} out of bounds (resource has {1} chunks)")]
    ChunkOutOfBounds(u64, u64),
}

/// The result of a successful proof generation.
#[derive(Debug)]
pub struct StorageProofResult {
    /// blake3(chunk_bytes || salt)
    pub salted_hash: [u8; 32],
    /// Merkle inclusion proof — sibling hashes from leaf to root.
    pub merkle_proof: Vec<Vec<u8>>,
}

/// Generates storage proofs from on-disk resources.
///
/// All public methods are synchronous — the caller is responsible
/// for wrapping them in `tokio::task::spawn_blocking`.
pub struct ProofEngine;

impl ProofEngine {
    /// Generate a complete storage proof for a challenged chunk.
    ///
    /// # Arguments
    ///
    /// * `storage_dir` — root directory where resource files live.
    /// * `resource_id` — Blake3 hash of the full resource (the Merkle root).
    /// * `chunk_index` — which chunk to prove possession of.
    /// * `chunk_size` — size of each chunk in bytes (default 1 MiB).
    /// * `salt` — epoch-derived salt from the challenge.
    ///
    /// # Errors
    ///
    /// Returns `ProofError::ResourceNotFound` if the resource file
    /// doesn't exist on this node.  Returns `ChunkOutOfBounds` if
    /// the chunk_index exceeds the file's length.
    pub fn generate_storage_proof(
        storage_dir: &Path,
        resource_id: &[u8; 32],
        chunk_index: u64,
        chunk_size: usize,
        salt: &[u8; 32],
    ) -> Result<StorageProofResult, ProofError> {
        // 1. Map resource_id → filename (hex-encoded).
        let file_name = hex::encode(resource_id);
        let file_path = storage_dir.join(&file_name);

        if !file_path.exists() {
            return Err(ProofError::ResourceNotFound(file_name));
        }

        // 2. Read the entire file to build the Merkle tree and
        //    extract the target chunk.  For production, we'd
        //    memory-map, but for Phase 6 this is correct and simple.
        let file_data = std::fs::read(&file_path)?;
        let total_chunks =
            (file_data.len() + chunk_size - 1) / chunk_size;

        if chunk_index as usize >= total_chunks {
            return Err(ProofError::ChunkOutOfBounds(
                chunk_index,
                total_chunks as u64,
            ));
        }

        // 3. Extract the target chunk bytes.
        let start = (chunk_index as usize) * chunk_size;
        let end = std::cmp::min(start + chunk_size, file_data.len());
        let chunk_bytes = &file_data[start..end];

        // 4. Compute salted hash: blake3(chunk_bytes || salt).
        let mut hasher = blake3::Hasher::new();
        hasher.update(chunk_bytes);
        hasher.update(salt);
        let salted_hash = *hasher.finalize().as_bytes();

        // 5. Build Merkle tree from all chunks and extract path.
        let chunks: Vec<Vec<u8>> = file_data
            .chunks(chunk_size)
            .map(|c| c.to_vec())
            .collect();
        let tree = MerkleTree::build(&chunks);
        let merkle_proof = tree.extract_merkle_path(chunk_index);

        Ok(StorageProofResult {
            salted_hash,
            merkle_proof,
        })
    }

    /// Create a storage directory and write a resource for testing.
    ///
    /// Returns the resource_id (blake3 hash of the data).
    pub fn store_resource(
        storage_dir: &Path,
        data: &[u8],
    ) -> Result<[u8; 32], std::io::Error> {
        let resource_id = *blake3::hash(data).as_bytes();
        let file_name = hex::encode(resource_id);
        let file_path = storage_dir.join(&file_name);

        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&file_path, data)?;

        Ok(resource_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn proof_for_stored_resource() {
        let dir = TempDir::new().unwrap();
        let data = b"Hello, Lattice storage verification! This is test data that spans multiple chunks to exercise the Merkle tree properly.";
        let resource_id = ProofEngine::store_resource(dir.path(), data).unwrap();

        // Challenge chunk 0 with a known salt.
        let salt = [0x42u8; 32];
        let result = ProofEngine::generate_storage_proof(
            dir.path(),
            &resource_id,
            0,          // chunk_index
            32,         // small chunk size for testing
            &salt,
        )
        .unwrap();

        // Verify salted hash: blake3(first 32 bytes || salt).
        let mut expected = blake3::Hasher::new();
        expected.update(&data[0..32]);
        expected.update(&salt);
        assert_eq!(
            result.salted_hash,
            *expected.finalize().as_bytes()
        );

        // Verify Merkle path exists.
        assert!(!result.merkle_proof.is_empty());
    }

    #[test]
    fn missing_resource_errors() {
        let dir = TempDir::new().unwrap();
        let fake_id = [0xFFu8; 32];
        let salt = [0u8; 32];

        let result = ProofEngine::generate_storage_proof(
            dir.path(),
            &fake_id,
            0,
            64 * 1024,
            &salt,
        );

        assert!(result.is_err());
        match result.unwrap_err() {
            ProofError::ResourceNotFound(_) => {} // expected
            e => panic!("expected ResourceNotFound, got {:?}", e),
        }
    }

    #[test]
    fn out_of_bounds_chunk_errors() {
        let dir = TempDir::new().unwrap();
        let data = b"tiny";
        let resource_id = ProofEngine::store_resource(dir.path(), data).unwrap();
        let salt = [0u8; 32];

        let result = ProofEngine::generate_storage_proof(
            dir.path(),
            &resource_id,
            99, // way past the end
            64 * 1024,
            &salt,
        );

        assert!(result.is_err());
        match result.unwrap_err() {
            ProofError::ChunkOutOfBounds(99, _) => {} // expected
            e => panic!("expected ChunkOutOfBounds, got {:?}", e),
        }
    }

    #[test]
    fn different_salts_produce_different_hashes() {
        let dir = TempDir::new().unwrap();
        let data = b"same data, different salt";
        let resource_id = ProofEngine::store_resource(dir.path(), data).unwrap();

        let salt_a = [0xAAu8; 32];
        let salt_b = [0xBBu8; 32];

        let proof_a = ProofEngine::generate_storage_proof(
            dir.path(), &resource_id, 0, 64 * 1024, &salt_a,
        )
        .unwrap();
        let proof_b = ProofEngine::generate_storage_proof(
            dir.path(), &resource_id, 0, 64 * 1024, &salt_b,
        )
        .unwrap();

        assert_ne!(proof_a.salted_hash, proof_b.salted_hash);
    }
}
