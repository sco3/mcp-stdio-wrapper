use crate::post_result::PostResult;
use flume::Sender;
use tracing::error;
/// writes worker output to stdout
pub async fn write_output(i: usize, tx: &Sender<String>, res: PostResult) {
    // check every line
    for line in res.out.lines() {
        let sse_line = line.trim();

        let line_to_send = if res.sse {
            sse_line
                .strip_prefix("data: ")
                .map(str::trim)
                .filter(|s| !s.is_empty())
        } else {
            None // Directly return None if res.sse is false
        };
        } else {
            Some(sse_line)
        };

        if let Some(line) = line_to_send {
            if let Err(e) = tx.send_async(line.to_string()).await {
                error!("Worker {i}: failed to send to writer: {e}");
                break;
            }
        }
    }
}
