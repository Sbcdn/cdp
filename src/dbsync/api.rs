use super::error::DataProviderDBSyncError;
use super::models::{PoolHash, PoolRetire, UnspentUtxo, UtxoView, Rewardtype};
use super::schema::*;
use crate::models::{
    CardanoNativeAssetView, DelegationView, HoldingWalletView, ScriptView, StakeDelegationView,
    StakeDeregistrationView, StakeRegistrationView, TokenInfoView, TxHistoryListQuery,
    TxHistoryListView, UTxOView, WithdrawalView, RewardView, CDPDatum, PoolView, TxHistoryListQueryLight,
    TransactionView
    
};
use crate::DBSyncProvider;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use dcslc::TransactionUnspentOutputs;
use diesel::prelude::*;
use log::debug;
use std::str::FromStr;
/// get all tokens of an utxo

pub fn get_utxo_tokens(
    dbs: &DBSyncProvider,
    tx_id: i64,
    tx_index: i16,
) -> Result<Vec<CardanoNativeAssetView>, DataProviderDBSyncError> {
    let multi_assets = multi_asset::table
        .inner_join(ma_tx_out::table.on(multi_asset::id.eq(ma_tx_out::ident)))
        .inner_join(tx_out::table.on(tx_out::id.eq(ma_tx_out::tx_out_id)))
        .inner_join(utxo_view::table.on(utxo_view::id.eq(tx_out::id)))
        .filter(utxo_view::tx_id.eq(tx_id))
        .filter(utxo_view::index.eq(tx_index))
        //.select((multi_asset::id,multi_asset::policy,multi_asset::name,multi_asset::fingerprint))
        .select((
            multi_asset::id,
            multi_asset::policy,
            multi_asset::name,
            multi_asset::fingerprint,
            ma_tx_out::quantity,
        ))
        .load::<CardanoNativeAssetView>(&mut dbs.connect()?)?;
    Ok(multi_assets)
}

#[deprecated(since = "0.1.1")]
pub fn get_utxo_tokens_dep(
    dbs: &DBSyncProvider,
    utxo_id: i64,
) -> Result<Vec<CardanoNativeAssetView>, DataProviderDBSyncError> {
    let multi_assets = multi_asset::table
        .inner_join(ma_tx_out::table.on(multi_asset::id.eq(ma_tx_out::ident)))
        .inner_join(tx_out::table.on(tx_out::id.eq(ma_tx_out::tx_out_id)))
        .inner_join(unspent_utxos::table.on(unspent_utxos::tx_id.eq(tx_out::tx_id)))
        .filter(unspent_utxos::id.eq(utxo_id))
        //.select((multi_asset::id,multi_asset::policy,multi_asset::name,multi_asset::fingerprint))
        .select((
            multi_asset::id,
            multi_asset::policy,
            multi_asset::name,
            multi_asset::fingerprint,
            ma_tx_out::quantity,
        ))
        .load::<CardanoNativeAssetView>(&mut dbs.connect()?)?;
    Ok(multi_assets)
}

pub fn select_addr_of_first_transaction(
    dbs: &DBSyncProvider,
    stake_address_in: &str,
) -> Result<String, DataProviderDBSyncError> {
    let resp = tx_out::table
        .left_join(tx::table.on(tx_out::tx_id.eq(tx::id)))
        .left_join(block::table.on(tx::block_id.eq(block::id)))
        .left_join(
            stake_address::table.on(tx_out::stake_address_id.eq(stake_address::id.nullable())),
        )
        .filter(stake_address::view.eq(stake_address_in))
        .select(tx_out::address)
        .order(block::slot_no.asc())
        .first::<String>(&mut dbs.connect()?);
    let resp = resp?;
    Ok(resp)
}

/// get all utxos of an address
pub fn get_address_utxos(
    dbs: &DBSyncProvider,
    addr: &str,
) -> Result<dcslc::TransactionUnspentOutputs, DataProviderDBSyncError> {
    let unspent = utxo_view::table
        .filter(utxo_view::address.eq(addr))
        .load::<UtxoView>(&mut dbs.connect()?)?;
    let mut utxos = dcslc::TransactionUnspentOutputs::new();
    for u in unspent {
        utxos.add(&u.to_txuo(dbs)?);
    }
    Ok(utxos)
}

/// get all utxos of an address
pub fn utxo_by_dataumhash(
    dbs: &DBSyncProvider,
    addr: &str,
    datumhash: &Vec<u8>,
) -> Result<dcslc::TransactionUnspentOutput, DataProviderDBSyncError> {
    let unspent = utxo_view::table
        .filter(utxo_view::address.eq(addr))
        .filter(utxo_view::data_hash.eq(datumhash))
        .first::<UtxoView>(&mut dbs.connect()?)?;
    unspent.to_txuo(dbs)
}

/// get utxos by hash and index
pub fn utxo_by_txid(
    dbs: &DBSyncProvider,
    txhash: &Vec<u8>,
    index: i16,
) -> Result<dcslc::TransactionUnspentOutput, DataProviderDBSyncError> {
    let unspent = utxo_view::table
        .inner_join(tx::table.on(tx::id.eq(utxo_view::tx_id)))
        .filter(tx::hash.eq(txhash))
        .filter(utxo_view::index.eq(index))
        .select((
            utxo_view::id,
            utxo_view::tx_id,
            utxo_view::index,
            utxo_view::address,
            utxo_view::address_raw,
            utxo_view::address_has_script,
            utxo_view::payment_cred,
            utxo_view::stake_address_id,
            utxo_view::value,
            utxo_view::data_hash,
            utxo_view::inline_datum_id,
            utxo_view::reference_script_id,
        ))
        .first::<UtxoView>(&mut dbs.connect()?)?;
    unspent.to_txuo(dbs)
}

/// get utxos by hash and index
fn utxo_by_txid_db(
    dbs: &DBSyncProvider,
    txid: i64,
    index: i16,
) -> Result<dcslc::TransactionUnspentOutput, DataProviderDBSyncError> {
    let unspent = utxo_view::table
        .filter(utxo_view::id.eq(txid))
        .filter(utxo_view::index.eq(index))
        .select((
            utxo_view::id,
            utxo_view::tx_id,
            utxo_view::index,
            utxo_view::address,
            utxo_view::address_raw,
            utxo_view::address_has_script,
            utxo_view::payment_cred,
            utxo_view::stake_address_id,
            utxo_view::value,
            utxo_view::data_hash,
            utxo_view::inline_datum_id,
            utxo_view::reference_script_id,
        ))
        .first::<UtxoView>(&mut dbs.connect()?)?;
    unspent.to_txuo(dbs)
}

/// get all utxos of an address
pub fn get_stake_address_utxos(
    dbs: &DBSyncProvider,
    saddr: &str,
) -> Result<dcslc::TransactionUnspentOutputs, DataProviderDBSyncError> {
    let unspent = utxo_view::table
        .left_join(
            stake_address::table.on(utxo_view::stake_address_id.eq(stake_address::id.nullable())),
        )
        .filter(stake_address::view.eq(saddr))
        .select((
            utxo_view::id,
            utxo_view::tx_id,
            utxo_view::index,
            utxo_view::address,
            utxo_view::address_raw,
            utxo_view::address_has_script,
            utxo_view::payment_cred,
            utxo_view::stake_address_id,
            utxo_view::value,
            utxo_view::data_hash,
            utxo_view::inline_datum_id,
            utxo_view::reference_script_id,
        ))
        .load::<UtxoView>(&mut dbs.connect()?)?;
    let mut utxos = dcslc::TransactionUnspentOutputs::new();
    for u in unspent {
        utxos.add(&u.to_txuo(dbs)?);
    }
    Ok(utxos)
}

pub fn asset_utxos_on_addr(
    dbs: &DBSyncProvider,
    addr: &str,
) -> Result<dcslc::TransactionUnspentOutputs, DataProviderDBSyncError> {
    let unspent_assets: Vec<UtxoView> = utxo_view::table
        .inner_join(ma_tx_out::table.on(ma_tx_out::tx_out_id.eq(utxo_view::tx_id)))
        .inner_join(multi_asset::table.on(multi_asset::id.eq(ma_tx_out::ident)))
        .filter(utxo_view::address.eq(addr))
        .select((
            utxo_view::id,
            utxo_view::tx_id,
            utxo_view::index,
            utxo_view::address,
            utxo_view::address_raw,
            utxo_view::address_has_script,
            utxo_view::payment_cred,
            utxo_view::stake_address_id,
            utxo_view::value,
            utxo_view::data_hash,
            utxo_view::inline_datum_id,
            utxo_view::reference_script_id,
        ))
        .load::<UtxoView>(&mut dbs.connect()?)?;

    let mut utxos = dcslc::TransactionUnspentOutputs::new();
    unspent_assets.iter().for_each(|n| {
        utxos.add(
            &n.to_txuo(dbs)
                .expect("Could not convert into TransactionUnspentOutput"),
        )
    });

    Ok(utxos)
}

/// get all utxos of an address
#[deprecated(since = "0.1.1")]
pub fn get_address_utxos_dep(
    dbs: &DBSyncProvider,
    addr: &str,
) -> Result<dcslc::TransactionUnspentOutputs, DataProviderDBSyncError> {
    let unspent = unspent_utxos::table
        .filter(unspent_utxos::address.eq(addr))
        .load::<UnspentUtxo>(&mut dbs.connect()?)?;
    let mut utxos = dcslc::TransactionUnspentOutputs::new();
    for u in unspent {
        utxos.add(&u.to_txuo(dbs)?);
    }
    Ok(utxos)
}

