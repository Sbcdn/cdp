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
use serde_json::json;



#[get("/utxos/{address}")]
#[openapi(
    id = "api.info.utxos",
    tags("UTxOs"),
    summary = "Get UTxOs for an address"
)]
pub async fn utxos_per_addr(
    address: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    // check against dataprovider
    let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").unwrap(),
    }));

    let utxos = dp
        .script_utxos(&address)
        .await
        .map_err(|_| RESTError::Custom("Could not find UTxOs".to_string()))?;

    let result = serde_json::to_value(utxos.to_hex().unwrap())
        .map_err(|_| RESTError::Custom("db error, could not get utxos".to_string()))?;
    Ok(rweb::Json::from(result))
}

#[get("/address/exist")]
#[openapi(
    id = "api.info.address",
    tags("Addresses"),
    summary = "Checks if the addresses are known by the blockchain"
)]
pub async fn address_exists(
    #[query] addresses: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").unwrap(),
    }));
    let mut addresses: Vec<String> = parse_string_vec_from_query(&addresses).unwrap();
    let addresses = addresses.iter_mut().map(|address| &address[..]).collect();

    let result = dp.addresses_exist(&addresses).await.unwrap();

    Ok(rweb::Json::from(json!(result)))
}

fn parse_string_vec_from_query(query: &str) -> Result<Vec<String>, RESTError> {
    debug!("Q:{:?}", query);
    let list: Vec<&str> = query.split('=').collect();
    debug!("1:{:?}", list);
    let list = list[1].replace("%22", &'"'.to_string());
    let list = list.replace("%5D", &']'.to_string());
    let list = list.replace("%5B", &'['.to_string());
    let list = list.replace("%2C", &','.to_string());
    debug!("2:{:?}", list);
    let list = serde_json::from_str::<Vec<String>>(&list);
    debug!("Vec: {:?}", list);
    let list = list.unwrap();
    Ok(list)
}

#[get("/asset/metadata/{fingerprint}")]
#[openapi(
    id = "api.info.asset",
    tags("Mint Metadata"),
    summary = "Retrieve minting metadata for the specified token. Expects fingerprint"
)]
pub async fn mint_metadata(
    fingerprint: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").unwrap(),
    }));
    let metadata: TokenInfoView = dp.mint_metadata(&fingerprint).await.unwrap();
    Ok(rweb::Json::from(json!(metadata)))
}

#[get("/asset/metadata/{policy}/{assetname}")]
#[openapi(
    id = "api.info.asset",
    tags("Mint Metadata"),
    summary = "Retrieve minting metadata for the specified token. Expects policy id and tokenname in url"
)]
pub async fn mint_metadata_policy_assetname(
    policy: String,
    assetname: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").unwrap(),
    }));
    let fingerprint = make_fingerprint(&policy, &assetname).unwrap();
    let metadata: TokenInfoView = dp.mint_metadata(&fingerprint).await.unwrap();
    Ok(rweb::Json::from(json!(metadata)))
}

#[get("/history/address/")]
#[openapi(
    id = "api.info.history",
    tags("Transaction History"),
    summary = "Retrieve minting metadata for the specified token. Expects fingerprint"
)]
pub async fn tx_history(
    #[query] addresses: String,
    #[query] slot: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").unwrap(),
    }));
    let mut addresses: Vec<String> = parse_string_vec_from_query(&addresses).unwrap();
    let addresses = addresses.iter_mut().map(|address| &address[..]).collect();

    let slot = slot.parse::<u64>().ok();

    let history = dp.tx_history(&addresses, slot).await.unwrap();

    Ok(rweb::Json::from(json!(history)))
}

#[get("/history/discover/{hash}")]
#[openapi(
    id = "api.info.history",
    tags("Transaction History"),
    summary = "Retrieve minting metadata for the specified token. Expects fingerprint"
)]
pub async fn tx_history_discover(
    hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").unwrap(),
    }));
    debug!("Try to discover Transaction: {:?}", hash);
    let tx = crate::dbsync::discover_transaction(dp.provider(), &hash).await;

    match tx {
        Ok(tx) => Ok(rweb::Json::from(json!(tx))),
        Err(e) => make_error(e.to_string(), None, None),
    }
}

