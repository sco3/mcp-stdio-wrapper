use flume::{Receiver, Sender};
use tracing::{debug, info};

pub fn spawn_workers(concurrency: usize, input_rx: Receiver<String>, output_tx: Sender<String>) {
    for i in 0..concurrency {
        let rx = input_rx.clone();
        let tx = output_tx.clone();

        tokio::spawn(async move {
            info!("Worker {} started", i);
            while let Ok(line) = rx.recv_async().await {
                debug!("Worker {i} processing message: {line}");
                
                let response = line;
                if tx.send_async(response).await.is_err() {
                    break;
                }
            }
            info!("Worker {} shutting down", i);
        });
    }
}
