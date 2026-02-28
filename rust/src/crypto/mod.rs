pub mod argon;
pub mod hash;
pub mod kem;
pub mod keys;
pub mod signing;

use subtle::ConstantTimeEq;

pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.ct_eq(b).into()
}

// Cryptographic stack:
// Signatures : CRYSTALS-Dilithium5 (ML-DSA) — quantum-resistant
// KEM        : CRYSTALS-Kyber1024 (ML-KEM)  — quantum-resistant
// Hash       : SHA3-512 / BLAKE3             — quantum-safe
// PoW Hash   : SHA3-256 (SUNYA variant)      — mining puzzle only
// KDF        : Argon2id                      — wallet key derivation
// All key material zeroized from memory on drop
// No RSA — ever
// No ECDSA — ever
// No SHA2 — ever
// No MD5 — ever

pub const DILITHIUM5_PUBLIC_KEY_BYTES: usize   = 2592;
pub const DILITHIUM5_SECRET_KEY_BYTES: usize   = 4864;
pub const DILITHIUM5_SIGNATURE_BYTES: usize    = 4595;

pub const KYBER1024_PUBLIC_KEY_BYTES: usize    = 1568;
pub const KYBER1024_SECRET_KEY_BYTES: usize    = 3168;
pub const KYBER1024_CIPHERTEXT_BYTES: usize    = 1568;
pub const KYBER1024_SHARED_SECRET_BYTES: usize = 32;

pub const SHA3_256_OUTPUT_BYTES: usize = 32;
pub const SHA3_512_OUTPUT_BYTES: usize = 64;
pub const BLAKE3_OUTPUT_BYTES: usize   = 32;
pub const ARGON2_OUTPUT_BYTES: usize   = 32;
pub const ARGON2_SALT_BYTES: usize     = 32;
pub const ARGON2_MEMORY_KB: u32        = 262_144;
pub const ARGON2_ITERATIONS: u32       = 4;
