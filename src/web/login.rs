use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{Error, Result};

#[derive(Debug, Deserialize)]
struct LoginCmd {
    username: String,
    password: String,
}

async fn api_login(cmd: Json<LoginCmd>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if cmd.username != "user1" || cmd.password != "123456" {
        return Err(Error::LoginFailed);
    }

    let body = Json(json!({"result": { "hey": cmd.username, "success": true }}));

    Ok(body)
}

pub fn login_routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}
