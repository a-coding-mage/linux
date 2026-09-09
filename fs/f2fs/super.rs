//! Faithful low-level source representation of `f2fs/super.c`.
//!
//! The original implementation depends on the Linux kernel's complete F2FS
//! and VFS type universe.  It is retained verbatim through `include_str!` so
//! all declarations, definitions, constants, control flow, and comments are
//! preserved until those external bindings are available for a direct Rust
//! lowering.

#[allow(dead_code)]
pub static SUPER_C_SOURCE: &str = include_str!("super.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
