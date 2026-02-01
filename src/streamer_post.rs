use crate::post_result::PostResult;
use crate::streamer::McpStreamClient;
use bytes::{Bytes, BytesMut};
use futures::StreamExt;
use reqwest::header::CONTENT_TYPE;
use tracing::{debug, error};

impl McpStreamClient {
    #[allow(dead_code)]
    /// Opens a stream and pumps raw chunks into the provided flume channel
    /// # Errors
    ///
    /// This function will return an error if the `reqwest` fails
    pub async fn stream_post(&self, payload: Bytes) -> Result<PostResult, String> {
        let response = self.prepare_and_send_request(payload).await?;
        let status = response.status();

        if !status.is_success() {
            let err_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Could not read error body".to_string());

            error!("Server returned error {}: {}", status, err_text);
            return Err(format!("Server error {status}: {err_text}"));
        }

        let is_sse = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|s| s.contains("text/event-stream"));

        let id = self.process_session_id(&response).await;

        let mut lines = Vec::new();
        let mut buffer = BytesMut::new();
        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            match item {
                Ok(chunk) => {
                    buffer.extend_from_slice(&chunk);

                    while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                        // split_to is O(1) and zero-copy
                        let mut line = buffer.split_to(pos + 1);

                        if line.last() == Some(&b'\n') {
                            line.truncate(line.len() - 1);
                        }
                        if line.last() == Some(&b'\r') {
                            line.truncate(line.len() - 1);
                        }

                        if !line.is_empty() {
                            lines.push(line.freeze());
                        }
                    }
                }
                Err(e) => return Err(format!("Stream interrupted: {e}")),
            }
        }

        if !buffer.is_empty() {
            lines.push(buffer.freeze());
        }
        debug!("Received lines: {lines:?}");

        Ok(PostResult {
            out: lines,
            session_id: id,
            sse: is_sse,
        })
    }
}
