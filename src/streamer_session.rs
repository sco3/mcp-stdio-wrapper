use crate::streamer::{McpStreamClient, SID};
use http::Response;
use tracing::{debug, error};

impl McpStreamClient {
    /// saves session id for future use
    pub async fn process_session_id<B>(&self, response: &Response<B>) -> Option<String> {
        if let Some(val) = response.headers().get(SID) {
            if let Ok(s) = val.to_str() {
                self.set_session_id(Some(s.to_string())).await;
                Some(s.to_string())
            } else {
                error!("Header contains invalid characters");
                None
            }
        } else {
            if !self.is_ready().await {
                debug!("Session id not found");
            }
            None
        }
    }
}
