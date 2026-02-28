use sha3::{Digest, Sha3_256, Sha3_512};
use blake3::Hasher as Blake3Hasher;

pub const HASH_SIZE_256: usize = 32;
pub const HASH_SIZE_512: usize = 64;

pub struct Hash256(pub [u8; HASH_SIZE_256]);
pub struct Hash512(pub [u8; HASH_SIZE_512]);

impl Hash256 {
    pub fn sha3_256(data: &[u8]) -> Self {
        let mut hasher = Sha3_256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut bytes = [0u8; HASH_SIZE_256];
        bytes.copy_from_slice(&result);
        Self(bytes)
    }

    pub fn blake3(data: &[u8]) -> Self {
        let mut hasher = Blake3Hasher::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut bytes = [0u8; HASH_SIZE_256];
        bytes.copy_from_slice(result.as_bytes());
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; HASH_SIZE_256] {
        &self.0
    }

    pub fn meets_difficulty(&self, difficulty: u32) -> bool {
        self.count_leading_zero_bits() >= difficulty
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
        self.0.iter_mut().for_each(|b| *b = 0);
    }
}

impl Hash512 {
    pub fn sha3_512(data: &[u8]) -> Self {
        let mut hasher = Sha3_512::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut bytes = [0u8; HASH_SIZE_512];
        bytes.copy_from_slice(&result);
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
