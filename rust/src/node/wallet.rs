// Arya-Vault wallet implementation
// Dilithium5 keypair generation
// Argon2id seed derivation
// Seed phrase displayed once — never stored

use zeroize::Zeroize;
use crate::crypto::keys::{
    PublicKey,
    SecretKey,
    DILITHIUM5_PUBLIC_KEY_SIZE,
    DILITHIUM5_SECRET_KEY_SIZE,
};

pub const SEED_BYTES: usize      = 32;
pub const MNEMONIC_WORDS: usize  = 24;

pub struct WalletSeed {
    bytes: [u8; SEED_BYTES],
}

impl Drop for WalletSeed {
    fn drop(&mut self) {
        // Critical: seed wiped from memory on drop
        self.bytes.zeroize();
    }
}

impl WalletSeed {
    pub fn from_bytes(bytes: [u8; SEED_BYTES]) -> Self {
        Self { bytes }
    }

    pub fn as_bytes(&self) -> &[u8; SEED_BYTES] {
        &self.bytes
    }
}

pub struct Wallet {
    pub public_key: PublicKey,
    pub address: [u8; 32],
}

pub enum WalletError {
    InvalidSeed,
    KeyGenFailed,
}

pub struct TimeLock {
    pub amount: u64,
    pub unlock_epoch: u64,
    pub can_cancel_until: u64,
}

impl TimeLock {
    // Transfers above 100,000 ARY are time-locked
    pub const THRESHOLD: u64 = 100_000;
    pub const LOCK_EPOCHS: u64 = 24;
    pub const CANCEL_WINDOW: u64 = 12;

    pub fn is_required(amount: u64) -> bool {
        amount >= Self::THRESHOLD
    }
}
