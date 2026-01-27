use crate::post_result::PostResult;
use flume::Sender;
use tracing::{debug, error};
/// writes worker output to stdout
pub async fn write_output(i: usize, tx: &Sender<String>, line: &String, res: PostResult) {
    // check every line
    for sse_line in res.out.lines() {
        let sse_line = sse_line.trim();
        if res.sse {
            // take only "data: ..."
            if let Some(clean_json) = sse_line.strip_prefix("data: ") {
                let clean_json = clean_json.trim();
                if !clean_json.is_empty() {
                    debug!("Worker {i} sends: {clean_json}");
                    if let Err(e) = tx.send_async(clean_json.to_string()).await {
                        error!("Worker {i}: failed to send to writer: {e}");
                        break;
                    }
                }
            }
        } else {
            debug!("Worker {i} sends: {sse_line}");
            if let Err(e) = tx.send_async(sse_line.to_string()).await {
                error!("Worker {i}: failed to send to writer: {e}");
                break;
            }
        }
    }
}