/// Get all utxos of a stake address
#[deprecated(since = "0.1.1")]
pub fn get_stake_address_utxos_dep(
    dbs: &DBSyncProvider,
    stake_addr: &str,
) -> Result<dcslc::TransactionUnspentOutputs, DataProviderDBSyncError> {
    let unspent = unspent_utxos::table
        .filter(unspent_utxos::stake_address.eq(stake_addr))
        .filter(unspent_utxos::address_has_script.eq(false))
        .load::<UnspentUtxo>(&mut dbs.connect()?)?;
    let mut utxos = dcslc::TransactionUnspentOutputs::new();
    for u in unspent {
        utxos.add(&u.to_txuo(dbs)?);
    }

    Ok(utxos)
}

/// Get all utxos of a stake address
#[deprecated(since = "0.1.1")]
pub fn asset_utxos_on_addr_dep(
    dbs: &DBSyncProvider,
    addr: &str,
) -> Result<dcslc::TransactionUnspentOutputs, DataProviderDBSyncError> {
    let unspent_assets: Vec<UnspentUtxo> = unspent_utxos::table
        .inner_join(ma_tx_out::table.on(ma_tx_out::tx_out_id.eq(unspent_utxos::id)))
        .inner_join(multi_asset::table.on(multi_asset::id.eq(ma_tx_out::ident)))
        .select((
            unspent_utxos::id,
            unspent_utxos::tx_id,
            unspent_utxos::hash,
            unspent_utxos::index,
            unspent_utxos::address,
            unspent_utxos::value,
            unspent_utxos::data_hash,
            unspent_utxos::address_has_script,
            unspent_utxos::stake_address,
        ))
        .filter(unspent_utxos::address.eq(addr))
        .load::<UnspentUtxo>(&mut dbs.connect()?)?;

    let mut utxos = dcslc::TransactionUnspentOutputs::new();
    unspent_assets.iter().for_each(|n| {
        utxos.add(
            &n.to_txuo(dbs)
                .expect("Could not convert into TransactionUnspentOutput"),
        )
    });

    Ok(utxos)
}

/// get all in/out datums for a transaction
pub fn find_datums_for_tx(
    dbs: &DBSyncProvider,
    txid: &Vec<u8>,
) -> Result<Vec<CDPDatum>, DataProviderDBSyncError> {
    let mut datums = datum::table
        .inner_join(
            tx_out::table.on(tx_out::data_hash
                .eq(datum::hash.nullable())
                .or(tx_out::inline_datum_id.eq(datum::id.nullable()))),
        )
        .inner_join(tx::table.on(tx::id.eq(tx_out::tx_id)))
        .filter(tx::hash.eq(txid))
        .select((
            datum::hash,
            datum::value.nullable(),
            datum::bytes,
            tx_out::address,
            tx_out::address_has_script,
        ))
        .load::<CDPDatum>(&mut dbs.connect()?);

    if let Ok(o) = datums {
        if !o.is_empty() {
            return Ok(o);
        }
    }

    let r: i64 = redeemer::table
        .inner_join(tx::table.on(tx::id.eq(redeemer::tx_id)))
        .filter(tx::hash.eq(txid))
        //ToDo: FIX ME!
        //   .filter(redeemer::purpose.eq(super::models::Scriptpurposetype::Spend))
        .select(redeemer::tx_id)
        .first::<i64>(&mut dbs.connect()?)?;

    let t: (Option<Vec<u8>>, Option<i64>) = tx_out::table
        .inner_join(tx_in::table.on(tx_in::tx_out_id.eq(tx_out::tx_id)))
        .filter(tx_in::tx_in_id.eq(r))
        .filter(tx_in::tx_out_index.eq(tx_out::index))
        .filter(tx_out::data_hash.is_not_null())
        .filter(tx_out::address_has_script.eq(true))
        .select((
            tx_out::data_hash.nullable(),
            tx_out::inline_datum_id.nullable(),
        ))
        .first::<(Option<Vec<u8>>, Option<i64>)>(&mut dbs.connect()?)?;

    datums = datum::table
        .inner_join(
            tx_out::table.on(tx_out::data_hash
                .eq(datum::hash.nullable())
                .or(tx_out::inline_datum_id.eq(datum::id.nullable()))),
        )
        .filter(diesel::BoolExpressionMethods::or(
            datum::hash.eq(t.0.unwrap()),
            datum::id.eq(t.1.unwrap()),
        ))
        .select((
            datum::hash,
            datum::value.nullable(),
            datum::bytes,
            tx_out::address,
            tx_out::address_has_script,
        ))
        .load::<CDPDatum>(&mut dbs.connect()?);

    Ok(datums?)
}

pub fn slot(dbs: &DBSyncProvider) -> Result<i64, DataProviderDBSyncError> {
    let slot = block::table
        .filter(block::block_no.is_not_null())
        .select(block::slot_no)
        .order(block::slot_no.desc())
        .limit(1)
        .load::<Option<i64>>(&mut dbs.connect()?)?;
    match slot[0] {
        Some(s) => Ok(s),
        None => Err(DataProviderDBSyncError::Custom(
            "ERROR: Could not find slot number in DBsync".to_string(),
        )),
    }
}

pub fn stakers_on_pool(
    dbs: &DBSyncProvider,
    pool: &str,
    epoch: i32,
) -> Result<Vec<StakeDelegationView>, DataProviderDBSyncError> {
    let pool_stake = epoch_stake::table
        .inner_join(pool_hash::table.on(pool_hash::id.eq(epoch_stake::pool_id)))
        .inner_join(stake_address::table.on(epoch_stake::addr_id.eq(stake_address::id)))
        .filter(pool_hash::view.eq(pool))
        .filter(epoch_stake::epoch_no.eq(epoch))
        .select((stake_address::view, epoch_stake::amount))
        .load::<StakeDelegationView>(&mut dbs.connect()?)?;
    Ok(pool_stake)
}

pub fn deligations_per_pool_for_epochs(
    dbs: &DBSyncProvider,
    pool: &str,
    start_epoch: i64,
    end_epoch: i64,
) -> Result<Vec<DelegationView>, DataProviderDBSyncError> {
    let deleg = delegation::table
        .inner_join(pool_hash::table.on(pool_hash::id.eq(delegation::pool_hash_id)))
        .inner_join(stake_address::table.on(delegation::addr_id.eq(stake_address::id)))
        .inner_join(tx::table.on(delegation::tx_id.eq(tx::id)))
        .filter(pool_hash::view.eq(pool))
        .filter(delegation::active_epoch_no.ge(start_epoch))
        .filter(delegation::active_epoch_no.le(end_epoch))
        .select((
            stake_address::view,
            tx::deposit,
            delegation::cert_index,
            delegation::active_epoch_no,
        ))
        .load::<DelegationView>(&mut dbs.connect()?)?;
    Ok(deleg)
}

pub fn pool_total_stake(
    dbs: &DBSyncProvider,
    pool: &str,
    epoch: i32,
) -> Result<u64, DataProviderDBSyncError> {
    let pool_stake = epoch_stake::table
        .inner_join(pool_hash::table.on(pool_hash::id.eq(epoch_stake::pool_id)))
        .filter(pool_hash::view.eq(pool))
        .filter(epoch_stake::epoch_no.eq(epoch))
        .select(epoch_stake::amount)
        .load::<BigDecimal>(&mut dbs.connect()?)?;

    let tot_stake: u64 = pool_stake.iter().map(|x| x.to_u64().unwrap()).sum();

    Ok(tot_stake)
}

pub fn current_epoch(dbs: &DBSyncProvider) -> Result<i32, DataProviderDBSyncError> {
    let epoch = epoch_stake::table
        .filter(epoch_stake::epoch_no.is_not_null())
        .select(epoch_stake::epoch_no)
        .order(epoch_stake::epoch_no.desc())
        .first::<i32>(&mut dbs.connect()?)?;

    Ok(epoch)
}

pub fn fingerprint(
    dbs: &DBSyncProvider,
    policy: &str,
    tokenname: &str,
) -> Result<String, DataProviderDBSyncError> {
    let fingerprint = multi_asset::table
        .filter(multi_asset::policy.eq(hex::decode(policy)?))
        .filter(multi_asset::name.eq(tokenname.as_bytes()))
        .select(multi_asset::fingerprint)
        .first::<String>(&mut dbs.connect()?)?;

    Ok(fingerprint)
}

pub fn token_info(
    dbs: &DBSyncProvider,
    fingerprint_in: &str,
) -> Result<TokenInfoView, DataProviderDBSyncError> {
    let fingerprint = multi_asset::table
        .filter(multi_asset::fingerprint.eq(fingerprint_in))
        .select((multi_asset::policy, multi_asset::name))
        .first::<(Vec<u8>, Vec<u8>)>(&mut dbs.connect()?)?;

    let policy = hex::encode(fingerprint.0);
    let tokenname = fingerprint.1;

    let ti = TokenInfoView {
        policy,
        tokenname,
        fingerprint: fingerprint_in.to_owned(),
        quantity: None,
        meta_key: None,
        json: None,
        mint_slot: None,
        txhash: None,
    };

    Ok(ti)
}

