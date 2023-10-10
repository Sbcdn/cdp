pub mod config;
pub mod error;
use crate::models::{CDPDatum, TxHistoryListView
};

use super::models::{
    CardanoNativeAssetView, DelegationView, HoldingWalletView, StakeDelegationView,
    StakeDeregistrationView, StakeRegistrationView, TokenInfoView, RewardView,
};
use async_trait::async_trait;
use cardano_serialization_lib::address::Address;
use dcslc::TransactionUnspentOutputs;
use error::DataProviderError;
use bigdecimal::BigDecimal;
use serde_json::Value;

#[async_trait]
pub trait CardanoDataProvider {
    ///returns true if the dataprovider is operational
    async fn alive(&self) -> bool;
    /// returns all TransactionUnspentOutputs of a stake address, does not include any script addresses
    async fn wallet_utxos(
        &self,
        stake_addr: &str,
    ) -> Result<TransactionUnspentOutputs, DataProviderError>;
    /// returns all TransactionUnspentOutputs of an address
    async fn script_utxos(
        &self,
        addr: &str,
    ) -> Result<TransactionUnspentOutputs, DataProviderError>;
    /// return all TransactionUnspentOutputs containing Cardano Native Tokens
    async fn asset_utxos_on_addr(
        &self,
        addr: &str,
    ) -> Result<TransactionUnspentOutputs, DataProviderError>;
    /// returns metadata of the last minting transaction
    async fn mint_metadata(&self, fingerprint_in: &str)
        -> Result<TokenInfoView, DataProviderError>;
    /// returns the first seen adress on the chain using the given stake adress
    async fn first_transaction_from_stake_addr(
        &self,
        stake_address_in: &str,
    ) -> Result<Address, DataProviderError>;
    /// returns Utxo of a certain datumhash on an address
    async fn utxo_by_dataumhash(
        &self,
        addr: &str,
        datumhash: &Vec<u8>,
    ) -> Result<dcslc::TransactionUnspentOutput, DataProviderError>;
    /// returns Utxo of a certain datumhash on an address
    async fn utxo_by_txid(
        &self,
        txhash: &Vec<u8>,
        index: i16,
    ) -> Result<dcslc::TransactionUnspentOutput, DataProviderError>;
    /// return the Cardano Native Tokens on an utxo using the dbsync txout-id
    async fn utxo_tokens(
        &self,
        tx_id: i64,
        tx_index: i16,
    ) -> Result<Vec<CardanoNativeAssetView>, DataProviderError>;
    /// find all datums included in this tx
    async fn find_datums_for_tx(&self, txid: &Vec<u8>) -> Result<Vec<CDPDatum>, DataProviderError>;
    /// returns the latest slot
    async fn slot(&self) -> Result<i64, DataProviderError>;
    /// return an Vector containing all stake addresses and their staked amount for the given epoch and pool
    async fn stakers_on_pool(
        &self,
        pool: &str,
        epoch: i32,
    ) -> Result<Vec<StakeDelegationView>, DataProviderError>;
    /// return the delegations for a pool in an epoch intervall
    async fn delegations_per_pool_epoch_intervall(
        &self,
        pool: &str,
        start_epoch: i64,
        end_epoch: i64,
    ) -> Result<Vec<DelegationView>, DataProviderError>;
    /// return the total staked ADA for pool and epoch
    async fn pool_total_staked(&self, pool: &str, epoch: i32) -> Result<u64, DataProviderError>;
    /// returns current epoch
    async fn current_epoch(&self) -> Result<i32, DataProviderError>;
    /// returns the fingerprint for the policy and tokenname; better use make_fingerprint() from csl_common library
    async fn fingerprint(&self, policy: &str, tokenname: &str)
        -> Result<String, DataProviderError>;
    /// returns token infos
    async fn token_info(&self, fingerprint_in: &str) -> Result<TokenInfoView, DataProviderError>;
    /// returns the stake registrations for a stake address
    async fn stake_registration(
        &self,
        stake_addr_in: &str,
    ) -> Result<Vec<StakeRegistrationView>, DataProviderError>;
    /// returns the stake de-registrations for a stake address
    async fn stake_deregistration(
        &self,
        stake_addr_in: &str,
    ) -> Result<Vec<StakeDeregistrationView>, DataProviderError>;
    /// returns true if a stake address is already registered
    async fn check_stakeaddr_registered(
        &self,
        stake_addr_in: &str,
    ) -> Result<bool, DataProviderError>;
    /// search for all wallets holding a specific token, optionally a minimum amount can be provided
    async fn lookup_token_holders(
        &self,
        fingerprint_in: &str,
        min_amount: Option<&i64>,
    ) -> Result<Vec<HoldingWalletView>, DataProviderError>;
    /// search all holders of a specific policy id where the asset value is 1
    async fn lookup_nft_token_holders(
        &self,
        policy: &str,
    ) -> Result<Vec<HoldingWalletView>, DataProviderError>;
    /// is the pool correctly registered and active
    async fn pool_valid(&self, pool_id: &str) -> Result<bool, DataProviderError>;
    /// checks if a utxo is already spent
    async fn txhash_spent(&self, txhash: &str) -> Result<bool, DataProviderError>;

