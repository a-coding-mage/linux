// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Author: Aleksa Sarai <cyphar@cyphar.com>
 * Copyright (C) 2025 SUSE LLC.
 */

/*
 * C dependencies removed from executable Rust:
 * assert.h, errno.h, fcntl.h, sched.h, stdbool.h, stdlib.h, string.h,
 * unistd.h, stdio.h, sys/mount.h, sys/stat.h, sys/prctl.h,
 * and "kselftest_harness.h".
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

type mode_t = c_uint;
type pid_t = c_int;

const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_CREAT: c_int = 0o100;
const O_CLOEXEC: c_int = 0o2000000;

const X_OK: c_int = 1;

const CLONE_NEWNS: c_int = 0x00020000;
const CLONE_NEWPID: c_int = 0x20000000;

const MS_RDONLY: c_long = 1;
const MS_NOSUID: c_long = 2;
const MS_NODEV: c_long = 4;
const MS_NOEXEC: c_long = 8;
const MS_SYNCHRONOUS: c_long = 16;
const MS_REMOUNT: c_long = 32;
const MS_MANDLOCK: c_long = 64;
const MS_DIRSYNC: c_long = 128;
const MS_NOSYMFOLLOW: c_long = 256;
const MS_NOATIME: c_long = 1024;
const MS_NODIRATIME: c_long = 2048;
const MS_BIND: c_long = 4096;
const MS_MOVE: c_long = 8192;
const MS_REC: c_long = 16384;
const MS_SILENT: c_long = 32768;
const MS_POSIXACL: c_long = 1 << 16;
const MS_UNBINDABLE: c_long = 1 << 17;
const MS_PRIVATE: c_long = 1 << 18;
const MS_SLAVE: c_long = 1 << 19;
const MS_SHARED: c_long = 1 << 20;
const MS_RELATIME: c_long = 1 << 21;
const MS_KERNMOUNT: c_long = 1 << 22;
const MS_I_VERSION: c_long = 1 << 23;
const MS_STRICTATIME: c_long = 1 << 24;
const MS_LAZYTIME: c_long = 1 << 25;
const MS_ACTIVE: c_long = 1 << 30;
const MS_NOUSER: c_long = 1 << 31;

const PR_SET_PDEATHSIG: c_int = 1;
const SIGKILL: c_int = 9;

const FSOPEN_CLOEXEC: c_uint = 0x00000001;
const FSMOUNT_CLOEXEC: c_uint = 0x00000001;

const FSCONFIG_SET_FLAG: c_uint = 0;
const FSCONFIG_SET_STRING: c_uint = 1;
const FSCONFIG_SET_BINARY: c_uint = 2;
const FSCONFIG_SET_PATH: c_uint = 3;
const FSCONFIG_SET_PATH_EMPTY: c_uint = 4;
const FSCONFIG_SET_FD: c_uint = 5;
const FSCONFIG_CMD_CREATE: c_uint = 6;
const FSCONFIG_CMD_RECONFIGURE: c_uint = 7;

const ENOENT: c_int = 2;
const EBUSY: c_int = 16;

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_long,
        data: *const c_void,
    ) -> c_int;
    fn mkdir(pathname: *const c_char, mode: mode_t) -> c_int;
    fn fork() -> pid_t;
    fn prctl(option: c_int, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn fsopen(fsname: *const c_char, flags: c_uint) -> c_int;
    fn fsconfig(
        fs_fd: c_int,
        cmd: c_uint,
        key: *const c_char,
        value: *const c_void,
        aux: c_int,
    ) -> c_int;
    fn fsmount(fs_fd: c_int, flags: c_uint, attr_flags: c_uint) -> c_int;
    fn faccessat(dirfd: c_int, pathname: *const c_char, mode: c_int, flags: c_int) -> c_int;
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

/*
 * Original kselftest assertions:
 *
 * #define ASSERT_ERRNO(expected, _t, seen) \
 *     __EXPECT(expected, #expected, \
 *         ({__typeof__(seen) _tmp_seen = (seen); \
 *           _tmp_seen >= 0 ? _tmp_seen : -errno; }), #seen, _t, 1)
 *
 * #define ASSERT_ERRNO_EQ(expected, seen) ASSERT_ERRNO(expected, ==, seen)
 * #define ASSERT_SUCCESS(seen) ASSERT_ERRNO(0, <=, seen)
 *
 * The Rust translation keeps assertion behavior local to this file while the
 * fixture/test registration intent remains represented by the same function
 * names below.
 */
