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

    pub fn batch_size(&self) -> u64 {
        if self.low_power_mode { 1_000 } else { 10_000 }
    }

    pub fn build_input(&self, nonce: u64) -> [u8; 88] {
        let mut input = [0u8; 88];
        input[..80].copy_from_slice(&self.block_header);
        input[80..].copy_from_slice(&nonce.to_le_bytes());
        input
    }

    pub fn try_nonce(&self, nonce: u64) -> Option<PowSolution> {
        let input = self.build_input(nonce);
        let hash = Hash256::sha3_256(&input);
        if hash.meets_difficulty(self.difficulty) {
            Some(PowSolution {
                nonce,
                hash: *hash.as_bytes(),
            })
        } else {
            None
        }
    }

    pub fn mine_batch(
        &self,
        start_nonce: u64,
        thermal_ok: bool,
    ) -> PowResult {
        if !thermal_ok {
            return PowResult::ThermalThrottle;
        }
        let end = start_nonce.saturating_add(self.batch_size());
        for nonce in start_nonce..end {
            if let Some(solution) = self.try_nonce(nonce) {
                return PowResult::Solved(solution);
            }
        }
        PowResult::Exhausted
    }
}
