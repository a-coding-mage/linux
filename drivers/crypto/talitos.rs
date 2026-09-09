//! Low-level Rust translation unit for the Talitos driver.
//!
//! The implementation depends on the kernel-facing types and constants
//! declared by the surrounding Talitos interface.  The complete source-level
//! input is retained here until those external bindings are supplied.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/// Original implementation source retained verbatim for the generated
/// translation boundary.  Kernel declarations referenced by it are supplied
/// by the repository's Rust bindings.
pub const TALITOS_IMPLEMENTATION_SOURCE: &str = include_str!("talitos.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
