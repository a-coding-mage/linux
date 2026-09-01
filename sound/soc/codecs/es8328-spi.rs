// SPDX-License-Identifier: GPL-2.0-only
/*
 * es8328.c  --  ES8328 ALSA SoC SPI Audio driver
 *
 * Copyright 2014 Sutajio Ko-Usagi PTE LTD
 *
 * Author: Sean Cross <xobs@kosagi.com>
 */

// C dependencies:
// #include <linux/module.h>
// #include <linux/regmap.h>
// #include <linux/spi/spi.h>
// #include <sound/soc.h>
// #include "es8328.h"

use core::ffi::{c_char, c_int};

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
pub struct spi_device {
    pub dev: device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
}

unsafe extern "C" {
    static es8328_regmap_config: regmap_config;

    fn es8328_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
}

static ES8328_COMPATIBLE: &[u8; 14] = b"everest,es8328\0";
static ES8328_NAME: &[u8; 7] = b"es8328\0";

static es8328_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: ES8328_COMPATIBLE.as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, es8328_of_match);

unsafe extern "C" fn es8328_spi_probe(spi: *mut spi_device) -> c_int {
    unsafe {
        es8328_probe(
            &mut (*spi).dev,
            devm_regmap_init_spi(spi, &es8328_regmap_config),
        )
    }
}

static es8328_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: ES8328_NAME.as_ptr() as *const c_char,
        of_match_table: es8328_of_match.as_ptr(),
    },
    probe: Some(es8328_spi_probe),
};

// module_spi_driver(es8328_spi_driver);
// MODULE_DESCRIPTION("ASoC ES8328 audio CODEC SPI driver");
// MODULE_AUTHOR("Sean Cross <xobs@kosagi.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
