// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct rtc_time {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn init_dragen2(command: *mut c_char, size: c_int);
    pub fn init_ucsimm(command: *mut c_char, size: c_int);
    pub fn m68328_hwclk(set: c_int, t: *mut rtc_time) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
