/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Defines for Multi-Channel Buffered Serial Port
 *
 * Copyright (C) 2002 RidgeRun, Inc.
 * Author: Steve Johnson
 */

// Dependencies supplied by the surrounding translation unit:
// #include <linux/spinlock.h>
// #include <linux/clk.h>

/* Platform specific configuration */
#[repr(C)]
pub struct omap_mcbsp_ops {
    pub request: Option<unsafe extern "C" fn(unsigned int)>,
    pub free: Option<unsafe extern "C" fn(unsigned int)>,
}

#[repr(C)]
pub struct omap_mcbsp_platform_data {
    pub ops: *mut omap_mcbsp_ops,
    pub buffer_size: u16,
    pub reg_size: u8,
    pub reg_step: u8,

    /* McBSP platform and instance specific features */
    pub has_wakeup: bool, /* Wakeup capability */
    pub has_ccr: bool, /* Transceiver has configuration control registers */
    pub force_ick_on: Option<unsafe extern "C" fn(clk: *mut clk, force_on: bool) -> i32>,
}

/* External type supplied by <linux/clk.h>. */
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn omap3_mcbsp_init_pdata_callback(pdata: *mut omap_mcbsp_platform_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
