/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (C) 2012 Dialog Semiconductor Ltd.
 */

pub const DA9055_MAX_REGULATORS: usize = 8;

#[repr(C)]
pub struct da9055;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum gpio_select {
    NO_GPIO = 0,
    GPIO_1,
    GPIO_2,
}

#[repr(C)]
pub struct regulator_init_data;

#[repr(C)]
pub struct da9055_pdata {
    pub init: Option<unsafe extern "C" fn(da9055_: *mut da9055) -> ::core::ffi::c_int>,
    pub irq_base: ::core::ffi::c_int,
    pub gpio_base: ::core::ffi::c_int,

    pub regulators: [*mut regulator_init_data; DA9055_MAX_REGULATORS],
    /* Enable RTC in RESET Mode */
    pub reset_enable: bool,
    /*
     * Regulator mode control bits value (GPI offset) that
     * controls the regulator state, 0 if not available.
     */
    pub reg_ren: *mut gpio_select,
    /*
     * Regulator mode control bits value (GPI offset) that
     * controls the regulator set A/B, 0 if  not available.
     */
    pub reg_rsel: *mut gpio_select,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
