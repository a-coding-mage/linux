/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Shared macros and function prototypes for the HDQ1W/1-wire IP block
 *
 * Copyright (C) 2012 Texas Instruments, Inc.
 * Paul Walmsley
 */

/* Dependency supplied by the corresponding translated OMAP hwmod definitions. */

/*
 * XXX A future cleanup patch should modify
 * drivers/w1/masters/omap_hdq.c to use these macros
 */
pub const HDQ_CTRL_STATUS_OFFSET: u32 = 0x0c;
pub const HDQ_CTRL_STATUS_CLOCKENABLE_SHIFT: u32 = 5;

unsafe extern "C" {
    pub fn omap_hdq1w_reset(oh: *mut omap_hwmod) -> i32;
}

#[repr(C)]
pub struct omap_hwmod {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
