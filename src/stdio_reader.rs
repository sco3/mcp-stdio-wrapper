use flume::Sender;
use tokio::io::{self, AsyncBufReadExt, AsyncRead, BufReader};
use tracing::debug;

/// stdio reader
pub fn spawn_reader<R>(tx: Sender<String>, reader: R)
where
    R: AsyncRead + Unpin + Send + 'static,
{
    tokio::spawn(async move {
        let mut reader = BufReader::new(reader).lines();

        while let Ok(Some(line)) = reader.next_line().await {
            debug!("Read: {line}");
            if tx.send_async(line).await.is_err() {
                break;
            }
        }
    });
}
