use crate::models::Chart;
use crate::schema::charts;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use bb8::Pool;
use diesel::Insertable;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use serde::Deserialize;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = charts)]
pub struct CreateChart {
    pub name: String,
    pub full_degree: f64,
    pub norm_degree: f64,
    pub speed: f64,
    pub is_retro: serde_json::Value,
    pub sign: String,
    pub sign_lord: String,
    pub nakshatra: String,
    pub nakshatra_lord: String,
    pub nakshatra_pad: i32,
    pub house: i32,
    pub is_planet_set: bool,
    pub planet_awastha: String,
    pub chart_name: String,
}

pub async fn save_chart(
    State(pool): State<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>>,
    Json(input): Json<CreateChart>,
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
