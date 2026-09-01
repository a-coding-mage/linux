// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for ADAU1361/ADAU1461/ADAU1761/ADAU1961 codec
 *
 * Copyright 2014 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

// C dependencies: linux/module.h, linux/regmap.h, linux/spi/spi.h,
// sound/soc.h, and "adau1761.h".

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

const EINVAL: c_int = 22;

extern "C" {
    static adau1761_regmap_config: regmap_config;
    static adau1761_spi_dt_ids: [of_device_id; 5];

    fn to_spi_device(dev: *mut device) -> *mut spi_device;
    fn spi_w8r8(spi: *mut spi_device, cmd: u8) -> u8;
    fn spi_get_device_id(spi: *mut spi_device) -> *const spi_device_id;
    fn devm_regmap_init_spi(
        spi: *mut spi_device,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn adau1761_probe(
        dev: *mut device,
        regmap: *mut regmap,
        driver_data: c_ulong,
        switch_mode: Option<unsafe extern "C" fn(*mut device)>,
    ) -> c_int;
    fn adau17x1_remove(dev: *mut device);
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct regmap_config {
    pub val_bits: c_uint,
    pub reg_bits: c_uint,
    pub read_flag_mask: c_uint,
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct spi_device_id {
    pub name: [c_char; 32],
    pub driver_data: c_ulong,
}

#[repr(C)]
#[derive(Clone, Copy)]
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

const ADAU1361: c_ulong = 0;
const ADAU1761: c_ulong = 1;

const fn c_name_32(bytes: &[u8]) -> [c_char; 32] {
    let mut out = [0 as c_char; 32];
    let mut i = 0;

    while i < bytes.len() {
        out[i] = bytes[i] as c_char;
        i += 1;
    }

    out
}

unsafe extern "C" fn adau1761_spi_switch_mode(dev: *mut device) {
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

unsafe extern "C" fn adau1761_spi_probe(spi: *mut spi_device) -> c_int {
    let id: *const spi_device_id = unsafe { spi_get_device_id(spi) };
    let mut config: regmap_config;

    if id.is_null() {
        return -EINVAL;
    }

    config = unsafe { adau1761_regmap_config };
    config.val_bits = 8;
    config.reg_bits = 24;
    config.read_flag_mask = 0x1;

    unsafe {
        adau1761_probe(
            &mut (*spi).dev,
            devm_regmap_init_spi(spi, &config),
            (*id).driver_data,
            Some(adau1761_spi_switch_mode),
        )
    }
}

unsafe extern "C" fn adau1761_spi_remove(spi: *mut spi_device) {
    unsafe {
        adau17x1_remove(&mut (*spi).dev);
    }
}

static adau1761_spi_id: [spi_device_id; 5] = [
    spi_device_id {
        name: c_name_32(b"adau1361"),
        driver_data: ADAU1361,
    },
    spi_device_id {
        name: c_name_32(b"adau1461"),
        driver_data: ADAU1761,
    },
    spi_device_id {
        name: c_name_32(b"adau1761"),
        driver_data: ADAU1761,
    },
    spi_device_id {
        name: c_name_32(b"adau1961"),
        driver_data: ADAU1361,
    },
    spi_device_id {
        name: [0; 32],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(spi, adau1761_spi_id);

// Original C condition: #if defined(CONFIG_OF)
static adau1761_spi_dt_ids_local: [of_device_id; 5] = [
    of_device_id {
        compatible: b"adi,adau1361\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"adi,adau1461\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"adi,adau1761\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"adi,adau1961\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, adau1761_spi_dt_ids);

unsafe fn of_match_ptr(ptr: *const of_device_id) -> *const of_device_id {
    ptr
}

static mut adau1761_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: b"adau1761\0".as_ptr() as *const c_char,
        of_match_table: unsafe { of_match_ptr(adau1761_spi_dt_ids_local.as_ptr()) },
    },
    probe: Some(adau1761_spi_probe),
    remove: Some(adau1761_spi_remove),
    id_table: adau1761_spi_id.as_ptr(),
};
// module_spi_driver(adau1761_spi_driver);

// MODULE_DESCRIPTION("ASoC ADAU1361/ADAU1461/ADAU1761/ADAU1961 CODEC SPI driver");
// MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
