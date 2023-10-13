#![allow(unused)]
#![allow(clippy::all)]
use crate::DBSyncProvider;

use super::error::DataProviderDBSyncError;
use super::schema::*;
use bigdecimal::{BigDecimal, ToPrimitive};
use cardano_serialization_lib::crypto::DataHash;
use cardano_serialization_lib::plutus::PlutusScript;
use cardano_serialization_lib::{NativeScript, ScriptRef};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use diesel::sql_types::{Array, BigInt, Float8, Int4, Jsonb, Numeric};
use diesel_derive_enum::DbEnum;
use super::api;

#[derive(Debug, Clone, DbEnum, QueryId)]
#[ExistingTypePath = "crate::dbsync::schema::sql_types::Syncstatetype"]
pub enum Syncstatetype {
    #[db_rename = "lagging"]
    Lagging,
    #[db_rename = "following"]
    Following,
}
#[derive(Debug, Clone, DbEnum, QueryId, SqlType)]
#[ExistingTypePath = "crate::dbsync::schema::sql_types::Scriptpurposetype"]
pub enum Scriptpurposetype {
    #[db_rename = "spend"]
    Spend,
    #[db_rename = "mint"]
    Mint,
    #[db_rename = "cert"]
    Cert,
    #[db_rename = "reward"]
    Reward,
}

/// Represents SQL enum
#[derive(Clone, Copy, Debug, DbEnum, PartialEq, Eq)] // Debug, Clone, QueryId
#[ExistingTypePath = "crate::dbsync::schema::sql_types::RewardType"]
pub enum RewardType {
    #[db_rename = "leader"]
    Leader,
    #[db_rename = "member"]
    Member,
    #[db_rename = "reserves"]
    Reserves,
    #[db_rename = "treasury"]
    Treasury,
    #[db_rename = "refund"]
    Refund,
}

#[derive(Debug, Clone, DbEnum, QueryId, serde::Serialize, serde::Deserialize)]
#[ExistingTypePath = "crate::dbsync::schema::sql_types::Scripttype"]
pub enum Scripttype {
    #[db_rename = "multisig"]
    Multisig,
    #[db_rename = "timelock"]
    Tiemlock,
    #[db_rename = "plutusV1"]
    PlutusV1,
    #[db_rename = "plutusV2"]
    PlutusV2,
}

#[deprecated(since = "0.1.1")]
#[derive(Queryable, Debug)]
pub struct UnspentUtxo {
    pub id: i64,
    pub tx_id: i64,
    pub hash: Vec<u8>,
    pub index: i16,
    pub address: String,
    pub value: BigDecimal,
    pub data_hash: Option<Vec<u8>>,
    pub address_has_script: bool,
    pub stake_address: Option<String>,
}

#[deprecated(since = "0.1.1")]
impl UnspentUtxo {
    pub fn to_txuo(
        &self,
        dbs: &DBSyncProvider,
    ) -> Result<dcslc::TransactionUnspentOutput, DataProviderDBSyncError> {
        let input = cardano_serialization_lib::TransactionInput::new(
            &cardano_serialization_lib::crypto::TransactionHash::from_bytes(self.hash.clone())?,
            self.index as u32,
        );
        let address = dcslc::addr_from_str(&self.address)?;
        let coin = match self.value.to_u64() {
            Some(c) => c,
            None => {
                return Err(DataProviderDBSyncError::Custom(
                    format!(
                        "Could not determinte Lovelace on Utxo:  {:?}#{:?}",
                        input, self.index
                    )
                    .to_string(),
                ))
            }
        };

        let mut amount = cardano_serialization_lib::utils::Value::new(
            &cardano_serialization_lib::utils::to_bignum(coin),
        );

        let tokens = api::get_utxo_tokens_dep(&dbs, self.id)?;
        let mut ma = cardano_serialization_lib::MultiAsset::new();
        for tok in tokens {
            match ma.get(&cardano_serialization_lib::PolicyID::from_bytes(
                tok.policy.clone(),
            )?) {
                Some(mut asset) => {
                    asset.insert(
                        &cardano_serialization_lib::AssetName::new(tok.name)?,
                        &cardano_serialization_lib::utils::to_bignum(
                            tok.quantity.to_u64().unwrap(),
                        ),
                    );
                    ma.insert(
                        &cardano_serialization_lib::PolicyID::from_bytes(tok.policy)?,
                        &asset,
                    );
                }
                None => {
                    let mut a = cardano_serialization_lib::Assets::new();
                    a.insert(
                        &cardano_serialization_lib::AssetName::new(tok.name)?,
                        &cardano_serialization_lib::utils::to_bignum(
                            tok.quantity.to_u64().unwrap(),
                        ),
                    );
                    ma.insert(
                        &cardano_serialization_lib::PolicyID::from_bytes(tok.policy)?,
                        &a,
                    );
                }
            }
        }
        if ma.len() > 0 {
            amount.set_multiasset(&ma)
        }

        let output = cardano_serialization_lib::TransactionOutput::new(&address, &amount);

        Ok(cardano_serialization_lib::utils::TransactionUnspentOutput::new(&input, &output))
    }
}

