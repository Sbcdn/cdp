use crate::models::{CDPDatum, TokenInfoView, CardanoNativeAssetView, StakeDelegationView, DelegationView,
    StakeRegistrationView, StakeDeregistrationView, HoldingWalletView, TxHistoryListView, RewardView
};
use crate::provider::error::DataProviderError;

use self::error::DataProviderKoiosError;
use async_trait::async_trait;
pub mod api;
pub mod error;
pub mod models;
use bigdecimal::BigDecimal;
use serde_json::Value;

pub struct Config {
    pub url: String,
    pub api_token: String,
}

pub struct KoiosProvider {
    config: Config,
}

impl KoiosProvider {
    pub fn new(config: Config) -> Self {
        KoiosProvider { config }
    }

    fn connect(&self) -> Result<(), DataProviderKoiosError> {
        Ok(())
    }
}

unsafe impl Send for KoiosProvider {}
unsafe impl Sync for KoiosProvider {}

#[async_trait]
impl super::provider::CardanoDataProvider for KoiosProvider {
    async fn alive(&self) -> bool {
        self.connect().is_ok()
    }

    async fn wallet_utxos(
        &self,
        stake_addr: &str,
    ) -> Result<dcslc::TransactionUnspentOutputs, DataProviderError> {
        Ok(api::get_stake_address_utxos(self, stake_addr)?)
    }

    async fn get_address_utxos(
        &self,
        addr: &str,
    ) -> Result<dcslc::TransactionUnspentOutputs, DataProviderError> {
        Ok(api::get_address_utxos(self, addr)?)
    }

    async fn asset_utxos_on_addr(
        &self,
        addr: &str,
    ) -> Result<dcslc::TransactionUnspentOutputs, DataProviderError> {
        Ok(api::asset_utxos_on_addr(self, addr)?)
    }

    async fn mint_metadata(
        &self,
        fingerprint_in: &str,
    ) -> Result<TokenInfoView, DataProviderError> {
        Ok(api::mint_metadata(self, fingerprint_in)?)
    }

    async fn first_transaction_from_stake_addr(
        &self,
        stake_address_in: &str,
    ) -> Result<
        cardano_serialization_lib::address::Address,
        DataProviderError,
    > {
        let str_addr = api::select_addr_of_first_transaction(self, stake_address_in)?;
        Ok(dcslc::addr_from_str(&str_addr)?)
    }

    /// get all utxos of an address
    async fn utxo_by_datumhash(
        &self,
        addr: &str,
        datumhash: &Vec<u8>,
    ) -> Result<dcslc::TransactionUnspentOutput, DataProviderError> {
        let utxo = api::get_utxo_by_datumhash(self, addr, datumhash)?;
        Ok(utxo)
    }

    /// returns Utxo of a certain datumhash on an address
    async fn utxo_by_txid(
        &self,
        txhash: &Vec<u8>,
        index: i16,
    ) -> Result<dcslc::TransactionUnspentOutput, DataProviderError> {
        let utxo = api::utxo_by_txid(self, txhash, index)?;
        Ok(utxo)
    }

    async fn utxo_tokens(
        &self,
        tx_id: i64,
        tx_index: i16,
    ) -> Result<Vec<CardanoNativeAssetView>, DataProviderError>
    {
        Ok(api::get_utxo_tokens(self, tx_id, tx_index)?)
    }

    async fn find_datums_for_tx(
        &self,
        txid: &Vec<u8>,
    ) -> Result<Vec<CDPDatum>, DataProviderError> {
        Ok(api::find_datums_for_tx(self, txid)?)
    }

    async fn slot(&self) -> Result<i64, DataProviderError> {
        Ok(api::slot(self)?)
    }

    async fn stakers_on_pool(
        &self,
        pool: &str,
        epoch: i32,
    ) -> Result<Vec<StakeDelegationView>, DataProviderError>
    {
        Ok(api::stakers_on_pool(self, pool, epoch)?)
    }

    async fn deligations_per_pool_epoch_intervall(
        &self,
        pool: &str,
        start_epoch: i64,
        end_epoch: i64,
    ) -> Result<Vec<DelegationView>, DataProviderError> {
        Ok(api::deligations_per_pool_for_epochs(
            self,
            pool,
            start_epoch,
            end_epoch,
        )?)
    }

    async fn pool_total_staked(
        &self,
        pool: &str,
        epoch: i32,
    ) -> Result<u64, DataProviderError> {
        Ok(api::pool_total_stake(self, pool, epoch)?)
    }

    async fn current_epoch(&self) -> Result<i32, DataProviderError> {
        Ok(api::current_epoch(self)?)
    }

    async fn fingerprint(
        &self,
        policy: &str,
        tokenname: &str,
    ) -> Result<String, DataProviderError> {
        Ok(api::fingerprint(self, policy, tokenname)?)
    }

    async fn token_info(
        &self,
        fingerprint_in: &str,
    ) -> Result<TokenInfoView, DataProviderError> {
        Ok(api::token_info(self, fingerprint_in)?)
    }

    async fn stake_registration(
        &self,
        stake_addr_in: &str,
    ) -> Result<Vec<StakeRegistrationView>, DataProviderError>
    {
        Ok(api::stake_registration(self, stake_addr_in)?)
    }

