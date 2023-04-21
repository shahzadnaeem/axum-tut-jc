use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::web::AUTH_COOKIE_NAME;
use crate::{Error, Result};

pub async fn require_auth<B>(cookies: Cookies, req: Request<B>, next: Next<B>) -> Result<Response> {
    println!("->> {:<12} - require_auth", "MIDDLEWARE");

    let cookie = cookies.get(AUTH_COOKIE_NAME).map(|c| c.value().to_string());

    // TODO: Auth token parsing and validation...

    let (user_id, _exp, _sig) = cookie.ok_or(Error::AuthNoCookie).and_then(parse_token)?;

    println!("-++ {:<12}     User ID: {}", "", user_id);

    Ok(next.run(req).await)
}

// Parse token value into (user_id, expiry, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_all, user_id, exp, sig) =
        regex_captures!(r#"user-(\d+)\.(.+)\.(.+)"#, &token).ok_or(Error::AuthTokenInvalid)?;

    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthTokenInvalid)?;

    Ok((user_id, exp.to_string(), sig.to_string()))
}
