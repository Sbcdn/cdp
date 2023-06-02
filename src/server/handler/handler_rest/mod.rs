pub(crate) mod aya;
pub(crate) mod info;

use info::{
    address_exists, handle_asset_for_stake_address, handle_get_asset_for_addresses, is_nft,
    mint_metadata, retrieve_active_pools, token_supply, tx_history, tx_history_discover,
    utxos_per_addr,
};

use rweb::*;

/// Info endpoints
#[router(
    "/info",
    services(
        utxos_per_addr,
        address_exists,
        mint_metadata,
        tx_history_discover,
        tx_history,
        handle_asset_for_stake_address,
        handle_get_asset_for_addresses,
        retrieve_active_pools,
        is_nft,
    )
)]
#[openapi(id = "api.info", description = "Information Requests")]
pub async fn info() {}
