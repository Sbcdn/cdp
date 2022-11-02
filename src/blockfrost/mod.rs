use self::error::DataProviderBlockfrostError;
use async_trait::async_trait;
use blockfrost::{load, BlockFrostApi};

pub mod api;
pub mod error;
pub mod models;

pub struct Config {
    pub url: String,
    pub api_key: String,
    pub ipfs_url: Option<String>,
    pub ipfs_api_key: Option<String>,
}

pub struct BlockfrostProvider {
    config: Config,
    api: BlockFrostApi,
}

unsafe impl Send for BlockfrostProvider {}
unsafe impl Sync for BlockfrostProvider {}

impl BlockfrostProvider {
    pub fn new(config: Config) -> Self {
        let api = BlockfrostProvider::build_api(&config).unwrap();
        BlockfrostProvider { config, api }
    }

    fn build_api(config: &Config) -> blockfrost::Result<BlockFrostApi> {
        let mut configurations = load::configurations_from_env()?;

        configurations["project_id"] = toml::value::Value::from(config.api_key.clone());
        configurations["blockfrost-network-address"] = toml::value::Value::from(config.url.clone());

        let api = BlockFrostApi::new(
            configurations["project_id"].as_str().unwrap(),
            Default::default(),
        );

        Ok(api)
    }
}

#[async_trait]
impl super::provider::CardanoDataProvider for BlockfrostProvider {
    async fn alive(&self) -> bool {
        self.api.health().await.is_ok()
    }

    async fn wallet_utxos(
        &self,
        stake_addr: &str,
    ) -> Result<
        drasil_csl_common::TransactionUnspentOutputs,
        crate::provider::error::DataProviderError,
    > {
        Ok(api::get_stake_address_utxos(self, stake_addr).await?)
    }

    async fn script_utxos(
        &self,
        addr: &str,
    ) -> Result<
        drasil_csl_common::TransactionUnspentOutputs,
        crate::provider::error::DataProviderError,
    > {
        Ok(api::get_address_utxos(self, addr).await?)
    }

    async fn asset_utxos_on_addr(
        &self,
        addr: &str,
    ) -> Result<
        drasil_csl_common::TransactionUnspentOutputs,
        crate::provider::error::DataProviderError,
    > {
        Ok(api::asset_utxos_on_addr(self, addr).await?)
    }

    async fn mint_metadata(
        &self,
        fingerprint_in: &str,
    ) -> Result<crate::models::TokenInfoView, crate::provider::error::DataProviderError> {
        Ok(api::mint_metadata(self, fingerprint_in)?)
    }

    async fn first_transaction_from_stake_addr(
        &self,
        stake_address_in: &str,
    ) -> Result<
        cardano_serialization_lib::address::Address,
        crate::provider::error::DataProviderError,
    > {
        let str_addr = api::select_addr_of_first_transaction(self, stake_address_in)?;
        Ok(drasil_csl_common::addr_from_str(&str_addr)?)
    }

    async fn utxo_tokens(
        &self,
        utxo_id: i64,
    ) -> Result<Vec<crate::models::CardanoNativeAssetView>, crate::provider::error::DataProviderError>
    {
        Ok(api::get_utxo_tokens(self, utxo_id)?)
    }

    async fn slot(&self) -> Result<i64, crate::provider::error::DataProviderError> {
        Ok(api::slot(self).await?)
    }

    async fn stakers_on_pool(
        &self,
        pool: &str,
        epoch: i32,
    ) -> Result<Vec<crate::models::StakeDelegationView>, crate::provider::error::DataProviderError>
    {
        Ok(api::pool_delegations(self, pool, epoch)?)
    }

    async fn deligations_per_pool_epoch_intervall(
        &self,
        pool: &str,
        start_epoch: i64,
        end_epoch: i64,
    ) -> Result<Vec<crate::models::DelegationView>, crate::provider::error::DataProviderError> {
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
    ) -> Result<u64, crate::provider::error::DataProviderError> {
        Ok(api::pool_total_stake(self, pool, epoch)?)
    }

    async fn current_epoch(&self) -> Result<i32, crate::provider::error::DataProviderError> {
        Ok(api::current_epoch(self).await?)
    }

    async fn fingerprint(
        &self,
        policy: &str,
        tokenname: &str,
    ) -> Result<String, crate::provider::error::DataProviderError> {
        Ok(api::fingerprint(self, policy, tokenname)?)
    }

    async fn token_info(
        &self,
        fingerprint_in: &str,
    ) -> Result<crate::models::TokenInfoView, crate::provider::error::DataProviderError> {
        Ok(api::token_info(self, fingerprint_in)?)
    }

    async fn stake_registration(
        &self,
        stake_addr_in: &str,
    ) -> Result<Vec<crate::models::StakeRegistrationView>, crate::provider::error::DataProviderError>
    {
        Ok(api::stake_registration(self, stake_addr_in)?)
    }

    async fn stake_deregistration(
        &self,
        stake_addr_in: &str,
    ) -> Result<
        Vec<crate::models::StakeDeregistrationView>,
        crate::provider::error::DataProviderError,
    > {
        Ok(api::stake_deregistration(self, stake_addr_in)?)
    }

    async fn check_stakeaddr_registered(
        &self,
        stake_addr_in: &str,
    ) -> Result<bool, crate::provider::error::DataProviderError> {
        Ok(api::check_stakeaddr_registered(self, stake_addr_in)?)
    }

    async fn lookup_token_holders(
        &self,
        fingerprint_in: &str,
        min_amount: Option<&i64>,
    ) -> Result<Vec<crate::models::HoldingWalletView>, crate::provider::error::DataProviderError>
    {
        Ok(api::lookup_token_holders(self, fingerprint_in, min_amount)?)
    }

    async fn lookup_nft_token_holders(
        &self,
        policy: &str,
    ) -> Result<Vec<crate::models::HoldingWalletView>, crate::provider::error::DataProviderError>
    {
        Ok(api::lookup_nft_token_holders(self, policy)?)
    }

    async fn pool_valid(
        &self,
        pool_id: &str,
    ) -> Result<bool, crate::provider::error::DataProviderError> {
        Ok(api::pool_valid(self, pool_id)?)
    }

    async fn txhash_spent(
        &self,
        txhash: &str,
    ) -> Result<bool, crate::provider::error::DataProviderError> {
        Ok(api::txhash_spent(self, txhash)?)
    }
}
