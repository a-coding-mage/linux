/* SPDX-License-Identifier: GPL-2.0 */

use std::os::raw::{c_char, c_int, c_ulong};

pub const NUM_SPARKS: c_int = 8;

extern "C" {
    pub fn print_spark(bf: *mut c_char, size: c_int, val: *mut c_ulong, numval: c_int) -> c_int;
}
