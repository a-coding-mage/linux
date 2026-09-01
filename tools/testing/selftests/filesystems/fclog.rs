// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Author: Aleksa Sarai <cyphar@cyphar.com>
 * Copyright (C) 2025 SUSE LLC.
 */

// C dependencies translated from:
// <assert.h>, <errno.h>, <fcntl.h>, <sched.h>, <unistd.h>, <sys/mount.h>
// and "kselftest_harness.h".

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    static mut errno: c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: usize,
        data: *const c_void,
    ) -> c_int;
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;

    fn fsopen(fs_name: *const c_char, flags: c_uint) -> c_int;
    fn fsconfig(
        fd: c_int,
        cmd: c_uint,
        key: *const c_char,
        value: *const c_void,
        aux: c_int,
    ) -> c_int;
    fn fsmount(fs_fd: c_int, flags: c_uint, attr_flags: c_uint) -> c_int;
    fn move_mount(
        from_dfd: c_int,
        from_pathname: *const c_char,
        to_dfd: c_int,
        to_pathname: *const c_char,
        flags: c_uint,
    ) -> c_int;
}

const O_RDONLY: c_int = 0;
const O_CLOEXEC: c_int = 0o2000000;
const CLONE_NEWNS: c_int = 0x00020000;
const MS_REC: usize = 16384;
const MS_PRIVATE: usize = 1 << 18;
const AT_FDCWD: c_int = -100;

const ENODATA: c_int = 61;
const EINVAL: c_int = 22;
const EMSGSIZE: c_int = 90;

const FSOPEN_CLOEXEC: c_uint = 0x00000001;
const FSCONFIG_SET_STRING: c_uint = 1;
const FSCONFIG_CMD_CREATE: c_uint = 6;
const FSMOUNT_CLOEXEC: c_uint = 0x00000001;
const MOUNT_ATTR_NOEXEC: c_uint = 0x00000001;
const MOUNT_ATTR_NOSUID: c_uint = 0x00000002;
const MOVE_MOUNT_F_EMPTY_PATH: c_uint = 0x00000004;

extern "Rust" {
    fn __EXPECT(
        expected: isize,
        expected_str: *const c_char,
        seen: isize,
        seen_str: *const c_char,
        t: *const c_char,
        terminate: c_int,
    );
    fn EXPECT_STREQ(expected: *const c_char, seen: *const c_char);
}

macro_rules! ASSERT_ERRNO {
    ($expected:expr, <=, $seen:expr) => {{
        let tmp_seen = $seen;
        let seen_errno = if tmp_seen >= 0 {
            tmp_seen as isize
        } else {
            unsafe { -(errno as isize) }
        };
        unsafe {
            __EXPECT(
                $expected as isize,
                concat!(stringify!($expected), "\0").as_ptr() as *const c_char,
                seen_errno,
                concat!(stringify!($seen), "\0").as_ptr() as *const c_char,
                b"<=\0".as_ptr() as *const c_char,
                1,
            );
        }
    }};
    ($expected:expr, ==, $seen:expr) => {{
        let tmp_seen = $seen;
        let seen_errno = if tmp_seen >= 0 {
            tmp_seen as isize
        } else {
            unsafe { -(errno as isize) }
        };
        unsafe {
            __EXPECT(
                $expected as isize,
                concat!(stringify!($expected), "\0").as_ptr() as *const c_char,
                seen_errno,
                concat!(stringify!($seen), "\0").as_ptr() as *const c_char,
                b"==\0".as_ptr() as *const c_char,
                1,
            );
        }
    }};
}

macro_rules! ASSERT_ERRNO_EQ {
    ($expected:expr, $seen:expr) => {
        ASSERT_ERRNO!($expected, ==, $seen)
    };
}

macro_rules! ASSERT_SUCCESS {
    ($seen:expr) => {
        ASSERT_ERRNO!(0, <=, $seen)
    };
}

#[repr(C)]
struct ns {
    host_mntns: c_int,
}

unsafe fn ns_setup(self_: *mut ns) {
    /* Stash the old mntns. */
    (*self_).host_mntns = open(
        b"/proc/self/ns/mnt\0".as_ptr() as *const c_char,
        O_RDONLY | O_CLOEXEC,
    );
    ASSERT_SUCCESS!((*self_).host_mntns);

    /* Create a new mount namespace and make it private. */
    ASSERT_SUCCESS!(unshare(CLONE_NEWNS));
    ASSERT_SUCCESS!(mount(
        core::ptr::null(),
        b"/\0".as_ptr() as *const c_char,
        core::ptr::null(),
        MS_PRIVATE | MS_REC,
        core::ptr::null(),
    ));
}

unsafe fn ns_teardown(self_: *mut ns) {
    ASSERT_SUCCESS!(setns((*self_).host_mntns, CLONE_NEWNS));
    ASSERT_SUCCESS!(close((*self_).host_mntns));
}

