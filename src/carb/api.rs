use super::error::DataProviderCarbError;
use super::CarbProvider;
use crate::models::{
    CDPDatum, CardanoNativeAssetView, DelegationView, HoldingWalletView, StakeDelegationView,
    StakeDeregistrationView, StakeRegistrationView, TokenInfoView, RewardView,
};
use crate::provider::error::DataProviderError;

use bigdecimal::BigDecimal;
use serde_json::{Value, json};

/// get all tokens of an utxo
pub fn get_utxo_tokens(
    bfp: &CarbProvider,
    tx_id: i64,
    tx_index: i16,
) -> Result<Vec<CardanoNativeAssetView>, DataProviderCarbError> {
    todo!()
}

pub fn select_addr_of_first_transaction(
    bfp: &CarbProvider,
    stake_address_in: &str,
) -> Result<String, DataProviderCarbError> {
    todo!()
}

/// get all utxos of an address
pub fn utxo_by_dataumhash(
    bfp: &CarbProvider,
    addr: &str,
    datumhash: &Vec<u8>,
) -> Result<dcslc::TransactionUnspentOutput, DataProviderCarbError> {
    todo!()
}

/// returns Utxo of a certain datumhash on an address
pub fn utxo_by_txid(
    bfp: &CarbProvider,
    txhash: &Vec<u8>,
    index: i16,
) -> Result<dcslc::TransactionUnspentOutput, DataProviderCarbError> {
    todo!()
}

/// get all utxos of an address
pub fn get_address_utxos(
    bfp: &CarbProvider,
    addr: &str,
) -> Result<dcslc::TransactionUnspentOutputs, DataProviderCarbError> {
    todo!()
}

/// Get all utxos of a stake address
pub fn get_stake_address_utxos(
    bfp: &CarbProvider,
    stake_addr: &str,
) -> Result<dcslc::TransactionUnspentOutputs, DataProviderCarbError> {
    todo!()
}

/// Get all utxos of a stake address
pub fn asset_utxos_on_addr(
    bfp: &CarbProvider,
    addr: &str,
) -> Result<dcslc::TransactionUnspentOutputs, DataProviderCarbError> {
    todo!()
}

pub fn find_datums_for_tx(
    bfp: &CarbProvider,
    txid: &Vec<u8>,
) -> Result<Vec<CDPDatum>, DataProviderError> {
    todo!()
}

pub fn slot(bfp: &CarbProvider) -> Result<i64, DataProviderCarbError> {
    todo!()
}

pub fn stakers_on_pool(
    bfp: &CarbProvider,
    pool: &str,
    epoch: i32,
) -> Result<Vec<StakeDelegationView>, DataProviderCarbError> {
    todo!()
}

pub fn delegations_per_pool_for_epochs(
    bfp: &CarbProvider,
    pool: &str,
    start_epoch: i64,
    end_epoch: i64,
) -> Result<Vec<DelegationView>, DataProviderCarbError> {
    todo!()
}

pub fn pool_total_stake(
    bfp: &CarbProvider,
    pool: &str,
    epoch: i32,
) -> Result<u64, DataProviderCarbError> {
    Ok(0)
}

pub fn current_epoch(bfp: &CarbProvider) -> Result<i32, DataProviderCarbError> {
    Ok(0)
}

pub fn fingerprint(
    bfp: &CarbProvider,
    policy: &str,
    tokenname: &str,
) -> Result<String, DataProviderCarbError> {
    todo!()
}

pub fn token_info(
    bfp: &CarbProvider,
    fingerprint_in: &str,
) -> Result<TokenInfoView, DataProviderCarbError> {
    todo!();
}

pub fn stake_registration(
    bfp: &CarbProvider,
    stake_addr_in: &str,
) -> Result<Vec<StakeRegistrationView>, DataProviderCarbError> {
    todo!();
}

#[allow(clippy::type_complexity)]
pub fn stake_deregistration(
    bfp: &CarbProvider,
    stake_addr_in: &str,
) -> Result<Vec<StakeDeregistrationView>, DataProviderCarbError> {
    todo!();
}

pub fn check_stakeaddr_registered(
    bfp: &CarbProvider,
    stake_addr_in: &str,
) -> Result<bool, DataProviderCarbError> {
    todo!()
}