    async fn addresses_exist(&self, addresses: &Vec<&str>) -> Result<Vec<bool>, DataProviderError>;

    async fn tx_history(
        &self,
        addresses: &Vec<&str>,
        slot: Option<u64>,
    ) -> Result<Vec<TxHistoryListView>, DataProviderError>;

    async fn retrieve_staked_amount (
        &self,
        epoch: i32,
        stake_addr: &str,
    ) -> Result<BigDecimal, DataProviderError>;

    async fn retrieve_generated_rewards (
        &self,
        stake_addr: &str,
    ) -> Result<Vec<RewardView>, DataProviderError>;

    async fn pool_vrf_key_hash(
        &self,
        pool_hash: &str,
    ) -> Result<Vec<u8>, DataProviderError>;

    async fn pool_blocks_minted(
        &self,
        pool_hash: &str,
    ) -> Result<i64, DataProviderError>;

    async fn pool_blocks_current_epoch(
        &self,
        pool_hash: &str,
    ) -> Result<i64, DataProviderError>;

    async fn pool_reward_recipients(
        &self,
        pool_hash: &str,
    ) -> Result<i64, DataProviderError>;

    async fn pool_last_reward_earned_epoch(
        &self,
        pool_hash: &str,
    ) -> Result<i64, DataProviderError>;

    async fn pool_declared_pledge(
        &self,
        pool_hash: &str,
    ) -> Result<BigDecimal, DataProviderError>;

    async fn pool_margin_cost(
        &self,
        pool_hash: &str,
    ) -> Result<f64, DataProviderError>;

    async fn pool_fixed_cost(
        &self,
        pool_hash: &str,
    ) -> Result<BigDecimal, DataProviderError>;

