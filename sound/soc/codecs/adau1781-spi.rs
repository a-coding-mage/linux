// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for ADAU1381/ADAU1781 CODEC
 *
 * Copyright 2014 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

// C includes translated as external dependencies:
// <linux/module.h>
// <linux/regmap.h>
// <linux/spi/spi.h>
// <sound/soc.h>
// "adau1781.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const EINVAL: c_int = 22;
const ADAU1381: c_ulong = 0;
const ADAU1781: c_ulong = 1;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_config {
    pub val_bits: c_uint,
    pub reg_bits: c_uint,
    pub read_flag_mask: c_uint,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_device_id {
    pub name: [c_char; 32],
    pub driver_data: c_ulong,
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
    pub remove: Option<unsafe extern "C" fn(*mut spi_device)>,
    pub id_table: *const spi_device_id,
}

unsafe extern "C" {
    static adau1781_regmap_config: regmap_config;

    fn to_spi_device(dev: *mut device) -> *mut spi_device;
    fn spi_w8r8(spi: *mut spi_device, cmd: u8) -> u8;
    fn spi_get_device_id(spi: *mut spi_device) -> *const spi_device_id;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn adau1781_probe(
        dev: *mut device,
        regmap: *mut regmap,
        driver_data: c_ulong,
        switch_mode: Option<unsafe extern "C" fn(*mut device)>,
    ) -> c_int;
    fn adau17x1_remove(dev: *mut device);
    fn of_match_ptr(ids: *const of_device_id) -> *const of_device_id;
}

unsafe extern "C" fn adau1781_spi_switch_mode(dev: *mut device) {
    let spi: *mut spi_device = unsafe { to_spi_device(dev) };

    /*
     * To get the device into SPI mode CLATCH has to be pulled low three
     * times.  Do this by issuing three dummy reads.
     */
    unsafe {
        spi_w8r8(spi, 0x00);
        spi_w8r8(spi, 0x00);
        spi_w8r8(spi, 0x00);
    }
}

unsafe extern "C" fn adau1781_spi_probe(spi: *mut spi_device) -> c_int {
    let id: *const spi_device_id = unsafe { spi_get_device_id(spi) };
    let mut config: regmap_config;

    if id.is_null() {
        return -EINVAL;
    }

    config = unsafe { adau1781_regmap_config };
    config.val_bits = 8;
    config.reg_bits = 24;
    config.read_flag_mask = 0x1;

    unsafe {
        adau1781_probe(
            &mut (*spi).dev,
            devm_regmap_init_spi(spi, &config),
            (*id).driver_data,
            Some(adau1781_spi_switch_mode),
        )
    }
}

unsafe extern "C" fn adau1781_spi_remove(spi: *mut spi_device) {
    unsafe {
        adau17x1_remove(&mut (*spi).dev);
    }
}

static adau1781_spi_id: [spi_device_id; 3] = [
    spi_device_id {
        name: [
            b'a' as c_char,
            b'd' as c_char,
            b'a' as c_char,
            b'u' as c_char,
            b'1' as c_char,
            b'3' as c_char,
            b'8' as c_char,
            b'1' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        driver_data: ADAU1381,
    },
    spi_device_id {
        name: [
            b'a' as c_char,
            b'd' as c_char,
            b'a' as c_char,
            b'u' as c_char,
            b'1' as c_char,
            b'7' as c_char,
            b'8' as c_char,
            b'1' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        driver_data: ADAU1781,
    },
    spi_device_id {
        name: [0; 32],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(spi, adau1781_spi_id);

// Original C condition: #if defined(CONFIG_OF)
static adau1781_spi_dt_ids: [of_device_id; 3] = [
    of_device_id {
        compatible: b"adi,adau1381\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"adi,adau1781\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, adau1781_spi_dt_ids);

static mut adau1781_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: b"adau1781\0".as_ptr() as *const c_char,
        of_match_table: unsafe { of_match_ptr(adau1781_spi_dt_ids.as_ptr()) },
    },
    probe: Some(adau1781_spi_probe),
    remove: Some(adau1781_spi_remove),
    id_table: adau1781_spi_id.as_ptr(),
};
// module_spi_driver(adau1781_spi_driver);

// MODULE_DESCRIPTION("ASoC ADAU1381/ADAU1781 CODEC SPI driver");
// MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
