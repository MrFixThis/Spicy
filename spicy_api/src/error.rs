use actix_web::{
    body,
    http::{self, StatusCode},
    HttpResponse, ResponseError,
};
use serde::Serialize;
use thiserror::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Error, Serialize)]
pub enum Error {
    #[error("auth error: {0}")]
    AuthError(String),

    #[error("fetch error: {1}")]
    FetchError(#[serde(skip_serializing)] StatusCode, String),

    #[error("internal error: {0}")]
    InternalError(String)
}

impl ResponseError for Error {
    fn status_code(&self) -> http::StatusCode {
        match self {
            Error::AuthError(_) => http::StatusCode::UNAUTHORIZED,
            Error::FetchError(sc, _) => sc.to_owned(),
            Error::InternalError(_) => todo!(),
        }
    }

    fn error_response(&self) -> HttpResponse<body::BoxBody> {
        HttpResponse::Unauthorized().json(self)
    }
}
