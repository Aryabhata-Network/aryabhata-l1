use zeroize::Zeroize;

// Post-quantum signature — CRYSTALS-Dilithium5 (ML-DSA)
// Key material zeroized immediately after use
// No RSA, no ECDSA, no classical cryptography

pub struct Dilithium5Key {
    inner: [u8; 32],
}

impl Dilithium5Key {
    pub fn new(seed: [u8; 32]) -> Self {
        Self { inner: seed }
    }
}

impl Drop for Dilithium5Key {
    fn drop(&mut self) {
        // Key material wiped from memory on drop
        self.inner.zeroize();
    }
}

// Kyber1024 (ML-KEM) — P2P channel encryption
// Quantum-resistant key encapsulation
pub struct Kyber1024Session {
    shared_secret: [u8; 32],
}

impl Drop for Kyber1024Session {
    fn drop(&mut self) {
        self.shared_secret.zeroize();
    }
}

// All hash operations use SHA3-512 or BLAKE3
// SHA3-256 used only for SUNYA-PoW puzzle
pub enum HashAlgorithm {
    Sha3_512,
    Sha3_256,
    Blake3,
}

// Constant-time comparison — prevents timing attacks
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}
