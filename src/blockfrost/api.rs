use blockfrost::{stream::StreamExt, AddressTransaction, Transaction, PoolMetadata, AccountHistory, AccountReward};
use itertools::Itertools;

use super::error::DataProviderBlockfrostError;
use super::BlockfrostProvider;
use crate::models::{
    CDPDatum, CardanoNativeAssetView, DelegationView, HoldingWalletView, StakeDelegationView,
    StakeDeregistrationView, StakeRegistrationView, TokenInfoView, RewardView, TxHistoryListView, PoolView
};

use cardano_serialization_lib as csl;
use crate::provider::error::DataProviderError;
use blockfrost::{AccountAddress, AddressUtxo};
use bigdecimal::BigDecimal;
use array_tool::vec::Uniq;

use log::debug;

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

// Convert a Blockfrost UTxO (AddressUtxo) to CSL UTxO (TransactionUnspentOutput)
fn bf_utxo_as_csl_utxo(address_utxo: &AddressUtxo) -> csl::utils::TransactionUnspentOutput {
    //debug!("C1: {:?}", address_utxo.tx_hash.clone());
    let input = csl::TransactionInput::new(
        &csl::crypto::TransactionHash::from_bytes(hex::decode(&address_utxo.tx_hash).unwrap()).unwrap(),
        address_utxo.output_index,
    );

    let caddr = csl::address::Address::from_bech32(&address_utxo.address).unwrap();
    let mut cvalue = csl::utils::Value::zero();
    let mut ma = csl::MultiAsset::new();
    for amount in address_utxo.amount.iter() {
        let qty = csl::utils::BigNum::from_str(&amount.quantity).unwrap();
        //debug!("amount: {:?}", amount);
        if amount.unit == "lovelace" {
            cvalue.set_coin(&qty);
        } else {
            let (hexpolicy, hexname) = amount.unit.split_at(56);
            ma.set_asset(
                &csl::PolicyID::from_bytes(hex::decode(hexpolicy).unwrap()).unwrap(),
                &csl::AssetName::new(hex::decode(hexname).unwrap()).unwrap(),
                qty);
        }
    };
    if ma.len() > 0 {
        cvalue.set_multiasset(&ma);
    }

    let mut output = csl::TransactionOutput::new(&caddr, &cvalue);

    address_utxo.inline_datum.as_ref().map(|datum| {
        let plutus_data = csl::plutus::PlutusData::from_hex(&datum).unwrap();
        output.set_plutus_data(&plutus_data);
    });

    csl::utils::TransactionUnspentOutput::new(&input, &output)
}

