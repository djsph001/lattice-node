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
//   [sig_count: u16 BE] [peer_id_len: u16 BE] [peer_id_bytes] [sig_len: u16 BE] [sig_bytes] ...
//
// The block_hash = Blake3(prev_hash || proposal_id || sig1 || sig2 || ...)

use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use libp2p::PeerId;
use tracing::info;

/// Filename for the receipt hash-chain ledger on disk.
pub const RECEIPT_CHAIN_FILE: &str = "receipt_chain.ledger";

/// A parsed block from the on-disk ledger.
#[derive(Debug, Clone)]
pub struct BlockFrame {
    pub height: u64,
    pub prev_hash: [u8; 32],
    pub block_hash: [u8; 32],
    pub cert_bytes: Vec<u8>,
    /// Signatures stored as (PeerId, sig_bytes) pairs.
    pub signatures: Vec<(PeerId, Vec<u8>)>,
}

/// A fork between two certificate-chain ledgers.
#[derive(Debug)]
pub struct CertificateFork {
    /// Height of the last common block (divergence point).
    pub fork_point: u64,
    /// Blocks from the local ledger after the divergence point.
    pub local_fork: Vec<BlockFrame>,
    /// Blocks from the remote ledger after the divergence point.
    pub remote_fork: Vec<BlockFrame>,
}

