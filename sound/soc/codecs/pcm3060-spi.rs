// SPDX-License-Identifier: GPL-2.0
//
// PCM3060 SPI driver
//
// Copyright (C) 2018 Kirill Marinushkin <k.marinushkin@gmail.com>

// C dependencies:
// #include <linux/module.h>
// #include <linux/spi/spi.h>
// #include <sound/soc.h>
// #include "pcm3060.h"

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static pcm3060_regmap: regmap_config;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn spi_set_drvdata(spi: *mut spi_device, data: *mut c_void);
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pcm3060_probe(dev: *mut device) -> c_int;
}

type c_uint = u32;

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
pub struct pcm3060_priv {
    pub regmap: *mut regmap,
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
}

#[repr(C)]
pub struct spi_device_id {
    pub name: [c_char; 32],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    // CONFIG_OF: .of_match_table = pcm3060_of_match,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: device_driver,
    pub id_table: *const spi_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
}

unsafe extern "C" fn pcm3060_spi_probe(spi: *mut spi_device) -> c_int {
    let priv_: *mut pcm3060_priv;

    priv_ = devm_kzalloc(
        unsafe { &mut (*spi).dev },
        core::mem::size_of::<pcm3060_priv>(),
        GFP_KERNEL,
    ) as *mut pcm3060_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    spi_set_drvdata(spi, priv_ as *mut c_void);

    unsafe {
        (*priv_).regmap = devm_regmap_init_spi(spi, &pcm3060_regmap);
        if IS_ERR((*priv_).regmap as *const c_void) {
            return PTR_ERR((*priv_).regmap as *const c_void);
        }
    }

    pcm3060_probe(unsafe { &mut (*spi).dev })
}

static pcm3060_spi_id: [spi_device_id; 2] = [
    spi_device_id {
        name: [
            b'p' as c_char,
            b'c' as c_char,
            b'm' as c_char,
            b'3' as c_char,
            b'0' as c_char,
            b'6' as c_char,
            b'0' as c_char,
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
    },
    spi_device_id { name: [0; 32] },
];
// MODULE_DEVICE_TABLE(spi, pcm3060_spi_id);

// CONFIG_OF:
static pcm3060_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"ti,pcm3060\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, pcm3060_of_match);

static mut pcm3060_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: b"pcm3060\0".as_ptr() as *const c_char,
        // CONFIG_OF:
        of_match_table: pcm3060_of_match.as_ptr(),
    },
    id_table: pcm3060_spi_id.as_ptr(),
    probe: Some(pcm3060_spi_probe),
};

// module_spi_driver(pcm3060_spi_driver);

// MODULE_DESCRIPTION("PCM3060 SPI driver");
// MODULE_AUTHOR("Kirill Marinushkin <k.marinushkin@gmail.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
