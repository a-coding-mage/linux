/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * (C) Copyright TOSHIBA CORPORATION 2007
 */

use core::ffi::c_ulong;

pub const NDFMC_PLAT_FLAG_USE_BSPRT: u32 = 0x01;
pub const NDFMC_PLAT_FLAG_NO_RSTR: u32 = 0x02;
pub const NDFMC_PLAT_FLAG_HOLDADD: u32 = 0x04;
pub const NDFMC_PLAT_FLAG_DUMMYWRITE: u32 = 0x08;

#[repr(C)]
pub struct txx9ndfmc_platform_data {
    pub shift: u32,
    pub gbus_clock: u32,
    pub hold: u32, /* hold time in nanosecond */
    pub spw: u32, /* strobe pulse width in nanosecond */
    pub flags: u32,
    pub ch_mask: u8, /* available channel bitmask */
    pub wp_mask: u8, /* write-protect bitmask */
    pub wide_mask: u8, /* 16bit-nand bitmask */
}

unsafe extern "C" {
    pub fn txx9_ndfmc_init(
        baseaddr: c_ulong,
        plat_data: *const txx9ndfmc_platform_data,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
