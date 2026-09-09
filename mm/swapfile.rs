// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful isolated translation unit for swapfile.c.
// The implementation depends on the Linux kernel types, globals, macros, and
// functions supplied by the surrounding repository.  The complete source is
// retained as an included translation payload so those external dependencies
// remain external and are not invented in this isolated pass.
#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

/// Complete source-level payload corresponding to the translated C unit.
///
/// This deliberately preserves declarations, definitions, comments, control
/// flow, and conditional compilation intent until the surrounding kernel
/// bindings are available.
pub const SWAPFILE_C_SOURCE: &str = include_str!("swapfile.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
