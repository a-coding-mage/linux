// SPDX-License-Identifier: GPL-2.0-or-later

// C source defined _GNU_SOURCE and __SANE_USERSPACE_TYPES__ before including
// fcntl.h, unistd.h, errno.h, string.h, sys/stat.h, and kselftest_harness.h.

use core::ffi::{c_char, c_int};

const O_EMPTYPATH: c_int = 1 << 26;
const EMPTYPATH_TEST_FILE: *const c_char = b"/tmp/emptypath_test\0".as_ptr() as *const c_char;
const EMPTY_PATH: *const c_char = b"\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct emptypath {
    opath_fd: c_int,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
}

// Constants supplied by the C headers included above.
unsafe extern "C" {
    static O_CREAT: c_int;
    static O_WRONLY: c_int;
    static O_PATH: c_int;
    static O_RDONLY: c_int;
    static S_IRWXU: c_int;
    static ENOENT: c_int;
    static EINVAL: c_int;
}

// kselftest_harness.h supplies these assertion, logging, skip, and test harness
// interfaces in the original source.
macro_rules! TH_LOG {
    ($($arg:tt)*) => {
        th_log(format_args!($($arg)*))
    };
}

unsafe extern "Rust" {
    fn th_log(args: core::fmt::Arguments<'_>);
    fn assert_ge(left: c_int, right: c_int) -> bool;
    fn assert_lt(left: c_int, right: c_int) -> bool;
    fn expect_eq(left: c_int, right: c_int) -> bool;
    fn skip_return(message: &str);
}

pub unsafe fn emptypath_setup(self_: *mut emptypath) {
    let fd: c_int;

    unsafe {
        (*self_).opath_fd = -1;

        fd = open(EMPTYPATH_TEST_FILE, O_CREAT | O_WRONLY, S_IRWXU);
        if !assert_ge(fd, 0) {
            TH_LOG!(
                "create {:?}: {:?}",
                EMPTYPATH_TEST_FILE,
                strerror(errno)
            );
            return;
        }
        close(fd);

        (*self_).opath_fd = open(EMPTYPATH_TEST_FILE, O_PATH);
        if !assert_ge((*self_).opath_fd, 0) {
            TH_LOG!(
                "open {:?} O_PATH: {:?}",
                EMPTYPATH_TEST_FILE,
                strerror(errno)
            );
        }
    }
}

pub unsafe fn emptypath_teardown(self_: *mut emptypath) {
    unsafe {
        if (*self_).opath_fd >= 0 {
            close((*self_).opath_fd);
        }
        unlink(EMPTYPATH_TEST_FILE);
    }
}

/* An empty path is rejected with ENOENT unless O_EMPTYPATH is set. */
pub unsafe fn emptypath_without_flag_returns_enoent(self_: *mut emptypath) {
    unsafe {
        let fd: c_int = openat((*self_).opath_fd, EMPTY_PATH, O_RDONLY);

        if fd >= 0 {
            close(fd);
        }
        if !assert_lt(fd, 0) {
            TH_LOG!("empty path without O_EMPTYPATH unexpectedly succeeded");
            return;
        }
        if !expect_eq(errno, ENOENT) {
            TH_LOG!("expected ENOENT, got {:?}", strerror(errno));
        }
    }
}

/* O_EMPTYPATH reopens the O_PATH fd through an empty path. */
pub unsafe fn emptypath_reopens_opath_fd(self_: *mut emptypath) {
    unsafe {
        let fd: c_int = openat((*self_).opath_fd, EMPTY_PATH, O_RDONLY | O_EMPTYPATH);

        if fd < 0 && errno == EINVAL {
            skip_return("O_EMPTYPATH not supported");
            return;
        }

        if !assert_ge(fd, 0) {
            TH_LOG!("O_EMPTYPATH failed: {:?}", strerror(errno));
            return;
        }
        close(fd);
    }
}

// TEST_HARNESS_MAIN
