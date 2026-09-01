// SPDX-License-Identifier: GPL-2.0-only
//
// CS35L56 ALSA SoC audio driver I2C binding
//
// Copyright (C) 2023 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

// Dependencies from:
// <linux/acpi.h>
// <linux/i2c.h>
// <linux/module.h>
// <linux/moduleparam.h>
// <linux/regmap.h>
// <linux/slab.h>
// <linux/types.h>
// "cs35l56.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;

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
}

#[repr(C)]
pub struct cs35l56_base {
    pub dev: *mut device,
    pub can_hibernate: bool,
    pub type_: c_uint,
    pub regmap: *mut regmap,
}

#[repr(C)]
pub struct cs35l56_private {
    pub base: cs35l56_base,
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
    pub pm: *const dev_pm_ops,
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
    static cs35l56_regmap_i2c: regmap_config;
    static cs35l63_regmap_i2c: regmap_config;
    static cs35l56_pm_ops_i2c_spi: dev_pm_ops;

    fn i2c_get_match_data(client: *mut i2c_client) -> *const c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(
        client: *mut i2c_client,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;

    fn cs35l56_common_probe(cs35l56: *mut cs35l56_private, irq: c_int) -> c_int;
    fn cs35l56_remove(cs35l56: *mut cs35l56_private);
}

unsafe extern "C" fn cs35l56_i2c_probe(client: *mut i2c_client) -> c_int {
    let id: c_uint = i2c_get_match_data(client) as usize as u32 as c_uint;
    let cs35l56: *mut cs35l56_private;
    let dev: *mut device = unsafe { &mut (*client).dev };
    let regmap_config: *const regmap_config;
    let ret: c_int;

    cs35l56 = unsafe {
        devm_kzalloc(dev, size_of::<cs35l56_private>(), GFP_KERNEL) as *mut cs35l56_private
    };
    if cs35l56.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*cs35l56).base.dev = dev;
        (*cs35l56).base.can_hibernate = true;
    }

    unsafe {
        i2c_set_clientdata(client, cs35l56 as *mut c_void);
    }

    match id {
        0x3556 => {
            regmap_config = unsafe { &cs35l56_regmap_i2c };
            unsafe {
                (*cs35l56).base.type_ = 0x56;
            }
        }
        0x3563 => {
            regmap_config = unsafe { &cs35l63_regmap_i2c };
            unsafe {
                (*cs35l56).base.type_ = 0x63;
            }
        }
        _ => {
            return -ENODEV;
        }
    }

    unsafe {
        (*cs35l56).base.regmap = devm_regmap_init_i2c(client, regmap_config);
    }
    if unsafe { IS_ERR((*cs35l56).base.regmap as *const c_void) } {
        ret = unsafe { PTR_ERR((*cs35l56).base.regmap as *const c_void) };
        return unsafe {
            dev_err_probe(
                (*cs35l56).base.dev,
                ret,
                c"Failed to allocate register map\n".as_ptr(),
            )
        };
    }

    unsafe { cs35l56_common_probe(cs35l56, (*client).irq) }
}

unsafe extern "C" fn cs35l56_i2c_remove(client: *mut i2c_client) {
    let cs35l56: *mut cs35l56_private =
        unsafe { i2c_get_clientdata(client) as *mut cs35l56_private };

    unsafe {
        cs35l56_remove(cs35l56);
    }
}

static cs35l56_id_i2c: [i2c_device_id; 3] = [
    i2c_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'3' as c_char,
            b'5' as c_char,
            b'l' as c_char,
            b'5' as c_char,
            b'6' as c_char,
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
        driver_data: 0x3556,
    },
    i2c_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'3' as c_char,
            b'5' as c_char,
            b'l' as c_char,
            b'6' as c_char,
            b'3' as c_char,
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
        driver_data: 0x3563,
    },
    i2c_device_id {
        name: [0; 20],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(i2c, cs35l56_id_i2c);

// #ifdef CONFIG_ACPI
static cs35l56_asoc_acpi_match: [acpi_device_id; 3] = [
    acpi_device_id {
        id: [
            b'C' as c_char,
            b'S' as c_char,
            b'C' as c_char,
            b'3' as c_char,
            b'5' as c_char,
            b'5' as c_char,
            b'C' as c_char,
            0,
            0,
        ],
        driver_data: 0x3556,
    },
    acpi_device_id {
        id: [
            b'C' as c_char,
            b'S' as c_char,
            b'C' as c_char,
            b'3' as c_char,
            b'5' as c_char,
            b'6' as c_char,
            b'C' as c_char,
            0,
            0,
        ],
        driver_data: 0x3563,
    },
    acpi_device_id {
        id: [0; 9],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(acpi, cs35l56_asoc_acpi_match);
// #endif

static mut cs35l56_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"cs35l56".as_ptr(),
        pm: unsafe { &cs35l56_pm_ops_i2c_spi },
        acpi_match_table: cs35l56_asoc_acpi_match.as_ptr(),
    },
    id_table: cs35l56_id_i2c.as_ptr(),
    probe: Some(cs35l56_i2c_probe),
    remove: Some(cs35l56_i2c_remove),
};

// module_i2c_driver(cs35l56_i2c_driver);

// MODULE_DESCRIPTION("ASoC CS35L56 I2C driver");
// MODULE_IMPORT_NS("SND_SOC_CS35L56_CORE");
// MODULE_IMPORT_NS("SND_SOC_CS35L56_SHARED");
// MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
// MODULE_AUTHOR("Simon Trimmer <simont@opensource.cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
