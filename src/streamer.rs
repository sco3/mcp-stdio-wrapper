use crate::config::Config;
use reqwest::Client;
use tokio::sync::RwLock;

pub const SID: &str = "mcp-session-id";

#[derive(Debug)]
pub struct McpStreamClient {
    pub(crate) client: Client,
    pub(crate) session_id: RwLock<Option<String>>,
    pub(crate) config: Config,
}
