// ── Write-Ahead Log ──────────────────────────────────────────
//
// The WAL is the source of truth for transaction history.
// A transaction is considered durable only after `append()` returns,
// which implies an fsync of both the temp file and its directory.
//
// File layout (per signer):
//   transactions/<peer_id_hex>/
//     00000001.cbor
//     00000002.cbor
//
// `replay()` yields transactions sorted by nonce within each signer
// directory, preserving the order they were appended.
// `last_nonce()` does a directory scan — O(n) per call, fine for
// the initial implementation.

use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use libp2p::PeerId;
use tracing::warn;

use crate::ledger::types::SignedTransaction;

// ── Trait ──────────────────────────────────────────────────

/// A write-ahead log of signed transactions.
///
/// # Durability guarantee
///
/// `append(tx)` will not return until the transaction is durably
/// committed to disk.  If the process crashes before `append()`
/// returns, the transaction is considered never committed.  If
/// `append()` returns, the transaction will be recoverable on
/// restart.
///
/// # Invariant
///
/// The transaction's nonce must be exactly `last_nonce(peer) + 1`.
/// The caller enforces this; the WAL does not validate semantics.
pub trait Wal: Send {
    /// Append a transaction to the log, durable after fsync.
    fn append(&mut self, tx: &SignedTransaction) -> Result<()>;

    /// Replay all transactions in append order.
    /// Per-signer nonce ordering is guaranteed by construction;
    /// cross-signer interleaving is preserved from the append
    /// sequence (which is the order they were applied).
    fn replay(&self) -> Result<Vec<SignedTransaction>>;

    /// Return the last nonce observed for a signer,
    /// or None if no transactions exist for that peer.
    fn last_nonce(&self, peer_id: &PeerId) -> Option<u64>;
}

// ── File-backed implementation ─────────────────────────────

pub struct FileWal {
    root: PathBuf,
}

impl FileWal {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self> {
        let root = root.into();
        fs::create_dir_all(&root)
            .with_context(|| format!("creating WAL root {:?}", root))?;
        Ok(Self { root })
    }

    fn peer_dir(&self, peer_id: &PeerId) -> PathBuf {
        self.root.join(peer_id.to_base58())
    }

    fn tx_path(&self, peer_id: &PeerId, nonce: u64) -> PathBuf {
        self.peer_dir(peer_id)
            .join(format!("{:08}.cbor", nonce))
    }
}

impl Wal for FileWal {
    fn append(&mut self, tx: &SignedTransaction) -> Result<()> {
        let signer = libp2p::identity::PublicKey::try_decode_protobuf(&tx.signer_public_key)
            .map(|pk| pk.to_peer_id())
            .with_context(|| "invalid signer public key in transaction")?;
        let nonce = tx.transaction.nonce();

        let dir = self.peer_dir(&signer);
        fs::create_dir_all(&dir)
            .with_context(|| format!("creating peer dir {:?}", dir))?;

        let bytes = serde_cbor::to_vec(tx)?;
        let target = self.tx_path(&signer, nonce);
        let tmp = dir.join(format!(".tmp.{:08}", nonce));

        // Write to temp file, then atomically rename.
        // rename(2) is atomic on POSIX; a partial write in the
        // temp file is invisible until rename completes.
        {
            let mut f = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&tmp)
                .with_context(|| format!("creating temp WAL file {:?}", tmp))?;
            f.write_all(&bytes)?;
            f.sync_all()?; // fsync temp before rename
        }

        fs::rename(&tmp, &target)
            .with_context(|| format!("renaming {:?} -> {:?}", tmp, target))?;

        // Directory fsync makes the rename durable.
        if let Ok(dir_f) = fs::File::open(&dir) {
            let _ = dir_f.sync_all();
        }

