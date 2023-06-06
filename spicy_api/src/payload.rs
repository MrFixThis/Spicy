use actix_web::http;
use serde::Serialize;

pub mod recipe_payload;
pub mod user_payload;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    status_code: u16,
    msg: String,
    err: Option<String>,
}

impl ErrorResponse {
    pub fn new(status_code: http::StatusCode, msg: &str, err: Option<String>) -> Self {
        Self {
            status_code: status_code.as_u16(),
            msg: msg.to_string(),
            err,
        }
    }
}
