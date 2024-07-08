use crate::handlers::astro_details::astro_details_handler;
use crate::handlers::hello::hello_handler;
use axum::{routing::get, routing::post, Router};

pub fn create_routes() -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
        .route("/astro_details", post(astro_details_handler))
}
