use crate::models::Chart;
use crate::schema::charts::dsl::*;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn get_all_charts(
    State(pool): State<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>>,
) -> Result<Json<Vec<Chart>>, StatusCode> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = charts
        .load::<Chart>(&mut conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}
