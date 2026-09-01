/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * unistd function definitions for NOLIBC
 * Copyright (C) 2017-2022 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols */
/* C dependencies: "nolibc.h", "std.h", "arch.h", "types.h", "sys.h" */

pub const STDIN_FILENO: i32 = 0;
pub const STDOUT_FILENO: i32 = 1;
pub const STDERR_FILENO: i32 = 2;

pub const F_OK: i32 = 0;
pub const X_OK: i32 = 1;
pub const W_OK: i32 = 2;
pub const R_OK: i32 = 4;

/*
 * int access(const char *path, int amode);
 * int faccessat(int fd, const char *path, int amode, int flag);
 */

unsafe extern "C" {
    static __NR_faccessat: core::ffi::c_long;
    static __NR_ftruncate: core::ffi::c_long;
    static __NR_getcwd: core::ffi::c_long;
    static __NR_readlinkat: core::ffi::c_long;
    static AT_FDCWD: core::ffi::c_int;
    static EINVAL: core::ffi::c_int;
    static ENOENT: core::ffi::c_int;
    static TIOCSPGRP: core::ffi::c_ulong;

    fn __nolibc_syscall2(
        nr: core::ffi::c_long,
        arg1: core::ffi::c_long,
        arg2: core::ffi::c_long,
    ) -> core::ffi::c_long;
    fn __nolibc_syscall3(
        nr: core::ffi::c_long,
        arg1: core::ffi::c_long,
        arg2: core::ffi::c_long,
        arg3: core::ffi::c_long,
    ) -> core::ffi::c_long;
    fn __nolibc_syscall4(
        nr: core::ffi::c_long,
        arg1: core::ffi::c_long,
        arg2: core::ffi::c_long,
        arg3: core::ffi::c_long,
        arg4: core::ffi::c_long,
    ) -> core::ffi::c_long;
    fn __sysret(ret: core::ffi::c_long) -> core::ffi::c_long;
    fn SET_ERRNO(errno: core::ffi::c_int);
    fn _sys_select(
        nfds: core::ffi::c_int,
        readfds: *mut core::ffi::c_void,
        writefds: *mut core::ffi::c_void,
        exceptfds: *mut core::ffi::c_void,
        timeout: *mut timeval,
    ) -> core::ffi::c_int;
    fn ioctl(fd: core::ffi::c_int, request: core::ffi::c_ulong, ...) -> core::ffi::c_int;
}

#[allow(non_camel_case_types)]
pub type size_t = usize;
#[allow(non_camel_case_types)]
pub type ssize_t = isize;
#[allow(non_camel_case_types)]
pub type off_t = core::ffi::c_long;
#[allow(non_camel_case_types)]
pub type pid_t = core::ffi::c_int;

#[repr(C)]
pub struct timeval {
    pub tv_sec: core::ffi::c_long,
    pub tv_usec: core::ffi::c_long,
}

#[allow(non_snake_case)]
unsafe fn __NOLIBC_LLARGPART(value: off_t, part: core::ffi::c_int) -> u32 {
    ((value as i64 as u64 >> (part * 32)) & 0xffff_ffff) as u32
}

#[allow(dead_code)]
unsafe fn _sys_faccessat(
    fd: core::ffi::c_int,
    path: *const core::ffi::c_char,
    amode: core::ffi::c_int,
    flag: core::ffi::c_int,
) -> core::ffi::c_int {
    unsafe {
        __nolibc_syscall4(
            __NR_faccessat,
            fd as core::ffi::c_long,
            path as core::ffi::c_long,
            amode as core::ffi::c_long,
            flag as core::ffi::c_long,
        ) as core::ffi::c_int
    }
}

#[allow(dead_code)]
pub unsafe fn faccessat(
    fd: core::ffi::c_int,
    path: *const core::ffi::c_char,
    amode: core::ffi::c_int,
    flag: core::ffi::c_int,
) -> core::ffi::c_int {
    unsafe { __sysret(_sys_faccessat(fd, path, amode, flag) as core::ffi::c_long) as core::ffi::c_int }
}

#[allow(dead_code)]
pub unsafe fn access(path: *const core::ffi::c_char, amode: core::ffi::c_int) -> core::ffi::c_int {
    unsafe { faccessat(AT_FDCWD, path, amode, 0) }
}

/* C conditional:
 * #if !defined(_sys_ftruncate64) && defined(__NR_ftruncate64)
 */
#[cfg(any())]
unsafe extern "C" {
    static __NR_ftruncate64: core::ffi::c_long;
}

#[cfg(any())]
#[allow(dead_code)]
unsafe fn _sys_ftruncate64(
    fd: core::ffi::c_int,
    length0: u32,
    length1: u32,
) -> core::ffi::c_int {
    unsafe {
        __nolibc_syscall3(
            __NR_ftruncate64,
            fd as core::ffi::c_long,
            length0 as core::ffi::c_long,
            length1 as core::ffi::c_long,
        ) as core::ffi::c_int
    }
}

