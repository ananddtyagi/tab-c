use std::{fs, path::Path};

use serde_json::json;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{UnixListener, UnixStream},
};
use tracing::{error, info, warn};

use crate::{
    core::EngineRuntime,
    ipc::protocol::{ClientMessage, EngineMessage},
};

pub struct UnixServer {
    listener: UnixListener,
    runtime: EngineRuntime,
}

impl UnixServer {
    pub async fn bind<P: AsRef<Path>>(path: P, runtime: EngineRuntime) -> anyhow::Result<Self> {
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        if path.as_ref().exists() {
            fs::remove_file(path.as_ref())?;
        }

        let listener = UnixListener::bind(&path)?;
        info!(socket = %path.as_ref().display(), "listening for IME clients");
        Ok(Self { listener, runtime })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        loop {
            let (stream, _) = self.listener.accept().await?;
            let runtime = self.runtime.clone();
            tokio::spawn(async move {
                if let Err(err) = handle_connection(stream, runtime).await {
                    warn!(?err, "connection closed with error");
                }
            });
        }
    }
}

async fn handle_connection(stream: UnixStream, runtime: EngineRuntime) -> anyhow::Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut sub = runtime.subscribe();
    let write_runtime = runtime.clone();

    let write_task = tokio::spawn(async move {
        while let Ok(message) = sub.recv().await {
            match message {
                EngineMessage::Instant(msg) => {
                    let payload = json!({
                        "type": "instant",
                        "cursor_id": msg.cursor_id,
                        "candidates": msg.candidates,
                        "expires_in_ms": msg.expires_in_ms,
                    });
                    let data = serde_json::to_vec(&payload)?;
                    writer.write_all(&data).await?;
                    writer.write_all(b"\n").await?;
                    writer.flush().await?;
                }
                EngineMessage::Refined(msg) => {
                    let payload = json!({
                        "type": "refined",
                        "cursor_id": msg.cursor_id,
                        "candidates": msg.candidates,
                        "model_info": {
                            "name": msg.model_info.name,
                            "latency_ms": msg.model_info.latency_ms,
                        }
                    });
                    let data = serde_json::to_vec(&payload)?;
                    writer.write_all(&data).await?;
                    writer.write_all(b"\n").await?;
                    writer.flush().await?;
                }
                EngineMessage::Shutdown => break,
            }
        }
        Ok::<(), anyhow::Error>(())
    });
    let mut buf = String::new();
    loop {
        buf.clear();
        let read = reader.read_line(&mut buf).await?;
        if read == 0 {
            break;
        }
        if buf.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<ClientMessage>(&buf) {
            Ok(ClientMessage::Complete(request)) => {
                write_runtime.handle_request(request).await;
            }
            Err(err) => {
                error!(?err, payload = %buf, "failed to decode message");
            }
        }
    }

    write_task.await??;
    Ok(())
}
