use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Socket path for IPC
    #[serde(default = "default_socket_path")]
    pub socket_path: String,

    /// Model path
    #[serde(default = "default_model_path")]
    pub model_path: String,

    /// Context window size
    #[serde(default = "default_ctx_size")]
    pub ctx_size: usize,

    /// Number of threads
    #[serde(default = "default_threads")]
    pub threads: usize,

    /// Enable Metal/MPS
    #[serde(default = "default_metal")]
    pub metal: bool,

    /// Instant engine settings
    #[serde(default)]
    pub instant: InstantConfig,

    /// LLM engine settings
    #[serde(default)]
    pub llm: LLMConfig,

    /// Per-app rules
    #[serde(default)]
    pub app_rules: HashMap<String, AppRule>,

    /// Privacy settings
    #[serde(default)]
    pub privacy: PrivacyConfig,

    /// Cache settings
    #[serde(default)]
    pub cache: CacheConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantConfig {
    /// Enable trie-based suggestions
    #[serde(default = "default_true")]
    pub enable_trie: bool,

    /// Trie data path
    #[serde(default = "default_trie_path")]
    pub trie_path: String,

    /// Maximum instant candidates
    #[serde(default = "default_instant_candidates")]
    pub max_candidates: usize,

    /// Instant timeout (ms)
    #[serde(default = "default_instant_timeout")]
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    /// Maximum tokens to generate
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,

    /// Temperature
    #[serde(default = "default_temperature")]
    pub temperature: f32,

    /// Top-k
    #[serde(default = "default_top_k")]
    pub top_k: i32,

    /// Top-p
    #[serde(default = "default_top_p")]
    pub top_p: f32,

    /// Llama.cpp server URL
    #[serde(default = "default_llm_server")]
    pub server_url: String,

    /// Refined timeout (ms)
    #[serde(default = "default_refined_timeout")]
    pub timeout_ms: u64,

    /// Enable KV cache reuse
    #[serde(default = "default_true")]
    pub enable_kv_cache: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRule {
    /// Enable autocomplete for this app
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Minimum prefix length
    #[serde(default = "default_min_prefix")]
    pub min_prefix_length: usize,

    /// Maximum suggestion length (tokens)
    #[serde(default = "default_max_suggestion")]
    pub max_suggestion_tokens: usize,

    /// Aggressiveness (0.0 - 1.0)
    #[serde(default = "default_aggressiveness")]
    pub aggressiveness: f32,

    /// Enable multiline suggestions
    #[serde(default = "default_false")]
    pub multiline: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    /// Enable learning from user acceptances
    #[serde(default = "default_false")]
    pub enable_learning: bool,

    /// Enable telemetry
    #[serde(default = "default_false")]
    pub enable_telemetry: bool,

    /// Excluded apps (never suggest)
    #[serde(default)]
    pub excluded_apps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// LRU cache size (number of entries)
    #[serde(default = "default_cache_size")]
    pub max_entries: usize,

    /// Enable disk persistence
    #[serde(default = "default_false")]
    pub enable_persistence: bool,
}

// Default value functions
fn default_socket_path() -> String {
    dirs::home_dir()
        .unwrap_or_default()
        .join("Library/Application Support/MacAutoComplete/engine.sock")
        .to_string_lossy()
        .to_string()
}

fn default_model_path() -> String {
    dirs::home_dir()
        .unwrap_or_default()
        .join("Library/Application Support/MacAutoComplete/models/tinyllama-1.1b.Q4_K_M.gguf")
        .to_string_lossy()
        .to_string()
}

fn default_trie_path() -> String {
    dirs::home_dir()
        .unwrap_or_default()
        .join("Library/Application Support/MacAutoComplete/trie.bin")
        .to_string_lossy()
        .to_string()
}

fn default_ctx_size() -> usize { 1024 }
fn default_threads() -> usize { 6 }
fn default_metal() -> bool { true }
fn default_true() -> bool { true }
fn default_false() -> bool { false }
fn default_instant_candidates() -> usize { 5 }
fn default_instant_timeout() -> u64 { 15 }
fn default_max_tokens() -> usize { 64 }
fn default_temperature() -> f32 { 0.7 }
fn default_top_k() -> i32 { 40 }
fn default_top_p() -> f32 { 0.9 }
fn default_llm_server() -> String { "http://127.0.0.1:8087".to_string() }
fn default_refined_timeout() -> u64 { 300 }
fn default_min_prefix() -> usize { 3 }
fn default_max_suggestion() -> usize { 32 }
fn default_aggressiveness() -> f32 { 0.7 }
fn default_cache_size() -> usize { 10000 }

impl Default for InstantConfig {
    fn default() -> Self {
        Self {
            enable_trie: default_true(),
            trie_path: default_trie_path(),
            max_candidates: default_instant_candidates(),
            timeout_ms: default_instant_timeout(),
        }
    }
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            max_tokens: default_max_tokens(),
            temperature: default_temperature(),
            top_k: default_top_k(),
            top_p: default_top_p(),
            server_url: default_llm_server(),
            timeout_ms: default_refined_timeout(),
            enable_kv_cache: default_true(),
        }
    }
}

impl Default for AppRule {
    fn default() -> Self {
        Self {
            enabled: default_true(),
            min_prefix_length: default_min_prefix(),
            max_suggestion_tokens: default_max_suggestion(),
            aggressiveness: default_aggressiveness(),
            multiline: default_false(),
        }
    }
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            enable_learning: default_false(),
            enable_telemetry: default_false(),
            excluded_apps: vec![
                "com.apple.keychainaccess".to_string(),
                "1Password".to_string(),
            ],
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: default_cache_size(),
            enable_persistence: default_false(),
        }
    }
}

impl Config {
    pub fn load(path: Option<&str>) -> Result<Self> {
        let config_path = if let Some(p) = path {
            Path::new(p).to_path_buf()
        } else {
            dirs::home_dir()
                .unwrap_or_default()
                .join("Library/Application Support/MacAutoComplete/config.json")
        };

        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            let config: Config = serde_json::from_str(&contents)?;
            Ok(config)
        } else {
            // Return default config
            Ok(Self::default())
        }
    }

    pub fn save(&self, path: Option<&str>) -> Result<()> {
        let config_path = if let Some(p) = path {
            Path::new(p).to_path_buf()
        } else {
            dirs::home_dir()
                .unwrap_or_default()
                .join("Library/Application Support/MacAutoComplete/config.json")
        };

        // Create parent directory if needed
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let contents = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, contents)?;
        Ok(())
    }

    pub fn get_app_rule(&self, app_id: &str) -> AppRule {
        self.app_rules.get(app_id).cloned().unwrap_or_default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            socket_path: default_socket_path(),
            model_path: default_model_path(),
            ctx_size: default_ctx_size(),
            threads: default_threads(),
            metal: default_metal(),
            instant: InstantConfig::default(),
            llm: LLMConfig::default(),
            app_rules: HashMap::new(),
            privacy: PrivacyConfig::default(),
            cache: CacheConfig::default(),
        }
    }
}
