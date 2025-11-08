use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Messages from IME to Engine
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Request {
    Complete {
        prefix: String,
        max_tokens: usize,
        app: String,
        cursor_id: String,
        #[serde(default)]
        fast_only: bool,
    },
    Cancel {
        cursor_id: String,
    },
    UpdateConfig {
        config_json: String,
    },
    GetMetrics,
    Shutdown,
}

/// Messages from Engine to IME
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Response {
    Instant {
        cursor_id: String,
        candidates: Vec<String>,
        #[serde(default)]
        scores: Vec<f32>,
        expires_in_ms: u64,
    },
    Refined {
        cursor_id: String,
        candidates: Vec<String>,
        #[serde(default)]
        scores: Vec<f32>,
        model_info: ModelInfo,
    },
    Error {
        cursor_id: Option<String>,
        message: String,
    },
    Metrics {
        data: MetricsData,
    },
    Ok,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub latency_ms: u64,
    #[serde(default)]
    pub tokens_per_sec: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsData {
    pub total_requests: u64,
    pub instant_latency_p50_ms: f64,
    pub instant_latency_p95_ms: f64,
    pub refined_latency_p50_ms: f64,
    pub refined_latency_p95_ms: f64,
    pub accept_rate: f32,
    pub cache_hit_rate: f32,
}

impl Request {
    pub fn new_complete(
        prefix: String,
        max_tokens: usize,
        app: String,
        fast_only: bool,
    ) -> Self {
        Request::Complete {
            prefix,
            max_tokens,
            app,
            cursor_id: Uuid::new_v4().to_string(),
            fast_only,
        }
    }

    pub fn new_cancel(cursor_id: String) -> Self {
        Request::Cancel { cursor_id }
    }

    pub fn cursor_id(&self) -> Option<&str> {
        match self {
            Request::Complete { cursor_id, .. } => Some(cursor_id),
            Request::Cancel { cursor_id } => Some(cursor_id),
            _ => None,
        }
    }
}

impl Response {
    pub fn instant(cursor_id: String, candidates: Vec<String>, expires_in_ms: u64) -> Self {
        let scores = vec![1.0; candidates.len()];
        Response::Instant {
            cursor_id,
            candidates,
            scores,
            expires_in_ms,
        }
    }

    pub fn refined(
        cursor_id: String,
        candidates: Vec<String>,
        model_name: String,
        latency_ms: u64,
        tokens_per_sec: f32,
    ) -> Self {
        let scores = vec![1.0; candidates.len()];
        Response::Refined {
            cursor_id,
            candidates,
            scores,
            model_info: ModelInfo {
                name: model_name,
                latency_ms,
                tokens_per_sec,
            },
        }
    }

    pub fn error(cursor_id: Option<String>, message: String) -> Self {
        Response::Error { cursor_id, message }
    }
}

/// Wire protocol: length-prefixed JSON
/// Format: [4 bytes length (little-endian)] + [JSON bytes]

pub fn encode_message(msg: &Response) -> anyhow::Result<Vec<u8>> {
    let json = serde_json::to_vec(msg)?;
    let len = json.len() as u32;
    let mut buf = Vec::with_capacity(4 + json.len());
    buf.extend_from_slice(&len.to_le_bytes());
    buf.extend_from_slice(&json);
    Ok(buf)
}

pub fn decode_message(data: &[u8]) -> anyhow::Result<(Request, usize)> {
    if data.len() < 4 {
        anyhow::bail!("Insufficient data for length prefix");
    }
    let len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
    if data.len() < 4 + len {
        anyhow::bail!("Insufficient data for message body");
    }
    let msg: Request = serde_json::from_slice(&data[4..4 + len])?;
    Ok((msg, 4 + len))
}
