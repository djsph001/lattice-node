// Phase 8 — Agent Registry
//
// File-backed JSONL store for agent task state. One JSON object per
// line (JSONL format) so the registry can be appended to without
// rewriting the entire file. On load, the last record for each
// task_id wins.
//
// Stored at: <storage_dir>/agent_registry.jsonl

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use tracing::{debug, info};

use super::checkpoint::Checkpoint;
use super::state::{AgentRecord, AgentStatus};

/// File-backed agent state registry.
#[derive(Debug)]
pub struct AgentRegistry {
    /// In-memory state — keyed by task_id.
    records: HashMap<String, AgentRecord>,
    /// Path to the JSONL file on disk.
    file_path: PathBuf,
}

impl AgentRegistry {
    /// Open or create the registry in the given storage directory.
    pub fn open(storage_dir: &PathBuf) -> Self {
        let file_path = storage_dir.join("agent_registry.jsonl");
        let mut records = HashMap::new();

        if file_path.exists() {
            if let Ok(file) = File::open(&file_path) {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        if line.trim().is_empty() {
                            continue;
                        }
                        if let Ok(record) = serde_json::from_str::<AgentRecord>(&line) {
                            // Last write wins for each task_id
                            records.insert(record.task.task_id.clone(), record);
                        }
                    }
                }
            }
            info!(
                count = records.len(),
                "[agent-registry] Loaded existing registry"
            );
        } else {
            info!("[agent-registry] No existing registry — starting fresh");
        }

        Self { records, file_path }
    }

    /// Number of tracked agent tasks.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Check if a task is already registered (dedup guard).
    pub fn contains(&self, task_id: &str) -> bool {
        self.records.contains_key(task_id)
    }

    /// Look up an agent by task_id.
    pub fn get(&self, task_id: &str) -> Option<&AgentRecord> {
        self.records.get(task_id)
    }

    /// List all task_ids assigned to a specific node.
    pub fn tasks_for_node(&self, peer_id: &str) -> Vec<&AgentRecord> {
        self.records
            .values()
            .filter(|r| r.assigned_node == peer_id)
            .collect()
    }

    /// Register a new task. Returns error if task_id already exists.
    pub fn register(&mut self, record: AgentRecord) -> Result<(), String> {
        if self.records.contains_key(&record.task.task_id) {
            return Err(format!(
                "Task {} already registered",
                record.task.task_id
            ));
        }
        self.records.insert(record.task.task_id.clone(), record.clone());
        self.append_to_file(&record);
        debug!(
            task_id = %record.task.task_id,
            node = %record.assigned_node,
            "[agent-registry] Task registered"
        );
        Ok(())
    }

    /// Update an existing task's status and checkpoint.
    pub fn update_status(
        &mut self,
        task_id: &str,
        status: AgentStatus,
        checkpoint: Option<Checkpoint>,
    ) -> Result<(), String> {
        let snapshot = {
            let record = self
                .records
                .get_mut(task_id)
                .ok_or_else(|| format!("Task {} not found", task_id))?;

            record.status = status;
            record.last_checkpoint = checkpoint;
            record.updated_at = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            record.clone()
        };

        self.append_to_file(&snapshot);
        debug!(
            task_id = %task_id,
            status = ?snapshot.status,
            "[agent-registry] Status updated"
        );
        Ok(())
    }

    /// Reassign a task to a new node (used on heartbeat failure — Phase 8b).
    pub fn reassign(&mut self, task_id: &str, new_node: &str) -> Result<(), String> {
        let snapshot = {
            let record = self
                .records
                .get_mut(task_id)
                .ok_or_else(|| format!("Task {} not found", task_id))?;

            record.assigned_node = new_node.to_string();
            record.status = AgentStatus::Idle;
            record.updated_at = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            record.clone()
        };

        self.append_to_file(&snapshot);
        info!(
            task_id = %task_id,
            new_node = %new_node,
            "[agent-registry] Task reassigned"
        );
        Ok(())
    }

    /// Return all records for iteration (e.g., deadline monitoring in Phase 8b).
    pub fn all(&self) -> impl Iterator<Item = &AgentRecord> {
        self.records.values()
    }

    // ── Internal ────────────────────────────────────────────

    fn append_to_file(&self, record: &AgentRecord) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
        {
            if let Ok(json) = serde_json::to_string(record) {
                let _ = writeln!(file, "{}", json);
                let _ = file.sync_all();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::state::{AgentStatus, AgentTask, ModelSize};

    fn make_test_record(task_id: &str, node: &str) -> AgentRecord {
        AgentRecord {
            task: AgentTask {
                task_id: task_id.to_string(),
                origin: "test-origin".to_string(),
                model: "test-model".to_string(),
                model_size: ModelSize::Small,
                vram_bytes: 0,
                harness_version: 1,
                graph_blob: vec![1, 2, 3],
                graph_hash: blake3::hash(b"test-graph").into(),
                deadline_epoch: 100,
                created_at: 0,
            },
            assigned_node: node.to_string(),
            status: AgentStatus::Idle,
            last_checkpoint: None,
            updated_at: 0,
        }
    }

    #[test]
    fn test_register_and_lookup() {
        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();
        let mut reg = AgentRegistry::open(&storage);

        let record = make_test_record("task-001", "node-alpha");
        reg.register(record).unwrap();

        assert_eq!(reg.len(), 1);
        let found = reg.get("task-001").unwrap();
        assert_eq!(found.task.task_id, "task-001");
        assert_eq!(found.assigned_node, "node-alpha");
    }

    #[test]
    fn test_duplicate_register_fails() {
        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();
        let mut reg = AgentRegistry::open(&storage);

        reg.register(make_test_record("task-001", "node-alpha")).unwrap();
        assert!(reg.register(make_test_record("task-001", "node-beta")).is_err());
    }

    #[test]
    fn test_update_status() {
        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();
        let mut reg = AgentRegistry::open(&storage);

        reg.register(make_test_record("task-001", "node-alpha")).unwrap();
        reg.update_status("task-001", AgentStatus::Running { step: 3 }, None).unwrap();

        let record = reg.get("task-001").unwrap();
        assert_eq!(record.status, AgentStatus::Running { step: 3 });
    }

    #[test]
    fn test_tasks_for_node() {
        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();
        let mut reg = AgentRegistry::open(&storage);

        reg.register(make_test_record("task-001", "node-alpha")).unwrap();
        reg.register(make_test_record("task-002", "node-alpha")).unwrap();
        reg.register(make_test_record("task-003", "node-beta")).unwrap();

        let alpha_tasks = reg.tasks_for_node("node-alpha");
        assert_eq!(alpha_tasks.len(), 2);

        let beta_tasks = reg.tasks_for_node("node-beta");
        assert_eq!(beta_tasks.len(), 1);
    }

    #[test]
    fn test_persistence_across_opens() {
        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();

        {
            let mut reg = AgentRegistry::open(&storage);
            reg.register(make_test_record("task-001", "node-alpha")).unwrap();
            reg.update_status("task-001", AgentStatus::Completed, None).unwrap();
        }

        // Re-open — should recover state
        let reg = AgentRegistry::open(&storage);
        assert_eq!(reg.len(), 1);
        let record = reg.get("task-001").unwrap();
        assert_eq!(record.status, AgentStatus::Completed);
    }

    #[test]
    fn test_reassign_changes_node_and_resets_status() {
        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();
        let mut reg = AgentRegistry::open(&storage);

        reg.register(make_test_record("task-001", "node-alpha")).unwrap();
        reg.update_status("task-001", AgentStatus::Running { step: 5 }, None).unwrap();
        reg.reassign("task-001", "node-beta").unwrap();

        let record = reg.get("task-001").unwrap();
        assert_eq!(record.assigned_node, "node-beta");
        assert_eq!(record.status, AgentStatus::Idle);
    }
}
