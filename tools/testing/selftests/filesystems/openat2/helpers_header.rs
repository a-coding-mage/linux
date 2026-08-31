// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Author: Aleksa Sarai <cyphar@cyphar.com>
 * Copyright (C) 2018-2019 SUSE LLC.
 * Copyright (C) 2026 Amutable GmbH
 */

// C header dependencies removed from executable Rust:
// _GNU_SOURCE, stdint.h, stdbool.h, errno.h, limits.h, linux/types.h,
// linux/unistd.h, linux/openat2.h, and "kselftest_harness.h".

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

// Expected to be supplied by the translated surrounding repository.
use crate::{open_how, __test_metadata, __NR_openat2, __NR_renameat2};
use crate::{AT_FDCWD, O_CREAT, PATH_MAX};

pub const OPEN_HOW_SIZE_VER0: usize = 24; /* sizeof first published struct */
pub const OPEN_HOW_SIZE_LATEST: usize = OPEN_HOW_SIZE_VER0;

unsafe extern "C" {
    static mut errno: c_int;

    fn syscall(num: c_long, ...) -> c_long;
    fn openat(dfd: c_int, path: *const c_char, flags: u64, mode: u64) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn readlink(pathname: *const c_char, buf: *mut c_char, bufsiz: usize) -> isize;
    fn free(ptr: *mut c_void);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

#[allow(dead_code)]
pub unsafe fn needs_openat2(how: *const open_how) -> bool {
    unsafe { (*how).resolve != 0 }
}

#[allow(dead_code)]
pub unsafe fn raw_openat2(
    dfd: c_int,
    path: *const c_char,
    how: *mut c_void,
    size: usize,
) -> c_int {
    let ret: c_int = unsafe { syscall(__NR_openat2 as c_long, dfd, path, how, size) as c_int };

    if ret >= 0 {
        ret
    } else {
        unsafe { -errno }
    }
}

#[allow(dead_code)]
pub unsafe fn sys_openat2(dfd: c_int, path: *const c_char, how: *mut open_how) -> c_int {
    unsafe {
        raw_openat2(
            dfd,
            path,
            how as *mut c_void,
            core::mem::size_of_val(&*how),
        )
    }
}

#[allow(dead_code)]
pub unsafe fn sys_openat(dfd: c_int, path: *const c_char, how: *mut open_how) -> c_int {
    let ret: c_int = unsafe { openat(dfd, path, (*how).flags, (*how).mode) };

    if ret >= 0 {
        ret
    } else {
        unsafe { -errno }
    }
}

#[allow(dead_code)]
pub unsafe fn sys_renameat2(
    olddirfd: c_int,
    oldpath: *const c_char,
    newdirfd: c_int,
    newpath: *const c_char,
    flags: c_uint,
) -> c_int {
    let ret: c_int = unsafe {
        syscall(
            __NR_renameat2 as c_long,
            olddirfd,
            oldpath,
            newdirfd,
            newpath,
            flags,
        ) as c_int
    };

    if ret >= 0 {
        ret
    } else {
        unsafe { -errno }
    }
}

#[allow(dead_code)]
pub unsafe fn touchat(dfd: c_int, path: *const c_char) -> c_int {
    let fd: c_int = unsafe { openat(dfd, path, O_CREAT as u64, 0o700) };

    if fd >= 0 {
        unsafe {
            close(fd);
        }
    }
    fd
}

#[allow(dead_code)]
pub unsafe fn fdreadlink(_metadata: *mut __test_metadata, fd: c_int) -> *mut c_char {
    let mut target: *mut c_char;
    let mut tmp: *mut c_char = core::ptr::null_mut();

    assert!(unsafe { asprintf(&mut tmp, c"/proc/self/fd/%d".as_ptr(), fd) } > 0);

    target = unsafe { malloc(PATH_MAX as usize) as *mut c_char };
    assert!(!target.is_null());
    unsafe {
        memset(target as *mut c_void, 0, PATH_MAX as usize);
    }

    assert!(unsafe { readlink(tmp, target, PATH_MAX as usize) } > 0);

    unsafe {
        free(tmp as *mut c_void);
    }
    target
}

#[allow(dead_code)]
pub unsafe fn fdequal(
    _metadata: *mut __test_metadata,
    fd: c_int,
    dfd: c_int,
    path: *const c_char,
) -> bool {
    let fdpath: *mut c_char;
    let dfdpath: *mut c_char;
    let mut other: *mut c_char = core::ptr::null_mut();
    let cmp: bool;

    fdpath = unsafe { fdreadlink(_metadata, fd) };
    dfdpath = unsafe { fdreadlink(_metadata, dfd) };

    if path.is_null() {
        assert!(unsafe { asprintf(&mut other, c"%s".as_ptr(), dfdpath) } > 0);
    } else if unsafe { *path } == b'/' as c_char {
        assert!(unsafe { asprintf(&mut other, c"%s".as_ptr(), path) } > 0);
    } else {
        assert!(unsafe { asprintf(&mut other, c"%s/%s".as_ptr(), dfdpath, path) } > 0);
    }

    cmp = unsafe { strcmp(fdpath, other) } == 0;

    unsafe {
        free(fdpath as *mut c_void);
        free(dfdpath as *mut c_void);
        free(other as *mut c_void);
    }
    cmp
}

pub static mut openat2_supported: bool = false;

#[used]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".init_array"))]
static __DETECT_OPENAT2_SUPPORTED_INIT: unsafe extern "C" fn() = __detect_openat2_supported;

unsafe extern "C" fn __detect_openat2_supported() {
    let mut how: open_how = unsafe { core::mem::zeroed() };
    let fd: c_int;

    const _: [(); OPEN_HOW_SIZE_VER0] = [(); core::mem::size_of::<open_how>()];

    /* Check openat2(2) support. */
    fd = unsafe { sys_openat2(AT_FDCWD, c".".as_ptr(), &mut how) };
    unsafe {
        openat2_supported = fd >= 0;
    }

    if fd >= 0 {
        unsafe {
            close(fd);
        }
    }
}
