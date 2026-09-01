/* SPDX-License-Identifier: GPL-2.0 */

/* C header dependency intent: <stdint.h> */

use std::os::raw::{c_char, c_int};

extern "C" {
    pub fn parse_branch_stack(opt: *const option, str: *const c_char, unset: c_int) -> c_int;
    pub fn parse_branch_str(str: *const c_char, mode: *mut __u64) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