#[derive(Queryable, Debug)]
pub struct UtxoView {
    pub id: i64,
    pub tx_id: i64,
    pub index: i16,
    pub address: String,
    pub address_raw: Vec<u8>,
    pub address_has_script: bool,
    pub payment_cred: Vec<u8>,
    pub stake_address_id: Option<i64>,
    pub value: BigDecimal,
    pub data_hash: Option<Vec<u8>>,
    pub inline_datum_id: Option<i64>,
    pub reference_script_id: Option<i64>,
}

impl UtxoView {
    pub fn to_txuo(
        &self,
        dbs: &DBSyncProvider,
    ) -> Result<dcslc::TransactionUnspentOutput, DataProviderDBSyncError> {
        use super::schema::*;

        let tx = tx::table
            .filter(tx::id.eq(&self.tx_id))
            .first::<Tx>(&mut dbs.connect()?)?;

        let input = cardano_serialization_lib::TransactionInput::new(
            &cardano_serialization_lib::crypto::TransactionHash::from_bytes(tx.hash.clone())?,
            self.index as u32,
        );
        let address = dcslc::addr_from_str(&self.address)?;
        let coin = match self.value.to_u64() {
            Some(c) => c,
            None => {
                return Err(DataProviderDBSyncError::Custom(
                    format!(
                        "Could not determinte Lovelace on Utxo:  {:?}#{:?}",
                        input, self.index
                    )
                    .to_string(),
                ))
            }
        };

        let mut amount = cardano_serialization_lib::utils::Value::new(
            &cardano_serialization_lib::utils::to_bignum(coin),
        );

        let tokens = api::get_utxo_tokens(&dbs, self.tx_id, self.index)?;
        let mut ma = cardano_serialization_lib::MultiAsset::new();
        for tok in tokens {
            match ma.get(&cardano_serialization_lib::PolicyID::from_bytes(
                tok.policy.clone(),
            )?) {
                Some(mut asset) => {
                    asset.insert(
                        &cardano_serialization_lib::AssetName::new(tok.name)?,
                        &cardano_serialization_lib::utils::to_bignum(
                            tok.quantity.to_u64().unwrap(),
                        ),
                    );
                    ma.insert(
                        &cardano_serialization_lib::PolicyID::from_bytes(tok.policy)?,
                        &asset,
                    );
                }
                None => {
                    let mut a = cardano_serialization_lib::Assets::new();
                    a.insert(
                        &cardano_serialization_lib::AssetName::new(tok.name)?,
                        &cardano_serialization_lib::utils::to_bignum(
                            tok.quantity.to_u64().unwrap(),
                        ),
                    );
                    ma.insert(
                        &cardano_serialization_lib::PolicyID::from_bytes(tok.policy)?,
                        &a,
                    );
                }
            }
        }
        if ma.len() > 0 {
            amount.set_multiasset(&ma)
        }

        let mut output = cardano_serialization_lib::TransactionOutput::new(&address, &amount);

        if let Some(hash) = &self.data_hash {
            output.set_data_hash(&(DataHash::from_bytes(hash.clone())?));
            if let Some(_) = self.inline_datum_id {
                let datum = datum::table
                    .filter(datum::hash.eq(hash))
                    .first::<Datum>(&mut dbs.connect()?)?;
                let pdatum =
                    cardano_serialization_lib::plutus::PlutusData::from_bytes(datum.bytes)?;

                output.set_plutus_data(&pdatum);
            }
        }

        if let Some(id) = self.reference_script_id {
            let script = script::table
                .filter(script::id.eq(id))
                .first::<Script>(&mut dbs.connect()?)?;

            let scr = match script.type_ {
                Scripttype::Multisig | Scripttype::Tiemlock => ScriptRef::new_native_script(
                    &NativeScript::from_json(&script.json.unwrap().to_string())?,
                ),
                Scripttype::PlutusV1 => {
                    ScriptRef::new_plutus_script(&PlutusScript::from_bytes(script.bytes.unwrap())?)
                }
                Scripttype::PlutusV2 => ScriptRef::new_plutus_script(&PlutusScript::from_bytes_v2(
                    script.bytes.unwrap(),
                )?),
            };
            output.set_script_ref(&scr)
        }

        Ok(cardano_serialization_lib::utils::TransactionUnspentOutput::new(&input, &output))
    }
}

