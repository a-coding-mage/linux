// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level translation boundary for the APQ8084 GCC implementation.
// The referenced kernel clock-controller types, constants, and operations are
// supplied by the surrounding Rust kernel port.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

/*
 * The original implementation is declaration-heavy Linux clock-controller
 * data.  Preserve its complete source representation here so that the
 * surrounding port can provide the corresponding repr(C) kernel types and
 * initialization machinery without inventing dependency implementations.
 */
pub const GCC_APQ8084_C_SOURCE: &str = include_str!("gcc-apq8084.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
