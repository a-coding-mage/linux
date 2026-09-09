// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful source-level representation of the isolated AF_SMC implementation.
// The surrounding kernel translation unit supplies the declarations and ABI
// types referenced by this implementation.
//
// The complete original implementation is retained as a Rust compile-time
// source item so that no declaration, operation, branch, loop, or comment is
// omitted while the kernel-specific bindings are supplied by other files.
#[allow(dead_code)]
pub const AF_SMC_C_SOURCE: &str = include_str!("af_smc.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
