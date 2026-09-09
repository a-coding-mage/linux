/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Functions to change memory attributes.
 */
extern "C" {
    pub fn set_memory_ro(addr: core::ffi::c_ulong, numpages: core::ffi::c_int) -> core::ffi::c_int;
    pub fn set_memory_rw(addr: core::ffi::c_ulong, numpages: core::ffi::c_int) -> core::ffi::c_int;
    pub fn set_memory_x(addr: core::ffi::c_ulong, numpages: core::ffi::c_int) -> core::ffi::c_int;
    pub fn set_memory_nx(addr: core::ffi::c_ulong, numpages: core::ffi::c_int) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
