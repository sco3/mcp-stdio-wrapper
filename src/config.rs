use clap::Parser;
use serde::Deserialize;

pub const DEFAULT_LOG_LEVEL: &str = "off";
pub const DEFAULT_CONCURRENCY: usize = 10;
pub const DEFAULT_AUTH: Option<&str> = None;

#[derive(Deserialize, Debug, Parser)]
pub struct Config {
    /// Gateway MCP endpoint URL
    #[arg(long = "url", env = "MCP_SERVER_URL")]
    pub mcp_server_url: String,

    /// Authorization header value
    #[arg(long = "auth", env = "MCP_AUTH")]
    pub mcp_auth: Option<String>,

    /// Max concurrent tool calls
    #[arg(long, default_value_t = DEFAULT_CONCURRENCY, env = "CONCURRENCY")]
    pub concurrency: usize,

    #[arg(
	   long="log-level", 
	   default_value_t = String::from(DEFAULT_LOG_LEVEL),
	   env="LOG_LEVEL"
	)]
    pub mcp_wrapper_log_level: String,

    #[arg(short, long = "log-file", env = "MCP_LOG_FILE")]
    pub mcp_wrapper_log_file: Option<String>,

    /// Response timeout in seconds
    #[arg(long = "timeout", default_value_t = 60, env = "MCP_TOOL_CALL_TIMEOUT")]
    pub mcp_tool_call_timeout: u64,

    /// Path to a custom CA certificate file (PEM format, e.g., .pem, .crt, .cert)
    #[arg(long = "tls-cert", value_name = "PATH", env = "TLS_CERT")]
    pub tls_cert: Option<std::path::PathBuf>,

    /// Content type header to send to server
    #[arg(
        long,
        short = 'c',
        default_value = "application/json",
        env = "MCP_CONTENT_TYPE"
    )]
    pub mcp_content_type: String,
}
