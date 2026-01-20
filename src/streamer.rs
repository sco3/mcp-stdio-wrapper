use futures::StreamExt;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::{Client, header};
use std::time::Duration;

pub const INIT: &str = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"demo","version":"0.0.1"}}}"#;

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
            .expect("Failed to build reqwest client");

        Self { client, url }
    }
    #[allow(dead_code)]
    /// Opens a stream and pumps raw chunks into the provided flume channel
    pub async fn stream_post(&self, payload: String) -> Result<String, String> {
        let mut result = String::new();
        let response = self
            .client
            .post(&self.url)
            // .header(CONTENT_TYPE, "application/json")
            // .header(ACCEPT, "application/json, text/event-stream")
            .body(payload)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Server error: {}", response.status()));
        }

        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            match item {
                Ok(bytes) => {
                    let chunk = String::from_utf8_lossy(&bytes);
                    result.push_str(&chunk);
                }
                Err(e) => return Err(format!("Stream interrupted: {}", e)),
            }
        }

        Ok(result)
    }
}
