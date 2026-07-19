// Phase 9b — Streaming Model Execution Bridge
//
// Multi-backend executor: Ollama (local) and OpenAI-compatible (local or cloud).
// The execution graph blob is JSON; its `model` field selects the backend:
//   - "llama3.2:3b" or "ollama:..." -> Ollama
//   - "openai:gpt-4" or "openai://gpt-4" -> OpenAI-compatible chat completions
//
// Each backend streams tokens, accumulates them, and seals a single Blake3-verified
// Checkpoint at stream end.

use std::time::Duration;

use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use super::checkpoint::Checkpoint;

const DEFAULT_OLLAMA_ENDPOINT: &str = "http://localhost:11434/api/generate";
const DEFAULT_OPENAI_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";
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
    #[error("Backend not configured: {0}")]
    BackendNotConfigured(String),
    #[error("Ollama request failed: {0}")]
    OllamaError(String),
    #[error("Ollama returned error: {0}")]
    OllamaResponse(String),
    #[error("OpenAI request failed: {0}")]
    OpenAiError(String),
    #[error("OpenAI returned error: {0}")]
    OpenAiResponse(String),
    #[error("Request timed out after {0}s")]
    Timeout(u64),
    #[error("Backend not reachable — is it running? ({0})")]
    NotReachable(String),
}

/// Top-level executor. Selects the backend from the model name in the
/// execution graph and dispatches to the concrete implementation.
#[derive(Clone)]
pub struct Executor {
    ollama: OllamaExecutor,
    openai: Option<OpenAiExecutor>,
}

impl Executor {
    /// Create a new executor.
    ///
    /// `openai_api_key` is required to enable the OpenAI backend; without it,
    /// requests for `openai:*` models will fail with `BackendNotConfigured`.
    /// `openai_endpoint` defaults to the official OpenAI endpoint.
    pub fn new(openai_api_key: Option<String>, openai_endpoint: Option<String>) -> Self {
        Self {
            ollama: OllamaExecutor::new(),
            openai: openai_api_key.map(|key| OpenAiExecutor::new(key, openai_endpoint)),
        }
    }

    /// Execute a task by streaming tokens from the selected backend.
    pub async fn execute(
        &self,
        task_id: &str,
        graph_blob: &[u8],
        graph_hash: &[u8; 32],
    ) -> Result<Checkpoint, ExecutorError> {
        let mut req: ExecutionRequest = serde_json::from_slice(graph_blob)
            .map_err(|e| ExecutorError::InvalidBlob(format!("JSON parse: {e}")))?;

        let model = req.model.as_str();
        if model.starts_with("openai:") || model.starts_with("openai://") {
            let Some(openai) = &self.openai else {
                return Err(ExecutorError::BackendNotConfigured(
                    "openai (set --openai-api-key)".to_string(),
                ));
            };
            req.model = strip_openai_prefix(model);
            openai.execute(task_id, req, graph_hash).await
        } else {
            // Strip optional "ollama:" prefix so Ollama sees just the model name.
            if let Some(stripped) = model.strip_prefix("ollama:") {
                req.model = stripped.to_string();
            } else if let Some(stripped) = model.strip_prefix("ollama://") {
                req.model = stripped.to_string();
            }
            self.ollama.execute(task_id, req, graph_hash).await
        }
    }
}

fn strip_openai_prefix(model: &str) -> String {
    model
        .strip_prefix("openai://")
        .or_else(|| model.strip_prefix("openai:"))
        .unwrap_or(model)
        .to_string()
}

/// Ollama local inference backend.
#[derive(Clone)]
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
        req: ExecutionRequest,
        graph_hash: &[u8; 32],
    ) -> Result<Checkpoint, ExecutorError> {
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

        let response = match self
            .client
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

        let state_blob = stream_ndjson_to_bytes(response, task_id, "response").await?;

        seal_checkpoint(task_id, graph_hash, state_blob)
    }
}

impl Default for OllamaExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenAI-compatible chat-completions backend.
///
/// Works with OpenAI, Groq, LocalAI, llama.cpp server, vLLM's OpenAI-compatible
/// mode, and any other provider implementing the `/v1/chat/completions` endpoint.
#[derive(Clone)]
pub struct OpenAiExecutor {
    client: reqwest::Client,
    endpoint: String,
    api_key: String,
}

impl OpenAiExecutor {
    pub fn new(api_key: String, endpoint: Option<String>) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()
            .expect("reqwest client builder");

