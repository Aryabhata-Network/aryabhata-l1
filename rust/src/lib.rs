//! ARYABHATA LAYER-1 EXECUTION SHELL
//!
//! This crate provides execution, networking, and data-transport
//! infrastructure for Aryabhata nodes.
//!
//! CONSTITUTIONAL CONSTRAINTS:
//! 1. NO consensus authority
//! 2. NO cryptographic authority
//! 3. NO time authority
//! 4. NO state transition logic
//! 5. NO protocol invariant enforcement
//!
//! All protocol truth is defined in Haskell and proven in Agda.
//! Rust is a pure execution shell.

#![no_std]
#![forbid(
    unsafe_code,
    unstable_features,
    unsafe_op_in_unsafe_fn
)]
#![deny(
    missing_docs,
    deprecated,
    unused_import_braces,
    unused_qualifications
)]

extern crate alloc;

// ============================================================================
// PUBLIC MODULES
// ============================================================================
pub mod ffi;
pub mod node;
pub mod types;

// ============================================================================
// EXECUTION-ONLY MARKER
// ============================================================================

/// Marker trait for execution-only components.
///
/// Implementations MUST NOT:
/// - Validate signatures
/// - Interpret protocol rules
/// - Enforce invariants
/// - Depend on system time
///
/// Any Rust component violating these constraints
/// is NOT a valid Aryabhata implementation.
pub trait ExecutionShell {}

// ============================================================================
// EXECUTION ERRORS (NON-CONSENSUS)
// ============================================================================

/// Errors related to execution mechanics only.
///
/// These errors do NOT imply protocol invalidity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum ExecutionError {
    /// Network transport failure
    NetworkFailure,

    /// Persistent storage failure
    StorageFailure,

    /// Memory allocation failure
    MemoryFailure,

    /// Foreign Function Interface failure
    FFIFailure,

    /// Malformed data (transport-level)
    InvalidEncoding,
}

// ============================================================================
// PRELUDE (DATA TRANSPORT ONLY)
// ============================================================================

/// Common opaque types for cross-layer transport.
///
/// Rust MUST NOT interpret the contents of these types.
pub mod prelude {
    pub use crate::types::{
        Block,
        Transaction,
        StateSnapshot,
    };

    pub use crate::ffi::{
        consensus_check,
    };
}

// ============================================================================
// PROTOCOL IDENTIFIERS (NON-AUTHORITATIVE)
// ============================================================================

/// Protocol identifier string.
///
/// Informational only. Not used for consensus.
pub const PROTOCOL_NAME: &str = "ARYABHATA-L1";

/// Execution ABI version.
///
/// Must match the Haskell FFI layer.
pub const EXECUTION_ABI_VERSION: u32 = 1;
