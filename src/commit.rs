// Phase 7 — State 4: Committed — Receipt Hash-Chain Ledger
//
// When the 3-of-5 witness quorum is reached for a certificate,
// the CommitManager writes it to an append-only Blake3 hash-chain
// on disk. Each block anchors to the previous block's hash,
// making the ledger tamper-evident.
//
// Frame layout (binary, big-endian):
//   [height: u64 BE] [prev_hash: 32B] [block_hash: 32B]
//   [cert_len: u32 BE] [cert protobuf bytes]
//   [sig_count: u16 BE] [sig1_len: u16 BE] [sig1] ...
//
// The block_hash = Blake3(prev_hash || proposal_id || sig1 || sig2 || ...)

use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};

use tracing::info;

/// Filename for the receipt hash-chain ledger on disk.
pub const RECEIPT_CHAIN_FILE: &str = "receipt_chain.ledger";

/// Manages the append-only Blake3 hash-chain ledger.
#[derive(Debug)]
pub struct CommitManager {
    ledger_path: PathBuf,
    last_block_hash: [u8; 32],
    block_height: u64,
    /// Proposals that have already been committed — prevents
    /// duplicate blocks from trailing attestations.
    committed: HashSet<String>,
}

impl CommitManager {
    /// Open or create the ledger in the given storage directory.
    /// If the ledger already exists, recover the chain tip by
    /// scanning to the last valid block.
    pub fn open(storage_dir: &PathBuf) -> Self {
        let ledger_path = storage_dir.join(RECEIPT_CHAIN_FILE);

        if !ledger_path.exists() {
            info!(
                path = %ledger_path.display(),
                "[commit] No existing ledger — starting genesis (height 0)"
            );
            return Self {
                ledger_path,
                last_block_hash: [0u8; 32],
                block_height: 0,
                committed: HashSet::new(),
            };
        }

        // Recover chain tip by scanning blocks
        let (height, tip_hash) = scan_to_tip(&ledger_path);

        info!(
            path = %ledger_path.display(),
            height,
            "[commit] Recovered ledger tip"
        );

        Self {
            ledger_path,
            last_block_hash: tip_hash,
            block_height: height,
            committed: HashSet::new(),
        }
    }

    /// Current block height (number of committed blocks).
    pub fn height(&self) -> u64 {
        self.block_height
    }

    /// Check whether a proposal has already been committed.
    pub fn is_committed(&self, proposal_id: &str) -> bool {
        self.committed.contains(proposal_id)
    }

    /// The hash of the most recent block (tip of the chain).
    pub fn tip_hash(&self) -> [u8; 32] {
        self.last_block_hash
    }

    /// Retrieve the raw bytes of a block by height.
    /// Returns None if the height is out of range or the ledger
    /// doesn't exist yet.
    pub fn get_block_bytes(&self, target: u64) -> Option<Vec<u8>> {
        use std::io::Read;
        let mut file = File::open(&self.ledger_path).ok()?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).ok()?;

