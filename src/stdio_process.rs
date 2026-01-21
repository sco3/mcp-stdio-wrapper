use flume::Receiver;
use tokio::io::{self, AsyncWriteExt, BufWriter, Stdout};
use tracing::{debug, error, info};

/// process a single message
pub async fn process_message(stdout: &mut BufWriter<Stdout>, message: &str) -> io::Result<()> {
    debug!("Write: {}", message);
    stdout.write_all(message.as_bytes()).await?;
    if !message.ends_with('\n') {
        stdout.write_all(b"\n").await?;
    }
    stdout.flush().await?;
    Ok(())
}
