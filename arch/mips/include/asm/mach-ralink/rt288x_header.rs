/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Parts of this file are based on Ralink's 2.6.21 BSP
 *
 * Copyright (C) 2008-2011 Gabor Juhos <juhosg@openwrt.org>
 * Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 * Copyright (C) 2013 John Crispin <john@phrozen.org>
 */

// C header guard: _RT288X_REGS_H_

macro_rules! IOMEM {
    ($x:expr) => {
        (KSEG1ADDR($x) as *mut core::ffi::c_void)
    };
}

pub const RT2880_SYSC_BASE: *mut core::ffi::c_void = IOMEM!(0x00300000);

pub const SYSC_REG_CHIP_NAME0: u32 = 0x00;
pub const SYSC_REG_CHIP_NAME1: u32 = 0x04;
pub const SYSC_REG_CHIP_ID: u32 = 0x0c;
pub const SYSC_REG_SYSTEM_CONFIG: u32 = 0x10;

pub const RT2880_CHIP_NAME0: u32 = 0x38325452;
pub const RT2880_CHIP_NAME1: u32 = 0x20203038;

pub const CHIP_ID_ID_MASK: u32 = 0xff;
pub const CHIP_ID_ID_SHIFT: u32 = 8;
pub const CHIP_ID_REV_MASK: u32 = 0xff;

pub const RT2880_SDRAM_BASE: u32 = 0x08000000;
pub const RT2880_MEM_SIZE_MIN: u32 = 2;
pub const RT2880_MEM_SIZE_MAX: u32 = 128;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
