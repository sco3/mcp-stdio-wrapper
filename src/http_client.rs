use crate::config::Config;
use crate::streamer_error::{build_error, invalid_error, read_error};
use reqwest::Client;
use std::fs::read;
use std::time::Duration;

pub fn get_http_client(config: &Config) -> Result<Client, String> {
    let pool_size = config.http_pool_size.unwrap_or(2);
    let mut build = Client::builder()
        .timeout(Duration::from_secs(config.mcp_tool_call_timeout))
        .http2_prior_knowledge()
        .tcp_nodelay(true)
        .pool_max_idle_per_host(pool_size)
        .pool_idle_timeout(Duration::from_secs(90));

    if let Some(cert_path) = &config.tls_cert {
        let cert_bytes = read(cert_path).map_err(|e| read_error(cert_path, &e))?;
        let cert = reqwest::Certificate::from_pem(&cert_bytes)
            .map_err(|e| invalid_error(cert_path, &e))?;
        build = build.add_root_certificate(cert);
    }

    build.build().map_err(|e| build_error(&e))
}
