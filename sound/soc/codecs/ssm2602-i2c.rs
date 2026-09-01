// SPDX-License-Identifier: GPL-2.0-only
/*
 * SSM2602/SSM2603/SSM2604 I2C audio driver
 *
 * Copyright 2014 Analog Devices Inc.
 */

// C dependencies:
// #include <linux/module.h>
// #include <linux/i2c.h>
// #include <linux/regmap.h>
// #include <sound/soc.h>
// #include "ssm2602.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

type uintptr_t = usize;
type kernel_ulong_t = c_ulong;

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct regmap_config {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
    pub driver_data: kernel_ulong_t,
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

extern "C" {
    static ssm2602_regmap_config: regmap_config;
    static SSM2602: kernel_ulong_t;
    static SSM2604: kernel_ulong_t;

    fn ssm2602_probe(dev: *mut device, kind: uintptr_t, regmap: *mut regmap) -> c_int;
    fn i2c_get_match_data(client: *mut i2c_client) -> *const c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
}

/*
 * ssm2602 2 wire address is determined by GPIO5
 * state during powerup.
 *    low  = 0x1a
 *    high = 0x1b
 */
unsafe extern "C" fn ssm2602_i2c_probe(client: *mut i2c_client) -> c_int {
    return ssm2602_probe(
        &mut (*client).dev,
        i2c_get_match_data(client) as uintptr_t,
        devm_regmap_init_i2c(client, &ssm2602_regmap_config),
    );
}

static ssm2602_i2c_id: [i2c_device_id; 4] = unsafe {
    [
        i2c_device_id {
            name: b"ssm2602\0".as_ptr() as *const c_char,
            driver_data: SSM2602,
        },
        i2c_device_id {
            name: b"ssm2603\0".as_ptr() as *const c_char,
            driver_data: SSM2602,
        },
        i2c_device_id {
            name: b"ssm2604\0".as_ptr() as *const c_char,
            driver_data: SSM2604,
        },
        i2c_device_id {
            name: core::ptr::null(),
            driver_data: 0,
        },
    ]
};
// MODULE_DEVICE_TABLE(i2c, ssm2602_i2c_id);

static ssm2602_of_match: [of_device_id; 4] = [
    of_device_id {
        compatible: b"adi,ssm2602\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"adi,ssm2603\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"adi,ssm2604\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, ssm2602_of_match);

static mut ssm2602_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"ssm2602\0".as_ptr() as *const c_char,
        of_match_table: ssm2602_of_match.as_ptr(),
    },
    probe: Some(ssm2602_i2c_probe),
    id_table: ssm2602_i2c_id.as_ptr(),
};
// module_i2c_driver(ssm2602_i2c_driver);

// MODULE_DESCRIPTION("ASoC SSM2602/SSM2603/SSM2604 I2C driver");
// MODULE_AUTHOR("Cliff Cai");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
