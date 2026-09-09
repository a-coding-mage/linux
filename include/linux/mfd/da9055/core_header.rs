/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * da9055 declarations for DA9055 PMICs.
 *
 * Copyright(c) 2012 Dialog Semiconductor Ltd.
 *
 * Author: David Dajun Chen <dchen@diasemi.com>
 */

use std::os::raw::{c_int, c_uchar, c_uint};

/* Dependencies supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_irq_chip_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    _private: [u8; 0],
}

pub struct da9055_pdata;

/*
 * PMIC IRQ
 */
pub const DA9055_IRQ_ALARM: c_uint = 0x01;
pub const DA9055_IRQ_TICK: c_uint = 0x02;
pub const DA9055_IRQ_NONKEY: c_uint = 0x00;
pub const DA9055_IRQ_REGULATOR: c_uint = 0x0B;
pub const DA9055_IRQ_HWMON: c_uint = 0x03;

#[repr(C)]
pub struct da9055 {
    pub regmap: *mut regmap,
    pub irq_data: *mut regmap_irq_chip_data,
    pub dev: *mut device,
    pub i2c_client: *mut i2c_client,

    pub irq_base: c_int,
    pub chip_irq: c_int,
}

extern "C" {
    pub fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    pub fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    pub fn regmap_bulk_read(
        map: *mut regmap,
        reg: c_uint,
        val: *mut c_uchar,
        val_count: c_uint,
    ) -> c_int;
    pub fn regmap_raw_write(
        map: *mut regmap,
        reg: c_uint,
        val: *mut c_uchar,
        val_len: c_uint,
    ) -> c_int;
    pub fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;

    pub fn da9055_device_init(da9055: *mut da9055) -> c_int;
    pub fn da9055_device_exit(da9055: *mut da9055);

    pub static da9055_regmap_config: regmap_config;
}

/* Device I/O */
#[inline]
pub unsafe fn da9055_reg_read(da9055: *mut da9055, reg: c_uchar) -> c_int {
    let mut val: c_int = 0;
    let ret = regmap_read((*da9055).regmap, reg as c_uint, &mut val);
    if ret < 0 {
        return ret;
    }
    val
}

#[inline]
pub unsafe fn da9055_reg_write(
    da9055: *mut da9055,
    reg: c_uchar,
    val: c_uchar,
) -> c_int {
    regmap_write((*da9055).regmap, reg as c_uint, val as c_uint)
}

#[inline]
pub unsafe fn da9055_group_read(
    da9055: *mut da9055,
    reg: c_uchar,
    reg_cnt: c_uint,
    val: *mut c_uchar,
) -> c_int {
    regmap_bulk_read((*da9055).regmap, reg as c_uint, val, reg_cnt)
}

#[inline]
pub unsafe fn da9055_group_write(
    da9055: *mut da9055,
    reg: c_uchar,
    reg_cnt: c_uint,
    val: *mut c_uchar,
) -> c_int {
    regmap_raw_write((*da9055).regmap, reg as c_uint, val, reg_cnt)
}

#[inline]
pub unsafe fn da9055_reg_update(
    da9055: *mut da9055,
    reg: c_uchar,
    bit_mask: c_uchar,
    reg_val: c_uchar,
) -> c_int {
    regmap_update_bits(
        (*da9055).regmap,
        reg as c_uint,
        bit_mask as c_uint,
        reg_val as c_uint,
    )
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
