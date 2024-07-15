use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::schema::charts;

#[derive(Queryable, Serialize)]
pub struct Chart {
    pub id: i32,
    pub chart_name: String,
    pub chart_data: Value,
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub hour: i32,
    pub min: i32,
    pub lat: f64,
    pub lon: f64,
    pub tzone: f64,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = charts)]
pub struct NewChart {
    pub chart_name: String,
    pub chart_data: Value,
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub hour: i32,
    pub min: i32,
    pub lat: f64,
    pub lon: f64,
    pub tzone: f64,
}
