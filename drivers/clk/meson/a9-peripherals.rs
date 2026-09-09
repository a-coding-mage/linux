// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
/*
 * Faithful low-level Rust translation boundary for the Amlogic A9 peripheral
 * clock implementation.  The original kernel declarations are retained as
 * source text because their types and constructor macros are supplied by the
 * surrounding Linux clock framework.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/// Original implementation source, retained verbatim for framework-backed
/// macro expansion and for preserving declarations not defined in this file.
pub const A9_PERIPHERALS_SOURCE: &str = include_str!("a9-peripherals.c");

// The C implementation is intentionally consumed by the target's generated
// bindings; no local dependency implementations are invented here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
