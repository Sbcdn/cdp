use std::iter::zip;

use bigdecimal::{BigDecimal, ToPrimitive};
use cardano_serialization_lib::{crypto::ScriptHash, utils::BigNum, AssetName};
use dcslc::{make_fingerprint, TransactionUnspentOutput};
use diesel::Queryable;

pub type Token = (ScriptHash, AssetName, BigNum);
pub type Tokens = Vec<Token>;

#[derive(Queryable, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TokenInfoView {
    pub fingerprint: String,
    pub policy: String,
    pub tokenname: String,
    pub quantity: Option<u64>,
    pub meta_key: Option<i64>,
    pub json: Option<serde_json::Value>,
    pub mint_slot: Option<i64>,
    pub txhash: Option<String>,
}

#[derive(Queryable, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct StakeDelegationView {
    pub stake_address: String,
    pub amount: BigDecimal,
}

#[derive(Queryable, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct DelegationView {
    pub stake_address: String,
    pub amount: i64,
    pub cert_index: i32,
    pub active_epoch_no: i64,
}

#[derive(Queryable, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct HoldingWalletView {
    pub stake_address: String,
    pub amount: u64,
    pub policy: String,
    pub tokenname: Option<String>,
    pub fingerprint: Option<String>,
}

#[derive(Queryable, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CardanoNativeAssetView {
    pub id: i64,
    pub policy: Vec<u8>,
    pub name: Vec<u8>,
    pub fingerprint: String,
    pub quantity: BigDecimal,
}

#[derive(Queryable, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StakeDeregistrationView {
    pub stake_address: String,
    pub tx_hash: Vec<u8>,
    pub cert_index: i32,
    pub epoch: i32,
}

#[derive(Queryable, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StakeRegistrationView {
    pub stake_address: String,
    pub tx_hash: Vec<u8>,
    pub cert_index: i32,
    pub epoch: i32,
}