pub fn lookup_token_holders(
    bfp: &CarbProvider,
    fingerprint_in: &str,
    min_amount: Option<&i64>,
) -> Result<Vec<HoldingWalletView>, DataProviderCarbError> {
    todo!();
}

pub fn lookup_nft_token_holders(
    bfp: &CarbProvider,
    policy: &str,
) -> Result<Vec<HoldingWalletView>, DataProviderCarbError> {
    todo!();
}

pub fn mint_metadata(
    bfp: &CarbProvider,
    fingerprint_in: &str,
) -> Result<TokenInfoView, DataProviderCarbError> {
    todo!()
}

pub fn pool_valid(bfp: &CarbProvider, pool_id: &str) -> Result<bool, DataProviderCarbError> {
    todo!()
}

pub fn txhash_spent(bfp: &CarbProvider, txhash: &str) -> Result<bool, DataProviderCarbError> {
    let txh_b = hex::decode(txhash)?;

    todo!()
}

pub fn retrieve_staked_amount (
    bfp: &CarbProvider,
    epoch: i32,
    stake_addr: &str,
) -> Result<BigDecimal, DataProviderError> {
    todo!()
}

pub fn retrieve_generated_rewards (
    bfp: &CarbProvider,
    stake_addr: &str,
) -> Result<Vec<RewardView>, DataProviderError> {
    todo!()
}

pub fn pool_vrf_key_hash (
    bfp: &CarbProvider,
    pool_hash: &str,
) -> Result<Vec<u8>, DataProviderError> {
    todo!()
}

pub fn pool_blocks_minted (
    bfp: &CarbProvider,
    pool_hash: &str,
) -> Result<i64, DataProviderError> {
    todo!()
}

pub fn pool_blocks_current_epoch(
    bfp: &CarbProvider,
    pool_hash: &str,
) -> Result<i64, DataProviderError> {
    todo!()
}

pub fn pool_reward_recipients(
    bfp: &CarbProvider,
    pool_hash: &str,
) -> Result<i64, DataProviderError> {
    todo!()
}

pub fn pool_last_reward_earned_epoch(
    bfp: &CarbProvider,
    pool_hash: &str,
) -> Result<i64, DataProviderError> {
    todo!()
}

pub fn pool_declared_pledge(
    bfp: &CarbProvider,
    pool_hash: &str,
) -> Result<BigDecimal, DataProviderError> {
    todo!()
}

pub fn pool_margin_cost (
    bfp: &CarbProvider,
    pool_hash: &str,
) -> Result<f64, DataProviderError> {
    todo!()
}

pub fn pool_fixed_cost(
    bfp: &CarbProvider,
    pool_hash: &str,
) -> Result<BigDecimal, DataProviderError> {
    todo!()
}

pub fn pool_reward_address(
    bfp: &CarbProvider,
    pool_hash: &str,
) -> Result<String, DataProviderError> {
    todo!()
}

pub fn pool_owner(
    bfp: &CarbProvider,
    pool_hash: &str,
) -> Result<String, DataProviderError> {
    todo!()
}

pub fn pool_registration(
    bfp: &CarbProvider,
    pool_hash: &str,
) -> Result<i64, DataProviderError> {
    todo!()
}

pub fn pool_retirement(
    bfp: &CarbProvider,
    pool_hash: &str,
) -> Result<i32, DataProviderError> {
    todo!()
}

pub fn pool_url(
    bfp: &CarbProvider,
    pool_hash: &str,
) -> Result<String, DataProviderError> {
    todo!()
}

pub fn pool_ticker(
    bfp: &CarbProvider,
    pool_hash: &str,
) -> Result<String, DataProviderError> {
    todo!()
}

pub fn pool_metadata_json(
    bfp: &CarbProvider,
    pool_hash: &str,
) -> Result<Value, DataProviderError> {
    todo!()
}

pub fn pool_name(
    bfp: &CarbProvider,
    pool_hash: &str,
) -> Result<String, DataProviderError> {
    todo!()
}

pub fn pool_homepage(
    bfp: &CarbProvider,
    pool_hash: &str,
) -> Result<String, DataProviderError> {
    todo!()
}

pub fn pool_description(
    bfp: &CarbProvider,
    pool_hash: &str,
) -> Result<String, DataProviderError> {
    todo!()
}