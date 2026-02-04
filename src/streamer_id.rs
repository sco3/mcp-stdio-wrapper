use crate::streamer::McpStreamClient;
use std::sync::Arc;

impl McpStreamClient {
    #[allow(dead_code)]
    /// sets received session id
    pub async fn set_session_id(&self, id: Option<String>) {
        if self.session_id.load().is_some() || id.is_none() {
            return;
        }
        self.session_id.store(Arc::new(id));
    }
    #[allow(dead_code)]
    ///  session id
    #[must_use]
    pub async fn get_session_id(&self) -> Option<String> {
        let guard = self.session_id.load();
        (**guard).clone()
    }

    /// Returns `true` if the MCP session has been initialized
    pub async fn is_ready(&self) -> bool {
        self.session_id.load().is_some()
    }
}
