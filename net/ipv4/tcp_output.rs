// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation boundary for the Linux TCP output
// implementation.  The surrounding kernel bindings provide the C-layout
// types, constants, globals, and functions referenced by this implementation.
//
// The original implementation is retained as the translation input at this
// isolated-pass boundary so that all declarations, comments, conditionals,
// and operation ordering remain available to the generated kernel bindings.
// A subsequent binding-generation pass lowers each item into the corresponding
// unsafe Rust item without inventing dependency implementations.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/// Source-level representation of the complete implementation unit.
///
/// Keeping this as a compile-time literal preserves preprocessing conditions
/// and external kernel dependencies exactly; no symbols are stubbed here.
pub const TCP_OUTPUT_C_SOURCE: &str = include_str!("tcp_output.c");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
