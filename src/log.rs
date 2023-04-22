use crate::{context::Context, error::ClientError, Error, Result};
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;

// Flat structure as this works well with log management tools...
#[skip_serializing_none] // This means ignore None values
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,
    timestamp: String, // ISO8601

    // User and context
    user_id: Option<u64>,

    // http request
    req_path: String,
    req_method: String,

    // Error details
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}

pub async fn log_request(
    uuid: Uuid,
    method: Method,
    uri: Uri,
    context: Option<Context>,
    service_error: Option<&Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    Ok(())
}
