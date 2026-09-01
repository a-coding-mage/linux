// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for ADAU1372 codec
 *
 * Copyright 2016 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

// C dependencies:
// #include <linux/i2c.h>
// #include <linux/module.h>
// #include <linux/regmap.h>
// #include <sound/soc.h>
// #include "adau1372.h"

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
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
pub struct of_device_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
    pub driver_data: usize,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    static adau1372_regmap_config: regmap_config;
    static adau1372_of_match: *const of_device_id;

    fn adau1372_probe(dev: *mut device, regmap: *mut regmap, switch_mode: *mut c_void) -> c_int;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
}

unsafe extern "C" fn adau1372_i2c_probe(client: *mut i2c_client) -> c_int {
    unsafe {
        adau1372_probe(
            &mut (*client).dev,
            devm_regmap_init_i2c(client, &adau1372_regmap_config),
            ptr::null_mut(),
        )
    }
}

static adau1372_i2c_ids: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b'a' as c_char,
            b'd' as c_char,
            b'a' as c_char,
            b'u' as c_char,
            b'1' as c_char,
            b'3' as c_char,
            b'7' as c_char,
            b'2' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        driver_data: 0,
    },
    i2c_device_id {
        name: [0; 20],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(i2c, adau1372_i2c_ids);

static mut adau1372_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"adau1372\0".as_ptr() as *const c_char,
        of_match_table: unsafe { adau1372_of_match },
    },
    probe: Some(adau1372_i2c_probe),
    id_table: adau1372_i2c_ids.as_ptr(),
};
// module_i2c_driver(adau1372_i2c_driver);

// MODULE_DESCRIPTION("ASoC ADAU1372 CODEC I2C driver");
// MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
