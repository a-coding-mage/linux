// SPDX-License-Identifier: GPL-2.0-only
/*
 * ADAV803 audio driver
 *
 * Copyright 2014 Analog Devices Inc.
 */

// C dependencies:
// #include <linux/module.h>
// #include <linux/i2c.h>
// #include <linux/regmap.h>
// #include <sound/soc.h>
// #include "adav80x.h"

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(client: *mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    static adav80x_regmap_config: regmap_config;

    fn devm_regmap_init_i2c(
        client: *mut i2c_client,
        config: *const regmap_config,
    ) -> *mut regmap;

    fn adav80x_bus_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
}

static ADAV803_NAME: &[u8] = b"adav803\0";

static adav803_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: ADAV803_NAME.as_ptr() as *const c_char,
    },
    i2c_device_id { name: ptr::null() },
];
// MODULE_DEVICE_TABLE(i2c, adav803_id);

unsafe extern "C" fn adav803_probe(client: *mut i2c_client) -> c_int {
    unsafe {
        adav80x_bus_probe(
            &mut (*client).dev,
            devm_regmap_init_i2c(client, &adav80x_regmap_config),
        )
    }
}

static mut adav803_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: ADAV803_NAME.as_ptr() as *const c_char,
    },
    probe: Some(adav803_probe),
    id_table: adav803_id.as_ptr(),
};
// module_i2c_driver(adav803_driver);

// MODULE_DESCRIPTION("ASoC ADAV803 driver");
// MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
// MODULE_AUTHOR("Yi Li <yi.li@analog.com>>");
// MODULE_LICENSE("GPL");

#[allow(dead_code)]
unsafe extern "C" fn _adav803_driver_registration_marker() -> *mut c_void {
    unsafe { &raw mut adav803_driver as *mut c_void }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
