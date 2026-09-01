// SPDX-License-Identifier: GPL-2.0
//
// cs35l41-i2c.c -- CS35l41 I2C driver
//
// Copyright 2017-2021 Cirrus Logic, Inc.
//
// Author: David Rhodes <david.rhodes@cirrus.com>

// C dependencies:
// linux/acpi.h, linux/delay.h, linux/i2c.h, linux/init.h, linux/kernel.h,
// linux/module.h, linux/moduleparam.h, linux/of.h, linux/platform_device.h,
// linux/slab.h, and "cs35l41.h".

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const GFP_KERNEL: c_ulong = 0;
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
pub struct cs35l41_hw_cfg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cs35l41_private {
    pub dev: *mut device,
    pub irq: c_int,
    pub regmap: *mut regmap,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub irq: c_int,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct of_device_id {
    pub name: [c_char; 32],
    pub type_: [c_char; 32],
    pub compatible: [c_char; 128],
    pub data: *const c_void,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 16],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub id_table: *const i2c_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
}

unsafe extern "C" {
    static cs35l41_regmap_i2c: regmap_config;
    static cs35l41_pm_ops: dev_pm_ops;

    fn dev_get_platdata(dev: *mut device) -> *mut cs35l41_hw_cfg;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_ulong) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn devm_regmap_init_i2c(
        client: *mut i2c_client,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn dev_err_probe(dev: *mut device, err: c_long, fmt: *const c_char, ...) -> c_int;
    fn cs35l41_probe(cs35l41: *mut cs35l41_private, hw_cfg: *mut cs35l41_hw_cfg) -> c_int;
    fn cs35l41_remove(cs35l41: *mut cs35l41_private);
}

const fn c_char_array_20(s: &[u8]) -> [c_char; 20] {
    let mut out = [0 as c_char; 20];
    let mut i = 0;

    while i < s.len() && i < 19 {
        out[i] = s[i] as c_char;
        i += 1;
    }

    out
}

const fn c_char_array_16(s: &[u8]) -> [c_char; 16] {
    let mut out = [0 as c_char; 16];
    let mut i = 0;

    while i < s.len() && i < 15 {
        out[i] = s[i] as c_char;
        i += 1;
    }

    out
}

const fn c_char_array_32(_s: &[u8]) -> [c_char; 32] {
    [0 as c_char; 32]
}

const fn c_char_array_128(s: &[u8]) -> [c_char; 128] {
    let mut out = [0 as c_char; 128];
    let mut i = 0;

    while i < s.len() && i < 127 {
        out[i] = s[i] as c_char;
        i += 1;
    }

    out
}

static cs35l41_id_i2c: [i2c_device_id; 5] = [
    i2c_device_id {
        name: c_char_array_20(b"cs35l40"),
        driver_data: 0,
    },
    i2c_device_id {
        name: c_char_array_20(b"cs35l41"),
        driver_data: 0,
    },
    i2c_device_id {
        name: c_char_array_20(b"cs35l51"),
        driver_data: 0,
    },
    i2c_device_id {
        name: c_char_array_20(b"cs35l53"),
        driver_data: 0,
    },
    i2c_device_id {
        name: [0 as c_char; 20],
        driver_data: 0,
    },
];

// MODULE_DEVICE_TABLE(i2c, cs35l41_id_i2c);

unsafe fn is_err(ptr: *const c_void) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn ptr_err(ptr: *const c_void) -> c_long {
    ptr as c_long
}

unsafe extern "C" fn cs35l41_i2c_probe(client: *mut i2c_client) -> c_int {
    let cs35l41: *mut cs35l41_private;
    let dev: *mut device = unsafe { &mut (*client).dev };
    let hw_cfg: *mut cs35l41_hw_cfg = unsafe { dev_get_platdata(dev) };
    let regmap_config: *const regmap_config = unsafe { &cs35l41_regmap_i2c };

    cs35l41 = unsafe {
        devm_kzalloc(dev, size_of::<cs35l41_private>(), GFP_KERNEL) as *mut cs35l41_private
    };

    if cs35l41.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*cs35l41).dev = dev;
        (*cs35l41).irq = (*client).irq;

        i2c_set_clientdata(client, cs35l41 as *mut c_void);
        (*cs35l41).regmap = devm_regmap_init_i2c(client, regmap_config);
        if is_err((*cs35l41).regmap as *const c_void) {
            return dev_err_probe(
                (*cs35l41).dev,
                ptr_err((*cs35l41).regmap as *const c_void),
                c"Failed to allocate register map\n".as_ptr(),
            );
        }

        cs35l41_probe(cs35l41, hw_cfg)
    }
}

unsafe extern "C" fn cs35l41_i2c_remove(client: *mut i2c_client) {
    let cs35l41: *mut cs35l41_private =
        unsafe { i2c_get_clientdata(client) as *mut cs35l41_private };

    unsafe {
        cs35l41_remove(cs35l41);
    }
}

// Original C condition: #ifdef CONFIG_OF
static cs35l41_of_match: [of_device_id; 3] = [
    of_device_id {
        name: c_char_array_32(b""),
        type_: c_char_array_32(b""),
        compatible: c_char_array_128(b"cirrus,cs35l40"),
        data: ptr::null(),
    },
    of_device_id {
        name: c_char_array_32(b""),
        type_: c_char_array_32(b""),
        compatible: c_char_array_128(b"cirrus,cs35l41"),
        data: ptr::null(),
    },
    of_device_id {
        name: [0 as c_char; 32],
        type_: [0 as c_char; 32],
        compatible: [0 as c_char; 128],
        data: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, cs35l41_of_match);

// Original C condition: #ifdef CONFIG_ACPI
static cs35l41_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id {
        id: c_char_array_16(b"CSC3541"),
        driver_data: 0,
    }, /* Cirrus Logic PnP ID + part ID */
    acpi_device_id {
        id: [0 as c_char; 16],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(acpi, cs35l41_acpi_match);

const fn pm_ptr(pm: *const dev_pm_ops) -> *const dev_pm_ops {
    pm
}

const fn of_match_ptr(matches: *const of_device_id) -> *const of_device_id {
    matches
}

const fn acpi_ptr(matches: *const acpi_device_id) -> *const acpi_device_id {
    matches
}

static mut cs35l41_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"cs35l41".as_ptr(),
        pm: unsafe { pm_ptr(&cs35l41_pm_ops) },
        of_match_table: of_match_ptr(cs35l41_of_match.as_ptr()),
        acpi_match_table: acpi_ptr(cs35l41_acpi_match.as_ptr()),
    },
    id_table: cs35l41_id_i2c.as_ptr(),
    probe: Some(cs35l41_i2c_probe),
    remove: Some(cs35l41_i2c_remove),
};

// module_i2c_driver(cs35l41_i2c_driver);

// MODULE_DESCRIPTION("I2C CS35L41 driver");
// MODULE_AUTHOR("David Rhodes, Cirrus Logic Inc, <david.rhodes@cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
