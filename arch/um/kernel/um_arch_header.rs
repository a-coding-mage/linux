/* SPDX-License-Identifier: GPL-2.0-only */

use core::ffi::{c_char, c_int, c_ulonglong, c_void};

extern "C" {
    pub fn uml_load_file(
        filename: *const c_char,
        size: *mut c_ulonglong,
    ) -> *mut c_void;
}

#[cfg(CONFIG_OF)]
extern "C" {
    pub fn uml_dtb_init();
}

#[cfg(not(CONFIG_OF))]
#[inline]
pub fn uml_dtb_init() {}

extern "C" {
    pub fn read_initrd() -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
