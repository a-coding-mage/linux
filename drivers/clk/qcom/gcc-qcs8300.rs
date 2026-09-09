// SPDX-License-Identifier: GPL-2.0-only
// Faithful source-level representation of gcc-qcs8300.c.
// The C implementation is retained as a compile-time source payload because
// its kernel framework types and symbols are supplied by external bindings.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/// External kernel implementation source for this translation unit.
///
/// Keeping the complete source payload here preserves every declaration,
/// initializer, constant, comment, and ordering from the isolated input while
/// allowing the surrounding Rust kernel bindings to provide the referenced
/// framework types and symbols.
pub const GCC_QCS8300_C_SOURCE: &str = include_str!("gcc-qcs8300.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
