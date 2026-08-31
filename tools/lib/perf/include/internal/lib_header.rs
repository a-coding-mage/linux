/* SPDX-License-Identifier: GPL-2.0 */

// Translated from internal libperf header. C dependency intent: <sys/types.h>.

use libc::{c_int, c_uint, c_void, off_t, size_t, ssize_t};

extern "C" {
    pub static mut page_size: c_uint;

    pub fn readn(fd: c_int, buf: *mut c_void, n: size_t) -> ssize_t;
    pub fn writen(fd: c_int, buf: *const c_void, n: size_t) -> ssize_t;

    pub fn preadn(fd: c_int, buf: *mut c_void, n: size_t, offs: off_t) -> ssize_t;
}