#[allow(clippy::type_complexity)]
pub fn stake_registration(
    dbs: &DBSyncProvider,
    stake_addr_in: &str,
) -> Result<Vec<StakeRegistrationView>, DataProviderDBSyncError> {
    let registration = stake_registration::table
        .inner_join(stake_address::table.on(stake_registration::addr_id.eq(stake_address::id)))
        .inner_join(tx::table.on(stake_registration::tx_id.eq(tx::id)))
        .filter(stake_address::view.eq(stake_addr_in))
        .select((
            stake_address::view,
            tx::hash,
            stake_registration::cert_index,
            stake_registration::epoch_no,
        ))
        .order(stake_registration::epoch_no.desc())
        .load::<(String, Vec<u8>, i32, i32)>(&mut dbs.connect()?)?;

    let mut out = Vec::<StakeRegistrationView>::new();
    for d in registration {
        out.push(StakeRegistrationView {
            stake_address: d.0,
            tx_hash: d.1,
            cert_index: d.2,
            epoch: d.3,
        });
    }

    Ok(out)
}

#[allow(clippy::type_complexity)]
pub fn stake_deregistration(
    dbs: &DBSyncProvider,
    stake_addr_in: &str,
) -> Result<Vec<StakeDeregistrationView>, DataProviderDBSyncError> {
    let deregistration = stake_deregistration::table
        .inner_join(stake_address::table.on(stake_deregistration::addr_id.eq(stake_address::id)))
        .inner_join(tx::table.on(stake_deregistration::tx_id.eq(tx::id)))
        .filter(stake_address::view.eq(stake_addr_in))
        .select((
            stake_address::view,
            tx::hash,
            stake_deregistration::cert_index,
            stake_deregistration::epoch_no,
            stake_deregistration::redeemer_id,
        ))
        .order(stake_deregistration::epoch_no.desc())
        .load::<(String, Vec<u8>, i32, i32, Option<i64>)>(&mut dbs.connect()?)?;
    let mut out = Vec::<StakeDeregistrationView>::new();
    for d in deregistration {
        out.push(StakeDeregistrationView {
            stake_address: d.0,
            tx_hash: d.1,
            cert_index: d.2,
            epoch: d.3,
        });
    }
    Ok(out)
}

