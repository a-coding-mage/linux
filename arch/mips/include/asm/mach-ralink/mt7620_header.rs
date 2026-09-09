/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Parts of this file are based on Ralink's 2.6.21 BSP
 *
 * Copyright (C) 2008-2011 Gabor Juhos <juhosg@openwrt.org>
 * Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 * Copyright (C) 2013 John Crispin <john@phrozen.org>
 */

/* The following symbols are supplied by the surrounding platform code. */
extern "C" {
    static ralink_soc: ralink_soc_type;
    fn rt_sysc_r32(reg: u32) -> u32;
}

/* C dependency: KSEG1ADDR supplies the uncached address conversion. */
macro_rules! IOMEM {
    ($x:expr) => {{ KSEG1ADDR($x) as *mut core::ffi::c_void }};
}

pub const MT7620_SYSC_BASE: *mut core::ffi::c_void = IOMEM!(0x10000000);

pub const SYSC_REG_CHIP_NAME0: u32 = 0x00;
pub const SYSC_REG_CHIP_NAME1: u32 = 0x04;
pub const SYSC_REG_EFUSE_CFG: u32 = 0x08;
pub const SYSC_REG_CHIP_REV: u32 = 0x0c;
pub const SYSC_REG_SYSTEM_CONFIG0: u32 = 0x10;
pub const SYSC_REG_SYSTEM_CONFIG1: u32 = 0x14;

pub const MT7620_CHIP_NAME0: u32 = 0x3637544d;
pub const MT7620_CHIP_NAME1: u32 = 0x20203032;
pub const MT7628_CHIP_NAME1: u32 = 0x20203832;

pub const CHIP_REV_PKG_MASK: u32 = 0x1;
pub const CHIP_REV_PKG_SHIFT: u32 = 16;
pub const CHIP_REV_VER_MASK: u32 = 0xf;
pub const CHIP_REV_VER_SHIFT: u32 = 8;
pub const CHIP_REV_ECO_MASK: u32 = 0xf;

pub const SYSCFG0_DRAM_TYPE_MASK: u32 = 0x3;
pub const SYSCFG0_DRAM_TYPE_SHIFT: u32 = 4;
pub const SYSCFG0_DRAM_TYPE_SDRAM: u32 = 0;
pub const SYSCFG0_DRAM_TYPE_DDR1: u32 = 1;
pub const SYSCFG0_DRAM_TYPE_DDR2: u32 = 2;
pub const SYSCFG0_DRAM_TYPE_UNKNOWN: u32 = 3;

pub const SYSCFG0_DRAM_TYPE_DDR2_MT7628: u32 = 0;
pub const SYSCFG0_DRAM_TYPE_DDR1_MT7628: u32 = 1;

pub const MT7620_DRAM_BASE: u32 = 0x0;
pub const MT7620_SDRAM_SIZE_MIN: u32 = 2;
pub const MT7620_SDRAM_SIZE_MAX: u32 = 64;
pub const MT7620_DDR1_SIZE_MIN: u32 = 32;
pub const MT7620_DDR1_SIZE_MAX: u32 = 128;
pub const MT7620_DDR2_SIZE_MIN: u32 = 32;
pub const MT7620_DDR2_SIZE_MAX: u32 = 256;

pub unsafe fn is_mt76x8() -> i32 {
    (ralink_soc == MT762X_SOC_MT7628AN || ralink_soc == MT762X_SOC_MT7688) as i32
}

pub unsafe fn mt7620_get_eco() -> i32 {
    (rt_sysc_r32(SYSC_REG_CHIP_REV) & CHIP_REV_ECO_MASK) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