#[derive(Queryable, Debug)]
pub struct AdaPot {
    pub id: i64,
    pub slot_no: i32,
    pub epoch_no: i32,
    pub treasury: BigDecimal,
    pub reserves: BigDecimal,
    pub rewards: BigDecimal,
    pub utxo: BigDecimal,
    pub deposits: BigDecimal,
    pub fees: BigDecimal,
    pub block_id: i64,
}

#[derive(Queryable, Debug)]
pub struct AdminUser {
    pub id: i64,
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Debug)]
pub struct Block {
    pub id: i64,
    pub hash: Vec<u8>,
    pub epoch_no: Option<i32>,
    pub slot_no: Option<i64>,
    pub epoch_slot_no: Option<i32>,
    pub block_no: Option<i32>,
    pub previous_id: Option<i64>,
    pub slot_leader_id: i64,
    pub size: i32,
    pub time: NaiveDateTime,
    pub tx_count: i64,
    pub proto_major: i32,
    pub proto_minor: i32,
    pub vrf_key: Option<String>,
    pub op_cert: Option<Vec<u8>>,
    pub op_cert_counter: Option<i64>,
}

#[derive(Queryable, Debug)]
pub struct CollateralTxIn {
    pub id: i64,
    pub tx_in_id: i64,
    pub tx_out_id: i64,
    pub tx_out_index: i16,
}

#[derive(Queryable, Debug)]
pub struct CostModel {
    pub id: i64,
    pub costs: Jsonb,
    pub block_id: i64,
}

#[derive(Queryable, Debug)]
pub struct Datum {
    pub id: i64,
    pub hash: Vec<u8>,
    pub tx_id: i64,
    pub value: Option<serde_json::Value>,
    pub bytes: Vec<u8>,
}

#[derive(Queryable, Debug)]
pub struct DelegationView {
    pub stake_addr: String,
    pub amount: i64,
    pub cert_index: i32,
    pub active_epoch_no: i64,
}

#[derive(Queryable, Debug)]
pub struct Delegation {
    pub id: i64,
    pub addr_id: i64,
    pub cert_index: i32,
    pub pool_hash_id: i64,
    pub active_epoch_no: i64,
    pub tx_id: i64,
    pub slot_no: i32,
    pub redeemer_id: Option<i64>,
}

#[derive(Queryable, Debug)]
pub struct DelistedPool {
    pub id: i64,
    pub hash_raw: Vec<u8>,
}

