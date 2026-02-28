pub mod protocol;

pub use protocol::{FfiHeader, FfiMessageType, FfiStatus, VERSION};

pub const TIMEOUT_MS: u64    = 500;
pub const MAX_PAYLOAD: usize = 65_536;

pub enum FfiError {
    Timeout,
    VersionMismatch,
    InvalidLength,
    InvalidData,
    HaskellPanic,
}

pub struct FfiRequest {
    pub header: FfiHeader,
    pub data: [u8; 4096],
}

pub struct FfiResponse {
    pub status: u8,
    pub length: u32,
    pub data: [u8; 4096],
}

impl FfiRequest {
    pub fn new(
        msg_type: FfiMessageType,
        payload: &[u8],
    ) -> Result<Self, FfiError> {
        if payload.len() > 4096 {
            return Err(FfiError::InvalidLength);
        }
        let header = FfiHeader::new(msg_type, payload.len() as u32);
        let mut data = [0u8; 4096];
        data[..payload.len()].copy_from_slice(payload);
        Ok(Self { header, data })
    }

    pub fn is_valid(&self) -> bool {
        self.header.is_valid_version()
    }
}
