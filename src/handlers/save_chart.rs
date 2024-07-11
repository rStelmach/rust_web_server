use axum::response::Json;
use serde_json::json;

pub async fn save_chart_handler() -> Json<serde_json::Value> {
    log::info!("message sent");
    Json(json!({ "message": "hello" }))
}
