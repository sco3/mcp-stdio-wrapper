use crate::streamer::McpStreamClient;

impl McpStreamClient {
    #[allow(dead_code)]
    /// sets received session id
    pub fn set_session_id(&mut self, id: Option<String>) {
        if self.session_id.is_none() && id.is_some() {
            self.session_id = id;
        }
    }
    #[allow(dead_code)]
    ///  session id
    #[must_use]
    pub fn get_session_id(&self) -> Option<&String> {
        self.session_id.as_ref()
    }
}