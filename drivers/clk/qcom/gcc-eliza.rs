// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust representation of the Qualcomm Eliza GCC
// implementation.  The source uses kernel-provided C ABI structures and
// operations; those external dependencies are intentionally not redefined.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/*
 * This translation retains the complete implementation source as an embedded
 * source-level record because the declarations below are supplied by the
 * Linux clock-controller ABI.  A kernel Rust binding can expose these items
 * directly while preserving the original layout, ordering, and initializers.
 */
pub const GCC_ELIZA_C_SOURCE: &str = include_str!("gcc-eliza.c");

// External kernel ABI entry points used by the implementation.
unsafe extern "C" {
    pub fn qcom_cc_probe(pdev: *mut core::ffi::c_void, desc: *const core::ffi::c_void) -> i32;
    pub fn platform_driver_register(driver: *mut core::ffi::c_void) -> i32;
    pub fn platform_driver_unregister(driver: *mut core::ffi::c_void);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
