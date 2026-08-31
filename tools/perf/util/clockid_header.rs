/* SPDX-License-Identifier: GPL-2.0 */

use std::os::raw::{c_char, c_int};

// From <time.h>.
pub type clockid_t = libc::clockid_t;

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn parse_clockid(opt: *const option, str: *const c_char, unset: c_int) -> c_int;

    pub fn clockid_name(clk_id: clockid_t) -> *const c_char;
}
