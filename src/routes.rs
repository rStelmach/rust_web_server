use crate::handlers::{
    astro_details::astro_details_handler, hello::hello_handler, save_chart::save_chart_handler,
};
use axum::{routing::get, routing::post, Router};

pub fn create_routes() -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
        .route("/astro_details", post(astro_details_handler))
        .route("/save_chart", get(save_chart_handler))
}

//db_pool: DbPool add to routes
