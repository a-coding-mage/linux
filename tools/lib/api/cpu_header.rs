// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_int, c_ulonglong};

unsafe extern "C" {
    pub fn cpu__get_max_freq(freq: *mut c_ulonglong) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
