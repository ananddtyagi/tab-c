use anyhow::Result;
use std::path::Path;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::RwLock;
use tracing::{error, info, warn};

use super::protocol::{decode_message, encode_message, Request, Response};
use crate::config::Config;
use crate::core::{instant::InstantEngine, llm::LLMEngine, cache::Cache};
use crate::metrics::Metrics;

pub struct UnixSocketServer {
    socket_path: String,
    instant_engine: Arc<InstantEngine>,
    llm_engine: Option<Arc<LLMEngine>>,
    config: Arc<RwLock<Config>>,
    cache: Arc<Cache>,
    metrics: Arc<Metrics>,
}

impl UnixSocketServer {
    pub fn new(
        socket_path: String,
        instant_engine: InstantEngine,
        llm_engine: LLMEngine,
        config: Config,
    ) -> Result<Self> {
        let cache = Cache::new(config.cache.max_entries);
        let metrics = Metrics::new();

        Ok(Self {
            socket_path,
            instant_engine: Arc::new(instant_engine),
            llm_engine: Some(Arc::new(llm_engine)),
            config: Arc::new(RwLock::new(config)),
            cache: Arc::new(cache),
            metrics: Arc::new(metrics),
        })
    }

    pub fn new_instant_only(
        socket_path: String,
        instant_engine: InstantEngine,
        config: Config,
    ) -> Result<Self> {
        let cache = Cache::new(config.cache.max_entries);
        let metrics = Metrics::new();

        Ok(Self {
            socket_path,
            instant_engine: Arc::new(instant_engine),
            llm_engine: None,
            config: Arc::new(RwLock::new(config)),
            cache: Arc::new(cache),
            metrics: Arc::new(metrics),
        })
    }

