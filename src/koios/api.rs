use super::error::DataProviderKoiosError;
use super::KoiosProvider;
use crate::models::{
    CDPDatum, CardanoNativeAssetView, DelegationView, HoldingWalletView, StakeDelegationView,
    StakeDeregistrationView, StakeRegistrationView, TokenInfoView, RewardView,
};
use bigdecimal::BigDecimal;
use serde_json::{Value, json};

/// get all tokens of an utxo
pub fn get_utxo_tokens(
    bfp: &KoiosProvider,
    tx_id: i64,
    tx_index: i16,
) -> Result<Vec<CardanoNativeAssetView>, DataProviderKoiosError> {
    todo!()
}

pub fn select_addr_of_first_transaction(
    bfp: &KoiosProvider,
    stake_address_in: &str,
) -> Result<String, DataProviderKoiosError> {
    Ok("".to_owned())
}

/// get all utxos of an address
pub fn get_utxo_by_dataumhash(
    bfp: &KoiosProvider,
    addr: &str,
    datumhash: &Vec<u8>,
) -> Result<dcslc::TransactionUnspentOutput, DataProviderKoiosError> {
    Err(DataProviderKoiosError::Custom(
        "not implemented".to_string(),
    ))
}

/// get all utxos of an address
pub fn utxo_by_txid(
    bfp: &KoiosProvider,
    txhash: &Vec<u8>,
    index: i16,
) -> Result<dcslc::TransactionUnspentOutput, DataProviderKoiosError> {
    Err(DataProviderKoiosError::Custom(
        "not implemented".to_string(),
    ))
}

/// get all utxos of an address
pub fn get_address_utxos(
    bfp: &KoiosProvider,
    addr: &str,
) -> Result<dcslc::TransactionUnspentOutputs, DataProviderKoiosError> {
    Ok(dcslc::TransactionUnspentOutputs::new())
}

/// Get all utxos of a stake address
pub fn get_stake_address_utxos(
    bfp: &KoiosProvider,
    stake_addr: &str,
) -> Result<dcslc::TransactionUnspentOutputs, DataProviderKoiosError> {
    Ok(dcslc::TransactionUnspentOutputs::new())
}

/// Get all utxos of a stake address
pub fn asset_utxos_on_addr(
    bfp: &KoiosProvider,
    addr: &str,
) -> Result<dcslc::TransactionUnspentOutputs, DataProviderKoiosError> {
    Ok(dcslc::TransactionUnspentOutputs::new())
}

pub fn find_datums_for_tx(
    bfp: &KoiosProvider,
    txid: &Vec<u8>,
) -> Result<Vec<CDPDatum>, crate::provider::error::DataProviderError> {
    todo!();
}

pub fn slot(bfp: &KoiosProvider) -> Result<i64, DataProviderKoiosError> {
    Ok(0)
}

pub fn stakers_on_pool(
    bfp: &KoiosProvider,
    pool: &str,
    epoch: i32,
) -> Result<Vec<StakeDelegationView>, DataProviderKoiosError> {
    todo!();
}

pub fn deligations_per_pool_for_epochs(
    bfp: &KoiosProvider,
    pool: &str,
    start_epoch: i64,
    end_epoch: i64,
) -> Result<Vec<DelegationView>, DataProviderKoiosError> {
    todo!();
}

pub fn pool_total_stake(
    bfp: &KoiosProvider,
    pool: &str,
    epoch: i32,
) -> Result<u64, DataProviderKoiosError> {
    Ok(0)
}

pub fn current_epoch(bfp: &KoiosProvider) -> Result<i32, DataProviderKoiosError> {
    Ok(0)
}

pub fn fingerprint(
    bfp: &KoiosProvider,
    policy: &str,
    tokenname: &str,
) -> Result<String, DataProviderKoiosError> {
    Ok("".to_owned())
}

pub fn token_info(
    bfp: &KoiosProvider,
    fingerprint_in: &str,
) -> Result<TokenInfoView, DataProviderKoiosError> {
    todo!();
}

pub fn stake_registration(
    bfp: &KoiosProvider,
    stake_addr_in: &str,
) -> Result<Vec<StakeRegistrationView>, DataProviderKoiosError> {
    todo!();
}

#[allow(clippy::type_complexity)]
pub fn stake_deregistration(
    bfp: &KoiosProvider,
    stake_addr_in: &str,
) -> Result<Vec<StakeDeregistrationView>, DataProviderKoiosError> {
    todo!();
}

pub fn check_stakeaddr_registered(
    bfp: &KoiosProvider,
    stake_addr_in: &str,
) -> Result<bool, DataProviderKoiosError> {
    Ok(false)
}