unsafe fn ASSERT_ERRNO_EQ(expected: c_int, seen: c_int) {
    let tmp_seen = seen;
    let actual = if tmp_seen >= 0 {
        tmp_seen
    } else {
        unsafe { -errno() }
    };
    assert_eq!(expected, actual);
}

unsafe fn ASSERT_SUCCESS(seen: c_int) {
    let tmp_seen = seen;
    let actual = if tmp_seen >= 0 {
        tmp_seen
    } else {
        unsafe { -errno() }
    };
    assert!(0 <= actual);
}

unsafe fn touch(path: *mut c_char) -> c_int {
    let fd = unsafe { open(path, O_WRONLY | O_CREAT | O_CLOEXEC, 0o644 as mode_t) };
    if fd < 0 {
        return -1;
    }
    unsafe { close(fd) }
}

#[repr(C)]
struct ns {
    host_mntns: c_int,
    host_pidns: c_int,
    dummy_pidns: c_int,
}

unsafe fn ns_setup(self_: *mut ns) {
    /* Stash the old mntns. */
    unsafe {
        (*self_).host_mntns = open(c"/proc/self/ns/mnt".as_ptr(), O_RDONLY | O_CLOEXEC);
        ASSERT_SUCCESS((*self_).host_mntns);
    }

    /* Create a new mount namespace and make it private. */
    unsafe {
        ASSERT_SUCCESS(unshare(CLONE_NEWNS));
        ASSERT_SUCCESS(mount(
            ptr::null(),
            c"/".as_ptr(),
            ptr::null(),
            MS_PRIVATE | MS_REC,
            ptr::null(),
        ));
    }

    /*
     * Create a proper tmpfs that we can use and will disappear once we
     * leave this mntns.
     */
    unsafe {
        ASSERT_SUCCESS(mount(
            c"tmpfs".as_ptr(),
            c"/tmp".as_ptr(),
            c"tmpfs".as_ptr(),
            0,
            ptr::null(),
        ));
    }

    /*
     * Create a pidns we can use for later tests. We need to fork off a
     * child so that we get a usable nsfd that we can bind-mount and open.
     */
    unsafe {
        ASSERT_SUCCESS(mkdir(c"/tmp/dummy".as_ptr(), 0o755));
        ASSERT_SUCCESS(touch(c"/tmp/dummy/pidns".as_ptr() as *mut c_char));
        ASSERT_SUCCESS(mkdir(c"/tmp/dummy/proc".as_ptr(), 0o755));

        (*self_).host_pidns = open(c"/proc/self/ns/pid".as_ptr(), O_RDONLY | O_CLOEXEC);
        ASSERT_SUCCESS((*self_).host_pidns);
        ASSERT_SUCCESS(unshare(CLONE_NEWPID));
    }

    let pid: pid_t = unsafe { fork() };
    unsafe { ASSERT_SUCCESS(pid) };
    if pid == 0 {
        unsafe {
            prctl(PR_SET_PDEATHSIG, SIGKILL);
            ASSERT_SUCCESS(mount(
                c"/proc/self/ns/pid".as_ptr(),
                c"/tmp/dummy/pidns".as_ptr(),
                ptr::null(),
                MS_BIND,
                ptr::null(),
            ));
            ASSERT_SUCCESS(mount(
                c"proc".as_ptr(),
                c"/tmp/dummy/proc".as_ptr(),
                c"proc".as_ptr(),
                0,
                ptr::null(),
            ));
            exit(0);
        }
    }

    let mut wstatus: c_int = 0;
    unsafe {
        assert_eq!(waitpid(pid, &mut wstatus, 0), pid);
        assert!(WIFEXITED(wstatus));
        assert_eq!(WEXITSTATUS(wstatus), 0);

        ASSERT_SUCCESS(setns((*self_).host_pidns, CLONE_NEWPID));

        (*self_).dummy_pidns = open(c"/tmp/dummy/pidns".as_ptr(), O_RDONLY | O_CLOEXEC);
        ASSERT_SUCCESS((*self_).dummy_pidns);
    }
}

unsafe fn ns_teardown(self_: *mut ns) {
    unsafe {
        ASSERT_SUCCESS(setns((*self_).host_mntns, CLONE_NEWNS));
        ASSERT_SUCCESS(close((*self_).host_mntns));

        ASSERT_SUCCESS(close((*self_).host_pidns));
        ASSERT_SUCCESS(close((*self_).dummy_pidns));
    }
}

