use std::str::from_utf8;

use crate::DBSyncProvider;
use crate::dbsync::get_stake_address_utxos_dep;
use crate::dbsync;
use crate::models::{AssetHandle, PoolView};
use crate::server::error::RESTError;
use crate::server::filter::with_auth;
use crate::server::handler::make_error;
use crate::{models::TokenInfoView, provider::CardanoDataProvider};
use crate::DataProvider;
use ::log::debug;
use cardano_serialization_lib::utils::from_bignum;
use dcslc::{make_fingerprint, TransactionUnspentOutputs};
use rweb::*;
use serde_json::json;

fn data_provider() -> Result<DataProvider<DBSyncProvider>, Rejection> {
    let dp: DataProvider<DBSyncProvider> = DataProvider::new(DBSyncProvider::new(crate::Config {
        db_path: std::env::var("DBSYNC_URL").map_err(|_| warp::reject::not_found())?,
    }));
    Ok(dp)
}

#[get("/utxos/{address}")]
#[openapi(
    id = "api.info.utxos",
    tags("UTxOs"),
    summary = "Get UTxOs for an address"
)]
pub async fn get_address_utxos(
    address: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let utxos = data_provider()?
        .get_address_utxos(&address)
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
    let mut addresses: Vec<String> = parse_string_vec_from_query(&addresses)?;
    let addresses = addresses.iter_mut().map(|address| &address[..]).collect();

    let result = data_provider()?
        .addresses_exist(&addresses).await?;

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
    tags("Mint Metadata Fingerprint"),
    summary = "Retrieve minting metadata for the specified token. Expects fingerprint"
)]
pub async fn mint_metadata(
    fingerprint: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let metadata: TokenInfoView = data_provider()?
        .mint_metadata(&fingerprint).await
        .map_err(|e| RESTError::Custom(e.to_string()))?;

    Ok(rweb::Json::from(json!(metadata)))
}

#[get("/asset/metadata/{policy}/{assetname}")]
#[openapi(
    id = "api.info.asset",
    tags("Mint Metadata Policy Assetname"),
    summary = "Retrieve minting metadata for the specified token. Expects policy id and tokenname in url"
)]
pub async fn mint_metadata_policy_assetname(
    policy: String,
    assetname: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let fingerprint = make_fingerprint(&policy, &assetname).unwrap();
    let metadata: TokenInfoView = data_provider()?
        .mint_metadata(&fingerprint).await?;
    Ok(rweb::Json::from(json!(metadata)))
}

#[get("/history/address/")]
#[openapi(
    id = "api.info.history",
    tags("Transaction History"),
    summary = "Retrieve transaction history for the given addresses on the given slot"
)]
pub async fn tx_history(
    #[query] addresses: String,
    #[query] slot: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let mut addresses: Vec<String> = parse_string_vec_from_query(&addresses)?;
    let addresses = addresses.iter_mut().map(|address| &address[..]).collect();

    let slot = slot.parse::<u64>().ok();

    let history = data_provider()?
        .tx_history(&addresses, slot).await?;

    Ok(rweb::Json::from(json!(history)))
}