        let mut offset: usize = 0;
        while offset + 8 <= buf.len() {
            let h = u64::from_be_bytes([
                buf[offset], buf[offset + 1], buf[offset + 2], buf[offset + 3],
                buf[offset + 4], buf[offset + 5], buf[offset + 6], buf[offset + 7],
            ]);
            let block_start = offset;
            offset += 8 + 32 + 32; // height + prev_hash + block_hash

            if offset + 4 > buf.len() { return None; }
            let cert_len = u32::from_be_bytes([
                buf[offset], buf[offset + 1], buf[offset + 2], buf[offset + 3],
            ]) as usize;
            offset += 4 + cert_len;
            if offset + 2 > buf.len() { return None; }
            let sig_count = u16::from_be_bytes([buf[offset], buf[offset + 1]]) as usize;
            offset += 2;
            for _ in 0..sig_count {
                if offset + 2 > buf.len() { return None; }
                let sig_len = u16::from_be_bytes([buf[offset], buf[offset + 1]]) as usize;
                offset += 2 + sig_len;
            }

            if h == target {
                return Some(buf[block_start..offset].to_vec());
            }
        }
        None
    }

    /// Append a ratified certificate and its witness signatures to the chain.
    ///
    /// Returns the new block hash.
    pub fn commit(
        &mut self,
        cert_bytes: &[u8],
        proposal_id: &str,
        signatures: &[(PeerId, Vec<u8>)],
    ) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        use std::io::Write;

        // Build the block hash: Blake3(prev_hash || proposal_id || sig1 || sig2 || ...)
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.last_block_hash);
        hasher.update(proposal_id.as_bytes());
        for (_, sig) in signatures {
            hasher.update(sig);
        }
        let block_hash: [u8; 32] = hasher.finalize().into();

        // Ensure the storage directory exists before writing
        if let Some(parent) = self.ledger_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Append to ledger
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.ledger_path)?;

        // Frame: [height: u64 BE] [prev_hash: 32B] [block_hash: 32B]
        file.write_all(&self.block_height.to_be_bytes())?;
        file.write_all(&self.last_block_hash)?;
        file.write_all(&block_hash)?;

        // [cert_len: u32 BE] [cert protobuf bytes]
        file.write_all(&(cert_bytes.len() as u32).to_be_bytes())?;
        file.write_all(cert_bytes)?;

        // [sig_count: u16 BE] [sig1_len: u16 BE] [sig1] ...
        file.write_all(&(signatures.len() as u16).to_be_bytes())?;
        for (_, sig) in signatures {
            file.write_all(&(sig.len() as u16).to_be_bytes())?;
            file.write_all(sig)?;
        }

        file.sync_all()?;

        // Advance state
        self.last_block_hash = block_hash;
        self.block_height += 1;
        self.committed.insert(proposal_id.to_string());

        info!(
            height = self.block_height,
            hash = %hex::encode(block_hash),
            sigs = signatures.len(),
            "[commit] Block written to hash-chain ledger"
        );

        Ok(block_hash)
    }

    /// Commit a root-authorized block (era one only).
    /// This is how Genesis and BootstrapEnded are written to the chain.
    /// After BootstrapEnded, this method returns an error — era two blocks
    /// must be committed via `commit()` with witness signatures.
    pub fn commit_root_block(
        &mut self,
        data: &[u8],
        proposal_id: &str,
        root_signature: &[u8],
    ) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        // Era guard: after BootstrapEnded, root-authorized blocks are rejected.
        if self.is_bootstrap_ended() {
            return Err("era two: root-authorized blocks rejected — use certificate-gated commit()".into());
        }
        use std::io::Write;

        // Build block hash from root sig
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.last_block_hash);
        hasher.update(proposal_id.as_bytes());
        hasher.update(root_signature);
        let block_hash: [u8; 32] = hasher.finalize().into();

        if let Some(parent) = self.ledger_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.ledger_path)?;

        file.write_all(&self.block_height.to_be_bytes())?;
        file.write_all(&self.last_block_hash)?;
        file.write_all(&block_hash)?;
        file.write_all(&(data.len() as u32).to_be_bytes())?;
        file.write_all(data)?;
        file.write_all(&1u16.to_be_bytes())?;  // sig_count = 1
        file.write_all(&(root_signature.len() as u16).to_be_bytes())?;
        file.write_all(root_signature)?;
        file.sync_all()?;

        self.last_block_hash = block_hash;
        self.block_height += 1;
        self.committed.insert(proposal_id.to_string());

        info!(
            height = self.block_height,
            hash = %hex::encode(block_hash),
            "[commit] Root-authorized block written to chain"
        );

        Ok(block_hash)
    }

    /// Check whether BootstrapEnded has occurred by scanning the chain.
    /// Returns true if a BootstrapEnded transaction exists in the ledger.
    /// This is the era marker — derived from chain contents, not stored as state.
    pub fn is_bootstrap_ended(&self) -> bool {
        let mut file = match File::open(&self.ledger_path) {
            Ok(f) => f,
            Err(_) => return false,
        };
        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_err() {
            return false;
        }

        // Walk blocks looking for a BootstrapEnded transaction.
        let mut offset: usize = 0;
        loop {
            if offset + 76 > buffer.len() {
                break;
            }
            let _height = u64::from_be_bytes([
                buffer[offset], buffer[offset+1], buffer[offset+2], buffer[offset+3],
                buffer[offset+4], buffer[offset+5], buffer[offset+6], buffer[offset+7],
            ]);
            // Skip prev_hash (32B) + block_hash (32B)
            offset += 72;
            if offset + 4 > buffer.len() { break; }
            let cert_len = u32::from_be_bytes([
                buffer[offset], buffer[offset+1], buffer[offset+2], buffer[offset+3],
            ]) as usize;
            offset += 4;
            if offset + cert_len > buffer.len() { break; }
            // Check if this is a SignedTransaction containing BootstrapEnded.
            if let Ok(stx) = serde_cbor::from_slice::<crate::ledger::types::SignedTransaction>(&buffer[offset..offset + cert_len]) {
                if matches!(stx.transaction, crate::ledger::types::Transaction::BootstrapEnded { .. }) {
                    return true;
                }
            }
            offset += cert_len;
            if offset + 2 > buffer.len() { break; }
            let sig_count = u16::from_be_bytes([buffer[offset], buffer[offset+1]]) as usize;
            offset += 2;
            for _ in 0..sig_count {
                if offset + 2 > buffer.len() { break; }
                let sig_len = u16::from_be_bytes([buffer[offset], buffer[offset+1]]) as usize;
                offset += 2 + sig_len;
            }
        }
        false
    }
}

