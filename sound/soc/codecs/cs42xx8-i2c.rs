// SPDX-License-Identifier: GPL-2.0
/*
 * Cirrus Logic CS42448/CS42888 Audio CODEC DAI I2C driver
 *
 * Copyright (C) 2014 Freescale Semiconductor, Inc.
 *
 * Author: Nicolin Chen <Guangyu.Chen@freescale.com>
 */

// C includes translated as external dependency intent:
// <linux/i2c.h>, <linux/module.h>, <linux/pm_runtime.h>, <sound/soc.h>
// "cs42xx8.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

type kernel_ulong_t = c_ulong;
type c_ulong = u64;

const EINVAL: c_int = 22;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct cs42xx8_driver_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub name: *const c_char,
    pub type_: *const c_char,
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
    pub driver_data: kernel_ulong_t,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
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
    static cs42xx8_regmap_config: regmap_config;
    static cs42448_data: cs42xx8_driver_data;
    static cs42888_data: cs42xx8_driver_data;
    static cs42xx8_pm: dev_pm_ops;

    fn i2c_get_match_data(i2c: *mut i2c_client) -> *const c_void;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn cs42xx8_probe(
        dev: *mut device,
        regmap: *mut c_void,
        drvdata: *const cs42xx8_driver_data,
    ) -> c_int;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut c_void;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_request_idle(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
}

unsafe extern "C" fn cs42xx8_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let ret: c_int;
    let drvdata: *const cs42xx8_driver_data;

    drvdata = i2c_get_match_data(i2c) as *const cs42xx8_driver_data;
    if drvdata.is_null() {
        return dev_err_probe(
            core::ptr::addr_of_mut!((*i2c).dev),
            -EINVAL,
            c"failed to find driver data\n".as_ptr(),
        );
    }

    ret = cs42xx8_probe(
        core::ptr::addr_of_mut!((*i2c).dev),
        devm_regmap_init_i2c(i2c, core::ptr::addr_of!(cs42xx8_regmap_config)),
        drvdata,
    );
    if ret != 0 {
        return ret;
    }

    pm_runtime_enable(core::ptr::addr_of_mut!((*i2c).dev));
    pm_request_idle(core::ptr::addr_of_mut!((*i2c).dev));

    0
}

unsafe extern "C" fn cs42xx8_i2c_remove(i2c: *mut i2c_client) {
    pm_runtime_disable(core::ptr::addr_of_mut!((*i2c).dev));
}

static cs42xx8_of_match: [of_device_id; 3] = [
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"cirrus,cs42448".as_ptr(),
        data: core::ptr::addr_of!(cs42448_data) as *const c_void,
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"cirrus,cs42888".as_ptr(),
        data: core::ptr::addr_of!(cs42888_data) as *const c_void,
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, cs42xx8_of_match);

static cs42xx8_i2c_id: [i2c_device_id; 3] = [
    i2c_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'4' as c_char,
            b'2' as c_char,
            b'4' as c_char,
            b'4' as c_char,
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
            0,
        ],
        driver_data: core::ptr::addr_of!(cs42448_data) as kernel_ulong_t,
    },
    i2c_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'4' as c_char,
            b'2' as c_char,
            b'8' as c_char,
            b'8' as c_char,
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
            0,
        ],
        driver_data: core::ptr::addr_of!(cs42888_data) as kernel_ulong_t,
    },
    i2c_device_id {
        name: [0; 20],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(i2c, cs42xx8_i2c_id);

static mut cs42xx8_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"cs42xx8".as_ptr(),
        pm: core::ptr::addr_of!(cs42xx8_pm),
        of_match_table: cs42xx8_of_match.as_ptr(),
    },
    probe: Some(cs42xx8_i2c_probe),
    remove: Some(cs42xx8_i2c_remove),
    id_table: cs42xx8_i2c_id.as_ptr(),
};

// module_i2c_driver(cs42xx8_i2c_driver);

// MODULE_DESCRIPTION("Cirrus Logic CS42448/CS42888 ALSA SoC Codec I2C Driver");
// MODULE_AUTHOR("Freescale Semiconductor, Inc.");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
