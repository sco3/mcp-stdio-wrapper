use crate::post_result::PostResult;
use crate::streamer::McpStreamClient;
use futures::StreamExt;
use tracing::error;

impl McpStreamClient {
    #[allow(dead_code)]
    /// Opens a stream and pumps raw chunks into the provided flume channel
    /// # Errors
    ///
    /// This function will return an error if the `reqwest` fails
    pub async fn stream_post(
        &self,
        payload: impl Into<reqwest::Body>,
    ) -> Result<PostResult, String> {
        let mut result = String::new();

        let response = self.prepare_and_send_request(payload).await?;

        let status = response.status();

        if !status.is_success() {
            // Attempt to get the error message from the server body
            let err_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Could not read error body".to_string());

            error!("Server returned error {}: {}", status, err_text);

            return Err(format!("Server error {status}: {err_text}"));
        }
        let id = self.process_session_id(&response).await;

        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            match item {
                Ok(bytes) => {
                    let chunk = String::from_utf8_lossy(&bytes);
                    result.push_str(&chunk);
                }
                Err(e) => return Err(format!("Stream interrupted: {e}")),
            }
        }

        Ok(PostResult {
            out: result,
            session_id: id,
        })
    }
}
