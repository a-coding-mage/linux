// SPDX-License-Identifier: GPL-2.0-only
/*
 * ADAU1977/ADAU1978/ADAU1979 driver
 *
 * Copyright 2014 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

// C dependencies:
// #include <linux/i2c.h>
// #include <linux/module.h>
// #include <linux/regmap.h>
// #include <sound/soc.h>
// #include "adau1977.h"

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
#[derive(Copy, Clone)]
pub struct regmap_config {
    pub val_bits: u32,
    pub reg_bits: u32,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
    pub driver_data: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

extern "C" {
    static adau1977_regmap_config: regmap_config;
    static ADAU1977: usize;
    static ADAU1978: usize;

    fn adau1977_probe(
        dev: *mut device,
        regmap: *mut regmap,
        driver_data: usize,
        switch_mode: *mut c_void,
    ) -> c_int;

    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn i2c_get_match_data(client: *mut i2c_client) -> *const c_void;
}

unsafe extern "C" fn adau1977_i2c_probe(client: *mut i2c_client) -> c_int {
    let mut config: regmap_config;

    config = adau1977_regmap_config;
    config.val_bits = 8;
    config.reg_bits = 8;

    adau1977_probe(
        &mut (*client).dev,
        devm_regmap_init_i2c(client, &config),
        i2c_get_match_data(client) as usize,
        core::ptr::null_mut(),
    )
}

static adau1977_i2c_ids: [i2c_device_id; 4] = [
    i2c_device_id {
        name: [
            b'a' as c_char,
            b'd' as c_char,
            b'a' as c_char,
            b'u' as c_char,
            b'1' as c_char,
            b'9' as c_char,
            b'7' as c_char,
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
        ],
        driver_data: unsafe { ADAU1977 },
    },
    i2c_device_id {
        name: [
            b'a' as c_char,
            b'd' as c_char,
            b'a' as c_char,
            b'u' as c_char,
            b'1' as c_char,
            b'9' as c_char,
            b'7' as c_char,
            b'8' as c_char,
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
        driver_data: unsafe { ADAU1978 },
    },
    i2c_device_id {
        name: [
            b'a' as c_char,
            b'd' as c_char,
            b'a' as c_char,
            b'u' as c_char,
            b'1' as c_char,
            b'9' as c_char,
            b'7' as c_char,
            b'9' as c_char,
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
        driver_data: unsafe { ADAU1978 },
    },
    i2c_device_id {
        name: [0; 20],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(i2c, adau1977_i2c_ids);

static mut adau1977_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"adau1977\0".as_ptr() as *const c_char,
    },
    probe: Some(adau1977_i2c_probe),
    id_table: adau1977_i2c_ids.as_ptr(),
};
// module_i2c_driver(adau1977_i2c_driver);

// MODULE_DESCRIPTION("ASoC ADAU1977/ADAU1978/ADAU1979 driver");
// MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