pub fn lookup_token_holders(
    bfp: &KoiosProvider,
    fingerprint_in: &str,
    min_amount: Option<&i64>,
) -> Result<Vec<HoldingWalletView>, DataProviderKoiosError> {
    todo!();
}

pub fn lookup_nft_token_holders(
    bfp: &KoiosProvider,
    policy: &str,
) -> Result<Vec<HoldingWalletView>, DataProviderKoiosError> {
    todo!();
}

pub fn mint_metadata(
    bfp: &KoiosProvider,
    fingerprint_in: &str,
) -> Result<TokenInfoView, DataProviderKoiosError> {
    Ok(TokenInfoView {
        fingerprint: "".to_owned(),
        policy: hex::encode(""),
        tokenname: "".to_owned(),
        meta_key: None,
        json: None,
        txhash: None,
        quantity: None,
        mint_slot: None,
    })
}

pub fn pool_valid(bfp: &KoiosProvider, pool_id: &str) -> Result<bool, DataProviderKoiosError> {
    Ok(true)
}

pub fn txhash_spent(bfp: &KoiosProvider, txhash: &str) -> Result<bool, DataProviderKoiosError> {
    let txh_b = hex::decode(txhash)?;

    Ok(false)
}

pub fn retrieve_staked_amount (
    bfp: &KoiosProvider,
    epoch: i32,
    stake_addr: &str,
) -> Result<BigDecimal, DataProviderKoiosError> {
    Ok(BigDecimal::from(0))
}

pub fn retrieve_generated_rewards (
    bfp: &KoiosProvider,
    stake_addr: &str,
) -> Result<Vec<RewardView>, DataProviderKoiosError> {
    Ok(vec![])
}

pub fn pool_vrf_key_hash(
    bfp: &KoiosProvider,
    pool_hash: &str,
) -> Result<Vec<u8>, DataProviderKoiosError> {
    todo!()
}

pub fn pool_blocks_minted(
    bfp: &KoiosProvider,
    pool_hash: &str,
) -> Result<i64, DataProviderKoiosError> {
    todo!()
} 

pub fn pool_blocks_current_epoch(
    bfp: &KoiosProvider,
    pool_hash: &str,
) -> Result<i64, DataProviderKoiosError> {
    todo!()
}

pub fn pool_reward_recipients(
    bfp: &KoiosProvider,
    pool_hash: &str,
) -> Result<i64, DataProviderKoiosError> {
    todo!()
}

pub fn pool_last_reward_earned_epoch(
    bfp: &KoiosProvider,
    pool_hash: &str,
) -> Result<i64, DataProviderKoiosError> {
    todo!()
}

pub fn pool_declared_pledge(
    bfp: &KoiosProvider,
    pool_hash: &str,
) -> Result<BigDecimal, DataProviderKoiosError> {
    todo!()
}

pub fn pool_margin_cost(
    bfp: &KoiosProvider,
    pool_hash: &str,
) -> Result<f64, DataProviderKoiosError> {
    todo!()
}

pub fn pool_fixed_cost(
    bfp: &KoiosProvider,
    pool_hash: &str,
) -> Result<BigDecimal, DataProviderKoiosError> {
    todo!()
}

pub fn pool_reward_address(
    bfp: &KoiosProvider,
    pool_hash: &str,
) -> Result<String, DataProviderKoiosError> {
    todo!()
}

pub fn pool_owner(
    bfp: &KoiosProvider,
    pool_hash: &str,
) -> Result<String, DataProviderKoiosError> {
    todo!()
}

pub fn pool_registration(
    bfp: &KoiosProvider,
    pool_hash: &str,
) -> Result<i64, DataProviderKoiosError> {
    todo!()
}

pub fn pool_retirement(
    bfp: &KoiosProvider,
    pool_hash: &str,
) -> Result<i32, DataProviderKoiosError> {
    todo!()
}

pub fn pool_url(
    bfp: &KoiosProvider,
    pool_hash: &str,
) -> Result<String, DataProviderKoiosError> {
    todo!()
}

pub fn pool_ticker(
    bfp: &KoiosProvider,
    pool_hash: &str,
) -> Result<String, DataProviderKoiosError> {
    todo!()
}

pub fn pool_metadata_json(
    bfp: &KoiosProvider,
    pool_hash: &str,
) -> Result<Value, DataProviderKoiosError> {
    todo!()
}

pub fn pool_name(
    bfp: &KoiosProvider,
    pool_hash: &str,
) -> Result<String, DataProviderKoiosError> {
    todo!()
}

pub fn pool_homepage(
    bfp: &KoiosProvider,
    pool_hash: &str,
) -> Result<String, DataProviderKoiosError> {
    todo!()
}

pub fn pool_description(
    bfp: &KoiosProvider,
    pool_hash: &str,
) -> Result<String, DataProviderKoiosError> {
    todo!()
}