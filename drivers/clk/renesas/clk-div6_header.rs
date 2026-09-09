/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_void};

// External types supplied by the surrounding translation unit.
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct raw_notifier_head {
    _private: [u8; 0],
}

extern "C" {
    pub fn cpg_div6_register(
        name: *const c_char,
        num_parents: u32,
        parent_names: *const *const c_char,
        reg: *mut c_void,
        notifiers: *mut raw_notifier_head,
    ) -> *mut clk;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
