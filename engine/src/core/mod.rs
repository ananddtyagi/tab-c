use std::sync::Arc;

use tokio::sync::{broadcast, Mutex};

use crate::{
    config::EngineConfig,
    ipc::protocol::{CompleteRequest, EngineMessage, InstantResponse, RefinedResponse},
};

pub mod cache;
pub mod context;
pub mod instant;
pub mod llm;
pub mod rank;

#[derive(Clone)]
pub struct EngineRuntime {
    config: EngineConfig,
    state: Arc<EngineState>,
    notifier: broadcast::Sender<EngineMessage>,
}

struct EngineState {
    instant: instant::InstantEngine,
    llm: llm::StubLlm,
    cache: Mutex<cache::PrefixCache>,
}

impl EngineRuntime {
    pub async fn new(config: EngineConfig) -> anyhow::Result<Self> {
        let (notifier, _) = broadcast::channel(32);
        let state = Arc::new(EngineState {
            instant: instant::InstantEngine::new(config.max_candidates),
            llm: llm::StubLlm::new(),
            cache: Mutex::new(cache::PrefixCache::new(1024)),
        });

        Ok(Self {
            config,
            state,
            notifier,
        })
    }

    pub async fn handle_request(&self, request: CompleteRequest) {
        let instant = self.state.instant.suggest(&request);
        let instant_message = EngineMessage::Instant(InstantResponse {
            cursor_id: request.cursor_id.clone(),
            candidates: instant.clone(),
            expires_in_ms: self.config.instant_deadline.as_millis() as u64,
        });
        let _ = self.notifier.send(instant_message);

        let llm = self.state.llm.clone();
        let cursor = request.cursor_id.clone();
        let max = self.config.max_candidates;
        let notifier = self.notifier.clone();
        let refined_delay = self.config.refined_delay;

        tokio::spawn(async move {
            tokio::time::sleep(refined_delay).await;
            let refined_candidates = llm.refine(&request.prefix, max).await;
            let message = EngineMessage::Refined(RefinedResponse {
                cursor_id: cursor,
                candidates: refined_candidates,
                model_info: llm::ModelInfo {
                    name: "stub-model".into(),
                    latency_ms: refined_delay.as_millis() as u64,
                },
            });
            let _ = notifier.send(message);
        });
    }

    pub fn subscribe(&self) -> broadcast::Receiver<EngineMessage> {
        self.notifier.subscribe()
    }

    pub async fn shutdown(&self) {
        let _ = self.notifier.send(EngineMessage::Shutdown);
    }
}
