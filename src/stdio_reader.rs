use flume::Receiver;
use tokio::io::{self, AsyncBufReadExt, BufReader};

/// stdio reader
pub(crate) fn spawn_reader() -> Receiver<String> {
    let (tx, rx) = flume::unbounded();

    tokio::spawn(async move {
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin).lines();

        while let Ok(Some(line)) = reader.next_line().await {
            if tx.send_async(line).await.is_err() {
                break;
            }
        }
    });

    rx
}
