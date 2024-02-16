use thiserror::Error;
use warp::reject;

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum DataProviderError {
    #[error("internal error: {:?}", self)]
    Custom(String),
    #[error("error in data provider")]
    General,
    #[error(transparent)]
    DBsyncError(#[from] crate::dbsync::error::DataProviderDBSyncError),
    #[error(transparent)]
    BlockFrostError(#[from] crate::blockfrost::error::DataProviderBlockfrostError),
    #[error(transparent)]
    KoiosError(#[from] crate::koios::error::DataProviderKoiosError),
    #[error(transparent)]
    CarbError(#[from] crate::carb::error::DataProviderCarbError),
    #[error(transparent)]
    HexDecoderError(#[from] hex::FromHexError),
    #[error(transparent)]
    JSONError(#[from] serde_json::Error),
    #[error(transparent)]
    CSLCommonError(#[from] dcslc::error::CSLCommonError),
    #[error(transparent)]
    BlockFrostApiError(#[from] blockfrost::Error),
}

impl From<std::string::String> for DataProviderError {
    fn from(err: std::string::String) -> Self {
        DataProviderError::Custom(err)
    }
}

impl From<cardano_serialization_lib::error::DeserializeError> for DataProviderError {
    fn from(err: cardano_serialization_lib::error::DeserializeError) -> Self {
        DataProviderError::Custom(err.to_string())
    }
}

impl From<cardano_serialization_lib::error::JsError> for DataProviderError {
    fn from(err: cardano_serialization_lib::error::JsError) -> Self {
        DataProviderError::Custom(err.to_string())
    }
}

impl reject::Reject for DataProviderError {}
