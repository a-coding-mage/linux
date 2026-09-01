/* SPDX-License-Identifier: GPL-2.0-only */

use core::ffi::{c_char, c_int, c_uint};

/* Default timeout */
pub const TIMEOUT: c_int = 10; /* seconds */

unsafe extern "C" {
    pub fn sigalrm(signo: c_int);
    pub fn timeout_begin(seconds: c_uint);
    pub fn timeout_check(operation: *const c_char);
    pub fn timeout_end();
    pub fn timeout_usleep(usec: crate::useconds_t) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
