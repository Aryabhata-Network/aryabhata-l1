// CNS tracker — on-chain behavioral score
// Tracks last 100 epochs
// Cannot be faked or gamed

pub const EPOCH_HISTORY: usize = 100;
pub const MAX_RESTARTS: f64    = 10.0;

pub struct CnsTracker {
    pub uptime_history: [bool; EPOCH_HISTORY],
    pub restart_count: u32,
    pub propagation_delays: [u64; EPOCH_HISTORY],
    pub epoch_index: usize,
    pub is_founding_node: bool,
}

impl CnsTracker {
    pub fn new(founding: bool) -> Self {
        Self {
            uptime_history: [false; EPOCH_HISTORY],
            restart_count: 0,
            propagation_delays: [0u64; EPOCH_HISTORY],
            epoch_index: 0,
            is_founding_node: founding,
        }
    }

    pub fn uptime_ratio(&self) -> f64 {
        let online = self.uptime_history
            .iter()
            .filter(|&&x| x)
            .count();
        online as f64 / EPOCH_HISTORY as f64
    }

    pub fn restart_stability(&self) -> f64 {
        let r = self.restart_count as f64;
        f64::max(0.0, 1.0 - r / MAX_RESTARTS)
    }

    pub fn propagation_score(&self) -> f64 {
        let sum: u64 = self.propagation_delays.iter().sum();
        let avg = sum as f64 / EPOCH_HISTORY as f64;
        f64::max(0.0, 1.0 - avg / 180.0)
    }

    pub fn compute_cns(&self) -> f64 {
        let u = self.uptime_ratio();
        let r = self.restart_stability();
        let p = self.propagation_score();
        let base = 0.60 * u + 0.25 * r + 0.15 * p;

        // Founding node gets permanent +0.05 bonus
        if self.is_founding_node {
            f64::min(1.0, base + 0.05)
        } else {
            base
        }
    }

    pub fn can_mine(&self) -> bool {
        self.compute_cns() >= 0.60
    }

    pub fn record_epoch(&mut self, online: bool, delay: u64) {
        let i = self.epoch_index % EPOCH_HISTORY;
        self.uptime_history[i] = online;
        self.propagation_delays[i] = delay;
        self.epoch_index += 1;
    }

    pub fn record_restart(&mut self) {
        self.restart_count += 1;
    }
}
