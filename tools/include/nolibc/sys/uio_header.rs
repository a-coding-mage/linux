/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * uio for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 * Copyright (C) 2025 Intel Corporation
 */

/* make sure to include all global symbols */
/* C dependency: ../nolibc.h */

/* C header guard _NOLIBC_SYS_UIO_H omitted in Rust. */

/* C dependencies:
 * ../sys.h
 * <linux/uio.h>
 */

/*
 * ssize_t readv(int fd, const struct iovec *iovec, int count);
 */
#[allow(dead_code)]
pub unsafe fn _sys_readv(fd: core::ffi::c_int, iovec: *const iovec, count: core::ffi::c_int) -> ssize_t {
    unsafe { __nolibc_syscall3(__NR_readv, fd, iovec, count) as ssize_t }
}

#[allow(dead_code)]
pub unsafe fn readv(fd: core::ffi::c_int, iovec: *const iovec, count: core::ffi::c_int) -> ssize_t {
    unsafe { __sysret(_sys_readv(fd, iovec, count)) as ssize_t }
}

/*
 * ssize_t writev(int fd, const struct iovec *iovec, int count);
 */
#[allow(dead_code)]
pub unsafe fn _sys_writev(fd: core::ffi::c_int, iovec: *const iovec, count: core::ffi::c_int) -> ssize_t {
    unsafe { __nolibc_syscall3(__NR_writev, fd, iovec, count) as ssize_t }
}

#[allow(dead_code)]
pub unsafe fn writev(fd: core::ffi::c_int, iovec: *const iovec, count: core::ffi::c_int) -> ssize_t {
    unsafe { __sysret(_sys_writev(fd, iovec, count)) as ssize_t }
}