#[allow(dead_code)]
unsafe fn _sys_ftruncate(fd: core::ffi::c_int, length: off_t) -> core::ffi::c_int {
    /* If _sys_ftruncate64 is available for the target, the C source calls:
     * _sys_ftruncate64(fd, __NOLIBC_LLARGPART(length, 0),
     *                  __NOLIBC_LLARGPART(length, 1));
     * Otherwise it falls back to __NR_ftruncate.
     */
    unsafe {
        __nolibc_syscall2(
            __NR_ftruncate,
            fd as core::ffi::c_long,
            length as core::ffi::c_long,
        ) as core::ffi::c_int
    }
}

#[allow(dead_code)]
pub unsafe fn ftruncate(fd: core::ffi::c_int, length: off_t) -> core::ffi::c_int {
    unsafe { __sysret(_sys_ftruncate(fd, length) as core::ffi::c_long) as core::ffi::c_int }
}

/*
 * char *getcwd(char *buf, size_t size);
 */

#[allow(dead_code)]
unsafe fn _sys_getcwd(buf: *mut core::ffi::c_char, size: size_t) -> core::ffi::c_int {
    unsafe {
        __nolibc_syscall2(
            __NR_getcwd,
            buf as core::ffi::c_long,
            size as core::ffi::c_long,
        ) as core::ffi::c_int
    }
}

#[allow(dead_code)]
pub unsafe fn getcwd(buf: *mut core::ffi::c_char, size: size_t) -> *mut core::ffi::c_char {
    let ret: core::ffi::c_int;

    /* Unlike other libc's we don't handle passing NULL for buf */
    if buf.is_null() || size == 0 {
        unsafe { SET_ERRNO(EINVAL) };
        return core::ptr::null_mut();
    }

    ret = unsafe { __sysret(_sys_getcwd(buf, size) as core::ffi::c_long) as core::ffi::c_int };

    /* On error return NULL, __sysret() above will have set errno */
    if ret < 0 {
        return core::ptr::null_mut();
    }

    /* Handle no path being written or the kernel putting
     * "(unreachable)" into the buffer instead of a path.
     * This matches what musl is doing.
     */
    if ret == 0 || unsafe { *buf } != b'/' as core::ffi::c_char {
        unsafe { SET_ERRNO(ENOENT) };
        return core::ptr::null_mut();
    }

    /* ret must be the number of bytes written at this point,
     * so return the pointer to buf.
     */
    buf
}

#[allow(dead_code)]
pub unsafe fn msleep(msecs: core::ffi::c_uint) -> core::ffi::c_int {
    let mut my_timeval = timeval {
        tv_sec: (msecs / 1000) as core::ffi::c_long,
        tv_usec: ((msecs % 1000) * 1000) as core::ffi::c_long,
    };

    if unsafe {
        _sys_select(
            0,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            &mut my_timeval,
        )
    } < 0
    {
        (my_timeval.tv_sec * 1000
            + my_timeval.tv_usec / 1000
            + ((my_timeval.tv_usec % 1000) != 0) as core::ffi::c_long) as core::ffi::c_int
    } else {
        0
    }
}

/*
 * ssize_t readlink(const char *path, char *buf, size_t bufsiz);
 */

#[allow(dead_code)]
unsafe fn _sys_readlink(
    path: *const core::ffi::c_char,
    buf: *mut core::ffi::c_char,
    bufsiz: size_t,
) -> ssize_t {
    unsafe {
        __nolibc_syscall4(
            __NR_readlinkat,
            AT_FDCWD as core::ffi::c_long,
            path as core::ffi::c_long,
            buf as core::ffi::c_long,
            bufsiz as core::ffi::c_long,
        ) as ssize_t
    }
}

#[allow(dead_code)]
pub unsafe fn readlink(
    path: *const core::ffi::c_char,
    buf: *mut core::ffi::c_char,
    bufsiz: size_t,
) -> ssize_t {
    unsafe { __sysret(_sys_readlink(path, buf, bufsiz) as core::ffi::c_long) as ssize_t }
}

#[allow(dead_code)]
pub unsafe fn sleep(seconds: core::ffi::c_uint) -> core::ffi::c_uint {
    let mut my_timeval = timeval {
        tv_sec: seconds as core::ffi::c_long,
        tv_usec: 0,
    };

    if unsafe {
        _sys_select(
            0,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            &mut my_timeval,
        )
    } < 0
    {
        (my_timeval.tv_sec + (my_timeval.tv_usec != 0) as core::ffi::c_long) as core::ffi::c_uint
    } else {
        0
    }
}

#[allow(dead_code)]
pub unsafe fn usleep(usecs: core::ffi::c_uint) -> core::ffi::c_int {
    let mut my_timeval = timeval {
        tv_sec: (usecs / 1000000) as core::ffi::c_long,
        tv_usec: (usecs % 1000000) as core::ffi::c_long,
    };

    unsafe {
        _sys_select(
            0,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            &mut my_timeval,
        )
    }
}

#[allow(dead_code)]
pub unsafe fn tcsetpgrp(fd: core::ffi::c_int, pid: pid_t) -> core::ffi::c_int {
    let mut pid = pid;

    unsafe { ioctl(fd, TIOCSPGRP, &mut pid as *mut pid_t) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
