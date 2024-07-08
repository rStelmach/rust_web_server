use axum::{http::StatusCode, response::IntoResponse};
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub struct AstroError {
    pub message: String,
}

impl AstroError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl IntoResponse for AstroError {
    fn into_response(self) -> axum::response::Response {
        let status_code = StatusCode::INTERNAL_SERVER_ERROR;
        let body = format!("{{\"error\": \"{}\"}}", self.message);
        (status_code, body).into_response()
    }
}

impl fmt::Display for AstroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
