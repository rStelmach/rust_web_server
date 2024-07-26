use crate::models::Chart;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use tracing::error;

pub async fn get_all_charts(
    State(pool): State<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>>,
) -> Result<Json<Vec<Chart>>, StatusCode> {
    use crate::schema::charts::dsl::*;

    let mut conn = pool.get().await.map_err(|err| {
        error!("Failed to get DB connection from pool: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let results = charts.load::<Chart>(&mut conn).await.map_err(|err| {
        error!("Failed to load charts from DB: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(results))
}
