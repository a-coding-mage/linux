/* SPDX-License-Identifier: GPL-2.0 */
/*
 * MIPI Camera Control Interface (CCI) register access helpers.
 *
 * Copyright (C) 2023 Hans de Goede <hansg@kernel.org>
 */

use core::ffi::c_int;

#[repr(C)]
pub struct i2c_client {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

/**
 * struct cci_reg_sequence - An individual write from a sequence of CCI writes
 *
 * @reg: Register address, use CCI_REG#() macros to encode reg width
 * @val: Register value
 *
 * Register/value pairs for sequences of writes.
 */
#[repr(C)]
pub struct cci_reg_sequence {
    pub reg: u32,
    pub val: u64,
}

/* Macros to define register address with the register width encoded into the
 * higher bits. */
pub const CCI_REG_ADDR_MASK: u32 = 0xffff;
pub const CCI_REG_WIDTH_SHIFT: u32 = 16;
pub const CCI_REG_WIDTH_MASK: u32 = 0x000f_0000;

/* Private CCI register flags, for the use of drivers. */
pub const CCI_REG_PRIVATE_SHIFT: u32 = 28;
pub const CCI_REG_PRIVATE_MASK: u32 = 0xf000_0000;

#[inline]
pub const fn cci_reg_width_bytes(x: u32) -> u32 {
    (x & CCI_REG_WIDTH_MASK) >> CCI_REG_WIDTH_SHIFT
}

#[inline]
pub const fn cci_reg_width(x: u32) -> u32 {
    cci_reg_width_bytes(x) << 3
}

#[inline]
pub const fn cci_reg_addr(x: u32) -> u32 {
    x & CCI_REG_ADDR_MASK
}

pub const CCI_REG_LE: u32 = 1 << 20;

#[macro_export]
macro_rules! CCI_REG8 {
    ($x:expr) => { ((1u32 << $crate::CCI_REG_WIDTH_SHIFT) | ($x as u32)) };
}
#[macro_export]
macro_rules! CCI_REG16 {
    ($x:expr) => { ((2u32 << $crate::CCI_REG_WIDTH_SHIFT) | ($x as u32)) };
}
#[macro_export]
macro_rules! CCI_REG24 {
    ($x:expr) => { ((3u32 << $crate::CCI_REG_WIDTH_SHIFT) | ($x as u32)) };
}
#[macro_export]
macro_rules! CCI_REG32 {
    ($x:expr) => { ((4u32 << $crate::CCI_REG_WIDTH_SHIFT) | ($x as u32)) };
}
#[macro_export]
macro_rules! CCI_REG64 {
    ($x:expr) => { ((8u32 << $crate::CCI_REG_WIDTH_SHIFT) | ($x as u32)) };
}
#[macro_export]
macro_rules! CCI_REG16_LE {
    ($x:expr) => { ($crate::CCI_REG_LE | (2u32 << $crate::CCI_REG_WIDTH_SHIFT) | ($x as u32)) };
}
#[macro_export]
macro_rules! CCI_REG24_LE {
    ($x:expr) => { ($crate::CCI_REG_LE | (3u32 << $crate::CCI_REG_WIDTH_SHIFT) | ($x as u32)) };
}
#[macro_export]
macro_rules! CCI_REG32_LE {
    ($x:expr) => { ($crate::CCI_REG_LE | (4u32 << $crate::CCI_REG_WIDTH_SHIFT) | ($x as u32)) };
}
#[macro_export]
macro_rules! CCI_REG64_LE {
    ($x:expr) => { ($crate::CCI_REG_LE | (8u32 << $crate::CCI_REG_WIDTH_SHIFT) | ($x as u32)) };
}

extern "C" {
    /** Read a value from a single CCI register. */
    pub fn cci_read(map: *mut regmap, reg: u32, val: *mut u64, err: *mut c_int) -> c_int;

    /** Write a value to a single CCI register. */
    pub fn cci_write(map: *mut regmap, reg: u32, val: u64, err: *mut c_int) -> c_int;

    /** Perform a read/modify/write cycle on a single CCI register. */
    pub fn cci_update_bits(
        map: *mut regmap,
        reg: u32,
        mask: u64,
        val: u64,
        err: *mut c_int,
    ) -> c_int;

    /** Write multiple registers to the device. */
    pub fn cci_multi_reg_write(
        map: *mut regmap,
        regs: *const cci_reg_sequence,
        num_regs: u32,
        err: *mut c_int,
    ) -> c_int;
}

/* Preserves the CONFIG_V4L2_CCI_I2C build-time condition from the C header. */
#[cfg(feature = "CONFIG_V4L2_CCI_I2C")]
extern "C" {
    /** Create a regmap to use with cci_*() register access functions. */
    pub fn devm_cci_regmap_init_i2c(
        client: *mut i2c_client,
        reg_addr_bits: c_int,
    ) -> *mut regmap;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