pub fn check_stakeaddr_registered(
    dbs: &DBSyncProvider,
    stake_addr_in: &str,
) -> Result<bool, DataProviderDBSyncError> {
    let registration = stake_registration::table
        .inner_join(stake_address::table.on(stake_registration::addr_id.eq(stake_address::id)))
        .inner_join(tx::table.on(stake_registration::tx_id.eq(tx::id)))
        .filter(stake_address::view.eq(stake_addr_in))
        .select((
            stake_address::view,
            tx::hash,
            stake_registration::cert_index,
            stake_registration::epoch_no,
        ))
        .order(stake_registration::epoch_no.desc())
        .load::<(String, Vec<u8>, i32, i32)>(&mut dbs.connect()?)?;

    let deregistration = stake_deregistration::table
        .inner_join(stake_address::table.on(stake_deregistration::addr_id.eq(stake_address::id)))
        .inner_join(tx::table.on(stake_deregistration::tx_id.eq(tx::id)))
        .filter(stake_address::view.eq(stake_addr_in))
        .select((
            stake_address::view,
            tx::hash,
            stake_deregistration::cert_index,
            stake_deregistration::epoch_no,
        ))
        .order(stake_deregistration::epoch_no.desc())
        .load::<(String, Vec<u8>, i32, i32)>(&mut dbs.connect()?)?;

    match registration.len() {
        0 => Ok(false),
        _ => match deregistration.len() {
            0 => Ok(true),
            _ => {
                if registration[0].3 > deregistration[0].3 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        },
    }
}

pub fn lookup_token_holders(
    dbs: &DBSyncProvider,
    fingerprint_in: &str,
    min_amount: Option<&i64>,
) -> Result<Vec<HoldingWalletView>, DataProviderDBSyncError> {
    let mut holders = unspent_utxos::table
        .inner_join(ma_tx_out::table.on(unspent_utxos::id.eq(ma_tx_out::tx_out_id)))
        .left_join(multi_asset::table.on(multi_asset::id.eq(ma_tx_out::ident)))
        .filter(multi_asset::fingerprint.eq(fingerprint_in))
        .filter(unspent_utxos::stake_address.is_not_null())
        .select((unspent_utxos::stake_address.nullable(), ma_tx_out::quantity))
        .load::<(Option<String>, BigDecimal)>(&mut dbs.connect()?)?;

    if let Some(amt) = min_amount {
        let a = BigDecimal::from_i64(*amt).unwrap();
        holders.retain(|p| p.1 >= a && p.0.is_some())
    } else {
        holders.retain(|p| p.0.is_some())
    }

    let mut ret = Vec::<HoldingWalletView>::new();
    ret.extend(holders.iter().map(|p| HoldingWalletView {
        stake_address: p.0.as_ref().unwrap().to_string(),
        amount: BigDecimal::to_u64(&p.1).unwrap(),
        tokenname: None,
        policy: "use fingerprint".to_owned(),
        fingerprint: Some(fingerprint_in.to_owned()),
    }));

    Ok(ret)
}

pub fn lookup_nft_token_holders(
    dbs: &DBSyncProvider,
    policy: &str,
) -> Result<Vec<HoldingWalletView>, DataProviderDBSyncError> {
    let pbyte = hex::decode(policy)?;

    let holders = unspent_utxos::table
        .inner_join(ma_tx_out::table.on(unspent_utxos::id.eq(ma_tx_out::tx_out_id)))
        .left_join(multi_asset::table.on(multi_asset::id.eq(ma_tx_out::ident)))
        .filter(multi_asset::policy.eq(pbyte))
        .filter(unspent_utxos::stake_address.is_not_null())
        .filter(ma_tx_out::quantity.eq(BigDecimal::from(1)))
        .select((unspent_utxos::stake_address.nullable(), ma_tx_out::quantity))
        .load::<(Option<String>, BigDecimal)>(&mut dbs.connect()?)
        .ok();

    let mut holders = if let Some(h) = holders { h } else { vec![] };

    holders.retain(|p| p.0.is_some());

    let mut ret = Vec::<HoldingWalletView>::new();
    ret.extend(holders.iter().map(|p| HoldingWalletView {
        stake_address: p.0.as_ref().unwrap().to_string(),
        amount: BigDecimal::to_u64(&p.1).unwrap(),
        tokenname: None,
        policy: policy.to_string(),
        fingerprint: None,
    }));

    Ok(ret)
}

pub fn mint_metadata(
    dbs: &DBSyncProvider,
    fingerprint_in: &str,
) -> Result<TokenInfoView, DataProviderDBSyncError> {
    let metadata = ma_tx_mint::table
        .inner_join(multi_asset::table.on(multi_asset::id.eq(ma_tx_mint::ident)))
        .inner_join(tx_metadata::table.on(tx_metadata::tx_id.eq(ma_tx_mint::tx_id)))
        .inner_join(tx::table.on(ma_tx_mint::tx_id.eq(tx::id)))
        .inner_join(block::table.on(tx::block_id.eq(block::id)))
        .filter(multi_asset::fingerprint.eq(fingerprint_in))
        .order_by(block::slot_no.desc())
        .select((
            multi_asset::fingerprint,
            multi_asset::policy,
            multi_asset::name,
            tx_metadata::key,
            tx_metadata::json.nullable(),
            tx::hash,
            block::slot_no.nullable(),
        ))
        .first::<(
            String,
            Vec<u8>,
            Vec<u8>,
            BigDecimal,
            Option<serde_json::Value>,
            Vec<u8>,
            Option<i64>,
        )>(&mut dbs.connect()?)
        .ok();
    if let Some(m) = metadata {
        let quantity = ma_tx_mint::table
            .inner_join(multi_asset::table.on(ma_tx_mint::ident.eq(multi_asset::id)))
            .filter(multi_asset::fingerprint.eq(m.0.clone()))
            .select(ma_tx_mint::quantity)
            .load::<BigDecimal>(&mut dbs.connect()?)
            .ok()
            .unwrap()
            .iter()
            .map(|n| n.to_u64())
            .sum();
        Ok(TokenInfoView {
            fingerprint: m.0,
            policy: hex::encode(m.1),
            tokenname: m.2,
            meta_key: Some(m.3.to_i64().unwrap()),
            json: m.4,
            txhash: Some(hex::encode(m.5)),
            quantity: quantity,
            mint_slot: m.6,
        })
    } else {
        let metadata = ma_tx_mint::table
            .inner_join(multi_asset::table.on(multi_asset::id.eq(ma_tx_mint::ident)))
            .inner_join(tx::table.on(ma_tx_mint::tx_id.eq(tx::id)))
            .inner_join(block::table.on(tx::block_id.eq(block::id)))
            .filter(multi_asset::fingerprint.eq(fingerprint_in))
            .order_by(block::slot_no.desc())
            .select((
                multi_asset::fingerprint,
                multi_asset::policy,
                multi_asset::name,
                tx::hash,
                block::slot_no.nullable(),
            ))
            .first::<(String, Vec<u8>, Vec<u8>, Vec<u8>, Option<i64>)>(&mut dbs.connect()?)
            .ok();
        if let Some(m) = metadata {
            let quantity = ma_tx_mint::table
                .inner_join(multi_asset::table.on(ma_tx_mint::ident.eq(multi_asset::id)))
                .filter(multi_asset::fingerprint.eq(m.0.clone()))
                .select(ma_tx_mint::quantity)
                .load::<BigDecimal>(&mut dbs.connect()?)
                .ok()
                .unwrap()
                .iter()
                .map(|n| n.to_u64())
                .sum();
            Ok(TokenInfoView {
                fingerprint: m.0,
                policy: hex::encode(m.1),
                tokenname: m.2,
                meta_key: None,
                json: None,
                txhash: Some(hex::encode(m.3)),
                quantity: quantity,
                mint_slot: m.4,
            })
        } else {
            Err(DataProviderDBSyncError::RequestValueNotFound(
                fingerprint_in.to_owned(),
            ))
        }
    }
}

pub async fn get_pools(
    dbs: &DBSyncProvider,
) -> Result<Vec<PoolView>, DataProviderDBSyncError> {
    let pools = pool_hash::table
        .left_join(pool_retire::table.on(pool_retire::hash_id.eq(pool_hash::id)))
        .inner_join(pool_offline_data::table.on(pool_offline_data::pool_id.eq(pool_hash::id)))
        .filter(pool_retire::hash_id.is_null())
        .select((
            pool_hash::view,
            pool_offline_data::ticker_name,
            pool_offline_data::json,
        ))
        .load::<PoolView>(&mut dbs.connect()?)
        .ok();
    log::debug!("get_pools: {:?}", pools);
    if let Some(p) = pools {
        Ok(p)
    } else {
        Err(DataProviderDBSyncError::RequestValueNotFound(
            "pools".to_owned(),
        ))
    }
}

pub async fn tx_metadata(
    dbs: &DBSyncProvider,
    hash: &str,
) -> Result<Option<serde_json::Value>, DataProviderDBSyncError> {
    let metadata: Option<Option<serde_json::Value>> = tx::table
        .inner_join(tx_metadata::table.on(tx_metadata::tx_id.eq(tx::id)))
        .filter(tx::hash.eq(hex::decode(hash)?))
        .select(tx_metadata::json.nullable())
        .first::<Option<serde_json::Value>>(&mut dbs.connect()?)
        .ok();

    if let Some(m) = metadata {
        Ok(m)
    } else {
        Ok(None)
    }
}

pub fn pool_valid(dbs: &DBSyncProvider, pool_id: &str) -> Result<bool, DataProviderDBSyncError> {
    let pool_stake = pool_hash::table
        .filter(pool_hash::view.eq(pool_id))
        .first::<PoolHash>(&mut dbs.connect()?)?;

    let pool_retire = pool_retire::table
        .filter(pool_retire::id.eq(&pool_stake.id))
        .load::<PoolRetire>(&mut dbs.connect()?)?;

    if !pool_retire.is_empty() {
        return Ok(false);
    }

    Ok(true)
}

pub fn txhash_spent(dbs: &DBSyncProvider, txhash: &str) -> Result<bool, DataProviderDBSyncError> {
    let txh_b = hex::decode(txhash)?;
    let tx = tx_out::table
        .inner_join(tx::table.on(tx::id.eq(tx_out::tx_id)))
        .left_join(
            tx_in::table.on(tx_in::tx_out_id
                .eq(tx::id)
                .and(tx_in::tx_out_index.eq(tx_out::index))),
        )
        .select((tx::hash, tx_out::index))
        .filter(tx_in::tx_in_id.is_not_null())
        .filter(tx::hash.eq(txh_b))
        .load::<(Vec<u8>, i16)>(&mut dbs.connect()?)?;
    if !tx.is_empty() {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn token_supply(
    dbs: &DBSyncProvider,
    fingerprint: &str,
) -> Result<Option<BigDecimal>, DataProviderDBSyncError> {
    utxo_view::table
        .inner_join(ma_tx_out::table.on(utxo_view::id.eq(ma_tx_out::tx_out_id)))
        .inner_join(multi_asset::table.on(multi_asset::id.eq(ma_tx_out::ident)))
        .filter(multi_asset::fingerprint.eq(fingerprint))
        .select(diesel::dsl::sum(ma_tx_out::quantity.nullable()))
        .first::<Option<BigDecimal>>(&mut dbs.connect()?)
        .map_err(|_| DataProviderDBSyncError::RequestValueNotFound(fingerprint.to_owned()))
}

pub async fn check_nft_supply(
    dbs: &DBSyncProvider,
    fingerprint: &str,
) -> Result<bool, DataProviderDBSyncError> {
    /*
    if let Ok(o) = utxo_view::table
         .inner_join(ma_tx_out::table.on(utxo_view::id.eq(ma_tx_out::tx_out_id)))
         .inner_join(multi_asset::table.on(multi_asset::id.eq(ma_tx_out::ident)))
         .filter(multi_asset::fingerprint.eq(fingerprint))
         .select(ma_tx_out::id)
         .limit(2)
         .load::<i64>(&mut dbs.connect()?)
         .map_err(|_| DataProviderDBSyncError::RequestValueNotFound(fingerprint.to_owned()))
     {
         if o.len() == 1 {
             return Ok(true);
         }
     }
     */
    debug!("check NFT supply called for {}", fingerprint);
    if let Ok(o) = utxo_token_view::table
        .filter(utxo_token_view::fingerprint.eq(fingerprint))
        .select(utxo_token_view::id)
        .limit(2)
        .load::<i64>(&mut dbs.connect()?)
        .map_err(|_| DataProviderDBSyncError::RequestValueNotFound(fingerprint.to_owned()))
    {
        if o.len() == 1 {
            return Ok(true);
        }
    }
    Ok(false)
}

pub async fn is_nft(
    dbs: &DBSyncProvider,
    fingerprints: &[&str],
) -> Result<Vec<bool>, DataProviderDBSyncError> {
    let mut out = Vec::<bool>::new();
    for f in fingerprints {
        if check_nft_supply(dbs, f).await? {
            out.push(true);
        } else {
            out.push(false);
        }
    }
    Ok(out)
}

pub fn addresses_exist(
    dbs: &DBSyncProvider,
    addresses: &Vec<&str>,
) -> Result<Vec<bool>, DataProviderDBSyncError> {
    let mut out = vec![];
    for a in addresses {
        let id = tx_out::table
            .select(tx_out::id)
            .filter(tx_out::address.eq(a))
            .load::<i64>(&mut dbs.connect()?)?;
        if id.is_empty() {
            out.push(false);
        } else {
            out.push(true);
        }
    }
    Ok(out)
}
// ToDo: Slot limitation to query just before a certain slot
fn _tx_history_q(
    dbs: &DBSyncProvider,
    addresses: &[&str],
    _slot: Option<u64>,
) -> Result<Vec<TxHistoryListView>, DataProviderDBSyncError> {
    let mut addresses = addresses.iter().fold("(".to_string(), |mut acc, a| {
        acc.push_str(&("'".to_string() + a + "',"));
        acc
    });
    addresses.pop();
    addresses.push(')');
    let query =format!("select 
    distinct t.hash,
    min(b.slot_no) as slot, asset.fingerprint,
    asset.value from tx_out to2
    join tx t on t.id = to2.tx_id
    join block b on b.id = t.block_id 
    join lateral (
        select array_agg(ma.fingerprint) as fingerprint, array_agg(mto.quantity) as value, mto.tx_out_id 
        from ma_tx_out as mto
        join multi_asset ma on ma.id = mto.ident
        where mto.tx_out_id = to2.id
        group by mto.tx_out_id 
        ) as asset on asset.tx_out_id  = to2.id 
    where address IN {addresses}
    group by t.hash, b.slot_no, asset.fingerprint, asset.value");
    log::debug!("{addresses}");
    let txhistory_q: Vec<TxHistoryListQuery> =
        diesel::sql_query(query).load(&mut dbs.connect()?).unwrap();

    let mut txhistory: Vec<_> = txhistory_q
        .iter()
        .map(TxHistoryListView::from_tx_history_list_query)
        .collect();
    txhistory.sort_by_key(|n| n.slot);
    Ok(txhistory)
}

// ToDo: Slot limitation to query just before a certain slot
pub fn tx_history(
    dbs: &DBSyncProvider,
    addresses: &[&str],
    slot: Option<u64>,
) -> Result<Vec<TxHistoryListView>, DataProviderDBSyncError> {
    let mut addresses = addresses.iter().fold("(".to_string(), |mut acc, a| {
        acc.push_str(&("'".to_string() + a + "',"));
        acc
    });
    addresses.pop();
    addresses.push(')');
    let query = format!(
        "select 
    distinct
    t.hash, min(b.slot_no) as slot from tx_out to2
    join tx t on t.id = to2.tx_id
    join block b on b.id = t.block_id 
    where address IN {addresses}
    group by b.slot_no, t.hash order by slot desc"
    );
    log::trace!("{addresses}");
    let txhistory: Option<Vec<TxHistoryListQueryLight>> =
        diesel::sql_query(query).load(&mut dbs.connect()?).ok();
    log::debug!("{txhistory:?}");
    let mut out = Vec::<TxHistoryListView>::new();
    if let Some(history) = txhistory {
        for t in history.into_iter() {
            out.push(TxHistoryListView {
                slot: t.slot,
                hash: hex::encode(t.hash),
                assets: vec![],
            });
        }
    }

    Ok(out)
}

pub async fn txo_by_id_index(
    dbs: &DBSyncProvider,
    id: i64,
    index: i16,
) -> Result<dcslc::TransactionUnspentOutput, DataProviderDBSyncError> {
    let txo = tx_out::table
        .filter(tx_out::id.eq(id))
        .filter(tx_out::index.eq(index))
        .select((
            tx_out::id,
            tx_out::tx_id,
            tx_out::index,
            tx_out::address,
            tx_out::address_raw,
            tx_out::address_has_script,
            tx_out::payment_cred,
            tx_out::stake_address_id,
            tx_out::value,
            tx_out::data_hash,
            tx_out::inline_datum_id,
            tx_out::reference_script_id,
        ))
        .first::<crate::dbsync::models::TxOut>(&mut dbs.connect()?)?;
    debug!("calling to_txo for: {:?}", txo);
    txo.to_txuo(dbs)
}

pub async fn collateral_txo_by_id_index(
    dbs: &DBSyncProvider,
    id: i64,
    index: i16,
) -> Result<dcslc::TransactionUnspentOutput, DataProviderDBSyncError> {
    let txo = collateral_tx_out::table
        .filter(collateral_tx_out::id.eq(id))
        .filter(collateral_tx_out::index.eq(index))
        .select((
            collateral_tx_out::id,
            collateral_tx_out::tx_id,
            collateral_tx_out::index,
            collateral_tx_out::address,
            collateral_tx_out::address_raw,
            collateral_tx_out::address_has_script,
            collateral_tx_out::payment_cred,
            collateral_tx_out::stake_address_id,
            collateral_tx_out::value,
            collateral_tx_out::data_hash,
            collateral_tx_out::inline_datum_id,
            collateral_tx_out::reference_script_id,
        ))
        .first::<crate::dbsync::models::TxOut>(&mut dbs.connect()?)?;
    txo.to_txuo(dbs)
}

pub async fn get_tx_inputs(
    dbs: &DBSyncProvider,
    hash: &str,
) -> Result<TransactionUnspentOutputs, DataProviderDBSyncError> {
    debug!("get_tx_inputs");
    let input_references: Vec<_> = tx::table
        .inner_join(tx_in::table.on(tx_in::tx_in_id.eq(tx::id)))
        .filter(tx::hash.eq(hex::decode(hash)?))
        .select((tx_in::tx_out_id, tx_in::tx_out_index))
        .load::<(i64, i16)>(&mut dbs.connect()?)?;
    let mut temp = Vec::<(i64, i16)>::new();
    debug!("enrich_tx_inputs");
    for i in input_references.into_iter() {
        let tx_out = tx_out::table
            .inner_join(tx::table.on(tx::id.eq(tx_out::tx_id)))
            .filter(tx::id.eq(i.0))
            .filter(tx_out::index.eq(i.1))
            .select((tx_out::id, tx_out::index))
            .first::<(i64, i16)>(&mut dbs.connect()?)?;
        temp.push(tx_out);
    }
    let mut txins = TransactionUnspentOutputs::new();
    for i in temp.into_iter() {
        // get utxos by hash and index
        debug!("txo by index");
        let txo = txo_by_id_index(dbs, i.0, i.1).await?;
        txins.add(&txo)
    }

    Ok(txins)
}

pub async fn get_tx_reference_inputs(
    dbs: &DBSyncProvider,
    hash: &str,
) -> Result<TransactionUnspentOutputs, DataProviderDBSyncError> {
    let input_references: Vec<_> = tx::table
        .inner_join(reference_tx_in::table.on(reference_tx_in::tx_in_id.eq(tx::id)))
        .filter(tx::hash.eq(hex::decode(hash)?))
        .select((reference_tx_in::tx_out_id, reference_tx_in::tx_out_index))
        .load::<(i64, i16)>(&mut dbs.connect()?)?;
    let mut temp = Vec::<(i64, i16)>::new();

    for i in input_references.into_iter() {
        let tx_out = tx_out::table
            .inner_join(tx::table.on(tx::id.eq(tx_out::tx_id)))
            .filter(tx::id.eq(i.0))
            .filter(tx_out::index.eq(i.1))
            .select((tx_out::id, tx_out::index))
            .first::<(i64, i16)>(&mut dbs.connect()?)?;
        temp.push(tx_out);
    }
    let mut txins = TransactionUnspentOutputs::new();
    for i in temp.into_iter() {
        // get utxos by hash and index
        let txo = txo_by_id_index(dbs, i.0, i.1).await?;
        txins.add(&txo)
    }
    Ok(txins)
}

pub async fn get_tx_outputs(
    dbs: &DBSyncProvider,
    hash: &str,
) -> Result<TransactionUnspentOutputs, DataProviderDBSyncError> {
    let output_references: Vec<_> = tx::table
        .inner_join(tx_out::table.on(tx_out::tx_id.eq(tx::id)))
        .filter(tx::hash.eq(hex::decode(hash)?))
        .select((tx_out::id, tx_out::index))
        .load::<(i64, i16)>(&mut dbs.connect()?)?;

    let mut txout = TransactionUnspentOutputs::new();

    for o in output_references.into_iter() {
        let txo = txo_by_id_index(dbs, o.0, o.1).await?;
        txout.add(&txo)
    }
    Ok(txout)
}

pub async fn get_withdrawls(
    dbs: &DBSyncProvider,
    hash: &str,
) -> Result<Vec<WithdrawalView>, DataProviderDBSyncError> {
    let withdrawals: Option<Vec<_>> = withdrawal::table
        .inner_join(tx::table.on(withdrawal::tx_id.eq(tx::id)))
        .inner_join(stake_address::table.on(withdrawal::addr_id.eq(stake_address::id)))
        .filter(tx::hash.eq(hex::decode(hash)?))
        .select((withdrawal::amount, stake_address::view))
        .load::<(BigDecimal, String)>(&mut dbs.connect()?)
        .ok();

    let mut out = Vec::<WithdrawalView>::new();
    if let Some(withdrawals) = withdrawals {
        for w in withdrawals {
            out.push(WithdrawalView {
                amount: w.0.to_u64().unwrap(),
                stake_address: w.1,
            });
        }
    }

    Ok(out)
}

pub async fn tx_stake_registration(
    dbs: &DBSyncProvider,
    hash: &str,
) -> Result<Vec<StakeRegistrationView>, DataProviderDBSyncError> {
    let stake_registration: Option<Vec<(String, i32, i32)>> = tx::table
        .inner_join(stake_registration::table.on(stake_registration::tx_id.eq(tx::id)))
        .inner_join(stake_address::table.on(stake_registration::addr_id.eq(stake_address::id)))
        .filter(tx::hash.eq(hex::decode(hash)?))
        .select((
            stake_address::view,
            stake_registration::epoch_no,
            stake_registration::cert_index,
        ))
        .load::<(String, i32, i32)>(&mut dbs.connect()?)
        .ok();

    let mut out = Vec::<StakeRegistrationView>::new();
    if let Some(stake_registration) = stake_registration {
        for s in stake_registration.into_iter() {
            out.push(StakeRegistrationView {
                stake_address: s.0,
                tx_hash: hex::decode(hash)?,
                epoch: s.1,
                cert_index: s.2,
            });
        }
    }

    Ok(out)
}

pub async fn tx_stake_deregistration(
    dbs: &DBSyncProvider,
    hash: &str,
) -> Result<Vec<StakeDeregistrationView>, DataProviderDBSyncError> {
    let stake_deregistration: Option<Vec<(String, i32, i32)>> = tx::table
        .inner_join(stake_deregistration::table.on(stake_deregistration::tx_id.eq(tx::id)))
        .inner_join(stake_address::table.on(stake_deregistration::addr_id.eq(stake_address::id)))
        .filter(tx::hash.eq(hex::decode(hash)?))
        .select((
            stake_address::view,
            stake_deregistration::epoch_no,
            stake_deregistration::cert_index,
        ))
        .load::<(String, i32, i32)>(&mut dbs.connect()?)
        .ok();

    let mut out = Vec::<StakeDeregistrationView>::new();
    if let Some(stake_deregistration) = stake_deregistration {
        for s in stake_deregistration.into_iter() {
            out.push(StakeDeregistrationView {
                stake_address: s.0,
                tx_hash: hex::decode(hash)?,
                epoch: s.1,
                cert_index: s.2,
            });
        }
    }

    Ok(out)
}

pub async fn tx_script(
    dbs: &DBSyncProvider,
    hash: &str,
) -> Result<Vec<ScriptView>, DataProviderDBSyncError> {
    let script: Option<
        Vec<(
            Vec<u8>,
            super::models::Scripttype,
            Option<serde_json::Value>,
            Option<Vec<u8>>,
        )>,
    > = tx::table
        .inner_join(script::table.on(script::tx_id.eq(tx::id)))
        .filter(tx::hash.eq(hex::decode(hash)?))
        .select((
            script::hash,
            script::type_,
            script::json.nullable(),
            script::bytes.nullable(),
        ))
        .load(&mut dbs.connect()?)
        .ok();
    let mut out = Vec::<ScriptView>::new();
    if let Some(script) = script {
        for s in script.into_iter() {
            let bytes = s.3.map(hex::encode);

            out.push(ScriptView {
                hash: hex::encode(s.0),
                r#type: s.1,
                json: s.2,
                bytes,
            });
        }
    }

    Ok(out)
}

pub async fn tx_collateral_in(
    dbs: &DBSyncProvider,
    hash: &str,
) -> Result<TransactionUnspentOutputs, DataProviderDBSyncError> {
    let collateral_tx_in_ref: Option<Vec<_>> = tx::table
        .inner_join(collateral_tx_in::table.on(collateral_tx_in::tx_in_id.eq(tx::id)))
        .filter(tx::hash.eq(hex::decode(hash)?))
        .select((collateral_tx_in::tx_out_id, collateral_tx_in::tx_out_index))
        .load::<(i64, i16)>(&mut dbs.connect()?)
        .ok();

    let mut temp = Vec::<(i64, i16)>::new();
    if let Some(collateral_tx_in_ref) = collateral_tx_in_ref {
        for i in collateral_tx_in_ref.into_iter() {
            let tx_out = tx_out::table
                .inner_join(tx::table.on(tx::id.eq(tx_out::tx_id)))
                .filter(tx::id.eq(i.0))
                .filter(tx_out::index.eq(i.1))
                .select((tx_out::id, tx_out::index))
                .first::<(i64, i16)>(&mut dbs.connect()?)
                .ok();
            if let Some(tx_out) = tx_out {
                temp.push(tx_out)
            }
        }
    }
    let mut txins = TransactionUnspentOutputs::new();

    for i in temp.into_iter() {
        let txo = txo_by_id_index(dbs, i.0, i.1).await?;
        txins.add(&txo)
    }
    Ok(txins)
}

pub async fn tx_collateral_out(
    dbs: &DBSyncProvider,
    hash: &str,
) -> Result<TransactionUnspentOutputs, DataProviderDBSyncError> {
    let output_references: Option<Vec<_>> = collateral_tx_out::table
        .inner_join(tx::table.on(collateral_tx_out::tx_id.eq(tx::id)))
        .filter(tx::hash.eq(hex::decode(hash)?))
        .select((collateral_tx_out::id, collateral_tx_out::index))
        .load::<(i64, i16)>(&mut dbs.connect()?)
        .ok();

    let mut txout = TransactionUnspentOutputs::new();
    if let Some(output_references) = output_references {
        for o in output_references.into_iter() {
            let txo = collateral_txo_by_id_index(dbs, o.0, o.1).await?;
            txout.add(&txo)
        }
    }
    Ok(txout)
}

pub fn retrieve_staked_amount(
    dbs: &DBSyncProvider,
    epoch: i32,
    stake_addr: &str,
) -> Result<BigDecimal, DataProviderDBSyncError> {
    Ok(epoch_stake::table
        .inner_join(stake_address::table.on(epoch_stake::addr_id.eq(stake_address::id)))
        .filter(stake_address::view.eq(stake_addr.to_string()))
        .filter(epoch_stake::epoch_no.eq(epoch))
        .select(epoch_stake::amount)
        .first::<BigDecimal>(&mut dbs.connect()?)
        .ok()
        .unwrap()
    )
}

pub fn retrieve_generated_rewards(
    dbs: &DBSyncProvider,
    stake_addr: &str,
) -> Result<Vec<RewardView>, DataProviderDBSyncError> {
    Ok(reward::table
        .inner_join(stake_address::table.on(stake_address::id.eq(reward::addr_id)))
        .filter(stake_address::view.eq(stake_addr.to_string()))
        .select((reward::amount, reward::earned_epoch, reward::spendable_epoch))
        .load::<(BigDecimal, i64, i64)>(&mut dbs.connect()?)
        .ok()
        .unwrap()
        .into_iter()
        .map(|t| {
            RewardView{
                amount: t.0.to_u64().unwrap(),
                earned_epoch: t.1,
                spendable_epoch: t.2,
            }
        }).collect::<Vec<RewardView>>()
    )
}

// R parameter in reward projection
pub fn total_available_rewards(
    dbs: &DBSyncProvider,
    current_epoch: i32,
) -> Result<BigDecimal, DataProviderDBSyncError> {
    Ok(ada_pots::table
        .filter(ada_pots::epoch_no.eq(current_epoch))
        .select(ada_pots::rewards)
        .first::<BigDecimal>(&mut dbs.connect()?)?
    )
}

// a0 parameter in reward projection
pub fn pledge_influence_factor(
    dbs: &DBSyncProvider,
    current_epoch: i32,
) -> Result<BigDecimal, DataProviderDBSyncError> {
    Ok(
        BigDecimal::from_f64(
            epoch_param::table
                .filter(epoch_param::epoch_no.eq(current_epoch))
                .select(epoch_param::influence)
                .first::<f64>(&mut dbs.connect()?)?
        ).unwrap()
    )
}

// z0 parameter in reward projection
pub fn relative_pool_saturation_size(
    dbs: &DBSyncProvider,
    current_epoch: i32,
) -> Result<BigDecimal, DataProviderDBSyncError>{
    // saturation level (optimal number of pools)
    let k = BigDecimal::from(
        epoch_param::table
            .filter(epoch_param::epoch_no.eq(current_epoch))
            .select(epoch_param::optimal_pool_count)
            .first::<i32>(&mut dbs.connect()?)?
    );

    let one = BigDecimal::from(1);
    let z0 = one/k;

    Ok(z0)
}

// s parameter in reward projection
pub fn stake_pledged_by_owner(
    dbs: &DBSyncProvider,
    pool_addr: &str,
    current_epoch: i32,
) -> Result<BigDecimal, DataProviderDBSyncError>{
    let owners_stake = pool_update::table
        .inner_join(pool_hash::table.on(pool_hash::id.eq(pool_update::hash_id)))
        .filter(pool_hash::view.eq(pool_addr.clone()))
        .filter(pool_update::active_epoch_no.le(current_epoch.clone() as i64))
        .select(pool_update::pledge)
        .load::<BigDecimal>(&mut dbs.connect()?)?;
    let owners_stake = owners_stake
        .last()
        .unwrap(); 
    

    let total_stake = BigDecimal::from(
        pool_total_stake(dbs, pool_addr, current_epoch)?
    ); 

    Ok(owners_stake/total_stake)
}

// σ parameter in reward projection
pub fn stake_delegated_to_pool(
    dbs: &DBSyncProvider,
    pool_addr: &str,
    epoch: i32,
) -> Result<BigDecimal, DataProviderDBSyncError>{
    let stake_of_pool = pool_total_stake(dbs, pool_addr, epoch)?; // 0.678 of current value 
    let total_stake_circulating = ada_pots::table
        .filter(ada_pots::epoch_no.eq(epoch))
        .select(ada_pots::treasury) // 1.475 of the value of ada_pots::treasury is correct
        .first::<BigDecimal>(&mut dbs.connect()?)?; 

    Ok(stake_of_pool/total_stake_circulating) // 0.678 of the value is correct
}

// input for "retrieve_rewards_next_epoch" function
pub fn reward_projection_parameters(
    dbs: &DBSyncProvider,
    current_epoch: i32, // latest transaction
    pool_addr: &str,
) -> Result<RewardProjectionParameters, DataProviderDBSyncError>{
    let r = total_available_rewards(dbs, current_epoch)?; // correct
    let a0 = pledge_influence_factor(dbs, current_epoch)?; // correct
    let z0 = relative_pool_saturation_size(dbs, current_epoch)?; // correct
    let sigma = stake_delegated_to_pool(dbs, pool_addr, current_epoch)?;
    let s = stake_pledged_by_owner(dbs, pool_addr, current_epoch)?;
    
    let sigma_ = if sigma < z0 {sigma.clone()} else {z0.clone()}; // min(sigma, z0)
    let s_ = if s < z0 {s.clone()} else {z0.clone()}; // min(s, z0)

    let one = BigDecimal::from(1); // correct

    dbg!(r.clone());
    dbg!(one.clone());
    dbg!(a0.clone());
    dbg!(sigma.clone());
    dbg!(sigma_.clone());
    dbg!(s.clone());
    dbg!(s_.clone());
    dbg!(z0.clone());

    Ok(RewardProjectionParameters{
        r, one, a0, sigma_, s_, z0
    })
}

// // (R, one, a0, sigma_, s_, z0)
// type RewardProjectionParameters = (BigDecimal, BigDecimal, BigDecimal, BigDecimal, BigDecimal, BigDecimal);
pub struct RewardProjectionParameters {
    r: BigDecimal,
    one: BigDecimal,
    a0: BigDecimal,
    sigma_: BigDecimal,
    s_: BigDecimal,
    z0: BigDecimal,
}

// projected rewards for the given pool
// source: https://docs.cardano.org/learn/pledging-rewards/
pub fn calculate_pool_rewards_next_epoch(
    p: RewardProjectionParameters
) -> Result<BigDecimal, DataProviderDBSyncError> {
    // reward formula: https://docs.cardano.org/learn/pledging-rewards/
    let rewards = ( p.r / ( p.one + p.a0.clone() ) )*(
        p.sigma_.clone()
        +
        ( p.s_.clone() * p.a0 / p.z0.clone() )
        *
        ( 
            p.sigma_.clone() 
            - 
            ( p.s_ / p.z0.clone() ) * ( p.z0 - p.sigma_ )
        )
    );

    Ok(rewards)
}

pub fn pool_owner_margin(
    dbs: &DBSyncProvider,
    pool_hash: &str,
    current_epoch: i64,
) -> Result<BigDecimal, DataProviderDBSyncError> {
    let mut margin = pool_update::table
        .inner_join(pool_hash::table.on(pool_hash::id.eq(pool_update::hash_id)))
        .filter(pool_update::active_epoch_no.le(current_epoch))
        .filter(pool_hash::view.eq(pool_hash.to_string()))
        .select(pool_update::margin)
        .load::<f64>(&mut dbs.connect()?)?;
    margin.reverse();
    let margin = BigDecimal::from_f64(
        margin.last().unwrap().clone()
    ).unwrap();

    Ok(margin)
}

pub fn personal_stake(
    dbs: &DBSyncProvider,
    stake_addr: &str,
    epoch: Option<i32>,
) -> Result<BigDecimal, DataProviderDBSyncError> {
    if let Some(epoch) = epoch {
        Ok(epoch_stake::table
            .inner_join(stake_address::table.on(stake_address::id.eq(epoch_stake::addr_id)))
            .filter(stake_address::view.eq(stake_addr))
            .filter(epoch_stake::epoch_no.eq(epoch))
            .select(epoch_stake::amount)
            .first::<BigDecimal>(&mut dbs.connect()?)?
        )
    } else {
        Ok(epoch_stake::table
            .inner_join(stake_address::table.on(stake_address::id.eq(epoch_stake::addr_id)))
            .filter(stake_address::view.eq(stake_addr))
            .select(epoch_stake::amount)
            .load::<BigDecimal>(&mut dbs.connect()?)?
            .iter()
            .last()
            .unwrap()
            .clone()
        )
    }
}

// Accurate for small epoch values, but inaccurate for large epoch values.
// Correction mechanism needs to be investigated and introduced to 
// make projection accurate for higher epoch values. Correction mechanism either
// in this function or any of its child functions. 
pub fn personal_delegator_rewards_next_epoch(
    dbs: &DBSyncProvider,
    pool_hash: &str,
    current_epoch: i32,
    stake_addr: &str,
) -> Result<BigDecimal, DataProviderDBSyncError>{
    let owner_margin = pool_owner_margin(dbs, pool_hash, current_epoch as i64)?; // correct
    let parameters = reward_projection_parameters(
        dbs, 
        current_epoch, 
        pool_hash,
    )?; // plausible, maybe wrong
    let total_pool_reward = calculate_pool_rewards_next_epoch(parameters)?; // correct
    let total_delegator_reward = total_pool_reward*(BigDecimal::from(1) - owner_margin); // correct

    let personal_stake = personal_stake(dbs, stake_addr, Some(current_epoch))?; // correct

    let total_pool_stake = BigDecimal::from(
        pool_total_stake(
            dbs, pool_hash, current_epoch
        )?
    ); // correct

    let personal_percentage = personal_stake/total_pool_stake; // seems right

    Ok(
        total_delegator_reward*personal_percentage
    )
}

pub fn earned_reward(
    dbs: &DBSyncProvider,
    stake_addr: &str,
    earned_epoch: i64,
) -> Result<BigDecimal, DataProviderDBSyncError>{
    Ok(reward::table
        .inner_join(stake_address::table.on(reward::addr_id.eq(stake_address::id)))
        .filter(stake_address::view.eq(stake_addr.to_string()))
        .filter(reward::earned_epoch.eq(earned_epoch))
        .select(reward::amount)
        .first::<BigDecimal>(&mut dbs.connect()?)?
    )
}

pub async fn discover_transaction(
    dbs: &DBSyncProvider,
    hash: &str,
) -> Result<TransactionView, DataProviderDBSyncError> {
    let inputs = get_tx_inputs(dbs, hash).await?;
    log::debug!("Inputs: \n{inputs:?}\n");
    let reference_inputs = get_tx_reference_inputs(dbs, hash).await?;
    log::debug!("Reference Inputs: \n{reference_inputs:?}\n");
    let outputs = get_tx_outputs(dbs, hash).await?;
    log::debug!("Outputs: \n{outputs:?}\n");
    let withdrawals = get_withdrawls(dbs, hash).await?;
    log::debug!("Withdrawal: \n{withdrawals:?}\n");
    let metadata = tx_metadata(dbs, hash).await?;
    log::debug!("Metadata: \n{metadata:?}\n");
    let stake_registration = tx_stake_registration(dbs, hash).await?;
    log::debug!("Stake Registration: \n{stake_registration:?}\n");
    let stake_deregistration = tx_stake_deregistration(dbs, hash).await?;
    log::debug!("Stake Deregistration: \n {stake_deregistration:?}\n");
    let script = tx_script(dbs, hash).await?;
    log::debug!("Script: {script:?}");
    let collateral_tx_in = tx_collateral_in(dbs, hash).await?;
    log::debug!("Collateral Tx In: {collateral_tx_in:?}");
    let collateral_tx_out = tx_collateral_out(dbs, hash).await?;
    log::debug!("Collateral Tx Out: {collateral_tx_out:?}");
    let txinfo = tx::table
        .inner_join(block::table.on(block::id.eq(tx::block_id)))
        .filter(tx::hash.eq(hex::decode(hash)?))
        .select((
            tx::fee,
            block::hash,
            block::epoch_no.nullable(),
            block::slot_no.nullable(),
        ))
        .first::<(BigDecimal, Vec<u8>, Option<i32>, Option<i64>)>(&mut dbs.connect()?)?;
    log::debug!("TxInfo: {txinfo:?}");

    Ok(TransactionView {
        hash: hash.to_string(),
        block: hex::encode(txinfo.1),
        slot: txinfo.3,
        inputs: inputs
            .into_iter()
            .fold(Vec::<UTxOView>::new(), |mut acc, n| {
                acc.push(UTxOView::from_txuo(&n));
                acc
            }),
        reference_inputs: if reference_inputs.is_empty() {
            None
        } else {
            Some(
                reference_inputs
                    .into_iter()
                    .fold(Vec::<UTxOView>::new(), |mut acc, n| {
                        acc.push(UTxOView::from_txuo(&n));
                        acc
                    }),
            )
        },
        outputs: outputs
            .into_iter()
            .fold(Vec::<UTxOView>::new(), |mut acc, n| {
                acc.push(UTxOView::from_txuo(&n));
                acc
            }),
        withdrawals: if withdrawals.is_empty() {
            None
        } else {
            Some(withdrawals)
        },
        metadata,
        stake_registration: if stake_registration.is_empty() {
            None
        } else {
            Some(stake_registration)
        },
        stake_deregistration: if stake_deregistration.is_empty() {
            None
        } else {
            Some(stake_deregistration)
        },
        script: if script.is_empty() {
            None
        } else {
            Some(script)
        },
        collateral_tx_in: if collateral_tx_in.is_empty() {
            None
        } else {
            Some(
                collateral_tx_in
                    .into_iter()
                    .fold(Vec::<UTxOView>::new(), |mut acc, n| {
                        acc.push(UTxOView::from_txuo(&n));
                        acc
                    }),
            )
        },
        collateral_tx_out: if collateral_tx_out.is_empty() {
            None
        } else {
            Some(
                collateral_tx_out
                    .into_iter()
                    .fold(Vec::<UTxOView>::new(), |mut acc, n| {
                        acc.push(UTxOView::from_txuo(&n));
                        acc
                    }),
            )
        },
        fee: txinfo.0.to_u64().unwrap(),
        cbor: None,
    })
}

pub async fn epoch_nonce(
    dbs: &DBSyncProvider,
    epoch: i32,
) -> Result<(Vec<u8>, Option<Vec<u8>>), DataProviderDBSyncError> {
    let out = epoch_param::table
        .filter(epoch_param::epoch_no.eq(epoch))
        .select((epoch_param::nonce, epoch_param::extra_entropy.nullable()))
        .first::<(Vec<u8>, Option<Vec<u8>>)>(&mut dbs.connect()?)?;
    Ok(out)
}

//ToDo:
//temporary declared here, common type library needs to be created
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct EpochChangeResponse {
    pub last_epoch: u64,
    pub last_blockhash: String,
    pub last_slot: u64,
    pub new_epoch: u64,
    pub new_slot: u64,
    pub new_blockhash: String,
    pub epoch_nonce: String,
    pub extra_entropy: Option<String>,
}

pub async fn epoch_change(
    dbs: &DBSyncProvider,
    epoch: Option<i32>,
) -> Result<EpochChangeResponse, DataProviderDBSyncError> {
    // select b.id, b.hash, b.slot_no, b.block_no, b.previous_id, ep.nonce, ep.extra_entropy from block b
    // left join epoch_param ep on ep.epoch_no  = b.epoch_no
    // where b.block_no = (select min(block_no-1) from block b where b.epoch_no = 209) or b.block_no = (select min(block_no) from block b where b.epoch_no = 209)
    // order by block_no DESC;

    let (b, b1, ep) = diesel::alias!(block as b, block as b1, epoch_param as ep);

    let min_block = b1
        .filter(b1.field(block::epoch_no).nullable().eq(epoch))
        .select(diesel::dsl::min(b1.field(block::block_no)))
        .single_value();

    let out = b
        .inner_join(
            ep.on(ep
                .field(epoch_param::epoch_no)
                .nullable()
                .eq(b.field(block::epoch_no))),
        )
        .filter(
            b.field(block::block_no)
                .eq(min_block - 1)
                .or(b.field(block::block_no).eq(min_block)),
        )
        .select((
            b.field(block::id),
            b.field(block::hash),
            b.field(block::slot_no).nullable(),
            b.field(block::block_no).nullable(),
            b.field(block::previous_id).nullable(),
            b.field(block::epoch_no).nullable(),
            ep.field(epoch_param::nonce),
            ep.field(epoch_param::extra_entropy).nullable(),
        ))
        .order_by(b.field(block::epoch_no).desc())
        .load::<(
            i64,
            Vec<u8>,
            Option<i64>,
            Option<i32>,
            Option<i64>,
            Option<i32>,
            Vec<u8>,
            Option<Vec<u8>>,
        )>(&mut dbs.connect()?)?;
    println!("out: {out:?}");
    let out: Vec<(
        i64,
        String,
        Option<i64>,
        Option<i32>,
        Option<i64>,
        Option<i32>,
        String,
        Option<String>,
    )> = out
        .into_iter()
        .map(|n| {
            (
                n.0,
                hex::encode(n.1),
                n.2,
                n.3,
                n.4,
                n.5,
                hex::encode(n.6),
                n.7.map(hex::encode),
            )
        })
        .collect();

    let out = EpochChangeResponse {
        last_epoch: out[1].5.unwrap() as u64,
        last_blockhash: out[1].1.clone(),
        last_slot: out[1].2.unwrap() as u64,
        new_epoch: out[0].5.unwrap() as u64,
        new_slot: out[0].2.unwrap() as u64,
        new_blockhash: out[0].1.clone(),
        epoch_nonce: out[0].6.clone(),
        extra_entropy: out[0].7.clone(),
    };

    Ok(out)
}

#[cfg(test)]
mod tests {
    use crate::{provider::CardanoDataProvider, dbsync::DataProviderDBSyncError};
    use bigdecimal::BigDecimal;
    use itertools::Itertools;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_tx_history() {
        let r = vec!["addr_test1qqt86eq9972q3qttj6ztje97llasktzfzvhmdccqjlqjaq2cer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qy6q5t2","addr_test1qpg8ehvgj9zxrx59et72yjn2p02xwsm3l89jwj8ujcj63ujcer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qw23emu","addr_test1qqdp3cry5vc2gfjljctdu638tvkcqfx40fjunht9hrmru5zcer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qnaxxgs","addr_test1qr2mw080ujz0unmpn9lx5ftfuewc6htyr6v3a0svul2zgezcer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qgryf7t","addr_test1qr7tqh7tsg4lut3jv6tsfwlv464m6knjjw90ugyz8uzgr6zcer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qt0jxzj","addr_test1qrscurjp292sxv24sepj7ghq4ydkkekzaz53zwfswcna6ljcer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6q8pu3l5","addr_test1qqssrphse6qmp9h0ksu5vfmsx99tfl2lc6rhvy2spd5wr86cer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qw59j4j","addr_test1qqgagc0fy6nm0qe4h8zqxsg952tqjeg7l7j0agd0cx4u25zcer3t74yn0dm8xqnr7rtwhkqcrpsmphwcf0mlmn39ry6qxvept2"];

        let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
            db_path: dotenv::var("DBSYNC_DB_URL").unwrap(),
        }));
        let t = dp.tx_history(&r, None).await.unwrap();
        println!("{t:?}");
    }

    #[tokio::test]
    async fn test_discover_transaction() {
        let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
            db_path: dotenv::var("DBSYNC_DB_URL").unwrap(),
        }));
        let t = crate::dbsync::discover_transaction(
            dp.provider(),
            "1b07f1152e52ce0a9dbb561aa2e2d1750ca3a1a4141150a8bad342947a66a3a6",
        )
        .await;
        println!("{t:?}");
    }

    #[tokio::test]
    async fn test_get_pools() {
        let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
            db_path: dotenv::var("DBSYNC_DB_URL").unwrap(),
        }));
        let t = crate::dbsync::get_pools(dp.provider()).await;
        println!("{t:?}");
    }

    #[tokio::test]
    async fn test_epoch_nonce() {
        let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
            db_path: dotenv::var("DBSYNC_DB_URL").unwrap(),
        }));
        let t = crate::dbsync::epoch_nonce(dp.provider(), 205)
            .await
            .unwrap();
        println!("Nonce: {}\nEntropy: {:?}", hex::encode(&t.0), t.1);
    }

    #[tokio::test]
    async fn test_is_nft() {
        let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
            db_path: dotenv::var("DBSYNC_DB_URL").unwrap(),
        }));
        let t = crate::dbsync::is_nft(
            dp.provider(),
            &[
                "asset1a0q0grruzd3dm2c9ev890zfaytty8tfcl4qt3a",
                "asset1h3pg9m9arlwl4l8z3dwg3lwg54j70zqdrjhy88",
                "asset1fqdnvjcwjcck8t34rvjyj8ccdradp5hkzycxpq",
                "asset1e83uya776dvqjauy270qnj03899hxxant6jp2g",
            ],
        )
        .await
        .unwrap();
        assert_eq!(t, vec![true, true, true, false]);
    }

    #[tokio::test]
    async fn test_supply() {
        let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
            db_path: dotenv::var("DBSYNC_DB_URL").unwrap(),
        }));
        let t = crate::dbsync::token_supply(
            dp.provider(),
            "asset1m099azmatp3f3xehsu4sqvr45jzqafxmm0dra0",
        )
        .await
        .unwrap();
        assert_eq!(t, Some(bigdecimal::BigDecimal::from(500000000u64)));
    }

    // Accurate for small epoch values, but inaccurate for large epoch values.
    // Correction mechanism needs to be investigated and introduced to 
    // make projection accurate for higher epoch values.
    #[tokio::test]
    async fn reward_projection() {
        let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
            db_path: dotenv::var("DBSYNC_DB_URL").unwrap(),
        }));

        let pool_hash = "pool1ayc7a29ray6yv4hn7ge72hpjafg9vvpmtscnq9v8r0zh7azas9c";
        let stake_addr = "stake_test1upvv3c4l2jfhkannqf3lp4htmqvpscdsmhvyhalaecj3jdqtfcgvh"; // stake_test1upvv3c4l2jfhkannqf3lp4htmqvpscdsmhvyhalaecj3jdqtfcgvh
        let current_epoch = 25;

        let func_value = super::personal_delegator_rewards_next_epoch(
            dp.provider(), pool_hash, current_epoch, stake_addr,
        ).unwrap();
        let real_value = super::earned_reward(
            dp.provider(), stake_addr, current_epoch as i64,
        ).unwrap();

        println!("correct_reward:   {}", real_value);
        println!("projected reward: {}", func_value);
        // no assertion due to  volatile difference
    }

    #[tokio::test]
    async fn retrieve_staked_amount() {
        let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
            db_path: dotenv::var("DBSYNC_DB_URL").unwrap(),
        }));
        let epoch = 275;
        let stake_addr = "stake_test1upvv3c4l2jfhkannqf3lp4htmqvpscdsmhvyhalaecj3jdqtfcgvh";

        let func_value = super::retrieve_staked_amount(dp.provider(), epoch, stake_addr)
            .unwrap();
        let real_value = BigDecimal::from_str("10305915710")
        .unwrap();

        assert_eq!(func_value, real_value);
    }

    #[tokio::test]
    async fn mint_metadata() {
        let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
            db_path: dotenv::var("DBSYNC_DB_URL").unwrap()
        }));
        let fingerprint_in = "asset1kngmwlxpfzc6pk027zvhsfpprp452gt3enhhxh";
        let func_value = super::mint_metadata(dp.provider(), fingerprint_in).unwrap();
        let real_value = super::TokenInfoView {
            fingerprint: "asset1kngmwlxpfzc6pk027zvhsfpprp452gt3enhhxh".to_string(),
            policy: "994cf4c18f5613ca49c275f63d464b6d95123bfa8985e82b24b5680b".to_string(),
            tokenname: "MyAmazingNFT".bytes().collect_vec(),
            quantity: None,
            meta_key: Some(721),
            json: Some(
                serde_json::Value::from_str(
                    "{\"994cf4c18f5613ca49c275f63d464b6d95123bfa8985e82b24b5680b\": {\"MyAmazingNFT\": {\"name\": \"NFT FTW: MyAmazingNFT\", \"image\": \"ipfs://XXXXYYYYZZZZ\"}}}"
                ).unwrap()
            ),
            mint_slot: Some(1888394),
            txhash: Some("9d276f5c9c4a785c349fa1daaaae4ab86b1c141ac547f55c2f8c8a0432b2ed04".to_string()),
        };
        assert_eq!(func_value.fingerprint, real_value.fingerprint);
        assert_eq!(func_value.policy, real_value.policy);
        assert_eq!(func_value.tokenname, real_value.tokenname);
        assert_eq!(func_value.quantity, real_value.quantity);
        assert_eq!(func_value.meta_key, real_value.meta_key);
        assert_eq!(func_value.json, real_value.json);
        assert_eq!(func_value.mint_slot, real_value.mint_slot);
        assert_eq!(func_value.txhash, real_value.txhash);
    }

    #[tokio::test]
    #[allow(non_snake_case)]
    async fn mint_metadata_bug_CMW_78() {
        let dp = crate::DataProvider::new(crate::DBSyncProvider::new(crate::Config {
            db_path: dotenv::var("DBSYNC_DB_URL").unwrap()
        }));
        let fingerprint_in = "asset162kdtwq54e5khz5y6naa55xqvk0zk5fpce8c76"; // contains non-UTF8 characters
        let func_value = super::mint_metadata(dp.provider(), fingerprint_in).unwrap();
        let real_value = super::TokenInfoView {
            fingerprint: "asset162kdtwq54e5khz5y6naa55xqvk0zk5fpce8c76".to_string(),
            policy: "8cbe56131657c928cee716677bd3eac885f9fcad10f9fa70e533f635".to_string(),
            tokenname: vec![0, 6, 64, 160, 100, 100],
            quantity: Some(1),
            meta_key: None,
            json: None,
            mint_slot: Some(829338),
            txhash: Some("1727810423ca5719a366af35058b7164d7fee44c8c1ca6e6ee6ff9b35490bf63".to_string()),
        };
        assert_eq!(func_value.fingerprint, real_value.fingerprint);
        assert_eq!(func_value.policy, real_value.policy);
        assert_eq!(func_value.tokenname, real_value.tokenname);
        assert_eq!(func_value.quantity, real_value.quantity);
        assert_eq!(func_value.meta_key, real_value.meta_key);
        assert_eq!(func_value.json, real_value.json);
        assert_eq!(func_value.mint_slot, real_value.mint_slot);
        assert_eq!(func_value.txhash, real_value.txhash);
    }
}
