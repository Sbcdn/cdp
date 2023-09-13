pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, cdp::provider::error::DataProviderError>;

#[tokio::main]
pub async fn main() {
    pretty_env_logger::init();
    let config = cdp::config::ConfigRoot::new(&None).unwrap();
    config.set_as_env();
    cdp::server::serve(Some(config.connectivity.dbsync_url))
        .await
        .unwrap();
}
