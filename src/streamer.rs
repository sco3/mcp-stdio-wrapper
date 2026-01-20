use futures::StreamExt;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::{header, Client};
use std::time::Duration;
use tracing::error;

use crate::post_result::PostResult;

#[derive(Debug)]
pub struct McpStreamClient {
    client: Client,
    url: String,
}

impl McpStreamClient {
    #[allow(unused)]
    /// Initialize the client with standard MCP/SSE headers
    pub fn new(url: String) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            ACCEPT,
            header::HeaderValue::from_static("application/json, text/event-stream"),
        );
        headers.insert(
            CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .tcp_keepalive(Duration::from_secs(60))
            .build()
            .unwrap_or_else(|error| {
                // Log to standard error (standard for CLI tools)
                error!("Error: {error}");
                // Terminate with code 1 (or 255 for -1 equivalent)
                std::process::exit(1);
            });

        Self { client, url }
    }
    #[allow(dead_code)]
    /// Opens a stream and pumps raw chunks into the provided flume channel
    /// # Errors
    ///
    /// This function will return an error if the `reqwest` fails
    pub async fn stream_post(&self, payload: String) -> Result<PostResult, String> {
        let mut result = String::new();
        let response = self
            .client
            .post(&self.url)
            .body(payload)
            .send()
            .await
            .map_err(|e| format!("Request failed: {e}"))?;

        if !response.status().is_success() {
            return Err(format!("Server error: {}", response.status()));
        }

        let id = match response.headers().get("mcp-session-id") {
            Some(val) => match val.to_str() {
                Ok(s) => Some(s.to_string()),
                Err(_) => {
                    error!("Header contains invalid characters");
                    None
                }
            },
            None => {
                error!("Mcp-Session-Id not found");
                None
            }
        };

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
