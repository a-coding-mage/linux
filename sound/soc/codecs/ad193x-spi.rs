// SPDX-License-Identifier: GPL-2.0-only
/*
 * AD1938/AD1939 audio driver
 *
 * Copyright 2014 Analog Devices Inc.
 */

// C dependencies:
// #include <linux/module.h>
// #include <linux/spi/spi.h>
// #include <linux/regmap.h>
// #include <sound/soc.h>
// #include "ad193x.h"

use core::ffi::{c_char, c_int, c_ulong};

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
#[derive(Clone, Copy)]
pub struct regmap_config {
    pub val_bits: c_int,
    pub reg_bits: c_int,
    pub read_flag_mask: c_int,
    pub write_flag_mask: c_int,
}

#[repr(C)]
pub struct spi_device_id {
    pub name: [c_char; 32],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
    pub id_table: *const spi_device_id,
}

#[repr(C)]
pub enum ad193x_type {
    AD193X = 0,
    AD1933 = 1,
    AD1934 = 2,
}

unsafe extern "C" {
    static ad193x_regmap_config: regmap_config;

    fn spi_get_device_id(spi: *mut spi_device) -> *const spi_device_id;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn ad193x_probe(dev: *mut device, regmap: *mut regmap, type_: ad193x_type) -> c_int;
}

unsafe extern "C" fn ad193x_spi_probe(spi: *mut spi_device) -> c_int {
    let id: *const spi_device_id = unsafe { spi_get_device_id(spi) };
    let mut config: regmap_config;

    config = unsafe { ad193x_regmap_config };
    config.val_bits = 8;
    config.reg_bits = 16;
    config.read_flag_mask = 0x09;
    config.write_flag_mask = 0x08;

    unsafe {
        ad193x_probe(
            &mut (*spi).dev,
            devm_regmap_init_spi(spi, &config),
            core::mem::transmute::<c_ulong, ad193x_type>((*id).driver_data),
        )
    }
}

static ad193x_spi_id: [spi_device_id; 7] = [
    spi_device_id {
        name: [
            b'a' as c_char,
            b'd' as c_char,
            b'1' as c_char,
            b'9' as c_char,
            b'3' as c_char,
            b'x' as c_char,
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
        driver_data: ad193x_type::AD193X as c_ulong,
    },
    spi_device_id {
        name: [
            b'a' as c_char,
            b'd' as c_char,
            b'1' as c_char,
            b'9' as c_char,
            b'3' as c_char,
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
        driver_data: ad193x_type::AD1933 as c_ulong,
    },
    spi_device_id {
        name: [
            b'a' as c_char,
            b'd' as c_char,
            b'1' as c_char,
            b'9' as c_char,
            b'3' as c_char,
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
        driver_data: ad193x_type::AD1934 as c_ulong,
    },
    spi_device_id {
        name: [
            b'a' as c_char,
            b'd' as c_char,
            b'1' as c_char,
            b'9' as c_char,
            b'3' as c_char,
            b'8' as c_char,
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
        driver_data: ad193x_type::AD193X as c_ulong,
    },
    spi_device_id {
        name: [
            b'a' as c_char,
            b'd' as c_char,
            b'1' as c_char,
            b'9' as c_char,
            b'3' as c_char,
            b'9' as c_char,
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
        driver_data: ad193x_type::AD193X as c_ulong,
    },
    spi_device_id {
        name: [
            b'a' as c_char,
            b'd' as c_char,
            b'a' as c_char,
            b'u' as c_char,
            b'1' as c_char,
            b'3' as c_char,
            b'2' as c_char,
            b'8' as c_char,
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
        driver_data: ad193x_type::AD193X as c_ulong,
    },
    spi_device_id {
        name: [0; 32],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(spi, ad193x_spi_id);

static mut ad193x_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: b"ad193x\0".as_ptr() as *const c_char,
    },
    probe: Some(ad193x_spi_probe),
    id_table: ad193x_spi_id.as_ptr(),
};
// module_spi_driver(ad193x_spi_driver);

// MODULE_DESCRIPTION("ASoC AD1938/AD1939 audio CODEC driver");
// MODULE_AUTHOR("Barry Song <21cnbao@gmail.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
