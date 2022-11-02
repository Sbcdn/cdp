use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum DataProviderDBSyncError {
    #[error("DBSync Error")]
    DBSyncError(String),
    #[error("Custom Error")]
    Custom(String),
    #[error(transparent)]
    ParseIntError(#[from] core::num::ParseIntError),
    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
    #[error(transparent)]
    DieselConnectionError(#[from] diesel::ConnectionError),
    #[error(transparent)]
    HexError(#[from] hex::FromHexError),
    #[error(transparent)]
    UTF8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    CSLCommonError(#[from] drasil_csl_common::error::CSLCommonError),
}

impl From<std::string::String> for DataProviderDBSyncError {
    fn from(err: std::string::String) -> Self {
        DataProviderDBSyncError::Custom(err)
    }
}

impl From<cardano_serialization_lib::error::JsError> for DataProviderDBSyncError {
    fn from(err: cardano_serialization_lib::error::JsError) -> Self {
        DataProviderDBSyncError::Custom(err.to_string())
    }
}

impl From<cardano_serialization_lib::error::DeserializeError> for DataProviderDBSyncError {
    fn from(err: cardano_serialization_lib::error::DeserializeError) -> Self {
        DataProviderDBSyncError::Custom(err.to_string())
    }
}
