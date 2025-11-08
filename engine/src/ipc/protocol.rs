use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum EngineMessage {
    #[serde(rename = "instant")]
    Instant(InstantResponse),
    #[serde(rename = "refined")]
    Refined(RefinedResponse),
    #[serde(skip)]
    Shutdown,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CompleteRequest {
    pub prefix: String,
    pub max_tokens: usize,
    pub app: String,
    pub cursor_id: String,
    #[serde(default)]
    pub fast_only: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InstantResponse {
    pub cursor_id: String,
    pub candidates: Vec<String>,
    pub expires_in_ms: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RefinedResponse {
    pub cursor_id: String,
    pub candidates: Vec<String>,
    pub model_info: super::super::core::llm::ModelInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "complete")]
    Complete(CompleteRequest),
}
