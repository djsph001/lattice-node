// Phase 9b — Streaming Model Execution Bridge
//
// Upgraded from single-response blocking to NDJSON token streaming.
// Ollama sends newline-delimited JSON chunks like:
//   {"model":"llama3.2","created_at":"...","response":"He","done":false}
//   {"model":"llama3.2","created_at":"...","response":"llo","done":false}
//   ...
//   {"model":"llama3.2","created_at":"...","response":"","done":true,"total_duration":...}
//
// The executor accumulates tokens into a complete response, then
// builds a single Blake3-verified Checkpoint at stream end.

use std::time::Duration;

use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use super::checkpoint::Checkpoint;

const DEFAULT_OLLAMA_ENDPOINT: &str = "http://localhost:11434/api/generate";
const DEFAULT_TIMEOUT_SECS: u64 = 300;

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

pub struct OllamaExecutor {
    client: reqwest::Client,
    endpoint: String,
}

impl OllamaExecutor {
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

    /// Execute a task by streaming tokens from Ollama.
    ///
    /// Uses `stream: true` and processes NDJSON chunks as they arrive.
    /// Tokens are accumulated into a single response, then sealed into
    /// a Blake3-verified Checkpoint at stream end.
    pub async fn execute(
        &self,
        task_id: &str,
        graph_blob: &[u8],
        graph_hash: &[u8; 32],
    ) -> Result<Checkpoint, ExecutorError> {
        let req: ExecutionRequest = serde_json::from_slice(graph_blob)
            .map_err(|e| ExecutorError::InvalidBlob(format!("JSON parse: {}", e)))?;

        info!(
            task_id = %task_id,
            model = %req.model,
            prompt_len = req.prompt.len(),
            "[executor] Streaming request to Ollama"
        );

        let body = serde_json::json!({
            "model": req.model,
            "prompt": req.prompt,
            "stream": true,
            "options": {
                "temperature": req.params.temperature.unwrap_or(0.7),
                "num_predict": req.params.max_tokens.unwrap_or(200),
            }
        });

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

        // Stream NDJSON chunks
        let mut stream = response.bytes_stream();
        let mut complete_response = String::new();
        let mut token_count = 0u64;

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| ExecutorError::OllamaError(e.to_string()))?;
            let text = String::from_utf8_lossy(&chunk);

            for line in text.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                match serde_json::from_str::<serde_json::Value>(line) {
                    Ok(parsed) => {
                        // Check for error
                        if let Some(error) = parsed.get("error") {
                            return Err(ExecutorError::OllamaResponse(error.to_string()));
                        }

                        // Collect token
                        if let Some(token) = parsed.get("response").and_then(|v| v.as_str()) {
                            complete_response.push_str(token);
                            token_count += 1;
                        }

                        // Done signal
                        if parsed.get("done").and_then(|v| v.as_bool()).unwrap_or(false) {
                            debug!(
                                task_id = %task_id,
                                tokens = token_count,
                                total_len = complete_response.len(),
                                "[executor] Stream complete"
                            );
                        }
                    }
                    Err(_) => {
                        // Skip malformed lines gracefully
                        debug!(task_id = %task_id, line = %line, "[executor] Skipping malformed NDJSON line");
                    }
                }
            }
        }

        let state_blob = complete_response.into_bytes();
        let state_hash = Checkpoint::compute_state_hash(&state_blob);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        info!(
            task_id = %task_id,
            tokens = token_count,
            response_len = state_blob.len(),
            "[executor] Streamed response complete"
        );

        Ok(Checkpoint {
            task_id: task_id.to_string(),
            graph_hash: *graph_hash,
            step_index: 0,
            state_blob,
            state_hash,
            epoch: 0,
            timestamp: now,
        })
    }
}

impl Default for OllamaExecutor {
    fn default() -> Self {
        Self::new()
    }
}
