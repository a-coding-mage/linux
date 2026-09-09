/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Broadcom BCM590xx PMU
 *
 * Copyright 2014 Linaro Limited
 * Author: Matt Porter <mporter@linaro.org>
 */

/* Dependencies supplied by other translation units. */
pub enum device {}
pub enum i2c_client {}
pub enum regmap {}

/* PMU ID register values; also used as device type */
pub const BCM590XX_PMUID_BCM59054: u8 = 0x54;
pub const BCM590XX_PMUID_BCM59056: u8 = 0x56;

/* Known chip revision IDs */
pub const BCM59054_REV_DIGITAL_A1: u8 = 1;
pub const BCM59054_REV_ANALOG_A1: u8 = 2;

pub const BCM59056_REV_DIGITAL_A0: u8 = 1;
pub const BCM59056_REV_ANALOG_A0: u8 = 1;

/* Known chip revision IDs */
pub const BCM59056_REV_DIGITAL_B0: u8 = 2;
pub const BCM59056_REV_ANALOG_B0: u8 = 2;

/* regmap types */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum bcm590xx_regmap_type {
    BCM590XX_REGMAP_PRI = 0,
    BCM590XX_REGMAP_SEC = 1,
}

/* max register address */
pub const BCM590XX_MAX_REGISTER_PRI: u8 = 0xe7;
pub const BCM590XX_MAX_REGISTER_SEC: u8 = 0xf0;

#[repr(C)]
pub struct bcm590xx {
    pub dev: *mut device,
    pub i2c_pri: *mut i2c_client,
    pub i2c_sec: *mut i2c_client,
    pub regmap_pri: *mut regmap,
    pub regmap_sec: *mut regmap,

    /* PMU ID value; also used as device type */
    pub pmu_id: u8,

    /* Chip revision, read from PMUREV reg */
    pub rev_digital: u8,
    pub rev_analog: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
