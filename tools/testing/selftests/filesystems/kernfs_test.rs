// SPDX-License-Identifier: GPL-2.0

// C source defined _GNU_SOURCE and __SANE_USERSPACE_TYPES__ before including:
// <fcntl.h>, <stdio.h>, <sys/stat.h>, <sys/xattr.h>,
// "kselftest_harness.h", and "wrappers.h".

use core::ffi::{c_char, c_int, c_void};

const O_RDONLY: c_int = 0;
const O_CLOEXEC: c_int = 0o2000000;
const ENODATA: c_int = 61;

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn flistxattr(fd: c_int, list: *mut c_char, size: usize) -> isize;
    fn fgetxattr(fd: c_int, name: *const c_char, value: *mut c_void, size: usize) -> isize;
}

unsafe extern "C" {
    static mut errno: c_int;
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {
        assert!($left >= $right)
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_LT {
    ($left:expr, $right:expr) => {
        assert!($left < $right)
    };
}

macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

fn kernfs_listxattr() {
    let fd: c_int;

    /* Read-only file that can never have any extended attributes set. */
    unsafe {
        fd = open(c"/sys/kernel/warn_count".as_ptr(), O_RDONLY | O_CLOEXEC);
        ASSERT_GE!(fd, 0);
        ASSERT_EQ!(flistxattr(fd, core::ptr::null_mut(), 0), 0);
        EXPECT_EQ!(close(fd), 0);
    }
}

fn kernfs_getxattr() {
    let fd: c_int;
    let mut buf: [c_char; 1] = [0; 1];

    /* Read-only file that can never have any extended attributes set. */
    unsafe {
        fd = open(c"/sys/kernel/warn_count".as_ptr(), O_RDONLY | O_CLOEXEC);
        ASSERT_GE!(fd, 0);
        ASSERT_LT!(
            fgetxattr(
                fd,
                c"user.foo".as_ptr(),
                buf.as_mut_ptr().cast::<c_void>(),
                core::mem::size_of_val(&buf),
            ),
            0
        );
        ASSERT_EQ!(errno, ENODATA);
        EXPECT_EQ!(close(fd), 0);
    }
}

fn main() {
    kernfs_listxattr();
    kernfs_getxattr();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