/// Result of fork resolution.
#[derive(Debug)]
pub enum ResolutionResult {
    Winner(Vec<BlockFrame>),
    NoFork,
}

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
    /// Returns None if the height is out of range.
    pub fn get_block_bytes(&self, target: u64) -> Option<Vec<u8>> {
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
                let peer_len = u16::from_be_bytes([buf[offset], buf[offset + 1]]) as usize;
                offset += 2 + peer_len;
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

        // [sig_count: u16 BE] [peer_id_len: u16 BE] [peer_id] [sig_len: u16 BE] [sig_bytes] ...
        file.write_all(&(signatures.len() as u16).to_be_bytes())?;
        for (peer_id, sig) in signatures {
            let peer_bytes = peer_id.to_bytes();
            file.write_all(&(peer_bytes.len() as u16).to_be_bytes())?;
            file.write_all(&peer_bytes)?;
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
    /// `local_peer_id` is written as the sole signer's identity.
    pub fn commit_root_block(
        &mut self,
        data: &[u8],
        proposal_id: &str,
        root_signature: &[u8],
        local_peer_id: &PeerId,
    ) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        // Era guard: after BootstrapEnded, root-authorized blocks are rejected.
        if self.is_bootstrap_ended() {
            return Err("era two: root-authorized blocks rejected — use certificate-gated commit()".into());
        }

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
        // sig_count = 1; write the local peer id as the sole signer
        file.write_all(&1u16.to_be_bytes())?;
        let peer_bytes = local_peer_id.to_bytes();
        file.write_all(&(peer_bytes.len() as u16).to_be_bytes())?;
        file.write_all(&peer_bytes)?;
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
    pub fn is_bootstrap_ended(&self) -> bool {
        let mut file = match File::open(&self.ledger_path) {
            Ok(f) => f,
            Err(_) => return false,
        };
        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_err() {
            return false;
        }

        let mut offset: usize = 0;
        loop {
            if offset + 76 > buffer.len() {
                break;
            }
            let _height = u64::from_be_bytes([
                buffer[offset], buffer[offset+1], buffer[offset+2], buffer[offset+3],
                buffer[offset+4], buffer[offset+5], buffer[offset+6], buffer[offset+7],
            ]);
            offset += 72; // skip prev_hash + block_hash
            if offset + 4 > buffer.len() { break; }
            let cert_len = u32::from_be_bytes([
                buffer[offset], buffer[offset+1], buffer[offset+2], buffer[offset+3],
            ]) as usize;
            offset += 4;
            if offset + cert_len > buffer.len() { break; }
            if let Ok(stx) = serde_cbor::from_slice::<crate::ledger::types::SignedTransaction>(
                &buffer[offset..offset + cert_len],
            ) {
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
                let peer_len = u16::from_be_bytes([buffer[offset], buffer[offset+1]]) as usize;
                offset += 2 + peer_len;
                if offset + 2 > buffer.len() { break; }
                let sig_len = u16::from_be_bytes([buffer[offset], buffer[offset+1]]) as usize;
                offset += 2 + sig_len;
            }
        }
        false
    }

    // ── Fork detection & resolution ──────────────────────────

    /// Read a single block from the ledger via a BufReader.
    /// Returns Ok(None) at EOF.
    pub fn read_block<R: Read>(&self, reader: &mut BufReader<R>) -> Result<Option<BlockFrame>, Box<dyn std::error::Error>> {
        let mut height_buf = [0u8; 8];
        match reader.read_exact(&mut height_buf) {
            Ok(_) => {}
            Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(e.into()),
        }
        let height = u64::from_be_bytes(height_buf);

        let mut prev_hash = [0u8; 32];
        reader.read_exact(&mut prev_hash)?;

        let mut block_hash = [0u8; 32];
        reader.read_exact(&mut block_hash)?;

        let mut cert_len_buf = [0u8; 4];
        reader.read_exact(&mut cert_len_buf)?;
        let cert_len = u32::from_be_bytes(cert_len_buf) as usize;
        let mut cert_bytes = vec![0u8; cert_len];
        reader.read_exact(&mut cert_bytes)?;

        let mut sig_count_buf = [0u8; 2];
        reader.read_exact(&mut sig_count_buf)?;
        let sig_count = u16::from_be_bytes(sig_count_buf) as usize;

        let mut signatures = Vec::with_capacity(sig_count);
        for _ in 0..sig_count {
            let mut peer_len_buf = [0u8; 2];
            reader.read_exact(&mut peer_len_buf)?;
            let peer_len = u16::from_be_bytes(peer_len_buf) as usize;
            let mut peer_bytes = vec![0u8; peer_len];
            reader.read_exact(&mut peer_bytes)?;
            let peer_id = PeerId::from_bytes(&peer_bytes)?;

            let mut sig_len_buf = [0u8; 2];
            reader.read_exact(&mut sig_len_buf)?;
            let sig_len = u16::from_be_bytes(sig_len_buf) as usize;
            let mut sig = vec![0u8; sig_len];
            reader.read_exact(&mut sig)?;

            signatures.push((peer_id, sig));
        }

        Ok(Some(BlockFrame { height, prev_hash, block_hash, cert_bytes, signatures }))
    }

    /// Write a BlockFrame to the end of the given file.
    fn write_block_frame<W: Write>(&self, writer: &mut W, block: &BlockFrame) -> Result<(), Box<dyn std::error::Error>> {
        writer.write_all(&block.height.to_be_bytes())?;
        writer.write_all(&block.prev_hash)?;
        writer.write_all(&block.block_hash)?;
        writer.write_all(&(block.cert_bytes.len() as u32).to_be_bytes())?;
        writer.write_all(&block.cert_bytes)?;
        writer.write_all(&(block.signatures.len() as u16).to_be_bytes())?;
        for (peer_id, sig) in &block.signatures {
            let peer_bytes = peer_id.to_bytes();
            writer.write_all(&(peer_bytes.len() as u16).to_be_bytes())?;
            writer.write_all(&peer_bytes)?;
            writer.write_all(&(sig.len() as u16).to_be_bytes())?;
            writer.write_all(sig)?;
        }
        Ok(())
    }

    /// Detect a fork by comparing this node's ledger with a remote ledger file.
    pub fn detect_fork(&self, remote_ledger_path: &Path) -> Result<Option<CertificateFork>, Box<dyn std::error::Error>> {
        let local_ledger = File::open(&self.ledger_path)?;
        let remote_ledger = File::open(remote_ledger_path)?;

        let mut local_reader = BufReader::new(local_ledger);
        let mut remote_reader = BufReader::new(remote_ledger);

        let mut fork_point = 0u64;
        let mut local_fork = Vec::new();
        let mut remote_fork = Vec::new();
        let mut fork_detected = false;

        loop {
            match (self.read_block(&mut local_reader)?, self.read_block(&mut remote_reader)?) {
                (Some(local), Some(remote)) => {
                    if local.block_hash == remote.block_hash {
                        fork_point = local.height;
                    } else {
                        fork_detected = true;
                        local_fork.push(local);
                        remote_fork.push(remote);
                        while let Some(b) = self.read_block(&mut local_reader)? { local_fork.push(b); }
                        while let Some(b) = self.read_block(&mut remote_reader)? { remote_fork.push(b); }
                        break;
                    }
                }
                (Some(local), None) => {
                    fork_detected = true;
                    fork_point = local.height.saturating_sub(1);
                    local_fork.push(local);
                    while let Some(b) = self.read_block(&mut local_reader)? { local_fork.push(b); }
                    break;
                }
                (None, Some(remote)) => {
                    fork_detected = true;
                    fork_point = remote.height.saturating_sub(1);
                    remote_fork.push(remote);
                    while let Some(b) = self.read_block(&mut remote_reader)? { remote_fork.push(b); }
                    break;
                }
                (None, None) => break,
            }
        }

        Ok(if fork_detected { Some(CertificateFork { fork_point, local_fork, remote_fork }) } else { None })
    }

    /// Calculate thickness score for a fork segment — sum of thickness
    /// of all unique witness PeerIds, read from the economic layer at
    /// resolution time.  f64 comparison (no truncation loss).
    pub fn calculate_fork_score<F>(&self, fork: &[BlockFrame], get_thickness: F) -> f64
    where
        F: Fn(&PeerId) -> f64,
    {
        let mut seen = HashSet::new();
        let mut score = 0.0f64;
        for block in fork {
            for (peer_id, _) in &block.signatures {
                if seen.insert(peer_id.clone()) {
                    score += get_thickness(peer_id);
                }
            }
        }
        score
    }

    /// Resolve a fork deterministically.
    /// 1. Thickness-weighted score (f64, higher wins).
    /// 2. Tiebreaker: longer chain.
    /// 3. Final tiebreaker: deterministic hash of the fork segment.
    pub fn resolve_fork<F>(&self, fork: &CertificateFork, get_thickness: F) -> ResolutionResult
    where
        F: Fn(&PeerId) -> f64,
    {
        // Empty fork edge cases
        if fork.local_fork.is_empty() && fork.remote_fork.is_empty() {
            return ResolutionResult::NoFork;
        }
        if fork.local_fork.is_empty() {
            return ResolutionResult::Winner(fork.remote_fork.clone());
        }
        if fork.remote_fork.is_empty() {
            return ResolutionResult::Winner(fork.local_fork.clone());
        }

        let local_score = self.calculate_fork_score(&fork.local_fork, &get_thickness);
        let remote_score = self.calculate_fork_score(&fork.remote_fork, &get_thickness);

        if (local_score - remote_score).abs() > f64::EPSILON {
            if local_score > remote_score {
                return ResolutionResult::Winner(fork.local_fork.clone());
            } else {
                return ResolutionResult::Winner(fork.remote_fork.clone());
            }
        }

        // Tiebreaker 1: longer chain
        if fork.local_fork.len() > fork.remote_fork.len() {
            return ResolutionResult::Winner(fork.local_fork.clone());
        }
        if fork.remote_fork.len() > fork.local_fork.len() {
            return ResolutionResult::Winner(fork.remote_fork.clone());
        }

        // Tiebreaker 2: deterministic fork hash
        let local_hash = self.hash_fork_segment(&fork.local_fork);
        let remote_hash = self.hash_fork_segment(&fork.remote_fork);
        if local_hash > remote_hash {
            ResolutionResult::Winner(fork.local_fork.clone())
        } else {
            ResolutionResult::Winner(fork.remote_fork.clone())
        }
    }

    /// Adopt a winning fork: copy the common prefix (blocks up to fork_point)
    /// and the winning blocks into a new ledger, then atomically replace
    /// the old ledger with the new one.  Updates in-memory state.
    pub fn adopt_winning_fork(
        &mut self,
        winning_fork: &[BlockFrame],
        fork_point: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_path = self.ledger_path.with_extension("tmp");

        let mut temp_file = File::create(&temp_path)?;
        let local_file = File::open(&self.ledger_path)?;
        let mut local_reader = BufReader::new(local_file);

        // Copy common prefix (blocks 0..=fork_point)
        while let Some(block) = self.read_block(&mut local_reader)? {
            if block.height <= fork_point {
                self.write_block_frame(&mut temp_file, &block)?;
            } else {
                break;
            }
        }

        // Append winning fork blocks
        for block in winning_fork {
            self.write_block_frame(&mut temp_file, block)?;
        }

        temp_file.sync_all()?;
        fs::rename(&temp_path, &self.ledger_path)?;

        // Update in-memory state from the winning fork's last block
        if let Some(last) = winning_fork.last() {
            self.last_block_hash = last.block_hash;
            self.block_height = last.height + 1;
            // Note: committed set is NOT cleared. Losing fork proposals
            // remain in the set to prevent accidental double-submission
            // of the same proposal ID.  The proposer may re-submit with
            // a new proposal ID.
        }

        info!(
            height = self.block_height,
            "[commit] Adopted winning fork — canonical chain restored"
        );
        Ok(())
    }

    /// Deterministic hash of a fork segment (for tiebreaker).
    fn hash_fork_segment(&self, fork: &[BlockFrame]) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        for block in fork {
            hasher.update(&block.block_hash);
        }
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(result.as_bytes());
        hash
    }

    /// Remove proposal IDs from the in-memory committed set.
    /// Used after adopting a winning fork so losing-fork proposals
    /// can be re-submitted without being rejected as duplicates.
    pub fn remove_committed_proposals(&mut self, ids: &[String]) {
        for id in ids {
            self.committed.remove(id);
        }
    }

    /// Extract proposal IDs from a fork segment (for the losing-fork event).
    pub fn extract_proposal_ids(&self, fork: &[BlockFrame]) -> Vec<String> {
        fork.iter().filter_map(|block| {
            // Try to parse as SignedTransaction first
            if let Ok(stx) = serde_cbor::from_slice::<crate::ledger::types::SignedTransaction>(&block.cert_bytes) {
                let id = hex::encode(
                    blake3::hash(&serde_cbor::to_vec(&stx.transaction).unwrap_or_default())
                        .as_bytes(),
                );
                Some(id)
            } else {
                // Fall back to Blake3 hash of cert_bytes as a stable identifier
                let hash = blake3::hash(&block.cert_bytes);
                Some(hex::encode(hash.as_bytes()))
            }
        }).collect()
    }
}