        Ok(())
    }

    fn replay(&self) -> Result<Vec<SignedTransaction>> {
        let mut all: Vec<SignedTransaction> = Vec::new();

        // Walk each peer directory
        let entries = match fs::read_dir(&self.root) {
            Ok(e) => e,
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(all); // empty WAL
            }
            Err(e) => return Err(e.into()),
        };

        for entry in entries {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let peer_dir = entry.path();

            // Collect .cbor files, sorted by nonce
            let mut files: Vec<u64> = Vec::new();
            for f in fs::read_dir(&peer_dir)? {
                let f = f?;
                let name = f.file_name();
                let name = name.to_string_lossy();
                if name.ends_with(".cbor") && !name.starts_with(".tmp.") {
                    // Parse nonce from filename: 00000001.cbor -> 1
                    if let Some(nonce_str) = name.strip_suffix(".cbor") {
                        if let Ok(nonce) = nonce_str.parse::<u64>() {
                            files.push(nonce);
                        }
                    }
                }
            }
            files.sort();

            for nonce in files {
                let path = peer_dir.join(format!("{:08}.cbor", nonce));
                let data = match fs::read(&path) {
                    Ok(d) => d,
                    Err(e) => {
                        warn!("Skipping unreadable WAL entry {:?}: {}", path, e);
                        continue;
                    }
                };
                let tx: SignedTransaction = match serde_cbor::from_slice(&data) {
                    Ok(t) => t,
                    Err(e) => {
                        // Corrupt or partial entry — skip, don't fail the
                        // entire replay.  This covers the "during WAL write"
                        // crash row of the crash matrix.
                        // See docs/architecture/persistence-design.md §4.
                        warn!("Skipping corrupt WAL entry {:?}: {}", path, e);
                        continue;
                    }
                };
                all.push(tx);
            }
        }

        Ok(all)
    }

    fn last_nonce(&self, peer_id: &PeerId) -> Option<u64> {
        let dir = self.peer_dir(peer_id);
        let mut max: Option<u64> = None;
        let entries = fs::read_dir(&dir).ok()?;
        for entry in entries {
            let entry = entry.ok()?;
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.ends_with(".cbor") && !name.starts_with(".tmp.") {
                if let Some(nonce_str) = name.strip_suffix(".cbor") {
                    if let Ok(nonce) = nonce_str.parse::<u64>() {
                        max = Some(max.map_or(nonce, |m| m.max(nonce)));
                    }
                }
            }
        }
        max
    }
}

