use serde::{Deserialize, Serialize};

use crate::provider::error::DataProviderError;

/// Well-known magic for testnet
pub const TESTNET_MAGIC: u64 = 1097911063;

/// Well-known magic for mainnet
pub const MAINNET_MAGIC: u64 = 764824073;

/// Well-known magic for preview
pub const PREVIEW_MAGIC: u64 = 2;

/// Well-known magic for pre-production
pub const PRE_PRODUCTION_MAGIC: u64 = 1;

/// Well-known information about the blockhain network
///
/// Some of the logic in Scrolls depends on particular characteristic of the
/// network that it's consuming from. For example: time calculation and bech32
/// encoding. This struct groups all of these blockchain network specific
/// values.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChainWellKnownInfo {
    pub magic: u64,
    pub byron_epoch_length: u32,
    pub byron_slot_length: u32,
    pub byron_known_slot: u64,
    pub byron_known_hash: String,
    pub byron_known_time: u64,
    pub shelley_epoch_length: u32,
    pub shelley_slot_length: u32,
    pub shelley_known_slot: u64,
    pub shelley_known_hash: String,
    pub shelley_known_time: u64,
    pub address_network_id: u8,
    pub adahandle_policy: String,
}

impl ChainWellKnownInfo {
    /// Hardcoded values for mainnet
    pub fn mainnet() -> Self {
        ChainWellKnownInfo {
            magic: MAINNET_MAGIC,
            byron_epoch_length: 432000,
            byron_slot_length: 20,
            byron_known_slot: 0,
            byron_known_time: 1506203091,
            byron_known_hash: "f0f7892b5c333cffc4b3c4344de48af4cc63f55e44936196f365a9ef2244134f"
                .to_string(),
            shelley_epoch_length: 432000,
            shelley_slot_length: 1,
            shelley_known_slot: 4492800,
            shelley_known_hash: "aa83acbf5904c0edfe4d79b3689d3d00fcfc553cf360fd2229b98d464c28e9de"
                .to_string(),
            shelley_known_time: 1596059091,
            address_network_id: 1,
            adahandle_policy: "f0ff48bbb7bbe9d59a40f1ce90e9e9d0ff5002ec48f232b49ca0fb9a"
                .to_string(),
        }
    }

    /// Hardcoded values for testnet
    pub fn testnet() -> Self {
        ChainWellKnownInfo {
            magic: TESTNET_MAGIC,
            byron_epoch_length: 432000,
            byron_slot_length: 20,
            byron_known_slot: 0,
            byron_known_time: 1564010416,
            byron_known_hash: "8f8602837f7c6f8b8867dd1cbc1842cf51a27eaed2c70ef48325d00f8efb320f"
                .to_string(),
            shelley_epoch_length: 432000,
            shelley_slot_length: 1,
            shelley_known_slot: 1598400,
            shelley_known_hash: "02b1c561715da9e540411123a6135ee319b02f60b9a11a603d3305556c04329f"
                .to_string(),
            shelley_known_time: 1595967616,
            address_network_id: 0,
            adahandle_policy: "8d18d786e92776c824607fd8e193ec535c79dc61ea2405ddf3b09fe3"
                .to_string(),
        }
    }

    pub fn preview() -> Self {
        ChainWellKnownInfo {
            magic: PREVIEW_MAGIC,
            byron_epoch_length: 432000,
            byron_slot_length: 20,
            byron_known_slot: 0,
            byron_known_hash: "".to_string(),
            byron_known_time: 1660003200,
            shelley_epoch_length: 432000,
            shelley_slot_length: 1,
            shelley_known_slot: 25260,
            shelley_known_hash: "cac921895ef5f2e85f7e6e6b51b663ab81b3605cd47d6b6d66e8e785e5c65011"
                .to_string(),
            shelley_known_time: 1660003200,
            address_network_id: 0,
            adahandle_policy: "".to_string(),
        }
    }

    /// Hardcoded values for the "pre-prod" testnet
    pub fn preprod() -> Self {
        ChainWellKnownInfo {
            magic: PRE_PRODUCTION_MAGIC,
            byron_epoch_length: 432000,
            byron_slot_length: 20,
            byron_known_slot: 0,
            byron_known_hash: "9ad7ff320c9cf74e0f5ee78d22a85ce42bb0a487d0506bf60cfb5a91ea4497d2"
                .to_string(),
            byron_known_time: 1654041600,
            shelley_epoch_length: 432000,
            shelley_slot_length: 1,
            shelley_known_slot: 86400,
            shelley_known_hash: "c4a1595c5cc7a31eda9e544986fe9387af4e3491afe0ca9a80714f01951bbd5c"
                .to_string(),
            shelley_known_time: 1654041600,
            address_network_id: 0,
            adahandle_policy: "".to_string(),
        }
    }

