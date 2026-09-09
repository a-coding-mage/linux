/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2011 Kionix, Inc.
 * Written by Chris Hudson <chudson@kionix.com>
 */

// C header dependency: `u8` is supplied by the surrounding kernel bindings.

pub const KXTJ9_I2C_ADDR: u8 = 0x0F;

#[repr(C)]
pub struct kxtj9_platform_data {
    pub min_interval: ::core::ffi::c_uint, /* minimum poll interval (in milli-seconds) */
    pub init_interval: ::core::ffi::c_uint, /* initial poll interval (in milli-seconds) */

    /*
     * By default, x is axis 0, y is axis 1, z is axis 2; these can be
     * changed to account for sensor orientation within the host device.
     */
    pub axis_map_x: u8,
    pub axis_map_y: u8,
    pub axis_map_z: u8,

    /*
     * Each axis can be negated to account for sensor orientation within
     * the host device.
     */
    pub negate_x: bool,
    pub negate_y: bool,
    pub negate_z: bool,

    /* CTRL_REG1: set resolution, g-range, data ready enable */
    /* Output resolution: 8-bit valid or 12-bit valid */
    pub res_12bit: u8,
    /* Output g-range: +/-2g, 4g, or 8g */
    pub g_range: ::core::ffi::c_int,

    pub init: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub exit: Option<unsafe extern "C" fn()>,
    pub power_on: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub power_off: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
}

pub const RES_8BIT: ::core::ffi::c_int = 0;
pub const RES_12BIT: ::core::ffi::c_int = 1 << 6;
pub const KXTJ9_G_2G: ::core::ffi::c_int = 0;
pub const KXTJ9_G_4G: ::core::ffi::c_int = 1 << 3;
pub const KXTJ9_G_8G: ::core::ffi::c_int = 1 << 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
