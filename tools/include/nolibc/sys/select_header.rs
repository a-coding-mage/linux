/* SPDX-License-Identifier: LGPL-2.1 OR MIT */

// Original C dependencies:
// #include "../nolibc.h"
// #include <linux/time.h>
// #include <linux/unistd.h>

use core::ffi::{c_int, c_long, c_ulong, c_void};

/* commonly an fd_set represents 256 FDs */
pub const FD_SETSIZE: usize = 256;

pub const FD_SETIDXMASK: usize = 8 * core::mem::size_of::<c_ulong>();
pub const FD_SETBITMASK: usize = 8 * core::mem::size_of::<c_ulong>() - 1;

/* for select() */
#[repr(C)]
pub struct fd_set {
    pub fds: [c_ulong; (FD_SETSIZE + FD_SETBITMASK) / FD_SETIDXMASK],
}

pub unsafe fn FD_CLR(fd: c_int, set: *mut fd_set) {
    let __set: *mut fd_set = set;
    let __fd: c_int = fd;
    if __fd >= 0 {
        unsafe {
            (*__set).fds[(__fd as usize) / FD_SETIDXMASK] &=
                !((1_u32.wrapping_shl(((__fd as usize) & FD_SETBITMASK) as u32)) as c_ulong);
        }
    }
}

pub unsafe fn FD_SET(fd: c_int, set: *mut fd_set) {
    let __set: *mut fd_set = set;
    let __fd: c_int = fd;
    if __fd >= 0 {
        unsafe {
            (*__set).fds[(__fd as usize) / FD_SETIDXMASK] |=
                (1_i32.wrapping_shl(((__fd as usize) & FD_SETBITMASK) as u32)) as c_ulong;
        }
    }
}

pub unsafe fn FD_ISSET(fd: c_int, set: *mut fd_set) -> c_int {
    let __set: *mut fd_set = set;
    let __fd: c_int = fd;
    let mut __r: c_int = 0;
    if __fd >= 0 {
        unsafe {
            __r = (((*__set).fds[(__fd as usize) / FD_SETIDXMASK]
                & ((1_u32.wrapping_shl(((__fd as usize) & FD_SETBITMASK) as u32)) as c_ulong))
                != 0) as c_int;
        }
    }
    __r
}

pub unsafe fn FD_ZERO(set: *mut fd_set) {
    let __set: *mut fd_set = set;
    let mut __idx: c_int;
    let __size: c_int = ((FD_SETSIZE + FD_SETBITMASK) / FD_SETIDXMASK) as c_int;
    __idx = 0;
    while __idx < __size {
        unsafe {
            (*__set).fds[__idx as usize] = 0;
        }
        __idx += 1;
    }
}

/*
 * int select(int nfds, fd_set *read_fds, fd_set *write_fds,
 *            fd_set *except_fds, struct timeval *timeout);
 */

extern "C" {
    fn __nolibc_syscall6(
        nr: c_long,
        arg1: c_long,
        arg2: *mut c_void,
        arg3: *mut c_void,
        arg4: *mut c_void,
        arg5: *mut c_void,
        arg6: *mut c_void,
    ) -> c_long;
    fn __sysret(ret: c_long) -> c_int;
}

// The following items are provided by the translated equivalents of
// <linux/time.h> and <linux/unistd.h>.
extern "C" {
    static __NR_pselect6: c_long;
}

extern "C" {
    type timeval;
    type __kernel_timespec;
    type __kernel_old_timespec;
}

// Field layout is supplied by the external Linux time translations.
extern "Rust" {
    fn timeval_tv_sec(timeout: *mut timeval) -> c_long;
    fn timeval_tv_usec(timeout: *mut timeval) -> c_long;
    fn __kernel_timespec_new(tv_sec: c_long, tv_nsec: c_long) -> __kernel_timespec;
    fn __kernel_old_timespec_new(tv_sec: c_long, tv_nsec: c_long) -> __kernel_old_timespec;
}

pub unsafe fn _sys_select(
    nfds: c_int,
    rfds: *mut fd_set,
    wfds: *mut fd_set,
    efds: *mut fd_set,
    timeout: *mut timeval,
) -> c_int {
    // C preprocessor branch:
    // #if defined(__NR_pselect6_time64)
    //     struct __kernel_timespec t;
    //     return __nolibc_syscall6(__NR_pselect6_time64, ...);
    // #else
    //     struct __kernel_old_timespec t;
    //     return __nolibc_syscall6(__NR_pselect6, ...);
    // #endif
    //
    // This file-local translation preserves the non-time64 fallback branch,
    // because the build-time availability of __NR_pselect6_time64 is defined
    // outside this isolated source.
    let mut t_storage: __kernel_old_timespec;
    let t_ptr: *mut c_void;

    if !timeout.is_null() {
        unsafe {
            t_storage = __kernel_old_timespec_new(
                timeval_tv_sec(timeout),
                ((timeval_tv_usec(timeout) as u32).wrapping_mul(1000)) as c_long,
            );
            t_ptr = (&mut t_storage as *mut __kernel_old_timespec).cast::<c_void>();
        }
    } else {
        t_ptr = core::ptr::null_mut();
    }

    unsafe {
        __nolibc_syscall6(
            __NR_pselect6,
            nfds as c_long,
            rfds.cast::<c_void>(),
            wfds.cast::<c_void>(),
            efds.cast::<c_void>(),
            t_ptr,
            core::ptr::null_mut(),
        ) as c_int
    }
}

pub unsafe fn select(
    nfds: c_int,
    rfds: *mut fd_set,
    wfds: *mut fd_set,
    efds: *mut fd_set,
    timeout: *mut timeval,
) -> c_int {
    unsafe { __sysret(_sys_select(nfds, rfds, wfds, efds, timeout) as c_long) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