    pub async fn run(self) -> Result<()> {
        // Remove old socket if exists
        let socket_path = Path::new(&self.socket_path);
        if socket_path.exists() {
            std::fs::remove_file(socket_path)?;
        }

        // Create parent directory if needed
        if let Some(parent) = socket_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let listener = UnixListener::bind(&self.socket_path)?;
        info!("Unix domain socket listening at: {}", self.socket_path);

        let server = Arc::new(self);

        loop {
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    let server = Arc::clone(&server);
                    tokio::spawn(async move {
                        if let Err(e) = server.handle_connection(stream).await {
                            error!("Error handling connection: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                }
            }
        }
    }

    async fn handle_connection(&self, mut stream: UnixStream) -> Result<()> {
        let mut buf = vec![0u8; 65536];
        let mut cursor = 0;

        loop {
            let n = stream.read(&mut buf[cursor..]).await?;
            if n == 0 {
                // Connection closed
                break;
            }
            cursor += n;

            // Try to decode messages
            let mut offset = 0;
            while offset < cursor {
                match decode_message(&buf[offset..cursor]) {
                    Ok((request, consumed)) => {
                        offset += consumed;

                        // Handle request
                        let responses = self.handle_request(request).await;

                        // Send responses
                        for response in responses {
                            let encoded = encode_message(&response)?;
                            stream.write_all(&encoded).await?;
                        }
                    }
                    Err(_) => {
                        // Not enough data yet
                        break;
                    }
                }
            }

            // Move remaining data to beginning of buffer
            if offset > 0 {
                buf.copy_within(offset..cursor, 0);
                cursor -= offset;
            }
        }

        Ok(())
    }

    async fn handle_request(&self, request: Request) -> Vec<Response> {
        match request {
            Request::Complete {
                prefix,
                max_tokens,
                app,
                cursor_id,
                fast_only,
            } => {
                self.handle_complete(prefix, max_tokens, app, cursor_id, fast_only)
                    .await
            }
            Request::Cancel { cursor_id } => {
                self.handle_cancel(cursor_id).await;
                vec![]
            }
            Request::UpdateConfig { config_json } => {
                match self.handle_update_config(config_json).await {
                    Ok(_) => vec![Response::Ok],
                    Err(e) => vec![Response::error(None, e.to_string())],
                }
            }
            Request::GetMetrics => {
                let data = self.metrics.snapshot();
                vec![Response::Metrics { data }]
            }
            Request::Shutdown => {
                info!("Shutdown requested");
                std::process::exit(0);
            }
        }
    }

    async fn handle_complete(
        &self,
        prefix: String,
        max_tokens: usize,
        app: String,
        cursor_id: String,
        fast_only: bool,
    ) -> Vec<Response> {
        let start = std::time::Instant::now();
        let config = self.config.read().await;
        let app_rule = config.get_app_rule(&app);

        // Check if app is excluded
        if config.privacy.excluded_apps.contains(&app) || !app_rule.enabled {
            return vec![Response::error(
                Some(cursor_id),
                "Autocomplete disabled for this app".to_string(),
            )];
        }

        // Check minimum prefix length
        if prefix.trim().len() < app_rule.min_prefix_length {
            return vec![];
        }

        // Check cache first
        let cache_key = format!("{}:{}", app, prefix);
        if let Some(cached) = self.cache.get(&cache_key) {
            self.metrics.record_cache_hit();
            return vec![Response::instant(
                cursor_id.clone(),
                cached,
                config.instant.timeout_ms,
            )];
        }
        self.metrics.record_cache_miss();

        let mut responses = Vec::new();

        // Get instant suggestions
        let instant_candidates = self
            .instant_engine
            .suggest(&prefix, config.instant.max_candidates);
        let instant_latency = start.elapsed().as_millis() as u64;
        self.metrics.record_instant_latency(instant_latency);

        if !instant_candidates.is_empty() {
            // Cache the result
            self.cache.put(cache_key.clone(), instant_candidates.clone());

            responses.push(Response::instant(
                cursor_id.clone(),
                instant_candidates.clone(),
                config.instant.timeout_ms,
            ));
        }

        // If fast_only, return early
        if fast_only || self.llm_engine.is_none() {
            return responses;
        }

        // Drop the read lock before spawning async task
        drop(config);

        // Get refined suggestions from LLM (async)
        if let Some(llm_engine) = &self.llm_engine {
            let llm = Arc::clone(llm_engine);
            let prefix_clone = prefix.clone();
            let cursor_id_clone = cursor_id.clone();
            let config_clone = Arc::clone(&self.config);
            let metrics_clone = Arc::clone(&self.metrics);
            let cache_clone = Arc::clone(&self.cache);
            let cache_key_clone = cache_key.clone();

            tokio::spawn(async move {
                let config = config_clone.read().await;
                let llm_start = std::time::Instant::now();

                match llm
                    .complete(&prefix_clone, max_tokens, &config.llm)
                    .await
                {
                    Ok(refined_candidates) => {
                        let llm_latency = llm_start.elapsed().as_millis() as u64;
                        metrics_clone.record_refined_latency(llm_latency);

                        if !refined_candidates.is_empty() {
                            // Update cache with refined results
                            cache_clone.put(cache_key_clone, refined_candidates.clone());

                            // Note: In production, we'd send this back through the socket
                            // For now, this is just logging
                            info!(
                                "Refined completion for cursor {} in {} ms: {:?}",
                                cursor_id_clone, llm_latency, refined_candidates
                            );
                        }
                    }
                    Err(e) => {
                        error!("LLM completion error: {}", e);
                    }
                }
            });
        }

        responses
    }

    async fn handle_cancel(&self, cursor_id: String) {
        info!("Cancellation requested for cursor: {}", cursor_id);
        // In a full implementation, we'd track ongoing LLM requests and cancel them
        // For now, this is a no-op
    }

    async fn handle_update_config(&self, config_json: String) -> Result<()> {
        let new_config: Config = serde_json::from_str(&config_json)?;
        let mut config = self.config.write().await;
        *config = new_config;
        info!("Configuration updated");
        Ok(())
    }
}
