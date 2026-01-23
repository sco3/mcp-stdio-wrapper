use crate::json_rpc_header::parse_id;
use tracing::error;

pub fn mcp_error(worker_id: &usize, json_str: &str, error_msg: &str) {
    let id_str = match parse_id(json_str) {
        Ok(id) => id.to_string(),
        Err(_) => "unknown".to_string(),
    };
    let msg = format!("Wrapper: MCP request failed: {error_msg}");
    error!("Worker {worker_id} rpc id: {id_str} {msg}");
}
