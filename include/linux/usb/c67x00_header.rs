// SPDX-License-Identifier: GPL-2.0+
/*
 * usb_c67x00.h: platform definitions for the Cypress C67X00 USB chip
 *
 * Copyright (C) 2006-2008 Barco N.V.
 */

/* SIE configuration */
pub const C67X00_SIE_UNUSED: i32 = 0;
pub const C67X00_SIE_HOST: i32 = 1;
pub const C67X00_SIE_PERIPHERAL_A: i32 = 2; /* peripheral on A port */
pub const C67X00_SIE_PERIPHERAL_B: i32 = 3; /* peripheral on B port */

#[inline]
pub const fn c67x00_sie_config(config: i32, n: i32) -> i32 {
    (config >> (4 * n)) & 0x3
}

pub const C67X00_SIE1_UNUSED: i32 = C67X00_SIE_UNUSED << 0;
pub const C67X00_SIE1_HOST: i32 = C67X00_SIE_HOST << 0;
pub const C67X00_SIE1_PERIPHERAL_A: i32 = C67X00_SIE_PERIPHERAL_A << 0;
pub const C67X00_SIE1_PERIPHERAL_B: i32 = C67X00_SIE_PERIPHERAL_B << 0;

pub const C67X00_SIE2_UNUSED: i32 = C67X00_SIE_UNUSED << 4;
pub const C67X00_SIE2_HOST: i32 = C67X00_SIE_HOST << 4;
pub const C67X00_SIE2_PERIPHERAL_A: i32 = C67X00_SIE_PERIPHERAL_A << 4;
pub const C67X00_SIE2_PERIPHERAL_B: i32 = C67X00_SIE_PERIPHERAL_B << 4;

#[repr(C)]
pub struct c67x00_platform_data {
    pub sie_config: core::ffi::c_int, /* SIEs config (C67X00_SIEx_*) */
    pub hpi_regstep: core::ffi::c_ulong, /* Step between HPI registers */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
