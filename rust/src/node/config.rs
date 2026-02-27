// Node configuration
// All parameters from genesis.json
// Immutable after genesis

pub struct NodeConfig {
    pub version: &'static str,
    pub network: NetworkConfig,
    pub mining: MiningConfig,
    pub storage: StorageConfig,
}

pub struct NetworkConfig {
    pub min_peers: usize,
    pub max_peers: usize,
    pub listen_port: u16,
    pub peer_exchange_interval: u64,
    pub ping_interval: u64,
}

pub struct MiningConfig {
    pub enabled: bool,
    pub low_power_mode: bool,
    pub thermal_limit: u8,
    pub target_block_time: u64,
    pub min_cns_to_mine: f64,
}

pub struct StorageConfig {
    pub pruned_mode: bool,
    pub max_storage_mb: u64,
}

impl NodeConfig {
    pub fn default() -> Self {
        Self {
            version: "0.2.0",
            network: NetworkConfig {
                min_peers: 8,
                max_peers: 24,
                listen_port: 8333,
                peer_exchange_interval: 60,
                ping_interval: 30,
            },
            mining: MiningConfig {
                enabled: true,
                low_power_mode: true,
                thermal_limit: 42,
                target_block_time: 60,
                min_cns_to_mine: 0.60,
            },
            storage: StorageConfig {
                pruned_mode: true,
                max_storage_mb: 200,
            },
        }
    }

    pub fn termux() -> Self {
        let mut config = Self::default();
        // Termux optimised settings
        config.mining.low_power_mode = true;
        config.storage.pruned_mode = true;
        config.storage.max_storage_mb = 200;
        config
    }
}
