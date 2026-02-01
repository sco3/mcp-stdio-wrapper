#[must_use]
pub fn default_concurrency() -> usize {
    10
}

#[must_use]
pub fn default_mcp_wrapper_log_level() -> String {
    "off".to_string()
}
// #[must_use]
// pub fn default_mcp_wrapper_log_file() -> Option<String> {
//     None
// }
#[must_use]
pub fn default_mcp_tool_call_timeout() -> u64 {
    60
}
#[must_use]
pub fn default_mcp_auth() -> String {
    String::new()
}
