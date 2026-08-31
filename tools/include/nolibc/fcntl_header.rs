/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * fcntl definition for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols */
/* C dependency: "nolibc.h" */

/* Header guard _NOLIBC_FCNTL_H omitted in Rust. */

/* C dependencies: "arch.h", "types.h", "sys.h" */

#[inline]
pub const fn __nolibc_open_flags(_flags: i32) -> i32 {
    _flags | O_LARGEFILE
}

/*
 * C macro:
 *
 * #define __nolibc_open_mode(_flags) ({ ... va_start(args, (_flags)); ... })
 *
 * This reads the optional mode_t argument following _flags in the caller's
 * variadic argument list. Rust has no direct stable file-local equivalent for
 * this statement-expression macro. Variadic wrappers below keep the C interface
 * intent and mark the extraction point explicitly.
 */

/*
 * int openat(int dirfd, const char *path, int flags[, mode_t mode]);
 */

#[inline]
pub unsafe fn _sys_openat(
    dirfd: i32,
    path: *const core::ffi::c_char,
    flags: i32,
    mode: mode_t,
) -> i32 {
    unsafe { __nolibc_syscall4(__NR_openat, dirfd, path, flags, mode) as i32 }
}

/*
 * Rust equivalent of the C variadic function:
 *
 * static int openat(int dirfd, const char *path, int flags, ...)
 * {
 *     return __sysret(_sys_openat(dirfd, path, __nolibc_open_flags(flags),
 *                                __nolibc_open_mode(flags)));
 * }
 *
 * TODO: supply the caller's optional mode_t argument using the same ABI-level
 * variadic access as C's __nolibc_open_mode(flags).
 */
pub unsafe extern "C" fn openat(
    dirfd: i32,
    path: *const core::ffi::c_char,
    flags: i32,
    mut _args: ...
) -> i32 {
    let mode: mode_t = {
        /* va_start(args, flags); mode = va_arg(args, mode_t); va_end(args); */
        unsafe { core::mem::zeroed() }
    };

    unsafe {
        __sysret(_sys_openat(
            dirfd,
            path,
            __nolibc_open_flags(flags),
            mode,
        ))
    }
}

/*
 * int open(const char *path, int flags[, mode_t mode]);
 */

#[inline]
pub unsafe fn _sys_open(path: *const core::ffi::c_char, flags: i32, mode: mode_t) -> i32 {
    unsafe { __nolibc_syscall4(__NR_openat, AT_FDCWD, path, flags, mode) as i32 }
}

/*
 * Rust equivalent of the C variadic function:
 *
 * static int open(const char *path, int flags, ...)
 * {
 *     return __sysret(_sys_open(path, __nolibc_open_flags(flags),
 *                              __nolibc_open_mode(flags)));
 * }
 *
 * TODO: supply the caller's optional mode_t argument using the same ABI-level
 * variadic access as C's __nolibc_open_mode(flags).
 */
pub unsafe extern "C" fn open(
    path: *const core::ffi::c_char,
    flags: i32,
    mut _args: ...
) -> i32 {
    let mode: mode_t = {
        /* va_start(args, flags); mode = va_arg(args, mode_t); va_end(args); */
        unsafe { core::mem::zeroed() }
    };

    unsafe { __sysret(_sys_open(path, __nolibc_open_flags(flags), mode)) }
}

/*
 * int creat(const char *path, mode_t mode);
 */

#[inline]
pub unsafe fn creat(path: *const core::ffi::c_char, mode: mode_t) -> i32 {
    unsafe { open(path, O_CREAT | O_WRONLY | O_TRUNC, mode) }
}
