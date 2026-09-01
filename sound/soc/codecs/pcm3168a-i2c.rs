// SPDX-License-Identifier: GPL-2.0-only
/*
 * PCM3168A codec i2c driver
 *
 * Copyright (C) 2015 Imagination Technologies Ltd.
 *
 * Author: Damien Horsley <Damien.Horsley@imgtec.com>
 */

// C dependencies translated as external items supplied by the surrounding kernel
// crate/bindings: <linux/i2c.h>, <linux/init.h>, <linux/module.h>,
// <sound/soc.h>, and "pcm3168a.h".

use core::ffi::{c_char, c_int};
use core::ptr;

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
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20usize],
    pub driver_data: usize,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 16usize],
    pub driver_data: usize,
}

#[repr(C)]
pub struct of_device_id {
    pub name: *const c_char,
    pub type_: *const c_char,
    pub compatible: *const c_char,
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub acpi_match_table: *const acpi_device_id,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct i2c_driver {
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
    pub driver: device_driver,
}

unsafe extern "C" {
    static pcm3168a_regmap: regmap_config;
    static pcm3168a_pm_ops: dev_pm_ops;

    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> c_int;
    fn pcm3168a_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
    fn pcm3168a_remove(dev: *mut device);
    fn pm_ptr(ptr: *const dev_pm_ops) -> *const dev_pm_ops;
}

unsafe extern "C" fn pcm3168a_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let regmap: *mut regmap;

    regmap = devm_regmap_init_i2c(i2c, &pcm3168a_regmap);
    if IS_ERR(regmap as *const core::ffi::c_void) {
        return PTR_ERR(regmap as *const core::ffi::c_void);
    }

    pcm3168a_probe(&mut (*i2c).dev, regmap)
}

unsafe extern "C" fn pcm3168a_i2c_remove(i2c: *mut i2c_client) {
    pcm3168a_remove(&mut (*i2c).dev);
}

static pcm3168a_i2c_id: [i2c_device_id; 2usize] = [
    i2c_device_id {
        name: [
            b'p' as c_char,
            b'c' as c_char,
            b'm' as c_char,
            b'3' as c_char,
            b'1' as c_char,
            b'6' as c_char,
            b'8' as c_char,
            b'a' as c_char,
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
        driver_data: 0,
    },
    i2c_device_id {
        name: [0; 20usize],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(i2c, pcm3168a_i2c_id);

static pcm3168a_acpi_match: [acpi_device_id; 3usize] = [
    acpi_device_id {
        id: [
            b'P' as c_char,
            b'C' as c_char,
            b'M' as c_char,
            b'3' as c_char,
            b'1' as c_char,
            b'6' as c_char,
            b'8' as c_char,
            b'A' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        driver_data: 0,
    },
    acpi_device_id {
        id: [
            b'1' as c_char,
            b'0' as c_char,
            b'4' as c_char,
            b'C' as c_char,
            b'3' as c_char,
            b'1' as c_char,
            b'6' as c_char,
            b'8' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        driver_data: 0,
    },
    acpi_device_id {
        id: [0; 16usize],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(acpi, pcm3168a_acpi_match);

static pcm3168a_of_match: [of_device_id; 2usize] = [
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: b"ti,pcm3168a\0".as_ptr() as *const c_char,
        data: ptr::null(),
    },
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, pcm3168a_of_match);

static mut pcm3168a_i2c_driver: i2c_driver = i2c_driver {
    probe: Some(pcm3168a_i2c_probe),
    remove: Some(pcm3168a_i2c_remove),
    id_table: pcm3168a_i2c_id.as_ptr(),
    driver: device_driver {
        name: b"pcm3168a\0".as_ptr() as *const c_char,
        acpi_match_table: pcm3168a_acpi_match.as_ptr(),
        of_match_table: pcm3168a_of_match.as_ptr(),
        pm: unsafe { pm_ptr(&pcm3168a_pm_ops) },
    },
};
// module_i2c_driver(pcm3168a_i2c_driver);

// MODULE_DESCRIPTION("PCM3168A I2C codec driver");
// MODULE_AUTHOR("Damien Horsley <Damien.Horsley@imgtec.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
