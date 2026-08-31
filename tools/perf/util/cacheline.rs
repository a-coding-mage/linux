// SPDX-License-Identifier: GPL-2.0

// Translated from cacheline.c. C dependencies:
// #include "cacheline.h"
// #include <unistd.h>

#[cfg(_SC_LEVEL1_DCACHE_LINESIZE)]
extern "C" {
    fn sysconf(name: libc::c_int) -> libc::c_long;
}

#[cfg(_SC_LEVEL1_DCACHE_LINESIZE)]
extern "C" {
    static _SC_LEVEL1_DCACHE_LINESIZE: libc::c_int;
}

#[cfg(not(_SC_LEVEL1_DCACHE_LINESIZE))]
extern "C" {
    fn sysfs__read_int(path: *const libc::c_char, value: *mut libc::c_int) -> libc::c_int;
    fn pr_debug(fmt: *const libc::c_char, ...);
}

#[cfg(_SC_LEVEL1_DCACHE_LINESIZE)]
unsafe fn cache_line_size(cacheline_sizep: *mut libc::c_int) {
    *cacheline_sizep = sysconf(_SC_LEVEL1_DCACHE_LINESIZE) as libc::c_int;
}

#[cfg(not(_SC_LEVEL1_DCACHE_LINESIZE))]
unsafe fn cache_line_size(cacheline_sizep: *mut libc::c_int) {
    if sysfs__read_int(
        b"devices/system/cpu/cpu0/cache/index0/coherency_line_size\0".as_ptr()
            as *const libc::c_char,
        cacheline_sizep,
    ) != 0
    {
        pr_debug(b"cannot determine cache line size\0".as_ptr() as *const libc::c_char);
    }
}

#[no_mangle]
pub unsafe extern "C" fn cacheline_size() -> libc::c_int {
    static mut SIZE: libc::c_int = 0;

    if SIZE == 0 {
        cache_line_size(&raw mut SIZE);
    }

    SIZE
}
