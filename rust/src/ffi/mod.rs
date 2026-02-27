// Haskell FFI Boundary — Safety Contract
// Rule 1: All data = [u32 length] + [u8 bytes]
// Rule 2: No raw pointers — ever
// Rule 3: No shared memory — ever
// Rule 4: Rust owns all buffers
// Rule 5: Haskell reads copies only
// Rule 6: Every call has 500ms deadline
// Rule 7: Version mismatch = hard reject

pub const FFI_PROTOCOL_VERSION: u8 = 2;
pub const FFI_TIMEOUT_MS: u64 = 500;

pub enum FfiError {
    Timeout,
    VersionMismatch,
    InvalidLength,
    InvalidData,
    HaskellPanic,
}

pub struct FfiRequest {
    pub version: u8,
    pub length: u32,
    pub data: [u8; 4096],
}

pub struct FfiResponse {
    pub version: u8,
    pub length: u32,
    pub data: [u8; 4096],
}

impl FfiRequest {
    pub fn new(payload: &[u8]) -> Result<Self, FfiError> {
        if payload.len() > 4096 {
            return Err(FfiError::InvalidLength);
        }
        let mut data = [0u8; 4096];
        data[..payload.len()].copy_from_slice(payload);
        Ok(Self {
            version: FFI_PROTOCOL_VERSION,
            length: payload.len() as u32,
            data,
        })
    }

    // Verify version before any processing
    pub fn verify_version(&self) -> Result<(), FfiError> {
        if self.version != FFI_PROTOCOL_VERSION {
            return Err(FfiError::VersionMismatch);
        }
        Ok(())
    }
}
