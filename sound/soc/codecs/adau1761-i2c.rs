// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for ADAU1361/ADAU1461/ADAU1761/ADAU1961 codec
 *
 * Copyright 2014 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

// C dependencies: <linux/i2c.h>, <linux/module.h>, <linux/regmap.h>,
// <sound/soc.h>, and "adau1761.h".

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
#[derive(Copy, Clone)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
    pub driver_data: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

extern "C" {
    static adau1761_regmap_config: regmap_config;

    fn adau1761_probe(
        dev: *mut device,
        regmap: *mut regmap,
        type_: usize,
        switch_mode: *mut c_void,
    ) -> c_int;
    fn adau17x1_remove(dev: *mut device);
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn i2c_get_match_data(client: *mut i2c_client) -> *const c_void;
}

extern "C" {
    static ADAU1361: usize;
    static ADAU1761: usize;
}

unsafe extern "C" fn adau1761_i2c_probe(client: *mut i2c_client) -> c_int {
    let mut config: regmap_config;

    config = adau1761_regmap_config;
    config.val_bits = 8;
    config.reg_bits = 16;

    adau1761_probe(
        unsafe { &mut (*client).dev },
        devm_regmap_init_i2c(client, &config),
        i2c_get_match_data(client) as usize,
        core::ptr::null_mut(),
    )
}

unsafe extern "C" fn adau1761_i2c_remove(client: *mut i2c_client) {
    adau17x1_remove(unsafe { &mut (*client).dev });
}

const fn i2c_name(name: &[u8]) -> [c_char; 20] {
    let mut out = [0 as c_char; 20];
    let mut i = 0;

    while i < name.len() && i < 19 {
        out[i] = name[i] as c_char;
        i += 1;
    }

    out
}

static adau1761_i2c_ids: [i2c_device_id; 5] = [
    i2c_device_id {
        name: i2c_name(b"adau1361"),
        driver_data: unsafe { ADAU1361 },
    },
    i2c_device_id {
        name: i2c_name(b"adau1461"),
        driver_data: unsafe { ADAU1761 },
    },
    i2c_device_id {
        name: i2c_name(b"adau1761"),
        driver_data: unsafe { ADAU1761 },
    },
    i2c_device_id {
        name: i2c_name(b"adau1961"),
        driver_data: unsafe { ADAU1361 },
    },
    i2c_device_id {
        name: [0; 20],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(i2c, adau1761_i2c_ids);

// Original C condition: #if defined(CONFIG_OF)
static adau1761_i2c_dt_ids: [of_device_id; 5] = [
    of_device_id {
        compatible: b"adi,adau1361\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"adi,adau1461\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"adi,adau1761\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"adi,adau1961\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, adau1761_i2c_dt_ids);

static mut adau1761_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"adau1761\0".as_ptr() as *const c_char,
        of_match_table: adau1761_i2c_dt_ids.as_ptr(),
    },
    probe: Some(adau1761_i2c_probe),
    remove: Some(adau1761_i2c_remove),
    id_table: adau1761_i2c_ids.as_ptr(),
};
// module_i2c_driver(adau1761_i2c_driver);

// MODULE_DESCRIPTION("ASoC ADAU1361/ADAU1461/ADAU1761/ADAU1961 CODEC I2C driver");
// MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
