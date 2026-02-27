// Node lifecycle management
// States: Init → Syncing → Mining → Suspended → Shutdown
// Thermal protection built in

use crate::node::config::NodeConfig;

pub enum NodeState {
    Init,
    Syncing,
    Mining,
    Suspended,
    Shutdown,
}

pub struct NodeLifecycle {
    pub state: NodeState,
    pub config: NodeConfig,
    pub cns_score: f64,
    pub current_temp: u8,
    pub peer_count: usize,
}

impl NodeLifecycle {
    pub fn new(config: NodeConfig) -> Self {
        Self {
            state: NodeState::Init,
            config,
            cns_score: 0.0,
            current_temp: 30,
            peer_count: 0,
        }
    }

    // Thermal check before each mining batch
    pub fn thermal_ok(&self) -> bool {
        self.current_temp < self.config.mining.thermal_limit
    }

    // CNS check before block submission
    pub fn can_mine(&self) -> bool {
        self.cns_score >= self.config.mining.min_cns_to_mine
            && self.thermal_ok()
    }

    // Minimum peers before mining starts
    pub fn has_enough_peers(&self) -> bool {
        self.peer_count >= self.config.network.min_peers
    }

    // Safe shutdown — no data corruption
    pub fn shutdown(&mut self) {
        self.state = NodeState::Shutdown;
    }
}
