// SPDX-License-Identifier: GPL-2.0
// Audio driver for PCM1789 I2C
// Copyright (C) 2018 Bootlin
// Mylène Josserand <mylene.josserand@bootlin.com>

// C dependencies:
// #include <linux/clk.h>
// #include <linux/delay.h>
// #include <linux/i2c.h>
// #include <linux/module.h>
// #include <linux/of.h>
// #include <linux/regmap.h>
// #include "pcm1789.h"

use core::ffi::{c_char, c_int, c_void};

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
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
}

unsafe extern "C" {
    static pcm1789_regmap_config: regmap_config;

    fn devm_regmap_init_i2c(
        client: *mut i2c_client,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn pcm1789_common_init(dev: *mut device, regmap: *mut regmap) -> c_int;
    fn pcm1789_common_exit(dev: *mut device);
}

unsafe extern "C" fn pcm1789_i2c_probe(client: *mut i2c_client) -> c_int {
    let mut regmap: *mut regmap;
    let ret: c_int;

    regmap = unsafe { devm_regmap_init_i2c(client, &raw const pcm1789_regmap_config) };
    if unsafe { IS_ERR(regmap as *const c_void) } {
        ret = unsafe { PTR_ERR(regmap as *const c_void) };
        unsafe {
            dev_err(
                &raw mut (*client).dev,
                c"Failed to allocate regmap: %d\n".as_ptr(),
                ret,
            );
        }
        return ret;
    }

    unsafe { pcm1789_common_init(&raw mut (*client).dev, regmap) }
}

unsafe extern "C" fn pcm1789_i2c_remove(client: *mut i2c_client) {
    unsafe {
        pcm1789_common_exit(&raw mut (*client).dev);
    }
}

// C condition preserved: #ifdef CONFIG_OF
static pcm1789_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"ti,pcm1789".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, pcm1789_of_match);

static pcm1789_i2c_ids: [i2c_device_id; 2] = [
    i2c_device_id {
        name: c"pcm1789".as_ptr(),
    },
    i2c_device_id {
        name: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(i2c, pcm1789_i2c_ids);

static mut pcm1789_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"pcm1789".as_ptr(),
        of_match_table: pcm1789_of_match.as_ptr(),
    },
    id_table: pcm1789_i2c_ids.as_ptr(),
    probe: Some(pcm1789_i2c_probe),
    remove: Some(pcm1789_i2c_remove),
};

// module_i2c_driver(pcm1789_i2c_driver);

// MODULE_DESCRIPTION("ASoC PCM1789 I2C driver");
// MODULE_AUTHOR("Mylène Josserand <mylene.josserand@bootlin.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
