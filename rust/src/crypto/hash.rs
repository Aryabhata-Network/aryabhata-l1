// SHA3-256 — SUNYA-PoW puzzle
// SHA3-512 — Block hash, transaction hash
// BLAKE3   — Merkle tree
// No MD5, no SHA1, no SHA2 — ever

pub const HASH_SIZE_256: usize = 32;
pub const HASH_SIZE_512: usize = 64;

pub struct Hash256([u8; HASH_SIZE_256]);
pub struct Hash512([u8; HASH_SIZE_512]);

impl Hash256 {
    pub fn from_bytes(bytes: [u8; HASH_SIZE_256]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; HASH_SIZE_256] {
        &self.0
    }

    // Check if hash meets difficulty target
    // Leading zeros method
    pub fn meets_difficulty(&self, difficulty: u32) -> bool {
        let leading_zeros = self.count_leading_zero_bits();
        leading_zeros >= difficulty
    }

    fn count_leading_zero_bits(&self) -> u32 {
        let mut count = 0u32;
        for byte in self.0.iter() {
            if *byte == 0 {
                count += 8;
            } else {
                count += byte.leading_zeros();
                break;
            }
        }
        count
    }
}

impl Drop for Hash256 {
    fn drop(&mut self) {
        // Wipe hash from memory
        self.0.iter_mut().for_each(|b| *b = 0);
    }
}

impl Hash512 {
    pub fn from_bytes(bytes: [u8; HASH_SIZE_512]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; HASH_SIZE_512] {
        &self.0
    }
}

impl Drop for Hash512 {
    fn drop(&mut self) {
        self.0.iter_mut().for_each(|b| *b = 0);
    }
}