unsafe fn ns_pidns_mount_string_path(_self: *mut ns) {
    unsafe {
        ASSERT_SUCCESS(mkdir(c"/tmp/proc-host".as_ptr(), 0o755));
        ASSERT_SUCCESS(mount(
            c"proc".as_ptr(),
            c"/tmp/proc-host".as_ptr(),
            c"proc".as_ptr(),
            0,
            c"pidns=/proc/self/ns/pid".as_ptr() as *const c_void,
        ));
        ASSERT_SUCCESS(access(c"/tmp/proc-host/self/".as_ptr(), X_OK));

        ASSERT_SUCCESS(mkdir(c"/tmp/proc-dummy".as_ptr(), 0o755));
        ASSERT_SUCCESS(mount(
            c"proc".as_ptr(),
            c"/tmp/proc-dummy".as_ptr(),
            c"proc".as_ptr(),
            0,
            c"pidns=/tmp/dummy/pidns".as_ptr() as *const c_void,
        ));
        ASSERT_ERRNO_EQ(-ENOENT, access(c"/tmp/proc-dummy/1/".as_ptr(), X_OK));
        ASSERT_ERRNO_EQ(-ENOENT, access(c"/tmp/proc-dummy/self/".as_ptr(), X_OK));
    }
}

unsafe fn ns_pidns_fsconfig_string_path(_self: *mut ns) {
    let fsfd = unsafe { fsopen(c"proc".as_ptr(), FSOPEN_CLOEXEC) };
    unsafe { ASSERT_SUCCESS(fsfd) };

    unsafe {
        ASSERT_SUCCESS(fsconfig(
            fsfd,
            FSCONFIG_SET_STRING,
            c"pidns".as_ptr(),
            c"/tmp/dummy/pidns".as_ptr() as *const c_void,
            0,
        ));
        ASSERT_SUCCESS(fsconfig(
            fsfd,
            FSCONFIG_CMD_CREATE,
            ptr::null(),
            ptr::null(),
            0,
        ));
    }

    let mountfd = unsafe { fsmount(fsfd, FSMOUNT_CLOEXEC, 0) };
    unsafe { ASSERT_SUCCESS(mountfd) };

    unsafe {
        ASSERT_ERRNO_EQ(-ENOENT, faccessat(mountfd, c"1/".as_ptr(), X_OK, 0));
        ASSERT_ERRNO_EQ(-ENOENT, faccessat(mountfd, c"self/".as_ptr(), X_OK, 0));

        ASSERT_SUCCESS(close(fsfd));
        ASSERT_SUCCESS(close(mountfd));
    }
}

unsafe fn ns_pidns_fsconfig_fd(self_: *mut ns) {
    let fsfd = unsafe { fsopen(c"proc".as_ptr(), FSOPEN_CLOEXEC) };
    unsafe { ASSERT_SUCCESS(fsfd) };

    unsafe {
        ASSERT_SUCCESS(fsconfig(
            fsfd,
            FSCONFIG_SET_FD,
            c"pidns".as_ptr(),
            ptr::null(),
            (*self_).dummy_pidns,
        ));
        ASSERT_SUCCESS(fsconfig(
            fsfd,
            FSCONFIG_CMD_CREATE,
            ptr::null(),
            ptr::null(),
            0,
        ));
    }

    let mountfd = unsafe { fsmount(fsfd, FSMOUNT_CLOEXEC, 0) };
    unsafe { ASSERT_SUCCESS(mountfd) };

    unsafe {
        ASSERT_ERRNO_EQ(-ENOENT, faccessat(mountfd, c"1/".as_ptr(), X_OK, 0));
        ASSERT_ERRNO_EQ(-ENOENT, faccessat(mountfd, c"self/".as_ptr(), X_OK, 0));

        ASSERT_SUCCESS(close(fsfd));
        ASSERT_SUCCESS(close(mountfd));
    }
}

