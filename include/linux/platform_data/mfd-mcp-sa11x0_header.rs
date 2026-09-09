/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright (C) 2005 Russell King.
 */

// Dependency equivalent of <linux/types.h> is supplied externally.

#[repr(C)]
pub struct mcp_plat_data {
    pub mccr0: u32,
    pub mccr1: u32,
    pub sclk_rate: ::core::ffi::c_uint,
    pub codec_pdata: *mut ::core::ffi::c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
