/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *
 *  Copyright (C) 2004 IDT Inc.
 *  Copyright (C) 2006 Felix Fietkau <nbd@openwrt.org>
 */

// The original header depends on the platform-provided KSEG1ADDR macro.
// It is retained as a Rust macro for use by dependent code.

pub const REGBASE: u32 = 0x18000000;

#[macro_export]
macro_rules! IDT434_REG_BASE {
    () => {
        KSEG1ADDR($crate::REGBASE)
    };
}

pub const UART0BASE: u32 = 0x58000;
pub const RST: i32 = 1 << 15;
pub const DEV0BASE: u32 = 0x010000;
pub const DEV0MASK: u32 = 0x010004;
pub const DEV0C: u32 = 0x010008;
pub const DEV0T: u32 = 0x01000C;
pub const DEV1BASE: u32 = 0x010010;
pub const DEV1MASK: u32 = 0x010014;
pub const DEV1C: u32 = 0x010018;
pub const DEV1TC: u32 = 0x01001C;
pub const DEV2BASE: u32 = 0x010020;
pub const DEV2MASK: u32 = 0x010024;
pub const DEV2C: u32 = 0x010028;
pub const DEV2TC: u32 = 0x01002C;
pub const DEV3BASE: u32 = 0x010030;
pub const DEV3MASK: u32 = 0x010034;
pub const DEV3C: u32 = 0x010038;
pub const DEV3TC: u32 = 0x01003C;
pub const BTCS: u32 = 0x010040;
pub const BTCOMPARE: u32 = 0x010044;
pub const LO_WPX: i32 = 1 << 0;
pub const LO_ALE: i32 = 1 << 1;
pub const LO_CLE: i32 = 1 << 2;
pub const LO_CEX: i32 = 1 << 3;
pub const LO_FOFF: i32 = 1 << 5;
pub const LO_SPICS: i32 = 1 << 6;
pub const LO_ULED: i32 = 1 << 7;

#[macro_export]
macro_rules! BIT_TO_MASK {
    ($x:expr) => {
        1 << $x
    };
}

#[repr(C)]
pub struct dev_reg {
    pub base: u32,
    pub mask: u32,
    pub ctl: u32,
    pub timing: u32,
}

#[repr(C)]
pub struct korina_device {
    pub name: *mut core::ffi::c_char,
    pub mac: [u8; 6],
    pub dev: *mut net_device,
}

#[repr(C)]
pub struct mpmc_device {
    pub state: u8,
    pub lock: spinlock_t,
    pub base: *mut core::ffi::c_void,
}

unsafe extern "C" {
    pub fn set_latch_u5(or_mask: u8, nand_mask: u8);
    pub fn get_latch_u5() -> u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
