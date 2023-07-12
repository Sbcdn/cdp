mod auth;
mod error;
mod examples;
mod filter;
mod handler;

pub(crate) mod models;
extern crate pretty_env_logger;

use crate::server::models::LocalTxCache;

use self::models::Clients;
use nonzero_ext::nonzero;
use ratelimit_meter::{DirectRateLimiter, LeakyBucket};
use rweb::openapi::{
    Components, Contact, Example, ExampleValue, License, MediaTypeExample, ObjectOrReference,
    SecurityScheme,
};
use rweb::rt::IndexMap;
use rweb::warp::Filter;
use rweb::{openapi, openapi_docs};
use tokio::sync::Mutex;
use tonic::transport::Server;

use std::borrow::Cow;
use std::net::SocketAddr;
use std::{collections::HashMap, env, sync::Arc};

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "4123";

pub async fn serve(dbsync_url: Option<String>) -> Result<(), error::RESTError> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }

    if let Some(url) = dbsync_url {
        env::set_var("DBSYNC_URL", &url)
    }

    let host: String = env::var("POD_HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string());
    let port = env::var("POD_PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());

    //Websocket clients
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    //Transaction Cache
    let _tx_cache: LocalTxCache = Arc::new(Mutex::new(HashMap::new()));

    //Rate Limitation
    let lim =
        DirectRateLimiter::<LeakyBucket>::new(nonzero!(3u32), std::time::Duration::from_secs(5));
    let server = host.clone() + ":" + &port;
    let socket: std::net::SocketAddr = server.parse().expect("Unable to parse socket address");

    log::info!("Starting update loop for websocket");
    let (mut spec, filter) = openapi::spec().build(handler::api);

    let c = clients.clone();
    tokio::task::spawn(async move {
        handler::handler_websocket::main_worker(clients).await;
    });

    // Manipulating OpenApi Spec
    // Add title
    spec.info.title = Cow::from("Cardano Transaction Processor");
    spec.info.description = Cow::from("This service creates and process cardano transactions\n\n Example Bearer Token: eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NiJ9.eyJzdWIiOiIwIiwiZXhwIjozMTg3Nzk5Mzg1NjB9.Xl2zKYDqYeIC8BurqUiHbu6_Fu-nY2vNCyHY5A69twnSyGeBTm7mqCdY-ryfxrNfsOmCXjrHSYfQy6PClFV0sw\n ");
    spec.info.version = Cow::from("Version 0.1");
    spec.info.contact = Some(Contact {
        name: Cow::from("WorldMobile"),
        url: Some(reqwest::Url::parse("https://worldmobile.io").unwrap()),
        email: Cow::from("torben.poguntke@worldmobile.io"),
    });
    spec.info.license = Some(License {
        name: Cow::from("WorldMobileLicense"),
        url: Some(reqwest::Url::parse("https://worldmobile.io/license").unwrap()),
    });
    spec.info.terms_of_service =
        Some(reqwest::Url::parse("https://worldmobile.io/privacy").unwrap());
    // Add API Key Login
    let security = openapi::SecurityScheme::Http {
        scheme: Cow::from("bearer"),
        bearer_format: Cow::from("JWT"),
    };
    let mut security_map = IndexMap::<Cow<'static, str>, ObjectOrReference<SecurityScheme>>::new();
    security_map.insert(Cow::from("apiKey"), ObjectOrReference::Object(security));

    let example_value = ExampleValue::Embedded { value: serde_json::json!("eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NiJ9.eyJzdWIiOiIwIiwiZXhwIjozMTg3Nzk5Mzg1NjB9.Xl2zKYDqYeIC8BurqUiHbu6_Fu-nY2vNCyHY5A69twnSyGeBTm7mqCdY-ryfxrNfsOmCXjrHSYfQy6PClFV0sw") };
    let example = Example {
        summary: Cow::from("apiKey"),
        description: Cow::from("Login with JWT apiKey"),
        value: Some(example_value),
    };

    let mut example_map = IndexMap::<Cow<'static, str>, ObjectOrReference<Example>>::new();
    example_map.insert(Cow::from("apiKey"), ObjectOrReference::Object(example));

    let comp = Components {
        security_schemes: security_map,
        examples: example_map,
        ..Default::default()
    };
    spec.components = Some(comp);
    let mut m = openapi::SecurityRequirement::new();
    m.insert(
        Cow::from("apiKey"),
        vec![Cow::from("http"), Cow::from("baerer")],
    );
    spec.security = vec![m];

    // Adding examples
    let mut path = spec.paths.clone();
    for p in path.iter_mut() {
        println!("{}", p.0);
        if p.0 == "/api/sc/build/{txtype}" || p.0 == "/api/stx/build/{txtype}" {
            let l = p.1.post.as_mut().unwrap();
            let n = match l.request_body.as_mut().unwrap() {
                ObjectOrReference::Object(o) => o,
                ObjectOrReference::Ref { ref_path: _ } => todo!(),
            };
            let j = n.content.get("application/json").unwrap();
            let e: MediaTypeExample = MediaTypeExample::Example {
                example: serde_json::from_str(examples::STANDARD_TX_BODY).unwrap(),
            };
            let mut m = j.clone();
            m.examples = Some(e);
            n.content.swap_remove_entry("application/json");
            n.content.insert(Cow::from("application/json"), m);
        }

        if p.0 == "/api/sc/settle/{tx_type}" {
            let l = p.1.post.as_mut().unwrap();
            let n = match l.request_body.as_mut().unwrap() {
                ObjectOrReference::Object(o) => o,
                ObjectOrReference::Ref { ref_path: _ } => todo!(),
            };
            let j = n.content.get("application/json").unwrap();
            let e: MediaTypeExample = MediaTypeExample::Example {
                example: serde_json::from_str(examples::SETTLE_BODY).unwrap(),
            };
            let mut m = j.clone();
            m.examples = Some(e);
            n.content.swap_remove_entry("application/json");
            n.content.insert(Cow::from("application/json"), m);
        }
    }
    spec.paths = path;
    // Manipulating OpenApi Spec End

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
        .allow_credentials(true)
        .allow_headers(vec![
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
            "Accept-Charset",
            "Authorization",
            "Connection",
            "Content-Length",
            "Host",
            "Sec-Fetch-Dest",
            "Sec-Fetch-Mode",
            "Sec-Fetch-Site",
            "Sec-GPC",
        ]);

    let addr = "0.0.0.0:50051".parse::<SocketAddr>().unwrap();
    let rpc_server = super::grpc::AyaCardanoRPCServer::default();

    let mut set = vec![]; //tokio::task::JoinSet::new();
    set.push(tokio::task::spawn(async move {
            Server::builder()
                .add_service(super::grpc::aya_cardano::chain_follower_request_service_server::ChainFollowerRequestServiceServer::new(rpc_server))
                .serve(addr).await.unwrap();}
        ));

    // Start server
    set.push(tokio::task::spawn(
        rweb::serve(
            filter
                .or(openapi_docs(spec))
                .or(handler::ws(lim, c))
                .with(cors),
        )
        .run(socket),
    ));

    futures::future::join_all(set).await;

    Ok(())
}
