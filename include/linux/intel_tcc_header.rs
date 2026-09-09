/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  header for Intel TCC (thermal control circuitry) library
 *
 *  Copyright (C) 2022  Intel Corporation.
 */

// Dependency equivalent of <linux/types.h> is supplied externally.

extern "C" {
    pub fn intel_tcc_get_tjmax(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn intel_tcc_get_offset(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn intel_tcc_set_offset(
        cpu: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn intel_tcc_get_temp(
        cpu: ::core::ffi::c_int,
        temp: *mut ::core::ffi::c_int,
        pkg: bool,
    ) -> ::core::ffi::c_int;
    pub fn intel_tcc_get_offset_mask() -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
