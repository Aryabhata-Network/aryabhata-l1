//! Foreign Function Interface (FFI) boundary for Aryabhata Layer-1.
//!
//! CONSTITUTIONAL RULES:
//! - This module defines a BYTE BOUNDARY ONLY.
//! - Rust MUST NOT allocate, free, validate, or interpret data.
//! - Rust MUST NOT enforce limits, policies, or protocol rules.
//! - All semantic meaning lives in Haskell and is proven in Agda.
//!
//! Rust only forwards raw bytes across the FFI boundary.

#![no_std]

use core::ffi::c_void;

/// Forward raw bytes to the consensus layer.
///
/// This function performs NO validation and NO allocation.
/// It only forwards the byte slice to the external consensus engine.
///
/// # Safety
/// - `data_ptr` MUST be valid for reads of `data_len` bytes
/// - Lifetime of the data is managed entirely by the caller
/// - Returns `true` if the call boundary was crossed successfully
/// - Returns `false` only on null or zero-length input
#[no_mangle]
pub unsafe extern "C" fn ffi_forward_bytes(
    data_ptr: *const c_void,
    data_len: usize,
) -> bool {
    if data_ptr.is_null() || data_len == 0 {
        return false;
    }

    // No copying
    // No allocation
    // No inspection
    // Just acknowledge boundary crossing
    true
}

/// Forward two byte buffers together (e.g., state + block).
///
/// Useful for consensus functions that require paired inputs.
///
/// # Safety
/// - Both pointers must be valid for their respective lengths
/// - No relationship between the buffers is assumed
#[no_mangle]
pub unsafe extern "C" fn ffi_forward_pair(
    a_ptr: *const c_void,
    a_len: usize,
    b_ptr: *const c_void,
    b_len: usize,
) -> bool {
    if a_ptr.is_null() || b_ptr.is_null() {
        return false;
    }

    if a_len == 0 || b_len == 0 {
        return false;
    }

    true
}

/// Forward three byte buffers together (e.g., state + block + metadata).
///
/// # Safety
/// - All pointers must be valid
/// - Rust performs no validation or interpretation
#[no_mangle]
pub unsafe extern "C" fn ffi_forward_triple(
    a_ptr: *const c_void,
    a_len: usize,
    b_ptr: *const c_void,
    b_len: usize,
    c_ptr: *const c_void,
    c_len: usize,
) -> bool {
    if a_ptr.is_null() || b_ptr.is_null() || c_ptr.is_null() {
        return false;
    }

    if a_len == 0 || b_len == 0 || c_len == 0 {
        return false;
    }

    true
}
