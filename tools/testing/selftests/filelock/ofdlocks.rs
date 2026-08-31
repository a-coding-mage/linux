// SPDX-License-Identifier: GPL-2.0

// #define _GNU_SOURCE
// #include <fcntl.h>
// #include <assert.h>
// #include <stdio.h>
// #include <unistd.h>
// #include <string.h>
// #include "kselftest.h"

use std::ffi::{c_char, c_int, c_long, c_uint, c_void};
use std::mem::{size_of, MaybeUninit};
use std::ptr;

const F_OFD_GETLK: c_int = 36;
const F_OFD_SETLK: c_int = 37;
const F_RDLCK: i16 = 0;
const F_WRLCK: i16 = 1;
const F_UNLCK: i16 = 2;
const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const O_CREAT: c_int = 0o100;
const O_EXCL: c_int = 0o200;
const SEEK_SET: i16 = 0;

#[repr(C)]
struct flock {
    l_type: i16,
    l_whence: i16,
    l_start: c_long,
    l_len: c_long,
    l_pid: c_int,
}

unsafe extern "C" {
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_print_msg(msg: *const c_char, ...);
    fn ksft_perror(msg: *const c_char);
    fn ksft_test_result(result: c_int, msg: *const c_char, ...);
    fn ksft_exit_fail_msg(msg: *const c_char, ...) -> !;
    fn ksft_exit_fail() -> !;
    fn ksft_finished() -> !;
}

unsafe fn lock_set(fd: c_int, fl: *mut flock) -> c_int {
    let ret: c_int;

    (*fl).l_pid = 0; // needed for OFD locks
    (*fl).l_whence = SEEK_SET;
    ret = fcntl(fd, F_OFD_SETLK, fl);
    if ret != 0 {
        ksft_perror(c"fcntl()".as_ptr());
    }
    ret
}

unsafe fn lock_get(fd: c_int, fl: *mut flock) -> c_int {
    let ret: c_int;

    (*fl).l_pid = 0; // needed for OFD locks
    (*fl).l_whence = SEEK_SET;
    ret = fcntl(fd, F_OFD_GETLK, fl);
    if ret != 0 {
        ksft_perror(c"fcntl()".as_ptr());
    }
    ret
}

