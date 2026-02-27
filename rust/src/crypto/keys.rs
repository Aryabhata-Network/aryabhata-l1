use zeroize::Zeroize;

// Dilithium5 key sizes (ML-DSA NIST standard)
pub const DILITHIUM5_PUBLIC_KEY_SIZE: usize  = 2592;
pub const DILITHIUM5_SECRET_KEY_SIZE: usize  = 4864;
pub const DILITHIUM5_SIGNATURE_SIZE: usize   = 4595;

// Kyber1024 key sizes (ML-KEM NIST standard)
pub const KYBER1024_PUBLIC_KEY_SIZE: usize   = 1568;
pub const KYBER1024_SECRET_KEY_SIZE: usize   = 3168;
pub const KYBER1024_CIPHERTEXT_SIZE: usize   = 1568;
pub const KYBER1024_SHARED_SECRET_SIZE: usize = 32;

pub struct PublicKey {
    bytes: [u8; DILITHIUM5_PUBLIC_KEY_SIZE],
}

pub struct SecretKey {
    bytes: [u8; DILITHIUM5_SECRET_KEY_SIZE],
}

pub struct Signature {
    bytes: [u8; DILITHIUM5_SIGNATURE_SIZE],
    len: usize,
}

impl SecretKey {
    pub fn from_bytes(bytes: [u8; DILITHIUM5_SECRET_KEY_SIZE]) -> Self {
        Self { bytes }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Drop for SecretKey {
    fn drop(&mut self) {
        // Critical: wipe secret key from memory immediately
        self.bytes.zeroize();
    }
}

impl PublicKey {
    pub fn from_bytes(bytes: [u8; DILITHIUM5_PUBLIC_KEY_SIZE]) -> Self {
        Self { bytes }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

pub struct KyberSecretKey {
    bytes: [u8; KYBER1024_SECRET_KEY_SIZE],
}

pub struct KyberSharedSecret {
    bytes: [u8; KYBER1024_SHARED_SECRET_SIZE],
}

impl Drop for KyberSecretKey {
    fn drop(&mut self) {
        self.bytes.zeroize();
    }
}

impl Drop for KyberSharedSecret {
    fn drop(&mut self) {
        self.bytes.zeroize();
    }
}
