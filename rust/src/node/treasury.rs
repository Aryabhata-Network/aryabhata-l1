// Protocol Fee Treasury tracker
// 2% of every transaction fee
// Time-locked, multisig, transparent

pub const TREASURY_FEE_PERCENT: f64  = 0.02;
pub const TIMELOCK_EPOCHS: u64       = 90;
pub const MAX_SPEND_PERCENT: f64     = 0.10;
pub const MULTISIG_REQUIRED: usize   = 3;
pub const MULTISIG_TOTAL: usize      = 5;

pub struct Treasury {
    pub balance: u64,
    pub total_accumulated: u64,
    pub last_withdrawal_epoch: u64,
}

impl Treasury {
    pub fn new() -> Self {
        Self {
            balance: 0,
            total_accumulated: 0,
            last_withdrawal_epoch: 0,
        }
    }

    // 2% of fee goes to treasury automatically
    pub fn accumulate(&mut self, fee: u64) {
        let share = (fee as f64 * TREASURY_FEE_PERCENT) as u64;
        self.balance += share;
        self.total_accumulated += share;
    }

    // Max 10% of balance per 90 epoch period
    pub fn max_withdrawal(&self) -> u64 {
        (self.balance as f64 * MAX_SPEND_PERCENT) as u64
    }

    // Check if withdrawal is allowed
    pub fn can_withdraw(
        &self,
        amount: u64,
        current_epoch: u64,
    ) -> bool {
        let time_ok = current_epoch
            >= self.last_withdrawal_epoch + TIMELOCK_EPOCHS;
        let amount_ok = amount <= self.max_withdrawal();
        time_ok && amount_ok
    }

    // If no block in 365 days, burn treasury
    pub fn should_burn(&self, last_block_epoch: u64, current_epoch: u64) -> bool {
        current_epoch - last_block_epoch > 365
    }
}
