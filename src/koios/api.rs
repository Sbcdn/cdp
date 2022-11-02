use super::error::DataProviderKoiosError;
use super::KoiosProvider;
use crate::models::{
    CardanoNativeAssetView, DelegationView, HoldingWalletView, StakeDelegationView,
    StakeDeregistrationView, StakeRegistrationView, TokenInfoView,
};

/// get all tokens of an utxo
pub fn get_utxo_tokens(
    bfp: &KoiosProvider,
    utxo_id: i64,
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
pub fn get_address_utxos(
    bfp: &KoiosProvider,
    addr: &str,
) -> Result<drasil_csl_common::TransactionUnspentOutputs, DataProviderKoiosError> {
    Ok(drasil_csl_common::TransactionUnspentOutputs::new())
}

/// Get all utxos of a stake address
pub fn get_stake_address_utxos(
    bfp: &KoiosProvider,
    stake_addr: &str,
) -> Result<drasil_csl_common::TransactionUnspentOutputs, DataProviderKoiosError> {
    Ok(drasil_csl_common::TransactionUnspentOutputs::new())
}

/// Get all utxos of a stake address
pub fn asset_utxos_on_addr(
    bfp: &KoiosProvider,
    addr: &str,
) -> Result<drasil_csl_common::TransactionUnspentOutputs, DataProviderKoiosError> {
    Ok(drasil_csl_common::TransactionUnspentOutputs::new())
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
        tokenname: String::from_utf8(hex::decode("".to_owned())?)?,
        meta_key: None,
        json: None,
        txhash: None,
        quantity: None,
    })
}

pub fn pool_valid(bfp: &KoiosProvider, pool_id: &str) -> Result<bool, DataProviderKoiosError> {
    Ok(true)
}

pub fn txhash_spent(bfp: &KoiosProvider, txhash: &str) -> Result<bool, DataProviderKoiosError> {
    let txh_b = hex::decode(txhash)?;

    Ok(false)
}
