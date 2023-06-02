pub(crate) mod handler_rest;
pub(crate) mod handler_websocket;

use crate::server::handler::handler_rest::aya::{
    current_epoch, latest_epoch_change, retry_epoch_event,
};

use super::{handler::handler_websocket::handle_ws_client, models::Clients};
use handler_rest::info;
use ratelimit_meter::{DirectRateLimiter, LeakyBucket};
use rweb::*;

/// REST API
#[router("/api", services(alive, info, aya))]
//#[header("Authorization", "*")]
#[openapi(id = "api", description = " REST API")]
#[cors(origins("*"), methods(get, post, option,), headers("*"), max_age = 600)]
pub async fn api() {}

/// Ayad endpoints
#[router(
    "/aya",
    services(retry_epoch_event, latest_epoch_change, current_epoch)
)]
#[openapi(id = "api.aya", description = "Aya Requests")]
pub async fn aya() {}

#[router("/ws", services(handle_ws_client))]
#[cors(
    origins("*"),
    methods(get, post, patch, delete, upgrade),
    headers("*"),
    max_age = 600
)]
pub fn ws(#[data] rate_limiter: DirectRateLimiter<LeakyBucket>, #[data] clients: Clients) {}

#[get("/alive")]
#[openapi(
    id = "api.alive",
    description = "Liveness signal returning OK 200",
    tags("Utils"),
    summary = "Liveness signal to check service availablity"
)]
pub fn alive() -> Result<impl rweb::warp::Reply, http::Error> {
    Ok(rweb::warp::reply::with_status(
        "I am alive".to_string(),
        rweb::warp::http::StatusCode::OK,
    ))
}

pub fn make_error(
    e: String,
    c: Option<i64>,
    d: Option<&str>,
) -> Result<Json<serde_json::Value>, Rejection> {
    Ok(rweb::Json::from(
        serde_json::json!({ "error": e, "code": c, "description": d }),
    ))
}

/*

#[cors(
    origins("*"),
    methods(get, post, patch, delete),
    headers(
        "Access-Control-Allow-Origin",
        "Access-Control-Allow-Credentials",
        "Access-Control-Allow-Headers",
        "Access-Control-Allow-Methods",
        "Access-Control-Expose-Headers",
        "Access-Control-Max-Age",
        "Access-Control-Request-Headers",
        "Access-Control-Request-Method",
        "Origin",
        "XMLHttpRequest",
        "X-Requested-With",
        "Accept",
        "Content-Type",
        "Referer",
        "User-Agent",
        "sec-ch-ua",
        "sec-ch-ua-mobile",
        "sec-ch-ua-platform",
        "Accept-Encoding",
        "Accept-Language",
        "authorization",
        "Connection",
        "Content-Length",
        "Host",
        "Sec-Fetch-Dest",
        "Sec-Fetch-Mode",
        "Sec-Fetch-Site",
    ),
    max_age = 600
)]*/
