// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PCM179X ASoC SPI driver
 *
 * Copyright (c) Amarula Solutions B.V. 2013
 *
 *     Michael Trimarchi <michael@amarulasolutions.com>
 */

// C dependencies:
// #include <linux/module.h>
// #include <linux/of.h>
// #include <linux/spi/spi.h>
// #include <linux/regmap.h>
// #include "pcm179x.h"

extern "C" {
    static pcm179x_regmap_config: regmap_config;

    fn devm_regmap_init_spi(
        spi: *mut spi_device,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> core::ffi::c_long;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn pcm179x_common_init(dev: *mut device, regmap: *mut regmap) -> core::ffi::c_int;
    fn of_match_ptr(ptr: *const of_device_id) -> *const of_device_id;
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
pub struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
}

#[repr(C)]
pub struct spi_device_id {
    pub name: *const core::ffi::c_char,
    pub driver_data: core::ffi::c_ulong,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: device_driver,
    pub id_table: *const spi_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> core::ffi::c_int>,
}

unsafe extern "C" fn pcm179x_spi_probe(spi: *mut spi_device) -> core::ffi::c_int {
    let regmap: *mut regmap;
    let ret: core::ffi::c_int;

    regmap = devm_regmap_init_spi(spi, &pcm179x_regmap_config);
    if IS_ERR(regmap as *const core::ffi::c_void) {
        ret = PTR_ERR(regmap as *const core::ffi::c_void) as core::ffi::c_int;
        dev_err(
            &mut (*spi).dev,
            b"Failed to allocate regmap: %d\n\0".as_ptr() as *const core::ffi::c_char,
            ret,
        );
        return ret;
    }

    return pcm179x_common_init(&mut (*spi).dev, regmap);
}

// static const struct of_device_id pcm179x_of_match[] __maybe_unused
#[used]
static pcm179x_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"ti,pcm1792a\0".as_ptr() as *const core::ffi::c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, pcm179x_of_match);

static pcm179x_spi_ids: [spi_device_id; 3] = [
    spi_device_id {
        name: b"pcm1792a\0".as_ptr() as *const core::ffi::c_char,
        driver_data: 0,
    },
    spi_device_id {
        name: b"pcm179x\0".as_ptr() as *const core::ffi::c_char,
        driver_data: 0,
    },
    spi_device_id {
        name: core::ptr::null(),
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(spi, pcm179x_spi_ids);

static mut pcm179x_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: b"pcm179x\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: unsafe { of_match_ptr(pcm179x_of_match.as_ptr()) },
    },
    id_table: pcm179x_spi_ids.as_ptr(),
    probe: Some(pcm179x_spi_probe),
};

// module_spi_driver(pcm179x_spi_driver);

// MODULE_DESCRIPTION("ASoC PCM179X SPI driver");
// MODULE_AUTHOR("Michael Trimarchi <michael@amarulasolutions.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