#[derive(Queryable, Debug)]
pub struct Epoch {
    pub id: i64,
    pub out_sum: BigDecimal,
    pub fees: BigDecimal,
    pub tx_count: i32,
    pub blk_count: i32,
    pub no: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct EpochParam {
    pub id: i64,
    pub epoch_no: i32,
    pub min_fee_a: i32,
    pub min_fee_b: i32,
    pub max_block_size: i32,
    pub max_tx_size: i32,
    pub max_bh_size: i32,
    pub key_deposit: BigDecimal,
    pub pool_deposit: BigDecimal,
    pub max_epoch: i32,
    pub optimal_pool_count: i32,
    pub influence: f64,
    pub monetary_expand_rate: f64,
    pub treasury_growth_rate: f64,
    pub decentralisation: f64,
    pub protocol_major: i32,
    pub protocol_minor: i32,
    pub min_utxo_value: BigDecimal,
    pub min_pool_cost: BigDecimal,
    pub nonce: Vec<u8>,
    pub cost_model_id: Option<i64>,
    pub price_mem: Option<f64>,
    pub price_step: Option<f64>,
    pub max_tx_ex_mem: Option<BigDecimal>,
    pub max_tx_ex_steps: Option<BigDecimal>,
    pub max_block_ex_mem: Option<BigDecimal>,
    pub max_block_ex_steps: Option<BigDecimal>,
    pub max_val_size: Option<BigDecimal>,
    pub collateral_percent: Option<i32>,
    pub max_collateral_inputs: Option<i32>,
    pub block_id: i64,
    pub entropy: Option<Vec<u8>>,
    pub coins_per_utxo_size: Option<BigDecimal>,
}

#[derive(Queryable, Debug)]
pub struct EpochRewardTotalReceived {
    pub id: i64,
    pub earned_epoch: i32,
    pub amount: BigDecimal,
}

#[derive(Queryable, Debug)]
pub struct EpochStakeView {
    pub stake_addr: String,
    pub amount: BigDecimal,
}

#[derive(Queryable, Debug)]
pub struct EpochStake {
    pub id: i64,
    pub addr_id: i64,
    pub pool_id: i64,
    pub amount: BigDecimal,
    pub epoch_no: i32,
}

#[derive(Queryable, Debug)]
pub struct EpochSyncTime {
    pub id: i64,
    pub no: i64,
    pub seconds: i64,
    pub state: Syncstatetype,
}

#[derive(Queryable, Debug)]
pub struct MaTxMint {
    pub id: i64,
    pub quantity: BigDecimal,
    pub tx_id: i64,
    pub ident: i64,
}

#[derive(Queryable, Debug)]
pub struct MaTxOut {
    pub id: i64,
    pub quantity: BigDecimal,
    pub tx_out_id: i64,
    pub ident: i64,
}

#[derive(Queryable, Debug)]
pub struct Meta {
    pub id: i64,
    pub start_time: NaiveDateTime,
    pub network_name: String,
    pub version: String,
}

#[derive(Queryable, Debug)]
pub struct MultiAsset {
    pub id: i64,
    pub policy: Vec<u8>,
    pub name: Vec<u8>,
    pub fingerprint: String,
}

#[derive(Queryable, Debug)]
pub struct UMultiAsset {
    pub id: i64,
    pub policy: Vec<u8>,
    pub name: Vec<u8>,
    pub fingerprint: String,
    pub quantity: BigDecimal,
}

#[derive(Queryable, Debug)]
pub struct ParamProposal {
    pub id: i64,
    pub epoch_no: i32,
    pub key: Vec<u8>,
    pub min_fee_a: Option<BigDecimal>,
    pub min_fee_b: Option<BigDecimal>,
    pub max_block_size: Option<BigDecimal>,
    pub max_tx_size: Option<BigDecimal>,
    pub max_bh_size: Option<BigDecimal>,
    pub key_deposit: Option<BigDecimal>,
    pub pool_deposit: Option<BigDecimal>,
    pub max_epoch: Option<BigDecimal>,
    pub optimal_pool_count: Option<BigDecimal>,
    pub influence: Option<f64>,
    pub monetary_expand_rate: Option<f64>,
    pub treasury_growth_rate: Option<f64>,
    pub decentralisation: Option<f64>,
    pub entropy: Option<Vec<u8>>,
    pub protocol_major: Option<i32>,
    pub protocol_minor: Option<i32>,
    pub min_utxo_value: Option<BigDecimal>,
    pub min_pool_cost: Option<BigDecimal>,
    pub coins_per_utxo_word: Option<BigDecimal>,
    pub cost_model_id: Option<i64>,
    pub price_mem: Option<f64>,
    pub price_step: Option<f64>,
    pub max_tx_ex_mem: Option<BigDecimal>,
    pub max_tx_ex_steps: Option<BigDecimal>,
    pub max_block_ex_mem: Option<BigDecimal>,
    pub max_block_ex_steps: Option<BigDecimal>,
    pub max_val_size: Option<BigDecimal>,
    pub collateral_percent: Option<i32>,
    pub max_collateral_inputs: Option<i32>,
    pub registered_tx_id: i64,
}

#[derive(Queryable, Debug)]
pub struct PoolHash {
    pub id: i64,
    pub hash_raw: Vec<u8>,
    pub view: String,
}

#[derive(Queryable, Debug)]
pub struct PoolMetadataRef {
    pub id: i64,
    pub pool_id: i64,
    pub url: String,
    pub hash: Vec<u8>,
    pub registered_tx_id: i64,
}

#[derive(Queryable, Debug)]
pub struct PoolOfflineData {
    pub id: i64,
    pub pool_id: i64,
    pub ticker_name: String,
    pub hash: Vec<u8>,
    pub json: Jsonb,
    pub bytes: Vec<u8>,
    pub pmr_id: i64,
}

#[derive(Queryable, Debug)]
pub struct PoolOfflineFetchError {
    pub id: i64,
    pub pool_id: i64,
    pub fetch_time: NaiveDateTime,
    pub pmr_id: i64,
    pub fetch_error: String,
    pub retry_count: i32,
}

#[derive(Queryable, Debug)]
pub struct PoolOwner {
    pub id: i64,
    pub addr_id: i64,
    pub pool_hash_id: i64,
    pub registered_tx_id: i64,
}

#[derive(Queryable, Debug)]
pub struct PoolRelay {
    pub id: i64,
    pub update_id: i64,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
    pub dns_name: Option<String>,
    pub dns_srv_name: Option<String>,
    pub port: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct PoolRetire {
    pub id: i64,
    pub hash_id: i64,
    pub cert_index: i32,
    pub announced_tx_id: i64,
    pub retiring_epoch: i32,
}

#[derive(Queryable, Debug)]
pub struct PoolUpdate {
    pub id: i64,
    pub hash_id: i64,
    pub cert_index: i32,
    pub vrf_key_hash: Vec<u8>,
    pub pledge: BigDecimal,
    pub reward_addr_id: i64,
    pub active_epoch_no: i64,
    pub meta_id: Option<i64>,
    pub margin: f64,
    pub fixed_cost: BigDecimal,
    pub registered_tx_id: i64,
}

#[derive(Queryable, Debug)]
pub struct PotTransfer {
    pub id: i64,
    pub cert_index: i32,
    pub treasury: BigDecimal,
    pub reserves: BigDecimal,
    pub tx_id: i64,
}

#[derive(Queryable, Debug)]
pub struct Redeemer {
    pub id: i64,
    pub tx_id: i64,
    pub unit_mem: i64,
    pub unit_steps: i64,
    pub fee: BigDecimal,
    pub purpose: Scriptpurposetype,
    pub index: i32,
    pub script_hash: Option<Vec<u8>>,
    pub datum_id: i64,
}

#[derive(Queryable, Debug)]
pub struct Reserve {
    pub id: i64,
    pub addr_id: i64,
    pub cert_index: i32,
    pub amount: BigDecimal,
    pub tx_id: i64,
}

#[derive(Queryable, Debug)]
pub struct ReservedPoolTicker {
    pub id: i64,
    pub name: String,
    pub pool_hash: Vec<u8>,
}

#[derive(Queryable, Debug)]
pub struct Reward {
    pub id: i64,
    pub addr_id: i64,
    pub type_: RewardType,
    pub amount: BigDecimal,
    pub earned_epoch: i64,
    pub spendable_epoch: i64,
    pub pool_id: Option<i64>,
}

#[derive(Queryable, Debug)]
pub struct SchemaVersion {
    pub id: i64,
    pub stage_one: i64,
    pub stage_two: i64,
    pub stage_three: i64,
}

#[derive(Queryable, Debug)]
pub struct Script {
    pub id: i64,
    pub tx_id: i64,
    pub hash: Vec<u8>,
    pub type_: Scripttype,
    pub json: Option<serde_json::Value>,
    pub bytes: Option<Vec<u8>>,
    pub serialised_size: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct SlotLeader {
    pub id: i64,
    pub hash: Vec<u8>,
    pub pool_hash_id: Option<i64>,
    pub description: String,
}

#[derive(Queryable, Debug)]
pub struct StakeAddres {
    pub id: i64,
    pub hash_raw: Vec<u8>,
    pub view: String,
    pub script_hash: Option<Vec<u8>>,
    pub registered_tx_id: i64,
}

#[derive(Queryable, Debug)]
pub struct StakeDeregistration {
    pub id: i64,
    pub addr_id: i64,
    pub cert_index: i32,
    pub epoch_no: i32,
    pub tx_id: i64,
    pub redeemer_id: Option<i64>,
}

#[derive(Queryable, Debug)]
pub struct StakeRegistration {
    pub id: i64,
    pub addr_id: i64,
    pub cert_index: i32,
    pub epoch_no: i32,
    pub tx_id: i64,
}

#[derive(Queryable, Debug)]
pub struct Treasury {
    pub id: i64,
    pub addr_id: i64,
    pub cert_index: i32,
    pub amount: BigDecimal,
    pub tx_id: i64,
}

#[derive(Queryable, Debug)]
pub struct Tx {
    pub id: i64,
    pub hash: Vec<u8>,
    pub block_id: i64,
    pub block_index: i32,
    pub out_sum: BigDecimal,
    pub fee: BigDecimal,
    pub deposit: i64,
    pub size: i32,
    pub invalid_before: Option<BigDecimal>,
    pub invalid_hereafter: Option<BigDecimal>,
    pub valid_contract: bool,
    pub script_size: i32,
}

#[derive(Queryable, Debug)]
pub struct TxIn {
    pub id: i64,
    pub tx_in_id: i64,
    pub tx_out_id: i64,
    pub tx_out_index: i16,
    pub redeemer_id: Option<i64>,
}

#[derive(Queryable, Debug)]
pub struct TxMetadata {
    pub id: i64,
    pub key: BigDecimal,
    pub json: Option<Jsonb>,
    pub bytes: Vec<u8>,
    pub tx_id: i64,
}

#[derive(Queryable, Debug)]
pub struct TxOut {
    pub id: i64,
    pub tx_id: i64,
    pub index: i16,
    pub address: String,
    pub address_raw: Vec<u8>,
    pub address_has_script: bool,
    pub payment_cred: Option<Vec<u8>>,
    pub stake_address_id: Option<i64>,
    pub value: BigDecimal,
    pub data_hash: Option<Vec<u8>>,
    pub inline_datum_id: Option<i64>,
    pub reference_script_id: Option<i64>,
}

impl TxOut {
    pub fn to_txuo(
        &self,
        dbs: &DBSyncProvider,
    ) -> Result<dcslc::TransactionUnspentOutput, DataProviderDBSyncError> {
        use super::schema::*;
        let tx = tx::table
            .filter(tx::id.eq(&self.tx_id))
            .first::<Tx>(&mut dbs.connect()?)?;
        log::debug!("try to create input utxo for tx {tx:?}");
        let input = cardano_serialization_lib::TransactionInput::new(
            &cardano_serialization_lib::crypto::TransactionHash::from_bytes(tx.hash.clone())
                .map_err(|_| {
                    DataProviderDBSyncError::Custom(
                        format!("Could not restore TransactionHash from bytes:  {:?}", tx)
                            .to_string(),
                    )
                })?,
            self.index as u32,
        );
        log::debug!("input is: {input:?}");
        log::debug!("try to decode address: {0:?}", self.address);
        let address = dcslc::addr_from_str(&self.address)?;
        log::debug!("Address is: {address:?}");
        let coin = match self.value.to_u64() {
            Some(c) => c,
            None => {
                return Err(DataProviderDBSyncError::Custom(
                    format!(
                        "Could not determinte Lovelace on Utxo:  {:?}#{:?}",
                        input, self.index
                    )
                    .to_string(),
                ))
            }
        };
        log::debug!("try to create ser lib coin Value from : {coin:?}");
        let mut amount = cardano_serialization_lib::utils::Value::new(
            &cardano_serialization_lib::utils::to_bignum(coin),
        );
        log::debug!("try to create tokens, Ada amount set Value from : {amount:?}");
        let tokens = api::get_txo_tokens(&dbs, self.tx_id, self.index)?;
        let mut ma = cardano_serialization_lib::MultiAsset::new();
        for tok in tokens {
            match ma.get(&cardano_serialization_lib::PolicyID::from_bytes(
                tok.policy.clone(),
            )?) {
                Some(mut asset) => {
                    asset.insert(
                        &cardano_serialization_lib::AssetName::new(tok.name)?,
                        &cardano_serialization_lib::utils::to_bignum(
                            tok.quantity.to_u64().unwrap(),
                        ),
                    );
                    ma.insert(
                        &cardano_serialization_lib::PolicyID::from_bytes(tok.policy)?,
                        &asset,
                    );
                }
                None => {
                    let mut a = cardano_serialization_lib::Assets::new();
                    a.insert(
                        &cardano_serialization_lib::AssetName::new(tok.name)?,
                        &cardano_serialization_lib::utils::to_bignum(
                            tok.quantity.to_u64().unwrap(),
                        ),
                    );
                    ma.insert(
                        &cardano_serialization_lib::PolicyID::from_bytes(tok.policy)?,
                        &a,
                    );
                }
            }
        }
        if ma.len() > 0 {
            amount.set_multiasset(&ma)
        }

        let mut output = cardano_serialization_lib::TransactionOutput::new(&address, &amount);

        if let Some(hash) = &self.data_hash {
            output.set_data_hash(&(DataHash::from_bytes(hash.clone())?));
            if let Some(_) = self.inline_datum_id {
                let datum = datum::table
                    .filter(datum::hash.eq(hash))
                    .first::<Datum>(&mut dbs.connect()?)?;
                let pdatum =
                    cardano_serialization_lib::plutus::PlutusData::from_bytes(datum.bytes)?;

                output.set_plutus_data(&pdatum);
            }
        }

        if let Some(id) = self.reference_script_id {
            let script = script::table
                .filter(script::id.eq(id))
                .first::<Script>(&mut dbs.connect()?)?;

            let scr = match script.type_ {
                Scripttype::Multisig | Scripttype::Tiemlock => ScriptRef::new_native_script(
                    &NativeScript::from_json(&script.json.unwrap().to_string())?,
                ),
                Scripttype::PlutusV1 => {
                    ScriptRef::new_plutus_script(&PlutusScript::from_bytes(script.bytes.unwrap())?)
                }
                Scripttype::PlutusV2 => ScriptRef::new_plutus_script(&PlutusScript::from_bytes_v2(
                    script.bytes.unwrap(),
                )?),
            };
            output.set_script_ref(&scr)
        }

        Ok(cardano_serialization_lib::utils::TransactionUnspentOutput::new(&input, &output))
    }
}

#[derive(Queryable, Debug)]
pub struct Withdrawal {
    pub id: i64,
    pub addr_id: i64,
    pub amount: BigDecimal,
    pub redeemer_id: Option<i64>,
    pub tx_id: i64,
}
