use bytes::Bytes;
use crate::mcp_workers_write::write_output;
use crate::streamer::McpStreamClient;
use crate::streamer_error::mcp_error;
use flume::{Receiver, Sender};
use std::sync::Arc;
use tracing::{debug, error};

pub fn spawn_workers(
    concurrency: usize,
    mcp_client: &Arc<McpStreamClient>,
    input_rx: &Receiver<Bytes>,
    output_tx: Sender<Bytes>,
) {
    for i in 0..concurrency {
        let rx = input_rx.clone();
        let tx = output_tx.clone();
        let client = mcp_client.clone();

        tokio::spawn(async move {
            while let Ok(line) = rx.recv_async().await {
                debug!("Worker {i} processing message: {}", String::from_utf8_lossy(&line));
                let response = client.stream_post(line.clone()).await;
                match response {
                    Ok(res) => {
                        write_output(i, &tx, res).await;
                    }
                    Err(e) => {
                        error!("Worker {i}: Post failed: {e}");
                        let line_str = String::from_utf8_lossy(&line);
                        mcp_error(&i, &line_str, &e, &tx).await;
                    }
                }
            }
            debug!("Worker {} shutting down", i);
        });
    }
    drop(output_tx);
}
