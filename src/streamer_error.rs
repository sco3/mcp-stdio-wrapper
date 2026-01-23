use crate::json_rpc_header::parse_id;
use tracing::error;

pub fn mcp_error(worker_id: &usize, json_str: &str, error_msg: &str) {
    let id_str = match parse_id(json_str) {
        Ok(id) => if let Some(s) = id.as_str() {
            s.to_string()
        } else {
            id.to_string()
        },
        Err(e) => {
            tracing::debug!("Failed to parse json rpc id from '{}': {}", json_str, e);
            "<unknown id>".to_string()
        }
    };
    error!("Worker {worker_id} rpc id: {id_str} Wrapper: MCP request failed: {error_msg}");
}
