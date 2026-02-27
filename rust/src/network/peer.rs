// Peer management
// Diversity scoring prevents eclipse attacks
// Max 24 peers, min 8 peers

pub const MIN_PEERS: usize = 8;
pub const MAX_PEERS: usize = 24;
pub const PEER_EXCHANGE_INTERVAL_SECS: u64 = 60;
pub const PING_INTERVAL_SECS: u64 = 30;
pub const CNS_CACHE_EPOCHS: u64 = 10;

#[derive(Clone)]
pub struct PeerAddress {
    pub ip: [u8; 4],
    pub port: u16,
}

#[derive(Clone)]
pub struct PeerInfo {
    pub address: PeerAddress,
    pub cns_score: f64,
    pub asn: u32,
    pub is_active: bool,
    pub last_seen: u64,
}

impl PeerInfo {
    pub fn new(ip: [u8; 4], port: u16, asn: u32) -> Self {
        Self {
            address: PeerAddress { ip, port },
            cns_score: 0.0,
            asn,
            is_active: false,
            last_seen: 0,
        }
    }

    pub fn is_alive(&self, current_time: u64) -> bool {
        current_time - self.last_seen < PING_INTERVAL_SECS * 3
    }
}

pub struct PeerSet {
    peers: [Option<PeerInfo>; MAX_PEERS],
    count: usize,
}

impl PeerSet {
    pub fn new() -> Self {
        const NONE: Option<PeerInfo> = None;
        Self {
            peers: [NONE; MAX_PEERS],
            count: 0,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn has_minimum(&self) -> bool {
        self.count >= MIN_PEERS
    }

    pub fn can_add(&self) -> bool {
        self.count < MAX_PEERS
    }

    // Diversity score — penalise same ASN peers
    // Prevents eclipse attacks
    pub fn asn_diversity_score(&self) -> f64 {
        if self.count == 0 {
            return 1.0;
        }
        let mut asns = [0u32; MAX_PEERS];
        let mut asn_count = 0;
        for peer in self.peers.iter().flatten() {
            if !asns[..asn_count].contains(&peer.asn) {
                asns[asn_count] = peer.asn;
                asn_count += 1;
            }
        }
        asn_count as f64 / self.count as f64
    }
}
