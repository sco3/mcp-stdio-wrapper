use flume::Sender;
use futures::StreamExt;
use reqwest::{Client, header};
use std::time::Duration;
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
            header::ACCEPT,
            header::HeaderValue::from_static("text/event-stream"),
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .tcp_keepalive(Duration::from_secs(60))
            .build()
            .expect("Failed to build reqwest client");

        Self { client, url }
    }

    /// Opens a stream and pumps raw chunks into the provided flume channel
    pub async fn stream_post(&self, payload: String, tx: Sender<String>) -> Result<(), String> {
        let response = self
            .client
            .post(&self.url)
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
                    let raw_str = String::from_utf8_lossy(&bytes).into_owned();

                    if tx.send_async(raw_str).await.is_err() {
                        break;
                    }
                }
                Err(e) => return Err(format!("Stream interrupted: {}", e)),
            }
        }

        Ok(())
    }
}
