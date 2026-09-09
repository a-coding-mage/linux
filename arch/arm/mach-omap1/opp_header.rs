/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/arch/arm/mach-omap1/opp.h
 *
 *  Copyright (C) 2004 - 2005 Nokia corporation
 *  Written by Tuukka Tikkanen <tuukka.tikkanen@elektrobit.com>
 *  Based on clocks.h by Tony Lindgren, Gordon McNutt and RidgeRun, Inc
 */

// Dependency equivalent of <linux/types.h>.

#[repr(C)]
pub struct mpu_rate {
    pub rate: core::ffi::c_ulong,
    pub xtal: core::ffi::c_ulong,
    pub pll_rate: core::ffi::c_ulong,
    pub ckctl_val: u16,
    pub dpllctl_val: u16,
    pub flags: u32,
}

extern "C" {
    pub static mut omap1_rate_table: [mpu_rate; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
