// SPDX-License-Identifier: GPL-2.0
/*
 * This test covers the PR_SET_NAME functionality of prctl calls
 */

use core::ffi::{c_char, c_int, c_long, c_void};

const CHANGE_NAME: &[u8] = b"changename\0";
const EMPTY_NAME: &[u8] = b"\0";
const TASK_COMM_LEN: usize = 16;
const MAX_PATH_LEN: usize = 50;

const EIO: c_int = 5;
const PR_SET_NAME: c_int = 15;
const PR_GET_NAME: c_int = 16;

type FILE = c_void;

unsafe extern "C" {
    fn prctl(option: c_int, ...) -> c_int;
    fn __errno_location() -> *mut c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn getpid() -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn ferror(stream: *mut FILE) -> c_int;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

pub unsafe fn set_name(name: *mut c_char) -> c_int {
    let res: c_int;

    res = prctl(
        PR_SET_NAME,
        name,
        core::ptr::null_mut::<c_void>(),
        core::ptr::null_mut::<c_void>(),
        core::ptr::null_mut::<c_void>(),
    );

    if res < 0 {
        return -errno();
    }
    res
}

pub unsafe fn check_is_name_correct(check_name: *mut c_char) -> c_int {
    let mut name: [c_char; TASK_COMM_LEN] = [0; TASK_COMM_LEN];
    let res: c_int;

    res = prctl(
        PR_GET_NAME,
        name.as_mut_ptr(),
        core::ptr::null_mut::<c_void>(),
        core::ptr::null_mut::<c_void>(),
        core::ptr::null_mut::<c_void>(),
    );

    if res < 0 {
        return -errno();
    }

    (strcmp(name.as_ptr(), check_name) == 0) as c_int
}

pub unsafe fn check_null_pointer(_check_name: *mut c_char) -> c_int {
    let name: *mut c_char = core::ptr::null_mut();
    let res: c_int;

    res = prctl(
        PR_GET_NAME,
        name,
        core::ptr::null_mut::<c_void>(),
        core::ptr::null_mut::<c_void>(),
        core::ptr::null_mut::<c_void>(),
    );

    res
}

pub unsafe fn check_name() -> c_int {
    let pid: c_int;

    pid = getpid();
    let mut fptr: *mut FILE = core::ptr::null_mut();
    let mut path: [c_char; MAX_PATH_LEN] = [0; MAX_PATH_LEN];
    let mut name: [c_char; TASK_COMM_LEN] = [0; TASK_COMM_LEN];
    let mut output: [c_char; TASK_COMM_LEN] = [0; TASK_COMM_LEN];
    let j: c_int;

    j = snprintf(
        path.as_mut_ptr(),
        MAX_PATH_LEN,
        b"/proc/self/task/%d/comm\0".as_ptr() as *const c_char,
        pid,
    );
    let _ = j;
    fptr = fopen(
        path.as_ptr(),
        b"r\0".as_ptr() as *const c_char,
    );
    if fptr.is_null() {
        return -EIO;
    }

    fscanf(
        fptr,
        b"%s\0".as_ptr() as *const c_char,
        output.as_mut_ptr(),
    );
    if ferror(fptr) != 0 {
        return -EIO;
    }

    let res: c_int = prctl(
        PR_GET_NAME,
        name.as_mut_ptr(),
        core::ptr::null_mut::<c_void>(),
        core::ptr::null_mut::<c_void>(),
        core::ptr::null_mut::<c_void>(),
    );

    if res < 0 {
        return -errno();
    }

    (strcmp(output.as_ptr(), name.as_ptr()) == 0) as c_int
}

pub unsafe fn rename_process() {
    assert!(set_name(CHANGE_NAME.as_ptr() as *mut c_char) >= 0);
    assert!(check_is_name_correct(CHANGE_NAME.as_ptr() as *mut c_char) != 0);

    assert!(set_name(EMPTY_NAME.as_ptr() as *mut c_char) >= 0);
    assert!(check_is_name_correct(EMPTY_NAME.as_ptr() as *mut c_char) != 0);

    assert!(set_name(CHANGE_NAME.as_ptr() as *mut c_char) >= 0);
    assert!(check_null_pointer(CHANGE_NAME.as_ptr() as *mut c_char) < 0);

    assert!(check_name() != 0);
}

pub unsafe fn main() {
    // Original C used TEST_HARNESS_MAIN from kselftest_harness.h.
    rename_process();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
