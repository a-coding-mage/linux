/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023 Nuvoton Technology Corp.
 * Author: Chi-Fang Li <cfli0@nuvoton.com>
 */

/* Translated from clk-ma35d1.h. C header guards and includes are omitted. */

use core::ffi::{c_char, c_void};

/* Opaque types supplied by the surrounding dependency set. */
pub type clk_hw = c_void;
pub type device = c_void;
pub type spinlock_t = c_void;

extern "C" {
    pub fn ma35d1_reg_clk_pll(
        dev: *mut device,
        id: u32,
        u8mode: u8,
        name: *const c_char,
        parent_hw: *mut clk_hw,
        base: *mut c_void,
    ) -> *mut clk_hw;

    pub fn ma35d1_reg_adc_clkdiv(
        dev: *mut device,
        name: *const c_char,
        parent_hw: *mut clk_hw,
        lock: *mut spinlock_t,
        flags: usize,
        reg: *mut c_void,
        shift: u8,
        width: u8,
        mask_bit: u32,
    ) -> *mut clk_hw;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
