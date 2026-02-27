// FFI Protocol definitions
// All messages between Rust and Haskell
// Version-tagged, length-prefixed, deadline-enforced

pub const VERSION: u8 = 2;
pub const TIMEOUT_MS: u64 = 500;
pub const MAX_PAYLOAD: usize = 65536;

pub enum FfiMessageType {
    ValidateBlock  = 0x01,
    ComputeCns     = 0x02,
    AdjustDiff     = 0x03,
    ValidateTx     = 0x04,
}

pub struct FfiHeader {
    pub version: u8,
    pub msg_type: u8,
    pub length: u32,
}

impl FfiHeader {
    pub fn new(msg_type: FfiMessageType, length: u32) -> Self {
        Self {
            version: VERSION,
            msg_type: msg_type as u8,
            length,
        }
    }

    pub fn is_valid_version(&self) -> bool {
        self.version == VERSION
    }

    pub fn to_bytes(&self) -> [u8; 6] {
        let mut bytes = [0u8; 6];
        bytes[0] = self.version;
        bytes[1] = self.msg_type;
        bytes[2..6].copy_from_slice(&self.length.to_le_bytes());
        bytes
    }
}

pub enum FfiStatus {
    Ok      = 0x00,
    Error   = 0x01,
    Timeout = 0x02,
    Invalid = 0x03,
}
