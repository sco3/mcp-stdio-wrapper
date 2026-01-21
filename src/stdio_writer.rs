use crate::stdio_process::process_message;
use flume::Receiver;
use tokio::io::{self, BufWriter};
use tracing::{error, info};

#[must_use]
pub fn spawn_writer(rx: Receiver<String>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut stdout = BufWriter::new(io::stdout());
        while let Ok(message) = rx.recv_async().await {
            if let Err(e) = process_message(&mut stdout, &message).await {
                error!("Failed to process message in writer: {}", e);
                break;
            }
        }
        info!("Writer task shutting down");
    })
}
