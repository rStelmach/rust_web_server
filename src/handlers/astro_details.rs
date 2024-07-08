use axum::Json;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use tracing::error;

use crate::errors::AstroError;

pub async fn astro_details_handler(
    Json(payload): Json<RequestPayload>,
) -> Result<Json<Vec<AstroApiResponseData>>, AstroError> {
    let astro_api_key = env::var("ASTRO_API_KEY").expect("ASTRO_API_KEY must be set");

    let client = Client::new();
    let response = client
        .post("https://json.astrologyapi.com/v1/planets")
        .header("Authorization", format!("Basic {}", astro_api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            error!("Request error: {:?}", e);
            AstroError::new(format!("Request error: {:?}", e))
        })?;

    let api_response: Vec<AstroApiResponseData> = response.json().await.map_err(|e| {
        error!("JSON deserialization error: {:?}", e);
        AstroError::new(format!("JSON deserialization error: {:?}", e))
    })?;
    log::info!("API response: {:?}", api_response);
    Ok(Json(api_response))
}

#[derive(Serialize, Deserialize)]
pub struct RequestPayload {
    day: String,
    month: String,
    year: String,
    hour: String,
    min: String,
    lat: String,
    lon: String,
    tzone: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    status: String,
    data: AstroApiResponseData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AstroApiResponseData {
    id: i32,
    name: String,
    #[serde(rename = "fullDegree")]
    full_degree: f64,
    #[serde(rename = "normDegree")]
    norm_degree: f64,
    speed: f64,
    #[serde(rename = "isRetro")]
    is_retro: Value,
    sign: String,
    #[serde(rename = "signLord")]
    sign_lord: String,
    nakshatra: String,
    #[serde(rename = "nakshatraLord")]
    nakshatra_lord: String,
    #[serde(rename = "nakshatra_pad")]
    nakshatra_pad: i32,
    house: i32,
    #[serde(rename = "is_planet_set")]
    is_planet_set: bool,
    #[serde(rename = "planet_awastha")]
    planet_awastha: String,
}
