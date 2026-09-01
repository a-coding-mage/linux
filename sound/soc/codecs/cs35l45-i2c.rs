// SPDX-License-Identifier: GPL-2.0
//
// cs35l45-i2c.c -- CS35L45 I2C driver
//
// Copyright 2019-2022 Cirrus Logic, Inc.
//
// Author: James Schulman <james.schulman@cirrus.com>

// C includes translated as external dependencies:
// <linux/device.h>
// <linux/module.h>
// <linux/i2c.h>
// <linux/regmap.h>
// "cs35l45.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

pub const GFP_KERNEL: c_uint = 0;
pub const ENOMEM: c_int = 12;
pub const CONTROL_BUS_I2C: c_uint = 0;

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
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub irq: c_int,
    pub addr: c_uint,
}

#[repr(C)]
pub struct cs35l45_private {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub irq: c_int,
    pub bus_type: c_uint,
    pub i2c_addr: c_uint,
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
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub id_table: *const i2c_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
}

unsafe extern "C" {
    static cs35l45_i2c_regmap: regmap_config;
    static cs35l45_pm_ops: dev_pm_ops;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn cs35l45_probe(cs35l45: *mut cs35l45_private) -> c_int;
    fn cs35l45_remove(cs35l45: *mut cs35l45_private);
}

#[inline]
unsafe fn pm_ptr(ptr: *const dev_pm_ops) -> *const dev_pm_ops {
    ptr
}

unsafe extern "C" fn cs35l45_i2c_probe(client: *mut i2c_client) -> c_int {
    let cs35l45: *mut cs35l45_private;
    let dev: *mut device = unsafe { &mut (*client).dev };
    let ret: c_int;

    cs35l45 = unsafe {
        devm_kzalloc(
            dev,
            size_of::<cs35l45_private>(),
            GFP_KERNEL,
        ) as *mut cs35l45_private
    };
    if cs35l45.is_null() {
        return -ENOMEM;
    }

    unsafe {
        i2c_set_clientdata(client, cs35l45 as *mut c_void);
        (*cs35l45).regmap = devm_regmap_init_i2c(client, &cs35l45_i2c_regmap);
    }
    if unsafe { IS_ERR((*cs35l45).regmap as *const c_void) } {
        ret = unsafe { PTR_ERR((*cs35l45).regmap as *const c_void) };
        unsafe {
            dev_err(
                dev,
                b"Failed to allocate register map: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
        }
        return ret;
    }

    unsafe {
        (*cs35l45).dev = dev;
        (*cs35l45).irq = (*client).irq;
        (*cs35l45).bus_type = CONTROL_BUS_I2C;
        (*cs35l45).i2c_addr = (*client).addr;

        cs35l45_probe(cs35l45)
    }
}

unsafe extern "C" fn cs35l45_i2c_remove(client: *mut i2c_client) {
    let cs35l45: *mut cs35l45_private =
        unsafe { i2c_get_clientdata(client) as *mut cs35l45_private };

    unsafe {
        cs35l45_remove(cs35l45);
    }
}

static cs35l45_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"cirrus,cs35l45\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, cs35l45_of_match);

static cs35l45_id_i2c: [i2c_device_id; 2] = [
    i2c_device_id {
        name: b"cs35l45\0".as_ptr() as *const c_char,
    },
    i2c_device_id {
        name: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(i2c, cs35l45_id_i2c);

static mut cs35l45_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"cs35l45\0".as_ptr() as *const c_char,
        of_match_table: cs35l45_of_match.as_ptr(),
        pm: unsafe { pm_ptr(&cs35l45_pm_ops) },
    },
    id_table: cs35l45_id_i2c.as_ptr(),
    probe: Some(cs35l45_i2c_probe),
    remove: Some(cs35l45_i2c_remove),
};
// module_i2c_driver(cs35l45_i2c_driver);

// MODULE_DESCRIPTION("I2C CS35L45 driver");
// MODULE_AUTHOR("James Schulman, Cirrus Logic Inc, <james.schulman@cirrus.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("SND_SOC_CS35L45");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
