use crate::streamer::McpStreamClient;
use crate::streamer_error::mcp_error;
use flume::{Receiver, Sender};
use std::sync::Arc;
use tracing::{debug, error, info};

pub fn spawn_workers(
    concurrency: usize,
    mcp_client: &Arc<McpStreamClient>,
    input_rx: &Receiver<String>,
    output_tx: Sender<String>,
) {
    for i in 0..concurrency {
        let rx = input_rx.clone();
        let tx = output_tx.clone();
        let client = mcp_client.clone();

        tokio::spawn(async move {
            debug!("Worker {} started", i);
            while let Ok(line) = rx.recv_async().await {
                debug!("Worker {i} processing message: {line}");
                let response = client.stream_post(line.clone()).await;
                match response {
                    Ok(res) => {
                        // check every line
                        for sse_line in res.out.lines() {
                            let sse_line = sse_line.trim();
                            // take only "data: ..."
                            if let Some(clean_json) = sse_line.strip_prefix("data: ") {
                                let clean_json = clean_json.trim();
                                if !clean_json.is_empty() {
                                    if let Err(e) = tx.send_async(clean_json.to_string()).await {
                                        error!("Worker {i}: failed to send to writer: {e}");
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Worker {i}: Post failed: {e}");
                        mcp_error(&i, &line, &e);
                    }
                }
            }
            debug!("Worker {} shutting down", i);
        });
    }
    drop(output_tx);
}
