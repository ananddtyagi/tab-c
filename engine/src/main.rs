mod config;
mod core;
mod ipc;
mod metrics;
mod util;

use anyhow::Result;
use std::path::PathBuf;
use tokio::signal;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone)]
struct EngineOptions {
    socket_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();

    let opts = parse_options();
    info!(socket = %opts.socket_path.display(), "starting engine");

    let runtime = core::EngineRuntime::new(config::EngineConfig::default()).await?;

    let server = ipc::uds::UnixServer::bind(opts.socket_path.clone(), runtime.clone()).await?;

    tokio::select! {
        res = server.run() => {
            if let Err(err) = res {
                error!(?err, "unix server exited with error");
            }
        }
        _ = signal::ctrl_c() => {
            info!("received ctrl-c; shutting down");
        }
    }

    runtime.shutdown().await;

    Ok(())
}

fn parse_options() -> EngineOptions {
    let mut socket_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Library/Application Support/MacAutoComplete/engine.sock");

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--socket" => {
                if let Some(value) = args.next() {
                    socket_path = PathBuf::from(value);
                }
            }
            _ => {}
        }
    }

    EngineOptions { socket_path }
}
