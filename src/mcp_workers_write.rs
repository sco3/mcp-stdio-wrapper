use crate::post_result::PostResult;
use bytes::Bytes;
use flume::Sender;
use tracing::error;

/// Trims leading and trailing ASCII whitespace from a byte slice.
fn trim_ascii_whitespace(bytes: &[u8]) -> &[u8] {
    let from = match bytes.iter().position(|x| !x.is_ascii_whitespace()) {
        Some(i) => i,
        None => return &[],
    };
    // This unwrap is safe because if `position` returns `Some`, `rposition` with the same predicate will also return `Some`.
    let to = bytes.iter().rposition(|x| !x.is_ascii_whitespace()).unwrap();
    &bytes[from..=to]
}

/// writes worker output to stdout channel
pub async fn write_output(i: usize, tx: &Sender<Bytes>, res: PostResult) {
    // By operating on byte slices (`&[u8]`), we can avoid intermediate
    // string allocations and UTF-8 validation overhead for each line.
    for line_bytes in res.out.as_bytes().split(|&c| c == b'\n') {
        let processed_line = if res.sse {
            // For SSE, strip "data: " prefix. If successful, trim the result.
            // If the prefix isn't found, the line is considered empty.
            match line_bytes.strip_prefix(b"data: ") {
                Some(stripped) => trim_ascii_whitespace(stripped),
                None => b"",
            }
        } else {
            // For non-SSE, just trim the line.
            trim_ascii_whitespace(line_bytes)
        };

        if !processed_line.is_empty() {
            // An owned copy is necessary to send data to another thread.
            if let Err(e) = tx.send_async(Bytes::copy_from_slice(processed_line)).await {
                error!("Worker {i}: failed to send: {e}");
                break;
            }
        }
    }
}
