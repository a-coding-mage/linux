/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Ioctl definitions for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols */
/* C dependencies: "../nolibc.h", "../sys.h", <linux/ioctl.h> */

/*
 * int ioctl(int fd, unsigned long cmd, ... arg);
 */

#[allow(dead_code)]
pub unsafe fn _sys_ioctl(fd: ::core::ffi::c_uint, cmd: ::core::ffi::c_uint, arg: ::core::ffi::c_ulong) -> ::core::ffi::c_long {
    unsafe { __nolibc_syscall3(__NR_ioctl, fd, cmd, arg) }
}

/* C macro: #define ioctl(fd, cmd, arg) __sysret(_sys_ioctl(fd, cmd, (unsigned long)(arg))) */
#[allow(dead_code)]
pub unsafe fn ioctl(fd: ::core::ffi::c_uint, cmd: ::core::ffi::c_uint, arg: ::core::ffi::c_ulong) -> ::core::ffi::c_long {
    unsafe { __sysret(_sys_ioctl(fd, cmd, arg as ::core::ffi::c_ulong)) }
}