    /// Uses the value of the magic to return either mainnet or testnet
    /// hardcoded values.
    pub fn try_from_magic(magic: u64) -> Result<ChainWellKnownInfo, DataProviderError> {
        match magic {
            MAINNET_MAGIC => Ok(Self::mainnet()),
            TESTNET_MAGIC => Ok(Self::testnet()),
            PREVIEW_MAGIC => Ok(Self::preview()),
            PRE_PRODUCTION_MAGIC => Ok(Self::preprod()),
            _ => Err(DataProviderError::Custom(
                "can't infer well-known chain infro from specified magic".into(),
            )),
        }
    }
}

impl Default for ChainWellKnownInfo {
    fn default() -> Self {
        Self::mainnet()
    }
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ChainConfig {
    Mainnet,
    Testnet,
    PreProd,
    Preview,
    Custom(u64),
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self::Mainnet
    }
}

impl From<ChainConfig> for ChainWellKnownInfo {
    fn from(other: ChainConfig) -> Self {
        match other {
            ChainConfig::Mainnet => ChainWellKnownInfo::mainnet(),
            ChainConfig::Testnet => ChainWellKnownInfo::testnet(),
            ChainConfig::PreProd => ChainWellKnownInfo::preprod(),
            ChainConfig::Preview => ChainWellKnownInfo::preview(),
            ChainConfig::Custom(x) => ChainWellKnownInfo::try_from_magic(x).unwrap(),
        }
    }
}

#[derive(Deserialize)]
pub struct ConfigRoot {
    pub appconfigs: appconfigs::Config,
    pub connectivity: connectivity::Config,
}

impl ConfigRoot {
    pub fn new(explicit_file: &Option<std::path::PathBuf>) -> Result<Self, config::ConfigError> {
        let mut s = config::Config::builder();

        // our base config will always be in /etc/scrolls
        s = s.add_source(config::File::with_name("/usr/bin/config.toml").required(false));

        // but we can override it by having a file in the working dir
        s = s.add_source(config::File::with_name("config.toml").required(false));

        // if an explicit file was passed, then we load it as mandatory
        if let Some(explicit) = explicit_file.as_ref().and_then(|x| x.to_str()) {
            s = s.add_source(config::File::with_name(explicit).required(true));
        }

        // finally, we use env vars to make some last-step overrides
        s = s.add_source(config::Environment::with_prefix("CONFIG").separator("_"));

        s.build()?.try_deserialize()
    }

    pub fn set_as_env(&self) {
        match self.appconfigs.clone() {
            appconfigs::Config::EarthNode(x) => std::env::set_var("ENNFT_POLICY", x.ennft_policy),
        }
        std::env::set_var("DBSYNC_URL", &self.connectivity.dbsync_url);
        std::env::set_var("TX_SUBMIT_ENDPOINT1", &self.connectivity.submit_endpoint_1);
        std::env::set_var("TX_SUBMIT_ENDPOINT2", &self.connectivity.submit_endpoint_2);
        std::env::set_var("TX_SUBMIT_ENDPOINT3", &self.connectivity.submit_endpoint_3);
        std::env::set_var("PPPATH", self.connectivity.protocoal_parameter_path.clone());
        std::env::set_var("JWT_PUB_KEY", self.connectivity.cert_pub_key.clone());
    }
}

mod connectivity {
    use serde::Deserialize;
    #[derive(Deserialize)]
    pub struct Config {
        pub dbsync_url: String,
        pub submit_endpoint_1: String,
        pub submit_endpoint_2: String,
        pub submit_endpoint_3: String,
        pub protocoal_parameter_path: String,
        pub cert_private_key: Option<String>,
        pub cert_pub_key: String,
    }
}

mod appconfigs {
    use super::earthnode;
    use serde::Deserialize;
    #[derive(Deserialize, Clone)]
    pub enum Config {
        EarthNode(earthnode::Config),
    }
}

mod earthnode {
    use serde::Deserialize;
    #[derive(Deserialize, Clone)]
    pub struct Config {
        pub ennft_policy: String,
    }
}
