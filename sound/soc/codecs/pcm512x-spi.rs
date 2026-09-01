// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for the PCM512x CODECs
 *
 * Author:	Mark Brown <broonie@kernel.org>
 *		Copyright 2014 Linaro Ltd
 */

/* Dependencies from:
 * <linux/init.h>
 * <linux/module.h>
 * <linux/spi/spi.h>
 * "pcm512x.h"
 */

use core::ffi::{c_char, c_int, c_ulong};
use core::ptr;

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
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
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
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct spi_driver {
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut spi_device)>,
    pub id_table: *const spi_device_id,
    pub driver: device_driver,
}

extern "C" {
    static pcm512x_regmap: regmap_config;
    static pcm512x_pm_ops: dev_pm_ops;

    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> c_int;
    fn pcm512x_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
    fn pcm512x_remove(dev: *mut device);
    fn pm_ptr(ptr: *const dev_pm_ops) -> *const dev_pm_ops;
}

unsafe extern "C" fn pcm512x_spi_probe(spi: *mut spi_device) -> c_int {
    let regmap: *mut regmap;
    let ret: c_int;

    regmap = devm_regmap_init_spi(spi, &pcm512x_regmap);
    if IS_ERR(regmap as *const core::ffi::c_void) {
        ret = PTR_ERR(regmap as *const core::ffi::c_void);
        return ret;
    }

    return pcm512x_probe(&mut (*spi).dev, regmap);
}

unsafe extern "C" fn pcm512x_spi_remove(spi: *mut spi_device) {
    pcm512x_remove(&mut (*spi).dev);
}

static pcm512x_spi_id: [spi_device_id; 6] = [
    spi_device_id {
        name: [
            b'p' as c_char,
            b'c' as c_char,
            b'm' as c_char,
            b'5' as c_char,
            b'1' as c_char,
            b'2' as c_char,
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
            0,
        ],
        driver_data: 0,
    },
    spi_device_id {
        name: [
            b'p' as c_char,
            b'c' as c_char,
            b'm' as c_char,
            b'5' as c_char,
            b'1' as c_char,
            b'2' as c_char,
            b'2' as c_char,
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
        driver_data: 0,
    },
    spi_device_id {
        name: [
            b'p' as c_char,
            b'c' as c_char,
            b'm' as c_char,
            b'5' as c_char,
            b'1' as c_char,
            b'4' as c_char,
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
            0,
        ],
        driver_data: 0,
    },
    spi_device_id {
        name: [
            b'p' as c_char,
            b'c' as c_char,
            b'm' as c_char,
            b'5' as c_char,
            b'1' as c_char,
            b'4' as c_char,
            b'2' as c_char,
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
        driver_data: 0,
    },
    spi_device_id {
        name: [
            b'p' as c_char,
            b'c' as c_char,
            b'm' as c_char,
            b'5' as c_char,
            b'2' as c_char,
            b'4' as c_char,
            b'2' as c_char,
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
        driver_data: 0,
    },
    spi_device_id {
        name: [0; 32],
        driver_data: 0,
    },
];
/* MODULE_DEVICE_TABLE(spi, pcm512x_spi_id); */

static pcm512x_of_match: [of_device_id; 6] = [
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: b"ti,pcm5121\0".as_ptr() as *const c_char,
        data: ptr::null(),
    },
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: b"ti,pcm5122\0".as_ptr() as *const c_char,
        data: ptr::null(),
    },
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: b"ti,pcm5141\0".as_ptr() as *const c_char,
        data: ptr::null(),
    },
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: b"ti,pcm5142\0".as_ptr() as *const c_char,
        data: ptr::null(),
    },
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: b"ti,pcm5242\0".as_ptr() as *const c_char,
        data: ptr::null(),
    },
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, pcm512x_of_match); */

static mut pcm512x_spi_driver: spi_driver = spi_driver {
    probe: Some(pcm512x_spi_probe),
    remove: Some(pcm512x_spi_remove),
    id_table: pcm512x_spi_id.as_ptr(),
    driver: device_driver {
        name: b"pcm512x\0".as_ptr() as *const c_char,
        of_match_table: pcm512x_of_match.as_ptr(),
        pm: unsafe { pm_ptr(&pcm512x_pm_ops) },
    },
};

/* module_spi_driver(pcm512x_spi_driver); */

/* MODULE_DESCRIPTION("ASoC PCM512x codec driver - SPI"); */
/* MODULE_AUTHOR("Mark Brown <broonie@kernel.org>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
