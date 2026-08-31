/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * random definitions for NOLIBC
 * Copyright (C) 2025 Thomas Weißschuh <thomas.weissschuh@linutronix.de>
 */

/* make sure to include all global symbols */
/* C dependencies: "../nolibc.h", "../arch.h", "../sys.h", <linux/random.h> */

/*
 * ssize_t getrandom(void *buf, size_t buflen, unsigned int flags);
 */

#[allow(non_camel_case_types)]
pub type ssize_t = isize;

#[allow(non_camel_case_types)]
pub type size_t = usize;

unsafe extern "C" {
    static __NR_getrandom: core::ffi::c_long;

    fn __nolibc_syscall3(
        nr: core::ffi::c_long,
        arg1: *mut core::ffi::c_void,
        arg2: size_t,
        arg3: core::ffi::c_uint,
    ) -> ssize_t;

    fn __sysret(ret: ssize_t) -> ssize_t;
}

#[allow(dead_code)]
unsafe fn _sys_getrandom(
    buf: *mut core::ffi::c_void,
    buflen: size_t,
    flags: core::ffi::c_uint,
) -> ssize_t {
    unsafe { __nolibc_syscall3(__NR_getrandom, buf, buflen, flags) }
}

#[allow(dead_code)]
unsafe fn getrandom(
    buf: *mut core::ffi::c_void,
    buflen: size_t,
    flags: core::ffi::c_uint,
) -> ssize_t {
    unsafe { __sysret(unsafe { _sys_getrandom(buf, buflen, flags) }) }
}
