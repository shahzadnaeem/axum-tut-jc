use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;

pub fn local_routes() -> Router {
    let routes_local = Router::new().nest_service("/", get_service(ServeDir::new("./")));

    routes_local
}
