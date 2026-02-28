pub mod pow;
pub mod scheduler;

pub use pow::{PowPuzzle, PowSolution, PowResult};
pub use scheduler::{MiningScheduler, SchedulerAction};

pub const TIME_MIN_SECONDS: u64 = 45;
pub const TIME_MAX_SECONDS: u64 = 180;
pub const MIN_CNS_TO_MINE: f64  = 0.60;
pub const THERMAL_LIMIT: u8     = 42;

pub struct SunyaMiner {
    pub scheduler: MiningScheduler,
    pub cns_score: f64,
}

impl SunyaMiner {
    pub fn new(low_power: bool, cns: f64) -> Self {
        Self {
            scheduler: MiningScheduler::new(low_power),
            cns_score: cns,
        }
    }

    pub fn can_submit_block(
        &self,
        work_valid: bool,
        time_valid: bool,
    ) -> bool {
        work_valid
            && time_valid
            && self.cns_score >= MIN_CNS_TO_MINE
    }
}
