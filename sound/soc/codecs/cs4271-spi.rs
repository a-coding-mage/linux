// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * CS4271 SPI audio driver
 *
 * Copyright (c) 2010 Alexander Sverdlin <subaparts@yandex.ru>
 */

// C dependencies: linux/module.h, linux/spi/spi.h, linux/regmap.h,
// sound/soc.h, and "cs4271.h".

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
#[derive(Copy, Clone)]
pub struct regmap_config {
    pub reg_bits: u32,
    pub read_flag_mask: u32,
    pub write_flag_mask: u32,
}

#[repr(C)]
pub struct of_device_id {
    _private: [u8; 0],
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
    static cs4271_regmap_config: regmap_config;
    static cs4271_dt_ids: [of_device_id; 0];

    fn cs4271_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
}

// Direct translation of of_match_ptr(cs4271_dt_ids).
#[inline]
unsafe fn of_match_ptr<T>(ptr: *const T) -> *const T {
    ptr
}

unsafe extern "C" fn cs4271_spi_probe(spi: *mut spi_device) -> c_int {
    let mut config: regmap_config;

    config = unsafe { cs4271_regmap_config };
    config.reg_bits = 16;
    config.read_flag_mask = 0x21;
    config.write_flag_mask = 0x20;

    unsafe { cs4271_probe(&mut (*spi).dev, devm_regmap_init_spi(spi, &config)) }
}

static cs4271_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: b"cs4271\0".as_ptr() as *const c_char,
        of_match_table: unsafe { of_match_ptr(cs4271_dt_ids.as_ptr()) },
    },
    probe: Some(cs4271_spi_probe),
};

// module_spi_driver(cs4271_spi_driver);

// MODULE_DESCRIPTION("ASoC CS4271 SPI Driver");
// MODULE_AUTHOR("Alexander Sverdlin <subaparts@yandex.ru>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
