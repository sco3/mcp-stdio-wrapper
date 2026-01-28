use crate::streamer::{McpStreamClient, SID};
use http::header;
use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Response;

impl McpStreamClient {
    /// prepare and send request
    pub(crate) async fn prepare_and_send_request(
        &self, //
        payload: impl Into<reqwest::Body>,
    ) -> Result<Response, String> {
        let url = &self.config.mcp_server_url;
        let mut request = self.client.post(url).body(payload);

        let sid = self.get_session_id().await;
        request = request.header(
            ACCEPT,
            header::HeaderValue::from_static(
                "application/json, application/x-ndjson, text/event-stream",
            ),
        );

        request = request.header(
            CONTENT_TYPE,
            header::HeaderValue::from_static("application/json; charset=utf-8"),
        );

        if self.is_auth() {
            let auth_header = header::HeaderValue::from_str(&self.config.mcp_auth)
                .map_err(|e| {
                    tracing::error!("Invalid auth header: {}", e); // Log the error
                    format!("Invalid auth header: {e}")
                })?;
            request = request.header(AUTHORIZATION, auth_header);
        }

        if let Some(sid) = sid {
            request = request.header(SID, sid);
        }

        let response = request
            .send()
            .await
            .map_err(|e| format!("Request failed: {e}"))?;
        Ok(response)
    }
}
