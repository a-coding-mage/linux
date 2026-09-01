// SPDX-License-Identifier: GPL-2.0
//
// CS35l41 HDA I2C driver
//
// Copyright 2021 Cirrus Logic, Inc.
//
// Author: Lucas Tanure <tanureal@opensource.cirrus.com>

// C dependencies:
// #include <linux/module.h>
// #include <linux/i2c.h>
// #include "cs35l41_hda.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
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
pub struct dev_pm_ops {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub addr: c_uint,
    pub irq: c_int,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
    pub driver_data: usize,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 16],
    pub driver_data: usize,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub acpi_match_table: *const acpi_device_id,
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
    static cs35l41_regmap_i2c: regmap_config;
    static cs35l41_hda_pm_ops: dev_pm_ops;

    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn dev_name(dev: *const device) -> *const c_char;
    fn devm_regmap_init_i2c(clt: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn cs35l41_hda_probe(
        dev: *mut device,
        device_name: *const c_char,
        addr: c_uint,
        irq: c_int,
        regmap: *mut regmap,
        bus: c_int,
    ) -> c_int;
    fn cs35l41_hda_remove(dev: *mut device);
}

const ENODEV: c_int = 19;
const I2C: c_int = 1;

unsafe extern "C" fn cs35l41_hda_i2c_probe(clt: *mut i2c_client) -> c_int {
    let device_name: *const c_char;

    /*
     * Compare against the device name so it works for SPI, normal ACPI
     * and for ACPI by serial-multi-instantiate matching cases.
     */
    if unsafe { !strstr(dev_name(&raw const (*clt).dev), c"CLSA0100".as_ptr()).is_null() } {
        device_name = c"CLSA0100".as_ptr();
    } else if unsafe { !strstr(dev_name(&raw const (*clt).dev), c"CLSA0101".as_ptr()).is_null() } {
        device_name = c"CLSA0101".as_ptr();
    } else if unsafe { !strstr(dev_name(&raw const (*clt).dev), c"CSC3551".as_ptr()).is_null() } {
        device_name = c"CSC3551".as_ptr();
    } else {
        return -ENODEV;
    }

    unsafe {
        cs35l41_hda_probe(
            &raw mut (*clt).dev,
            device_name,
            (*clt).addr,
            (*clt).irq,
            devm_regmap_init_i2c(clt, &raw const cs35l41_regmap_i2c),
            I2C,
        )
    }
}

unsafe extern "C" fn cs35l41_hda_i2c_remove(clt: *mut i2c_client) {
    unsafe {
        cs35l41_hda_remove(&raw mut (*clt).dev);
    }
}

static cs35l41_hda_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'3' as c_char,
            b'5' as c_char,
            b'l' as c_char,
            b'4' as c_char,
            b'1' as c_char,
            b'-' as c_char,
            b'h' as c_char,
            b'd' as c_char,
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
        ],
        driver_data: 0,
    },
    i2c_device_id {
        name: [0; 20],
        driver_data: 0,
    },
];

static cs35l41_acpi_hda_match: [acpi_device_id; 4] = [
    acpi_device_id {
        id: [
            b'C' as c_char,
            b'L' as c_char,
            b'S' as c_char,
            b'A' as c_char,
            b'0' as c_char,
            b'1' as c_char,
            b'0' as c_char,
            b'0' as c_char,
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
            b'C' as c_char,
            b'L' as c_char,
            b'S' as c_char,
            b'A' as c_char,
            b'0' as c_char,
            b'1' as c_char,
            b'0' as c_char,
            b'1' as c_char,
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
            b'C' as c_char,
            b'S' as c_char,
            b'C' as c_char,
            b'3' as c_char,
            b'5' as c_char,
            b'5' as c_char,
            b'1' as c_char,
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
    acpi_device_id {
        id: [0; 16],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(acpi, cs35l41_acpi_hda_match);

static mut cs35l41_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"cs35l41-hda".as_ptr(),
        acpi_match_table: cs35l41_acpi_hda_match.as_ptr(),
        pm: &raw const cs35l41_hda_pm_ops,
    },
    id_table: cs35l41_hda_i2c_id.as_ptr(),
    probe: Some(cs35l41_hda_i2c_probe),
    remove: Some(cs35l41_hda_i2c_remove),
};
// module_i2c_driver(cs35l41_i2c_driver);

// MODULE_DESCRIPTION("HDA CS35L41 driver");
// MODULE_IMPORT_NS("SND_HDA_SCODEC_CS35L41");
// MODULE_AUTHOR("Lucas Tanure <tanureal@opensource.cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