#[get("/addresses/assets/")]
#[openapi(
    id = "api.info.address",
    tags("AssetHandles"),
    summary = "Get AssetHandles for addresses"
)]
pub async fn handle_get_asset_for_addresses(
    #[query] addresses: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    debug!("{addresses:?}");
    let addresses = match parse_string_vec_from_query(&addresses) {
        Ok(u) => u,
        Err(e) => {
            return make_error(
                e.to_string(),
                Some(1001),
                Some("Could not parse addresses from query parameter"),
            );
        }
    };

    Ok(rweb::Json::from(json!(
        get_asset_for_addresses(&addresses).await?
    )))
}

pub(crate) async fn get_asset_for_addresses(
    addresses: &Vec<String>,
) -> Result<Vec<AssetHandle>, Rejection> {
    debug!("{addresses:?}");
    let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").unwrap(),
    }));

    let mut utxos = TransactionUnspentOutputs::new();

    for a in addresses {
        let us = dp.script_utxos(a).await.unwrap();
        utxos.merge(us);
    }

    let mut handles = Vec::<AssetHandle>::new();
    for u in utxos {
        let v = u.output().amount();
        let ada = v.coin();
        handles.push(AssetHandle {
            fingerprint: None,
            policy: None,
            tokenname: None,
            amount: from_bignum(&ada),
            metadata: None,
        });
        if let Some(multis) = v.multiasset() {
            let policies = multis.keys();
            for p in 0..policies.len() {
                let policy = policies.get(p);
                if let Some(assets) = multis.get(&policy) {
                    let k = assets.keys();
                    for a in 0..k.len() {
                        let asset = k.get(a);
                        let amt = assets.get(&asset).unwrap();
                        let fingerprint =
                            make_fingerprint(&policy.to_hex(), &hex::encode(asset.name())).unwrap();
                        let metadata = dp.mint_metadata(&fingerprint).await.unwrap();
                        handles.push(AssetHandle {
                            fingerprint: Some(fingerprint),
                            policy: Some(policy.to_hex()),
                            tokenname: Some(from_utf8(&asset.name()).unwrap().to_owned()),
                            amount: from_bignum(&amt),
                            metadata: metadata.json,
                        })
                    }
                }
            }
        }
    }
    debug!("Handles: {:?}", handles);
    let mut handles_summed = Vec::<AssetHandle>::new();

    for h in &handles {
        if handles_summed
            .iter()
            .filter(|n| h.same_asset(n))
            .collect::<Vec<&AssetHandle>>()
            .is_empty()
        {
            let sum = handles.iter().fold(AssetHandle::new_empty(), |mut acc, f| {
                if h.same_asset(f) {
                    acc.amount = acc.amount.checked_add(f.amount).unwrap();

                    if acc.metadata.is_none() && f.metadata.is_some() {
                        acc.metadata = h.metadata.clone()
                    }
                    if acc.fingerprint.is_none() && f.fingerprint.is_some() {
                        acc.fingerprint = h.fingerprint.clone()
                    }
                    if acc.policy.is_none() && f.policy.is_some() {
                        acc.policy = h.policy.clone()
                    }
                    if acc.tokenname.is_none() && f.tokenname.is_some() {
                        acc.tokenname = h.tokenname.clone()
                    }
                }
                acc
            });
            handles_summed.push(sum)
        }
    }
    Ok(handles_summed)
}

