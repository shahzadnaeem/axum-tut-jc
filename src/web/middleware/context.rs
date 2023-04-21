use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::context::Context;
use crate::web::AUTH_COOKIE_NAME;
use crate::{Error, Result};

pub async fn context_resolver<B>(
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - context_resolver", "MIDDLEWARE");

    let cookie = cookies.get(AUTH_COOKIE_NAME).map(|c| c.value().to_string());

    let context = match cookie.ok_or(Error::AuthNoCookie).and_then(parse_token) {
        Ok((user_id, _exp, _sig)) => {
            println!("    {:<12}     User ID: {}", "", user_id);
            // TODO: Token validation here
            Ok(Context::new(user_id))
        }
        Err(e) => Err(e),
    };

    if context.is_err() && !matches!(context, Err(Error::AuthNoCookie)) {
        // Remove the cookie if it was not valid
        cookies.remove(Cookie::named(AUTH_COOKIE_NAME));
    }

    req.extensions_mut().insert(context);

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Context {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Context", "EXTRACTOR");

        let context = parts
            .extensions
            .get::<Result<Context>>()
            .ok_or(Error::AuthNoContextFound)?
            .clone();

        context
    }
}

// Parse token value into (user_id, expiry, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_all, user_id, exp, sig) =
        regex_captures!(r#"user-(\d+)\.(.+)\.(.+)"#, &token).ok_or(Error::AuthTokenInvalid)?;

    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthTokenInvalid)?;

    Ok((user_id, exp.to_string(), sig.to_string()))
}
