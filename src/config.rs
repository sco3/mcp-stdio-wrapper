use clap::Parser;
use serde::Deserialize;

#[derive(Deserialize, Debug, Parser)]
pub struct Config {
    /// Gateway MCP endpoint URL
    #[arg(long = "url")]
    pub mcp_server_url: String,

    /// Authorization header value
    #[serde(default = "crate::config_defaults::default_mcp_auth")]
    #[arg(long = "auth", default_value_t = crate::config_defaults::default_mcp_auth())]
    pub mcp_auth: String,

    /// Max concurrent tool calls
    #[serde(default = "crate::config_defaults::default_concurrency")]
    #[arg(long, default_value_t = crate::config_defaults::default_concurrency())]
    pub concurrency: usize,

    /// Logging level, or "off" to disable
    #[serde(default = "crate::config_defaults::default_mcp_wrapper_log_level")]
    #[arg(long="log-level", default_value_t = crate::config_defaults::default_mcp_wrapper_log_level())]
    pub mcp_wrapper_log_level: String,

    #[arg(long = "log-file")]
    pub mcp_wrapper_log_file: Option<String>,

    /// Response timeout in seconds
    #[serde(default = "crate::config_defaults::default_mcp_tool_call_timeout")]
    #[arg(long = "timeout", default_value_t = crate::config_defaults::default_mcp_tool_call_timeout())]
    pub mcp_tool_call_timeout: u64,
    /// Path to a custom CA certificate file (PEM format, e.g., .pem, .crt, .cert)
    #[arg(long = "tls-cert", value_name = "PATH")]
    pub tls_cert: Option<std::path::PathBuf>,
}
