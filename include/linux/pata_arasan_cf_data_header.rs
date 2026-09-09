/*
 * include/linux/pata_arasan_cf_data.h
 *
 * Arasan Compact Flash host controller platform data header file
 *
 * Copyright (C) 2011 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 *
 * This file is licensed under the terms of the GNU General
 * Public License version 2. This program is licensed "as is" without
 * any warranty of any kind, whether express or implied.
 */

// Dependency supplied by the platform-device definitions.

#[repr(C)]
pub struct arasan_cf_pdata {
    pub cf_if_clk: u8,
    pub quirk: u32,
}

pub const CF_IF_CLK_100M: u8 = 0x0;
pub const CF_IF_CLK_75M: u8 = 0x1;
pub const CF_IF_CLK_66M: u8 = 0x2;
pub const CF_IF_CLK_50M: u8 = 0x3;
pub const CF_IF_CLK_40M: u8 = 0x4;
pub const CF_IF_CLK_33M: u8 = 0x5;
pub const CF_IF_CLK_25M: u8 = 0x6;
pub const CF_IF_CLK_125M: u8 = 0x7;
pub const CF_IF_CLK_150M: u8 = 0x8;
pub const CF_IF_CLK_166M: u8 = 0x9;
pub const CF_IF_CLK_200M: u8 = 0xA;

/*
 * Platform specific incapabilities of CF controller is handled via
 * quirks
 */
pub const CF_BROKEN_PIO: u32 = 1;
pub const CF_BROKEN_MWDMA: u32 = 1 << 1;
pub const CF_BROKEN_UDMA: u32 = 1 << 2;

pub unsafe fn set_arasan_cf_pdata(
    pdev: *mut platform_device,
    data: *mut arasan_cf_pdata,
) {
    (*pdev).dev.platform_data = data;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
