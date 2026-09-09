/* SPDX-License-Identifier: GPL-2.0
 *
 * SH Pin Function Control Initialization
 *
 * Copyright (C) 2012  Renesas Solutions Corp.
 */

// Translated from the C header. The original include of <linux/types.h>
// supplies u32; Rust's u32 is used directly here.

use core::ffi::c_char;

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn sh_pfc_register(
        name: *const c_char,
        resource: *mut resource,
        num_resources: u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
