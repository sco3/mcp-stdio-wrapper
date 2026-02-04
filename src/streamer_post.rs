use crate::post_result::PostResult;
use crate::streamer::McpStreamClient;
use crate::streamer_lines::extract_lines;
use bytes::{Bytes, BytesMut};
use http_body_util::BodyExt;
use tracing::{debug, error};

impl McpStreamClient {
    #[allow(dead_code)]
    /// Performs a streaming POST request and processes the response into lines of bytes.
    /// # Errors
    /// This function will return an error if the request or stream processing fails.
    pub async fn stream_post(&self, payload: Bytes) -> Result<PostResult, String> {
        let response = self.prepare_and_send_request(payload).await?;
        let status = response.status();

        if !status.is_success() {
            let body = response.into_body();
            let body_bytes = body
                .collect()
                .await
                .map_err(|e| format!("Failed to read error body: {e}"))?
                .to_bytes();
            
            let err_text = String::from_utf8_lossy(&body_bytes).to_string();
            error!("Server returned error {}: {}", status, err_text);
            return Err(format!("Server error {status}: {err_text}"));
        }

        let sse = response
            .headers()
            .get(http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|s| s.contains("text/event-stream"));

        let session_id = self.process_session_id(&response).await;

        let mut out = Vec::new();
        let mut buffer = BytesMut::new();
        let mut body = response.into_body();

        while let Some(frame_result) = body.frame().await {
            match frame_result {
                Ok(frame) => {
                    if let Some(chunk) = frame.data_ref() {
                        buffer.extend_from_slice(chunk);
                        extract_lines(&mut buffer, &mut out);
                    }
                }
                Err(e) => return Err(format!("Stream interrupted: {e}")),
            }
        }

        if !buffer.is_empty() {
            out.push(buffer.freeze());
        }
        debug!("Received lines: {out:?}");

        Ok(PostResult {
            session_id,
            out,
            sse,
        })
    }
}