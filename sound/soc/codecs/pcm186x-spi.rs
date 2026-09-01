// SPDX-License-Identifier: GPL-2.0
/*
 * Texas Instruments PCM186x Universal Audio ADC - SPI
 *
 * Copyright (C) 2015-2017 Texas Instruments Incorporated - https://www.ti.com
 *	Andreas Dannenberg <dannenberg@ti.com>
 *	Andrew F. Davis <afd@ti.com>
 */

// C dependencies:
// #include <linux/init.h>
// #include <linux/module.h>
// #include <linux/spi/spi.h>
// #include "pcm186x.h"

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

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
pub struct of_device_id {
    pub name: *const c_char,
    pub type_: *const c_char,
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct spi_device_id {
    pub name: [c_char; 32],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
    pub irq: c_int,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct spi_driver {
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
    pub id_table: *const spi_device_id,
    pub driver: device_driver,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum pcm186x_type {
    PCM1862,
    PCM1863,
    PCM1864,
    PCM1865,
}

unsafe extern "C" {
    static pcm186x_regmap: regmap_config;

    fn spi_get_device_id(spi: *const spi_device) -> *const spi_device_id;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn pcm186x_probe(
        dev: *mut device,
        type_: pcm186x_type,
        irq: c_int,
        regmap: *mut regmap,
    ) -> c_int;
}

const fn spi_device_id_name(name: &[u8]) -> [c_char; 32] {
    let mut out = [0 as c_char; 32];
    let mut i = 0;

    while i < name.len() && i < 31 {
        out[i] = name[i] as c_char;
        i += 1;
    }

    out
}

static pcm186x_of_match: [of_device_id; 5] = [
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"ti,pcm1862".as_ptr(),
        data: pcm186x_type::PCM1862 as usize as *const c_void,
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"ti,pcm1863".as_ptr(),
        data: pcm186x_type::PCM1863 as usize as *const c_void,
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"ti,pcm1864".as_ptr(),
        data: pcm186x_type::PCM1864 as usize as *const c_void,
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"ti,pcm1865".as_ptr(),
        data: pcm186x_type::PCM1865 as usize as *const c_void,
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, pcm186x_of_match);

unsafe extern "C" fn pcm186x_spi_probe(spi: *mut spi_device) -> c_int {
    let type_: pcm186x_type =
        (*spi_get_device_id(spi)).driver_data as usize as pcm186x_type;
    let irq: c_int = (*spi).irq;
    let regmap: *mut regmap;

    regmap = devm_regmap_init_spi(spi, &pcm186x_regmap);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void) as c_int;
    }

    pcm186x_probe(&mut (*spi).dev, type_, irq, regmap)
}

static pcm186x_spi_id: [spi_device_id; 5] = [
    spi_device_id {
        name: spi_device_id_name(b"pcm1862"),
        driver_data: pcm186x_type::PCM1862 as c_ulong,
    },
    spi_device_id {
        name: spi_device_id_name(b"pcm1863"),
        driver_data: pcm186x_type::PCM1863 as c_ulong,
    },
    spi_device_id {
        name: spi_device_id_name(b"pcm1864"),
        driver_data: pcm186x_type::PCM1864 as c_ulong,
    },
    spi_device_id {
        name: spi_device_id_name(b"pcm1865"),
        driver_data: pcm186x_type::PCM1865 as c_ulong,
    },
    spi_device_id {
        name: [0 as c_char; 32],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(spi, pcm186x_spi_id);

static mut pcm186x_spi_driver: spi_driver = spi_driver {
    probe: Some(pcm186x_spi_probe),
    id_table: pcm186x_spi_id.as_ptr(),
    driver: device_driver {
        name: c"pcm186x".as_ptr(),
        of_match_table: pcm186x_of_match.as_ptr(),
    },
};
// module_spi_driver(pcm186x_spi_driver);

// MODULE_AUTHOR("Andreas Dannenberg <dannenberg@ti.com>");
// MODULE_AUTHOR("Andrew F. Davis <afd@ti.com>");
// MODULE_DESCRIPTION("PCM186x Universal Audio ADC SPI Interface Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
