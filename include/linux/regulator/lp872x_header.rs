/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2012 Texas Instruments
 *
 * Author: Milo(Woogyom) Kim <milo.kim@ti.com>
 */

// Dependencies supplied by the corresponding Linux kernel headers:
// linux/regulator/machine.h, linux/platform_device.h, linux/gpio/consumer.h

pub const LP872X_MAX_REGULATORS: usize = 9;

pub const LP8720_ENABLE_DELAY: u32 = 200;
pub const LP8725_ENABLE_DELAY: u32 = 30000;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum lp872x_regulator_id {
    LP8720_ID_BASE = 0,
    LP8720_ID_LDO1 = 0,
    LP8720_ID_LDO2 = 1,
    LP8720_ID_LDO3 = 2,
    LP8720_ID_LDO4 = 3,
    LP8720_ID_LDO5 = 4,
    LP8720_ID_BUCK = 5,

    LP8725_ID_BASE = 6,
    LP8725_ID_LDO1 = 6,
    LP8725_ID_LDO2 = 7,
    LP8725_ID_LDO3 = 8,
    LP8725_ID_LDO4 = 9,
    LP8725_ID_LDO5 = 10,
    LP8725_ID_LILO1 = 11,
    LP8725_ID_LILO2 = 12,
    LP8725_ID_BUCK1 = 13,
    LP8725_ID_BUCK2 = 14,

    LP872X_ID_MAX = 15,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum lp872x_dvs_sel {
    SEL_V1 = 0,
    SEL_V2 = 1,
}

/**
 * lp872x_dvs
 * @gpio       : gpio descriptor for dvs control
 * @vsel       : dvs selector for buck v1 or buck v2 register
 * @init_state : initial dvs pin state
 */
#[repr(C)]
pub struct lp872x_dvs {
    pub gpio: *mut gpio_desc,
    pub vsel: lp872x_dvs_sel,
    pub init_state: gpiod_flags,
}

/**
 * lp872x_regdata
 * @id        : regulator id
 * @init_data : init data for each regulator
 */
#[repr(C)]
pub struct lp872x_regulator_data {
    pub id: lp872x_regulator_id,
    pub init_data: *mut regulator_init_data,
}

/**
 * lp872x_platform_data
 * @general_config    : the value of LP872X_GENERAL_CFG register
 * @update_config     : if LP872X_GENERAL_CFG register is updated, set true
 * @regulator_data    : platform regulator id and init data
 * @dvs               : dvs data for buck voltage control
 * @enable_gpio       : gpio descriptor for enable control
 */
#[repr(C)]
pub struct lp872x_platform_data {
    pub general_config: u8,
    pub update_config: bool,
    pub regulator_data: [lp872x_regulator_data; LP872X_MAX_REGULATORS],
    pub dvs: *mut lp872x_dvs,
    pub enable_gpio: *mut gpio_desc,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
