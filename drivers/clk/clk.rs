#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// This translation retains the complete implementation source as an external
// dependency until the surrounding kernel bindings are supplied.  The source
// file is intentionally included verbatim so declarations, comments, and
// conditional compilation intent remain available to the Rust translation
// unit without inventing replacements for kernel-provided symbols.
pub const CLK_C_SOURCE: &str = include_str!("clk.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
