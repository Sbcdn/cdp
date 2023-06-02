use std::str::from_utf8;

use crate::models::AssetHandle;
use crate::server::error::RESTError;
use crate::server::filter::with_auth;
use crate::server::handler::make_error;
use crate::{models::TokenInfoView, provider::CardanoDataProvider};
use ::log::debug;
use cardano_serialization_lib::utils::from_bignum;
use dcslc::{make_fingerprint, TransactionUnspentOutputs};
use rweb::*;
use serde::{Deserialize, Serialize};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct EpochMessage {
    pub last_epoch: u64,
    pub last_blockhash: String,
    pub last_slot: u64,
    pub new_epoch: u64,
    pub new_slot: u64,
    pub new_blockhash: String,
    pub epoch_nonce: String,
    pub extra_entropy: Option<String>,
}

/// Get epoch change event from epoch x to epoch y
#[get("/epoch/change/from/{epoch1}/{epoch2}")]
#[openapi(
    id = "aya.epoch.change.from",
    tags("AyA Data Provider"),
    summary = "Retrieve minitng metadata for the specified token. expects fingerprint"
)]
pub async fn retry_epoch_event(
    epoch1: String,
    epoch2: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").unwrap(),
    }));

    //select * from block b where b.block_no = (select min(block_no-1) from block b where b.epoch_no = 209) or b.block_no = (select max(block_no) from block b where b.epoch_no = 209) order by block_no DESC;

    Ok(rweb::Json::from(serde_json::json!(())))
}

/// Get latest epoch change event
#[get("/epoch/change/latest")]
#[openapi(
    id = "aya.epoch.change.latest",
    tags("AyA Data Provider"),
    summary = "Retrieve minitng metadata for the specified token. expects fingerprint"
)]
pub async fn latest_epoch_change(
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").unwrap(),
    }));

    //select * from block b where b.block_no = (select min(block_no-1) from block b where b.epoch_no = 209) or b.block_no = (select max(block_no) from block b where b.epoch_no = 209) order by block_no DESC;

    Ok(rweb::Json::from(serde_json::json!(())))
}

/// Get current epoch number and nonce
#[get("/epoch/current/")]
#[openapi(
    id = "aya.epoch.current",
    tags("AyA Data Provider"),
    summary = "Retrieve minitng metadata for the specified token. expects fingerprint"
)]
pub async fn current_epoch(
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").unwrap(),
    }));

    //select * from block b where b.block_no = (select min(block_no-1) from block b where b.epoch_no = 209) or b.block_no = (select max(block_no) from block b where b.epoch_no = 209) order by block_no DESC;

    Ok(rweb::Json::from(serde_json::json!(())))
}
