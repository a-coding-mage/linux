// SPDX-License-Identifier: GPL-2.0
// C dependencies: stdio.h, errno.h, string.h
// Local dependencies: sysctl_helpers.h, test_progs.h

use core::ffi::{c_char, c_int, c_void};

const ENOENT: c_int = 2;
const SEEK_SET: c_int = 0;

type FILE = c_void;

unsafe extern "C" {
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn fseek(stream: *mut FILE, offset: libc::c_long, whence: c_int) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;

    static mut errno: c_int;
}

// External test_progs.h macro/function equivalent.
unsafe extern "C" {
    fn PRINT_FAIL(format: *const c_char, ...);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sysctl_set(
    sysctl_path: *const c_char,
    old_val: *mut c_char,
    new_val: *const c_char,
) -> c_int {
    let mut ret: c_int = 0;
    let fp: *mut FILE;

    fp = unsafe { fopen(sysctl_path, c"r+".as_ptr()) };
    if fp.is_null() {
        return unsafe { -errno };
    }
    if !old_val.is_null() && unsafe { fscanf(fp, c"%s".as_ptr(), old_val) } <= 0 {
        ret = -ENOENT;
    } else if old_val.is_null() || unsafe { strcmp(old_val, new_val) } != 0 {
        unsafe {
            fseek(fp, 0, SEEK_SET);
        }
        if unsafe { fprintf(fp, c"%s".as_ptr(), new_val) } < 0 {
            ret = unsafe { -errno };
        }
    }
    unsafe {
        fclose(fp);
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sysctl_set_or_fail(
    sysctl_path: *const c_char,
    old_val: *mut c_char,
    new_val: *const c_char,
) -> c_int {
    let err: c_int;

    err = unsafe { sysctl_set(sysctl_path, old_val, new_val) };
    if err != 0 {
        unsafe {
            PRINT_FAIL(
                c"failed to set %s to %s: %s\n".as_ptr(),
                sysctl_path,
                new_val,
                strerror(-err),
            );
        }
    }
    err
}
