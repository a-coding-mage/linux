// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8804-spi.c  --  WM8804 S/PDIF transceiver driver - SPI
 *
 * Copyright 2015 Cirrus Logic Inc
 *
 * Author: Charles Keepax <ckeepax@opensource.wolfsonmicro.com>
 */

// Dependencies from the original C file:
// #include <linux/init.h>
// #include <linux/module.h>
// #include <linux/spi/spi.h>
// #include "wm8804.h"

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

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
pub struct of_device_id {
    pub name: *const c_char,
    pub type_: *const c_char,
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut spi_device)>,
}

unsafe extern "C" {
    static wm8804_regmap_config: regmap_config;
    static wm8804_pm: dev_pm_ops;

    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn wm8804_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
    fn wm8804_remove(dev: *mut device);

    fn __module_spi_driver(driver: *mut spi_driver);
}

unsafe extern "C" fn wm8804_spi_probe(spi: *mut spi_device) -> c_int {
    let regmap: *mut regmap;

    regmap = unsafe { devm_regmap_init_spi(spi, &wm8804_regmap_config) };
    if unsafe { IS_ERR(regmap as *const c_void) } {
        return unsafe { PTR_ERR(regmap as *const c_void) };
    }

    unsafe { wm8804_probe(&mut (*spi).dev, regmap) }
}

unsafe extern "C" fn wm8804_spi_remove(spi: *mut spi_device) {
    unsafe {
        wm8804_remove(&mut (*spi).dev);
    }
}

#[used]
static wm8804_of_match: [of_device_id; 2] = [
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: b"wlf,wm8804\0".as_ptr() as *const c_char,
        data: ptr::null(),
    },
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: ptr::null(),
        data: ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, wm8804_of_match);

#[used]
static mut wm8804_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: b"wm8804\0".as_ptr() as *const c_char,
        pm: unsafe { &wm8804_pm },
        of_match_table: wm8804_of_match.as_ptr(),
    },
    probe: Some(wm8804_spi_probe),
    remove: Some(wm8804_spi_remove),
};

// module_spi_driver(wm8804_spi_driver);
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_module() -> c_int {
    unsafe {
        __module_spi_driver(&raw mut wm8804_spi_driver);
    }

    0
}

// MODULE_DESCRIPTION("ASoC WM8804 driver - SPI");
// MODULE_AUTHOR("Charles Keepax <ckeepax@opensource.wolfsonmicro.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
