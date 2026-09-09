/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP4 Clock Management (CM) definitions
 *
 * Copyright (C) 2007-2011 Texas Instruments, Inc.
 * Copyright (C) 2007-2009 Nokia Corporation
 *
 * Written by Paul Walmsley
 *
 * OMAP4 has two separate CM blocks, CM1 and CM2.  This file contains
 * macros and function prototypes that are applicable to both.
 */

// Dependency intent from the original includes: "prcm-common.h", "cm.h".

pub const OMAP4_CM_CLKSTCTRL: u32 = 0x0000;
pub const OMAP4_CM_STATICDEP: u32 = 0x0004;

#[repr(C)]
pub struct omap_prcm_init_data {
    _private: [u8; 0],
}

extern "C" {
    pub fn omap4_cm_init(data: *const omap_prcm_init_data) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
