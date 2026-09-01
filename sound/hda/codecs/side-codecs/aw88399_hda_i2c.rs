// SPDX-License-Identifier: GPL-2.0-only
//
// AW88399 HDA I2C driver
//
// Based on cs35l41_hda_i2c.c
//

// C includes translated as external dependency intent:
// #include <linux/i2c.h>
// #include <linux/module.h>
// #include <linux/regmap.h>
// #include "aw88399_hda.h"

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

const ENODEV: c_int = 19;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
    pub driver_data: usize,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 9],
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
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    static aw88399_remap_config: regmap_config;
    static aw88399_hda_pm_ops: dev_pm_ops;

    fn dev_name(dev: *const device) -> *const c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn aw88399_hda_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
    fn aw88399_hda_remove(dev: *mut device);
}

unsafe extern "C" fn aw88399_hda_i2c_probe(clt: *mut i2c_client) -> c_int {
    if strstr(dev_name(ptr::addr_of!((*clt).dev)), c"AWDZ8399".as_ptr()).is_null() {
        return -ENODEV;
    }

    aw88399_hda_probe(
        ptr::addr_of_mut!((*clt).dev),
        devm_regmap_init_i2c(clt, ptr::addr_of!(aw88399_remap_config)),
    )
}

unsafe extern "C" fn aw88399_hda_i2c_remove(clt: *mut i2c_client) {
    aw88399_hda_remove(ptr::addr_of_mut!((*clt).dev));
}

static aw88399_hda_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b'a' as c_char,
            b'w' as c_char,
            b'8' as c_char,
            b'8' as c_char,
            b'3' as c_char,
            b'9' as c_char,
            b'9' as c_char,
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

static aw88399_acpi_hda_match: [acpi_device_id; 2] = [
    acpi_device_id {
        id: [
            b'A' as c_char,
            b'W' as c_char,
            b'D' as c_char,
            b'Z' as c_char,
            b'8' as c_char,
            b'3' as c_char,
            b'9' as c_char,
            b'9' as c_char,
            0,
        ],
        driver_data: 0,
    },
    acpi_device_id {
        id: [0; 9],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(acpi, aw88399_acpi_hda_match);

static mut aw88399_hda_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"aw88399-hda".as_ptr(),
        acpi_match_table: aw88399_acpi_hda_match.as_ptr(),
        pm: ptr::addr_of!(aw88399_hda_pm_ops),
    },
    probe: Some(aw88399_hda_i2c_probe),
    remove: Some(aw88399_hda_i2c_remove),
    id_table: aw88399_hda_i2c_id.as_ptr(),
};
// module_i2c_driver(aw88399_hda_i2c_driver);

// MODULE_DESCRIPTION("HDA AW88399 I2C driver");
// MODULE_IMPORT_NS("SND_HDA_SCODEC_AW88399");
// MODULE_AUTHOR("Yakov Till <yakov.till@gmail.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
