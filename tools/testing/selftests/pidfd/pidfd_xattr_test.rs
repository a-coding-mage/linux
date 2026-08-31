// SPDX-License-Identifier: GPL-2.0

// C dependencies originally included:
// errno.h, fcntl.h, limits.h, linux/types.h, poll.h, pthread.h, sched.h,
// signal.h, stdio.h, stdlib.h, string.h, syscall.h, sys/prctl.h, sys/wait.h,
// unistd.h, sys/socket.h, linux/kcmp.h, sys/stat.h, sys/xattr.h,
// "pidfd.h", and "kselftest_harness.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type pid_t = c_int;
type idtype_t = c_uint;
type size_t = usize;
type ssize_t = isize;

const CLONE_NEWUSER: c_int = 0x10000000;
const CLONE_NEWPID: c_int = 0x20000000;
const EXIT_SUCCESS: c_int = 0;
const PATH_MAX: usize = 4096;
const P_PID: idtype_t = 1;
const WEXITED: c_int = 0x00000004;
const EBADF: c_int = 9;
const ENODATA: c_int = 61;

unsafe extern "C" {
    fn create_child(pidfd: *mut c_int, flags: c_int) -> pid_t;
    fn sys_waitid(which: idtype_t, pid: pid_t, infop: *mut c_void, options: c_int) -> c_int;
    fn sys_pidfd_open(pid: pid_t, flags: c_uint) -> c_int;

    fn _exit(status: c_int) -> !;
    fn close(fd: c_int) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;

    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;

    fn fsetxattr(
        fd: c_int,
        name: *const c_char,
        value: *const c_void,
        size: size_t,
        flags: c_int,
    ) -> c_int;
    fn fgetxattr(fd: c_int, name: *const c_char, value: *mut c_void, size: size_t) -> ssize_t;
    fn flistxattr(fd: c_int, list: *mut c_char, size: size_t) -> ssize_t;
    fn fremovexattr(fd: c_int, name: *const c_char) -> c_int;

    static mut errno: c_int;
}

#[repr(C)]
struct pidfs_xattr {
    child_pid: pid_t,
    child_pidfd: c_int,
}

unsafe fn pidfs_xattr_setup(self_: *mut pidfs_xattr) {
    unsafe {
        (*self_).child_pid = create_child(
            &mut (*self_).child_pidfd,
            CLONE_NEWUSER | CLONE_NEWPID,
        );
        EXPECT_GE!((*self_).child_pid, 0);

        if (*self_).child_pid == 0 {
            _exit(EXIT_SUCCESS);
        }
    }
}

unsafe fn pidfs_xattr_teardown(self_: *mut pidfs_xattr) {
    unsafe {
        sys_waitid(P_PID, (*self_).child_pid, ptr::null_mut(), WEXITED);
    }
}

unsafe fn pidfs_xattr_set_get_list_xattr_multiple(self_: *mut pidfs_xattr) {
    unsafe {
        let mut ret: c_int;
        let mut i: c_int;
        let mut xattr_name: [c_char; 32] = [0; 32];
        let mut xattr_value: [c_char; 32] = [0; 32];
        let mut buf: [c_char; 32] = [0; 32];
        const num_xattrs: c_int = 10;
        let mut list: [c_char; PATH_MAX] = [0; PATH_MAX];

        i = 0;
        while i < num_xattrs {
            snprintf(
                xattr_name.as_mut_ptr(),
                size_of::<[c_char; 32]>(),
                b"trusted.testattr%d\0".as_ptr() as *const c_char,
                i,
            );
            snprintf(
                xattr_value.as_mut_ptr(),
                size_of::<[c_char; 32]>(),
                b"testvalue%d\0".as_ptr() as *const c_char,
                i,
            );
            ret = fsetxattr(
                (*self_).child_pidfd,
                xattr_name.as_ptr(),
                xattr_value.as_ptr() as *const c_void,
                strlen(xattr_value.as_ptr()),
                0,
            );
            ASSERT_EQ!(ret, 0);
            i += 1;
        }

        i = 0;
        while i < num_xattrs {
            snprintf(
                xattr_name.as_mut_ptr(),
                size_of::<[c_char; 32]>(),
                b"trusted.testattr%d\0".as_ptr() as *const c_char,
                i,
            );
            snprintf(
                xattr_value.as_mut_ptr(),
                size_of::<[c_char; 32]>(),
                b"testvalue%d\0".as_ptr() as *const c_char,
                i,
            );
            memset(
                buf.as_mut_ptr() as *mut c_void,
                0,
                size_of::<[c_char; 32]>(),
            );
            ret = fgetxattr(
                (*self_).child_pidfd,
                xattr_name.as_ptr(),
                buf.as_mut_ptr() as *mut c_void,
                size_of::<[c_char; 32]>(),
            ) as c_int;
            ASSERT_EQ!(ret, strlen(xattr_value.as_ptr()) as c_int);
            ASSERT_EQ!(strcmp(buf.as_ptr(), xattr_value.as_ptr()), 0);
            i += 1;
        }

        ret = flistxattr(
            (*self_).child_pidfd,
            list.as_mut_ptr(),
            size_of::<[c_char; PATH_MAX]>(),
        ) as c_int;
        ASSERT_GT!(ret, 0);
        i = 0;
        while i < num_xattrs {
            snprintf(
                xattr_name.as_mut_ptr(),
                size_of::<[c_char; 32]>(),
                b"trusted.testattr%d\0".as_ptr() as *const c_char,
                i,
            );
            let mut found: bool = false;
            let mut it: *mut c_char = list.as_mut_ptr();
            while it < list.as_mut_ptr().add(ret as usize) {
                if strcmp(it, xattr_name.as_ptr()) != 0 {
                    it = it.add(strlen(it) + 1);
                    continue;
                }
                found = true;
                break;
            }
            ASSERT_TRUE!(found);
            i += 1;
        }

        i = 0;
        while i < num_xattrs {
            snprintf(
                xattr_name.as_mut_ptr(),
                size_of::<[c_char; 32]>(),
                b"trusted.testattr%d\0".as_ptr() as *const c_char,
                i,
            );
            ret = fremovexattr((*self_).child_pidfd, xattr_name.as_ptr());
            ASSERT_EQ!(ret, 0);

            ret = fgetxattr(
                (*self_).child_pidfd,
                xattr_name.as_ptr(),
                buf.as_mut_ptr() as *mut c_void,
                size_of::<[c_char; 32]>(),
            ) as c_int;
            ASSERT_EQ!(ret, -1);
            ASSERT_EQ!(errno, ENODATA);
            i += 1;
        }
    }
}

