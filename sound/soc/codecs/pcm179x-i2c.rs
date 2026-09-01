// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PCM179X ASoC I2C driver
 *
 * Copyright (c) Teenage Engineering AB 2016
 *
 *     Jacob Siverskog <jacob@teenage.engineering>
 */

// C dependencies:
// #include <linux/module.h>
// #include <linux/of.h>
// #include <linux/i2c.h>
// #include <linux/regmap.h>
// #include "pcm179x.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};

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
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub id_table: *const i2c_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
}

unsafe extern "C" {
    static pcm179x_regmap_config: regmap_config;

    fn devm_regmap_init_i2c(
        client: *mut i2c_client,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn pcm179x_common_init(dev: *mut device, regmap: *mut regmap) -> c_int;
}

unsafe extern "C" fn pcm179x_i2c_probe(client: *mut i2c_client) -> c_int {
    let regmap: *mut regmap;
    let ret: c_int;

    regmap = devm_regmap_init_i2c(client, &pcm179x_regmap_config);
    if IS_ERR(regmap as *const c_void) {
        ret = PTR_ERR(regmap as *const c_void);
        dev_err(
            &mut (*client).dev,
            c"Failed to allocate regmap: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    pcm179x_common_init(&mut (*client).dev, regmap)
}

// Original C condition: #ifdef CONFIG_OF
#[cfg(CONFIG_OF)]
static pcm179x_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"ti,pcm1792a".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, pcm179x_of_match);

#[cfg(CONFIG_OF)]
const PCM179X_OF_MATCH_TABLE: *const of_device_id = pcm179x_of_match.as_ptr();

#[cfg(not(CONFIG_OF))]
const PCM179X_OF_MATCH_TABLE: *const of_device_id = core::ptr::null();

static pcm179x_i2c_ids: [i2c_device_id; 2] = [
    i2c_device_id {
        name: c"pcm179x".as_ptr(),
    },
    i2c_device_id {
        name: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(i2c, pcm179x_i2c_ids);

static mut pcm179x_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"pcm179x".as_ptr(),
        of_match_table: PCM179X_OF_MATCH_TABLE,
    },
    id_table: pcm179x_i2c_ids.as_ptr(),
    probe: Some(pcm179x_i2c_probe),
};

// module_i2c_driver(pcm179x_i2c_driver);

// MODULE_DESCRIPTION("ASoC PCM179X I2C driver");
// MODULE_AUTHOR("Jacob Siverskog <jacob@teenage.engineering>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