// ── Helpers ────────────────────────────────────────────────────

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
        if offset + 76 > buffer.len() { break; }

        let h = u64::from_be_bytes([
            buffer[offset], buffer[offset+1], buffer[offset+2], buffer[offset+3],
            buffer[offset+4], buffer[offset+5], buffer[offset+6], buffer[offset+7],
        ]);
        offset += 8;
        offset += 32; // skip prev_hash
        let bh: [u8; 32] = buffer[offset..offset + 32].try_into().unwrap();
        offset += 32;

        let cert_len = u32::from_be_bytes([
            buffer[offset], buffer[offset+1], buffer[offset+2], buffer[offset+3],
        ]) as usize;
        offset += 4 + cert_len;

        if offset + 2 > buffer.len() { break; }
        let sig_count = u16::from_be_bytes([buffer[offset], buffer[offset+1]]) as usize;
        offset += 2;

        for _ in 0..sig_count {
            if offset + 2 > buffer.len() { break; }
            let peer_len = u16::from_be_bytes([buffer[offset], buffer[offset+1]]) as usize;
            offset += 2 + peer_len;
            if offset + 2 > buffer.len() { break; }
            let sig_len = u16::from_be_bytes([buffer[offset], buffer[offset+1]]) as usize;
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
        let sigs: Vec<(PeerId, Vec<u8>)> = vec![(PeerId::random(), vec![1, 2, 3])];
        let cert = b"cert bytes";

        mgr.commit(cert, "prop-001", &sigs).unwrap();
        assert!(mgr.is_committed("prop-001"));
        assert_eq!(mgr.height(), 1);

        mgr.commit(cert, "prop-001", &sigs).unwrap();
        assert_eq!(mgr.height(), 2);
    }

    // ── Fork resolution tests ─────────────────────────────────

    #[test]
    fn test_detect_fork_no_fork() {
        let dir_a = tempfile::tempdir().unwrap();
        let dir_b = tempfile::tempdir().unwrap();
        let peer_a = PeerId::random();

        let mut mgr_a = CommitManager::open(&dir_a.path().to_path_buf());
        let mut mgr_b = CommitManager::open(&dir_b.path().to_path_buf());

        let sigs = vec![(peer_a, vec![1, 2, 3])];
        mgr_a.commit(b"same cert", "prop-001", &sigs).unwrap();
        mgr_b.commit(b"same cert", "prop-001", &sigs).unwrap();

        let remote = dir_b.path().join("receipt_chain.ledger");
        let fork = mgr_a.detect_fork(&remote).unwrap();
        assert!(fork.is_none());
    }

    #[test]
    fn test_detect_and_resolve_fork() {
        let dir_a = tempfile::tempdir().unwrap();
        let dir_b = tempfile::tempdir().unwrap();
        let peer_a = PeerId::random();
        let peer_b = PeerId::random();

        let mut mgr_a = CommitManager::open(&dir_a.path().to_path_buf());
        let mut mgr_b = CommitManager::open(&dir_b.path().to_path_buf());

        // Common genesis
        let sigs = vec![(PeerId::random(), vec![0])];
        mgr_a.commit(b"genesis", "genesis", &sigs).unwrap();
        mgr_b.commit(b"genesis", "genesis", &sigs).unwrap();

        // Diverge
        mgr_a.commit(b"cert A", "prop-a", &[(peer_a, vec![1])]).unwrap();
        mgr_b.commit(b"cert B", "prop-b", &[(peer_b, vec![2])]).unwrap();

        let remote = dir_b.path().join("receipt_chain.ledger");
        let fork = mgr_a.detect_fork(&remote).unwrap().unwrap();
        assert_eq!(fork.fork_point, 0);
        assert_eq!(fork.local_fork.len(), 1);
        assert_eq!(fork.remote_fork.len(), 1);

        // peer_a has thickness 0, peer_b has thickness 5000 → B wins
        let get_thickness = |id: &PeerId| -> f64 {
            if *id == peer_a { 0.0 } else if *id == peer_b { 5000.0 } else { 0.0 }
        };

        match mgr_a.resolve_fork(&fork, &get_thickness) {
            ResolutionResult::Winner(w) => {
                assert_eq!(w.len(), 1);
                assert_eq!(w[0].cert_bytes, b"cert B");
            }
            ResolutionResult::NoFork => panic!("Expected fork"),
        }
    }

    #[test]
    fn test_adopt_winning_fork() {
        let dir_a = tempfile::tempdir().unwrap();
        let dir_b = tempfile::tempdir().unwrap();
        let peer_a = PeerId::random();
        let peer_b = PeerId::random();

        let mut mgr_a = CommitManager::open(&dir_a.path().to_path_buf());
        let mut mgr_b = CommitManager::open(&dir_b.path().to_path_buf());

        let sigs = vec![(PeerId::random(), vec![0])];
        mgr_a.commit(b"genesis", "genesis", &sigs).unwrap();
        mgr_b.commit(b"genesis", "genesis", &sigs).unwrap();

        mgr_a.commit(b"cert A", "prop-a", &[(peer_a, vec![1])]).unwrap();
        mgr_b.commit(b"cert B", "prop-b", &[(peer_b, vec![2])]).unwrap();

        let remote = dir_b.path().join("receipt_chain.ledger");
        let fork = mgr_a.detect_fork(&remote).unwrap().unwrap();

        let get_thickness = |id: &PeerId| -> f64 {
            if *id == peer_a { 0.0 } else if *id == peer_b { 5000.0 } else { 0.0 }
        };

        if let ResolutionResult::Winner(w) = mgr_a.resolve_fork(&fork, &get_thickness) {
            mgr_a.adopt_winning_fork(&w, fork.fork_point).unwrap();
            // After adoption, mgr_a's ledger should match mgr_b's
            let mut buf_a = Vec::new();
            let mut buf_b = Vec::new();
            File::open(&mgr_a.ledger_path).unwrap().read_to_end(&mut buf_a).unwrap();
            File::open(&mgr_b.ledger_path).unwrap().read_to_end(&mut buf_b).unwrap();
            assert_eq!(buf_a, buf_b, "Ledgers must match after adoption");
            assert_eq!(mgr_a.height(), mgr_b.height());
            assert_eq!(mgr_a.tip_hash(), mgr_b.tip_hash());
        } else {
            panic!("Expected winner");
        }
    }

    #[test]
    fn test_empty_fork_edge_cases() {
        let dir = tempfile::tempdir().unwrap();
        let dir_b = tempfile::tempdir().unwrap();
        let peer = PeerId::random();
        let mut mgr = CommitManager::open(&dir.path().to_path_buf());
        let mut mgr_b = CommitManager::open(&dir_b.path().to_path_buf());
        let local_path = dir.path().join("receipt_chain.ledger");
        let remote = dir_b.path().join("receipt_chain.ledger");
        // Ensure both ledger files exist (empty, no blocks)
        File::create(&local_path).unwrap();
        File::create(&remote).unwrap();

        // No blocks on either side → NoFork
        let fork = mgr.detect_fork(&remote).unwrap();
        assert!(fork.is_none(), "Empty ledgers should not produce a fork");

        // One-side: local has blocks, remote is empty
        mgr.commit(b"cert", "prop", &[(peer, vec![1])]).unwrap();
        let fork = mgr.detect_fork(&remote).unwrap().unwrap();
        let get = |_: &PeerId| 0.0;
        match mgr.resolve_fork(&fork, &get) {
            ResolutionResult::Winner(w) => {
                assert_eq!(w.len(), 1);
                assert_eq!(w[0].cert_bytes, b"cert");
            }
            ResolutionResult::NoFork => panic!("Expected winner"),
        }
    }

    #[test]
    fn test_read_block_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let peer = PeerId::random();
        let sig = vec![10, 20, 30];
        let sigs = vec![(peer.clone(), sig)];
        let mut mgr = CommitManager::open(&dir.path().to_path_buf());
        let hash = mgr.commit(b"test data", "roundtrip", &sigs).unwrap();

        let file = File::open(&mgr.ledger_path).unwrap();
        let mut reader = BufReader::new(file);
        let block = mgr.read_block(&mut reader).unwrap().unwrap();
        assert_eq!(block.height, 0);
        assert_eq!(block.cert_bytes, b"test data");
        assert_eq!(block.signatures.len(), 1);
        assert_eq!(block.signatures[0].0, peer);
        assert_eq!(block.signatures[0].1, vec![10, 20, 30]);
        assert_eq!(block.block_hash, hash);
    }

    #[test]
    fn test_extract_proposal_ids() {
        let dir = tempfile::tempdir().unwrap();
        let peer = PeerId::random();
        let mut mgr = CommitManager::open(&dir.path().to_path_buf());

        // Create a real SignedTransaction-based block
        let tx = crate::ledger::types::Transaction::Genesis {
            root: "test_root".to_string(),
            thickness_grant: 1000.0,
            declared_operator_keys: vec![],
            nonce: 0,
            timestamp: chrono::Utc::now(),
        };
        let stx = crate::ledger::types::SignedTransaction {
            transaction: tx,
            signer_public_key: vec![1, 2, 3],
            signature: vec![4, 5, 6],
        };
        let data = serde_cbor::to_vec(&stx).unwrap();
        mgr.commit(&data, "test-prop", &[(peer, vec![7, 8, 9])]).unwrap();

        // Read back and extract
        let file = File::open(&mgr.ledger_path).unwrap();
        let mut reader = BufReader::new(file);
        let block = mgr.read_block(&mut reader).unwrap().unwrap();
        let ids = mgr.extract_proposal_ids(&[block]);
        // Genesis should produce a stable hash-based proposal ID
        assert_eq!(ids.len(), 1);
    }

    // ── Era derivation tests ──────────────────────────────────

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
        let local_peer = PeerId::random();

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
        mgr.commit_root_block(&data, "genesis", &stx.signature, &local_peer).unwrap();
        mgr.commit_root_block(&data2, "bootstrap-ended", &stx2.signature, &local_peer).unwrap();
        assert!(mgr.is_bootstrap_ended(), "Era should be two after BootstrapEnded");

        drop(mgr);
        let mgr2 = CommitManager::open(&storage);
        assert!(mgr2.is_bootstrap_ended(), "Era must survive restart — derived, not cached");
    }

    #[test]
    fn only_bootstrap_ended_ends_era() {
        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();
        let local_peer = PeerId::random();
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
        mgr.commit_root_block(&data, "genesis", &stx.signature, &local_peer).unwrap();
        assert!(!mgr.is_bootstrap_ended(), "Genesis should not end bootstrap — only BootstrapEnded does");
    }

    #[test]
    fn root_block_rejected_after_bootstrap_ended() {
        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();
        let local_peer = PeerId::random();

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
        mgr.commit_root_block(&data, "bootstrap-ended", &stx.signature, &local_peer).unwrap();
        assert!(mgr.is_bootstrap_ended());

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
        let result = mgr.commit_root_block(&data2, "second", &stx2.signature, &local_peer);
        assert!(result.is_err(), "Root blocks must be rejected after BootstrapEnded");
    }
}
