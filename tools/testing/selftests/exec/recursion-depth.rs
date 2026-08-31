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
/* Test that pointing #! script interpreter to self doesn't recurse. */

use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_ulong};
use std::ptr;

const ENOSYS: c_int = 38;
const EPERM: c_int = 1;
const ELOOP: c_int = 40;

const CLONE_NEWNS: c_int = 0x0002_0000;
const MS_REC: c_ulong = 16_384;
const MS_PRIVATE: c_ulong = 1 << 18;

const FILENAME: &[u8] = b"/tmp/1\0";
const S: &[u8] = b"#!/tmp/1\n\0";

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;

    fn unshare(flags: c_int) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn creat(pathname: *const c_char, mode: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn execve(
        pathname: *const c_char,
        argv: *const *const c_char,
        envp: *const *const c_char,
    ) -> c_int;
    fn strlen(s: *const c_char) -> usize;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_exit_fail_perror(msg: *const c_char) -> !;
    fn ksft_test_result(condition: bool, fmt: *const c_char, ...);
    fn ksft_finished() -> !;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

fn main() {
    let fd: c_int;
    let rv: c_int;

    unsafe {
        ksft_print_header();
        ksft_set_plan(1);

        if unshare(CLONE_NEWNS) == -1 {
            if errno() == ENOSYS || errno() == EPERM {
                ksft_test_result_skip(b"error: unshare, errno %d\n\0".as_ptr() as *const c_char, errno());
                ksft_finished();
            }
            ksft_exit_fail_perror(b"error: unshare\0".as_ptr() as *const c_char);
        }

        if mount(
            ptr::null(),
            b"/\0".as_ptr() as *const c_char,
            ptr::null(),
            MS_PRIVATE | MS_REC,
            ptr::null(),
        ) == -1
        {
            ksft_exit_fail_perror(b"error: mount '/'\0".as_ptr() as *const c_char);
        }

        /* Require "exec" filesystem. */
        if mount(
            ptr::null(),
            b"/tmp\0".as_ptr() as *const c_char,
            b"ramfs\0".as_ptr() as *const c_char,
            0,
            ptr::null(),
        ) == -1
        {
            ksft_exit_fail_perror(b"error: mount ramfs\0".as_ptr() as *const c_char);
        }

        fd = creat(FILENAME.as_ptr() as *const c_char, 0o700);
        if fd == -1 {
            ksft_exit_fail_perror(b"error: creat\0".as_ptr() as *const c_char);
        }

        let s_len = strlen(S.as_ptr() as *const c_char);
        if write(fd, S.as_ptr() as *const c_void, s_len) != s_len as isize {
            ksft_exit_fail_perror(b"error: write\0".as_ptr() as *const c_char);
        }

        close(fd);

        rv = execve(
            FILENAME.as_ptr() as *const c_char,
            ptr::null(),
            ptr::null(),
        );
        ksft_test_result(
            rv == -1 && errno() == ELOOP,
            b"execve failed as expected (ret %d, errno %d)\n\0".as_ptr() as *const c_char,
            rv,
            errno(),
        );
        ksft_finished();
    }
}
