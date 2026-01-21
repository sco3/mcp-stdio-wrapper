use crate::streamer::{McpStreamClient, SID};
use reqwest::Response;
use tracing::error;

impl McpStreamClient {
    /// saves session id for future use
    pub async fn process_session_id(&self, response: &Response) -> Option<String> {
        let id = if let Some(val) = response.headers().get(SID) {
            if let Ok(s) = val.to_str() {
                self.set_session_id(Some(s.to_string())).await;
                Some(s.to_string())
            } else {
                error!("Header contains invalid characters");
                None
            }
        } else {
            error!("Session id not found");
            None
        };
        id
    }
}
