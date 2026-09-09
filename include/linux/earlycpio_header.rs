/* SPDX-License-Identifier: GPL-2.0 */

// Dependency corresponding to <linux/types.h>.
use core::ffi::{c_char, c_long, c_void};

pub const MAX_CPIO_FILE_NAME: usize = 18;

#[repr(C)]
pub struct cpio_data {
    pub data: *mut c_void,
    pub size: usize,
    pub name: [c_char; MAX_CPIO_FILE_NAME],
}

extern "C" {
    pub fn find_cpio_data(
        path: *const c_char,
        data: *mut c_void,
        len: usize,
        offset: *mut c_long,
    ) -> cpio_data;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
