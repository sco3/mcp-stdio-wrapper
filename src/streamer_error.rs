use crate::json_rpc_id_fast::parse_id_fast;
use bytes::Bytes;
use flume::Sender;
use jsonrpc_core::{serde_json, Error, ErrorCode, Failure, Id, Version};
use serde_json::json;
use tracing::error;

/// creates error message
pub async fn mcp_error(
    //
    worker_id: &usize,
    json_str: &[u8],
    error_msg: &str,
    tx: &Sender<Bytes>,
) {
    let id = parse_id_fast(json_str);
    tracing::debug!("Json rpc id:{id:?}");
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

    let json_bytes = match serde_json::to_vec(&response) {
        Ok(bytes) => Bytes::from(bytes),
        Err(e) => get_error_bytes(id, &e),
    };

    if let Err(e) = tx.send_async(json_bytes).await {
        error!("Worker {worker_id}: failed to send JSON-RPC response: {e}");
    }
}
/// creates error message as bytes
pub fn get_error_bytes(id: Id, e: &serde_json::Error) -> Bytes {
    let error_json = json!({
        "jsonrpc": "2.0",
        "error": {"code": ErrorCode::InternalError,"message": e.to_string()},
        "id": id
    });
    
    // Use to_vec to avoid intermediate String allocation
    match serde_json::to_vec(&error_json) {
        Ok(bytes) => Bytes::from(bytes),
        // Fallback to a simple error message if serialization fails
        Err(_) => Bytes::from_static(b"{\"jsonrpc\":\"2.0\",\"error\":{\"code\":-32603,\"message\":\"Internal error\"},\"id\":null}"),
    }
}
