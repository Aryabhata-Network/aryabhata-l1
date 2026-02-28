// Network message types
// All messages padded to fixed frame sizes
// Prevents traffic analysis and AI fingerprinting

pub const FRAME_SMALL: usize  = 512;
pub const FRAME_MEDIUM: usize = 4_096;
pub const FRAME_LARGE: usize  = 65_536;

pub const PING_FRAME_SIZE: usize = 64;

pub enum MessageType {
    BlockPropose  = 0x01,
    BlockAck      = 0x02,
    TxBroadcast   = 0x03,
    PeerExchange  = 0x04,
    CnsProbe      = 0x05,
    Ping          = 0x06,
    Pong          = 0x07,
}

pub struct Message {
    pub msg_type: u8,
    pub payload_len: u32,
    pub payload: [u8; FRAME_MEDIUM],
}

impl Message {
    pub fn new(msg_type: MessageType, data: &[u8]) -> Option<Self> {
        if data.len() > FRAME_MEDIUM {
            return None;
        }
        let mut payload = [0u8; FRAME_MEDIUM];
        payload[..data.len()].copy_from_slice(data);
        Some(Self {
            msg_type: msg_type as u8,
            payload_len: data.len() as u32,
            payload,
        })
    }

    pub fn is_known_type(&self) -> bool {
        matches!(
            self.msg_type,
            0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07
        )
    }
}
