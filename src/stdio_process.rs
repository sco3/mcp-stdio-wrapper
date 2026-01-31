use bytes::Bytes;
use tokio::io::{self, AsyncWrite, AsyncWriteExt, BufWriter};
use tracing::debug;

/// process a single message
/// # Errors
/// * error happens on stream close
pub async fn process_message<W>(stdout: &mut BufWriter<W>, message: &Bytes) -> io::Result<()>
where
    W: AsyncWrite + Unpin + Send + 'static,
{
    debug!("Write: {}", String::from_utf8_lossy(message));
    stdout.write_all(message).await?;
    if !message.ends_with(b"\n") {
        stdout.write_all(b"\n").await?;
    }
    stdout.flush().await?;
    Ok(())
}