    async fn stake_deregistration(
        &self,
        stake_addr_in: &str,
    ) -> Result<
        Vec<StakeDeregistrationView>,
        DataProviderError,
    > {
        Ok(api::stake_deregistration(self, stake_addr_in)?)
    }

    async fn check_stakeaddr_registered(
        &self,
        stake_addr_in: &str,
    ) -> Result<bool, DataProviderError> {
        Ok(api::check_stakeaddr_registered(self, stake_addr_in)?)
    }

    async fn lookup_token_holders(
        &self,
        fingerprint_in: &str,
        min_amount: Option<&i64>,
    ) -> Result<Vec<HoldingWalletView>, DataProviderError>
    {
        Ok(api::lookup_token_holders(self, fingerprint_in, min_amount)?)
    }

    async fn lookup_nft_token_holders(
        &self,
        policy: &str,
    ) -> Result<Vec<HoldingWalletView>, DataProviderError>
    {
        Ok(api::lookup_nft_token_holders(self, policy)?)
    }

    async fn pool_valid(
        &self,
        pool_id: &str,
    ) -> Result<bool, DataProviderError> {
        Ok(api::pool_valid(self, pool_id)?)
    }

    async fn txhash_spent(
        &self,
        txhash: &str,
    ) -> Result<bool, DataProviderError> {
        Ok(api::txhash_spent(self, txhash)?)
    }

    async fn addresses_exist(
        &self,
        address: &Vec<&str>,
    ) -> Result<Vec<bool>, DataProviderError> {
        Ok(Vec::new())
    }

    async fn tx_history(
        &self,
        addresses: &Vec<&str>,
        slot: Option<u64>,
    ) -> Result<Vec<TxHistoryListView>, DataProviderError>
    {
        Ok(Vec::new())
    }

    async fn retrieve_staked_amount (
        &self,
        epoch: i32,
        stake_addr: &str,
    ) -> Result<BigDecimal, DataProviderError> {
        Ok(api::retrieve_staked_amount(self, epoch, stake_addr)?)
    }

    async fn retrieve_generated_rewards (
        &self,
        stake_addr: &str,
    ) -> Result<Vec<RewardView>, DataProviderError> {
        Ok(api::retrieve_generated_rewards(self, stake_addr)?)
    }

    async fn pool_vrf_key_hash(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError> {
        Ok(api::pool_vrf_key_hash(self, pool_hash)?)
    }

    async fn pool_blocks_minted(
        &self,
        pool_hash: &str,
    ) -> Result<i64, DataProviderError> {
        Ok(api::pool_blocks_minted(self, pool_hash)?)
    }

    async fn pool_blocks_current_epoch(
        &self,
        pool_hash: &str,
    ) -> Result<i64, DataProviderError> {
        Ok(api::pool_blocks_current_epoch(self, pool_hash)?)
    }

    async fn pool_reward_recipients(
        &self, 
        pool_hash: &str,
    ) -> Result<i64, DataProviderError> {
        Ok(api::pool_reward_recipients(self, pool_hash)?)
    }

    async fn pool_last_reward_earned_epoch(
        &self, 
        pool_hash: &str,
    ) -> Result<i64, DataProviderError> {
        Ok(api::pool_last_reward_earned_epoch(self, pool_hash)?)
    }

    async fn pool_declared_pledge(
        &self, 
        pool_hash: &str,
    ) -> Result<BigDecimal, DataProviderError> {
        Ok(api::pool_declared_pledge(self, pool_hash)?)
    }

    async fn pool_margin_cost(
        &self, 
        pool_hash: &str,
    ) -> Result<f64, DataProviderError> {
        Ok(api::pool_margin_cost(self, pool_hash)?)
    }

    async fn pool_fixed_cost(
        &self, 
        pool_hash: &str,
    ) -> Result<BigDecimal, DataProviderError> {
        Ok(api::pool_fixed_cost(self, pool_hash)?)
    }

    async fn pool_owner(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError> {
        Ok(api::pool_owner(self, pool_hash)?)
    }

    async fn pool_registration(
        &self,
        pool_hash: &str,
    ) -> Result<i64, DataProviderError> {
        Ok(api::pool_registration(self, pool_hash)?)
    }

    async fn pool_retirement(
        &self,
        pool_hash: &str,
    ) -> Result<i32, DataProviderError> {
        Ok(api::pool_retirement(self, pool_hash)?)
    }

    async fn pool_url(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError> {
        Ok(api::pool_url(self, pool_hash)?)
    }

    async fn pool_ticker(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError> {
        Ok(api::pool_ticker(self, pool_hash)?)
    }

    async fn pool_metadata_json(
        &self,
        pool_hash: &str,
    ) -> Result<Value, DataProviderError> {
        Ok(api::pool_metadata_json(self, pool_hash)?)
    }

    async fn pool_name(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError> {
        Ok(api::pool_name(self, pool_hash)?)
    }

    async fn pool_homepage(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError> {
        Ok(api::pool_homepage(self, pool_hash)?)
    }

    async fn pool_description(
        &self,
        pool_hash: &str,
    ) -> Result<String, DataProviderError> {
        Ok(api::pool_description(self, pool_hash)?)
    }
}