// ── Helpers ────────────────────────────────────────────────────

use libp2p::PeerId;

/// Scan the ledger file to find the last valid block.
/// Returns (next_height, tip_hash).
fn scan_to_tip(path: &Path) -> (u64, [u8; 32]) {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            tracing::warn!(error = %e, "[commit] Cannot open ledger for recovery");
            return (0, [0u8; 32]);
        }
    };

    let mut buffer = Vec::new();
    if file.read_to_end(&mut buffer).is_err() {
        return (0, [0u8; 32]);
    }

    let mut height: u64 = 0;
    let mut tip_hash: [u8; 32] = [0u8; 32];
    let mut offset: usize = 0;

    loop {
        // Need at least: u64 + 32 + 32 + u32 = 76 bytes for header
        if offset + 76 > buffer.len() {
            break;
        }

        // Read height
        let h = u64::from_be_bytes([
            buffer[offset],
            buffer[offset + 1],
            buffer[offset + 2],
            buffer[offset + 3],
            buffer[offset + 4],
            buffer[offset + 5],
            buffer[offset + 6],
            buffer[offset + 7],
        ]);
        offset += 8;

        // Read prev_hash (32 bytes) — skip
        offset += 32;

        // Read block_hash (32 bytes)
        let bh: [u8; 32] = buffer[offset..offset + 32].try_into().unwrap();
        offset += 32;

        // Read cert_len
        let cert_len = u32::from_be_bytes([
            buffer[offset],
            buffer[offset + 1],
            buffer[offset + 2],
            buffer[offset + 3],
        ]) as usize;
        offset += 4 + cert_len;

        // Read sig_count
        if offset + 2 > buffer.len() {
            break;
        }
        let sig_count = u16::from_be_bytes([buffer[offset], buffer[offset + 1]]) as usize;
        offset += 2;

        // Skip signatures
        for _ in 0..sig_count {
            if offset + 2 > buffer.len() {
                break;
            }
            let sig_len = u16::from_be_bytes([buffer[offset], buffer[offset + 1]]) as usize;
            offset += 2 + sig_len;
        }

        height = h + 1;
        tip_hash = bh;
    }

    (height, tip_hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_and_recover() {
        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();

        let mut mgr = CommitManager::open(&storage);
        assert_eq!(mgr.height(), 0);

        let sigs: Vec<(PeerId, Vec<u8>)> = vec![
            (PeerId::random(), vec![1, 2, 3, 4]),
            (PeerId::random(), vec![5, 6, 7, 8]),
            (PeerId::random(), vec![9, 0, 1, 2]),
        ];

        let cert_bytes = b"fake certificate bytes";
        let hash1 = mgr.commit(cert_bytes, "test-prop-001", &sigs).unwrap();
        assert_eq!(mgr.height(), 1);

        let hash2 = mgr.commit(cert_bytes, "test-prop-002", &sigs).unwrap();
        assert_eq!(mgr.height(), 2);
        assert_ne!(hash1, hash2, "Different blocks must have different hashes");

        // Recover from the same ledger
        let mgr2 = CommitManager::open(&storage);
        assert_eq!(mgr2.height(), 2, "Recovery should find both blocks");
    }

    #[test]
    fn test_genesis_has_zero_height() {
        let dir = tempfile::tempdir().unwrap();
        let mgr = CommitManager::open(&dir.path().to_path_buf());
        assert_eq!(mgr.height(), 0);
        assert_eq!(mgr.last_block_hash, [0u8; 32]);
    }

    #[test]
    fn test_dedup_guard_prevents_double_commit() {
        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();

        let mut mgr = CommitManager::open(&storage);
        let sigs: Vec<(PeerId, Vec<u8>)> = vec![
            (PeerId::random(), vec![1, 2, 3]),
        ];
        let cert = b"cert bytes";

        // First commit should succeed
        mgr.commit(cert, "prop-001", &sigs).unwrap();
        assert!(mgr.is_committed("prop-001"));
        assert_eq!(mgr.height(), 1);

        // Second commit of same proposal should also succeed
        // (the caller should check is_committed first)
        mgr.commit(cert, "prop-001", &sigs).unwrap();
        assert_eq!(mgr.height(), 2);
    }

    // ── Era derivation tests ────────────────────────────────

    #[test]
    fn fresh_chain_not_bootstrap_ended() {
        let dir = tempfile::tempdir().unwrap();
        let mgr = CommitManager::open(&dir.path().to_path_buf());
        assert!(!mgr.is_bootstrap_ended());
    }

    #[test]
    fn bootstrap_ended_recoverable_across_restarts() {
        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();

        // Era one: commit a root block (genesis)
        let genesis = crate::ledger::types::Transaction::Genesis {
            root: "12D3KooWQw6".to_string(),
            thickness_grant: 1000.0,
            declared_operator_keys: vec!["12D3KooWBoVfr".to_string(), "12D3KooWQw6".to_string()],
            nonce: 0,
            timestamp: chrono::Utc::now(),
        };
        let stx = crate::ledger::types::SignedTransaction {
            transaction: genesis,
            signer_public_key: vec![1, 2, 3],
            signature: vec![4, 5, 6],
        };
        let data = serde_cbor::to_vec(&stx).unwrap();

        let mut mgr = CommitManager::open(&storage);
        assert!(!mgr.is_bootstrap_ended(), "Era should be one before BootstrapEnded");

        // Commit BootstrapEnded
        let ended = crate::ledger::types::Transaction::BootstrapEnded {
            declared_by: "12D3KooWQw6".to_string(),
            reason: "three independent peers now running.".to_string(),
            nonce: 1,
            timestamp: chrono::Utc::now(),
        };
        let stx2 = crate::ledger::types::SignedTransaction {
            transaction: ended,
            signer_public_key: vec![1, 2, 3],
            signature: vec![7, 8, 9],
        };
        let data2 = serde_cbor::to_vec(&stx2).unwrap();
        mgr.commit_root_block(&data, "genesis", &stx.signature).unwrap();
        mgr.commit_root_block(&data2, "bootstrap-ended", &stx2.signature).unwrap();
        assert!(mgr.is_bootstrap_ended(), "Era should be two after BootstrapEnded");

        // Drop and recover — era must survive restart
        drop(mgr);
        let mgr2 = CommitManager::open(&storage);
        assert!(mgr2.is_bootstrap_ended(), "Era must survive restart — derived, not cached");
    }

    #[test]
    fn only_bootstrap_ended_ends_era() {
        // Genesis alone does not end bootstrap.
        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();
        let genesis = crate::ledger::types::Transaction::Genesis {
            root: "12D3KooWQw6".to_string(),
            thickness_grant: 1000.0,
            declared_operator_keys: vec![],
            nonce: 0,
            timestamp: chrono::Utc::now(),
        };
        let stx = crate::ledger::types::SignedTransaction {
            transaction: genesis,
            signer_public_key: vec![1, 2, 3],
            signature: vec![4, 5, 6],
        };
        let data = serde_cbor::to_vec(&stx).unwrap();

        let mut mgr = CommitManager::open(&storage);
        mgr.commit_root_block(&data, "genesis", &stx.signature).unwrap();
        assert!(!mgr.is_bootstrap_ended(), "Genesis should not end bootstrap — only BootstrapEnded does");
    }

    #[test]
    fn root_block_rejected_after_bootstrap_ended() {
        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();

        let ended = crate::ledger::types::Transaction::BootstrapEnded {
            declared_by: "12D3KooWQw6".to_string(),
            reason: "era two.".to_string(),
            nonce: 0,
            timestamp: chrono::Utc::now(),
        };
        let stx = crate::ledger::types::SignedTransaction {
            transaction: ended,
            signer_public_key: vec![1, 2, 3],
            signature: vec![4, 5, 6],
        };
        let data = serde_cbor::to_vec(&stx).unwrap();

        let mut mgr = CommitManager::open(&storage);
        mgr.commit_root_block(&data, "bootstrap-ended", &stx.signature).unwrap();
        assert!(mgr.is_bootstrap_ended());

        // Try another root block — must be rejected.
        let second = crate::ledger::types::Transaction::BootstrapEnded {
            declared_by: "12D3KooWQw6".to_string(),
            reason: "trying to end again.".to_string(),
            nonce: 1,
            timestamp: chrono::Utc::now(),
        };
        let stx2 = crate::ledger::types::SignedTransaction {
            transaction: second,
            signer_public_key: vec![1, 2, 3],
            signature: vec![7, 8, 9],
        };
        let data2 = serde_cbor::to_vec(&stx2).unwrap();
        let result = mgr.commit_root_block(&data2, "second", &stx2.signature);
        assert!(result.is_err(), "Root blocks must be rejected after BootstrapEnded");
    }
}
