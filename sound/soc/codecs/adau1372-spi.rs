// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for ADAU1372 codec
 *
 * Copyright 2016 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

// Dependencies from Linux kernel headers and "adau1372.h":
// linux/module.h, linux/regmap.h, linux/spi/spi.h, sound/soc.h

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
    pub read_flag_mask: u8,
}

#[repr(C)]
pub struct spi_device_id {
    pub name: [::core::ffi::c_char; 32],
    pub driver_data: usize,
}

#[repr(C)]
pub struct of_device_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const ::core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> ::core::ffi::c_int>,
    pub id_table: *const spi_device_id,
}

unsafe extern "C" {
    static adau1372_regmap_config: regmap_config;
    static adau1372_of_match: *const of_device_id;

    fn to_spi_device(dev: *mut device) -> *mut spi_device;
    fn spi_w8r8(spi: *mut spi_device, cmd: u8) -> u8;
    fn devm_regmap_init_spi(
        spi: *mut spi_device,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn adau1372_probe(
        dev: *mut device,
        regmap: *mut regmap,
        switch_mode: Option<unsafe extern "C" fn(*mut device)>,
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" fn adau1372_spi_switch_mode(dev: *mut device) {
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

unsafe extern "C" fn adau1372_spi_probe(spi: *mut spi_device) -> ::core::ffi::c_int {
    let mut config: regmap_config;

    config = unsafe { adau1372_regmap_config };
    config.read_flag_mask = 0x1;

    unsafe {
        adau1372_probe(
            &mut (*spi).dev,
            devm_regmap_init_spi(spi, &config),
            Some(adau1372_spi_switch_mode),
        )
    }
}

static adau1372_spi_id: [spi_device_id; 2] = [
    spi_device_id {
        name: [
            b'a' as ::core::ffi::c_char,
            b'd' as ::core::ffi::c_char,
            b'a' as ::core::ffi::c_char,
            b'u' as ::core::ffi::c_char,
            b'1' as ::core::ffi::c_char,
            b'3' as ::core::ffi::c_char,
            b'7' as ::core::ffi::c_char,
            b'2' as ::core::ffi::c_char,
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
        driver_data: 0,
    },
    spi_device_id {
        name: [0; 32],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(spi, adau1372_spi_id);

static mut adau1372_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: c"adau1372".as_ptr(),
        of_match_table: unsafe { adau1372_of_match },
    },
    probe: Some(adau1372_spi_probe),
    id_table: adau1372_spi_id.as_ptr(),
};
// module_spi_driver(adau1372_spi_driver);

// MODULE_DESCRIPTION("ASoC ADAU1372 CODEC SPI driver");
// MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
