use reqwest::Client;
#[derive(Debug)]
pub struct McpStreamClient {
    pub(crate) client: Client,
    pub(crate) url: String,
    pub(crate) session_id: Option<String>,
}

