use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum DataProviderBlockfrostError {
    #[error("blockfrost provider general error")]
    GeneralError(String),
    #[error("Custom Error: {:?}", self)]
    Custom(String),
    #[error(transparent)]
    ParseIntError(#[from] core::num::ParseIntError),
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
    #[error(transparent)]
    HexError(#[from] hex::FromHexError),
    #[error(transparent)]
    UTF8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    CSLCommonError(#[from] drasil_csl_common::error::CSLCommonError),
}

impl From<std::string::String> for DataProviderBlockfrostError {
    fn from(err: std::string::String) -> Self {
        DataProviderBlockfrostError::Custom(err)
    }
}

impl From<cardano_serialization_lib::error::JsError> for DataProviderBlockfrostError {
    fn from(err: cardano_serialization_lib::error::JsError) -> Self {
        DataProviderBlockfrostError::Custom(err.to_string())
    }
}

impl From<cardano_serialization_lib::error::DeserializeError> for DataProviderBlockfrostError {
    fn from(err: cardano_serialization_lib::error::DeserializeError) -> Self {
        DataProviderBlockfrostError::Custom(err.to_string())
    }
}
