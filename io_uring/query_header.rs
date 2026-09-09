// SPDX-License-Identifier: GPL-2.0

// Dependency supplied by the translated Linux io_uring types.

use core::ffi::{c_uint, c_void};

unsafe extern "C" {
    pub fn io_query(arg: *mut c_void, nr_args: c_uint) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
