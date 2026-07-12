// Phase 9 — Model Execution Bridge
//
// The OllamaExecutor bridges the lattice's sortition-based task selection
// with actual model inference. When a node is selected as executor for
// an AgentTask, the executor:
//
// 1. Parses the task's graph_blob as a JSON execution request
// 2. POSTs to the local Ollama API (http://localhost:11434/api/generate)
// 3. Stores the response as a Blake3-verified Checkpoint
// 4. Returns the checkpoint for registry update
//
// Graph blob format (JSON):
// {
//   "prompt": "What is the capital of France?",
//   "model": "llama3.2:3b",
//   "params": {
//     "temperature": 0.7,
//     "max_tokens": 100
//   }
// }

use std::time::Duration;

use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use super::checkpoint::Checkpoint;

/// Default Ollama API endpoint.
const DEFAULT_OLLAMA_ENDPOINT: &str = "http://localhost:11434/api/generate";

/// Default request timeout (5 minutes — covers cold model loads).
const DEFAULT_TIMEOUT_SECS: u64 = 300;

/// The parsed execution request extracted from a task's graph_blob.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRequest {
    pub prompt: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default)]
    pub params: ExecutionParams,
}

fn default_model() -> String {
    "llama3.2:3b".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExecutionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

/// Errors that can occur during model execution.
#[derive(Debug, thiserror::Error)]
pub enum ExecutorError {
    #[error("Invalid graph blob: {0}")]
    InvalidBlob(String),
    #[error("Ollama request failed: {0}")]
    OllamaError(String),
    #[error("Ollama returned error: {0}")]
    OllamaResponse(String),
    #[error("Request timed out after {0}s")]
    Timeout(u64),
    #[error("Ollama not reachable — is it running? ({0})")]
    NotReachable(String),
}

/// Executes agent tasks by calling a local Ollama instance.
pub struct OllamaExecutor {
    client: reqwest::Client,
    endpoint: String,
}

impl OllamaExecutor {
    /// Create a new executor with default settings.
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()
            .expect("reqwest client builder");

        Self {
            client,
            endpoint: DEFAULT_OLLAMA_ENDPOINT.to_string(),
        }
    }

    /// Execute a task by sending its prompt to Ollama.
    ///
    /// Parses `graph_blob` as JSON, POSTs to the Ollama generate endpoint,
    /// and returns a Checkpoint containing the model's response.
    pub async fn execute(
        &self,
        task_id: &str,
        graph_blob: &[u8],
        graph_hash: &[u8; 32],
    ) -> Result<Checkpoint, ExecutorError> {
        // Parse the execution request from the graph blob
        let req: ExecutionRequest = serde_json::from_slice(graph_blob)
            .map_err(|e| ExecutorError::InvalidBlob(format!("JSON parse: {}", e)))?;

        info!(
            task_id = %task_id,
            model = %req.model,
            prompt_len = req.prompt.len(),
            "[executor] Sending request to Ollama"
        );

        // Build the Ollama generate request body
        let body = serde_json::json!({
            "model": req.model,
            "prompt": req.prompt,
            "stream": false,
            "options": {
                "temperature": req.params.temperature.unwrap_or(0.7),
                "num_predict": req.params.max_tokens.unwrap_or(100),
            }
        });

        // POST to Ollama
        let response = match self.client
            .post(&self.endpoint)
            .json(&body)
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                if e.is_timeout() {
                    return Err(ExecutorError::Timeout(DEFAULT_TIMEOUT_SECS));
                }
                if e.is_connect() {
                    return Err(ExecutorError::NotReachable(e.to_string()));
                }
                return Err(ExecutorError::OllamaError(e.to_string()));
            }
        };

        // Parse the Ollama response
        let ollama_resp: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ExecutorError::OllamaError(format!("Response parse: {}", e)))?;

        // Check for Ollama error
        if let Some(error) = ollama_resp.get("error") {
            return Err(ExecutorError::OllamaResponse(error.to_string()));
        }

        // Extract the generated text
        let response_text = ollama_resp
            .get("response")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let state_blob = response_text.into_bytes();
        let state_hash = Checkpoint::compute_state_hash(&state_blob);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        info!(
            task_id = %task_id,
            response_len = state_blob.len(),
            "[executor] Ollama response received"
        );

        Ok(Checkpoint {
            task_id: task_id.to_string(),
            graph_hash: *graph_hash,
            step_index: 0,
            state_blob,
            state_hash,
            epoch: 0, // set by caller
            timestamp: now,
        })
    }
}

impl Default for OllamaExecutor {
    fn default() -> Self {
        Self::new()
    }
}
