use crate::config::Config;
use bytes::Bytes;
use http::HeaderMap;
use http_body_util::Full;
use hyper_util::client::legacy::Client;
use tokio::sync::RwLock;

pub const SID: &str = "mcp-session-id";

pub type HttpsConnector = hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>;

#[derive(Debug)]
pub struct McpStreamClient {
    pub(crate) client: Client<HttpsConnector, Full<Bytes>>,
    pub(crate) session_id: RwLock<Option<String>>,
    pub(crate) config: Config,
    pub(crate) static_headers: HeaderMap,
}