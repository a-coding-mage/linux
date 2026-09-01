// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8804.h  --  WM8804 S/PDIF transceiver driver
 *
 * Copyright 2010 Wolfson Microelectronics plc
 *
 * Author: Dimitris Papastamos <dp@opensource.wolfsonmicro.com>
 */

// C header dependency: <linux/regmap.h>

/*
 * Register values.
 */
pub const WM8804_RST_DEVID1: u32 = 0x00;
pub const WM8804_DEVID2: u32 = 0x01;
pub const WM8804_DEVREV: u32 = 0x02;
pub const WM8804_PLL1: u32 = 0x03;
pub const WM8804_PLL2: u32 = 0x04;
pub const WM8804_PLL3: u32 = 0x05;
pub const WM8804_PLL4: u32 = 0x06;
pub const WM8804_PLL5: u32 = 0x07;
pub const WM8804_PLL6: u32 = 0x08;
pub const WM8804_SPDMODE: u32 = 0x09;
pub const WM8804_INTMASK: u32 = 0x0A;
pub const WM8804_INTSTAT: u32 = 0x0B;
pub const WM8804_SPDSTAT: u32 = 0x0C;
pub const WM8804_RXCHAN1: u32 = 0x0D;
pub const WM8804_RXCHAN2: u32 = 0x0E;
pub const WM8804_RXCHAN3: u32 = 0x0F;
pub const WM8804_RXCHAN4: u32 = 0x10;
pub const WM8804_RXCHAN5: u32 = 0x11;
pub const WM8804_SPDTX1: u32 = 0x12;
pub const WM8804_SPDTX2: u32 = 0x13;
pub const WM8804_SPDTX3: u32 = 0x14;
pub const WM8804_SPDTX4: u32 = 0x15;
pub const WM8804_SPDTX5: u32 = 0x16;
pub const WM8804_GPO0: u32 = 0x17;
pub const WM8804_GPO1: u32 = 0x18;
pub const WM8804_GPO2: u32 = 0x1A;
pub const WM8804_AIFTX: u32 = 0x1B;
pub const WM8804_AIFRX: u32 = 0x1C;
pub const WM8804_SPDRX1: u32 = 0x1D;
pub const WM8804_PWRDN: u32 = 0x1E;

pub const WM8804_REGISTER_COUNT: u32 = 30;
pub const WM8804_MAX_REGISTER: u32 = 0x1E;

pub const WM8804_TX_CLKSRC_MCLK: u32 = 1;
pub const WM8804_TX_CLKSRC_PLL: u32 = 2;

pub const WM8804_CLKOUT_SRC_CLK1: u32 = 3;
pub const WM8804_CLKOUT_SRC_OSCCLK: u32 = 4;

pub const WM8804_CLKOUT_DIV: u32 = 1;
pub const WM8804_MCLK_DIV: u32 = 2;

pub const WM8804_MCLKDIV_256FS: u32 = 0;
pub const WM8804_MCLKDIV_128FS: u32 = 1;

unsafe extern "C" {
    pub static wm8804_regmap_config: regmap_config;
    pub static wm8804_pm: dev_pm_ops;

    pub fn wm8804_probe(dev: *mut device, regmap: *mut regmap) -> ::core::ffi::c_int;
    pub fn wm8804_remove(dev: *mut device);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
