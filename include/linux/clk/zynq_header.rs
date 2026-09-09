/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2013 Xilinx Inc.
 * Copyright (C) 2012 National Instruments
 */

// Dependency corresponding to <linux/spinlock.h>.

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

extern "C" {
    pub fn zynq_clock_init();

    pub fn clk_register_zynq_pll(
        name: *const c_char,
        parent: *const c_char,
        pll_ctrl: *mut c_void,
        pll_status: *mut c_void,
        lock_index: u8,
        lock: *mut spinlock_t,
    ) -> *mut clk;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
