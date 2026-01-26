use crate::json_rpc_header::find_first_id;
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
    let id = find_first_id(json_str).unwrap_or_else(|| {
        tracing::debug!("Failed to parse json rpc id from '{json_str}'");
        Id::Null
    });

    let error_obj = Error {
        code: ErrorCode::InternalError,
        message: error_msg.to_string(),
        data: None,
    };

    let response = Failure {
        jsonrpc: Some(Version::V2),
        error: error_obj,
        id: id.clone(),
    };

    let json_msg = match serde_json::to_string(&response) {
        Ok(msg) => msg,
        Err(_e) => make_json_message(id, error_msg),
    };

    if let Err(e) = tx.send_async(json_msg).await {
        error!("Worker {worker_id}: failed to send JSON-RPC response: {e}");
    }
}

#[must_use] 
pub fn make_json_message(id: Id, error_msg: &str) -> String {
    json!({
        "jsonrpc": Version::V2,
        "error": {"code": ErrorCode::InternalError,"message": error_msg},
        "id": id
    })
    .to_string()
}
