// SPDX-License-Identifier: GPL-2.0-only
//
// CS35L56 HDA audio driver SPI binding
//
// Copyright (C) 2023 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

// C dependencies:
// #include <linux/module.h>
// #include <linux/regmap.h>
// #include <linux/spi/spi.h>
// #include "cs35l56_hda.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;

const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

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
pub struct spi_device {
    pub dev: device,
    pub irq: c_int,
}

#[repr(C)]
pub struct spi_device_id {
    pub name: [c_char; 32],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 16],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub acpi_match_table: *const acpi_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: device_driver,
    pub id_table: *const spi_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut spi_device)>,
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
    static cs35l56_regmap_spi: regmap_config;
    static cs35l56_hda_pm_ops: dev_pm_ops;

    fn spi_get_device_id(spi: *mut spi_device) -> *const spi_device_id;
    fn spi_get_chipselect(spi: *mut spi_device, idx: c_uint) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn cs35l56_init_config_for_spi(base: *mut cs35l56_base, spi: *mut spi_device) -> c_int;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn cs35l56_hda_common_probe(
        cs35l56: *mut cs35l56_hda,
        driver_data: c_ulong,
        chipselect: c_uint,
    ) -> c_int;
    fn cs35l56_irq_request(base: *mut cs35l56_base, irq: c_int) -> c_int;
    fn cs35l56_hda_remove(dev: *mut device);
}

unsafe extern "C" fn cs35l56_hda_spi_probe(spi: *mut spi_device) -> c_int {
    let id: *const spi_device_id = unsafe { spi_get_device_id(spi) };
    let cs35l56: *mut cs35l56_hda;
    let mut ret: c_int;

    cs35l56 = unsafe {
        devm_kzalloc(
            &mut (*spi).dev,
            size_of::<cs35l56_hda>(),
            GFP_KERNEL,
        ) as *mut cs35l56_hda
    };
    if cs35l56.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*cs35l56).base.dev = &mut (*spi).dev;
    }
    ret = unsafe { cs35l56_init_config_for_spi(&mut (*cs35l56).base, spi) };
    if ret != 0 {
        return ret;
    }

    // #ifdef CS35L56_WAKE_HOLD_TIME_US
    // cs35l56->base.can_hibernate = true;
    // #endif
    #[cfg(CS35L56_WAKE_HOLD_TIME_US)]
    unsafe {
        (*cs35l56).base.can_hibernate = true;
    }

    unsafe {
        (*cs35l56).base.regmap = devm_regmap_init_spi(spi, &cs35l56_regmap_spi);
    }
    if unsafe { IS_ERR((*cs35l56).base.regmap as *const c_void) } {
        ret = unsafe { PTR_ERR((*cs35l56).base.regmap as *const c_void) };
        unsafe {
            dev_err(
                (*cs35l56).base.dev,
                c"Failed to allocate register map: %d\n".as_ptr(),
                ret,
            );
        }
        return ret;
    }

    ret = unsafe {
        cs35l56_hda_common_probe(
            cs35l56,
            (*id).driver_data,
            spi_get_chipselect(spi, 0),
        )
    };
    if ret != 0 {
        return ret;
    }
    ret = unsafe { cs35l56_irq_request(&mut (*cs35l56).base, (*spi).irq) };
    if ret < 0 {
        unsafe {
            cs35l56_hda_remove((*cs35l56).base.dev);
        }
    }

    ret
}

unsafe extern "C" fn cs35l56_hda_spi_remove(spi: *mut spi_device) {
    unsafe {
        cs35l56_hda_remove(&mut (*spi).dev);
    }
}

static cs35l56_hda_spi_id: [spi_device_id; 4] = [
    spi_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'3' as c_char,
            b'5' as c_char,
            b'l' as c_char,
            b'5' as c_char,
            b'4' as c_char,
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
        driver_data: 0x3554,
    },
    spi_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'3' as c_char,
            b'5' as c_char,
            b'l' as c_char,
            b'5' as c_char,
            b'6' as c_char,
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
    spi_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'3' as c_char,
            b'5' as c_char,
            b'l' as c_char,
            b'5' as c_char,
            b'7' as c_char,
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
        driver_data: 0x3557,
    },
    spi_device_id {
        name: [0; 32],
        driver_data: 0,
    },
];

static cs35l56_acpi_hda_match: [acpi_device_id; 4] = [
    acpi_device_id {
        id: [
            b'C' as c_char,
            b'S' as c_char,
            b'C' as c_char,
            b'3' as c_char,
            b'5' as c_char,
            b'5' as c_char,
            b'4' as c_char,
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
        id: [
            b'C' as c_char,
            b'S' as c_char,
            b'C' as c_char,
            b'3' as c_char,
            b'5' as c_char,
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
            b'7' as c_char,
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
// MODULE_DEVICE_TABLE(acpi, cs35l56_acpi_hda_match);

static mut cs35l56_hda_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: c"cs35l56-hda".as_ptr(),
        acpi_match_table: cs35l56_acpi_hda_match.as_ptr(),
        pm: unsafe { &cs35l56_hda_pm_ops },
    },
    id_table: cs35l56_hda_spi_id.as_ptr(),
    probe: Some(cs35l56_hda_spi_probe),
    remove: Some(cs35l56_hda_spi_remove),
};
// module_spi_driver(cs35l56_hda_spi_driver);

// MODULE_DESCRIPTION("HDA CS35L56 SPI driver");
// MODULE_IMPORT_NS("SND_HDA_SCODEC_CS35L56");
// MODULE_IMPORT_NS("SND_SOC_CS35L56_SHARED");
// MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
// MODULE_AUTHOR("Simon Trimmer <simont@opensource.cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
