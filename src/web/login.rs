use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{
    web::{AUTH_COOKIE_NAME, DUMMY_AUTH_TOKEN},
    Error, Result,
};

#[derive(Debug, Deserialize)]
struct LoginCmd {
    username: String,
    password: String,
}

async fn api_login(cookies: Cookies, cmd: Json<LoginCmd>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if cmd.username != "user1" || cmd.password != "123456" {
        return Err(Error::LoginFailed);
    }

    // TODO: Do real JWT creation or similar here for 'value'
    cookies.add(Cookie::new(AUTH_COOKIE_NAME, DUMMY_AUTH_TOKEN));

    let body = Json(json!({"result": { "hey": cmd.username, "success": true }}));

    Ok(body)
}

pub fn login_routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}