#[get("/history/discover/{hash}")]
#[openapi(
    id = "api.info.history",
    tags("Transaction History"),
    summary = "Retrieve minting metadata for the specified token. Expects transaction hash"
)]
pub async fn tx_history_discover(
    hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    debug!("Try to discover Transaction: {:?}", hash);
    let tx = dbsync::discover_transaction(
        data_provider()?.provider(), &hash
    ).await;

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

    let mut utxos = TransactionUnspentOutputs::new();

    for a in addresses {
        let us = data_provider()?.get_address_utxos(a).await?;
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
                        // Deactivated Metadata Requests for performance
                        //let metadata = dp.mint_metadata(&fingerprint).await.unwrap();
                        handles.push(AssetHandle {
                            fingerprint: Some(fingerprint),
                            policy: Some(policy.to_hex()),
                            tokenname: Some(match from_utf8(&asset.name()) {
                                Ok(s) => s.to_owned(),
                                Err(_) => hex::encode(&asset.name()),
                            }),
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
        if !handles_summed.contains(h) {
            let sum = handles.iter().fold(AssetHandle::new_empty(), |mut acc, f| {
                if h == f {
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

    // dp
    //.wallet_utxos(&reward_address.to_bech32(None).unwrap())
    let utxos = match get_stake_address_utxos_dep(
        data_provider()?.provider(),
        &reward_address.to_bech32(None).unwrap(),
    ) {
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
                            tokenname: Some(match from_utf8(&asset.name()) {
                                Ok(s) => s.to_owned(),
                                Err(_) => hex::encode(&asset.name()),
                            }),
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
        if !handles_summed.contains(h) {
            let sum = handles.iter().fold(AssetHandle::new_empty(), |mut acc, f| {
                if h == f {
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
    id = "api.info.pools_one_page",
    tags("Stake Pool"),
    summary = "Get Stake Pool List (specified page)"
)]
pub async fn retrieve_active_pools(
    page: usize,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let pools = dbsync::get_pools(
        data_provider()?.provider()
    ).await?;
    let pools_paged: Vec<Vec<PoolView>> = pools.chunks(100).map(|s| s.into()).collect();
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
    let supply = dbsync::token_supply(data_provider()?.provider(), &fingerprint).await;
    if let Err(e) = &supply {
        return make_error(
            format!("Could not get supply for {:?}", e.to_string()),
            None,
            None,
        );
    }
    Ok(rweb::Json::from(json!(supply?)))
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
    debug!("Try to execute query");
    let supply = dbsync::is_nft(
        data_provider()?.provider(),
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
    Ok(rweb::Json::from(json!(supply?)))
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
    let staked_amount = data_provider()?
        .retrieve_staked_amount(epoch, &stake_addr)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find staked amount".to_string()))?;

    Ok(rweb::Json::from(json!(staked_amount)))
}

#[get("/reward/amount/{stake_addr}")]
#[openapi(
    id = "api.info.reward",
    tags("Generated Rewards"),
    summary = "Retrieve the amount of Ada that the given address is rewarded with for their stake"
)]
pub async fn retrieve_generated_rewards(
    stake_addr: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let generated_rewards = data_provider()?
        .retrieve_generated_rewards(&stake_addr)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find generated rewards".to_string()))?;

    Ok(rweb::Json::from(json!(generated_rewards)))
}

#[get("/pool/vrf_key_hash/{pool_hash}")]
#[openapi(
    id = "api.info.pool.vrf_key_hash",
    tags("Pool"),
    summary = "VRF key hash generated by the pool's VRF private key"
)]
pub async fn pool_vrf_key_hash(
    pool_hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let pool_vrf_key_hash = data_provider()?
        .pool_vrf_key_hash(&pool_hash)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find pools VRF key hash".to_string()))?;

    Ok(rweb::Json::from(json!(pool_vrf_key_hash)))
}

#[get("/pool/blocks_minted/{pool_hash}")]
#[openapi(
    id = "api.info.pool.blocks_minted",
    tags("Pool"),
    summary = "Total number of blocks minted by the given pool"
)]
pub async fn pool_blocks_minted(
    pool_hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let pool_blocks_minted = data_provider()?
        .pool_blocks_minted(&pool_hash)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find the total number of blocks minted by the given pool".to_string()))?;

    Ok(rweb::Json::from(json!(pool_blocks_minted)))
}

#[get("/pool/blocks_current_epoch/{pool_hash}")]
#[openapi(
    id = "api.info.pool.blocks_current_epoch",
    tags("Pool"),
    summary = "Quantity of blocks minted by the given pool in current epoch"
)]
pub async fn pool_blocks_current_epoch(
    pool_hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let pool_blocks_current_epoch = data_provider()?
        .pool_blocks_current_epoch(&pool_hash)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find the quantity of blocks minted by the given pool in current epoch".to_string()))?;

    Ok(rweb::Json::from(json!(pool_blocks_current_epoch)))
}

#[get("/pool/pool_reward_recipients/{pool_hash}")]
#[openapi(
    id = "api.info.pool.pool_reward_recipients",
    tags("Pool"),
    summary = "The quantity of delegators that received rewards last time (epoch) the given pool was a slot leader."
)]
pub async fn pool_reward_recipients(
    pool_hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let pool_reward_recipients = data_provider()?
        .pool_reward_recipients(&pool_hash)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find the quantity of delegators that received rewards".to_string()))?;

    Ok(rweb::Json::from(json!(pool_reward_recipients)))
}

#[get("/pool/last_reward_earned_epoch/{pool_hash}")]
#[openapi(
    id = "api.info.pool.last_reward_earned_epoch",
    tags("Pool"),
    summary = "The last epoch when the given pool gave rewards to delegators"
)]
pub async fn pool_last_reward_earned_epoch(
    pool_hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let pool_last_reward_earned_epoch = data_provider()?
        .pool_last_reward_earned_epoch(&pool_hash)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find the last epoch when the given pool distributed rewards".to_string()))?;

    Ok(rweb::Json::from(json!(pool_last_reward_earned_epoch)))
}