unsafe fn fscontext_log_enodata(_self: *mut ns) {
    let fsfd: c_int = fsopen(b"tmpfs\0".as_ptr() as *const c_char, FSOPEN_CLOEXEC);
    ASSERT_SUCCESS!(fsfd);

    /* A brand new fscontext has no log entries. */
    let mut buf: [c_char; 128] = [0; 128];
    for _i in 0..16 {
        ASSERT_ERRNO_EQ!(
            -ENODATA,
            read(fsfd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf))
        );
    }

    ASSERT_SUCCESS!(close(fsfd));
}

unsafe fn fscontext_log_errorfc(_self: *mut ns) {
    let fsfd: c_int = fsopen(b"tmpfs\0".as_ptr() as *const c_char, FSOPEN_CLOEXEC);
    ASSERT_SUCCESS!(fsfd);

    ASSERT_ERRNO_EQ!(
        -EINVAL,
        fsconfig(
            fsfd,
            FSCONFIG_SET_STRING,
            b"invalid-arg\0".as_ptr() as *const c_char,
            b"123\0".as_ptr() as *const c_void,
            0,
        )
    );

    let mut buf: [c_char; 128] = [0; 128];
    ASSERT_SUCCESS!(read(
        fsfd,
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf),
    ));
    EXPECT_STREQ(
        b"e tmpfs: Unknown parameter 'invalid-arg'\n\0".as_ptr() as *const c_char,
        buf.as_ptr(),
    );

    /* The message has been consumed. */
    ASSERT_ERRNO_EQ!(
        -ENODATA,
        read(fsfd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf))
    );
    ASSERT_SUCCESS!(close(fsfd));
}

unsafe fn fscontext_log_errorfc_after_fsmount(_self: *mut ns) {
    let fsfd: c_int = fsopen(b"tmpfs\0".as_ptr() as *const c_char, FSOPEN_CLOEXEC);
    ASSERT_SUCCESS!(fsfd);

    ASSERT_ERRNO_EQ!(
        -EINVAL,
        fsconfig(
            fsfd,
            FSCONFIG_SET_STRING,
            b"invalid-arg\0".as_ptr() as *const c_char,
            b"123\0".as_ptr() as *const c_void,
            0,
        )
    );

    ASSERT_SUCCESS!(fsconfig(
        fsfd,
        FSCONFIG_CMD_CREATE,
        core::ptr::null(),
        core::ptr::null(),
        0,
    ));
    let mfd: c_int = fsmount(
        fsfd,
        FSMOUNT_CLOEXEC,
        MOUNT_ATTR_NOEXEC | MOUNT_ATTR_NOSUID,
    );
    ASSERT_SUCCESS!(mfd);
    ASSERT_SUCCESS!(move_mount(
        mfd,
        b"\0".as_ptr() as *const c_char,
        AT_FDCWD,
        b"/tmp\0".as_ptr() as *const c_char,
        MOVE_MOUNT_F_EMPTY_PATH,
    ));

    /*
     * The fscontext log should still contain data even after
     * FSCONFIG_CMD_CREATE and fsmount().
     */
    let mut buf: [c_char; 128] = [0; 128];
    ASSERT_SUCCESS!(read(
        fsfd,
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf),
    ));
    EXPECT_STREQ(
        b"e tmpfs: Unknown parameter 'invalid-arg'\n\0".as_ptr() as *const c_char,
        buf.as_ptr(),
    );

    /* The message has been consumed. */
    ASSERT_ERRNO_EQ!(
        -ENODATA,
        read(fsfd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf))
    );
    ASSERT_SUCCESS!(close(fsfd));
}

unsafe fn fscontext_log_emsgsize(_self: *mut ns) {
    let fsfd: c_int = fsopen(b"tmpfs\0".as_ptr() as *const c_char, FSOPEN_CLOEXEC);
    ASSERT_SUCCESS!(fsfd);

    ASSERT_ERRNO_EQ!(
        -EINVAL,
        fsconfig(
            fsfd,
            FSCONFIG_SET_STRING,
            b"invalid-arg\0".as_ptr() as *const c_char,
            b"123\0".as_ptr() as *const c_void,
            0,
        )
    );

    let mut buf: [c_char; 128] = [0; 128];
    /*
     * Attempting to read a message with too small a buffer should not
     * result in the message getting consumed.
     */
    ASSERT_ERRNO_EQ!(-EMSGSIZE, read(fsfd, buf.as_mut_ptr() as *mut c_void, 0));
    ASSERT_ERRNO_EQ!(-EMSGSIZE, read(fsfd, buf.as_mut_ptr() as *mut c_void, 1));
    for _i in 0..16 {
        ASSERT_ERRNO_EQ!(-EMSGSIZE, read(fsfd, buf.as_mut_ptr() as *mut c_void, 16));
    }

    ASSERT_SUCCESS!(read(
        fsfd,
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf),
    ));
    EXPECT_STREQ(
        b"e tmpfs: Unknown parameter 'invalid-arg'\n\0".as_ptr() as *const c_char,
        buf.as_ptr(),
    );

    /* The message has been consumed. */
    ASSERT_ERRNO_EQ!(
        -ENODATA,
        read(fsfd, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf))
    );
    ASSERT_SUCCESS!(close(fsfd));
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
