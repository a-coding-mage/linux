// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for ADAU1381/ADAU1781 CODEC
 *
 * Copyright 2014 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

/* Dependencies from the original C source:
 * linux/i2c.h, linux/module.h, linux/regmap.h, sound/soc.h, and "adau1781.h".
 */

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
#[derive(Clone, Copy)]
pub struct regmap_config {
    pub val_bits: c_int,
    pub reg_bits: c_int,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
    pub driver_data: usize,
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
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    static adau1781_regmap_config: regmap_config;
    static ADAU1381: usize;
    static ADAU1781: usize;

    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn i2c_get_match_data(client: *mut i2c_client) -> *const c_void;
    fn adau1781_probe(
        dev: *mut device,
        regmap: *mut regmap,
        driver_data: usize,
        arg: *mut c_void,
    ) -> c_int;
    fn adau17x1_remove(dev: *mut device);
}

unsafe extern "C" fn adau1781_i2c_probe(client: *mut i2c_client) -> c_int {
    let mut config: regmap_config;

    config = adau1781_regmap_config;
    config.val_bits = 8;
    config.reg_bits = 16;

    adau1781_probe(
        &mut (*client).dev,
        devm_regmap_init_i2c(client, &config),
        i2c_get_match_data(client) as usize,
        core::ptr::null_mut(),
    )
}

unsafe extern "C" fn adau1781_i2c_remove(client: *mut i2c_client) {
    adau17x1_remove(&mut (*client).dev);
}

static ADAU1381_NAME: &[u8] = b"adau1381\0";
static ADAU1781_NAME: &[u8] = b"adau1781\0";

static adau1781_i2c_ids: [i2c_device_id; 3] = [
    i2c_device_id {
        name: ADAU1381_NAME.as_ptr() as *const c_char,
        driver_data: unsafe { ADAU1381 },
    },
    i2c_device_id {
        name: ADAU1781_NAME.as_ptr() as *const c_char,
        driver_data: unsafe { ADAU1781 },
    },
    i2c_device_id {
        name: core::ptr::null(),
        driver_data: 0,
    },
];
/* MODULE_DEVICE_TABLE(i2c, adau1781_i2c_ids); */

/* Original C condition: #if defined(CONFIG_OF) */
static ADAU1381_COMPATIBLE: &[u8] = b"adi,adau1381\0";
static ADAU1781_COMPATIBLE: &[u8] = b"adi,adau1781\0";

static adau1781_i2c_dt_ids: [of_device_id; 3] = [
    of_device_id {
        compatible: ADAU1381_COMPATIBLE.as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ADAU1781_COMPATIBLE.as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, adau1781_i2c_dt_ids); */

static adau1781_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: ADAU1781_NAME.as_ptr() as *const c_char,
        /* of_match_ptr(adau1781_i2c_dt_ids) */
        of_match_table: adau1781_i2c_dt_ids.as_ptr(),
    },
    probe: Some(adau1781_i2c_probe),
    remove: Some(adau1781_i2c_remove),
    id_table: adau1781_i2c_ids.as_ptr(),
};
/* module_i2c_driver(adau1781_i2c_driver); */

/* MODULE_DESCRIPTION("ASoC ADAU1381/ADAU1781 CODEC I2C driver"); */
/* MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
