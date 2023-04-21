use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::context::Context;
use crate::web::AUTH_COOKIE_NAME;
use crate::{Error, Result};

// NOTE: context can be one of three types
//       Context         - failure is managed before this function is called
//       Result<Context> - allows error condition to be managed here
//       Option<Context> - ignores errors, but still allows them to be handled here

pub async fn require_auth<B>(context: Context, req: Request<B>, next: Next<B>) -> Result<Response> {
    println!("->> {:<12} - require_auth", "MIDDLEWARE");

    let _user_id = context.user_id();

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Context {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Context", "EXTRACTOR");

        let cookies = parts.extract::<Cookies>().await.unwrap();

        let cookie = cookies.get(AUTH_COOKIE_NAME).map(|c| c.value().to_string());

        let (user_id, _exp, _sig) = cookie.ok_or(Error::AuthNoCookie).and_then(parse_token)?;

        println!("-++ {:<12}     User ID: {}", "", user_id);

        Ok(Context::new(user_id))
    }
}

// Parse token value into (user_id, expiry, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_all, user_id, exp, sig) =
        regex_captures!(r#"user-(\d+)\.(.+)\.(.+)"#, &token).ok_or(Error::AuthTokenInvalid)?;

    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthTokenInvalid)?;

    Ok((user_id, exp.to_string(), sig.to_string()))
}
