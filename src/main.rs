mod config;
mod errors;
mod handlers;
mod routes;
mod state;

use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    tracing_subscriber::fmt::init();

    let app = routes::create_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    log::info!("Listening on {:?}", listener.local_addr());
    axum::serve(listener, app).await.unwrap();
}
