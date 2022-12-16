use super::error::DataProviderDBSyncError;
use super::models::{PoolHash, PoolRetire, UnspentUtxo, UtxoView};
use super::schema::*;
use crate::models::{
    CardanoNativeAssetView, DelegationView, HoldingWalletView, StakeDelegationView,
    StakeDeregistrationView, StakeRegistrationView, TokenInfoView,
};
use crate::DBSyncProvider;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use diesel::prelude::*;
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
    log::debug!(
        "Try to find first address used by this stake address: {}",
        stake_address_in
    );
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
    log::debug!("Found address: {:?}", resp);
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
) -> Result<Vec<crate::models::CDPDatum>, DataProviderDBSyncError> {
    let mut datums = datum::table
        .inner_join(
            tx_out::table.on(tx_out::data_hash
                .eq(datum::hash.nullable())
                .or(tx_out::inline_datum_id.eq(datum::id.nullable()))),
        )
        .inner_join(tx::table.on(tx::id.eq(tx_out::tx_id)))
        .filter(tx::hash.eq(txid))
        .select((datum::hash, datum::value.nullable(), datum::bytes))
        .load::<crate::models::CDPDatum>(&mut dbs.connect()?);

    if let Ok(o) = datums {
        if !o.is_empty() {
            return Ok(o);
        }
    }

    let r: (i64, i32) = redeemer::table
        .inner_join(tx::table.on(tx::id.eq(redeemer::tx_id)))
        .filter(tx::hash.eq(txid))
        .select((redeemer::tx_id, redeemer::index))
        .first::<(i64, i32)>(&mut dbs.connect()?)?;

    let t: (Option<Vec<u8>>, Option<i64>) = tx_out::table
        .inner_join(tx_in::table.on(tx_in::tx_out_id.eq(tx_out::tx_id)))
        .filter(tx_in::tx_in_id.eq(r.0))
        .filter(tx_in::tx_out_index.eq(r.1 as i16))
        .filter(tx_out::data_hash.is_not_null())
        .select((
            tx_out::data_hash.nullable(),
            tx_out::inline_datum_id.nullable(),
        ))
        .first::<(Option<Vec<u8>>, Option<i64>)>(&mut dbs.connect()?)?;

    datums = datum::table
        .filter(diesel::BoolExpressionMethods::or(
            datum::hash.eq(t.0.unwrap()),
            datum::id.eq(t.1.unwrap()),
        ))
        .select((datum::hash, datum::value.nullable(), datum::bytes))
        .load::<crate::models::CDPDatum>(&mut dbs.connect()?);

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
    let tokenname = hex::encode(fingerprint.1);

    let ti = TokenInfoView {
        policy,
        tokenname,
        fingerprint: fingerprint_in.to_owned(),
        quantity: None,
        meta_key: None,
        json: None,
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
    let pbyte = hex::decode(&policy)?;

    let mut holders = unspent_utxos::table
        .inner_join(ma_tx_out::table.on(unspent_utxos::id.eq(ma_tx_out::tx_out_id)))
        .left_join(multi_asset::table.on(multi_asset::id.eq(ma_tx_out::ident)))
        .filter(multi_asset::policy.eq(pbyte))
        .filter(unspent_utxos::stake_address.is_not_null())
        .filter(ma_tx_out::quantity.eq(BigDecimal::from(1)))
        .select((unspent_utxos::stake_address.nullable(), ma_tx_out::quantity))
        .load::<(Option<String>, BigDecimal)>(&mut dbs.connect()?)?;

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
        ))
        .first::<(
            String,
            Vec<u8>,
            Vec<u8>,
            BigDecimal,
            Option<serde_json::Value>,
            Vec<u8>,
        )>(&mut dbs.connect()?)?;

    Ok(TokenInfoView {
        fingerprint: metadata.0,
        policy: hex::encode(metadata.1),
        tokenname: String::from_utf8(metadata.2)?,
        meta_key: Some(metadata.3.to_i64().unwrap()),
        json: metadata.4,
        txhash: Some(hex::encode(metadata.5)),
        quantity: None,
    })
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