/// get all utxos of an address
pub async fn get_address_utxos(
    bfp: &BlockfrostProvider,
    addr: &str,
) -> Result<dcslc::TransactionUnspentOutputs, DataProviderBlockfrostError> {

    let mut address_utxos = Vec::<AddressUtxo>::new();

    // TODO: See if we could fold with take_while instead of take(10000)
    // utxos.extend(
    //     bfp.api
    //         .addresses_utxos_all(&addr)
    //         .fold(Vec::<AddressUtxo>::new(), |mut acc, n| async move {
    //             acc.extend(n.unwrap().iter().map(|n| n.to_owned()));
    //             acc
    //         })
    //         .await,
    // );
    let mut lister =  bfp.api.addresses_utxos_all(&addr).take(10000);
    while let Some(n) = lister.next().await {
        let n = n.map_err( |e| DataProviderBlockfrostError::GeneralError(e.to_string()))?;
        if n.is_empty() {
            break;
        };
        address_utxos.extend(n);
    }
    let mut utxos = dcslc::TransactionUnspentOutputs::new();
    for address_utxo in address_utxos {
        // TODO: Add error handling to bf_utxo_as_csl_utxo
        utxos.add(&bf_utxo_as_csl_utxo(&address_utxo));
    }

    Ok(utxos)
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

/// Check if addresses exist
pub async fn addresses_exist(
    bfp: &BlockfrostProvider,
    addresses: &Vec<&str>,
) -> Result<Vec<bool>, DataProviderBlockfrostError> {
    let mut exists = Vec::with_capacity(addresses.len());
    for addr in addresses {
        exists.push(bfp.api.addresses(&addr).await.is_ok())
    };
    Ok(exists)
}


/// Get list of txs involving addresses
pub async fn get_addresses_transactions(
    bfp: &BlockfrostProvider,
    addresses: &Vec<&str>,
    _slot: Option<u64>,
) -> Result<Vec<TxHistoryListView>, DataProviderBlockfrostError> {

    let mut address_txs = Vec::<AddressTransaction>::new();

    // TODO: See if we could fold with take_while instead of take(10000)
    for addr in addresses {
        let mut lister =  bfp.api.addresses_transactions_all(&addr).take(10000);
        while let Some(n) = lister.next().await {
            let n = n.map_err( |e| DataProviderBlockfrostError::GeneralError(e.to_string()))?;
            if n.is_empty() {
                break;
            };
            address_txs.extend(n);
        }
    }

    let tx_hashes = address_txs.iter().map(|t| t.tx_hash.to_owned()).collect::<Vec<_>>().unique();

    let mut transactions = Vec::<Transaction>::new();
    for tx_hash in tx_hashes {
        let tx = bfp.api
            .transaction_by_hash(&tx_hash)
            .await
            .map_err( |e| DataProviderBlockfrostError::GeneralError(e.to_string()))?;
        transactions.push(tx);
        
    }

    let r =  transactions.iter().map(|tx| TxHistoryListView::from_blockfrost_tx(tx)).collect::<Vec<_>>();
    Ok(r)
}

/// Get paged list of active pools with details
pub async fn active_pools(
    bfp: &BlockfrostProvider,
    page: usize,
) -> Result<Vec<PoolView>, DataProviderBlockfrostError> {
    let mut pool_ids = Vec::<String>::new();

    // TODO: See if we could fold with take_while instead of take(10000)
    let mut lister =  bfp.api.pools_all().take(10000);
    while let Some(n) = lister.next().await {
        let n = n.map_err( |e| DataProviderBlockfrostError::GeneralError(e.to_string()))?;
        if n.is_empty() {
            break;
        };
        pool_ids.extend(n);
    }

    let pool_ids_page = match pool_ids.chunks(50).nth(page) {
        Some(pool_ids) => pool_ids,
        None => &[],
    };

    let mut pools = Vec::<PoolMetadata>::new();

    for pool_id in pool_ids_page {
        let pool_metadata = bfp.api.pools_metadata(&pool_id)
            .await
            .map_err( |e| DataProviderBlockfrostError::GeneralError(e.to_string()))?;
        pools.push(pool_metadata);
        
    }

    let r =  pools.iter().map(|pm|
                                     PoolView {
                                         pool_hash: pm.pool_id.clone(),
                                         ticker: pm.ticker.as_ref().unwrap_or(&"".to_string()).clone(),
                                         json: serde_json::json!({
                                             "name": pm.name.as_ref().unwrap_or(&"".to_string()).clone(),
                                             "homepage": pm.homepage.as_ref().unwrap_or(&"".to_string()).clone(),
                                             "description": pm.name.as_ref().unwrap_or(&"".to_string()).clone(),
                                         }),
                                     })
        .collect::<Vec<_>>();

    Ok(r)
}


pub fn find_datums_for_tx(
    bfp: &BlockfrostProvider,
    txid: &Vec<u8>,
) -> Result<Vec<CDPDatum>, DataProviderError> {
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
    _bfp: &BlockfrostProvider,
    _fingerprint_in: &str,
) -> Result<TokenInfoView, DataProviderBlockfrostError> {
    // Impossible only with blockfrost, not supported querying any asset info from fingerprint
    // (and is not possible to get policy+assetname from fingerprint)
    // See https://blockfrost.dev/support/cardano#poolpm-uses-fingerprints-for-querying-assets-why-dont-you-too
    Err(DataProviderBlockfrostError::Custom("Fingerprint unsupported, use policy/assetname instead".to_string()))
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

pub async fn retrieve_staked_amount (
    bfp: &BlockfrostProvider,
    epoch: i32,
    stake_addr: &str,
) -> Result<BigDecimal, DataProviderError> {

    let mut account_history = Vec::<AccountHistory>::new();
    // TODO: See if we could fold with take_while instead of take(10000)
    let mut lister =  bfp.api.accounts_history_all(stake_addr).take(10000);
    while let Some(n) = lister.next().await {
        let n = n.map_err( |e| DataProviderBlockfrostError::GeneralError(e.to_string()))?;
        if n.is_empty() {
            break;
        };
        account_history.extend(n);
    }

    debug!("AC: {:?}", account_history);

    let i128_epoch = i128::from(epoch);
    let amount = account_history.iter()
        .find(|h| h.active_epoch == i128_epoch)
        .and_then(|h| h.amount.parse::<u128>()).unwrap_or(0);

    debug!("Amount: {:?}", amount);
    Ok(BigDecimal::from(amount))
}

pub async fn retrieve_generated_rewards (
    bfp: &BlockfrostProvider,
    stake_addr: &str,
) -> Result<Vec<RewardView>, DataProviderError> {

    let mut account_rewards = Vec::<AccountReward>::new();

    // TODO: See if we could fold with take_while instead of take(10000)
    let mut lister =  bfp.api.accounts_rewards_all(stake_addr).take(10000);
    while let Some(n) = lister.next().await {
        let n = n.map_err( |e| DataProviderBlockfrostError::GeneralError(e.to_string()))?;
        if n.is_empty() {
            break;
        };
        account_rewards.extend(n);
    }

    debug!("AC: {:?}", account_rewards);

    let r =  account_rewards.iter().map(|h|
                                        RewardView {
                                            amount: h.amount.parse::<u64>().unwrap_or(0),
                                            earned_epoch: i64::try_from(h.epoch).unwrap_or(0),
                                            spendable_epoch: i64::try_from(h.epoch+2).unwrap_or(0),
                                        })
        .collect::<Vec<_>>();

    Ok(r)
}
