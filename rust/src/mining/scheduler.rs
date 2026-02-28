Mining scheduler
// Manages thermal throttle and low power mode
// Termux-safe: no OS-specific calls

pub const BATCH_SIZE_NORMAL: u64    = 10_000;
pub const BATCH_SIZE_LOW_POWER: u64 = 1_000;
pub const THERMAL_LIMIT: u8         = 42;
pub const THERMAL_PAUSE_SECS: u64   = 30;

pub enum SchedulerAction {
    Mine,
    Pause,
    ThermalThrottle,
    CnsInsufficient,
}

pub struct MiningScheduler {
    pub low_power_mode: bool,
    pub current_temp: u8,
    pub cns_score: f64,
}

impl MiningScheduler {
    pub fn new(low_power: bool) -> Self {
        Self {
            low_power_mode: low_power,
            current_temp: 30,
            cns_score: 0.0,
        }
    }

    pub fn next_action(&self) -> SchedulerAction {
        if self.current_temp >= THERMAL_LIMIT {
            return SchedulerAction::ThermalThrottle;
        }
        if self.cns_score < 0.60 {
            return SchedulerAction::CnsInsufficient;
        }
        SchedulerAction::Mine
    }

    pub fn batch_size(&self) -> u64 {
        if self.low_power_mode {
            BATCH_SIZE_LOW_POWER
        } else {
            BATCH_SIZE_NORMAL
        }
    }
