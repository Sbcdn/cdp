pub(crate) mod aya;
pub(crate) mod info;

use info::{
    address_exists, handle_asset_for_stake_address, handle_get_asset_for_addresses, is_nft,
    mint_metadata, mint_metadata_policy_assetname, retrieve_active_pools, token_supply, tx_history, 
    tx_history_discover, get_address_utxos, retrieve_staked_amount, retrieve_generated_rewards, pool_vrf_key_hash,
    pool_blocks_minted, pool_blocks_current_epoch, pool_reward_recipients, pool_last_reward_earned_epoch, 
    pool_declared_pledge, pool_margin_cost, pool_fixed_cost, pool_owner, 
    pool_registration, pool_retirement, pool_url, pool_ticker, pool_metadata_json, pool_name, 
    pool_homepage, pool_description,
};

use rweb::*;

/// Info endpoints
#[router(
    "/info",
    services(
        get_address_utxos,
        address_exists,
        mint_metadata,
        mint_metadata_policy_assetname,
        tx_history_discover,
        tx_history,
        handle_asset_for_stake_address,
        handle_get_asset_for_addresses,
        retrieve_active_pools,
        token_supply,
        is_nft,
        retrieve_staked_amount,
        retrieve_generated_rewards,
        pool_vrf_key_hash, 
        pool_blocks_minted,
        pool_blocks_current_epoch,
        pool_reward_recipients,
        pool_last_reward_earned_epoch,
        pool_declared_pledge,
        pool_margin_cost,
        pool_fixed_cost,
        pool_owner,
        pool_registration,
        pool_retirement,
        pool_url,
        pool_ticker,
        pool_metadata_json,
        pool_name,
        pool_homepage,
        pool_description,
    )
)]
#[openapi(id = "api.info", description = "Information Requests")]
pub async fn info() {}
