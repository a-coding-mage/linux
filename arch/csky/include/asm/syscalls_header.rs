/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <asm-generic/syscalls.h> dependency.

use core::ffi::c_void;

extern "C" {
    pub fn sys_cacheflush(
        arg1: *mut c_void,
        arg2: ::core::ffi::c_ulong,
        arg3: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;

    pub fn sys_set_thread_area(addr: ::core::ffi::c_ulong) -> ::core::ffi::c_long;

    pub fn sys_csky_fadvise64_64(
        fd: ::core::ffi::c_int,
        advice: ::core::ffi::c_int,
        offset: loff_t,
        len: loff_t,
    ) -> ::core::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
