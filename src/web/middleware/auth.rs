use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

use crate::context::Context;
use crate::Result;

// NOTE: context can be one of three types
//       Context         - failure is managed before this function is called
//       Result<Context> - allows error condition to be managed here
//       Option<Context> - ignores errors, but still allows them to be handled here

pub async fn require_auth<B>(context: Context, req: Request<B>, next: Next<B>) -> Result<Response> {
    println!("->> {:<12} - require_auth", "MIDDLEWARE");

    let _user_id = context.user_id();

    Ok(next.run(req).await)
}
