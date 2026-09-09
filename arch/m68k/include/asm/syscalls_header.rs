/* SPDX-License-Identifier: GPL-2.0-only */

// Translated from the m68k syscall header.
// `asmlinkage` specifies the calling convention in the C build; these
// declarations use the C ABI here.

use core::ffi::{c_int, c_ulong};

extern "C" {
    pub fn sys_cacheflush(addr: c_ulong, scope: c_int, cache: c_int, len: c_ulong) -> c_int;
    pub fn sys_atomic_cmpxchg_32(
        newval: c_ulong,
        oldval: c_int,
        d3: c_int,
        d4: c_int,
        d5: c_int,
        mem: *mut c_ulong,
    ) -> c_int;
    pub fn sys_getpagesize() -> c_int;
    pub fn sys_get_thread_area() -> c_ulong;
    pub fn sys_set_thread_area(tp: c_ulong) -> c_int;
    pub fn sys_atomic_barrier() -> c_int;
}

// The declarations from <asm-generic/syscalls.h> are supplied by the
// corresponding translated dependency.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
