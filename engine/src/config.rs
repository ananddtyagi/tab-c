use std::time::Duration;

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub instant_deadline: Duration,
    pub refined_delay: Duration,
    pub max_candidates: usize,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            instant_deadline: Duration::from_millis(20),
            refined_delay: Duration::from_millis(180),
            max_candidates: 3,
        }
    }
}
