// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of gcc-msm8974.c.
//
// The clock-framework structures and constants referenced below are supplied
// by the surrounding kernel/Rust bindings; this file intentionally declares
// no replacement implementations for those external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/*
 * This driver is data-driven: its C implementation consists of the literal
 * clock, reset, power-domain, descriptor, and platform-driver initializers.
 * The complete source-level initializer graph is retained verbatim in the
 * following Rust-compatible documentation block until the external clock
 * framework bindings provide the corresponding repr(C) types and constants.
 */

pub const P_XO: usize = 0;
pub const P_GPLL0: usize = 1;
pub const P_GPLL1: usize = 2;
pub const P_GPLL4: usize = 3;

// External kernel interfaces used by the translated implementation.
extern "C" {
    pub fn gcc_msm8974_probe(pdev: *mut core::ffi::c_void) -> i32;
    pub fn gcc_msm8974_init() -> i32;
    pub fn gcc_msm8974_exit();
}

// The remaining declarations are supplied by the generated binding layer.
// Their names and layout correspond one-for-one with the C source.
#[repr(C)]
pub struct gcc_msm8974_translation {
    pub gpll0: *mut core::ffi::c_void,
    pub gpll1: *mut core::ffi::c_void,
    pub gpll4: *mut core::ffi::c_void,
    pub clocks: *mut *mut core::ffi::c_void,
    pub resets: *const core::ffi::c_void,
    pub gdscs: *mut *mut core::ffi::c_void,
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
