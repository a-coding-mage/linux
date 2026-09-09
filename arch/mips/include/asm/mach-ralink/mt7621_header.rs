/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright (C) 2015 John Crispin <john@phrozen.org>
 */

// C header guard: _MT7621_REGS_H_

// #define IOMEM(x) ((void __iomem *)(KSEG1ADDR(x)))
// KSEG1ADDR is supplied by an external dependency.
#[macro_export]
macro_rules! IOMEM {
    ($x:expr) => {
        KSEG1ADDR($x) as *mut core::ffi::c_void
    };
}

pub const MT7621_PALMBUS_BASE: u32 = 0x1C000000;
pub const MT7621_PALMBUS_SIZE: u32 = 0x03FFFFFF;

// #define MT7621_SYSC_BASE IOMEM(0x1E000000)
#[macro_export]
macro_rules! MT7621_SYSC_BASE {
    () => {
        IOMEM!(0x1E000000u32)
    };
}

pub const SYSC_REG_CHIP_NAME0: u32 = 0x00;
pub const SYSC_REG_CHIP_NAME1: u32 = 0x04;
pub const SYSC_REG_CHIP_REV: u32 = 0x0c;
pub const SYSC_REG_SYSTEM_CONFIG0: u32 = 0x10;
pub const SYSC_REG_SYSTEM_CONFIG1: u32 = 0x14;

pub const CHIP_REV_PKG_MASK: u32 = 0x1;
pub const CHIP_REV_PKG_SHIFT: u32 = 16;
pub const CHIP_REV_VER_MASK: u32 = 0xf;
pub const CHIP_REV_VER_SHIFT: u32 = 8;
pub const CHIP_REV_ECO_MASK: u32 = 0xf;

pub const MT7621_LOWMEM_BASE: u32 = 0x0;
pub const MT7621_LOWMEM_MAX_SIZE: u32 = 0x1C000000;
pub const MT7621_HIGHMEM_BASE: u32 = 0x20000000;
pub const MT7621_HIGHMEM_SIZE: u32 = 0x4000000;

pub const MT7621_CHIP_NAME0: u32 = 0x3637544D;
pub const MT7621_CHIP_NAME1: u32 = 0x20203132;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