#[get("/pool/declared_pledge/{pool_hash}")]
#[openapi(
    id = "api.info.pool.declared_pledge",
    tags("Pool"),
    summary = "The amount of Ada that the given stake pool has pledged to stake into their own pool"
)]
pub async fn pool_declared_pledge(
    pool_hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let pool_declared_pledge = data_provider()?
        .pool_declared_pledge(&pool_hash)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find the amount of Ada pledged by the given pool".to_string()))?;

    Ok(rweb::Json::from(json!(pool_declared_pledge)))
}

#[get("/pool/margin_cost/{pool_hash}")]
#[openapi(
    id = "api.info.pool.margin_cost",
    tags("Pool"),
    summary = "The percentage of delegator's stake rewards that pool owner receives as compensation for enabling delegators to delegate"
)]
pub async fn pool_margin_cost(
    pool_hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let pool_margin_cost = data_provider()?
        .pool_margin_cost(&pool_hash)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find the margin cost of the given pool".to_string()))?;

    Ok(rweb::Json::from(json!(pool_margin_cost)))
}

#[get("/pool/fixed_cost/{pool_hash}")]
#[openapi(
    id = "api.info.pool.fixed_cost",
    tags("Pool"),
    summary = "The fixed amount of Ada that the given stake pool receives every epoch from each delegator"
)]
pub async fn pool_fixed_cost(
    pool_hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let pool_fixed_cost = data_provider()?
        .pool_fixed_cost(&pool_hash)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find the fixed cost of the given pool".to_string()))?;

    Ok(rweb::Json::from(json!(pool_fixed_cost)))
}

#[get("/pool/owner/{pool_hash}")]
#[openapi(
    id = "api.info.pool.owner",
    tags("Pool"),
    summary = "The stake address that represents the owner of the given stake pool"
)]
pub async fn pool_owner(
    pool_hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let pool_owner = data_provider()?
        .pool_owner(&pool_hash)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find the owner of the given pool".to_string()))?;

    Ok(rweb::Json::from(json!(pool_owner)))
}

#[get("/pool/registration/{pool_hash}")]
#[openapi(
    id = "api.info.pool.registration",
    tags("Pool"),
    summary = "The epoch in which the given stake pool made its latest registration."
)]
pub async fn pool_registration(
    pool_hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let pool_registration = data_provider()?
        .pool_registration(&pool_hash)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find the epoch in which the given pool made its latest registration".to_string()))?;

    Ok(rweb::Json::from(json!(pool_registration)))
}

#[get("/pool/retirement/{pool_hash}")]
#[openapi(
    id = "api.info.pool.retirement",
    tags("Pool"),
    summary = "The epoch in which the given pool retired."
)]
pub async fn pool_retirement(
    pool_hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let pool_retirement = data_provider()?
        .pool_retirement(&pool_hash)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find the epoch in which the given pool retired".to_string()))?;

    Ok(rweb::Json::from(json!(pool_retirement)))
}

