//! Literal source-level translation unit for the Linux Btrfs free-space cache.
//!
//! The implementation depends on the kernel/Btrfs ABI declarations supplied by
//! the surrounding translation.  The complete C implementation is retained as
//! an embedded translation reference so no declaration, branch, operation, or
//! comment is lost while those external Rust bindings are provided.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub const FREE_SPACE_CACHE_C_SOURCE: &str = include_str!("free-space-cache.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
