use blockfrost::stream::StreamExt;

use super::error::DataProviderBlockfrostError;
use super::BlockfrostProvider;
use crate::models::{
    CDPDatum, CardanoNativeAssetView, DelegationView, HoldingWalletView, StakeDelegationView,
    StakeDeregistrationView, StakeRegistrationView, TokenInfoView, RewardView,
};
use blockfrost::{AccountAddress, AddressUtxo};
use bigdecimal::BigDecimal;

/// get all tokens of an utxo
pub fn get_utxo_tokens(
    bfp: &BlockfrostProvider,
    tx_id: i64,
    tx_index: i16,
) -> Result<Vec<CardanoNativeAssetView>, DataProviderBlockfrostError> {
    todo!()
}

pub fn select_addr_of_first_transaction(
    bfp: &BlockfrostProvider,
    stake_address_in: &str,
) -> Result<String, DataProviderBlockfrostError> {
    Ok("".to_owned())
}

/// get all utxos of an address
pub fn utxo_by_dataumhash(
    bfp: &BlockfrostProvider,
    addr: &str,
    datumhash: &Vec<u8>,
) -> Result<dcslc::TransactionUnspentOutput, DataProviderBlockfrostError> {
    Err(DataProviderBlockfrostError::Custom(
        "not implemented".to_string(),
    ))
}

/// returns Utxo of a certain datumhash on an address
pub fn utxo_by_txid(
    bfp: &BlockfrostProvider,
    txhash: &Vec<u8>,
    index: i16,
) -> Result<dcslc::TransactionUnspentOutput, DataProviderBlockfrostError> {
    Err(DataProviderBlockfrostError::Custom(
        "not implemented".to_string(),
    ))
}

/// get all utxos of an address
pub async fn get_address_utxos(
    bfp: &BlockfrostProvider,
    addr: &str,
) -> Result<dcslc::TransactionUnspentOutputs, DataProviderBlockfrostError> {
    let mut utxos = Vec::<AddressUtxo>::new();
    utxos.extend(
        bfp.api
            .addresses_utxos_all(&addr)
            .fold(Vec::<AddressUtxo>::new(), |mut acc, n| async move {
                acc.extend(n.unwrap().iter().map(|n| n.to_owned()));
                acc
            })
            .await,
    );
    Ok(dcslc::TransactionUnspentOutputs::new())
}

/// Get all utxos of a stake address
pub async fn get_stake_address_utxos(
    bfp: &BlockfrostProvider,
    stake_addr: &str,
) -> Result<dcslc::TransactionUnspentOutputs, DataProviderBlockfrostError> {
    let addresses = bfp
        .api
        .accounts_addresses_all(stake_addr)
        .fold(Vec::<AccountAddress>::new(), |mut acc, n| async move {
            acc.extend(n.unwrap().iter().map(|n| n.to_owned()));
            acc
        })
        .await;

    let mut utxos = Vec::<AddressUtxo>::new();
    for address in addresses {
        utxos.extend(
            bfp.api
                .addresses_utxos_all(&address.address)
                .fold(Vec::<AddressUtxo>::new(), |mut acc, n| async move {
                    acc.extend(n.unwrap().iter().map(|n| n.to_owned()));
                    acc
                })
                .await,
        );
    }

    Ok(dcslc::TransactionUnspentOutputs::new())
}

pub async fn asset_utxos_on_addr(
    bfp: &BlockfrostProvider,
    addr: &str,
) -> Result<dcslc::TransactionUnspentOutputs, DataProviderBlockfrostError> {
    let mut utxos = Vec::<AddressUtxo>::new();
    utxos.extend(
        bfp.api
            .addresses_utxos_all(&addr)
            .fold(Vec::<AddressUtxo>::new(), |mut acc, n| async move {
                acc.extend(n.unwrap().iter().map(|n| n.to_owned()));
                acc
            })
            .await,
    );

    // ToDo: Filter asset utxos and transform

    Ok(dcslc::TransactionUnspentOutputs::new())
}

pub fn find_datums_for_tx(
    bfp: &BlockfrostProvider,
    txid: &Vec<u8>,
) -> Result<Vec<CDPDatum>, crate::provider::error::DataProviderError> {
    todo!();
}

