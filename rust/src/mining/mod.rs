// SUNYA-PoW Miner
// Three conditions must ALL pass:
// 1. WORK    — SHA3-256 hash meets target difficulty
// 2. TIME    — timestamp in [parent+45s, parent+180s]
// 3. SILENCE — miner CNS score >= 0.60

pub const TIME_MIN_SECONDS: u64 = 45;
pub const TIME_MAX_SECONDS: u64 = 180;
pub const MIN_CNS_TO_MINE: f64 = 0.60;
pub const THERMAL_LIMIT_CELSIUS: u8 = 42;
pub const LOW_POWER_HASH_REDUCTION: f64 = 0.30;

pub struct SunyaMiner {
    pub low_power_mode: bool,
    pub thermal_limit: u8,
    pub cns_score: f64,
}

impl SunyaMiner {
    pub fn new(low_power: bool, cns: f64) -> Self {
        Self {
            low_power_mode: low_power,
            thermal_limit: THERMAL_LIMIT_CELSIUS,
            cns_score: cns,
        }
    }

    // Check all three SUNYA-PoW conditions
    pub fn can_submit_block(
        &self,
        work_valid: bool,
        time_valid: bool,
    ) -> BlockSubmitResult {
        if !work_valid {
            return BlockSubmitResult::InvalidWork;
        }
        if !time_valid {
            return BlockSubmitResult::InvalidTime;
        }
        if self.cns_score < MIN_CNS_TO_MINE {
            return BlockSubmitResult::InvalidSilence;
        }
        BlockSubmitResult::Valid
    }

    // Thermal protection — pause mining if device too hot
    pub fn thermal_check(&self, current_temp: u8) -> bool {
        current_temp < self.thermal_limit
    }
}

pub enum BlockSubmitResult {
    Valid,
    InvalidWork,
    InvalidTime,
    InvalidSilence,
}
