// SPDX-License-Identifier: GPL-2.0-only
//
// CS35L56 HDA audio driver I2C binding
//
// Copyright (C) 2023 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

// C dependencies:
// #include <linux/i2c.h>
// #include <linux/module.h>
// #include <linux/regmap.h>
// #include "cs35l56_hda.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const GFP_KERNEL: c_uint = 0;
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
pub struct dev_pm_ops {
    _private: [u8; 0],
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

#[repr(C)]
pub struct cs35l56_base {
    pub dev: *mut device,
    pub can_hibernate: bool,
    pub regmap: *mut regmap,
}

#[repr(C)]
pub struct cs35l56_hda {
    pub base: cs35l56_base,
}

unsafe extern "C" {
    static cs35l56_regmap_i2c: regmap_config;
    static cs35l56_hda_pm_ops: dev_pm_ops;

    fn i2c_client_get_device_id(clt: *mut i2c_client) -> *const i2c_device_id;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(
        clt: *mut i2c_client,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn cs35l56_hda_common_probe(
        cs35l56: *mut cs35l56_hda,
        driver_data: usize,
        addr: c_uint,
    ) -> c_int;
    fn cs35l56_irq_request(base: *mut cs35l56_base, irq: c_int) -> c_int;
    fn cs35l56_hda_remove(dev: *mut device);
    fn module_i2c_driver(driver: *mut i2c_driver);
}

unsafe extern "C" fn cs35l56_hda_i2c_probe(clt: *mut i2c_client) -> c_int {
    let id: *const i2c_device_id = i2c_client_get_device_id(clt);
    let cs35l56: *mut cs35l56_hda;
    let mut ret: c_int;

    cs35l56 = devm_kzalloc(
        ptr::addr_of_mut!((*clt).dev),
        size_of::<cs35l56_hda>(),
        GFP_KERNEL,
    ) as *mut cs35l56_hda;
    if cs35l56.is_null() {
        return -ENOMEM;
    }

    (*cs35l56).base.dev = ptr::addr_of_mut!((*clt).dev);

    // C conditional: #ifdef CS35L56_WAKE_HOLD_TIME_US
    #[cfg(CS35L56_WAKE_HOLD_TIME_US)]
    {
        (*cs35l56).base.can_hibernate = true;
    }

    (*cs35l56).base.regmap =
        devm_regmap_init_i2c(clt, ptr::addr_of!(cs35l56_regmap_i2c));
    if IS_ERR((*cs35l56).base.regmap as *const c_void) {
        ret = PTR_ERR((*cs35l56).base.regmap as *const c_void);
        dev_err(
            (*cs35l56).base.dev,
            c"Failed to allocate register map: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = cs35l56_hda_common_probe(cs35l56, (*id).driver_data, (*clt).addr);
    if ret != 0 {
        return ret;
    }
    ret = cs35l56_irq_request(ptr::addr_of_mut!((*cs35l56).base), (*clt).irq);
    if ret < 0 {
        cs35l56_hda_remove((*cs35l56).base.dev);
    }

    ret
}

unsafe extern "C" fn cs35l56_hda_i2c_remove(clt: *mut i2c_client) {
    cs35l56_hda_remove(ptr::addr_of_mut!((*clt).dev));
}

static cs35l56_hda_i2c_id: [i2c_device_id; 4] = [
    i2c_device_id {
        name: *b"cs35l54-hda\0\0\0\0\0\0\0\0\0",
        driver_data: 0x3554,
    },
    i2c_device_id {
        name: *b"cs35l56-hda\0\0\0\0\0\0\0\0\0",
        driver_data: 0x3556,
    },
    i2c_device_id {
        name: *b"cs35l57-hda\0\0\0\0\0\0\0\0\0",
        driver_data: 0x3557,
    },
    i2c_device_id {
        name: [0; 20],
        driver_data: 0,
    },
];

static cs35l56_acpi_hda_match: [acpi_device_id; 4] = [
    acpi_device_id {
        id: *b"CSC3554\0\0\0\0\0\0\0\0\0",
        driver_data: 0,
    },
    acpi_device_id {
        id: *b"CSC3556\0\0\0\0\0\0\0\0\0",
        driver_data: 0,
    },
    acpi_device_id {
        id: *b"CSC3557\0\0\0\0\0\0\0\0\0",
        driver_data: 0,
    },
    acpi_device_id {
        id: [0; 16],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(acpi, cs35l56_acpi_hda_match);

static mut cs35l56_hda_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"cs35l56-hda".as_ptr(),
        acpi_match_table: cs35l56_acpi_hda_match.as_ptr(),
        pm: ptr::addr_of!(cs35l56_hda_pm_ops),
    },
    id_table: cs35l56_hda_i2c_id.as_ptr(),
    probe: Some(cs35l56_hda_i2c_probe),
    remove: Some(cs35l56_hda_i2c_remove),
};

unsafe extern "C" fn __register_cs35l56_hda_i2c_driver() {
    module_i2c_driver(ptr::addr_of_mut!(cs35l56_hda_i2c_driver));
}

// module_i2c_driver(cs35l56_hda_i2c_driver);
// MODULE_DESCRIPTION("HDA CS35L56 I2C driver");
// MODULE_IMPORT_NS("SND_HDA_SCODEC_CS35L56");
// MODULE_IMPORT_NS("SND_SOC_CS35L56_SHARED");
// MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
// MODULE_AUTHOR("Simon Trimmer <simont@opensource.cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
