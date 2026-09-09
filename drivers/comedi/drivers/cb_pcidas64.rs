// SPDX-License-Identifier: GPL-2.0+
//
// Faithful source-preserving Rust representation of the implementation source.
// The C translation unit and its externally supplied kernel/comedi dependencies
// remain the authoritative source-level interface for this isolated pass.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/// Complete implementation source retained for the translation unit's future
/// dependency-resolution pass.  Keeping this literal also preserves comments,
/// declarations, constants, control-flow text, and conditional intent exactly.
pub const CB_PCIDAS64_C_SOURCE: &str = include_str!("cb_pcidas64.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