#[get("/pool/url/{pool_hash}")]
#[openapi(
    id = "api.info.pool.url",
    tags("Pool"),
    summary = "The url in which the given pool stores its metadata."
)]
pub async fn pool_url(
    pool_hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let pool_url = data_provider()?
        .pool_url(&pool_hash)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find the url in which the given pool stores its metadata".to_string()))?;

    Ok(rweb::Json::from(json!(pool_url)))
}

#[get("/pool/ticker/{pool_hash}")]
#[openapi(
    id = "api.info.pool.ticker",
    tags("Pool"),
    summary = "The ticker (abbreviated name) of the given stakepool."
)]
pub async fn pool_ticker(
    pool_hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let pool_ticker = data_provider()?
        .pool_ticker(&pool_hash)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find the ticker of the given stake pool".to_string()))?;

    Ok(rweb::Json::from(json!(pool_ticker)))
}

#[get("/pool/metadata_json/{pool_hash}")]
#[openapi(
    id = "api.info.pool.metadata_json",
    tags("Pool"),
    summary = "Metadata, of the given stake pool, presented in JSON format"
)]
pub async fn pool_metadata_json(
    pool_hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let pool_metadata_json = data_provider()?
        .pool_metadata_json(&pool_hash)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find the metadata JSON of the given stake pool".to_string()))?;

    Ok(rweb::Json::from(pool_metadata_json))
}

#[get("/pool/name/{pool_hash}")]
#[openapi(
    id = "api.info.pool.name",
    tags("Pool"),
    summary = "Name of the given stake pool"
)]
pub async fn pool_name(
    pool_hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let pool_name = data_provider()?
        .pool_name(&pool_hash)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find the name of the given stake pool".to_string()))?;

    Ok(rweb::Json::from(json!(pool_name)))
}

#[get("/pool/homepage/{pool_hash}")]
#[openapi(
    id = "api.info.pool.homepage",
    tags("Pool"),
    summary = "Homepage of the given stake pool"
)]
pub async fn pool_homepage(
    pool_hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let pool_homepage = data_provider()?
        .pool_homepage(&pool_hash)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find the homepage of the given stake pool".to_string()))?;

    Ok(rweb::Json::from(json!(pool_homepage)))
}

#[get("/pool/description/{pool_hash}")]
#[openapi(
    id = "api.info.pool.description",
    tags("Pool"),
    summary = "Description of the given stake pool"
)]
pub async fn pool_description(
    pool_hash: String,
    #[filter = "with_auth"] _user_id: String,
) -> Result<Json<serde_json::Value>, Rejection> {
    let pool_description = data_provider()?
        .pool_description(&pool_hash)
        .await
        .map_err(|_| RESTError::Custom("Couldn't find the description of the given stake pool".to_string()))?;

    Ok(rweb::Json::from(json!(pool_description)))
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_parse_from_query() {
        let r = crate::server::handler::handler_rest::info::parse_string_vec_from_query("addresses=[\"addr_test1qqt86eq9972q3qttj6ztje97llasktzfzvhmdccqjlqjaq2cer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qy6q5t2\",\"addr_test1qpg8ehvgj9zxrx59et72yjn2p02xwsm3l89jwj8ujcj63ujcer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qw23emu\",\"addr_test1qqdp3cry5vc2gfjljctdu638tvkcqfx40fjunht9hrmru5zcer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qnaxxgs\",\"addr_test1qr2mw080ujz0unmpn9lx5ftfuewc6htyr6v3a0svul2zgezcer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qgryf7t\",\"addr_test1qr7tqh7tsg4lut3jv6tsfwlv464m6knjjw90ugyz8uzgr6zcer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qt0jxzj\",\"addr_test1qrscurjp292sxv24sepj7ghq4ydkkekzaz53zwfswcna6ljcer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6q8pu3l5\",\"addr_test1qqssrphse6qmp9h0ksu5vfmsx99tfl2lc6rhvy2spd5wr86cer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qw59j4j\",\"addr_test1qqgagc0fy6nm0qe4h8zqxsg952tqjeg7l7j0agd0cx4u25zcer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qxvept2\"]").unwrap();
        println!("{r:?}");
    }
}
