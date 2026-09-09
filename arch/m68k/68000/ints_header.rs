/* SPDX-License-Identifier: GPL-2.0-only */

// C dependency: <linux/linkage.h>

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn process_int(vec: core::ffi::c_int, fp: *mut pt_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
