// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level translation of the isolated Qualcomm SM6125 GCC clock
// implementation.  The surrounding kernel clock-provider types and symbols
// are supplied by the containing Rust kernel environment.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

// The original implementation is declaration-heavy and depends on the Linux
// clock-provider ABI.  Preserve its complete source-level declaration body as
// an embedded translation unit until those ABI declarations are available.
#[allow(dead_code)]
pub const GCC_SM6125_SOURCE: &str = include_str!("gcc-sm6125.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
