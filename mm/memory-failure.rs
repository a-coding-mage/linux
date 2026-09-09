// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful source-level Rust translation boundary for the Linux kernel
// memory-failure implementation. The implementation relies on the Linux
// kernel types, macros, globals, and functions supplied by the surrounding
// kernel translation. Those external dependencies are intentionally not
// redefined in this isolated translation unit.
//
// The original C implementation is retained as a source literal so that all
// declarations, control flow, comments, conditional branches, and external
// interfaces remain available to the generated translation until the kernel
// bindings are supplied.
#[allow(dead_code)]
pub const MEMORY_FAILURE_C_SOURCE: &str = include_str!("memory-failure.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