#[derive(Queryable, Debug, Clone)]
pub struct CDPDatum {
    pub hash: Vec<u8>,
    pub json: Option<serde_json::Value>,
    pub bytes: Vec<u8>,
    pub address: String,
    pub addr_has_script: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TransactionView {
    pub hash: String,
    pub block: String,
    pub slot: Option<i64>,
    pub inputs: Vec<UTxOView>,
    pub reference_inputs: Option<Vec<UTxOView>>,
    pub outputs: Vec<UTxOView>,
    pub withdrawals: Option<Vec<WithdrawalView>>,
    pub metadata: Option<serde_json::Value>,
    pub stake_registration: Option<Vec<StakeRegistrationView>>,
    pub stake_deregistration: Option<Vec<StakeDeregistrationView>>,
    pub script: Option<Vec<ScriptView>>,
    pub collateral_tx_in: Option<Vec<UTxOView>>,
    pub collateral_tx_out: Option<Vec<UTxOView>>,
    pub fee: u64,
    pub cbor: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TxHistoryListAssetView {
    fingerprint: String,
    amount: u64,
}

impl Into<String> for TxHistoryListAssetView {
    fn into(self) -> String {
        self.fingerprint + "$" + &(self.amount.to_string())
    }
}

impl TxHistoryListAssetView {
    pub fn new(fingerprint: String, amount: u64) -> Self {
        Self {
            fingerprint,
            amount,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, QueryableByName)]
pub struct TxHistoryListQuery {
    #[diesel(sql_type = diesel::sql_types::Bytea, column_name = hash, deserialize_as = Vec<u8>)]
    hash: Vec<u8>,
    #[diesel(sql_type = diesel::sql_types::BigInt, column_name = slot, deserialize_as = i64)]
    slot: i64,
    #[diesel(sql_type = diesel::sql_types::Array<diesel::sql_types::Text>, column_name = fingerprint)]
    fingerprint: Vec<String>,
    #[diesel(sql_type = diesel::sql_types::Array<diesel::sql_types::Numeric>, column_name = value)]
    value: Vec<BigDecimal>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, QueryableByName)]
pub struct TxHistoryListQueryLight {
    #[diesel(sql_type = diesel::sql_types::Bytea, column_name = hash, deserialize_as = Vec<u8>)]
    pub hash: Vec<u8>,
    #[diesel(sql_type = diesel::sql_types::BigInt, column_name = slot, deserialize_as = i64)]
    pub slot: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TxHistoryListViewLight {
    pub hash: Vec<u8>,
    pub slot: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TxHistoryListView {
    pub hash: String,
    pub slot: i64,
    pub assets: Vec<TxHistoryListAssetView>,
}

impl TxHistoryListView {
    pub fn new(hash: String, slot: i64, assets: Vec<TxHistoryListAssetView>) -> Self {
        Self { hash, slot, assets }
    }

    pub fn from_tx_history_list_query(d: &TxHistoryListQuery) -> Self {
        let mut assets = Vec::<TxHistoryListAssetView>::new();
        for (f, a) in zip(d.fingerprint.iter(), d.value.iter()) {
            let asset = TxHistoryListAssetView::new(f.to_string(), a.to_u64().unwrap());
            assets.push(asset)
        }
        Self {
            hash: hex::encode(&d.hash),
            slot: d.slot,
            assets,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct WithdrawalView {
    pub stake_address: String,
    pub amount: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ScriptView {
    pub hash: String,
    pub r#type: crate::dbsync::models::Scripttype,
    pub json: Option<serde_json::Value>,
    pub bytes: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct UTxOView {
    pub hash: String,
    pub index: i32,
    pub address: String,
    pub amount: ValueView,
    pub plutus_data: Option<String>,
    pub script_ref: Option<String>,
}

impl UTxOView {
    pub fn from_txuo(txuo: &TransactionUnspentOutput) -> Self {
        let plutus_data = txuo
            .output()
            .plutus_data()
            .as_ref()
            .map(|data| data.to_hex());

        let ma = txuo
            .output()
            .amount()
            .multiasset()
            .as_ref()
            .map(AssetHandle::from_multi_asset);

        let amount = ValueView {
            coin: txuo.output().amount().coin().into(),
            multiasset: ma,
        };

        let script_ref = txuo
            .output()
            .script_ref()
            .as_ref()
            .map(|script_ref| script_ref.to_hex());

        Self {
            hash: txuo.input().transaction_id().to_hex(),
            index: txuo.input().index() as i32,
            address: txuo.output().address().to_bech32(None).unwrap(),
            amount,
            plutus_data,
            script_ref,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ValueView {
    pub coin: u64,
    pub multiasset: Option<Vec<AssetHandle>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AssetHandle {
    pub fingerprint: Option<String>,
    pub policy: Option<String>,
    pub tokenname: Option<String>,
    pub amount: u64,
    pub metadata: Option<serde_json::Value>,
}

impl AssetHandle {
    pub fn from_multi_asset(ma: &cardano_serialization_lib::MultiAsset) -> Vec<Self> {
        let mut out = Vec::<AssetHandle>::new();
        let policies = ma.keys();

        for p in 0..policies.len() {
            let policy = policies.get(p);
            let assets = ma.get(&policies.get(p)).unwrap().keys();
            for a in 0..assets.len() {
                let assetname = assets.get(a);
                let amount = ma.get_asset(&policy, &assetname);
                out.push(Self {
                    fingerprint: if let Ok(f) =
                        make_fingerprint(&policy.to_hex(), &hex::encode(assetname.name()))
                    {
                        Some(f)
                    } else {
                        None
                    },
                    policy: Some(policy.to_hex()),
                    tokenname: Some(hex::encode(assetname.name())),
                    amount: amount.into(),
                    metadata: None,
                })
            }
        }
        out
    }

    pub fn new_empty() -> Self {
        AssetHandle {
            fingerprint: None,
            policy: None,
            tokenname: None,
            amount: 0,
            metadata: None,
        }
    }
}

impl PartialEq for AssetHandle {
    fn eq(&self, other: &Self) -> bool {
        match self.policy {
            Some(_) => {
                self.fingerprint == other.fingerprint
                    && self.policy == other.policy
                    && self.tokenname == other.tokenname
            }
            None => {
                other.policy.is_none()
                    && other.fingerprint.is_none()
                    && other.tokenname.is_none()
                    && self.tokenname.is_none()
                    && self.fingerprint.is_none()
            }
        }
    }
}

#[derive(Queryable, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PoolView {
    pub pool_hash: String,
    pub ticker: String,
    pub json: serde_json::Value,
}

#[derive(Queryable, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RewardView {
    pub amount: u64,
    pub earned_epoch: i64,
    pub spendable_epoch: i64, 
}
