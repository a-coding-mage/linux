// SPDX-License-Identifier: GPL-2.0
//
// C dependencies from the original header:
// - <linux/types.h> provides loff_t and u64.
// - <sys/types.h> provides mode_t.
// - <fcntl.h> is included for the C interface context.

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct nsinfo {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn copyfile(from: *const c_char, to: *const c_char) -> c_int;
    pub fn copyfile_mode(from: *const c_char, to: *const c_char, mode: mode_t) -> c_int;
    pub fn copyfile_ns(from: *const c_char, to: *const c_char, nsi: *mut nsinfo) -> c_int;
    pub fn copyfile_offset(ifd: c_int, off_in: loff_t, ofd: c_int, off_out: loff_t, size: u64) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