#[get("/address/stake/assets/")]
#[openapi(
    id = "api.info.address",
    tags("AssetHandles"),
    summary = "Get AssetHandles for staking addresses"
)]
pub async fn handle_asset_for_stake_address(
    #[query] stake_address: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    debug!("{stake_address:?}");
    let split = stake_address.split('=').collect::<Vec<&str>>();
    let bstake_addr = match dcslc::addr_from_str(split[1]) {
        Ok(s) => s,
        Err(e) => {
            return make_error(
                e.to_string(),
                Some(1002),
                Some("The provided stake address is invalid"),
            );
        }
    };
    let reward_address = match dcslc::get_stakeaddr_from_addr(&bstake_addr) {
        Ok(r) => r,
        Err(e) => {
            return make_error(
                e.to_string(),
                Some(1003),
                Some("The provided address is not a stake address"),
            );
        }
    };

    let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").unwrap(),
    }));

    let utxos = match dp
        .wallet_utxos(&reward_address.to_bech32(None).unwrap())
        .await
    {
        Ok(u) => u,
        Err(e) => {
            return make_error(
                e.to_string(),
                Some(2001),
                Some(&format!(
                    "Could not retrieve utxos for stake address: {:?}",
                    reward_address.to_bech32(None).unwrap()
                )),
            );
        }
    };

    let mut handles = Vec::<AssetHandle>::new();
    for u in utxos {
        let v = u.output().amount();
        let ada = v.coin();
        handles.push(AssetHandle {
            fingerprint: None,
            policy: None,
            tokenname: None,
            amount: from_bignum(&ada),
            metadata: None,
        });
        if let Some(multis) = v.multiasset() {
            let policies = multis.keys();
            for p in 0..policies.len() {
                let policy = policies.get(p);
                if let Some(assets) = multis.get(&policy) {
                    let k = assets.keys();
                    for a in 0..k.len() {
                        let asset = k.get(a);
                        let amt = assets.get(&asset).unwrap();
                        let fingerprint =
                            make_fingerprint(&policy.to_hex(), &hex::encode(asset.name())).unwrap();
                        //let metadata = dp.mint_metadata(&fingerprint).await.unwrap();
                        handles.push(AssetHandle {
                            fingerprint: Some(fingerprint),
                            policy: Some(policy.to_hex()),
                            tokenname: Some(from_utf8(&asset.name()).unwrap().to_owned()),
                            amount: from_bignum(&amt),
                            metadata: None, //metadata.json,
                        })
                    }
                }
            }
        }
    }
    debug!("Handles: {:?}", handles);
    let mut handles_summed = Vec::<AssetHandle>::new();

    for h in &handles {
        if handles_summed
            .iter()
            .filter(|n| h.same_asset(n))
            .collect::<Vec<&AssetHandle>>()
            .is_empty()
        {
            let sum = handles.iter().fold(AssetHandle::new_empty(), |mut acc, f| {
                if h.same_asset(f) {
                    acc.amount = acc.amount.checked_add(f.amount).unwrap();

                    if acc.metadata.is_none() && f.metadata.is_some() {
                        acc.metadata = h.metadata.clone()
                    }
                    if acc.fingerprint.is_none() && f.fingerprint.is_some() {
                        acc.fingerprint = h.fingerprint.clone()
                    }
                    if acc.policy.is_none() && f.policy.is_some() {
                        acc.policy = h.policy.clone()
                    }
                    if acc.tokenname.is_none() && f.tokenname.is_some() {
                        acc.tokenname = h.tokenname.clone()
                    }
                }
                acc
            });
            handles_summed.push(sum)
        }
    }
    debug!("Handles summed: {:?}", handles_summed);
    Ok(rweb::Json::from(json!(handles_summed)))
}

#[get("/pools/{page}")]
#[openapi(
    id = "api.info.pools",
    tags("Stake Pool"),
    summary = "Get Stake Pool List"
)]
pub async fn retrieve_active_pools(
    page: usize,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").unwrap(),
    }));
    let pools = crate::dbsync::get_pools(dp.provider()).await.unwrap();
    let pools_paged: Vec<Vec<crate::models::PoolView>> =
        pools.chunks(100).map(|s| s.into()).collect();
    if pools_paged.len() < page {
        return make_error(
            format!("Page {} is the last page", pools_paged.len()),
            None,
            None,
        );
    }
    Ok(rweb::Json::from(json!(pools_paged[page])))
}

