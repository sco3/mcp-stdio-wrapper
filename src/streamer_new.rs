use crate::config::Config;
use crate::streamer::McpStreamClient;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{header, Client};
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error};

impl McpStreamClient {
    #[allow(unused)]
    /// Initialize the client with standard MCP headers
    ///
    /// # Errors
    ///
    /// * invalid auth header
    pub fn try_new(config: Config) -> Result<Self, header::InvalidHeaderValue> {
        let mut headers = header::HeaderMap::new();
        debug!("Headers: {headers:?}");
        headers.insert(
            ACCEPT,
            header::HeaderValue::from_static("application/json, text/event-stream"),
        );
        debug!("Headers: {headers:?}");
        headers.insert(
            CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        debug!("Headers: {headers:?}");
        if !config.mcp_auth.is_empty() {
            debug!("auth value: [{}]", config.mcp_auth);
            let auth_header = header::HeaderValue::from_str(&config.mcp_auth)?;
            headers.insert(AUTHORIZATION, auth_header);
        }
        debug!("Headers: {headers:?}");
        let timeout = config.mcp_tool_call_timeout;
        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(timeout))
            .build()
            .unwrap_or_else(|error| {
                // Log to standard error (standard for CLI tools)
                error!("Error: {error}");
                // Terminate with code 1 (or 255 for -1 equivalent)
                std::process::exit(1);
            });

        Ok(Self {
            client,
            session_id: RwLock::new(None),
            config,
        })
    }
}
