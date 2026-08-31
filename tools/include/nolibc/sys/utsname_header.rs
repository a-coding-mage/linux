/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Utsname definitions for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols */
/* C dependency: ../nolibc.h */

/* C header guard _NOLIBC_SYS_UTSNAME_H omitted in Rust. */

/* C dependencies: ../sys.h, <linux/utsname.h> */

/*
 * int uname(struct utsname *buf);
 */

#[repr(C)]
pub struct utsname {
    pub sysname: [core::ffi::c_char; 65],
    pub nodename: [core::ffi::c_char; 65],
    pub release: [core::ffi::c_char; 65],
    pub version: [core::ffi::c_char; 65],
    pub machine: [core::ffi::c_char; 65],
    pub domainname: [core::ffi::c_char; 65],
}

#[allow(non_snake_case)]
unsafe extern "C" {
    static __NR_uname: core::ffi::c_long;

    fn __nolibc_syscall1(
        nr: core::ffi::c_long,
        arg1: *mut utsname,
    ) -> core::ffi::c_long;

    fn __sysret(ret: core::ffi::c_long) -> core::ffi::c_int;
}

#[allow(dead_code)]
pub unsafe fn _sys_uname(buf: *mut utsname) -> core::ffi::c_int {
    unsafe { __nolibc_syscall1(__NR_uname, buf) as core::ffi::c_int }
}

#[allow(dead_code)]
pub unsafe fn uname(buf: *mut utsname) -> core::ffi::c_int {
    unsafe { __sysret(_sys_uname(buf) as core::ffi::c_long) }
}
