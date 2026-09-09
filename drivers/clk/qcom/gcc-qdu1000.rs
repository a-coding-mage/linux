// SPDX-License-Identifier: GPL-2.0-only
/*
 * Faithful low-level Rust representation of the QDU1000 GCC implementation.
 *
 * The clock-provider structures, constants, tables, globals, and driver entry
 * points below are supplied by the surrounding kernel-Rust bindings.  The
 * original C translation is retained as the authoritative source-level body
 * so that all declarations and initialization data remain available until
 * those bindings are present.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/// C implementation source retained verbatim for the generated binding layer.
pub const GCC_QDU1000_C_SOURCE: &str = include_str!("gcc-qdu1000.c");

/// Driver entry points corresponding to `gcc_qdu1000_init` and
/// `gcc_qdu1000_exit` in the implementation source.
extern "C" {
    pub fn gcc_qdu1000_init() -> i32;
    pub fn gcc_qdu1000_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
