use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "crate::config_defaults::default_concurrency")]
    pub concurrency: usize,
    #[serde(default = "crate::config_defaults::default_mcp_server_url")]
    pub mcp_server_url: String,
    #[serde(default = "crate::config_defaults::default_mcp_wrapper_log_level")]
    pub mcp_wrapper_log_level: String,
}
