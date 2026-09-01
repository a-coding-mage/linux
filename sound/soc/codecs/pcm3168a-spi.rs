// SPDX-License-Identifier: GPL-2.0-only
/*
 * PCM3168A codec spi driver
 *
 * Copyright (C) 2015 Imagination Technologies Ltd.
 *
 * Author: Damien Horsley <Damien.Horsley@imgtec.com>
 */

// C dependencies:
// #include <linux/init.h>
// #include <linux/module.h>
// #include <linux/spi/spi.h>
// #include <sound/soc.h>
// #include "pcm3168a.h"

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
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
pub struct spi_device_id {
    pub name: [c_char; 32],
    pub driver_data: usize,
}

#[repr(C)]
pub struct of_device_id {
    pub name: [c_char; 32],
    pub type_: [c_char; 32],
    pub compatible: [c_char; 128],
    pub data: *const c_void,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct spi_driver {
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut spi_device)>,
    pub id_table: *const spi_device_id,
    pub driver: device_driver,
}

unsafe extern "C" {
    static pcm3168a_regmap: regmap_config;
    static pcm3168a_pm_ops: dev_pm_ops;

    fn devm_regmap_init_spi(
        spi: *mut spi_device,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pcm3168a_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
    fn pcm3168a_remove(dev: *mut device);
    fn pm_ptr(ptr: *const dev_pm_ops) -> *const dev_pm_ops;
}

unsafe extern "C" fn pcm3168a_spi_probe(spi: *mut spi_device) -> c_int {
    let regmap: *mut regmap;

    regmap = unsafe { devm_regmap_init_spi(spi, &raw const pcm3168a_regmap) };
    if unsafe { IS_ERR(regmap as *const c_void) } {
        return unsafe { PTR_ERR(regmap as *const c_void) };
    }

    unsafe { pcm3168a_probe(&raw mut (*spi).dev, regmap) }
}

unsafe extern "C" fn pcm3168a_spi_remove(spi: *mut spi_device) {
    unsafe {
        pcm3168a_remove(&raw mut (*spi).dev);
    }
}

static pcm3168a_spi_id: [spi_device_id; 2] = [
    spi_device_id {
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
    spi_device_id {
        name: [0; 32],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(spi, pcm3168a_spi_id);

static pcm3168a_of_match: [of_device_id; 2] = [
    of_device_id {
        name: [0; 32],
        type_: [0; 32],
        compatible: [
            b't' as c_char,
            b'i' as c_char,
            b',' as c_char,
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
        data: core::ptr::null(),
    },
    of_device_id {
        name: [0; 32],
        type_: [0; 32],
        compatible: [0; 128],
        data: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, pcm3168a_of_match);

static mut pcm3168a_spi_driver: spi_driver = spi_driver {
    probe: Some(pcm3168a_spi_probe),
    remove: Some(pcm3168a_spi_remove),
    id_table: pcm3168a_spi_id.as_ptr(),
    driver: device_driver {
        name: c"pcm3168a".as_ptr(),
        of_match_table: pcm3168a_of_match.as_ptr(),
        pm: unsafe { pm_ptr(&raw const pcm3168a_pm_ops) },
    },
};
// module_spi_driver(pcm3168a_spi_driver);

// MODULE_DESCRIPTION("PCM3168A SPI codec driver");
// MODULE_AUTHOR("Damien Horsley <Damien.Horsley@imgtec.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
