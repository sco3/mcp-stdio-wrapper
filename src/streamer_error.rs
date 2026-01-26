use crate::json_rpc_header::parse_id;
use flume::Sender;
use jsonrpc_core::{serde_json, Error, ErrorCode, Failure, Id, Version};
use serde_json::json;
use tracing::error;

/// creates error message
pub async fn mcp_error(
    //
    worker_id: &usize,
    json_str: &str,
    error_msg: &str,
    tx: &Sender<String>,
) {
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
        jsonrpc: Some(Version::V2),
        error: error_obj,
        id: Id::Str(id_str.clone()),
    };

    let json_msg = match serde_json::to_string(&response) {
        Ok(msg) => msg,
        Err(e) => json!({
            "jsonrpc": "2.0",
            "error": {
                "code": -32603,
                "message": e.to_string()
            },
            "id": id_str
        })
        .to_string(),
    };

    if let Err(e) = tx.send_async(json_msg).await {
        error!("Worker {worker_id}: failed to send JSON-RPC response: {e}");
    }
}
