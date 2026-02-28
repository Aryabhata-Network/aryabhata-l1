use argon2::{Argon2, Params, Version, Algorithm};
use zeroize::Zeroize;

pub const ARGON2_MEMORY_KB: u32    = 262_144;
pub const ARGON2_ITERATIONS: u32   = 4;
pub const ARGON2_PARALLELISM: u32  = 1;
pub const ARGON2_OUTPUT_LEN: usize = 32;

pub struct Argon2Output {
    bytes: [u8; ARGON2_OUTPUT_LEN],
}

impl Drop for Argon2Output {
    fn drop(&mut self) {
        self.bytes.zeroize();
    }
}

impl Argon2Output {
    pub fn as_bytes(&self) -> &[u8; ARGON2_OUTPUT_LEN] {
        &self.bytes
    }
}

pub fn derive_key(
    password: &[u8],
    salt: &[u8; 32],
) -> Result<Argon2Output, ()> {
    let params = Params::new(
        ARGON2_MEMORY_KB,
        ARGON2_ITERATIONS,
        ARGON2_PARALLELISM,
        Some(ARGON2_OUTPUT_LEN),
    ).map_err(|_| ())?;

    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        params,
    );

    let mut output = [0u8; ARGON2_OUTPUT_LEN];
    argon2
        .hash_password_into(password, salt, &mut output)
        .map_err(|_| ())?;

    Ok(Argon2Output { bytes: output })
}
