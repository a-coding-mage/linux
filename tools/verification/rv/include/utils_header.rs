// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int};

pub const MAX_PATH: usize = 1024;

unsafe extern "C" {
    pub fn debug_msg(fmt: *const c_char, ...);
    pub fn err_msg(fmt: *const c_char, ...);

    pub static mut config_debug: c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
