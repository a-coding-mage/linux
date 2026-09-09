/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * fixed.h
 *
 * Copyright 2008 Wolfson Microelectronics PLC.
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 *
 * Copyright (c) 2009 Nokia Corporation
 * Roger Quadros <ext-roger.quadros@nokia.com>
 */

// C dependency: struct regulator_init_data.
pub struct regulator_init_data;

/**
 * struct fixed_voltage_config - fixed_voltage_config structure
 * @supply_name: Name of the regulator supply
 * @input_supply: Name of the input regulator supply
 * @microvolts: Output voltage of regulator
 * @startup_delay: Start-up time in microseconds
 * @enabled_at_boot: Whether regulator has been enabled at boot or not.
 *                   1 = Yes, 0 = No. This is used to keep the regulator
 *                   at the default state
 * @init_data: regulator_init_data
 *
 * This structure contains fixed voltage regulator configuration
 * information that must be passed by platform code to the fixed
 * voltage regulator driver.
 */
#[repr(C)]
pub struct fixed_voltage_config {
    pub supply_name: *const core::ffi::c_char,
    pub input_supply: *const core::ffi::c_char,
    pub microvolts: core::ffi::c_int,
    pub startup_delay: core::ffi::c_uint,
    pub off_on_delay: core::ffi::c_uint,
    // C declaration: unsigned enabled_at_boot:1;
    pub enabled_at_boot: core::ffi::c_uchar,
    pub init_data: *mut regulator_init_data,
}

// C dependency: struct regulator_consumer_supply.
pub struct regulator_consumer_supply;

// C dependency: struct platform_device.
pub struct platform_device;

// The C declaration is selected by IS_ENABLED(CONFIG_REGULATOR). When the
// configuration is disabled, the inline implementation returns NULL.
#[cfg(feature = "CONFIG_REGULATOR")]
unsafe extern "C" {
    pub fn regulator_register_always_on(
        id: core::ffi::c_int,
        name: *const core::ffi::c_char,
        supplies: *mut regulator_consumer_supply,
        num_supplies: core::ffi::c_int,
        uv: core::ffi::c_int,
    ) -> *mut platform_device;
}

#[cfg(not(feature = "CONFIG_REGULATOR"))]
#[inline]
pub unsafe fn regulator_register_always_on(
    _id: core::ffi::c_int,
    _name: *const core::ffi::c_char,
    _supplies: *mut regulator_consumer_supply,
    _num_supplies: core::ffi::c_int,
    _uv: core::ffi::c_int,
) -> *mut platform_device {
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn regulator_register_fixed(
    id: core::ffi::c_int,
    s: *mut regulator_consumer_supply,
    ns: core::ffi::c_int,
) -> *mut platform_device {
    regulator_register_always_on(
        id,
        c"fixed-dummy".as_ptr(),
        s,
        ns,
        0,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