unsafe fn pidfs_xattr_set_get_list_xattr_persistent(self_: *mut pidfs_xattr) {
    unsafe {
        let mut ret: c_int;
        let mut buf: [c_char; 32] = [0; 32];
        let mut list: [c_char; PATH_MAX] = [0; PATH_MAX];

        ret = fsetxattr(
            (*self_).child_pidfd,
            b"trusted.persistent\0".as_ptr() as *const c_char,
            b"persistent value\0".as_ptr() as *const c_void,
            strlen(b"persistent value\0".as_ptr() as *const c_char),
            0,
        );
        ASSERT_EQ!(ret, 0);

        memset(
            buf.as_mut_ptr() as *mut c_void,
            0,
            size_of::<[c_char; 32]>(),
        );
        ret = fgetxattr(
            (*self_).child_pidfd,
            b"trusted.persistent\0".as_ptr() as *const c_char,
            buf.as_mut_ptr() as *mut c_void,
            size_of::<[c_char; 32]>(),
        ) as c_int;
        ASSERT_EQ!(
            ret,
            strlen(b"persistent value\0".as_ptr() as *const c_char) as c_int
        );
        ASSERT_EQ!(
            strcmp(
                buf.as_ptr(),
                b"persistent value\0".as_ptr() as *const c_char
            ),
            0
        );

        ret = flistxattr(
            (*self_).child_pidfd,
            list.as_mut_ptr(),
            size_of::<[c_char; PATH_MAX]>(),
        ) as c_int;
        ASSERT_GT!(ret, 0);
        ASSERT_EQ!(
            strcmp(
                list.as_ptr(),
                b"trusted.persistent\0".as_ptr() as *const c_char
            ),
            0
        );

        ASSERT_EQ!(close((*self_).child_pidfd), 0);
        (*self_).child_pidfd = -EBADF;
        sleep(2);

        (*self_).child_pidfd = sys_pidfd_open((*self_).child_pid, 0);
        ASSERT_GE!((*self_).child_pidfd, 0);

        memset(
            buf.as_mut_ptr() as *mut c_void,
            0,
            size_of::<[c_char; 32]>(),
        );
        ret = fgetxattr(
            (*self_).child_pidfd,
            b"trusted.persistent\0".as_ptr() as *const c_char,
            buf.as_mut_ptr() as *mut c_void,
            size_of::<[c_char; 32]>(),
        ) as c_int;
        ASSERT_EQ!(
            ret,
            strlen(b"persistent value\0".as_ptr() as *const c_char) as c_int
        );
        ASSERT_EQ!(
            strcmp(
                buf.as_ptr(),
                b"persistent value\0".as_ptr() as *const c_char
            ),
            0
        );

        ret = flistxattr(
            (*self_).child_pidfd,
            list.as_mut_ptr(),
            size_of::<[c_char; PATH_MAX]>(),
        ) as c_int;
        ASSERT_GT!(ret, 0);
        ASSERT_EQ!(
            strcmp(
                list.as_ptr(),
                b"trusted.persistent\0".as_ptr() as *const c_char
            ),
            0
        );
    }
}

TEST_HARNESS_MAIN!();
