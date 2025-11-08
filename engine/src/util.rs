use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

pub async fn read_json_messages<T, R>(reader: R) -> anyhow::Result<Vec<T>>
where
    T: for<'de> serde::Deserialize<'de>,
    R: tokio::io::AsyncRead + Unpin,
{
    let mut reader = BufReader::new(reader);
    let mut buf = String::new();
    let mut out = Vec::new();

    while reader.read_line(&mut buf).await? != 0 {
        if buf.trim().is_empty() {
            continue;
        }
        let value = serde_json::from_str(&buf)?;
        out.push(value);
        buf.clear();
    }

    Ok(out)
}

pub async fn write_json_message<T, W>(mut writer: W, value: &T) -> anyhow::Result<()>
where
    T: serde::Serialize,
    W: tokio::io::AsyncWrite + Unpin,
{
    let data = serde_json::to_vec(value)?;
    writer.write_all(&data).await?;
    writer.write_all(b"\n").await?;
    writer.flush().await?;
    Ok(())
}
