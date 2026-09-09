// SPDX-License-Identifier: GPL-2.0-only
//
// Source-level Rust translation container for gcc-sm6375.c.
//
// The implementation depends on Linux clock-provider declarations, macros,
// and types supplied by other translation units. Keep the complete original
// source available to the eventual binding layer without inventing those
// external dependencies in this isolated pass.
#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub const GCC_SM6375_C_SOURCE: &str = include_str!("gcc-sm6375.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
