//! Source-level Rust translation boundary for the BPF verifier implementation.
//!
//! The implementation depends on the Linux kernel declarations, generated
//! BPF type tables, and macro environment supplied by the surrounding build.
//! The complete isolated source is retained verbatim as an inert Rust raw
//! string until those external declarations are available to lower it.

pub const VERIFIER_C_SOURCE: &str = include_str!("verifier.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
