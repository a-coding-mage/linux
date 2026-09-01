// SPDX-License-Identifier: GPL-2.0
//
// PCM3060 I2C driver
//
// Copyright (C) 2018 Kirill Marinushkin <k.marinushkin@gmail.com>

// C includes translated as external dependencies:
// <linux/i2c.h>
// <linux/module.h>
// <sound/soc.h>
// "pcm3060.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type gfp_t = c_uint;
type c_uint = u32;

const GFP_KERNEL: gfp_t = 0;
const ENOMEM: c_int = 12;

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
pub struct pcm3060_priv {
    pub regmap: *mut regmap,
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
    // Present in C only under CONFIG_OF:
    // pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub id_table: *const i2c_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
}

unsafe extern "C" {
    static pcm3060_regmap: regmap_config;

    fn devm_kzalloc(dev: *mut device, size: usize, gfp: gfp_t) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pcm3060_probe(dev: *mut device) -> c_int;
}

unsafe extern "C" fn pcm3060_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let priv_: *mut pcm3060_priv;

    priv_ = devm_kzalloc(
        unsafe { &mut (*i2c).dev },
        size_of::<pcm3060_priv>(),
        GFP_KERNEL,
    ) as *mut pcm3060_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    unsafe {
        i2c_set_clientdata(i2c, priv_ as *mut c_void);
    }

    unsafe {
        (*priv_).regmap = devm_regmap_init_i2c(i2c, &pcm3060_regmap);
    }
    if unsafe { IS_ERR((*priv_).regmap as *const c_void) } {
        return unsafe { PTR_ERR((*priv_).regmap as *const c_void) };
    }

    unsafe { pcm3060_probe(&mut (*i2c).dev) }
}

static pcm3060_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: b"pcm3060\0".as_ptr() as *const c_char,
    },
    i2c_device_id { name: ptr::null() },
];
// MODULE_DEVICE_TABLE(i2c, pcm3060_i2c_id);

// CONFIG_OF:
static pcm3060_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"ti,pcm3060\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, pcm3060_of_match);

static mut pcm3060_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"pcm3060\0".as_ptr() as *const c_char,
        // CONFIG_OF:
        // of_match_table: pcm3060_of_match.as_ptr(),
    },
    id_table: pcm3060_i2c_id.as_ptr(),
    probe: Some(pcm3060_i2c_probe),
};

// module_i2c_driver(pcm3060_i2c_driver);

// MODULE_DESCRIPTION("PCM3060 I2C driver");
// MODULE_AUTHOR("Kirill Marinushkin <k.marinushkin@gmail.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
