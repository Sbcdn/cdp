use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum DataProviderKoiosError {
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

impl From<std::string::String> for DataProviderKoiosError {
    fn from(err: std::string::String) -> Self {
        DataProviderKoiosError::Custom(err)
    }
}

impl From<cardano_serialization_lib::error::JsError> for DataProviderKoiosError {
    fn from(err: cardano_serialization_lib::error::JsError) -> Self {
        DataProviderKoiosError::Custom(err.to_string())
    }
}

impl From<cardano_serialization_lib::error::DeserializeError> for DataProviderKoiosError {
    fn from(err: cardano_serialization_lib::error::DeserializeError) -> Self {
        DataProviderKoiosError::Custom(err.to_string())
    }
}
