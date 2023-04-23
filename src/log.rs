use std::time::{SystemTime, UNIX_EPOCH};

use crate::{context::Context, error::ClientError, Error, Result};
use axum::http::{Method, StatusCode, Uri};
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
    req_status: String,

    // Error details
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}

pub async fn log_request(
    uuid: Uuid,
    method: Method,
    uri: Uri,
    status: StatusCode,
    context: Option<Context>,
    service_error: Option<&Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let error_type = service_error.map(|se| se.as_ref().to_string());

    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: ts.to_string(),
        req_path: uri.path().to_string(),
        req_method: method.to_string(),
        req_status: status.to_string(),
        user_id: context.map(|c| c.user_id()),
        client_error_type: client_error.map(|e| e.as_ref().to_string()),
        error_type,
        error_data,
    };

    println!("    ->> LOG:\n{}", json!(log_line));

    // TODO: Send to log - local and/or external system...

    Ok(())
}
