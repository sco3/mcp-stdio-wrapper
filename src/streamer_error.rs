use crate::json_rpc_header::parse_id;
use crate::streamer::McpStreamClient;
use flume::Sender;
use tracing::error;

pub async fn mcp_error(worker_id: &usize, json_str: &str, error_msg: &str, _tx: &Sender<String>) {
    let id = parse_id(&json_str);
    let msg = format!("Wrapper: MCP request failed: {error_msg}");
    error!("Worker {worker_id} rpc id: {id:?} {msg}");
}
