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

    let json_msg = match serde_json::to_string(&response) {
        Ok(msg) => msg,
        Err(e) => get_error(&id, &e),
    };

    if let Err(e) = tx.send_async(Bytes::from(json_msg)).await {
        error!("Worker {worker_id}: failed to send JSON-RPC response: {e}");
    }
}
/// creates error message
#[must_use]
pub fn get_error(id: &Id, e: &serde_json::Error) -> String {
    json!({
        "jsonrpc": "2.0",
        "error": {"code": ErrorCode::InternalError,"message": e.to_string()},
        "id": id
    })
    .to_string()
}
