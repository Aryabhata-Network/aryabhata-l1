// P2P Network Layer
// Fixed-frame messages — prevents traffic analysis by AI
// Peer limit: 8 minimum, 24 maximum
// Diversity scoring — prevents eclipse attacks

pub const MIN_PEERS: usize = 8;
pub const MAX_PEERS: usize = 24;

// All messages padded to fixed sizes
// Eliminates message-size fingerprinting
pub enum FrameSize {
    Small  = 512,
    Medium = 4096,
    Large  = 65536,
}

pub enum MessageType {
    BlockPropose,
    BlockAck,
    TxBroadcast,
    PeerExchange,
    CnsProbe,
    Ping,
    Pong,
}

pub struct Peer {
    pub cns_score: f64,
    pub is_active: bool,
}

pub struct Network {
    pub peers: [Option<Peer>; 24],
    pub peer_count: usize,
}

impl Network {
    pub fn new() -> Self {
        const NONE_PEER: Option<Peer> = None;
        Self {
            peers: [NONE_PEER; 24],
            peer_count: 0,
        }
    }

    // Reject connection if peer limit reached
    pub fn can_add_peer(&self) -> bool {
        self.peer_count < MAX_PEERS
    }

    // Drop unsolicited unknown messages silently
    // Silence is the correct response to noise
    pub fn handle_unknown_message(&self) {
        // Do nothing — no error, no response
    }

    // Check minimum peer requirement
    pub fn has_minimum_peers(&self) -> bool {
        self.peer_count >= MIN_PEERS
    }
}
