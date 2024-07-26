mod config;
mod errors;
mod handlers;
mod models;
mod routes;
mod schema;

use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_web_server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    log::info!("Database URL: {:?}", db_url);

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url.clone());
    log::info!("Config: {:?}", config);

    let pool = match bb8::Pool::builder().build(config).await {
        Ok(pool) => pool,
        Err(e) => {
            log::error!("Failed to create pool: {:?}", e);
            return;
        }
    };
    log::info!("Pool created: {:?}", pool);

    let app = routes::create_routes(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    log::info!("Listening on {:?}", listener.local_addr());
    axum::serve(listener, app).await.unwrap();
}
