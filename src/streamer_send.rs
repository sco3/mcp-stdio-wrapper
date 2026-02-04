use crate::streamer::{McpStreamClient, SID};
use reqwest::Response;

impl McpStreamClient {
    /// prepare and send request
    pub(crate) async fn prepare_and_send_request(
        &self, //
        payload: impl Into<reqwest::Body>,
    ) -> Result<Response, String> {
        let url = &self.config.mcp_server_url;
        let mut request = self.client.post(url).body(payload);

        for (key, value) in self.static_headers.iter() {
            request = request.header(key, value);
        }

        // Add dynamic session_id header if available
        if let Some(sid) = self.get_session_id().await {
            request = request.header(SID, sid);
        }

        let response = request
            .send()
            .await
            .map_err(|e| format!("Request failed: {e}"))?;
        Ok(response)
    }
}