pub async fn slot(bfp: &BlockfrostProvider) -> Result<i64, DataProviderBlockfrostError> {
    let block = bfp.api.blocks_latest().await.unwrap();
    Ok(block.slot.unwrap() as i64)
}

pub fn pool_delegations(
    bfp: &BlockfrostProvider,
    pool: &str,
    epoch: i32,
) -> Result<Vec<StakeDelegationView>, DataProviderBlockfrostError> {
    todo!();
}

pub fn deligations_per_pool_for_epochs(
    bfp: &BlockfrostProvider,
    pool: &str,
    start_epoch: i64,
    end_epoch: i64,
) -> Result<Vec<DelegationView>, DataProviderBlockfrostError> {
    todo!();
}

pub fn pool_total_stake(
    bfp: &BlockfrostProvider,
    pool: &str,
    epoch: i32,
) -> Result<u64, DataProviderBlockfrostError> {
    Ok(0)
}

pub async fn current_epoch(bfp: &BlockfrostProvider) -> Result<i32, DataProviderBlockfrostError> {
    let epoch = bfp.api.epochs_latest().await.unwrap();
    Ok(epoch.epoch as i32)
}

pub fn fingerprint(
    bfp: &BlockfrostProvider,
    policy: &str,
    tokenname: &str,
) -> Result<String, DataProviderBlockfrostError> {
    Ok("".to_owned())
}

pub fn token_info(
    bfp: &BlockfrostProvider,
    fingerprint_in: &str,
) -> Result<TokenInfoView, DataProviderBlockfrostError> {
    todo!();
}

pub fn stake_registration(
    bfp: &BlockfrostProvider,
    stake_addr_in: &str,
) -> Result<Vec<StakeRegistrationView>, DataProviderBlockfrostError> {
    todo!();
}

#[allow(clippy::type_complexity)]
pub fn stake_deregistration(
    bfp: &BlockfrostProvider,
    stake_addr_in: &str,
) -> Result<Vec<StakeDeregistrationView>, DataProviderBlockfrostError> {
    todo!();
}

pub fn check_stakeaddr_registered(
    bfp: &BlockfrostProvider,
    stake_addr_in: &str,
) -> Result<bool, DataProviderBlockfrostError> {
    Ok(false)
}

pub fn lookup_token_holders(
    bfp: &BlockfrostProvider,
    fingerprint_in: &str,
    min_amount: Option<&i64>,
) -> Result<Vec<HoldingWalletView>, DataProviderBlockfrostError> {
    todo!();
}

pub fn lookup_nft_token_holders(
    bfp: &BlockfrostProvider,
    policy: &str,
) -> Result<Vec<HoldingWalletView>, DataProviderBlockfrostError> {
    todo!();
}

pub fn mint_metadata(
    bfp: &BlockfrostProvider,
    fingerprint_in: &str,
) -> Result<TokenInfoView, DataProviderBlockfrostError> {
    Ok(TokenInfoView {
        fingerprint: "".to_owned(),
        policy: hex::encode(""),
        tokenname: String::from_utf8(hex::decode("")?)?,
        meta_key: None,
        json: None,
        txhash: None,
        quantity: None,
        mint_slot: None,
    })
}

pub fn pool_valid(
    bfp: &BlockfrostProvider,
    pool_id: &str,
) -> Result<bool, DataProviderBlockfrostError> {
    Ok(true)
}

pub fn txhash_spent(
    bfp: &BlockfrostProvider,
    txhash: &str,
) -> Result<bool, DataProviderBlockfrostError> {
    let txh_b = hex::decode(txhash)?;

    Ok(false)
}

pub fn retrieve_staked_amount (
    bfp: &BlockfrostProvider,
    epoch: i32,
    stake_addr: &str,
) -> Result<BigDecimal, crate::provider::error::DataProviderError> {
    Ok(BigDecimal::from(0))
}

pub fn retrieve_generated_rewards (
    bfp: &BlockfrostProvider,
    stake_addr: &str,
) -> Result<Vec<RewardView>, crate::provider::error::DataProviderError> {
    Ok(vec![])
}
