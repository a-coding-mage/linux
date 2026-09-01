// SPDX-License-Identifier: GPL-2.0
//
// Driver for SRC4XXX codecs
//
// Copyright 2021-2022 Deqx Pty Ltd
// Author: Matt Flax <flatmax@flatmax.com>

// C dependencies:
// #include <linux/i2c.h>
// #include <linux/module.h>
// #include <linux/regmap.h>
// #include "src4xxx.h"

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
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
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
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
    static src4xxx_regmap_config: regmap_config;

    fn src4xxx_probe(dev: *mut device, regmap: *mut regmap, data: *mut c_void) -> c_int;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
}

unsafe extern "C" fn src4xxx_i2c_probe(i2c: *mut i2c_client) -> c_int {
    unsafe {
        src4xxx_probe(
            &mut (*i2c).dev,
            devm_regmap_init_i2c(i2c, &src4xxx_regmap_config),
            ptr::null_mut(),
        )
    }
}

static SRC4XXX_I2C_IDS: [i2c_device_id; 2] = [
    i2c_device_id {
        name: c"src4392".as_ptr(),
    },
    i2c_device_id { name: ptr::null() },
];
// MODULE_DEVICE_TABLE(i2c, src4xxx_i2c_ids);

// Original declaration included __maybe_unused.
static SRC4XXX_OF_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: c"ti,src4392".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, src4xxx_of_match);

// of_match_ptr(src4xxx_of_match)
static mut SRC4XXX_I2C_DRIVER: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"src4xxx".as_ptr(),
        of_match_table: SRC4XXX_OF_MATCH.as_ptr(),
    },
    probe: Some(src4xxx_i2c_probe),
    id_table: SRC4XXX_I2C_IDS.as_ptr(),
};
// module_i2c_driver(src4xxx_i2c_driver);

// MODULE_DESCRIPTION("ASoC SRC4392 CODEC I2C driver");
// MODULE_AUTHOR("Matt Flax <flatmax@flatmax.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
