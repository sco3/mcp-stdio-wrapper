use crate::config::Config;
use crate::streamer::McpStreamClient;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::error;

impl McpStreamClient {
    #[allow(unused)]
    /// Initialize the client with standard MCP headers
    /// # Errors
    /// * invalid auth header
    /// # Panics
    /// * wrong or missing tls certificate
    pub fn try_new(config: Config) -> Result<Self, reqwest::header::InvalidHeaderValue> {
        let timeout = config.mcp_tool_call_timeout;
        let mut client_builder = Client::builder().timeout(Duration::from_secs(timeout));

        // Add root certificate if specified
        if let Some(tls_cert_path) = &config.tls_cert {
            let cert_bytes = std::fs::read(tls_cert_path).unwrap_or_else(|error| {
                panic!(
                    "Failed to read cert file {}: {}",
                    tls_cert_path.display(),
                    error
                );
            });
            let cert = reqwest::Certificate::from_pem(&cert_bytes).unwrap_or_else(|error| {
                panic!(
                    "Invalid PEM in cert file {}: {}",
                    tls_cert_path.display(),
                    error
                );
            });
            client_builder = client_builder.add_root_certificate(cert);
        }

        let client = client_builder.build().unwrap_or_else(|error| {
            // Log to standard error (standard for CLI tools)
            error!("Error: {error}");
            // Terminate with code 1 (or 255 for -1 equivalent)
            std::process::exit(1);
        });

        // Build static headers once during initialization
        let mut static_headers = HeaderMap::new();
        static_headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/json, application/x-ndjson, text/event-stream"),
        );
        static_headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str(&config.mcp_content_type)?,
        );

        // Add authorization header if configured
        if let Some(auth) = config.mcp_auth.clone() {
            let auth_header = HeaderValue::from_str(&auth)?;
            static_headers.insert(AUTHORIZATION, auth_header);
        }

        Ok(Self {
            client,
            session_id: RwLock::new(None),
            config,
            static_headers,
        })
    }
}
