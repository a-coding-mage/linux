// SPDX-License-Identifier: GPL-2.0-only
/*
 * ADAU1977/ADAU1978/ADAU1979 driver
 *
 * Copyright 2014 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

// Dependencies from Linux/module, regmap, OF, SPI, ASoC, and "adau1977.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const EINVAL: c_int = 22;

#[repr(C)]
pub struct device {
    _data: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _data: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
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
pub struct spi_device_id {
    pub name: [c_char; 32],
    pub driver_data: c_ulong,
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
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
    pub id_table: *const spi_device_id,
}

unsafe extern "C" {
    static adau1977_regmap_config: regmap_config;
    static ADAU1977: c_ulong;
    static ADAU1978: c_ulong;

    fn to_spi_device(dev: *mut device) -> *mut spi_device;
    fn spi_w8r8(spi: *mut spi_device, cmd: u8) -> u8;
    fn spi_get_device_id(spi: *mut spi_device) -> *const spi_device_id;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn adau1977_probe(
        dev: *mut device,
        regmap: *mut regmap,
        driver_data: c_ulong,
        switch_mode: Option<unsafe extern "C" fn(*mut device)>,
    ) -> c_int;
    fn of_match_ptr(matches: *const of_device_id) -> *const of_device_id;
    fn module_spi_driver(driver: *mut spi_driver);
}

unsafe extern "C" fn adau1977_spi_switch_mode(dev: *mut device) {
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

unsafe extern "C" fn adau1977_spi_probe(spi: *mut spi_device) -> c_int {
    let id: *const spi_device_id = unsafe { spi_get_device_id(spi) };
    let mut config: regmap_config;

    if id.is_null() {
        return -EINVAL;
    }

    config = unsafe { adau1977_regmap_config };
    config.val_bits = 8;
    config.reg_bits = 16;
    config.read_flag_mask = 0x1;

    unsafe {
        adau1977_probe(
            &mut (*spi).dev,
            devm_regmap_init_spi(spi, &config),
            (*id).driver_data,
            Some(adau1977_spi_switch_mode),
        )
    }
}

static adau1977_spi_ids: [spi_device_id; 4] = [
    spi_device_id {
        name: [
            b'a' as c_char,
            b'd' as c_char,
            b'a' as c_char,
            b'u' as c_char,
            b'1' as c_char,
            b'9' as c_char,
            b'7' as c_char,
            b'7' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        driver_data: unsafe { ADAU1977 },
    },
    spi_device_id {
        name: [
            b'a' as c_char,
            b'd' as c_char,
            b'a' as c_char,
            b'u' as c_char,
            b'1' as c_char,
            b'9' as c_char,
            b'7' as c_char,
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
        driver_data: unsafe { ADAU1978 },
    },
    spi_device_id {
        name: [
            b'a' as c_char,
            b'd' as c_char,
            b'a' as c_char,
            b'u' as c_char,
            b'1' as c_char,
            b'9' as c_char,
            b'7' as c_char,
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
        ],
        driver_data: unsafe { ADAU1978 },
    },
    spi_device_id {
        name: [0; 32],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(spi, adau1977_spi_ids);

// __maybe_unused
static adau1977_spi_of_match: [of_device_id; 4] = [
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"adi,adau1977".as_ptr(),
        data: core::ptr::null(),
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"adi,adau1978".as_ptr(),
        data: core::ptr::null(),
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"adi,adau1979".as_ptr(),
        data: core::ptr::null(),
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, adau1977_spi_of_match);

static mut adau1977_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: c"adau1977".as_ptr(),
        of_match_table: unsafe { of_match_ptr(adau1977_spi_of_match.as_ptr()) },
    },
    probe: Some(adau1977_spi_probe),
    id_table: adau1977_spi_ids.as_ptr(),
};

unsafe fn __register_adau1977_spi_driver() {
    unsafe {
        module_spi_driver(&mut adau1977_spi_driver);
    }
}

// MODULE_DESCRIPTION("ASoC ADAU1977/ADAU1978/ADAU1979 driver");
// MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
