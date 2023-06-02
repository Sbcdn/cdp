use super::auth::authorize;
use http::{HeaderMap, HeaderValue};
use rweb::warp::{header::headers_cloned, Filter, Rejection};

pub fn with_auth() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (headers))
        .and_then(authorize)
}
