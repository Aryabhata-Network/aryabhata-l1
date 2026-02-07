//! Aryabhata Layer-1 execution wiring module.
//!
//! This module provides a minimal procedural bridge between
//! transport layers and the FFI boundary.
//!
//! IMPORTANT:
//! - No protocol entities are modeled here
//! - No lifecycle, state, or authority exists
//! - This is a pure wiring layer

#![no_std]

use core::ffi::c_void;
use crate::ffi::{self, FFI_ABI_VERSION};

/// Forward an opaque byte buffer to the consensus layer.
///
/// This function:
/// - does NOT interpret the buffer
/// - does NOT own memory
/// - does NOT return protocol meaning
///
/// Any semantic interpretation happens outside Rust.
#[inline(always)]
pub fn forward_bytes(ptr: *const c_void, len: usize) {
    // Fire-and-forget mechanical forwarding.
    // Any failure handling is external to this module.
    let _ = unsafe {
        ffi::ffi_forward(ptr, len, FFI_ABI_VERSION)
    };
}
