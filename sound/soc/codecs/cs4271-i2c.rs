// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * CS4271 I2C audio driver
 *
 * Copyright (c) 2010 Alexander Sverdlin <subaparts@yandex.ru>
 */

// C includes translated as external dependencies:
// <linux/module.h>
// <linux/i2c.h>
// <linux/regmap.h>
// <sound/soc.h>
// "cs4271.h"

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
#[derive(Copy, Clone)]
pub struct regmap_config {
    pub reg_bits: c_int,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
    pub driver_data: usize,
}

#[repr(C)]
pub struct of_device_id {
    _private: [u8; 0],
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
    static cs4271_regmap_config: regmap_config;
    static cs4271_dt_ids: [of_device_id; 0];

    fn cs4271_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
}

#[inline]
unsafe fn of_match_ptr(ptr: *const of_device_id) -> *const of_device_id {
    ptr
}

unsafe extern "C" fn cs4271_i2c_probe(client: *mut i2c_client) -> c_int {
    let mut config: regmap_config;

    config = unsafe { cs4271_regmap_config };
    config.reg_bits = 8;

    unsafe {
        cs4271_probe(
            &mut (*client).dev,
            devm_regmap_init_i2c(client, &config),
        )
    }
}

static cs4271_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'4' as c_char,
            b'2' as c_char,
            b'7' as c_char,
            b'1' as c_char,
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
// MODULE_DEVICE_TABLE(i2c, cs4271_i2c_id);

static mut cs4271_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"cs4271\0".as_ptr() as *const c_char,
        of_match_table: unsafe { of_match_ptr(cs4271_dt_ids.as_ptr()) },
    },
    probe: Some(cs4271_i2c_probe),
    id_table: cs4271_i2c_id.as_ptr(),
};
// module_i2c_driver(cs4271_i2c_driver);

// MODULE_DESCRIPTION("ASoC CS4271 I2C Driver");
// MODULE_AUTHOR("Alexander Sverdlin <subaparts@yandex.ru>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
