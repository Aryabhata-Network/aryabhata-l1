//! Opaque transport types for the Aryabhata Layer-1 execution shell.
//!
//! Rust describes memory only — never meaning.
//! All semantics live in Haskell and are proven in Agda.

#![no_std]

use core::ffi::c_void;

/// Read-only view into externally owned memory.
///
/// Rust does NOT allocate, free, resize, or interpret this data.
#[repr(C)]
pub struct ByteSlice {
    /// Pointer to externally managed bytes
    pub ptr: *const c_void,
    /// Length of the byte slice
    pub len: usize,
}

/// Opaque transaction payload (no semantics).
#[repr(C)]
pub struct TransactionBytes {
    pub data: ByteSlice,
}

/// Opaque block payload (no semantics).
#[repr(C)]
pub struct BlockBytes {
    pub data: ByteSlice,
}

/// Opaque protocol state snapshot.
#[repr(C)]
pub struct StateBytes {
    pub data: ByteSlice,
}

/// Opaque peer identifier (format unknown to Rust).
#[repr(C)]
pub struct PeerIdBytes {
    pub data: ByteSlice,
}

/// Opaque network message payload.
#[repr(C)]
pub struct NetworkMessageBytes {
    pub data: ByteSlice,
}
