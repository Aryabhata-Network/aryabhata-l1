//! Foreign Function Interface (FFI) boundary for Aryabhata Layer-1.
//!
//! This module defines a strictly mechanical, versioned, and discoverable
//! byte-transport contract between Rust and the Haskell consensus engine.
//!
//! Rust has ZERO protocol authority.

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

use core::ffi::c_void;

/// FFI ABI version implemented by this Rust binary.
pub const FFI_ABI_VERSION: u32 = 1;

/// Maximum payload size allowed for any FFI call (bytes).
///
/// Mechanical transport limit only (mobile safety).
pub const FFI_MAX_PAYLOAD_BYTES: usize = 16 * 1024 * 1024; // 16 MB

/// Mechanical FFI result codes.
///
/// These codes carry NO protocol meaning.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FfiStatus {
    /// Operation mechanically valid
    Ok = 0,
    /// Null pointer passed
    NullPointer = 1,
    /// Length is zero
    ZeroLength = 2,
    /// Length exceeds allowed transport limit
    LengthExceeded = 3,
    /// ABI version mismatch
    AbiMismatch = 4,
}

/// Internal mechanical validation.
#[inline(always)]
fn validate(ptr: *const c_void, len: usize) -> FfiStatus {
    if ptr.is_null() {
        return FfiStatus::NullPointer;
    }
    if len == 0 {
        return FfiStatus::ZeroLength;
    }
    if len > FFI_MAX_PAYLOAD_BYTES {
        return FfiStatus::LengthExceeded;
    }
    FfiStatus::Ok
}

/// Forward opaque bytes across the FFI boundary.
///
/// # Safety
/// - Caller guarantees pointer validity and lifetime for call duration
/// - Data MUST NOT be mutated concurrently
#[no_mangle]
pub unsafe extern "C" fn ffi_forward(
    ptr: *const c_void,
    len: usize,
    expected_abi: u32,
) -> FfiStatus {
    if expected_abi != FFI_ABI_VERSION {
        return FfiStatus::AbiMismatch;
    }
    validate(ptr, len)
}

/// Query ABI version implemented by Rust.
#[no_mangle]
pub extern "C" fn ffi_query_abi() -> u32 {
    FFI_ABI_VERSION
}

/// Query maximum payload size allowed by Rust execution shell.
#[no_mangle]
pub extern "C" fn ffi_query_max_payload() -> usize {
    FFI_MAX_PAYLOAD_BYTES
  }
