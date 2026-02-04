use crate::config::Config;
use crate::streamer::McpStreamClient;
use crate::streamer_error::{invalid_error, read_error};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use std::fs::read;
use std::time::Duration;
use tokio::sync::RwLock;

const ACCEPT_VALUES: &str = "application/json, application/x-ndjson, text/event-stream";

impl McpStreamClient {
    #[allow(unused)]
    /// Initialize the client with standard MCP headers
    /// # Errors
    /// * invalid auth header
    /// # Panics
    /// * wrong or missing tls certificate
    pub fn try_new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let mut client_builder =
            Client::builder() // http client
                .timeout(Duration::from_secs(config.mcp_tool_call_timeout));

        // Add root certificate if specified
        if let Some(cert_path) = &config.tls_cert {
            let cert_bytes = read(cert_path) // may fail
                .map_err(|e| read_error(cert_path, e))?;

            let cert = reqwest::Certificate::from_pem(&cert_bytes)
                .map_err(|e| invalid_error(cert_path, e))?;

            client_builder = client_builder.add_root_certificate(cert);
        }

        let client = client_builder.build()?;

        // Build static headers once during initialization
        let mut static_headers = HeaderMap::new();
        static_headers.insert(ACCEPT, HeaderValue::from_static(ACCEPT_VALUES));
        let cont_type = HeaderValue::from_str(&config.mcp_content_type)?;
        static_headers.insert(CONTENT_TYPE, cont_type);

        // Add authorization header if configured
        if let Some(auth) = config.mcp_auth.as_ref() {
            let auth_header = HeaderValue::from_str(auth)?;
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
