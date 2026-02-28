pub mod message;
pub mod peer;

pub use message::{Message, MessageType};
pub use peer::{PeerInfo, PeerSet};

pub const MIN_PEERS: usize = 8;
pub const MAX_PEERS: usize = 24;

pub struct Network {
    pub peers: PeerSet,
}

impl Network {
    pub fn new() -> Self {
        Self {
            peers: PeerSet::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.peers.has_minimum()
    }

    pub fn handle_message(&self, msg: &Message) {
        if !msg.is_known_type() {
            return;
        }
    }
}
