/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Functions to change memory attributes.
 */
extern "C" {
    pub fn set_memory_ro(addr: kernel::ffi::c_ulong, numpages: kernel::ffi::c_int) -> kernel::ffi::c_int;
    pub fn set_memory_rw(addr: kernel::ffi::c_ulong, numpages: kernel::ffi::c_int) -> kernel::ffi::c_int;
    pub fn set_memory_x(addr: kernel::ffi::c_ulong, numpages: kernel::ffi::c_int) -> kernel::ffi::c_int;
    pub fn set_memory_nx(addr: kernel::ffi::c_ulong, numpages: kernel::ffi::c_int) -> kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
