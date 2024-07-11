mod config;
mod errors;
mod handlers;
mod routes;
mod state;

use state::establish_connection;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    tracing_subscriber::fmt::init();

    let db_pool = establish_connection();

    match db_pool.get() {
        Ok(_) => println!("Successfully connected to the database."),
        Err(err) => println!("Failed to connect to the database: {:?}", err),
    }

    let app = routes::create_routes();
    //db_pool: DbPool add to routes
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    log::info!("Listening on {:?}", listener.local_addr());
    axum::serve(listener, app).await.unwrap();
}
