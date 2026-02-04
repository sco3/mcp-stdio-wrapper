use clap::Parser;
use serde::Deserialize;

#[derive(Deserialize, Debug, Parser)]
pub struct Config {
    /// Gateway MCP endpoint URL
    #[arg(long = "url", env = "MCP_SERVER_URL")]
    pub mcp_server_url: String,

    /// Authorization header value
    #[arg(long = "auth", env="MCP_AUTH" , default_value_t = crate::config_defaults::default_mcp_auth())]
    pub mcp_auth: String,

    /// Max concurrent tool calls
    #[arg(long, default_value_t = crate::config_defaults::default_concurrency(), env="CONCURRENCY")]
    pub concurrency: usize,

    #[arg(
	   long="log-level", 
	   default_value_t = crate::config_defaults::default_mcp_wrapper_log_level(),
	   env="LOG_LEVEL"
	)]
    pub mcp_wrapper_log_level: String,

    #[arg(short, long = "log-file", env = "MCP_LOG_FILE")]
    pub mcp_wrapper_log_file: Option<String>,

    /// Response timeout in seconds
    #[arg(
	   long = "timeout", 
	   default_value_t = crate::config_defaults::default_mcp_tool_call_timeout(),
	   env="MCP_TOOL_CALL_TIMEOUT"
	)]
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
