use crate::json_rpc_header::parse_id;
use jsonrpc_core::{serde_json, Error, ErrorCode, Failure, Id, Output};
use tracing::error;
/// creates error message
pub fn mcp_error(worker_id: &usize, json_str: &str, error_msg: &str) -> String {
    let id_str = match parse_id(json_str) {
        Ok(id) => {
            if let Some(s) = id.as_str() {
                s.to_string()
            } else {
                id.to_string()
            }
        }
        Err(e) => {
            tracing::debug!("Failed to parse json rpc id from '{}': {}", json_str, e);
            "<unknown id>".to_string()
        }
    };
    let error_obj = Error {
        code: ErrorCode::InternalError,
        message: error_msg.to_string(),
        data: None,
    };

    let response = Failure {
        jsonrpc: None, // The serializer will handle the "2.0" versioning
        error: error_obj,
        id: Id::Str(id_str),
    };

    serde_json::to_string_pretty(&response).unwrap()
    //error!("Worker {worker_id} rpc id: {id_str} Wrapper: MCP request failed: {error_msg}");
}