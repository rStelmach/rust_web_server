use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use bb8::Pool;
use diesel::prelude::*;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;

pub async fn delete_chart(
    Path(chart_id): Path<i32>,
    State(pool): State<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>>,
) -> Result<Json<bool>, axum::http::StatusCode> {
    use crate::schema::charts::dsl::*;

    let mut conn = pool
        .get()
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    diesel::delete(charts.filter(id.eq(chart_id)))
        .execute(&mut conn)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(true))
}