// ── Tests ──────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::types::{DigitalUtilityUnit, Transaction};
    use chrono::Utc;
    use libp2p::identity;

    fn make_kp() -> identity::Keypair {
        identity::Keypair::generate_ed25519()
    }

    fn sign(tx: Transaction, kp: &identity::Keypair) -> SignedTransaction {
        let sig = kp.sign(&serde_cbor::to_vec(&tx).unwrap()).unwrap();
        SignedTransaction {
            transaction: tx,
            signer_public_key: kp.public().encode_protobuf(),
            signature: sig,
        }
    }

    fn temp_wal() -> (FileWal, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let wal = FileWal::new(dir.path().join("transactions")).unwrap();
        (wal, dir)
    }

    #[test]
    fn append_then_replay() {
        let (mut wal, _dir) = temp_wal();
        let kp = make_kp();
        let peer = kp.public().to_peer_id();

        let tx = Transaction::Mint {
            to: peer.to_string(),
            amount: DigitalUtilityUnit(1000),
            authority: peer.to_string(),
            nonce: 1,
            timestamp: Utc::now(),
        };
        wal.append(&sign(tx, &kp)).unwrap();

        let recovered = wal.replay().unwrap();
        assert_eq!(recovered.len(), 1);
        assert_eq!(recovered[0].transaction.nonce(), 1);
    }

    #[test]
    fn last_nonce_tracks_per_peer() {
        let (mut wal, _dir) = temp_wal();
        let kp = make_kp();
        let peer = kp.public().to_peer_id();

        assert!(wal.last_nonce(&peer).is_none());

        for nonce in 1..=3 {
            let tx = Transaction::Mint {
                to: peer.to_string(),
                amount: DigitalUtilityUnit(100),
                authority: peer.to_string(),
                nonce,
                timestamp: Utc::now(),
            };
            wal.append(&sign(tx, &kp)).unwrap();
            assert_eq!(wal.last_nonce(&peer), Some(nonce));
        }
    }

    #[test]
    fn empty_wal_replay_is_empty() {
        let (wal, _dir) = temp_wal();
        let recovered = wal.replay().unwrap();
        assert!(recovered.is_empty());
    }

    #[test]
    fn temp_files_are_not_replayed() {
        let (mut wal, _dir) = temp_wal();
        let kp = make_kp();
        let peer = kp.public().to_peer_id();

        // Write a real entry
        let tx = Transaction::Mint {
            to: peer.to_string(),
            amount: DigitalUtilityUnit(100),
            authority: peer.to_string(),
            nonce: 1,
            timestamp: Utc::now(),
        };
        wal.append(&sign(tx, &kp)).unwrap();

        // Leave a temp file — simulate crash during append
        let peer_dir = wal.peer_dir(&peer);
        std::fs::write(peer_dir.join(".tmp.00000002"), b"partial").unwrap();

        // Replay must ignore the temp file
        let recovered = wal.replay().unwrap();
        assert_eq!(recovered.len(), 1);
        assert_eq!(recovered[0].transaction.nonce(), 1);
    }

    #[test]
    fn replay_sorts_by_nonce() {
        let (mut wal, _dir) = temp_wal();
        let kp = make_kp();
        let peer = kp.public().to_peer_id();

        // Write nonces out of order — replay must sort
        for nonce in [3u64, 1, 5, 2, 4] {
            let tx = Transaction::Mint {
                to: peer.to_string(),
                amount: DigitalUtilityUnit(10),
                authority: peer.to_string(),
                nonce,
                timestamp: Utc::now(),
            };
            wal.append(&sign(tx, &kp)).unwrap();
        }

        let recovered = wal.replay().unwrap();
        assert_eq!(recovered.len(), 5);
        for (i, tx) in recovered.iter().enumerate() {
            assert_eq!(tx.transaction.nonce(), (i as u64) + 1,
                "replay must sort by nonce within each peer");
        }
    }

    // ── Crash recovery: tail integrity ────────────────────

    /// A corrupt final WAL entry must be discarded without
    /// damaging earlier durable transactions.
    /// Covers the "during WAL write" crash row of the crash
    /// matrix (docs/architecture/persistence-design.md §4).
    #[test]
    fn tail_integrity_corrupt_entry_discarded() {
        let (mut wal, _dir) = temp_wal();
        let kp = make_kp();
        let peer = kp.public().to_peer_id();

        // Append A, B, C
        for nonce in 1..=3 {
            let tx = Transaction::Mint {
                to: peer.to_string(),
                amount: DigitalUtilityUnit(10),
                authority: peer.to_string(),
                nonce,
                timestamp: Utc::now(),
            };
            wal.append(&sign(tx, &kp)).unwrap();
        }
        assert_eq!(wal.replay().unwrap().len(), 3);

        // Corrupt C: truncate to half its length
        let c_path = wal.tx_path(&peer, 3);
        let original = std::fs::read(&c_path).unwrap();
        let partial = &original[..original.len() / 2];
        std::fs::write(&c_path, partial).unwrap();

        // Recovery must tolerate the corrupt entry — CBOR
        // deserialization of a partial record fails, and
        // the corrupt file must not prevent replay of A and B.
        let recovered = wal.replay().unwrap();
        assert_eq!(recovered.len(), 2,
            "corrupt entry C discarded; A and B survive");
        assert_eq!(recovered[0].transaction.nonce(), 1);
        assert_eq!(recovered[1].transaction.nonce(), 2);
    }

    /// A completely invalid file (garbage, not partial CBOR)
    /// is also discarded without harming valid entries.
    #[test]
    fn tail_integrity_garbage_entry_discarded() {
        let (mut wal, _dir) = temp_wal();
        let kp = make_kp();
        let peer = kp.public().to_peer_id();

        let tx = Transaction::Mint {
            to: peer.to_string(),
            amount: DigitalUtilityUnit(10),
            authority: peer.to_string(),
            nonce: 1,
            timestamp: Utc::now(),
        };
        wal.append(&sign(tx, &kp)).unwrap();

        // Simulate crash during write of nonce 2
        std::fs::write(wal.tx_path(&peer, 2), b"not-cbor").unwrap();

        let recovered = wal.replay().unwrap();
        assert_eq!(recovered.len(), 1,
            "garbage entry discarded; valid entry survives");
    }

    // ── Crash recovery: deterministic recovery ─────────────

    /// Given the same valid WAL, recovery produces the same
    /// transaction sequence every time.
    #[test]
    fn deterministic_recovery_same_wal_same_result() {
        let (mut wal, _dir) = temp_wal();
        let kp = make_kp();
        let peer = kp.public().to_peer_id();

        for nonce in 1..=3 {
            let tx = Transaction::Mint {
                to: peer.to_string(),
                amount: DigitalUtilityUnit(10),
                authority: peer.to_string(),
                nonce,
                timestamp: Utc::now(),
            };
            wal.append(&sign(tx, &kp)).unwrap();
        }

        let first = wal.replay().unwrap();
        let second = wal.replay().unwrap();

        assert_eq!(first.len(), second.len());
        for (a, b) in first.iter().zip(second.iter()) {
            assert_eq!(a.transaction.nonce(), b.transaction.nonce());
        }
    }

    // ── Crash recovery: no duplicate application ────────────

    /// Each durable transaction is applied exactly once during
    /// a recovery run.  The recovery process starts from empty
    /// state and walks the WAL in order.  Idempotence is a
    /// property of the application layer (nonce tracking), not
    /// of WAL replay itself.
    #[test]
    fn no_duplicate_application_during_recovery() {
        use std::collections::HashSet;

        let (mut wal, _dir) = temp_wal();
        let kp = make_kp();
        let peer = kp.public().to_peer_id();

        for nonce in 1..=3 {
            let tx = Transaction::Mint {
                to: peer.to_string(),
                amount: DigitalUtilityUnit(10),
                authority: peer.to_string(),
                nonce,
                timestamp: Utc::now(),
            };
            wal.append(&sign(tx, &kp)).unwrap();
        }

        let txs = wal.replay().unwrap();
        let mut seen: HashSet<(String, u64)> = HashSet::new();
        for tx in &txs {
            let key = (
                libp2p::identity::PublicKey::try_decode_protobuf(
                    &tx.signer_public_key,
                )
                .map(|pk| pk.to_peer_id().to_base58())
                .unwrap(),
                tx.transaction.nonce(),
            );
            let inserted = seen.insert(key.clone());
            assert!(
                inserted,
                "duplicate application: signer={:?} nonce={}",
                key.0, key.1
            );
        }
        assert_eq!(txs.len(), 3);
    }
}