unsafe fn ns_pidns_reconfigure_remount(_self: *mut ns) {
    unsafe {
        ASSERT_SUCCESS(mkdir(c"/tmp/proc".as_ptr(), 0o755));
        ASSERT_SUCCESS(mount(
            c"proc".as_ptr(),
            c"/tmp/proc".as_ptr(),
            c"proc".as_ptr(),
            0,
            c"".as_ptr() as *const c_void,
        ));

        ASSERT_SUCCESS(access(c"/tmp/proc/1/".as_ptr(), X_OK));
        ASSERT_SUCCESS(access(c"/tmp/proc/self/".as_ptr(), X_OK));

        ASSERT_ERRNO_EQ(
            -EBUSY,
            mount(
                ptr::null(),
                c"/tmp/proc".as_ptr(),
                ptr::null(),
                MS_REMOUNT,
                c"pidns=/tmp/dummy/pidns".as_ptr() as *const c_void,
            ),
        );

        ASSERT_SUCCESS(access(c"/tmp/proc/1/".as_ptr(), X_OK));
        ASSERT_SUCCESS(access(c"/tmp/proc/self/".as_ptr(), X_OK));
    }
}

unsafe fn ns_pidns_reconfigure_fsconfig_string_path(_self: *mut ns) {
    let fsfd = unsafe { fsopen(c"proc".as_ptr(), FSOPEN_CLOEXEC) };
    unsafe { ASSERT_SUCCESS(fsfd) };

    unsafe {
        ASSERT_SUCCESS(fsconfig(
            fsfd,
            FSCONFIG_CMD_CREATE,
            ptr::null(),
            ptr::null(),
            0,
        ));
    }

    let mountfd = unsafe { fsmount(fsfd, FSMOUNT_CLOEXEC, 0) };
    unsafe { ASSERT_SUCCESS(mountfd) };

    unsafe {
        ASSERT_SUCCESS(faccessat(mountfd, c"1/".as_ptr(), X_OK, 0));
        ASSERT_SUCCESS(faccessat(mountfd, c"self/".as_ptr(), X_OK, 0));

        ASSERT_ERRNO_EQ(
            -EBUSY,
            fsconfig(
                fsfd,
                FSCONFIG_SET_STRING,
                c"pidns".as_ptr(),
                c"/tmp/dummy/pidns".as_ptr() as *const c_void,
                0,
            ),
        );
        ASSERT_SUCCESS(fsconfig(
            fsfd,
            FSCONFIG_CMD_RECONFIGURE,
            ptr::null(),
            ptr::null(),
            0,
        )); /* noop */

        ASSERT_SUCCESS(faccessat(mountfd, c"1/".as_ptr(), X_OK, 0));
        ASSERT_SUCCESS(faccessat(mountfd, c"self/".as_ptr(), X_OK, 0));

        ASSERT_SUCCESS(close(fsfd));
        ASSERT_SUCCESS(close(mountfd));
    }
}

unsafe fn ns_pidns_reconfigure_fsconfig_fd(self_: *mut ns) {
    let fsfd = unsafe { fsopen(c"proc".as_ptr(), FSOPEN_CLOEXEC) };
    unsafe { ASSERT_SUCCESS(fsfd) };

    unsafe {
        ASSERT_SUCCESS(fsconfig(
            fsfd,
            FSCONFIG_CMD_CREATE,
            ptr::null(),
            ptr::null(),
            0,
        ));
    }

    let mountfd = unsafe { fsmount(fsfd, FSMOUNT_CLOEXEC, 0) };
    unsafe { ASSERT_SUCCESS(mountfd) };

    unsafe {
        ASSERT_SUCCESS(faccessat(mountfd, c"1/".as_ptr(), X_OK, 0));
        ASSERT_SUCCESS(faccessat(mountfd, c"self/".as_ptr(), X_OK, 0));

        ASSERT_ERRNO_EQ(
            -EBUSY,
            fsconfig(
                fsfd,
                FSCONFIG_SET_FD,
                c"pidns".as_ptr(),
                ptr::null(),
                (*self_).dummy_pidns,
            ),
        );
        ASSERT_SUCCESS(fsconfig(
            fsfd,
            FSCONFIG_CMD_RECONFIGURE,
            ptr::null(),
            ptr::null(),
            0,
        )); /* noop */

        ASSERT_SUCCESS(faccessat(mountfd, c"1/".as_ptr(), X_OK, 0));
        ASSERT_SUCCESS(faccessat(mountfd, c"self/".as_ptr(), X_OK, 0));

        ASSERT_SUCCESS(close(fsfd));
        ASSERT_SUCCESS(close(mountfd));
    }
}

fn main() {
    /*
     * TEST_HARNESS_MAIN
     *
     * The original C file relies on kselftest_harness.h to discover and run
     * TEST_F(ns, ...) bodies with FIXTURE_SETUP/FIXTURE_TEARDOWN. This file
     * preserves the translated test bodies and fixture routines without
     * reimplementing the external harness.
     */
}
