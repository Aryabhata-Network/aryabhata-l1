// Storage layer — pruned mode for mobile
// Stores: block headers + UTXO set only
// Full blocks discarded after confirmation
// Max 200 MB on device

pub const MAX_STORAGE_MB: u64 = 200;
pub const BLOCK_HEADER_SIZE: usize = 80;
pub const UTXO_ENTRY_SIZE: usize = 44;

pub struct BlockHeader {
    pub height: u64,
    pub hash: [u8; 32],
    pub parent_hash: [u8; 32],
    pub timestamp: u64,
    pub nonce: u64,
    pub difficulty: u32,
}

pub struct UtxoEntry {
    pub tx_hash: [u8; 32],
    pub output_index: u32,
    pub amount: u64,
    pub owner_pubkey_hash: [u8; 32],
    pub is_spent: bool,
}

pub struct Storage {
    pub pruned_mode: bool,
    pub headers_count: u64,
    pub utxo_count: u64,
}

impl Storage {
    pub fn new(pruned: bool) -> Self {
        Self {
            pruned_mode: pruned,
            headers_count: 0,
            utxo_count: 0,
        }
    }

    // Estimated storage usage in MB
    pub fn estimated_size_mb(&self) -> u64 {
        let headers = self.headers_count
            * BLOCK_HEADER_SIZE as u64
            / 1_048_576;
        let utxos = self.utxo_count
            * UTXO_ENTRY_SIZE as u64
            / 1_048_576;
        headers + utxos
    }

    // Reject storage if limit exceeded
    pub fn has_space(&self) -> bool {
        self.estimated_size_mb() < MAX_STORAGE_MB
    }
}
