use axum::response::Response;
use axum::{middleware, Router};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;

pub use self::error::{Error, Result};

use crate::web::controller::AppState;
use crate::web::hello::hello_routes;
use crate::web::local::local_routes;
use crate::web::login::login_routes;
use crate::web::middleware::context::context_resolver;
use crate::web::tickets::ticket_routes;

mod context;
mod error;
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

async fn response_mapper(resp: Response) -> Response {
    println!("->> {:<12} - reponse_mapper", "RESP_MAPPER");

    println!();

    resp
}
