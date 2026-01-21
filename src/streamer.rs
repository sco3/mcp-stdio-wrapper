use reqwest::Client;
use tokio::sync::RwLock;
use crate::config::Config;

pub const SID: &str = "mcp-session-id";

#[derive(Debug)]
pub struct McpStreamClient {
    pub(crate) client: Client,
    pub(crate) session_id: RwLock<Option<String>>,
    pub(crate) config:Config,
}
