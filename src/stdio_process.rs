use tokio::io::{self, AsyncWrite, AsyncWriteExt, BufWriter, Stdout};
use tracing::debug;

/// process a single message
/// # Errors
/// * error happens on stream close
pub async fn process_message<W>(stdout: &mut BufWriter<W>, message: &str) -> io::Result<()>
where
    W: AsyncWrite + Unpin + Send + 'static,
{
    debug!("Write: {}", message);
    stdout.write_all(message.as_bytes()).await?;
    if !message.ends_with('\n') {
        stdout.write_all(b"\n").await?;
    }
    stdout.flush().await?;
    Ok(())
}
