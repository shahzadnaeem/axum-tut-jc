use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .route("/hello", get(hello_handler))
        .route("/hello2/:name", get(hello2_handler));

    // region: --- Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on http://{addr} - http://{addr}/hello\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
    // endregion:

    // region: --- Hello handler

    #[derive(Debug, Deserialize)]
    struct HelloParams {
        name: Option<String>,
    }

    async fn hello_handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
        println!("--> {:<12} - hello_handler - {params:?}", "HANDLER");

        let name = params.name.as_deref().unwrap_or("World");

        Html(format!("Hello <strong>{name}</stong>"))
    }

    async fn hello2_handler(Path(name): Path<String>) -> impl IntoResponse {
        println!("--> {:<12} - hello_handler - {name:?}", "HANDLER");

        Html(format!("Hello <strong>{name}</stong>"))
    }

    // endregion:
}
