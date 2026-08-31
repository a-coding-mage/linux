// SPDX-License-Identifier: GPL-2.0

// C dependencies translated from:
// errno.h, stdbool.h, stdio.h, stdlib.h, string.h, sys/stat.h, unistd.h,
// linux/limits.h, and "kselftest.h".

use core::ffi::{c_char, c_int, c_uint, c_void};

const MIN_TTY_PATH_LEN: usize = 8;
const PATH_MAX: usize = 4096;
const EIO: c_int = 5;
const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;
const KSFT_SKIP: c_int = 4;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

#[repr(C)]
struct stat {
    st_dev: u64,
    st_ino: u64,
    st_nlink: u64,
    st_mode: c_uint,
    st_uid: c_uint,
    st_gid: c_uint,
    __pad0: c_int,
    st_rdev: u64,
    st_size: i64,
    st_blksize: i64,
    st_blocks: i64,
    st_atim: timespec,
    st_mtim: timespec,
    st_ctim: timespec,
    __glibc_reserved: [i64; 3],
}

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn ksft_finished();
    fn ksft_print_header();
    fn ksft_print_msg(msg: *const c_char, ...);
    fn ksft_set_plan(plan: c_uint);
    fn ksft_test_result_report(result: c_int, name: *const c_char, ...);
    fn readlink(pathname: *const c_char, buf: *mut c_char, bufsiz: usize) -> isize;
    fn sleep(seconds: c_uint) -> c_uint;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
}

unsafe fn tty_valid(tty: *mut c_char) -> bool {
    if unsafe { strlen(tty) } < MIN_TTY_PATH_LEN {
        return false;
    }

    if unsafe { strncmp(tty, c"/dev/tty".as_ptr(), MIN_TTY_PATH_LEN) } == 0
        || unsafe { strncmp(tty, c"/dev/pts".as_ptr(), MIN_TTY_PATH_LEN) } == 0
    {
        return true;
    }

    false
}

unsafe fn write_dev_tty() -> c_int {
    let f: *mut FILE;
    let mut r: c_int = 0;

    f = unsafe { fopen(c"/dev/tty".as_ptr(), c"r+".as_ptr()) };
    if f.is_null() {
        return -unsafe { *__errno_location() };
    }

    r = unsafe { fprintf(f, c"hello, world!\n".as_ptr()) };
    if r != unsafe { strlen(c"hello, world!\n".as_ptr()) } as c_int {
        r = -EIO;
    }

    unsafe {
        fclose(f);
    }
    r
}

fn main() {
    unsafe {
        let mut r: c_int;
        let mut tty: [c_char; PATH_MAX] = [0; PATH_MAX];
        let mut st1: stat = core::mem::zeroed();
        let mut st2: stat = core::mem::zeroed();
        let mut result: c_int = KSFT_FAIL;

        ksft_print_header();
        ksft_set_plan(1);

        r = readlink(c"/proc/self/fd/0".as_ptr(), tty.as_mut_ptr(), PATH_MAX) as c_int;
        if r < 0 {
            ksft_print_msg(c"readlink on /proc/self/fd/0 failed: %m\n".as_ptr());
            goto_out(result);
            return;
        }

        if !tty_valid(tty.as_mut_ptr()) {
            ksft_print_msg(c"invalid tty path '%s'\n".as_ptr(), tty.as_mut_ptr());
            result = KSFT_SKIP;
            goto_out(result);
            return;
        }

        r = stat(tty.as_mut_ptr(), &mut st1);
        if r < 0 {
            ksft_print_msg(c"stat failed on tty path '%s': %m\n".as_ptr(), tty.as_mut_ptr());
            goto_out(result);
            return;
        }

        /* We need to wait at least 8 seconds in order to observe timestamp change */
        /* https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=fbf47635315ab308c9b58a1ea0906e711a9228de */
        sleep(10);

        r = write_dev_tty();
        if r < 0 {
            ksft_print_msg(
                c"failed to write to /dev/tty: %s\n".as_ptr(),
                strerror(-r),
            );
            goto_out(result);
            return;
        }

        r = stat(tty.as_mut_ptr(), &mut st2);
        if r < 0 {
            ksft_print_msg(c"stat failed on tty path '%s': %m\n".as_ptr(), tty.as_mut_ptr());
            goto_out(result);
            return;
        }

        /* We wrote to the terminal so timestamps should have been updated */
        if st1.st_atim.tv_sec == st2.st_atim.tv_sec
            && st1.st_mtim.tv_sec == st2.st_mtim.tv_sec
        {
            ksft_print_msg(c"tty timestamps not updated\n".as_ptr());
            goto_out(result);
            return;
        }

        ksft_print_msg(
            c"timestamps of terminal '%s' updated after write to /dev/tty\n".as_ptr(),
            tty.as_mut_ptr(),
        );
        result = KSFT_PASS;

        goto_out(result);
    }
}

unsafe fn goto_out(result: c_int) {
    unsafe {
        ksft_test_result_report(result, c"tty_tstamp_update\n".as_ptr());

        ksft_finished();
    }
}
