use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{PathBuf};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use super::thickness::ThicknessGraph;
use super::types::SignedTransaction;

// ── Persistent state (serializable) ──────────────────────────────

/// Components of economic state that survive restarts.
/// Transient buffers (tx_store, pending, outbound) repopulate from gossip.
/// 
/// NOTE: ThicknessGraph is complex to serialize (contains PeerId keys
/// that don't implement serde). We store seen_nonces in the snapshot;
/// the thickness graph is reconstructed from WAL replay on startup.
/// A full snapshot (graph + nonces) is a future optimization.
#[derive(Serialize, Deserialize)]
pub struct PersistentEconomicState {
    /// Per-peer highest applied nonce, keyed by base58 PeerId string.
    pub seen_nonces: HashMap<String, u64>,
}

impl PersistentEconomicState {
    pub fn new() -> Self {
        Self {
            seen_nonces: HashMap::new(),
        }
    }

    /// Import seen_nonces from the node's in-memory HashMap.
    pub fn import_nonces(&mut self, nonces: &HashMap<PeerId, u64>) {
        self.seen_nonces = nonces
            .iter()
            .map(|(k, v)| (k.to_base58(), *v))
            .collect();
    }

    /// Export seen_nonces back to PeerId-keyed HashMap.
    pub fn export_nonces(&self) -> HashMap<PeerId, u64> {
        self.seen_nonces
            .iter()
            .filter_map(|(k, v)| {
                k.parse::<PeerId>().ok().map(|pid| (pid, *v))
            })
            .collect()
    }
}

// ── StateStore trait ──────────────────────────────────────────────

pub trait StateStore: Send {
    fn persist(&mut self, tx: &SignedTransaction) -> Result<()>;
    fn recover(&mut self) -> Result<PersistentEconomicState>;
    fn take_snapshot(&mut self, state: &PersistentEconomicState) -> Result<()>;
}

// ── WAL State Store ───────────────────────────────────────────────

pub struct WalStateStoreConfig {
    pub data_dir: PathBuf,
    pub fsync_batch_size: u32,
    pub fsync_interval: Duration,
}

pub struct WalStateStore {
    config: WalStateStoreConfig,
    wal_path: PathBuf,
    snapshot_path: PathBuf,
    wal_buffer: Vec<u8>,
    fsync_counter: u32,
    last_fsync: Instant,
}

impl WalStateStore {
    pub fn new(config: WalStateStoreConfig) -> Result<Self> {
        fs::create_dir_all(&config.data_dir)
            .with_context(|| format!("creating data dir {:?}", config.data_dir))?;
        let wal_path = config.data_dir.join("transactions.wal");
        let snapshot_path = config.data_dir.join("state.snapshot");
        Ok(Self {
            config,
            wal_path,
            snapshot_path,
            wal_buffer: Vec::new(),
            fsync_counter: 0,
            last_fsync: Instant::now(),
        })
    }

    fn should_fsync(&self) -> bool {
        // First transaction always flushes; subsequent ones batch.
        self.fsync_counter == 1
            || self.fsync_counter >= self.config.fsync_batch_size
            || self.last_fsync.elapsed() >= self.config.fsync_interval
    }

    fn flush_wal(&mut self) -> Result<()> {
        if !self.wal_buffer.is_empty() {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.wal_path)?;
            file.write_all(&self.wal_buffer)?;
            file.sync_all()?;
            self.wal_buffer.clear();
            self.fsync_counter = 0;
            self.last_fsync = Instant::now();
        }
        Ok(())
    }

    fn apply_tx_to_state(state: &mut PersistentEconomicState, tx: &SignedTransaction) {
        let signer: String = match tx.transaction.signer().parse::<PeerId>() {
            Ok(pid) => pid.to_base58(),
            Err(_) => return,
        };
        let nonce = tx.transaction.nonce();
        let entry = state.seen_nonces.entry(signer).or_insert(0);
        if nonce > *entry {
            *entry = nonce;
        }
    }
}

impl StateStore for WalStateStore {
    fn persist(&mut self, tx: &SignedTransaction) -> Result<()> {
        let tx_bytes = serde_cbor::to_vec(tx)?;
        let len = tx_bytes.len() as u32;
        self.wal_buffer.extend_from_slice(&len.to_be_bytes());
        self.wal_buffer.extend_from_slice(&tx_bytes);

        self.fsync_counter += 1;
        if self.should_fsync() {
            self.flush_wal()?;
        }
        Ok(())
    }

    fn recover(&mut self) -> Result<PersistentEconomicState> {
        // 1. Load snapshot (nonces only — thickness graph from WAL replay)
        let mut state = match fs::read(&self.snapshot_path) {
            Ok(bytes) => {
                let s: PersistentEconomicState = serde_cbor::from_slice(&bytes)?;
                info!("Recovered nonces from snapshot");
                s
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                warn!("No snapshot found, starting fresh");
                PersistentEconomicState::new()
            }
            Err(e) => return Err(e.into()),
        };

        // 2. Replay WAL
        self.flush_wal()?;
        let mut wal_data = Vec::new();
        match OpenOptions::new().read(true).open(&self.wal_path) {
            Ok(mut f) => {
                f.read_to_end(&mut wal_data)?;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                info!("No WAL found, starting from snapshot only");
                return Ok(state);
            }
            Err(e) => return Err(e.into()),
        }

        let mut offset = 0;
        let mut replayed = 0u64;
        while offset + 4 <= wal_data.len() {
            let len = u32::from_be_bytes([
                wal_data[offset],
                wal_data[offset + 1],
                wal_data[offset + 2],
                wal_data[offset + 3],
            ]) as usize;
            offset += 4;
            if offset + len > wal_data.len() {
                warn!("Truncated WAL entry — stopping replay");
                break;
            }
            if let Ok(tx) = serde_cbor::from_slice::<SignedTransaction>(&wal_data[offset..offset + len]) {
                Self::apply_tx_to_state(&mut state, &tx);
                replayed += 1;
            }
            offset += len;
        }
        info!(replayed, "Replayed transactions from WAL");
        Ok(state)
    }

    fn take_snapshot(&mut self, state: &PersistentEconomicState) -> Result<()> {
        self.flush_wal()?;
        let bytes = serde_cbor::to_vec(state)?;
        let tmp = self.snapshot_path.with_extension("tmp");
        fs::write(&tmp, &bytes)?;
        fs::rename(&tmp, &self.snapshot_path)?;
        info!("Snapshot saved");
        Ok(())
    }
}

// ── Tests ─────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn persist_and_recover_empty() {
        let dir = tempdir().unwrap();
        let config = WalStateStoreConfig {
            data_dir: dir.path().to_path_buf(),
            fsync_batch_size: 100,
            fsync_interval: Duration::from_secs(60),
        };
        let mut store = WalStateStore::new(config).unwrap();
        let state = store.recover().unwrap();
        assert!(state.seen_nonces.is_empty());
    }

    #[test]
    fn snapshot_roundtrip() {
        let dir = tempdir().unwrap();
        let config = WalStateStoreConfig {
            data_dir: dir.path().to_path_buf(),
            fsync_batch_size: 100,
            fsync_interval: Duration::from_secs(60),
        };
        let mut store = WalStateStore::new(config).unwrap();
        let mut state = PersistentEconomicState::new();
        state.seen_nonces.insert("test-peer".into(), 42);
        store.take_snapshot(&state).unwrap();
        let recovered = store.recover().unwrap();
        assert_eq!(recovered.seen_nonces.get("test-peer"), Some(&42));
    }
}
