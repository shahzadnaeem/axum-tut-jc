use axum::response::Response;
use axum::{middleware, Router};
use std::net::SocketAddr;

pub use self::error::{Error, Result};

use crate::web::hello::hello_routes;
use crate::web::local::local_routes;
use crate::web::login::login_routes;

mod error;
mod web;

#[tokio::main]
async fn main() {
    // Start server ...

    let routes = Router::new()
        .merge(hello_routes())
        .merge(login_routes())
        .layer(middleware::map_response(response_mapper))
        .fallback_service(local_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on http://{addr} - http://{addr}/hello\n");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

async fn response_mapper(resp: Response) -> Response {
    println!("->> {:<12} - reponse_mapper", "RESP_MAPPER");

    println!();

    resp
}
