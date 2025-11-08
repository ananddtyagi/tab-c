use anyhow::Result;
use clap::Parser;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod core;
mod ipc;
mod metrics;
mod util;

use config::Config;
use ipc::uds::UnixSocketServer;

#[derive(Parser, Debug)]
#[command(name = "mac_autocomplete_engine")]
#[command(about = "Ultra-low-latency autocomplete engine for macOS", long_about = None)]
struct Args {
    /// Path to the model file (GGUF format)
    #[arg(short, long)]
    model: Option<String>,

    /// Unix domain socket path
    #[arg(short, long)]
    socket: Option<String>,

    /// Context size
    #[arg(long, default_value = "1024")]
    ctx: usize,

    /// Number of threads
    #[arg(short, long, default_value = "6")]
    threads: usize,

    /// Enable Metal/MPS acceleration
    #[arg(long, default_value = "true")]
    metal: bool,

    /// Config file path
    #[arg(short, long)]
    config: Option<String>,

    /// Log level
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| args.log_level.clone().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("MacAutoComplete Engine starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = Config::load(args.config.as_deref())?;
    info!("Configuration loaded");

    // Override config with CLI args
    let socket_path = args.socket.unwrap_or_else(|| config.socket_path.clone());
    let model_path = args.model.unwrap_or_else(|| config.model_path.clone());

    info!("Socket path: {}", socket_path);
    info!("Model path: {}", model_path);
    info!("Context size: {}", args.ctx);
    info!("Threads: {}", args.threads);
    info!("Metal acceleration: {}", args.metal);

    // Initialize instant engine (trie, n-gram, etc.)
    info!("Initializing instant suggestion engine...");
    let instant_engine = core::instant::InstantEngine::new(&config)?;
    info!("Instant engine ready");

    // Initialize LLM engine (llama.cpp)
    #[cfg(feature = "llama-server")]
    {
        info!("Initializing LLM engine (llama.cpp server mode)...");
        let llm_engine = core::llm::LLMEngine::new_server(
            &model_path,
            args.ctx,
            args.threads,
            args.metal,
        )?;
        info!("LLM engine ready");

        // Start IPC server
        info!("Starting Unix domain socket server...");
        let server = UnixSocketServer::new(socket_path, instant_engine, llm_engine, config)?;
        server.run().await?;
    }

    #[cfg(not(feature = "llama-server"))]
    {
        warn!("LLM engine disabled (no feature flag)");
        // Start with instant engine only
        let server = UnixSocketServer::new_instant_only(socket_path, instant_engine, config)?;
        server.run().await?;
    }

    Ok(())
}
