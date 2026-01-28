use crate::post_result::PostResult;
use flume::Sender;
use tracing::error;
/// writes worker output to stdout channel
pub async fn write_output(i: usize, tx: &Sender<String>, res: PostResult) {
    // check every line
    for line in res.out.lines() {
        let line = if res.sse {
            line.strip_prefix("data: ").map(str::trim).unwrap_or("")
        } else {
            line.trim()
        };
        if !line.is_empty() {
            if let Err(e) = tx.send_async(line.to_string()).await {
                error!("Worker {i}: failed to send: {e}");
                break;
            }
        }
    }
}
