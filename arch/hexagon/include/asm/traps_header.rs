/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Trap support for Hexagon
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// Dependency equivalent of: #include <asm/registers.h>

use core::ffi::{c_char, c_long};

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn die(str_: *const c_char, regs: *mut pt_regs, err: c_long) -> core::ffi::c_int;
    pub fn die_if_kernel(str_: *mut c_char, regs: *mut pt_regs, err: c_long) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
