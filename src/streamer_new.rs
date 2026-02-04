use crate::config::Config;
use crate::streamer::McpStreamClient;
use http::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use hyper_rustls::ConfigBuilderExt;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use tokio::sync::RwLock;
use tracing::error;

impl McpStreamClient {
    #[allow(unused)]
    /// Initialize the client with standard MCP headers
    /// # Errors
    /// * invalid auth header
    /// # Panics
    /// * wrong or missing tls certificate
    pub fn try_new(config: Config) -> Result<Self, http::header::InvalidHeaderValue> {
        let timeout = config.mcp_tool_call_timeout;

        // Install default crypto provider (required for rustls)
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

        // Build TLS configuration
        let mut tls_config = rustls::ClientConfig::builder()
            .with_native_roots()
            .map_err(|e| {
                error!("Failed to load native root certificates: {e}");
                std::process::exit(1);
            })
            .unwrap()
            .with_no_client_auth();

        // Add custom root certificate if specified
        if let Some(tls_cert_path) = &config.tls_cert {
            let cert_file = std::fs::File::open(tls_cert_path).unwrap_or_else(|error| {
                panic!(
                    "Failed to open cert file {}: {}",
                    tls_cert_path.display(),
                    error
                );
            });
            let mut reader = std::io::BufReader::new(cert_file);
            
            let certs = rustls_pemfile::certs(&mut reader)
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_else(|error| {
                    panic!(
                        "Failed to parse cert file {}: {}",
                        tls_cert_path.display(),
                        error
                    );
                });

            let mut root_store = rustls::RootCertStore::empty();
            for cert in certs {
                root_store.add(cert).unwrap_or_else(|error| {
                    panic!(
                        "Failed to add cert from {}: {}",
                        tls_cert_path.display(),
                        error
                    );
                });
            }

            tls_config = rustls::ClientConfig::builder()
                .with_root_certificates(root_store)
                .with_no_client_auth();
        }

        // Create HTTPS connector
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_tls_config(tls_config)
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build();

        // Build client with timeout
        let client = Client::builder(TokioExecutor::new())
            .build(https);

        // Build static headers once during initialization
        let mut static_headers = HeaderMap::new();
        static_headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/json, application/x-ndjson, text/event-stream"),
        );
        static_headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );

        // Add authorization header if configured
        if !config.mcp_auth.is_empty() {
            let auth_header = HeaderValue::from_str(&config.mcp_auth)?;
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