        Self {
            client,
            endpoint: endpoint.unwrap_or_else(|| DEFAULT_OPENAI_ENDPOINT.to_string()),
            api_key,
        }
    }

    /// Execute a task by streaming tokens from an OpenAI-compatible endpoint.
    ///
    /// Uses `stream: true` and processes SSE lines starting with `data: `. The
    /// content of each delta is accumulated into the final response.
    pub async fn execute(
        &self,
        task_id: &str,
        req: ExecutionRequest,
        graph_hash: &[u8; 32],
    ) -> Result<Checkpoint, ExecutorError> {
        info!(
            task_id = %task_id,
            model = %req.model,
            prompt_len = req.prompt.len(),
            endpoint = %self.endpoint,
            "[executor] Streaming request to OpenAI-compatible endpoint"
        );

        let body = serde_json::json!({
            "model": req.model,
            "messages": [{"role": "user", "content": req.prompt}],
            "stream": true,
            "temperature": req.params.temperature.unwrap_or(0.7),
            "max_tokens": req.params.max_tokens.unwrap_or(200),
        });

        let response = match self
            .client
            .post(&self.endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
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
                return Err(ExecutorError::OpenAiError(e.to_string()));
            }
        };

        // Surface provider-side errors before streaming.
        let status = response.status();
        if !status.is_success() {
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "(could not read error body)".to_string());
            return Err(ExecutorError::OpenAiResponse(format!(
                "HTTP {status}: {text}"
            )));
        }

        let state_blob = stream_openai_sse_to_bytes(response, task_id).await?;

        seal_checkpoint(task_id, graph_hash, state_blob)
    }
}

/// Accumulate NDJSON tokens from a streaming response into the final byte blob.
async fn stream_ndjson_to_bytes(
    response: reqwest::Response,
    task_id: &str,
    token_field: &str,
) -> Result<Vec<u8>, ExecutorError> {
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
                    if let Some(error) = parsed.get("error") {
                        return Err(ExecutorError::OllamaResponse(error.to_string()));
                    }

                    if let Some(token) = parsed.get(token_field).and_then(|v| v.as_str()) {
                        complete_response.push_str(token);
                        token_count += 1;
                    }

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
                    debug!(task_id = %task_id, line = %line, "[executor] Skipping malformed NDJSON line");
                }
            }
        }
    }

    info!(
        task_id = %task_id,
        tokens = token_count,
        response_len = complete_response.len(),
        "[executor] Streamed response complete"
    );

    Ok(complete_response.into_bytes())
}

/// Accumulate OpenAI-style SSE deltas into the final byte blob.
async fn stream_openai_sse_to_bytes(
    response: reqwest::Response,
    task_id: &str,
) -> Result<Vec<u8>, ExecutorError> {
    let mut stream = response.bytes_stream();
    let mut complete_response = String::new();
    let mut token_count = 0u64;

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.map_err(|e| ExecutorError::OpenAiError(e.to_string()))?;
        let text = String::from_utf8_lossy(&chunk);

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(":") {
                continue;
            }
            let Some(data) = line.strip_prefix("data: ") else {
                continue;
            };
            if data == "[DONE]" {
                debug!(
                    task_id = %task_id,
                    tokens = token_count,
                    total_len = complete_response.len(),
                    "[executor] SSE stream complete"
                );
                continue;
            }

            match serde_json::from_str::<serde_json::Value>(data) {
                Ok(parsed) => {
                    if let Some(error) = parsed.get("error") {
                        return Err(ExecutorError::OpenAiResponse(error.to_string()));
                    }

                    let delta = parsed
                        .pointer("/choices/0/delta/content")
                        .and_then(|v| v.as_str());
                    if let Some(token) = delta {
                        complete_response.push_str(token);
                        token_count += 1;
                    }
                }
                Err(_) => {
                    debug!(task_id = %task_id, line = %line, "[executor] Skipping malformed SSE data line");
                }
            }
        }
    }

    info!(
        task_id = %task_id,
        tokens = token_count,
        response_len = complete_response.len(),
        "[executor] SSE response complete"
    );

    Ok(complete_response.into_bytes())
}

/// Build the final Checkpoint from accumulated response bytes.
fn seal_checkpoint(
    task_id: &str,
    graph_hash: &[u8; 32],
    state_blob: Vec<u8>,
) -> Result<Checkpoint, ExecutorError> {
    let state_hash = Checkpoint::compute_state_hash(&state_blob);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_executor_has_no_openai_backend() {
        let ex = Executor::new(None, None);
        assert!(ex.openai.is_none());
    }

    #[test]
    fn strip_openai_prefix_variants() {
        assert_eq!(strip_openai_prefix("openai:gpt-4"), "gpt-4");
        assert_eq!(strip_openai_prefix("openai://gpt-4"), "gpt-4");
        assert_eq!(strip_openai_prefix("gpt-4"), "gpt-4");
    }
}
