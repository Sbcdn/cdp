use super::error::DataProviderCarbError;
use super::CarbProvider;
use crate::models::{
    CardanoNativeAssetView, DelegationView, HoldingWalletView, StakeDelegationView,
    StakeDeregistrationView, StakeRegistrationView, TokenInfoView,
};

/// get all tokens of an utxo
pub fn get_utxo_tokens(
    bfp: &CarbProvider,
    utxo_id: i64,
) -> Result<Vec<CardanoNativeAssetView>, DataProviderCarbError> {
    todo!()
}

pub fn select_addr_of_first_transaction(
    bfp: &CarbProvider,
    stake_address_in: &str,
) -> Result<String, DataProviderCarbError> {
    Ok("".to_owned())
}

/// get all utxos of an address
pub fn get_address_utxos(
    bfp: &CarbProvider,
    addr: &str,
) -> Result<drasil_csl_common::TransactionUnspentOutputs, DataProviderCarbError> {
    Ok(drasil_csl_common::TransactionUnspentOutputs::new())
}

/// Get all utxos of a stake address
pub fn get_stake_address_utxos(
    bfp: &CarbProvider,
    stake_addr: &str,
) -> Result<drasil_csl_common::TransactionUnspentOutputs, DataProviderCarbError> {
    Ok(drasil_csl_common::TransactionUnspentOutputs::new())
}

/// Get all utxos of a stake address
pub fn asset_utxos_on_addr(
    bfp: &CarbProvider,
    addr: &str,
) -> Result<drasil_csl_common::TransactionUnspentOutputs, DataProviderCarbError> {
    Ok(drasil_csl_common::TransactionUnspentOutputs::new())
}

pub fn slot(bfp: &CarbProvider) -> Result<i64, DataProviderCarbError> {
    Ok(0)
}

pub fn stakers_on_pool(
    bfp: &CarbProvider,
    pool: &str,
    epoch: i32,
) -> Result<Vec<StakeDelegationView>, DataProviderCarbError> {
    todo!();
}

pub fn deligations_per_pool_for_epochs(
    bfp: &CarbProvider,
    pool: &str,
    start_epoch: i64,
    end_epoch: i64,
) -> Result<Vec<DelegationView>, DataProviderCarbError> {
    todo!();
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
    Ok("".to_owned())
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
    Ok(false)
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
    Ok(TokenInfoView {
        fingerprint: "".to_owned(),
        policy: hex::encode(""),
        tokenname: String::from_utf8(hex::decode("".to_owned())?)?,
        meta_key: None,
        json: None,
        txhash: None,
        quantity: None,
    })
}

pub fn pool_valid(bfp: &CarbProvider, pool_id: &str) -> Result<bool, DataProviderCarbError> {
    Ok(true)
}

pub fn txhash_spent(bfp: &CarbProvider, txhash: &str) -> Result<bool, DataProviderCarbError> {
    let txh_b = hex::decode(txhash)?;

    Ok(false)
}
