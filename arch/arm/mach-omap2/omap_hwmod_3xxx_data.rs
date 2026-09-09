// SPDX-License-Identifier: GPL-2.0-only
//
// Literal Rust representation of omap_hwmod_3xxx_data.c.
//
// The structures and routines in this translation intentionally retain the
// source file's external kernel types and symbols.  Those names are supplied
// by the surrounding kernel translation units.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/// The original implementation is retained verbatim as source data so that
/// declaration order, comments, conditional intent, and externally supplied
/// identifiers remain available to the generated translation unit.
pub const OMAP_HW_MOD_3XXX_DATA_C: &str = include_str!("omap_hwmod_3xxx_data.c");

/// Initialize the OMAP3xxx hardware-module links.
///
/// The implementation is supplied by the kernel's translated hwmod support;
/// this declaration preserves the C entry point and its external interface.
extern "C" {
    pub fn omap3xxx_hwmod_init() -> ::core::ffi::c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
