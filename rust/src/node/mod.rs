pub mod config;
pub mod cns_tracker;
pub mod lifecycle;
pub mod storage;
pub mod treasury;
pub mod wallet;

pub use config::NodeConfig;
pub use lifecycle::{NodeLifecycle, NodeState};

pub struct Node;

impl Node {
    pub fn start() {
        let config = NodeConfig::termux();
        let mut lifecycle = NodeLifecycle::new(config);

        if !lifecycle.has_enough_peers() {
            return;
        }

        if !lifecycle.thermal_ok() {
            return;
        }

        if lifecycle.can_mine() {
            lifecycle.state = NodeState::Mining;
        }
    }
}
