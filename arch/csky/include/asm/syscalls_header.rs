/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <asm-generic/syscalls.h> dependency.

use kernel::ffi::c_void;

extern "C" {
    pub fn sys_cacheflush(
        arg1: *mut c_void,
        arg2: ::kernel::ffi::c_ulong,
        arg3: ::kernel::ffi::c_int,
    ) -> ::kernel::ffi::c_long;

    pub fn sys_set_thread_area(addr: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_long;

    pub fn sys_csky_fadvise64_64(
        fd: ::kernel::ffi::c_int,
        advice: ::kernel::ffi::c_int,
        offset: loff_t,
        len: loff_t,
    ) -> ::kernel::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
