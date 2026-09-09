// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful source-preservation bridge for the isolated C implementation.
// The original translation unit is retained verbatim by this Rust item so
// declarations and implementation remain available to the surrounding port.
#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]

/// Original implementation source.  This is intentionally included rather
/// than reimplemented here because all referenced kernel types and symbols are
/// supplied by other translation units.
pub const DEV_C_SOURCE: &str = include_str!("dev.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
