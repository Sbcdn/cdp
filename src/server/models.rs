use cardano_serialization_lib::metadata::AuxiliaryData;
use cardano_serialization_lib::{Transaction, TransactionBody, TransactionWitnessSet};
use rweb::warp::ws::Message;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, Mutex};

use crate::models::{AssetHandle, TokenInfoView};

#[derive(Serialize, Debug)]
pub(crate) struct ErrorResponse {
    pub message: String,
    pub status: String,
}

impl ErrorResponse {
    pub fn new(message: String, status: String) -> ErrorResponse {
        ErrorResponse { message, status }
    }
}

#[derive(Clone)]
pub struct TxCacheItem {
    pub id: uuid::Uuid,
    pub tx_body: TransactionBody,
    pub tx_wittness: TransactionWitnessSet,
    pub tx_unsigned: Transaction,
    pub tx_aux: AuxiliaryData,
    pub ttl: Option<u64>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Role {
    System,
    WMUser,
    External,
    Admin,
}

impl From<&str> for Role {
    fn from(role: &str) -> Role {
        match role {
            "0" => Role::Admin,
            "1" => Role::System,
            "2" => Role::WMUser,
            _ => Role::External,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Admin => write!(f, "0"),
            Role::System => write!(f, "1"),
            Role::WMUser => write!(f, "2"),
            Role::External => write!(f, "3"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Claims {
    pub sub: String,
    pub rpm: String,
    pub exp: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClaimsNR {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub user_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, rweb::warp::Error>>>,
}

pub type Clients = Arc<Mutex<HashMap<String, Client>>>;
pub type LocalTxCache = Arc<Mutex<HashMap<String, TxCacheItem>>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct WSResponse {
    pub message_id: String,
    pub response: WSResponseTypes,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WSResponseTypes {
    VTokenInfoView(Vec<TokenInfoView>),
    VAssetHandle(Vec<AssetHandle>),
    VBool(Vec<bool>),
    String(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WSMessage {
    pub message_id: String,
    pub request: WSRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WSRequest {
    Alive,
    IsNFT(Vec<String>),
    AddressAssetHandles(Vec<String>),
    MintMetadata(Vec<String>),
}
