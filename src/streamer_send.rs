use crate::streamer::{McpStreamClient, SID};
use bytes::Bytes;
use http::{Request, Response};
use http_body_util::Full;
use hyper::body::Incoming;

impl McpStreamClient {
    /// prepare and send request
    pub(crate) async fn prepare_and_send_request(
        &self,
        payload: Bytes,
    ) -> Result<Response<Incoming>, String> {
        let url = &self.config.mcp_server_url;
        
        // Parse the URL
        let uri = url.parse::<http::Uri>()
            .map_err(|e| format!("Invalid URL: {e}"))?;

        // Build the request
        let mut request_builder = Request::builder()
            .method("POST")
            .uri(uri);

        // Add static headers
        for (key, value) in &self.static_headers {
            request_builder = request_builder.header(key, value);
        }

        // Add dynamic session_id header if available
        if let Some(sid) = self.get_session_id().await {
            request_builder = request_builder.header(SID, sid);
        }

        let request = request_builder
            .body(Full::new(payload))
            .map_err(|e| format!("Failed to build request: {e}"))?;

        let response = self
            .client
            .request(request)
            .await
            .map_err(|e| format!("Request failed: {e}"))?;

        Ok(response)
    }
}