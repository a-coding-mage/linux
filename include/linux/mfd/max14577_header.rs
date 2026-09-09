/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * max14577.h - Driver for the Maxim 14577/77836
 *
 * Copyright (C) 2014 Samsung Electronics
 * Chanwoo Choi <cw00.choi@samsung.com>
 * Krzysztof Kozlowski <krzk@kernel.org>
 *
 * This driver is based on max8997.h
 *
 * MAX14577 has MUIC, Charger devices.
 * The devices share the same I2C bus and interrupt line
 * included in this mfd driver.
 *
 * MAX77836 has additional PMIC and Fuel-Gauge on different I2C slave
 * addresses.
 */

// Dependency supplied by the surrounding kernel translation.
#[repr(C)]
pub struct regulator_init_data {
    _private: [u8; 0],
}

// Dependency supplied by the surrounding kernel translation.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

/* MAX14577 regulator IDs */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum max14577_regulators {
    MAX14577_SAFEOUT = 0,
    MAX14577_CHARGER,
    MAX14577_REGULATOR_NUM,
}

/* MAX77836 regulator IDs */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum max77836_regulators {
    MAX77836_SAFEOUT = 0,
    MAX77836_CHARGER,
    MAX77836_LDO1,
    MAX77836_LDO2,
    MAX77836_REGULATOR_NUM,
}

#[repr(C)]
pub struct max14577_regulator_platform_data {
    pub id: i32,
    pub initdata: *mut regulator_init_data,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct max14577_charger_platform_data {
    pub constant_uvolt: u32,
    pub fast_charge_uamp: u32,
    pub eoc_uamp: u32,
    pub ovp_uvolt: u32,
}

/*
 * MAX14577 MFD platform data
 */
#[repr(C)]
pub struct max14577_platform_data {
    /* IRQ */
    pub irq_base: i32,

    /* current control GPIOs */
    pub gpio_pogo_vbatt_en: i32,
    pub gpio_pogo_vbus_en: i32,

    /* current control GPIO control function */
    pub set_gpio_pogo_vbatt_en: Option<unsafe extern "C" fn(gpio_val: i32) -> i32>,
    pub set_gpio_pogo_vbus_en: Option<unsafe extern "C" fn(gpio_val: i32) -> i32>,

    pub set_gpio_pogo_cb: Option<unsafe extern "C" fn(new_dev: i32) -> i32>,

    pub regulators: *mut max14577_regulator_platform_data,
}

/*
 * Valid limits of current for max14577 and max77836 chargers.
 * They must correspond to MBCICHWRCL and MBCICHWRCH fields in CHGCTRL4
 * register for given chipset.
 */
#[repr(C)]
pub struct maxim_charger_current {
    /* Minimal current, set in CHGCTRL4/MBCICHWRCL, uA */
    pub min: ::core::ffi::c_uint,
    /*
     * Minimal current when high setting is active,
     * set in CHGCTRL4/MBCICHWRCH, uA
     */
    pub high_start: ::core::ffi::c_uint,
    /* Value of one step in high setting, uA */
    pub high_step: ::core::ffi::c_uint,
    /* Maximum current of high setting, uA */
    pub max: ::core::ffi::c_uint,
}

unsafe extern "C" {
    pub static maxim_charger_currents: [maxim_charger_current; 0];
    pub fn maxim_charger_calc_reg_current(
        limits: *const maxim_charger_current,
        min_ua: ::core::ffi::c_uint,
        max_ua: ::core::ffi::c_uint,
        dst: *mut u8,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