    async fn pool_reward_address(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError>;

    async fn pool_owner(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError>;

    async fn pool_registration(
        &self,
        pool_hash: &str,
    ) -> Result<i64, DataProviderError>;

    async fn pool_retirement(
        &self,
        pool_hash: &str,
    ) -> Result<i32, DataProviderError>;

    async fn pool_url(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError>;

    async fn pool_ticker(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError>;

    async fn pool_metadata_json(
        &self,
        pool_hash: &str,
    ) -> Result<Value, DataProviderError>;

    async fn pool_name(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError>;

    async fn pool_homepage(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError>;

    async fn pool_description(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError>;
}

pub struct DataProvider<T: CardanoDataProvider> {
    //config: Config,
    provider: T,
}

impl<T: CardanoDataProvider> DataProvider<T> {
    pub fn new(provider: T) -> Self {
        DataProvider::<T> { provider }
    }

    pub fn provider(&self) -> &T {
        &self.provider
    }
}
#[async_trait]
impl<T: CardanoDataProvider + std::marker::Sync + std::marker::Send> CardanoDataProvider
    for DataProvider<T>
{
    async fn wallet_utxos(
        &self,
        stake_addr: &str,
    ) -> Result<TransactionUnspentOutputs, DataProviderError> {
        self.provider().wallet_utxos(stake_addr).await
    }

    async fn script_utxos(
        &self,
        addr: &str,
    ) -> Result<TransactionUnspentOutputs, DataProviderError> {
        self.provider().script_utxos(addr).await
    }

    async fn asset_utxos_on_addr(
        &self,
        addr: &str,
    ) -> Result<TransactionUnspentOutputs, DataProviderError> {
        self.provider().asset_utxos_on_addr(addr).await
    }

    async fn mint_metadata(
        &self,
        fingerprint_in: &str,
    ) -> Result<TokenInfoView, DataProviderError> {
        self.provider().mint_metadata(fingerprint_in).await
    }

    async fn first_transaction_from_stake_addr(
        &self,
        stake_address_in: &str,
    ) -> Result<Address, DataProviderError> {
        self.provider()
            .first_transaction_from_stake_addr(stake_address_in)
            .await
    }
    /// returns Utxo of a certain datumhash on an address
    async fn utxo_by_txid(
        &self,
        txhash: &Vec<u8>,
        index: i16,
    ) -> Result<dcslc::TransactionUnspentOutput, DataProviderError> {
        self.provider().utxo_by_txid(txhash, index).await
    }

    /// get all utxos of an address
    async fn utxo_by_dataumhash(
        &self,
        addr: &str,
        datumhash: &Vec<u8>,
    ) -> Result<dcslc::TransactionUnspentOutput, DataProviderError> {
        self.provider().utxo_by_dataumhash(addr, datumhash).await
    }
    async fn utxo_tokens(
        &self,
        tx_id: i64,
        tx_index: i16,
    ) -> Result<Vec<CardanoNativeAssetView>, DataProviderError> {
        self.provider().utxo_tokens(tx_id, tx_index).await
    }

    async fn find_datums_for_tx(&self, txid: &Vec<u8>) -> Result<Vec<CDPDatum>, DataProviderError> {
        self.provider().find_datums_for_tx(txid).await
    }
    async fn slot(&self) -> Result<i64, DataProviderError> {
        self.provider().slot().await
    }

    async fn stakers_on_pool(
        &self,
        pool: &str,
        epoch: i32,
    ) -> Result<Vec<StakeDelegationView>, DataProviderError> {
        self.provider().stakers_on_pool(pool, epoch).await
    }

    async fn delegations_per_pool_epoch_intervall(
        &self,
        pool: &str,
        start_epoch: i64,
        end_epoch: i64,
    ) -> Result<Vec<DelegationView>, DataProviderError> {
        self.provider()
            .delegations_per_pool_epoch_intervall(pool, start_epoch, end_epoch)
            .await
    }

    async fn pool_total_staked(&self, pool: &str, epoch: i32) -> Result<u64, DataProviderError> {
        self.provider().pool_total_staked(pool, epoch).await
    }

    async fn current_epoch(&self) -> Result<i32, DataProviderError> {
        self.provider().current_epoch().await
    }

    async fn fingerprint(
        &self,
        policy: &str,
        tokenname: &str,
    ) -> Result<String, DataProviderError> {
        self.provider().fingerprint(policy, tokenname).await
    }

    async fn token_info(&self, fingerprint_in: &str) -> Result<TokenInfoView, DataProviderError> {
        self.provider().token_info(fingerprint_in).await
    }

    async fn stake_registration(
        &self,
        stake_addr_in: &str,
    ) -> Result<Vec<StakeRegistrationView>, DataProviderError> {
        self.provider().stake_registration(stake_addr_in).await
    }

    async fn stake_deregistration(
        &self,
        stake_addr_in: &str,
    ) -> Result<Vec<StakeDeregistrationView>, DataProviderError> {
        self.provider().stake_deregistration(stake_addr_in).await
    }

    async fn check_stakeaddr_registered(
        &self,
        stake_addr_in: &str,
    ) -> Result<bool, DataProviderError> {
        self.provider()
            .check_stakeaddr_registered(stake_addr_in)
            .await
    }

    async fn lookup_token_holders(
        &self,
        fingerprint_in: &str,
        min_amount: Option<&i64>,
    ) -> Result<Vec<HoldingWalletView>, DataProviderError> {
        self.provider()
            .lookup_token_holders(fingerprint_in, min_amount)
            .await
    }

    async fn lookup_nft_token_holders(
        &self,
        policy: &str,
    ) -> Result<Vec<HoldingWalletView>, DataProviderError> {
        self.provider().lookup_nft_token_holders(policy).await
    }

    async fn pool_valid(&self, pool_id: &str) -> Result<bool, DataProviderError> {
        self.provider().pool_valid(pool_id).await
    }

    async fn txhash_spent(&self, txhash: &str) -> Result<bool, DataProviderError> {
        self.provider().txhash_spent(txhash).await
    }

    async fn alive(&self) -> bool {
        self.provider().alive().await
    }

    async fn addresses_exist(&self, addresses: &Vec<&str>) -> Result<Vec<bool>, DataProviderError> {
        self.provider().addresses_exist(addresses).await
    }

    async fn tx_history(
        &self,
        addresses: &Vec<&str>,
        slot: Option<u64>,
    ) -> Result<Vec<TxHistoryListView>, DataProviderError> {
        self.provider().tx_history(addresses, slot).await
    }

    async fn retrieve_staked_amount (
        &self,
        epoch: i32,
        stake_addr: &str,
    ) -> Result<BigDecimal, DataProviderError> {
        dbg!(self.provider().retrieve_staked_amount(epoch, stake_addr).await)
    }

    async fn retrieve_generated_rewards (
        &self,
        stake_addr: &str,
    ) -> Result<Vec<RewardView>, DataProviderError> {
        dbg!(self.provider().retrieve_generated_rewards(stake_addr).await)
    }

    async fn pool_vrf_key_hash (
        &self,
        pool_hash: &str,
    ) -> Result<Vec<u8>, DataProviderError> {
        self.provider().pool_vrf_key_hash(pool_hash).await
    }

    async fn pool_blocks_minted (
        &self,
        pool_hash: &str,
    ) -> Result<i64, DataProviderError> {
        self.provider().pool_blocks_minted(pool_hash).await
    }

    async fn pool_blocks_current_epoch (
        &self,
        pool_hash: &str,
    ) -> Result<i64, DataProviderError> {
        self.provider().pool_blocks_current_epoch(pool_hash).await
    }

    async fn pool_reward_recipients (
        &self,
        pool_hash: &str,
    ) -> Result<i64, DataProviderError> {
        self.provider().pool_reward_recipients(pool_hash).await
    }

    async fn pool_last_reward_earned_epoch (
        &self,
        pool_hash: &str,
    ) -> Result<i64, DataProviderError> {
        self.provider().pool_last_reward_earned_epoch(pool_hash).await
    }

    async fn pool_declared_pledge (
        &self,
        pool_hash: &str,
    ) -> Result<BigDecimal, DataProviderError> {
        self.provider().pool_declared_pledge(pool_hash).await
    }

    async fn pool_margin_cost(
        &self, 
        pool_hash: &str,
    ) -> Result<f64, DataProviderError> {
        self.provider().pool_margin_cost(pool_hash).await
    }

    async fn pool_fixed_cost(
        &self,
        pool_hash: &str,
    ) -> Result<BigDecimal, DataProviderError> {
        self.provider().pool_fixed_cost(pool_hash).await
    }

    async fn pool_reward_address(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError> {
        self.provider().pool_reward_address(pool_hash).await
    }

    async fn pool_owner(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError> {
        self.provider().pool_owner(pool_hash).await
    }

    async fn pool_registration(
        &self,
        pool_hash: &str,
    ) -> Result<i64, DataProviderError> {
        self.provider().pool_registration(pool_hash).await
    }

    async fn pool_retirement(
        &self,
        pool_hash: &str,
    ) -> Result<i32, DataProviderError> {
        self.provider().pool_retirement(pool_hash).await
    }

    async fn pool_url(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError> {
        self.provider().pool_url(pool_hash).await
    }

    async fn pool_ticker(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError> {
        self.provider().pool_ticker(pool_hash).await
    }

    async fn pool_metadata_json(
        &self,
        pool_hash: &str,
    ) -> Result<Value, DataProviderError> {
        self.provider().pool_metadata_json(pool_hash).await
    }

    async fn pool_name(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError> {
        self.provider().pool_name(pool_hash).await
    }

    async fn pool_homepage(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError> {
        self.provider().pool_homepage(pool_hash).await
    }

    async fn pool_description(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError> {
        self.provider().pool_description(pool_hash).await
    }
}
