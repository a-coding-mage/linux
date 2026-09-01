// SPDX-License-Identifier: GPL-2.0-only
/*
 * ADAV801 audio driver
 *
 * Copyright 2014 Analog Devices Inc.
 */

// C dependencies: <linux/module.h>, <linux/spi/spi.h>, <linux/regmap.h>,
// <sound/soc.h>, and "adav80x.h".

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_device_id {
    pub name: [::core::ffi::c_char; 32],
    pub driver_data: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_config {
    pub read_flag_mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_driver {
    pub name: *const ::core::ffi::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> ::core::ffi::c_int>,
    pub id_table: *const spi_device_id,
}

unsafe extern "C" {
    static adav80x_regmap_config: regmap_config;

    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn adav80x_bus_probe(dev: *mut device, regmap: *mut regmap) -> ::core::ffi::c_int;
}

const fn spi_device_id_name(name: &[u8]) -> [::core::ffi::c_char; 32] {
    let mut out = [0 as ::core::ffi::c_char; 32];
    let mut i = 0;

    while i < name.len() && i < 31 {
        out[i] = name[i] as ::core::ffi::c_char;
        i += 1;
    }

    out
}

static adav80x_spi_id: [spi_device_id; 2] = [
    spi_device_id {
        name: spi_device_id_name(b"adav801"),
        driver_data: 0,
    },
    spi_device_id {
        name: [0 as ::core::ffi::c_char; 32],
        driver_data: 0,
    },
];

// MODULE_DEVICE_TABLE(spi, adav80x_spi_id);

unsafe extern "C" fn adav80x_spi_probe(spi: *mut spi_device) -> ::core::ffi::c_int {
    let mut config: regmap_config;

    unsafe {
        config = adav80x_regmap_config;
    }
    config.read_flag_mask = 0x01;

    unsafe { adav80x_bus_probe(&mut (*spi).dev, devm_regmap_init_spi(spi, &config)) }
}

static mut adav80x_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: c"adav801".as_ptr(),
    },
    probe: Some(adav80x_spi_probe),
    id_table: adav80x_spi_id.as_ptr(),
};

// module_spi_driver(adav80x_spi_driver);

// MODULE_DESCRIPTION("ASoC ADAV801 driver");
// MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
// MODULE_AUTHOR("Yi Li <yi.li@analog.com>>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
