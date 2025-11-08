use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{error, info};

use crate::config::LLMConfig;

#[cfg(feature = "llama-server")]
use reqwest::Client;

/// LLM Engine wrapper for llama.cpp server
pub struct LLMEngine {
    #[cfg(feature = "llama-server")]
    client: Client,
    #[cfg(feature = "llama-server")]
    server_url: String,
    model_name: String,
}

#[derive(Debug, Serialize)]
struct CompletionRequest {
    prompt: String,
    n_predict: usize,
    temperature: f32,
    top_k: i32,
    top_p: f32,
    stop: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_prompt: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct CompletionResponse {
    content: String,
    #[serde(default)]
    tokens_predicted: usize,
    #[serde(default)]
    tokens_evaluated: usize,
    #[serde(default)]
    generation_settings: GenerationSettings,
}

#[derive(Debug, Default, Deserialize)]
struct GenerationSettings {
    #[serde(default)]
    model: String,
}

impl LLMEngine {
    #[cfg(feature = "llama-server")]
    pub fn new_server(
        model_path: &str,
        _ctx_size: usize,
        _threads: usize,
        _metal: bool,
    ) -> Result<Self> {
        info!("Initializing LLM engine (server mode)");
        info!(
            "Note: You need to start llama.cpp server separately with: ./server -m {}",
            model_path
        );

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            server_url: "http://127.0.0.1:8087".to_string(),
            model_name: Self::extract_model_name(model_path),
        })
    }

    #[cfg(not(feature = "llama-server"))]
    pub fn new_server(
        model_path: &str,
        _ctx_size: usize,
        _threads: usize,
        _metal: bool,
    ) -> Result<Self> {
        Ok(Self {
            model_name: Self::extract_model_name(model_path),
        })
    }

    fn extract_model_name(path: &str) -> String {
        std::path::Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string()
    }

    #[cfg(feature = "llama-server")]
    pub async fn complete(
        &self,
        prefix: &str,
        max_tokens: usize,
        config: &LLMConfig,
    ) -> Result<Vec<String>> {
        let request = CompletionRequest {
            prompt: prefix.to_string(),
            n_predict: max_tokens.min(config.max_tokens),
            temperature: config.temperature,
            top_k: config.top_k,
            top_p: config.top_p,
            stop: vec!["\n\n".to_string(), "\n".to_string()],
            cache_prompt: Some(config.enable_kv_cache),
        };

        let url = format!("{}/completion", self.server_url);

        match self.client.post(&url).json(&request).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    let completion: CompletionResponse = response.json().await?;
                    let text = completion.content.trim();

                    if text.is_empty() {
                        return Ok(Vec::new());
                    }

                    // Return multiple candidates by splitting on natural boundaries
                    let candidates = self.split_into_candidates(text);
                    Ok(candidates)
                } else {
                    let status = response.status();
                    let error_text = response.text().await.unwrap_or_default();
                    error!("LLM server error {}: {}", status, error_text);
                    anyhow::bail!("LLM server returned error: {}", status);
                }
            }
            Err(e) => {
                error!("Failed to connect to LLM server: {}", e);
                anyhow::bail!("LLM server connection failed: {}", e);
            }
        }
    }

    #[cfg(not(feature = "llama-server"))]
    pub async fn complete(
        &self,
        _prefix: &str,
        _max_tokens: usize,
        _config: &LLMConfig,
    ) -> Result<Vec<String>> {
        anyhow::bail!("LLM engine not enabled (compile with llama-server feature)");
    }

    /// Split completion into multiple candidates
    fn split_into_candidates(&self, text: &str) -> Vec<String> {
        let mut candidates = Vec::new();

        // Primary candidate: full text
        candidates.push(text.to_string());

        // Secondary candidates: split on sentence boundaries
        let sentences: Vec<&str> = text.split(". ").collect();
        if sentences.len() > 1 {
            candidates.push(sentences[0].to_string());
        }

        // Tertiary candidates: split on word boundaries (shorter completions)
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.len() > 5 {
            candidates.push(words[..words.len() / 2].join(" "));
        }

        // Remove duplicates and empty strings
        candidates.retain(|c| !c.is_empty());
        candidates.dedup();

        candidates
    }

    pub fn model_name(&self) -> &str {
        &self.model_name
    }

    #[cfg(feature = "llama-server")]
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/health", self.server_url);
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    #[cfg(not(feature = "llama-server"))]
    pub async fn health_check(&self) -> Result<bool> {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_candidates() {
        let engine = LLMEngine {
            #[cfg(feature = "llama-server")]
            client: reqwest::Client::new(),
            #[cfg(feature = "llama-server")]
            server_url: "http://localhost:8087".to_string(),
            model_name: "test".to_string(),
        };

        let text = "The quick brown fox jumps over the lazy dog";
        let candidates = engine.split_into_candidates(text);
        assert!(!candidates.is_empty());
        assert_eq!(candidates[0], text);
    }
}
