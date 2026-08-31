/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Mount definitions for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols */
/* C dependency: "../nolibc.h" */

/* Header guard _NOLIBC_SYS_MOUNT_H omitted in Rust. */

/* C dependencies: "../sys.h", <linux/mount.h> */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

unsafe extern "C" {
    fn __nolibc_syscall5(
        nr: c_long,
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: *const c_char,
        arg4: c_ulong,
        arg5: *const c_void,
    ) -> c_long;

    fn __sysret(arg: c_long) -> c_int;
}

/*
 * int mount(const char *source, const char *target,
 *           const char *fstype, unsigned long flags,
 *           const void *data);
 */
#[allow(dead_code)]
pub unsafe fn _sys_mount(
    src: *const c_char,
    tgt: *const c_char,
    fst: *const c_char,
    flags: c_ulong,
    data: *const c_void,
) -> c_int {
    unsafe { __nolibc_syscall5(__NR_mount, src, tgt, fst, flags, data) as c_int }
}

#[allow(dead_code)]
pub unsafe fn mount(
    src: *const c_char,
    tgt: *const c_char,
    fst: *const c_char,
    flags: c_ulong,
    data: *const c_void,
) -> c_int {
    unsafe { __sysret(_sys_mount(src, tgt, fst, flags, data) as c_long) }
}
