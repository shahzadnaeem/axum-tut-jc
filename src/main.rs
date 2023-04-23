use axum::http::{Method, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::{middleware, Json, Router};
use context::Context;
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use uuid::Uuid;

pub use self::error::{Error, Result};

use crate::log::log_request;
use crate::web::controller::AppState;
use crate::web::hello::hello_routes;
use crate::web::local::local_routes;
use crate::web::login::login_routes;
use crate::web::middleware::context::context_resolver;
use crate::web::tickets::ticket_routes;

mod context;
mod error;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // Controller layer - currently only Tickets functionality
    let state = AppState::new().await?;

    let api_auth = ticket_routes(state.clone())
        .route_layer(middleware::from_fn(web::middleware::auth::require_auth));

    // Set up the routes
    let routes = Router::new()
        .merge(hello_routes())
        .merge(login_routes())
        .nest("/api", api_auth)
        .layer(middleware::map_response(response_mapper))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            context_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(local_routes());

    // Start...
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on http://{addr} - http://{addr}/hello\n");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn response_mapper(
    context: Option<Context>,
    uri: Uri,
    req_method: Method,
    resp: Response,
) -> Response {
    println!("->> {:<12} - reponse_mapper", "RESP_MAPPER");

    let uuid = Uuid::new_v4();

    let response_error = resp.extensions().get::<Error>();
    let mapped_error = response_error.map(|s| s.to_client_error());

    // If we have an error, update the response to include the additional info
    let error_response = mapped_error.as_ref().map(|(status_code, client_error)| {
        let error_body = json!({
            "error": {
                "type": client_error.as_ref(),
                "req_uuid": uuid.to_string(),
            }
        });

        println!("    ->> client_error_body: {error_body}");

        (*status_code, Json(error_body)).into_response()
    });

    let (status, client_error) = mapped_error.map_or((StatusCode::OK, None), |(code, error)| {
        (code.clone(), Some(error.clone()))
    });

    // TODO: Add a per request server log line
    log_request(
        uuid,
        req_method,
        uri,
        status,
        context,
        response_error,
        client_error,
    )
    .await
    .ok();

    // println!("    ->> server log - {uuid} - Error: {response_error:?}");

    println!();

    // Use error if there was one, or the original good response
    error_response.unwrap_or(resp)
}
