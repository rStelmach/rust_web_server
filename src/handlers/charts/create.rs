use crate::models::{Chart, NewChart};
use crate::schema::charts;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;

pub async fn save_chart(
    State(pool): State<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>>,
    Json(input): Json<NewChart>,
) -> Result<Json<Chart>, StatusCode> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let new_chart: Chart = diesel::insert_into(charts::table)
        .values(&input)
        .get_result(&mut conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(new_chart))
}
