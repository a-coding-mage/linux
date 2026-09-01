// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs42l56.c -- CS42L51 ALSA SoC I2C audio driver
 *
 * Copyright 2014 CirrusLogic, Inc.
 *
 * Author: Brian Austin <brian.austin@cirrus.com>
 */

// C dependencies:
// #include <linux/i2c.h>
// #include <linux/module.h>
// #include <sound/soc.h>
// #include "cs42l51.h"

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct of_device_id {
    pub name: *const c_char,
    pub type_: *const c_char,
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub freeze: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub thaw: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub poweroff: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub restore: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    static cs42l51_regmap: regmap_config;

    fn cs42l51_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
    fn cs42l51_remove(dev: *mut device);
    fn cs42l51_suspend(dev: *mut device) -> c_int;
    fn cs42l51_resume(dev: *mut device) -> c_int;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
}

static cs42l51_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: b"cs42l51\0".as_ptr() as *const c_char,
        driver_data: 0,
    },
    i2c_device_id {
        name: core::ptr::null(),
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(i2c, cs42l51_i2c_id);

static cs42l51_of_match: [of_device_id; 2] = [
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: b"cirrus,cs42l51\0".as_ptr() as *const c_char,
        data: core::ptr::null(),
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, cs42l51_of_match);

unsafe extern "C" fn cs42l51_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut config: regmap_config;

    config = cs42l51_regmap;

    cs42l51_probe(
        &mut (*i2c).dev,
        devm_regmap_init_i2c(i2c, &config),
    )
}

unsafe extern "C" fn cs42l51_i2c_remove(i2c: *mut i2c_client) {
    cs42l51_remove(&mut (*i2c).dev);
}

static cs42l51_pm_ops: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(cs42l51_suspend, cs42l51_resume)
    suspend: Some(cs42l51_suspend),
    resume: Some(cs42l51_resume),
    freeze: Some(cs42l51_suspend),
    thaw: Some(cs42l51_resume),
    poweroff: Some(cs42l51_suspend),
    restore: Some(cs42l51_resume),
};

static mut cs42l51_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"cs42l51\0".as_ptr() as *const c_char,
        of_match_table: cs42l51_of_match.as_ptr(),
        pm: &cs42l51_pm_ops,
    },
    probe: Some(cs42l51_i2c_probe),
    remove: Some(cs42l51_i2c_remove),
    id_table: cs42l51_i2c_id.as_ptr(),
};

// module_i2c_driver(cs42l51_i2c_driver);

// MODULE_DESCRIPTION("ASoC CS42L51 I2C Driver");
// MODULE_AUTHOR("Brian Austin, Cirrus Logic Inc, <brian.austin@cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
