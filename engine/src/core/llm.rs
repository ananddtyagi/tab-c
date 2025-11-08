use std::sync::Arc;

use tokio::sync::Mutex;

#[derive(Clone)]
pub struct StubLlm {
    state: Arc<Mutex<()>>,
}

#[derive(Clone, Debug)]
pub struct ModelInfo {
    pub name: String,
    pub latency_ms: u64,
}

impl StubLlm {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(())),
        }
    }

    pub async fn refine(&self, prefix: &str, max_candidates: usize) -> Vec<String> {
        let _guard = self.state.lock().await;
        let trimmed = prefix.trim();
        if trimmed.is_empty() {
            return Vec::new();
        }
        (1..=max_candidates)
            .map(|n| format!("{} · refined option {}", trimmed, n))
            .collect()
    }
}
