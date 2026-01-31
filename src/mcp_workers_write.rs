use bytes::Bytes;
use crate::post_result::PostResult;
use flume::Sender;
use tracing::error;
/// writes worker output to stdout channel
pub async fn write_output(i: usize, tx: &Sender<Bytes>, res: PostResult) {
    // check every line
    for line in res.out.lines() {
        let line = if res.sse {
            line.strip_prefix("data: ").map_or("", str::trim)
        } else {
            line.trim()
        };
        if !line.is_empty() {
            if let Err(e) = tx.send_async(Bytes::from(line.to_string())).await {
                error!("Worker {i}: failed to send: {e}");
                break;
            }
        }
    }
}
