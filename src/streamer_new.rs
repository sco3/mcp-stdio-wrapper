use crate::config::Config;
use crate::streamer::McpStreamClient;
use reqwest::{header, Client};
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::error;

impl McpStreamClient {
    #[allow(unused)]
    /// Initialize the client with standard MCP headers
    ///
    /// # Errors
    ///
    /// * invalid auth header
    pub fn try_new(config: Config) -> Result<Self, header::InvalidHeaderValue> {
        let timeout = config.mcp_tool_call_timeout;
        let client = Client::builder()
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
