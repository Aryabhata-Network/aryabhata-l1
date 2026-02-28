```rust
// Argon2id — wallet key derivation
// Memory-hard: resistant to GPU and ASIC attacks
// Parameters match protocol specification

pub const ARGON2_MEMORY_KB: u32    = 262_144; // 256 MB
pub const ARGON2_ITERATIONS: u32   = 4;
pub const ARGON2_PARALLELISM: u32  = 1;
pub const ARGON2_OUTPUT_LEN: usize = 32;
pub const ARGON2_SALT_LEN: usize   = 32;

pub struct Argon2Params {
    pub memory_kb: u32,
    pub iterations: u32,
    pub parallelism: u32,
    pub output_len: usize,
}

impl Argon2Params {
    pub fn protocol_standard() -> Self {
        Self {
            memory_kb:   ARGON2_MEMORY_KB,
            iterations:  ARGON2_ITERATIONS,
            parallelism: ARGON2_PARALLELISM,
            output_len:  ARGON2_OUTPUT_LEN,
        }
    }
}

pub struct Argon2Output {
    bytes: [u8; ARGON2_OUTPUT_LEN],
}

impl Drop for Argon2Output {
    fn drop(&mut self) {
        // Wipe derived key from memory
        self.bytes.iter_mut().for_each(|b| *b = 0);
    }
}

impl Argon2Output {
    pub fn from_bytes(bytes: [u8; ARGON2_OUTPUT_LEN]) -> Self {
        Self { bytes }
    }

    pub fn as_bytes(&self) -> &[u8; ARGON2_OUTPUT_LEN] {
        &self.bytes
    }
}
