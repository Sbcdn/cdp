use super::error::RESTError;
use super::models::{Claims, ClaimsNR, Role};
use chrono::prelude::*;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use rweb::warp::{
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Rejection,
};

lazy_static! {
    static ref JWT_SECRET_KEY: String =
        std::env::var("JWT_KEY").unwrap_or_else(|_| "secret_key".to_string());
    static ref JWT_PUB_KEY: String =
        std::env::var("JWT_PUB_KEY").unwrap_or_else(|_| "pub_key".to_string());
}

const BEARER: &str = "Bearer ";

/// Creates an expiring (30 minutes) JWT token for the private-key set in env JWT_SECRET_KEY
pub fn _create_jwt(uid: &str, role: &Role, expire: Option<i64>) -> Result<String, RESTError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(expire.unwrap_or(0)))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uid.to_owned(),
        rpm: role.to_string(),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::ES256);
    let key = JWT_SECRET_KEY.to_owned().into_bytes();
    encode(&header, &claims, &EncodingKey::from_ec_pem(&key).unwrap())
        .map_err(|_| RESTError::JWTTokenCreationError)
}

/// Check validity of JWT-Token by decoding with its Public Key
pub async fn authorize(headers: HeaderMap<HeaderValue>) -> std::result::Result<String, Rejection> {
    let publ = JWT_PUB_KEY.to_owned().into_bytes();
    log::debug!("Try authorization");
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<ClaimsNR>(
                &jwt,
                &DecodingKey::from_ec_pem(&publ).unwrap(),
                &Validation::new(Algorithm::ES256),
            );
            let decoded = decoded.map_err(|_| reject::custom(RESTError::JWTTokenError))?;
            log::debug!("authorized");
            Ok(decoded.claims.sub)
        }
        Err(e) => Err(reject::custom(e)),
    }
}

/// Retrieve JWT token from header
fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String, RESTError> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(RESTError::NoAuthHeaderError),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(RESTError::NoAuthHeaderError),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(RESTError::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
