use axum::response::Response;
use axum::{middleware, Router};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;

pub use self::error::{Error, Result};

use crate::web::controller::AppState;
use crate::web::hello::hello_routes;
use crate::web::local::local_routes;
use crate::web::login::login_routes;
use crate::web::tickets::ticket_routes;

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // Controller layer - currently only Tickets functionality
    let controller = AppState::new().await?;

    // Set up the routes
    let routes = Router::new()
        .merge(hello_routes())
        .merge(login_routes())
        .nest("/api", ticket_routes(controller.clone()))
        .layer(middleware::map_response(response_mapper))
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