fn main() {
    unsafe {
        let mut rc: c_int;
        let mut fl = MaybeUninit::<flock>::uninit();
        let mut fl2 = MaybeUninit::<flock>::uninit();
        let fd: c_int = open(c"/tmp/aa".as_ptr(), O_RDWR | O_CREAT | O_EXCL, 0o600);
        let fd2: c_int = open(c"/tmp/aa".as_ptr(), O_RDONLY);

        ksft_print_header();
        ksft_set_plan(4);

        unlink(c"/tmp/aa".as_ptr());
        assert!(fd != -1);
        assert!(fd2 != -1);
        ksft_print_msg(c"opened fds %i %i\n".as_ptr(), fd, fd2);

        /* Set some read lock */
        ptr::addr_of_mut!((*fl.as_mut_ptr()).l_type).write(F_RDLCK);
        ptr::addr_of_mut!((*fl.as_mut_ptr()).l_start).write(5);
        ptr::addr_of_mut!((*fl.as_mut_ptr()).l_len).write(3);
        rc = lock_set(fd, fl.as_mut_ptr());
        ksft_test_result((rc == 0) as c_int, c"set OFD read lock on first fd\n".as_ptr());
        if rc != 0 {
            ksft_finished();
        }

        /* Make sure read locks do not conflict on different fds. */
        ptr::addr_of_mut!((*fl.as_mut_ptr()).l_type).write(F_RDLCK);
        ptr::addr_of_mut!((*fl.as_mut_ptr()).l_start).write(5);
        ptr::addr_of_mut!((*fl.as_mut_ptr()).l_len).write(1);
        rc = lock_get(fd2, fl.as_mut_ptr());
        if rc != 0 {
            ksft_finished();
        }
        if (*fl.as_ptr()).l_type != F_UNLCK {
            ksft_exit_fail_msg(c"read locks conflicted\n".as_ptr());
        }

        /* Make sure read/write locks do conflict on different fds. */
        ptr::addr_of_mut!((*fl.as_mut_ptr()).l_type).write(F_WRLCK);
        ptr::addr_of_mut!((*fl.as_mut_ptr()).l_start).write(5);
        ptr::addr_of_mut!((*fl.as_mut_ptr()).l_len).write(1);
        rc = lock_get(fd2, fl.as_mut_ptr());
        if rc != 0 {
            ksft_finished();
        }
        ksft_test_result(
            ((*fl.as_ptr()).l_type != F_UNLCK) as c_int,
            c"read and write locks conflicted\n".as_ptr(),
        );
        if (*fl.as_ptr()).l_type == F_UNLCK {
            ksft_finished();
        }

        /* Get info about the lock on first fd. */
        ptr::addr_of_mut!((*fl.as_mut_ptr()).l_type).write(F_UNLCK);
        ptr::addr_of_mut!((*fl.as_mut_ptr()).l_start).write(5);
        ptr::addr_of_mut!((*fl.as_mut_ptr()).l_len).write(1);
        rc = lock_get(fd, fl.as_mut_ptr());
        if rc != 0 {
            ksft_exit_fail_msg(c"F_OFD_GETLK with F_UNLCK not supported\n".as_ptr());
        }
        ksft_test_result(
            ((*fl.as_ptr()).l_type != F_UNLCK) as c_int,
            c"F_OFD_GETLK with F_UNLCK returned lock info\n".as_ptr(),
        );
        if (*fl.as_ptr()).l_type == F_UNLCK {
            ksft_exit_fail();
        }
        ksft_print_msg(
            c"F_UNLCK test returns: locked, type %i pid %i len %zi\n".as_ptr(),
            (*fl.as_ptr()).l_type as c_int,
            (*fl.as_ptr()).l_pid,
            (*fl.as_ptr()).l_len,
        );

        /* Try the same but by locking everything by len==0. */
        ptr::addr_of_mut!((*fl2.as_mut_ptr()).l_type).write(F_UNLCK);
        ptr::addr_of_mut!((*fl2.as_mut_ptr()).l_start).write(0);
        ptr::addr_of_mut!((*fl2.as_mut_ptr()).l_len).write(0);
        rc = lock_get(fd, fl2.as_mut_ptr());
        if rc != 0 {
            ksft_exit_fail_msg(c"F_OFD_GETLK with F_UNLCK not supported\n".as_ptr());
        }
        ksft_test_result(
            (memcmp(
                fl.as_ptr() as *const c_void,
                fl2.as_ptr() as *const c_void,
                size_of::<flock>(),
            ) == 0) as c_int,
            c"F_UNLCK with len==0 returned the same\n".as_ptr(),
        );
        if memcmp(
            fl.as_ptr() as *const c_void,
            fl2.as_ptr() as *const c_void,
            size_of::<flock>(),
        ) != 0
        {
            ksft_exit_fail_msg(
                c"F_UNLCK test returns: locked, type %i pid %i len %zi\n".as_ptr(),
                (*fl.as_ptr()).l_type as c_int,
                (*fl.as_ptr()).l_pid,
                (*fl.as_ptr()).l_len,
            );
        }

        /* Get info about the lock on second fd - no locks on it. */
        ptr::addr_of_mut!((*fl.as_mut_ptr()).l_type).write(F_UNLCK);
        ptr::addr_of_mut!((*fl.as_mut_ptr()).l_start).write(0);
        ptr::addr_of_mut!((*fl.as_mut_ptr()).l_len).write(0);
        lock_get(fd2, fl.as_mut_ptr());
        ksft_test_result(
            ((*fl.as_ptr()).l_type == F_UNLCK) as c_int,
            c"F_OFD_GETLK with F_UNLCK return lock info from another fd\n".as_ptr(),
        );

        ksft_finished();
    }
}
