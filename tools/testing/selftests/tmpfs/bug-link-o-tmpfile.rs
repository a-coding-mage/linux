/*
 * Copyright (c) 2019 Alexey Dobriyan <adobriyan@gmail.com>
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */
/* Test that open(O_TMPFILE), linkat() doesn't screw accounting. */

use std::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use std::ptr;

type mode_t = c_uint;

const CLONE_NEWNS: c_int = 0x00020000;
const ENOSYS: c_int = 38;
const EPERM: c_int = 1;
const MS_PRIVATE: c_ulong = 1 << 18;
const MS_REC: c_ulong = 16_384;
const AT_FDCWD: c_int = -100;
const AT_EMPTY_PATH: c_int = 0x1000;
const O_WRONLY: c_int = 1;
const O_TMPFILE: c_int = 0o20200000;

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn geteuid() -> c_uint;
    fn unshare(flags: c_int) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, mode: mode_t) -> c_int;
    fn linkat(
        olddirfd: c_int,
        oldpath: *const c_char,
        newdirfd: c_int,
        newpath: *const c_char,
        flags: c_int,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;

    /* From kselftest.h. */
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_exit_skip(fmt: *const c_char, ...);
    fn ksft_exit_fail_msg(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_exit_pass();
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

fn main() -> c_int {
    let mut fd: c_int;

    unsafe {
        // Setting up kselftest framework
        ksft_print_header();
        ksft_set_plan(1);

        // Check if test is run as root
        if geteuid() != 0 {
            ksft_exit_skip(c"This test needs root to run!\n".as_ptr());
            return 1;
        }

        if unshare(CLONE_NEWNS) == -1 {
            if errno() == ENOSYS || errno() == EPERM {
                ksft_exit_skip(c"unshare() error: unshare, errno %d\n".as_ptr(), errno());
            } else {
                ksft_exit_fail_msg(c"unshare() error: unshare, errno %d\n".as_ptr(), errno());
            }
        }

        if mount(
            ptr::null(),
            c"/".as_ptr(),
            ptr::null(),
            MS_PRIVATE | MS_REC,
            ptr::null(),
        ) == -1
        {
            ksft_exit_fail_msg(
                c"mount() error: Root filesystem private mount: Fail %d\n".as_ptr(),
                errno(),
            );
        }

        /* Our heroes: 1 root inode, 1 O_TMPFILE inode, 1 permanent inode. */
        if mount(
            ptr::null(),
            c"/tmp".as_ptr(),
            c"tmpfs".as_ptr(),
            0,
            c"nr_inodes=3".as_ptr() as *const c_void,
        ) == -1
        {
            ksft_exit_fail_msg(
                c"mount() error: Mounting tmpfs on /tmp: Fail %d\n".as_ptr(),
                errno(),
            );
        }

        fd = openat(AT_FDCWD, c"/tmp".as_ptr(), O_WRONLY | O_TMPFILE, 0o600);
        if fd == -1 {
            ksft_exit_fail_msg(
                c"openat() error: Open first temporary file: Fail %d\n".as_ptr(),
                errno(),
            );
        }

        if linkat(fd, c"".as_ptr(), AT_FDCWD, c"/tmp/1".as_ptr(), AT_EMPTY_PATH) == -1 {
            ksft_exit_fail_msg(
                c"linkat() error: Linking the temporary file: Fail %d\n".as_ptr(),
                errno(),
            );
            /* Ensure fd is closed on failure */
            close(fd);
        }
        close(fd);

        fd = openat(AT_FDCWD, c"/tmp".as_ptr(), O_WRONLY | O_TMPFILE, 0o600);
        if fd == -1 {
            ksft_exit_fail_msg(
                c"openat() error: Opening the second temporary file: Fail %d\n".as_ptr(),
                errno(),
            );
        }
        ksft_test_result_pass(c" ".as_ptr());
        ksft_exit_pass();
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
