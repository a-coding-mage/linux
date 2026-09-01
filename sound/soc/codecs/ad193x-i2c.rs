// SPDX-License-Identifier: GPL-2.0-only
/*
 * AD1936/AD1937 audio driver
 *
 * Copyright 2014 Analog Devices Inc.
 */

// C includes translated as external dependencies:
// linux/module.h, linux/i2c.h, linux/regmap.h, sound/soc.h, "ad193x.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type c_char = i8;
type c_int = i32;
type uintptr_t = usize;

const AD193X: kernel_ulong_t = 0;

type kernel_ulong_t = usize;

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
#[derive(Copy, Clone)]
pub struct regmap_config {
    pub val_bits: c_int,
    pub reg_bits: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
    pub driver_data: kernel_ulong_t,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

extern "C" {
    static ad193x_regmap_config: regmap_config;

    fn ad193x_probe(dev: *mut device, regmap: *mut regmap, driver_data: uintptr_t) -> c_int;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn i2c_get_match_data(client: *mut i2c_client) -> *const core::ffi::c_void;
}

static ad193x_id: [i2c_device_id; 3] = [
    i2c_device_id {
        name: [
            b'a' as c_char,
            b'd' as c_char,
            b'1' as c_char,
            b'9' as c_char,
            b'3' as c_char,
            b'6' as c_char,
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
        driver_data: AD193X,
    },
    i2c_device_id {
        name: [
            b'a' as c_char,
            b'd' as c_char,
            b'1' as c_char,
            b'9' as c_char,
            b'3' as c_char,
            b'7' as c_char,
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
        driver_data: AD193X,
    },
    i2c_device_id {
        name: [0; 20],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(i2c, ad193x_id);

unsafe extern "C" fn ad193x_i2c_probe(client: *mut i2c_client) -> c_int {
    let mut config: regmap_config;

    config = ad193x_regmap_config;
    config.val_bits = 8;
    config.reg_bits = 8;

    ad193x_probe(
        &mut (*client).dev,
        devm_regmap_init_i2c(client, &config),
        i2c_get_match_data(client) as uintptr_t,
    )
}

static mut ad193x_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"ad193x\0".as_ptr() as *const c_char,
    },
    probe: Some(ad193x_i2c_probe),
    id_table: ad193x_id.as_ptr(),
};
// module_i2c_driver(ad193x_i2c_driver);

// MODULE_DESCRIPTION("ASoC AD1936/AD1937 audio CODEC driver");
// MODULE_AUTHOR("Barry Song <21cnbao@gmail.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
