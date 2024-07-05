use crate::handlers::hello::hello_handler;
use axum::Router;

pub fn create_routes() -> Router {
    Router::new().route("/hello", axum::routing::get(hello_handler))
}
