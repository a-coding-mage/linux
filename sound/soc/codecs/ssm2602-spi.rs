// SPDX-License-Identifier: GPL-2.0-only
/*
 * SSM2602 SPI audio driver
 *
 * Copyright 2014 Analog Devices Inc.
 */

// C includes translated as external dependencies:
// linux/module.h, linux/spi/spi.h, linux/regmap.h, sound/soc.h, "ssm2602.h"

use core::ffi::{c_char, c_int, c_void};

pub const SSM2602: c_int = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_device {
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
    static ssm2602_regmap_config: regmap_config;

    fn ssm2602_probe(dev: *mut device, type_: c_int, regmap: *mut regmap) -> c_int;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
}

unsafe extern "C" fn ssm2602_spi_probe(spi: *mut spi_device) -> c_int {
    unsafe {
        ssm2602_probe(
            &mut (*spi).dev,
            SSM2602,
            devm_regmap_init_spi(spi, &ssm2602_regmap_config),
        )
    }
}

static SSM2602_COMPATIBLE: &[u8; 12] = b"adi,ssm2602\0";
static SSM2602_DRIVER_NAME: &[u8; 8] = b"ssm2602\0";

static ssm2602_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: SSM2602_COMPATIBLE.as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, ssm2602_of_match);

static mut ssm2602_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: SSM2602_DRIVER_NAME.as_ptr() as *const c_char,
        of_match_table: ssm2602_of_match.as_ptr(),
    },
    probe: Some(ssm2602_spi_probe),
};

// module_spi_driver(ssm2602_spi_driver);

#[used]
static MODULE_DESCRIPTION: &[u8; 26] = b"ASoC SSM2602 SPI driver\0";

#[used]
static MODULE_AUTHOR: &[u8; 10] = b"Cliff Cai\0";

#[used]
static MODULE_LICENSE: &[u8; 4] = b"GPL\0";


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
