use crate::handlers::{
    astro_details::astro_details_handler,
    charts::{create::save_chart, delete::delete_chart, get::get_all_charts},
    hello::hello_handler,
};
use axum::{
    routing::{delete, get, post},
    Router,
};
use bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub fn create_routes(pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>) -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
        .route("/astro_details", post(astro_details_handler))
        .route("/charts", post(save_chart))
        .route("/charts/:id", delete(delete_chart))
        .route("/charts", get(get_all_charts))
        .with_state(pool)
}
