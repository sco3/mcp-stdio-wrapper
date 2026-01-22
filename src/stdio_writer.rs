use crate::stdio_process::process_message;
use flume::Receiver;
use tokio::io::{self, BufWriter, AsyncWrite};
use tracing::{error, info};



// We make the function generic over W (any AsyncWriter)
pub fn spawn_writer<W>(rx: Receiver<String>, mut writer: W) -> tokio::task::JoinHandle<()>
where
    W: AsyncWrite + Unpin + Send + 'static
{
    tokio::spawn(async move {
        let mut stdout = BufWriter::new(writer);
        while let Ok(message) = rx.recv_async().await {
            if let Err(e) = process_message(&mut stdout, &message).await {
                error!("Failed to process message in writer: {}", e);
                break;
            }
        }
    })
}