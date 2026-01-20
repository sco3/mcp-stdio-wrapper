use crate::streamer::McpStreamClient;
use flume::{Receiver, Sender};
use std::sync::Arc;
use tracing::{debug, error, info};

pub fn spawn_workers(
    concurrency: usize,
    mcp_client: Arc<McpStreamClient>,
    input_rx: &Receiver<String>,
    output_tx: &Sender<String>,
) {
    for i in 0..concurrency {
        let rx = input_rx.clone();
        let tx = output_tx.clone();
        let client = mcp_client.clone();

        tokio::spawn(async move {
            info!("Worker {} started", i);
            while let Ok(line) = rx.recv_async().await {
                debug!("Worker {i} processing message: {line}");
                let response = client.stream_post(line).await;
                match response {
                    Ok(response) => {
                        if tx.send_async(response.out).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        error!("Post failed: {e}")
                    }
                }
            }
            info!("Worker {} shutting down", i);
        });
    }
}
