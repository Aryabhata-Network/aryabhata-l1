// SUNYA-PoW — Work condition implementation
// SHA3-256 based puzzle
// Mobile-optimised: low-power mode reduces iterations

use crate::crypto::hash::Hash256;

pub const MAX_NONCE: u64 = u64::MAX;

pub struct PowPuzzle {
    pub block_header: [u8; 80],
    pub difficulty: u32,
    pub low_power_mode: bool,
}

pub struct PowSolution {
    pub nonce: u64,
    pub hash: [u8; 32],
}

pub enum PowResult {
    Solved(PowSolution),
    ThermalThrottle,
    Exhausted,
}

impl PowPuzzle {
    pub fn new(
        header: [u8; 80],
        difficulty: u32,
        low_power: bool,
    ) -> Self {
        Self {
            block_header: header,
            difficulty,
            low_power_mode: low_power,
        }
    }

    // Batch size — smaller on low power mode
    // Saves battery on mobile devices
    pub fn batch_size(&self) -> u64 {
        if self.low_power_mode {
            1_000
        } else {
            10_000
        }
    }

    // Combine block header with nonce for hashing
    pub fn build_input(&self, nonce: u64) -> [u8; 88] {
        let mut input = [0u8; 88];
        input[..80].copy_from_slice(&self.block_header);
        input[80..].copy_from_slice(&nonce.to_le_bytes());
        input
    }
}
