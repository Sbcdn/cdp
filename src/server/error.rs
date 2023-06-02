use rweb::warp::{http::StatusCode, Rejection, Reply};
use std::convert::Infallible;
use thiserror::Error;

use super::models::ErrorResponse;

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum RESTError {
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("could not create jwt token")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("no permission")]
    NoPermissionError,
    #[error("internal error: {:?}", self)]
    Custom(String),
    #[error("too many requests")]
    RateLimitReachedError,
    #[error("internal error: {:?}", self)]
    InternalError,
}

impl rweb::warp::reject::Reject for RESTError {}

pub async fn _handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if let Some(e) = err.find::<RESTError>() {
        match e {
            RESTError::JWTTokenError => (StatusCode::UNAUTHORIZED, e.to_string()),
            RESTError::NoAuthHeaderError => {
                (StatusCode::NETWORK_AUTHENTICATION_REQUIRED, e.to_string())
            }
            RESTError::Custom(s) => (StatusCode::INTERNAL_SERVER_ERROR, s.to_string()),
            RESTError::RateLimitReachedError => (StatusCode::TOO_MANY_REQUESTS, e.to_string()),
            _ => (StatusCode::BAD_REQUEST, e.to_string()),
        }
    } else if err.find::<rweb::warp::reject::MethodNotAllowed>().is_some() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "Method Not Allowed".to_string(),
        )
    } else {
        eprintln!("unhandled error: {err:?}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    let json = rweb::warp::reply::json(&ErrorResponse {
        status: code.to_string(),
        message,
    });

    Ok(rweb::warp::reply::with_status(json, code))
}

impl From<std::string::String> for RESTError {
    fn from(err: std::string::String) -> Self {
        RESTError::Custom(err)
    }
}

impl From<jsonwebtoken::errors::Error> for RESTError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        RESTError::Custom(err.to_string())
    }
}