#[get("/tokens/supply/{fingerprint}")]
#[openapi(
    id = "api.info.tokens.supply",
    tags("Tokens"),
    summary = "Token Supply"
)]
pub async fn token_supply(
    fingerprint: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").unwrap(),
    }));
    let supply = crate::dbsync::token_supply(dp.provider(), &fingerprint).await;
    if let Err(e) = &supply {
        return make_error(
            format!("Could not get supply for {:?}", e.to_string()),
            None,
            None,
        );
    }
    Ok(rweb::Json::from(json!(supply.unwrap())))
}

#[get("/tokens/isNft/")]
#[openapi(
    id = "api.info.tokens.isNft",
    tags("Tokens"),
    summary = "Tests if a given list of tokens contains NFTs (Tokens of supply 1)"
)]
pub async fn is_nft(
    #[query] fingerprints: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let f = match parse_string_vec_from_query(&fingerprints) {
        Ok(u) => u,
        Err(e) => {
            return make_error(
                e.to_string(),
                Some(1004),
                Some("Could not parse list of fingerprints from query parameter"),
            );
        }
    };
    debug!("Creatign dataprovider instance");
    let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").unwrap(),
    }));
    debug!("Try to execute query");
    let supply = crate::dbsync::is_nft(
        dp.provider(),
        &f.iter().map(|n| &**n).collect::<Vec<&str>>()[..],
    )
    .await;
    debug!("Received query results: {supply:?}");
    if let Err(e) = &supply {
        return make_error(
            format!("Could not get supply for {f:?}, error: {e}"),
            None,
            None,
        );
    }
    Ok(rweb::Json::from(json!(supply.unwrap())))
}

#[get("/epoch/stake/amount/{stake_addr}/{epoch}")]
#[openapi(
    id = "api.info.stake",
    tags("Staked Amount"),
    summary = "Retrieve the amount of Ada staked by given address"
)]
pub async fn retrieve_staked_amount(
    epoch: i32,
    stake_addr: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").unwrap(),
    }));

    dbg!(epoch.clone());
    dbg!(stake_addr.clone());
    
    let staked_amount = dp.retrieve_staked_amount(epoch, &stake_addr)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find staked amount".to_string()))?;
    dbg!(staked_amount.clone());

    Ok(rweb::Json::from(json!(staked_amount)))
}

#[get("/reward/amount/{stake_addr}")]
#[openapi(
    id = "api.info.reward",
    tags("Generated Rewards"),
    summary = "Retrieve the amount of Ada that the given address is rewarded with for their stake"
)]
pub async fn retrieve_generated_rewards (
    stake_addr: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").unwrap(),
    }));

    let generated_rewards = dp
        .retrieve_generated_rewards(&stake_addr)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find generated rewards".to_string()))?;

    Ok(rweb::Json::from(json!(generated_rewards)))
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_parse_from_query() {
        let r = crate::server::handler::handler_rest::info::parse_string_vec_from_query("addresses=[\"addr_test1qqt86eq9972q3qttj6ztje97llasktzfzvhmdccqjlqjaq2cer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qy6q5t2\",\"addr_test1qpg8ehvgj9zxrx59et72yjn2p02xwsm3l89jwj8ujcj63ujcer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qw23emu\",\"addr_test1qqdp3cry5vc2gfjljctdu638tvkcqfx40fjunht9hrmru5zcer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qnaxxgs\",\"addr_test1qr2mw080ujz0unmpn9lx5ftfuewc6htyr6v3a0svul2zgezcer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qgryf7t\",\"addr_test1qr7tqh7tsg4lut3jv6tsfwlv464m6knjjw90ugyz8uzgr6zcer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qt0jxzj\",\"addr_test1qrscurjp292sxv24sepj7ghq4ydkkekzaz53zwfswcna6ljcer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6q8pu3l5\",\"addr_test1qqssrphse6qmp9h0ksu5vfmsx99tfl2lc6rhvy2spd5wr86cer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qw59j4j\",\"addr_test1qqgagc0fy6nm0qe4h8zqxsg952tqjeg7l7j0agd0cx4u25zcer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qxvept2\"]").unwrap();
        println!("{r:?}");
    }
}
