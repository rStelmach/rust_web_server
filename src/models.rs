use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::schema::charts;

#[derive(Queryable, Serialize)]
pub struct Chart {
    pub id: i32,
    pub chart_name: String,
    pub chart_data: Value, // JSONB field to store the entire chart data
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = charts)]
pub struct NewChart {
    pub chart_name: String,
    pub chart_data: Value, // JSONB field to store the entire chart data
}
