use diesel::prelude::*;
use serde::Serialize;
use serde_json::Value;

#[derive(Queryable, Serialize)]
pub struct Chart {
    pub id: i32,
    pub name: String,
    pub full_degree: f64,
    pub norm_degree: f64,
    pub speed: f64,
    pub is_retro: Value,
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
