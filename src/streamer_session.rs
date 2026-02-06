use crate::streamer::{McpStreamClient, SID};
use reqwest::Response;
use tracing::error;

impl McpStreamClient {
    /// saves session id for future use
    pub fn process_session_id(&self, response: &Response) -> Option<String> {
        if let Some(val) = response.headers().get(SID) {
            if let Ok(s) = val.to_str() {
                self.set_session_id(s);
                let r = s.to_string();
                return Some(r);
            }
            error!("Header contains invalid characters");
        }
        None
    }
}
